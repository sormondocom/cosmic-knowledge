pub mod session;
pub mod verses_data;

pub use session::run_zohar_session;

/// Metadata for one section of the Zohar.
pub struct ZoharSection {
    pub name:     &'static str,
    pub short:    &'static str,
    pub chapters: u32,
    pub blurb:    &'static str,
}

pub static SECTIONS: &[ZoharSection] = &[
    ZoharSection {
        name:     "Introduction",
        short:    "intro",
        chapters: 8,
        blurb:    "8 named introductory discourses (The Lily, The Occult Origin, etc.)",
    },
    ZoharSection {
        name:     "Bereshith",
        short:    "ber",
        chapters: 79,
        blurb:    "Kabbalistic commentary on Genesis, chapters I–LXXIX",
    },
    ZoharSection {
        name:     "Lekh Lekha",
        short:    "lekh",
        chapters: 22,
        blurb:    "The Call of Abram — chapters LXXX–CI (renumbered 1–22)",
    },
];

/// Resolve user input to a canonical section name.
pub fn resolve_section(input: &str) -> Option<&'static ZoharSection> {
    let s = input.trim().to_lowercase();
    for sec in SECTIONS {
        if s == sec.name.to_lowercase() || s == sec.short {
            return Some(sec);
        }
    }
    for sec in SECTIONS {
        if sec.name.to_lowercase().contains(&s) || s.contains(sec.short) {
            return Some(sec);
        }
    }
    // Numeric shorthand: "1" → Introduction, "2" → Bereshith, "3" → Lekh Lekha
    if let Ok(n) = s.parse::<usize>() {
        return SECTIONS.get(n.wrapping_sub(1));
    }
    None
}
