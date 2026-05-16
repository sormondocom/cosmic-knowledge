pub mod session;
pub mod verses_data;

pub use session::run_trimorphic_session;

/// Metadata for one discourse of the Trimorphic Protennoia.
pub struct Discourse {
    pub num:   u32,
    pub name:  &'static str,
    pub blurb: &'static str,
}

pub static DISCOURSES: &[Discourse] = &[
    Discourse {
        num:   1,
        name:  "The Thought",
        blurb: "Protennoia as Father — the ineffable Thought dwelling in the light of silence",
    },
    Discourse {
        num:   2,
        name:  "The Voice",
        blurb: "Protennoia as Mother — the Voice that appeared through Thought, bringing light",
    },
    Discourse {
        num:   3,
        name:  "The Word",
        blurb: "Protennoia as Son/Logos — the Word descending three times to redeem the light-sparks",
    },
];

/// Resolve a user string like "1", "2", "voice", "logos" to a discourse record.
pub fn resolve_discourse(input: &str) -> Option<&'static Discourse> {
    let s = input.trim().to_lowercase();
    for d in DISCOURSES {
        if s == d.num.to_string() {
            return Some(d);
        }
        let lower = d.name.to_lowercase();
        // Match on the second word (Thought / Voice / Word)
        if let Some(word) = lower.split_ascii_whitespace().nth(1) {
            if s == word || lower == s {
                return Some(d);
            }
        }
    }
    None
}
