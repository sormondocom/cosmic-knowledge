pub mod session;
pub mod verses_data;

pub use session::run_apocrypha_session;

/// Metadata for one book in the apocrypha collection.
pub struct ApocrBook {
    pub name:     &'static str,
    pub short:    &'static str,
    pub chapters: u32,
    pub blurb:    &'static str,
}

pub static BOOKS: &[ApocrBook] = &[
    ApocrBook {
        name:     "1 Enoch",
        short:    "1en",
        chapters: 108,
        blurb:    "R.H. Charles tr. (1917) — 108 chapters",
    },
    ApocrBook {
        name:     "2 Enoch",
        short:    "2en",
        chapters: 68,
        blurb:    "Secrets of Enoch (Slavonic Enoch) — 68 chapters",
    },
    ApocrBook {
        name:     "Jubilees",
        short:    "jub",
        chapters: 50,
        blurb:    "Little Genesis, R.H. Charles tr. — 50 chapters",
    },
];

/// Resolve a user-supplied string to a canonical book name.
/// Accepts: "1 enoch", "1en", "enoch 1", "jubilees", "jub", "2en", etc.
pub fn resolve_book(input: &str) -> Option<&'static ApocrBook> {
    let s = input.trim().to_lowercase();
    // Exact name match
    for b in BOOKS {
        if s == b.name.to_lowercase() || s == b.short {
            return Some(b);
        }
    }
    // Prefix / contains
    for b in BOOKS {
        if b.name.to_lowercase().contains(&s) || s.contains(b.short) {
            return Some(b);
        }
    }
    // Numeric shorthand: "1" → 1 Enoch, "2" → 2 Enoch, "3" → Jubilees
    if let Ok(n) = s.parse::<usize>() {
        return BOOKS.get(n.wrapping_sub(1));
    }
    None
}
