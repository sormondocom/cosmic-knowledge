"""Fetch Trimorphic Protennoia from earlychristianwritings.com and
parse into SQLite (trimorphic_verses table).

Source: http://www.earlychristianwritings.com/text/trimorphic.html
Translator: John D. Turner (from Robinson, Nag Hammadi Library)

The text has three discourses (NHC XIII,1):
  Discourse 1 — Protennoia as the Thought
  Discourse 2 — Protennoia as the Voice
  Discourse 3 — Protennoia as the Word/Logos

Data model: (book, chapter, verse) where
  book    = "Trimorphic Protennoia"
  chapter = discourse number (1, 2, or 3)
  verse   = paragraph index within discourse
"""

import re
import sqlite3
import urllib.request
from pathlib import Path
from bs4 import BeautifulSoup, NavigableString

DB_PATH = Path(__file__).parent / "cosmic_knowledge.db"
URL     = "http://www.earlychristianwritings.com/text/trimorphic.html"

# ── Fetch ─────────────────────────────────────────────────────────────────────

def fetch_html(url: str) -> str:
    headers = {"User-Agent": "Mozilla/5.0 (compatible; cosmic-knowledge/1.0)"}
    req = urllib.request.Request(url, headers=headers)
    with urllib.request.urlopen(req, timeout=30) as resp:
        return resp.read().decode("utf-8", errors="replace")


# ── Clean ─────────────────────────────────────────────────────────────────────

def clean(text: str) -> str:
    text = text.replace("\xa0", " ")
    text = re.sub(r"\s+", " ", text)
    return text.strip()


# ── Parse ─────────────────────────────────────────────────────────────────────

# Heading patterns that mark a new discourse — must be the ENTIRE segment (short lines)
DISCOURSE_HEADING_RE = re.compile(
    r"(discourse of protennoia\s*[:\-]?\s*one"
    r"|on fate\s*[:\-]?\s*two"
    r"|the third discourse"
    r"|secret logos"
    r"|discourse\s+three)",
    re.I,
)

def is_discourse_heading(text: str) -> bool:
    # Only treat short lines as headings to avoid matching mid-paragraph sentences
    return len(text) < 80 and bool(DISCOURSE_HEADING_RE.search(text))

def is_noise(text: str) -> bool:
    tl = text.lower()
    noise = [
        "early christian writings",
        "earlychristianwritings",
        "peter kirby",
        "translated by",
        "translation",
        "please buy",
        "support the site",
        "view it without ads",
        "bonus stuff",
        "copyright",
        "all rights reserved",
    ]
    if any(n in tl for n in noise):
        return True
    # Filter very short lines only if they're not discourse headings
    if len(text) < 10:
        return True
    return False


def parse_page(html: str) -> list[tuple[str, int, int, str]]:
    soup = BeautifulSoup(html, "html.parser")

    # Remove nav, header, footer, script, style
    for tag in soup.find_all(["script", "style", "nav", "header", "footer"]):
        tag.decompose()

    # Collect text blocks — the page may use <p>, <br>, or bare text nodes
    # Strategy: gather all text from the body, split on double-newline or <br><br>
    body = soup.body or soup

    # Build a flat list of text segments from <p> tags; fall back to <br> splits
    p_tags = body.find_all("p")
    segments: list[str] = []

    if p_tags:
        for p in p_tags:
            t = clean(p.get_text(" ", strip=True))
            if not t:
                continue
            # Always keep potential discourse headings even if short
            if is_discourse_heading(t) or (t and not is_noise(t)):
                segments.append(t)
    else:
        # Flatten body text, splitting at consecutive <br> tags
        raw = str(body)
        raw = re.sub(r"<br\s*/?\s*>\s*<br\s*/?\s*>", "\n\n", raw, flags=re.I)
        text_only = BeautifulSoup(raw, "html.parser").get_text("\n")
        for block in re.split(r"\n{2,}", text_only):
            t = clean(block)
            if t and not is_noise(t):
                segments.append(t)

    # Debug: show what we found
    print(f"  Found {len(segments)} text segments")
    for i, s in enumerate(segments[:5]):
        print(f"  [{i}] {s[:80]}...")

    # Assign discourse numbers based on section headings
    rows: list[tuple[str, int, int, str]] = []
    discourse = 1
    para      = 0

    for seg in segments:
        if is_discourse_heading(seg):
            discourse += 1
            para = 0
            continue
        # Skip the title line itself
        if seg.lower().startswith("trimorphic protennoia") and len(seg) < 60:
            continue
        para += 1
        rows.append(("Trimorphic Protennoia", discourse, para, seg))

    return rows


# ── Database writer ───────────────────────────────────────────────────────────

SCHEMA = """
CREATE TABLE IF NOT EXISTS trimorphic_verses (
    id      INTEGER PRIMARY KEY,
    book    TEXT    NOT NULL,
    chapter INTEGER NOT NULL,
    verse   INTEGER NOT NULL,
    text    TEXT    NOT NULL
);
CREATE INDEX IF NOT EXISTS trim_chap ON trimorphic_verses(book, chapter, verse);

CREATE VIRTUAL TABLE IF NOT EXISTS trimorphic_fts USING fts5(
    book UNINDEXED,
    chapter UNINDEXED,
    verse UNINDEXED,
    text,
    content='trimorphic_verses',
    content_rowid='id'
);
CREATE TRIGGER IF NOT EXISTS trim_ai AFTER INSERT ON trimorphic_verses BEGIN
    INSERT INTO trimorphic_fts(rowid, book, chapter, verse, text)
    VALUES (new.id, new.book, new.chapter, new.verse, new.text);
END;
CREATE TRIGGER IF NOT EXISTS trim_ad AFTER DELETE ON trimorphic_verses BEGIN
    INSERT INTO trimorphic_fts(trimorphic_fts, rowid, book, chapter, verse, text)
    VALUES ('delete', old.id, old.book, old.chapter, old.verse, old.text);
END;
CREATE TRIGGER IF NOT EXISTS trim_au AFTER UPDATE ON trimorphic_verses BEGIN
    INSERT INTO trimorphic_fts(trimorphic_fts, rowid, book, chapter, verse, text)
    VALUES ('delete', old.id, old.book, old.chapter, old.verse, old.text);
    INSERT INTO trimorphic_fts(rowid, book, chapter, verse, text)
    VALUES (new.id, new.book, new.chapter, new.verse, new.text);
END;
"""

def write_db(rows: list[tuple[str, int, int, str]]) -> None:
    conn = sqlite3.connect(DB_PATH)
    conn.executescript(SCHEMA)
    conn.execute("DELETE FROM trimorphic_verses")
    conn.execute("DELETE FROM trimorphic_fts")
    conn.executemany(
        "INSERT INTO trimorphic_verses(book, chapter, verse, text) VALUES (?,?,?,?)",
        rows,
    )
    conn.execute("INSERT INTO trimorphic_fts(trimorphic_fts) VALUES ('rebuild')")
    conn.commit()
    conn.close()


# ── Main ──────────────────────────────────────────────────────────────────────

if __name__ == "__main__":
    print(f"Fetching {URL} ...")
    html = fetch_html(URL)
    print(f"  Downloaded {len(html):,} bytes")

    print("Parsing ...")
    rows = parse_page(html)
    print(f"  {len(rows)} paragraphs extracted")

    if not rows:
        print("ERROR: no paragraphs found — check HTML structure above")
        raise SystemExit(1)

    write_db(rows)
    print(f"Written to {DB_PATH}")

    conn = sqlite3.connect(DB_PATH)
    for ch in range(1, 4):
        count = conn.execute(
            "SELECT COUNT(*) FROM trimorphic_verses WHERE chapter=?", (ch,)
        ).fetchone()[0]
        print(f"  Discourse {ch}: {count} paragraphs")
    conn.close()
