"""Parse the Zohar (Bereshith to Lekh Lekha, Nurho de Manhar tr.)
from the N: drive sacred-texts.com offline mirror into SQLite.

Source: N:/jud/zdm/ — 111 HTML files (zdm000.htm – zdm110.htm)

Sections:
  Introduction  — zdm002–zdm009  (8 named chapters)
  Bereshith     — zdm010–zdm088  (chapters I–LXXIX, 79 chapters)
  Lekh Lekha    — zdm089–zdm110  (chapters LXXX–CI, renumbered 1–22)

Data model: (section, chapter, paragraph) stored as
            (book TEXT, chapter INTEGER, verse INTEGER, text TEXT)
matching the apocrypha_verses schema so session.rs can reuse the pattern.
"""

import re
import sqlite3
from pathlib import Path
from bs4 import BeautifulSoup

DB_PATH  = Path(__file__).parent / "cosmic_knowledge.db"
ZDM_DIR  = Path("N:/jud/zdm")

# ── Roman numeral helper ──────────────────────────────────────────────────────

ROMAN = {'I': 1, 'V': 5, 'X': 10, 'L': 50, 'C': 100, 'D': 500, 'M': 1000}

def from_roman(s: str) -> int:
    s = s.upper().strip().rstrip('.')
    val, prev = 0, 0
    for ch in reversed(s):
        cur = ROMAN.get(ch, 0)
        val += cur if cur >= prev else -cur
        prev = cur
    return val


# ── HTML cleaning ─────────────────────────────────────────────────────────────

NAV_TEXTS = {'Sacred Texts', 'Judaism', 'Index', 'Previous', 'Next'}

def clean_text(text: str) -> str:
    text = text.replace('\xa0', ' ')
    text = re.sub(r'\s+', ' ', text)
    return text.strip()


def strip_noise(soup: BeautifulSoup) -> None:
    # Navigation links at top
    for a in soup.find_all('a'):
        if a.get_text(strip=True) in NAV_TEXTS:
            a.decompose()
    # Page number annotations: <A NAME="page_N"><FONT SIZE=1 COLOR=GREEN>p. N</FONT></A>
    for tag in soup.find_all('font', attrs={'size': '1'}):
        parent = tag.parent
        tag.decompose()
        if parent and parent.name == 'a' and parent.get('name', '').startswith('page_'):
            parent.decompose()
    # Marginal notes: <span class="margnote">...</span>
    for tag in soup.find_all('span', class_='margnote'):
        tag.decompose()
    # contnote spans
    for tag in soup.find_all('span', class_='contnote'):
        tag.decompose()
    # Images
    for tag in soup.find_all('img'):
        tag.decompose()
    # HR elements
    for tag in soup.find_all('hr'):
        tag.decompose()
    # CENTER navigation block (first CENTER contains nav links)
    for center in soup.find_all('center'):
        text = center.get_text(strip=True)
        if not text or len(text) < 5:
            center.decompose()


def body_paragraphs(soup: BeautifulSoup) -> list[str]:
    strip_noise(soup)
    result = []
    for tag in soup.body.children if soup.body else []:
        if not hasattr(tag, 'name'):
            continue
        if tag.name == 'p':
            t = clean_text(tag.get_text(' ', strip=True))
            if t and len(t) > 10:
                result.append(t)
        elif tag.name in ('h1', 'h2', 'h3', 'h4', 'h5', 'h6'):
            # Include headings as paragraph 0 (section title)
            t = clean_text(tag.get_text(' ', strip=True))
            if t and len(t) > 3:
                result.append(t)
    return result


# ── Section/chapter detection from <title> tag ────────────────────────────────

TITLE_INTRO_RE  = re.compile(r'Zohar:\s*Introduction:\s*(.+)', re.I)
TITLE_CHAP_RE   = re.compile(r'Zohar:\s*Genesis:\s*Chapter\s+([IVXLCDM]+)', re.I)

def detect_section_chapter(title: str) -> tuple[str, int] | None:
    """Return (section_name, chapter_number) or None to skip."""
    m = TITLE_INTRO_RE.search(title)
    if m:
        return None  # chapter number assigned by caller

    m = TITLE_CHAP_RE.search(title)
    if m:
        n = from_roman(m.group(1))
        if n <= 79:
            return ('Bereshith', n)
        else:
            return ('Lekh Lekha', n - 79)  # restart at 1

    return None


# ── Main parser ───────────────────────────────────────────────────────────────

def parse_zohar() -> list[tuple[str, int, int, str]]:
    rows: list[tuple[str, int, int, str]] = []

    # Introduction files: zdm002–zdm009, chapters numbered 1–8
    for intro_num, file_num in enumerate(range(2, 10), start=1):
        fpath = ZDM_DIR / f"zdm{file_num:03d}.htm"
        if not fpath.exists():
            continue
        raw  = fpath.read_text(encoding='utf-8', errors='replace')
        soup = BeautifulSoup(raw, 'html.parser')
        paras = body_paragraphs(soup)
        for para_num, text in enumerate(paras, start=1):
            rows.append(('Introduction', intro_num, para_num, text))

    # Genesis (Bereshith) and Lekh Lekha: zdm010–zdm110
    for file_num in range(10, 111):
        fpath = ZDM_DIR / f"zdm{file_num:03d}.htm"
        if not fpath.exists():
            continue
        raw  = fpath.read_text(encoding='utf-8', errors='replace')
        soup = BeautifulSoup(raw, 'html.parser')
        title_tag = soup.find('title')
        title = title_tag.get_text() if title_tag else ''
        sc = detect_section_chapter(title)
        if sc is None:
            continue
        section, chap = sc
        paras = body_paragraphs(soup)
        for para_num, text in enumerate(paras, start=1):
            rows.append((section, chap, para_num, text))

    return rows


# ── Database writer ───────────────────────────────────────────────────────────

SCHEMA = """
CREATE TABLE IF NOT EXISTS zohar_verses (
    id      INTEGER PRIMARY KEY,
    book    TEXT    NOT NULL,
    chapter INTEGER NOT NULL,
    verse   INTEGER NOT NULL,
    text    TEXT    NOT NULL
);
CREATE INDEX IF NOT EXISTS zohar_book_chap ON zohar_verses(book, chapter, verse);

CREATE VIRTUAL TABLE IF NOT EXISTS zohar_fts USING fts5(
    book UNINDEXED,
    chapter UNINDEXED,
    verse UNINDEXED,
    text,
    content='zohar_verses',
    content_rowid='id'
);
CREATE TRIGGER IF NOT EXISTS zohar_fts_ins AFTER INSERT ON zohar_verses BEGIN
    INSERT INTO zohar_fts(rowid, book, chapter, verse, text)
    VALUES (new.id, new.book, new.chapter, new.verse, new.text);
END;
CREATE TRIGGER IF NOT EXISTS zohar_fts_del AFTER DELETE ON zohar_verses BEGIN
    INSERT INTO zohar_fts(zohar_fts, rowid, book, chapter, verse, text)
    VALUES ('delete', old.id, old.book, old.chapter, old.verse, old.text);
END;
CREATE TRIGGER IF NOT EXISTS zohar_fts_upd AFTER UPDATE ON zohar_verses BEGIN
    INSERT INTO zohar_fts(zohar_fts, rowid, book, chapter, verse, text)
    VALUES ('delete', old.id, old.book, old.chapter, old.verse, old.text);
    INSERT INTO zohar_fts(rowid, book, chapter, verse, text)
    VALUES (new.id, new.book, new.chapter, new.verse, new.text);
END;
"""

def write_db(rows: list[tuple[str, int, int, str]]) -> None:
    conn = sqlite3.connect(DB_PATH)
    conn.executescript(SCHEMA)
    conn.execute("DELETE FROM zohar_verses")
    conn.execute("DELETE FROM zohar_fts")
    conn.executemany(
        "INSERT INTO zohar_verses(book, chapter, verse, text) VALUES (?,?,?,?)",
        rows,
    )
    conn.execute("INSERT INTO zohar_fts(zohar_fts) VALUES ('rebuild')")
    conn.commit()
    conn.close()


# ── Main ──────────────────────────────────────────────────────────────────────

if __name__ == '__main__':
    print("Parsing Zohar (Bereshith to Lekh Lekha)...")
    rows = parse_zohar()
    print(f"  {len(rows)} paragraphs extracted")

    print(f"Writing to {DB_PATH}...")
    write_db(rows)
    print("Done.")

    conn = sqlite3.connect(DB_PATH)
    for section in ['Introduction', 'Bereshith', 'Lekh Lekha']:
        count = conn.execute(
            "SELECT COUNT(*) FROM zohar_verses WHERE book=?", (section,)
        ).fetchone()[0]
        chapters = conn.execute(
            "SELECT COUNT(DISTINCT chapter) FROM zohar_verses WHERE book=?", (section,)
        ).fetchone()[0]
        print(f"  {section}: {count} paragraphs across {chapters} chapters")
    conn.close()
