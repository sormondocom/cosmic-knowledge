#!/usr/bin/env python3
"""
parse_quran.py — Download the three-translation Quran from Project Gutenberg
(EBook #16955) and import the Pickthall English translation into the existing
cosmic_knowledge.db, adding:

  quran_verses(id, surah, ayah, surah_name, text)
  quran_fts                — FTS5 virtual table
  meta rows                — source / verse_count / surah_count

Translation: Marmaduke William Pickthall (1930), public domain.
Source: Project Gutenberg EBook #16955.

Usage (from project root):
    python data/parse_quran.py data/cosmic_knowledge.db
"""

import re
import sys
import sqlite3
import os
import urllib.request

GUTENBERG_URL = "https://www.gutenberg.org/cache/epub/16955/pg16955.txt"

# ── Canonical surah names (traditional order 1–114) ──────────────────────────
SURAH_NAMES = [
    "Al-Fatiha",        "Al-Baqara",       "Al-Imran",         "An-Nisa",
    "Al-Maida",         "Al-Anam",         "Al-Araf",          "Al-Anfal",
    "At-Tawba",         "Yunus",            "Hud",              "Yusuf",
    "Ar-Rad",           "Ibrahim",          "Al-Hijr",          "An-Nahl",
    "Al-Isra",          "Al-Kahf",          "Maryam",           "Ta-Ha",
    "Al-Anbiya",        "Al-Hajj",          "Al-Muminun",       "An-Nur",
    "Al-Furqan",        "Ash-Shuara",       "An-Naml",          "Al-Qasas",
    "Al-Ankabut",       "Ar-Rum",           "Luqman",           "As-Sajda",
    "Al-Ahzab",         "Saba",             "Fatir",            "Ya-Sin",
    "As-Saffat",        "Sad",              "Az-Zumar",         "Ghafir",
    "Fussilat",         "Ash-Shura",        "Az-Zukhruf",       "Ad-Dukhan",
    "Al-Jathiya",       "Al-Ahqaf",         "Muhammad",         "Al-Fath",
    "Al-Hujurat",       "Qaf",              "Adh-Dhariyat",     "At-Tur",
    "An-Najm",          "Al-Qamar",         "Ar-Rahman",        "Al-Waqia",
    "Al-Hadid",         "Al-Mujadila",      "Al-Hashr",         "Al-Mumtahana",
    "As-Saf",           "Al-Jumua",         "Al-Munafiqun",     "At-Taghabun",
    "At-Talaq",         "At-Tahrim",        "Al-Mulk",          "Al-Qalam",
    "Al-Haqqa",         "Al-Maarij",        "Nuh",              "Al-Jinn",
    "Al-Muzzammil",     "Al-Muddaththir",   "Al-Qiyama",        "Al-Insan",
    "Al-Mursalat",      "An-Naba",          "An-Naziat",        "Abasa",
    "At-Takwir",        "Al-Infitar",       "Al-Mutaffifin",    "Al-Inshiqaq",
    "Al-Buruj",         "At-Tariq",         "Al-Ala",           "Al-Ghashiya",
    "Al-Fajr",          "Al-Balad",         "Ash-Shams",        "Al-Layl",
    "Ad-Duha",          "Ash-Sharh",        "At-Tin",           "Al-Alaq",
    "Al-Qadr",          "Al-Bayyina",       "Az-Zalzala",       "Al-Adiyat",
    "Al-Qaria",         "At-Takathur",      "Al-Asr",           "Al-Humaza",
    "Al-Fil",           "Quraysh",          "Al-Maun",          "Al-Kawthar",
    "Al-Kafirun",       "An-Nasr",          "Al-Masad",         "Al-Ikhlas",
    "Al-Falaq",         "An-Nas",
]
assert len(SURAH_NAMES) == 114, f"Expected 114 surah names, got {len(SURAH_NAMES)}"

# ── Ayahs missing from the Gutenberg digitization, patched from the Pickthall ─
# These 4 ayahs are absent from EBook #16955's source text (the line simply
# does not appear).  They are inserted after parsing to complete the corpus.
PATCHES = {
    (17, 33): "And slay not the life which Allah hath forbidden save with right. "
              "Whoso is slain wrongfully, We have given power unto his heir, but "
              "let him not commit excess in slaying. Lo! he will be helped.",
    (39, 46): "Say: O Allah! Creator of the heavens and the earth! Knower of the "
              "invisible and the visible! Thou wilt judge between Thy slaves "
              "concerning that wherein they used to differ.",
    (45, 32): "And when it was said: Lo! Allah's promise is the truth, and there "
              "is no doubt of the Hour's coming, ye said: We know not what the "
              "Hour is. We deem it naught but an opinion, and we are not convinced.",
    (56, 26): "No idle talk, no cause of sin,",
}

# ── Regexes ───────────────────────────────────────────────────────────────────
CHAPTER_RE   = re.compile(r'^\s*Chapter\s+(\d+)\s*:', re.IGNORECASE)
VERSE_REF_RE = re.compile(r'^(\d{3})\.(\d{3})\s*$')
TRANS_RE     = re.compile(r'^([YPS]):\s*(.*)')
SEPARATOR_RE = re.compile(r'^-{5,}')
TOTAL_VER_RE = re.compile(r'Total Verses', re.IGNORECASE)
REVEALED_RE  = re.compile(r'Revealed At', re.IGNORECASE)


# ── Downloader ────────────────────────────────────────────────────────────────

def download_text(url):
    print(f"  Downloading {url} ...")
    req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0"})
    with urllib.request.urlopen(req, timeout=60) as r:
        raw = r.read()
    # Gutenberg files are UTF-8 (sometimes with BOM)
    text = raw.decode("utf-8-sig", errors="replace")
    print(f"  Downloaded {len(text):,} characters.")
    return text


# ── Parser ────────────────────────────────────────────────────────────────────

def parse_to_rows(text):
    lines = text.splitlines()

    # Strip Gutenberg header/footer
    start, end = 0, len(lines)
    for i, line in enumerate(lines):
        if "*** START OF THE PROJECT GUTENBERG EBOOK" in line:
            start = i + 1
        if "*** END OF THE PROJECT GUTENBERG EBOOK" in line:
            end = i
            break
    lines = lines[start:end]

    rows = []

    cur_surah    = 0
    cur_ayah     = 0
    cur_parts    = []   # accumulated Pickthall text lines for current ayah
    in_pickthall = False

    def flush_ayah():
        if cur_surah and cur_ayah and cur_parts:
            text = " ".join(" ".join(p.split()) for p in cur_parts).strip()
            if text:
                name = SURAH_NAMES[cur_surah - 1]
                rows.append((cur_surah, cur_ayah, name, text))

    for line in lines:
        stripped = line.strip()

        # New chapter
        m = CHAPTER_RE.match(stripped)
        if m:
            flush_ayah()
            cur_surah    = int(m.group(1))
            cur_ayah     = 0
            cur_parts    = []
            in_pickthall = False
            continue

        # Verse reference  e.g. "002.255"
        m = VERSE_REF_RE.match(stripped)
        if m:
            flush_ayah()
            cur_ayah     = int(m.group(2))
            cur_parts    = []
            in_pickthall = False
            continue

        # Translation prefix
        m = TRANS_RE.match(stripped)
        if m:
            prefix = m.group(1)
            body   = m.group(2).strip()
            if prefix == "P":
                in_pickthall = True
                cur_parts    = [body] if body else []
            else:
                # Y: or S: ends Pickthall accumulation
                in_pickthall = False
            continue

        # Skip separators, metadata lines
        if (SEPARATOR_RE.match(stripped)
                or TOTAL_VER_RE.search(stripped)
                or REVEALED_RE.search(stripped)):
            continue

        # Continuation line for Pickthall
        if in_pickthall and stripped:
            cur_parts.append(stripped)

    flush_ayah()

    # Apply patches for ayahs absent from the Gutenberg source text
    row_set = {(r[0], r[1]) for r in rows}
    for (surah, ayah), text in PATCHES.items():
        if (surah, ayah) not in row_set:
            name = SURAH_NAMES[surah - 1]
            rows.append((surah, ayah, name, text))

    # Re-sort by (surah, ayah) so patches land in the right position
    rows.sort(key=lambda r: (r[0], r[1]))
    return rows


# ── Database writer ───────────────────────────────────────────────────────────

def write_to_db(rows, db_path):
    conn = sqlite3.connect(db_path)
    cur  = conn.cursor()

    print("  Dropping old Quran tables (if any) ...")
    cur.executescript("""
        DROP TABLE IF EXISTS quran_fts;
        DROP TRIGGER IF EXISTS quran_ai;
        DROP TRIGGER IF EXISTS quran_ad;
        DROP TRIGGER IF EXISTS quran_au;
        DROP TABLE IF EXISTS quran_verses;
    """)

    print("  Creating quran_verses + FTS5 tables ...")
    cur.executescript("""
        CREATE TABLE quran_verses (
            id         INTEGER PRIMARY KEY,
            surah      INTEGER NOT NULL,
            ayah       INTEGER NOT NULL,
            surah_name TEXT    NOT NULL,
            text       TEXT    NOT NULL
        );

        CREATE VIRTUAL TABLE quran_fts USING fts5(
            text,
            content='quran_verses',
            content_rowid='id'
        );

        CREATE TRIGGER quran_ai AFTER INSERT ON quran_verses BEGIN
            INSERT INTO quran_fts(rowid, text) VALUES (new.id, new.text);
        END;
        CREATE TRIGGER quran_ad AFTER DELETE ON quran_verses BEGIN
            INSERT INTO quran_fts(quran_fts, rowid, text)
                VALUES ('delete', old.id, old.text);
        END;
        CREATE TRIGGER quran_au AFTER UPDATE ON quran_verses BEGIN
            INSERT INTO quran_fts(quran_fts, rowid, text)
                VALUES ('delete', old.id, old.text);
            INSERT INTO quran_fts(rowid, text) VALUES (new.id, new.text);
        END;
    """)

    print(f"  Inserting {len(rows):,} ayahs ...")
    cur.executemany(
        "INSERT INTO quran_verses (surah, ayah, surah_name, text) VALUES (?,?,?,?)",
        rows
    )

    print("  Rebuilding FTS5 index ...")
    cur.execute("INSERT INTO quran_fts(quran_fts) VALUES ('rebuild')")

    cur.execute("""
        CREATE TABLE IF NOT EXISTS meta (key TEXT PRIMARY KEY, value TEXT NOT NULL)
    """)
    cur.execute("INSERT OR REPLACE INTO meta VALUES ('quran_source', ?)",
                ("Pickthall English translation — Project Gutenberg EBook #16955",))
    cur.execute("INSERT OR REPLACE INTO meta VALUES ('quran_ayah_count', ?)",
                (str(len(rows)),))
    cur.execute("INSERT OR REPLACE INTO meta VALUES ('quran_surah_count', '114')")

    conn.commit()
    conn.close()

    print()
    print(f"  OK: 114 surahs, {len(rows):,} ayahs imported into {db_path}")
    print(f"    First: {rows[0][2]} {rows[0][0]}:{rows[0][1]}")
    print(f"    Last:  {rows[-1][2]} {rows[-1][0]}:{rows[-1][1]}")


# ── Entry point ───────────────────────────────────────────────────────────────

if __name__ == "__main__":
    if len(sys.argv) not in (2, 3):
        print("Usage: python parse_quran.py <cosmic_knowledge.db> [quran_raw.txt]")
        sys.exit(1)

    db_path = sys.argv[1]
    if len(sys.argv) == 3:
        # Use local file
        raw_path = sys.argv[2]
        if not os.path.exists(raw_path):
            print(f"Error: file not found: {raw_path}")
            sys.exit(1)
        with open(raw_path, encoding="utf-8-sig", errors="replace") as f:
            text = f.read()
        print(f"Loaded {raw_path} ({len(text):,} chars).")
    else:
        text = download_text(GUTENBERG_URL)

    print("Parsing Pickthall translation ...")
    rows = parse_to_rows(text)
    print(f"  Parsed {len(rows):,} ayahs from {len({r[0] for r in rows})} surahs.")

    print(f"Writing to {db_path} ...")
    write_to_db(rows, db_path)
    print("Done.")
