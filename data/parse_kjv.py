#!/usr/bin/env python3
"""
parse_kjv.py — Import the KJV Bible (kjv_raw.txt) into the existing
cosmic_knowledge.db SQLite database, adding:

  verses(id, book, chapter, verse, text)  — one row per Bible verse
  verses_fts                              — FTS5 virtual table for fast search
  meta row                                — source / verse_count / book_count

Existing tables (users, rng_sessions, readings) are untouched.
Running a second time safely drops and re-creates only the Bible tables.

Usage (from the data/ directory):
    python parse_kjv.py kjv_raw.txt ../data/cosmic_knowledge.db
  or from the project root:
    python data/parse_kjv.py data/kjv_raw.txt data/cosmic_knowledge.db
"""

import re
import sys
import sqlite3
import os

# ── Canonical book names ──────────────────────────────────────────────────────
BOOK_MAP = {
    "The First Book of Moses: Called Genesis":          "Genesis",
    "The Second Book of Moses: Called Exodus":          "Exodus",
    "The Third Book of Moses: Called Leviticus":        "Leviticus",
    "The Fourth Book of Moses: Called Numbers":         "Numbers",
    "The Fifth Book of Moses: Called Deuteronomy":      "Deuteronomy",
    "The Book of Joshua":                               "Joshua",
    "The Book of Judges":                               "Judges",
    "The Book of Ruth":                                 "Ruth",
    "The First Book of Samuel":                         "1 Samuel",
    "The Second Book of Samuel":                        "2 Samuel",
    "The First Book of the Kings":                      "1 Kings",
    "The Second Book of the Kings":                     "2 Kings",
    "The First Book of the Chronicles":                 "1 Chronicles",
    "The Second Book of the Chronicles":                "2 Chronicles",
    "Ezra":                                             "Ezra",
    "The Book of Nehemiah":                             "Nehemiah",
    "The Book of Esther":                               "Esther",
    "The Book of Job":                                  "Job",
    "The Book of Psalms":                               "Psalms",
    "The Proverbs":                                     "Proverbs",
    "Ecclesiastes":                                     "Ecclesiastes",
    "The Song of Solomon":                              "Song of Solomon",
    "The Book of the Prophet Isaiah":                   "Isaiah",
    "The Book of the Prophet Jeremiah":                 "Jeremiah",
    "The Lamentations of Jeremiah":                     "Lamentations",
    "The Book of the Prophet Ezekiel":                  "Ezekiel",
    "The Book of Daniel":                               "Daniel",
    "Hosea":                                            "Hosea",
    "Joel":                                             "Joel",
    "Amos":                                             "Amos",
    "Obadiah":                                          "Obadiah",
    "Jonah":                                            "Jonah",
    "Micah":                                            "Micah",
    "Nahum":                                            "Nahum",
    "Habakkuk":                                         "Habakkuk",
    "Zephaniah":                                        "Zephaniah",
    "Haggai":                                           "Haggai",
    "Zechariah":                                        "Zechariah",
    "Malachi":                                          "Malachi",
    "The Gospel According to Saint Matthew":            "Matthew",
    "The Gospel According to Saint Mark":               "Mark",
    "The Gospel According to Saint Luke":               "Luke",
    "The Gospel According to Saint John":               "John",
    "The Acts of the Apostles":                         "Acts",
    "The Epistle of Paul the Apostle to the Romans":    "Romans",
    "The First Epistle of Paul the Apostle to the Corinthians":  "1 Corinthians",
    "The Second Epistle of Paul the Apostle to the Corinthians": "2 Corinthians",
    "The Epistle of Paul the Apostle to the Galatians":          "Galatians",
    "The Epistle of Paul the Apostle to the Ephesians":          "Ephesians",
    "The Epistle of Paul the Apostle to the Philippians":        "Philippians",
    "The Epistle of Paul the Apostle to the Colossians":         "Colossians",
    "The First Epistle of Paul the Apostle to the Thessalonians":  "1 Thessalonians",
    "The Second Epistle of Paul the Apostle to the Thessalonians": "2 Thessalonians",
    "The First Epistle of Paul the Apostle to Timothy":  "1 Timothy",
    "The Second Epistle of Paul the Apostle to Timothy": "2 Timothy",
    "The Epistle of Paul the Apostle to Titus":          "Titus",
    "The Epistle of Paul the Apostle to Philemon":       "Philemon",
    "The Epistle of Paul the Apostle to the Hebrews":   "Hebrews",
    "The General Epistle of James":                     "James",
    "The First Epistle General of Peter":               "1 Peter",
    "The Second General Epistle of Peter":              "2 Peter",
    "The First Epistle General of John":                "1 John",
    "The Second Epistle General of John":               "2 John",
    "The Third Epistle General of John":                "3 John",
    "The General Epistle of Jude":                      "Jude",
    "The Revelation of Saint John the Divine":          "Revelation",
}

SECTION_HEADERS = {
    "The Old Testament of the King James Version of the Bible",
    "The New Testament of the King James Bible",
}

VERSE_START  = re.compile(r'^(\d+):(\d+)\s+(.*)', re.DOTALL)
# Match an inline verse reference (chapter:verse) that is NOT part of a larger
# number (e.g. "110:5" must not match as chapter 10 verse 5).
# Trailing \s* instead of \s+ so refs at end-of-line (after stripping) also match.
INLINE_REF   = re.compile(r'(?<!\d)(\d+):(\d+)\s*')


# ── Parser ────────────────────────────────────────────────────────────────────

def collect(rows, book, chap, verse, parts):
    if book and verse is not None and parts:
        text = " ".join(" ".join(p.split()) for p in parts).strip()
        if text:
            rows.append((book, chap, verse, text))


def split_inline(s, rows, cur_book, cur_chap, cur_verse, cur_parts):
    """
    Split `s` on any inline verse references (e.g. '...saying, 9:9 And…').
    Emits completed verses into `rows` and returns the updated
    (cur_chap, cur_verse, cur_parts) tuple for the caller.

    Handles refs at end-of-line (no trailing whitespace) because INLINE_REF
    uses \\s* rather than \\s+.  The caller is responsible for appending any
    remaining continuation text to cur_parts.
    """
    parts = INLINE_REF.split(s)
    if len(parts) <= 1:
        # No inline ref — plain text belonging to the current verse.
        if s:
            cur_parts.append(s)
        return cur_chap, cur_verse, cur_parts

    # parts layout from re.split with two capture groups:
    #   [text_before, chap1, verse1, text_after1, chap2, verse2, text_after2, …]
    pre = parts[0].strip()
    if pre:
        cur_parts.append(pre)
    i = 1
    while i + 2 <= len(parts) - 1:
        collect(rows, cur_book, cur_chap, cur_verse, cur_parts)
        cur_chap  = int(parts[i])
        cur_verse = int(parts[i + 1])
        body = parts[i + 2].strip()
        cur_parts = [body] if body else []
        i += 3
    return cur_chap, cur_verse, cur_parts


def parse_to_rows(in_path):
    with open(in_path, encoding="utf-8") as f:
        raw_lines = f.readlines()

    # Strip Gutenberg header / footer
    start, end = 0, len(raw_lines)
    for i, line in enumerate(raw_lines):
        if "*** START OF THE PROJECT GUTENBERG EBOOK" in line:
            start = i + 1
        if "*** END OF THE PROJECT GUTENBERG EBOOK" in line:
            end = i
            break
    lines = raw_lines[start:end]

    # Skip the ToC block: find the second occurrence of the first book heading
    heading_seen: dict = {}
    actual_start = 0
    for i, raw in enumerate(lines):
        s = raw.strip()
        if s in BOOK_MAP:
            heading_seen[s] = heading_seen.get(s, 0) + 1
            if heading_seen[s] == 2:
                actual_start = i
                break
    text_lines = lines[actual_start:]

    rows = []
    cur_book = cur_chap = cur_verse = None
    cur_parts: list = []
    skip_next_book_heading = False  # set after "Otherwise Called:" to skip alt title

    for raw in text_lines:
        s = raw.strip()

        # "Otherwise Called:" is an alternate-title block in the Gutenberg KJV
        # (appears in 1 Samuel and 2 Samuel).  Skip the marker and the heading
        # that follows it so it doesn't accidentally switch the current book.
        if s == "Otherwise Called:":
            skip_next_book_heading = True
            continue

        if s in BOOK_MAP:
            if skip_next_book_heading:
                skip_next_book_heading = False
                continue
            collect(rows, cur_book, cur_chap, cur_verse, cur_parts)
            cur_parts = []
            cur_verse = cur_chap = None
            cur_book = BOOK_MAP[s]
            continue

        if s:
            skip_next_book_heading = False  # any non-empty, non-heading line resets flag

        if s in SECTION_HEADERS or s == "":
            continue

        m = VERSE_START.match(s)
        if m:
            collect(rows, cur_book, cur_chap, cur_verse, cur_parts)
            cur_chap  = int(m.group(1))
            cur_verse = int(m.group(2))
            rest = m.group(3).strip()
            cur_parts = []
            # Apply inline splitting to the rest of the opening line; this catches
            # verses packed onto a single line e.g. "1:1 Text A. 1:2 Text B."
            cur_chap, cur_verse, cur_parts = split_inline(
                rest, rows, cur_book, cur_chap, cur_verse, cur_parts
            )
            continue

        # Continuation line — may also contain inline run-on verse refs
        cur_chap, cur_verse, cur_parts = split_inline(
            s, rows, cur_book, cur_chap, cur_verse, cur_parts
        )

    collect(rows, cur_book, cur_chap, cur_verse, cur_parts)
    return rows


# ── Database writer ───────────────────────────────────────────────────────────

def write_to_db(rows, db_path):
    """
    Add Bible verses to an existing cosmic_knowledge.db.
    Drops and recreates ONLY the Bible-specific tables so existing user data
    (users, rng_sessions, readings) is never touched.
    """
    conn = sqlite3.connect(db_path)
    cur = conn.cursor()

    # Verify the existing user tables are present (sanity check)
    cur.execute("SELECT name FROM sqlite_master WHERE type='table' AND name='users'")
    if not cur.fetchone():
        print("WARNING: 'users' table not found — ensure cosmic_knowledge.db is "
              "initialised by running the app at least once before importing.")

    # Drop and recreate only the Bible tables (safe to re-run)
    print("  Dropping old Bible tables (if any) …")
    cur.executescript("""
        DROP TABLE IF EXISTS verses_fts;
        DROP TRIGGER IF EXISTS verses_ai;
        DROP TRIGGER IF EXISTS verses_ad;
        DROP TRIGGER IF EXISTS verses_au;
        DROP TABLE IF EXISTS verses;
    """)

    print("  Creating verses + FTS5 tables …")
    cur.executescript("""
        CREATE TABLE verses (
            id      INTEGER PRIMARY KEY,
            book    TEXT    NOT NULL,
            chapter INTEGER NOT NULL,
            verse   INTEGER NOT NULL,
            text    TEXT    NOT NULL
        );

        -- FTS5 content table (no text duplication; rowid = verses.id)
        CREATE VIRTUAL TABLE verses_fts USING fts5(
            text,
            content='verses',
            content_rowid='id'
        );

        -- Keep FTS in sync on future inserts / updates / deletes
        CREATE TRIGGER verses_ai AFTER INSERT ON verses BEGIN
            INSERT INTO verses_fts(rowid, text) VALUES (new.id, new.text);
        END;
        CREATE TRIGGER verses_ad AFTER DELETE ON verses BEGIN
            INSERT INTO verses_fts(verses_fts, rowid, text)
                VALUES ('delete', old.id, old.text);
        END;
        CREATE TRIGGER verses_au AFTER UPDATE ON verses BEGIN
            INSERT INTO verses_fts(verses_fts, rowid, text)
                VALUES ('delete', old.id, old.text);
            INSERT INTO verses_fts(rowid, text) VALUES (new.id, new.text);
        END;
    """)

    print(f"  Inserting {len(rows):,} verses …")
    cur.executemany(
        "INSERT INTO verses (book, chapter, verse, text) VALUES (?, ?, ?, ?)",
        rows
    )

    # Rebuild FTS index in one shot (much faster than row-by-row trigger)
    print("  Rebuilding FTS5 index …")
    cur.execute("INSERT INTO verses_fts(verses_fts) VALUES ('rebuild')")

    # Store metadata alongside existing meta (INSERT OR REPLACE is safe)
    cur.execute("""
        CREATE TABLE IF NOT EXISTS meta (
            key TEXT PRIMARY KEY, value TEXT NOT NULL
        )
    """)
    books = sorted({r[0] for r in rows})
    cur.execute("INSERT OR REPLACE INTO meta VALUES ('kjv_source',      ?)",
                ("King James Version — Project Gutenberg EBook #10",))
    cur.execute("INSERT OR REPLACE INTO meta VALUES ('kjv_verse_count', ?)",
                (str(len(rows)),))
    cur.execute("INSERT OR REPLACE INTO meta VALUES ('kjv_book_count',  ?)",
                (str(len(books)),))

    conn.commit()
    conn.close()

    print()
    print(f"  OK: {len(books)} books, {len(rows):,} verses imported into {db_path}")
    print(f"    First book: {books[0]}   Last book: {books[-1]}")


# ── Entry point ───────────────────────────────────────────────────────────────

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python parse_kjv.py <kjv_raw.txt> <cosmic_knowledge.db>")
        sys.exit(1)

    in_path, db_path = sys.argv[1], sys.argv[2]

    if not os.path.exists(in_path):
        print(f"Error: input file not found: {in_path}")
        sys.exit(1)

    print(f"Parsing {in_path} …")
    rows = parse_to_rows(in_path)
    print(f"  Parsed {len(rows):,} verses from {len({r[0] for r in rows})} books.")

    print(f"Writing to {db_path} …")
    write_to_db(rows, db_path)
    print("Done.")
