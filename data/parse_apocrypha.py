"""Parse 1 Enoch, 2 Enoch (Secrets of Enoch), and Jubilees from
the N: drive sacred-texts.com offline mirror into SQLite.

Run once to populate data/cosmic_knowledge.db with apocrypha_verses table.
Then run gen_apocrypha_rs.py to emit src/apocrypha/verses_data.rs.
"""

import re
import sqlite3
from pathlib import Path
from bs4 import BeautifulSoup, Tag

DB_PATH = Path(__file__).parent / "cosmic_knowledge.db"

# ── Roman numeral helper ─────────────────────────────────────────────────────

ROMAN = {
    'I': 1, 'V': 5, 'X': 10, 'L': 50,
    'C': 100, 'D': 500, 'M': 1000,
}

def from_roman(s: str) -> int:
    s = s.upper().strip().rstrip('.')
    val = 0
    prev = 0
    for ch in reversed(s):
        cur = ROMAN.get(ch, 0)
        if cur < prev:
            val -= cur
        else:
            val += cur
        prev = cur
    return val


# ── HTML cleaning helpers ────────────────────────────────────────────────────

def clean_text(text: str) -> str:
    text = text.replace('\xa0', ' ')
    text = text.replace('†', '')
    text = re.sub(r'\s+', ' ', text)
    return text.strip()


def strip_noise(soup: BeautifulSoup) -> None:
    """Destructively remove all noise nodes from the soup in-place."""
    # Green font page-number annotations
    for tag in soup.find_all('font', attrs={'color': re.compile(r'green', re.I)}):
        tag.decompose()
    # In-text footnote anchors <a name="fr_N">
    for tag in soup.find_all('a', attrs={'name': re.compile(r'^fr_')}):
        tag.decompose()
    # In-text footnote links <a href="#fn_N">
    for tag in soup.find_all('a', href=re.compile(r'^#fn_')):
        tag.decompose()
    # Footnote destination anchors <a name="fn_N">
    for tag in soup.find_all('a', attrs={'name': re.compile(r'^fn_')}):
        tag.decompose()
    # Footnote back-links in footnote section <a href="file.htm#fr_N">
    for tag in soup.find_all('a', href=re.compile(r'#fr_\d')):
        tag.decompose()
    # Superscript numbers
    for tag in soup.find_all('sup'):
        tag.decompose()
    # <font size="1"> footnote markers
    for tag in soup.find_all('font', attrs={'size': '1'}):
        tag.decompose()
    # TABLE elements (Jubilees margin date notes)
    for tag in soup.find_all('table'):
        tag.decompose()
    # Images
    for tag in soup.find_all('img'):
        tag.decompose()
    # Cut off everything from the Footnotes H3 onward
    for hx in soup.find_all(['h3', 'h4']):
        if 'footnotes' in hx.get_text(strip=True).lower():
            # Remove this tag and everything that follows it in the parent
            for sibling in list(hx.find_all_next()):
                sibling.decompose()
            hx.decompose()
            break


def body_paragraphs(soup: BeautifulSoup) -> list[str]:
    """Return cleaned non-empty paragraph texts after stripping noise."""
    strip_noise(soup)
    result = []
    for p in soup.find_all('p'):
        t = clean_text(p.get_text(' ', strip=True))
        if t:
            result.append(t)
    return result


# ── Inline verse splitter (for 1 Enoch and Jubilees) ────────────────────────

# Matches "1. " or "27. " as a verse marker; avoids "i.e.", "cf.", "Mt."
VERSE_RE = re.compile(r'(?<![a-zA-Z])(\d+)\.\s+')

def split_inline_verses(text: str) -> list[tuple[int, str]]:
    """Return [(verse_num, text), ...]. verse_num=0 means unnumbered prose."""
    parts = VERSE_RE.split(text)
    if len(parts) <= 1:
        t = text.strip()
        return [(0, t)] if t else []
    results = []
    pre = parts[0].strip()
    if pre:
        results.append((0, pre))
    i = 1
    while i + 1 < len(parts):
        vnum = int(parts[i])
        body = parts[i + 1].strip()
        if body:
            results.append((vnum, body))
        i += 2
    return results


# ── 2 Enoch verse extraction (paragraph-per-verse format) ───────────────────

LEADING_NUM_RE = re.compile(r'^(\d+)\s+(.+)', re.DOTALL)

def extract_2enoch_verses(texts: list[str]) -> list[tuple[int, str]]:
    """Extract verses from 2 Enoch's paragraph-per-verse format."""
    results = []
    verse_zero_parts = []
    for t in texts:
        m = LEADING_NUM_RE.match(t)
        if m:
            vnum = int(m.group(1))
            body = m.group(2).strip()
            if body:
                results.append((vnum, body))
        else:
            verse_zero_parts.append(t)
    if verse_zero_parts:
        results.insert(0, (0, ' '.join(verse_zero_parts)))
    return results


NAV_RE = re.compile(r'^(Sacred Texts|Next:|The Forgotten Books)', re.I)


# ── 1 Enoch parser ───────────────────────────────────────────────────────────

BOE_DIR = Path("N:/bib/boe")
BOE_CHAPTER_RE = re.compile(r'\bCHAPTER\s+([IVXLCDM]+)\b', re.I)

def parse_1enoch() -> list[tuple[str, int, int, str]]:
    rows = []
    for fpath in sorted(BOE_DIR.glob("boe*.htm")):
        num = int(fpath.stem[3:])
        if num < 4:
            continue
        raw = fpath.read_text(encoding='utf-8', errors='replace')
        soup = BeautifulSoup(raw, 'html.parser')

        # Find the H3 containing "CHAPTER N" (not the section summary H3)
        chap = None
        for h3 in soup.find_all('h3'):
            m = BOE_CHAPTER_RE.search(h3.get_text())
            if m:
                chap = from_roman(m.group(1))
                break
        if not chap:
            continue

        paras = body_paragraphs(soup)
        filtered = [t for t in paras if not NAV_RE.match(t)]
        combined = ' '.join(filtered)
        # Remove chapter headers like "CHAPTER VI." and section labels
        combined = BOE_CHAPTER_RE.sub('', combined)
        combined = re.sub(r'\bBOOK OF ENOCH\b', '', combined, flags=re.I)
        combined = re.sub(r'\b[IVXLCDM]+-[IVXLCDM]+\.\s+', '', combined)
        combined = combined.strip()

        for vnum, vtext in split_inline_verses(combined):
            if vtext:
                rows.append(('1 Enoch', chap, vnum, vtext))

    return rows


# ── 2 Enoch parser ───────────────────────────────────────────────────────────

FBE_DIR = Path("N:/bib/fbe")
FBE_START, FBE_END = 108, 175
FBE_CHAP_RE = re.compile(r'^([IVXLCDM]+)\.?$', re.I)

def parse_2enoch() -> list[tuple[str, int, int, str]]:
    rows = []
    for num in range(FBE_START, FBE_END + 1):
        fpath = FBE_DIR / f"fbe{num:03d}.htm"
        if not fpath.exists():
            continue
        raw = fpath.read_text(encoding='utf-8', errors='replace')
        soup = BeautifulSoup(raw, 'html.parser')

        h3 = soup.find('h3')
        if not h3:
            continue
        m = FBE_CHAP_RE.match(h3.get_text().strip())
        if not m:
            continue
        chap = from_roman(m.group(1))
        if chap <= 0:
            continue

        paras = body_paragraphs(soup)
        filtered = [t for t in paras if not NAV_RE.match(t)]

        for vnum, vtext in extract_2enoch_verses(filtered):
            if vtext:
                rows.append(('2 Enoch', chap, vnum, vtext))

    return rows


# ── Book of Jasher parser ────────────────────────────────────────────────────

JASHER_DIR = Path("N:/chr/apo/jasher")
JASHER_VERSE_RE = re.compile(r'^(\d+)\s+(.+)', re.DOTALL)

def parse_jasher() -> list[tuple[str, int, int, str]]:
    rows = []
    for chap in range(1, 92):
        fpath = JASHER_DIR / f"{chap}.htm"
        if not fpath.exists():
            continue
        raw = fpath.read_text(encoding='utf-8', errors='replace')
        soup = BeautifulSoup(raw, 'html.parser')
        paras = body_paragraphs(soup)
        for t in paras:
            if NAV_RE.match(t):
                continue
            m = JASHER_VERSE_RE.match(t)
            if m:
                vnum = int(m.group(1))
                vtext = m.group(2).strip()
                if vtext:
                    rows.append(('Book of Jasher', chap, vnum, vtext))
    return rows


# ── Jubilees parser ──────────────────────────────────────────────────────────

JUB_DIR = Path("N:/bib/jub")
JUB_START, JUB_END = 11, 87
JUB_CHAP_REF_RE = re.compile(r'\(([ivxlcdm]+)\.\s*\d', re.I)

def _jubilees_chapter(soup: BeautifulSoup, title: str) -> int | None:
    if 'Prologue' in title:
        return 0
    for hx in soup.find_all(['h1', 'h2', 'h3', 'h4']):
        m = JUB_CHAP_REF_RE.search(hx.get_text())
        if m:
            return from_roman(m.group(1))
    # Fallback: check raw text of first <p> elements
    for p in soup.find_all('p', limit=5):
        m = JUB_CHAP_REF_RE.search(p.get_text())
        if m:
            return from_roman(m.group(1))
    return None


# Jubilees footnote paragraphs: after stripping fn_ anchors, the first token
# is often a page:fn ref like "35:5" or plain text like "Cf. Exod."
# We detect footnote section now by stripping at the H3 "Footnotes" level
# (done in strip_noise), so this should already be handled.

def parse_jubilees() -> list[tuple[str, int, int, str]]:
    rows = []
    for num in range(JUB_START, JUB_END + 1):
        fpath = JUB_DIR / f"jub{num:02d}.htm"
        if not fpath.exists():
            continue
        raw = fpath.read_text(encoding='utf-8', errors='replace')
        soup = BeautifulSoup(raw, 'html.parser')

        title = (soup.find('title') or soup.new_tag('x')).get_text()
        chap = _jubilees_chapter(soup, title)
        if chap is None:
            continue

        paras = body_paragraphs(soup)
        filtered = []
        for t in paras:
            if NAV_RE.match(t):
                continue
            # Skip section title paragraphs that are purely the "(i. 1-26)" style note
            if JUB_CHAP_REF_RE.search(t) and len(t) < 80:
                continue
            filtered.append(t)

        combined = ' '.join(filtered)
        for vnum, vtext in split_inline_verses(combined):
            if vtext and vnum >= 0:
                rows.append(('Jubilees', chap, vnum, vtext))

    return rows


# ── Database writer ──────────────────────────────────────────────────────────

SCHEMA = """
CREATE TABLE IF NOT EXISTS apocrypha_verses (
    id      INTEGER PRIMARY KEY,
    book    TEXT    NOT NULL,
    chapter INTEGER NOT NULL,
    verse   INTEGER NOT NULL,
    text    TEXT    NOT NULL
);
CREATE INDEX IF NOT EXISTS apocrypha_book_chap ON apocrypha_verses(book, chapter, verse);

CREATE VIRTUAL TABLE IF NOT EXISTS apocrypha_fts USING fts5(
    book UNINDEXED,
    chapter UNINDEXED,
    verse UNINDEXED,
    text,
    content='apocrypha_verses',
    content_rowid='id'
);
CREATE TRIGGER IF NOT EXISTS apocrypha_fts_ins AFTER INSERT ON apocrypha_verses BEGIN
    INSERT INTO apocrypha_fts(rowid, book, chapter, verse, text)
    VALUES (new.id, new.book, new.chapter, new.verse, new.text);
END;
CREATE TRIGGER IF NOT EXISTS apocrypha_fts_del AFTER DELETE ON apocrypha_verses BEGIN
    INSERT INTO apocrypha_fts(apocrypha_fts, rowid, book, chapter, verse, text)
    VALUES ('delete', old.id, old.book, old.chapter, old.verse, old.text);
END;
CREATE TRIGGER IF NOT EXISTS apocrypha_fts_upd AFTER UPDATE ON apocrypha_verses BEGIN
    INSERT INTO apocrypha_fts(apocrypha_fts, rowid, book, chapter, verse, text)
    VALUES ('delete', old.id, old.book, old.chapter, old.verse, old.text);
    INSERT INTO apocrypha_fts(rowid, book, chapter, verse, text)
    VALUES (new.id, new.book, new.chapter, new.verse, new.text);
END;
"""

def deduplicate(rows: list[tuple[str, int, int, str]]) -> list[tuple[str, int, int, str]]:
    """Merge rows with the same (book, chapter, verse) by concatenating text."""
    seen: dict[tuple[str, int, int], str] = {}
    for book, chap, verse, text in rows:
        key = (book, chap, verse)
        if key in seen:
            seen[key] = seen[key] + ' ' + text
        else:
            seen[key] = text
    return [(b, c, v, t) for (b, c, v), t in seen.items()]


def write_db(rows: list[tuple[str, int, int, str]]) -> None:
    rows = deduplicate(rows)
    conn = sqlite3.connect(DB_PATH)
    conn.executescript(SCHEMA)
    conn.execute("DELETE FROM apocrypha_verses")
    conn.execute("DELETE FROM apocrypha_fts")
    conn.executemany(
        "INSERT INTO apocrypha_verses(book, chapter, verse, text) VALUES (?,?,?,?)",
        rows,
    )
    conn.execute("INSERT INTO apocrypha_fts(apocrypha_fts) VALUES ('rebuild')")
    conn.commit()
    conn.close()


# ── Main ─────────────────────────────────────────────────────────────────────

if __name__ == '__main__':
    print("Parsing 1 Enoch...")
    r1 = parse_1enoch()
    print(f"  {len(r1)} verse fragments")

    print("Parsing 2 Enoch (Secrets of Enoch)...")
    r2 = parse_2enoch()
    print(f"  {len(r2)} verse fragments")

    print("Parsing Jubilees...")
    r3 = parse_jubilees()
    print(f"  {len(r3)} verse fragments")

    print("Parsing Book of Jasher...")
    r4 = parse_jasher()
    print(f"  {len(r4)} verses")

    all_rows = r1 + r2 + r3 + r4
    print(f"Total: {len(all_rows)} records")

    print(f"Writing to {DB_PATH}...")
    write_db(all_rows)
    print("Done.")

    conn = sqlite3.connect(DB_PATH)
    for book in ['1 Enoch', '2 Enoch', 'Jubilees', 'Book of Jasher']:
        count = conn.execute(
            "SELECT COUNT(*) FROM apocrypha_verses WHERE book=?", (book,)
        ).fetchone()[0]
        chapters = conn.execute(
            "SELECT COUNT(DISTINCT chapter) FROM apocrypha_verses WHERE book=?", (book,)
        ).fetchone()[0]
        print(f"  {book}: {count} verses across {chapters} chapters")
    conn.close()
