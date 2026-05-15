"""Generate src/zohar/verses_data.rs from cosmic_knowledge.db.

Run after parse_zohar.py to bake the Zohar into the binary.
"""

import sqlite3
from pathlib import Path

DB_PATH  = Path(__file__).parent / "cosmic_knowledge.db"
OUT_PATH = Path(__file__).parent.parent / "src" / "zohar" / "verses_data.rs"

def escape_rust(s: str) -> str:
    return s.replace('\\', '\\\\').replace('"', '\\"')

def main() -> None:
    conn = sqlite3.connect(DB_PATH)
    rows = conn.execute(
        "SELECT book, chapter, verse, text FROM zohar_verses ORDER BY book, chapter, verse"
    ).fetchall()
    conn.close()

    print(f"Generating {OUT_PATH} with {len(rows)} paragraphs...")
    OUT_PATH.parent.mkdir(parents=True, exist_ok=True)

    with open(OUT_PATH, 'w', encoding='utf-8') as f:
        f.write("// Auto-generated — do not edit. Re-run data/gen_zohar_rs.py.\n")
        f.write("pub static ZOHAR_VERSES: &[(&str, u32, u32, &str)] = &[\n")
        for book, chap, verse, text in rows:
            f.write(f'    ("{escape_rust(book)}", {chap}, {verse}, "{escape_rust(text)}"),\n')
        f.write("];\n")

    print("Done.")

if __name__ == '__main__':
    main()
