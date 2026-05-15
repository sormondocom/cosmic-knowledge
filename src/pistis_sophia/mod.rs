pub mod session;
pub mod verses_data;

pub use session::run_ps_session;

/// Metadata for one book of the Pistis Sophia.
pub struct PsBook {
    pub name:        &'static str,
    pub short:       &'static str,
    pub ch_lo:       u32,
    pub ch_hi:       u32,
    pub blurb:       &'static str,
}

pub static BOOKS: &[PsBook] = &[
    PsBook {
        name:  "Book 1",
        short: "b1",
        ch_lo: 1,
        ch_hi: 62,
        blurb: "The First Book — chapters 1–62",
    },
    PsBook {
        name:  "Book 2",
        short: "b2",
        ch_lo: 63,
        ch_hi: 101,
        blurb: "The Second Book — chapters 63–101",
    },
    PsBook {
        name:  "Book 3",
        short: "b3",
        ch_lo: 102,
        ch_hi: 125,
        blurb: "A Third Book — chapters 102–125",
    },
    PsBook {
        name:  "Book 4",
        short: "b4",
        ch_lo: 126,
        ch_hi: 135,
        blurb: "A Fourth Book — chapters 126–135",
    },
    PsBook {
        name:  "Book 5",
        short: "b5",
        ch_lo: 136,
        ch_hi: 143,
        blurb: "A Fifth Book — chapters 136–143",
    },
    PsBook {
        name:  "Book 6",
        short: "b6",
        ch_lo: 144,
        ch_hi: 148,
        blurb: "A Sixth Book — chapters 144–148",
    },
];

/// Resolve user input to a canonical book record.
pub fn resolve_book(input: &str) -> Option<&'static PsBook> {
    let s = input.trim().to_lowercase();
    for b in BOOKS {
        if s == b.name.to_lowercase() || s == b.short {
            return Some(b);
        }
    }
    // Numeric shorthand: "1" → Book 1, etc.
    if let Ok(n) = s.parse::<usize>() {
        return BOOKS.get(n.wrapping_sub(1));
    }
    // Chapter number → whichever book contains it
    if let Ok(ch) = s.parse::<u32>() {
        for b in BOOKS {
            if ch >= b.ch_lo && ch <= b.ch_hi {
                return Some(b);
            }
        }
    }
    None
}

/// Return the book that contains a given global chapter number.
pub fn book_for_chapter(ch: u32) -> Option<&'static PsBook> {
    BOOKS.iter().find(|b| ch >= b.ch_lo && ch <= b.ch_hi)
}
