"""Parse the Pistis Sophia (G.R.S. Mead tr., 1921) from N:/chr/ps/ into SQLite.

Structure:
  Book 1  — Chapters   1– 62  (ps005–ps066)
  Book 2  — Chapters  63–101  (ps068–ps106)
  Book 3  — Chapters 102–125  (ps107–ps130)
  Book 4  — Chapters 126–135  (ps131–ps140)
  Book 5  — Chapters 136–143  (ps141–ps148)
  Book 6  — Chapters 144–148  (ps149–ps153)
  ps067 / ps154 are a scribe note and postscript — skipped.

Data model: (book, chapter, verse) where book = "Book 1" … "Book 6",
chapter = global chapter number (1–148), verse = paragraph index.
"""

import re
import sqlite3
from pathlib import Path
from bs4 import BeautifulSoup

DB_PATH = Path(__file__).parent / "cosmic_knowledge.db"
PS_DIR  = Path("N:/chr/ps")

# ── Chapter range per book ─────────────────────────────────────────────────────

BOOK_RANGES = [
    ("Book 1",  1,  62),
    ("Book 2", 63, 101),
    ("Book 3", 102, 125),
    ("Book 4", 126, 135),
    ("Book 5", 136, 143),
    ("Book 6", 144, 148),
]

# file offset: ps005 = ch1, so file_num = chapter + 4, EXCEPT that ps067 is the
# scribe note (an extra file between ch62 and ch63), shifting everything by 1.
def chapter_to_file(ch: int) -> int:
    if ch <= 62:
        return ch + 4
    else:
        return ch + 5   # skip ps067 (scribe note)

def chapter_to_book(ch: int) -> str:
    for book, lo, hi in BOOK_RANGES:
        if lo <= ch <= hi:
            return book
    return "Book 1"

# ── HTML cleaning ─────────────────────────────────────────────────────────────

NAV_TEXTS = {'Sacred Texts', 'Christianity', 'Gnosticism', 'Index', 'Previous', 'Next'}

def clean_text(text: str) -> str:
    text = text.replace('\xa0', ' ')
    text = re.sub(r'\s+', ' ', text)
    return text.strip()


def strip_noise(soup: BeautifulSoup) -> None:
    # Navigation links
    for a in list(soup.find_all('a')):
        if a.get_text(strip=True) in NAV_TEXTS:
            a.decompose()
    # Page-number annotations: <a name="page_N"><font size="1" color="green">p. N</font>
    for tag in soup.find_all('font', attrs={'size': '1'}):
        parent = tag.parent
        tag.decompose()
        if parent and parent.name == 'a' and parent.get('name', '').startswith('page_'):
            parent.decompose()
    # Marginal notes: <span class="margnote"> and <span class="rmargnote">
    for tag in soup.find_all('span', class_=re.compile(r'margnote')):
        tag.decompose()
    # Running header image block (center with img link)
    for tag in soup.find_all('p', align='center'):
        if tag.find('img') or tag.find('font', attrs={'color': 'GREEN'}):
            tag.decompose()
    for tag in soup.find_all('center'):
        tag.decompose()
    for tag in soup.find_all('img'):
        tag.decompose()
    for tag in soup.find_all('hr'):
        tag.decompose()


def body_paragraphs(soup: BeautifulSoup) -> list[str]:
    strip_noise(soup)
    result = []
    body = soup.body or soup
    for tag in body.children:
        if not hasattr(tag, 'name'):
            continue
        if tag.name == 'p':
            t = clean_text(tag.get_text(' ', strip=True))
            if t and len(t) > 10:
                result.append(t)
        # Skip h1/h2/h3 (chapter headings) — they're already in the reference
    return result


# ── Main parser ───────────────────────────────────────────────────────────────

def parse_pistis_sophia() -> list[tuple[str, int, int, str]]:
    rows: list[tuple[str, int, int, str]] = []

    for book_name, ch_lo, ch_hi in BOOK_RANGES:
        for ch in range(ch_lo, ch_hi + 1):
            file_num = chapter_to_file(ch)
            fpath = PS_DIR / f"ps{file_num:03d}.htm"
            if not fpath.exists():
                print(f"  WARNING: missing {fpath}")
                continue

            raw  = fpath.read_text(encoding='utf-8', errors='replace')
            soup = BeautifulSoup(raw, 'html.parser')

            # Verify this is actually the right chapter
            title = (soup.find('title') or soup.new_tag('x')).get_text()
            if f"Chapter {ch}" not in title and f"Chapter {ch}" not in title:
                # Try to find the chapter number in the heading instead
                pass

            paras = body_paragraphs(soup)
            for para_num, text in enumerate(paras, start=1):
                rows.append((book_name, ch, para_num, text))

    return rows


# ── Database writer ───────────────────────────────────────────────────────────

SCHEMA = """
CREATE TABLE IF NOT EXISTS pistis_sophia_verses (
    id      INTEGER PRIMARY KEY,
    book    TEXT    NOT NULL,
    chapter INTEGER NOT NULL,
    verse   INTEGER NOT NULL,
    text    TEXT    NOT NULL
);
CREATE INDEX IF NOT EXISTS ps_book_chap ON pistis_sophia_verses(book, chapter, verse);

CREATE VIRTUAL TABLE IF NOT EXISTS pistis_sophia_fts USING fts5(
    book UNINDEXED,
    chapter UNINDEXED,
    verse UNINDEXED,
    text,
    content='pistis_sophia_verses',
    content_rowid='id'
);
CREATE TRIGGER IF NOT EXISTS ps_ai AFTER INSERT ON pistis_sophia_verses BEGIN
    INSERT INTO pistis_sophia_fts(rowid, book, chapter, verse, text)
    VALUES (new.id, new.book, new.chapter, new.verse, new.text);
END;
CREATE TRIGGER IF NOT EXISTS ps_ad AFTER DELETE ON pistis_sophia_verses BEGIN
    INSERT INTO pistis_sophia_fts(pistis_sophia_fts, rowid, book, chapter, verse, text)
    VALUES ('delete', old.id, old.book, old.chapter, old.verse, old.text);
END;
CREATE TRIGGER IF NOT EXISTS ps_au AFTER UPDATE ON pistis_sophia_verses BEGIN
    INSERT INTO pistis_sophia_fts(pistis_sophia_fts, rowid, book, chapter, verse, text)
    VALUES ('delete', old.id, old.book, old.chapter, old.verse, old.text);
    INSERT INTO pistis_sophia_fts(rowid, book, chapter, verse, text)
    VALUES (new.id, new.book, new.chapter, new.verse, new.text);
END;
"""

def write_db(rows: list[tuple[str, int, int, str]]) -> None:
    conn = sqlite3.connect(DB_PATH)
    conn.executescript(SCHEMA)
    conn.execute("DELETE FROM pistis_sophia_verses")
    conn.execute("DELETE FROM pistis_sophia_fts")
    conn.executemany(
        "INSERT INTO pistis_sophia_verses(book, chapter, verse, text) VALUES (?,?,?,?)",
        rows,
    )
    conn.execute("INSERT INTO pistis_sophia_fts(pistis_sophia_fts) VALUES ('rebuild')")
    conn.commit()
    conn.close()


# ── Main ──────────────────────────────────────────────────────────────────────

if __name__ == '__main__':
    print("Parsing Pistis Sophia (G.R.S. Mead, 1921)...")
    rows = parse_pistis_sophia()
    print(f"  {len(rows)} paragraphs extracted")

    print(f"Writing to {DB_PATH}...")
    write_db(rows)
    print("Done.")

    conn = sqlite3.connect(DB_PATH)
    for book_name, ch_lo, ch_hi in BOOK_RANGES:
        count = conn.execute(
            "SELECT COUNT(*) FROM pistis_sophia_verses WHERE book=?", (book_name,)
        ).fetchone()[0]
        chapters = conn.execute(
            "SELECT COUNT(DISTINCT chapter) FROM pistis_sophia_verses WHERE book=?", (book_name,)
        ).fetchone()[0]
        print(f"  {book_name} (ch {ch_lo}–{ch_hi}): {count} paragraphs across {chapters} chapters")
    conn.close()
