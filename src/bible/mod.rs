//! KJV Bible Search — beta module.
//!
//! Provides full-text search over the King James Version Bible via the
//! SQLite FTS5 index populated by `data/parse_kjv.py`.
//!
//! ## Source
//!
//! King James Version (1611), public domain.  Text taken from
//! Project Gutenberg EBook #10 (`https://www.gutenberg.org/ebooks/10`).
//!
//! ## Search syntax
//!
//! Uses SQLite FTS5 syntax:
//! - Plain words     `love mercy`       — verses containing both words
//! - Exact phrase    `"still small voice"` — phrase match
//! - Prefix          `mercif*`           — prefix wildcard
//! - Boolean         `light AND NOT darkness`
//!
//! ## Book name aliases
//!
//! All standard abbreviations are accepted (case-insensitive):
//! Gen, Exo, Lev, Num, Deu, Jos, Jdg, Rut, 1Sa, 2Sa, 1Ki, 2Ki,
//! 1Ch, 2Ch, Ezr, Neh, Est, Job, Psa, Pro, Ecc, Sol, Isa, Jer,
//! Lam, Eze, Dan, Hos, Joe, Amo, Oba, Jon, Mic, Nah, Hab, Zep,
//! Hag, Zec, Mal, Mat, Mar, Luk, Joh, Act, Rom, 1Co, 2Co, Gal,
//! Eph, Phi, Col, 1Th, 2Th, 1Ti, 2Ti, Tit, Phm, Heb, Jas, 1Pe,
//! 2Pe, 1Jo, 2Jo, 3Jo, Jud, Rev

mod session;
pub mod verses_data;
pub use session::run_bible_session;

/// Resolve a short book abbreviation (or partial name) to its canonical
/// full name as stored in the `verses` table.  Returns `None` if unrecognised.
pub fn resolve_book(input: &str) -> Option<&'static str> {
    let s = input.trim().to_lowercase();
    for &(canon, aliases) in BOOK_ALIASES {
        let canon_lower = canon.to_lowercase();
        if s == canon_lower {
            return Some(canon);
        }
        for &alias in aliases {
            if s == alias {
                return Some(canon);
            }
        }
    }
    // Partial prefix match (e.g. "gene" → "Genesis")
    for &(canon, _) in BOOK_ALIASES {
        if canon.to_lowercase().starts_with(&s) {
            return Some(canon);
        }
    }
    None
}

/// Return all 66 canonical book names in canonical order.
pub fn all_books() -> impl Iterator<Item = &'static str> {
    BOOK_ALIASES.iter().map(|&(canon, _)| canon)
}

// ─── Book alias table ─────────────────────────────────────────────────────────

static BOOK_ALIASES: &[(&str, &[&str])] = &[
    // ── Old Testament ────────────────────────────────────────────────────────
    ("Genesis",         &["gen", "ge"]),
    ("Exodus",          &["exo", "ex"]),
    ("Leviticus",       &["lev", "le"]),
    ("Numbers",         &["num", "nu"]),
    ("Deuteronomy",     &["deu", "de", "deut"]),
    ("Joshua",          &["jos", "josh"]),
    ("Judges",          &["jdg", "judg"]),
    ("Ruth",            &["rut", "ru"]),
    ("1 Samuel",        &["1sa", "1sam", "1s", "i samuel", "i sam"]),
    ("2 Samuel",        &["2sa", "2sam", "2s", "ii samuel", "ii sam"]),
    ("1 Kings",         &["1ki", "1kin", "1k", "i kings"]),
    ("2 Kings",         &["2ki", "2kin", "2k", "ii kings"]),
    ("1 Chronicles",    &["1ch", "1chr", "1chron", "i chronicles"]),
    ("2 Chronicles",    &["2ch", "2chr", "2chron", "ii chronicles"]),
    ("Ezra",            &["ezr", "ez"]),
    ("Nehemiah",        &["neh", "ne"]),
    ("Esther",          &["est", "esth"]),
    ("Job",             &["job", "jb"]),
    ("Psalms",          &["psa", "ps", "psalm"]),
    ("Proverbs",        &["pro", "prov", "prv"]),
    ("Ecclesiastes",    &["ecc", "eccl", "qoh"]),
    ("Song of Solomon", &["sol", "sos", "song", "canticles"]),
    ("Isaiah",          &["isa", "is"]),
    ("Jeremiah",        &["jer", "je"]),
    ("Lamentations",    &["lam", "la"]),
    ("Ezekiel",         &["eze", "ezek"]),
    ("Daniel",          &["dan", "da"]),
    ("Hosea",           &["hos", "ho"]),
    ("Joel",            &["joe", "jl"]),
    ("Amos",            &["amo", "am"]),
    ("Obadiah",         &["oba", "ob"]),
    ("Jonah",           &["jon", "jnh"]),
    ("Micah",           &["mic", "mi"]),
    ("Nahum",           &["nah", "na"]),
    ("Habakkuk",        &["hab", "hb"]),
    ("Zephaniah",       &["zep", "zeph"]),
    ("Haggai",          &["hag", "hg"]),
    ("Zechariah",       &["zec", "zech"]),
    ("Malachi",         &["mal", "ml"]),
    // ── New Testament ────────────────────────────────────────────────────────
    ("Matthew",          &["mat", "matt", "mt"]),
    ("Mark",             &["mar", "mrk", "mk"]),
    ("Luke",             &["luk", "lk"]),
    ("John",             &["joh", "jn"]),
    ("Acts",             &["act", "ac"]),
    ("Romans",           &["rom", "ro"]),
    ("1 Corinthians",    &["1co", "1cor", "i corinthians", "i cor"]),
    ("2 Corinthians",    &["2co", "2cor", "ii corinthians", "ii cor"]),
    ("Galatians",        &["gal", "ga"]),
    ("Ephesians",        &["eph", "ep"]),
    ("Philippians",      &["phi", "php", "phil"]),
    ("Colossians",       &["col", "co"]),
    ("1 Thessalonians",  &["1th", "1thes", "1thess", "i thessalonians"]),
    ("2 Thessalonians",  &["2th", "2thes", "2thess", "ii thessalonians"]),
    ("1 Timothy",        &["1ti", "1tim", "i timothy"]),
    ("2 Timothy",        &["2ti", "2tim", "ii timothy"]),
    ("Titus",            &["tit", "ti"]),
    ("Philemon",         &["phm", "phlm"]),
    ("Hebrews",          &["heb", "he"]),
    ("James",            &["jas", "jm"]),
    ("1 Peter",          &["1pe", "1pet", "i peter"]),
    ("2 Peter",          &["2pe", "2pet", "ii peter"]),
    ("1 John",           &["1jo", "1jn", "i john"]),
    ("2 John",           &["2jo", "2jn", "ii john"]),
    ("3 John",           &["3jo", "3jn", "iii john"]),
    ("Jude",             &["jud", "jde"]),
    ("Revelation",       &["rev", "re", "apoc", "apocalypse"]),
];
