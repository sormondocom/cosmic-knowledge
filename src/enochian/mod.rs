//! Enochian angelology module — John Dee / Edward Kelley (1582–1587)
//!
//! Sub-modules:
//!  - `alphabet`  — The 21-letter Enochian alphabet, substitution rules, gematria lookup
//!  - `aethyrs`   — The 30 celestial Aethyrs with lookup and display
//!  - `messages`  — Root-number meanings, angelic call fragments, and the 19 Enochian Keys
//!  - `session`   — Interactive terminal session (menus, translation, gematria browser)

pub mod aethyrs;
pub mod alphabet;
pub mod messages;
pub mod session;

pub use aethyrs::{aethyr_lookup, show_aethyr_info, show_aethyr_table, AETHYRS};
pub use alphabet::{enochian_lookup, enochian_substitute, show_enochian_table, ENOCHIAN_LETTERS};
pub use messages::{enochian_angelic_message, enochian_meaning, ENOCHIAN_KEYS};
pub use session::run_enochian_session;

// ─── Aethyr intro chord ───────────────────────────────────────────────────────

/// Pick a random Aethyr and derive three frequencies from its letter ordinals.
///
/// Each of the three letters maps to a frequency via 21-TET rooted at 432 Hz:
///   `f = base × 2^((ordinal − 1) / 21)`
///
/// The three notes are placed in successive octaves (base = 216, 432, 864 Hz)
/// so the chord is always open-voiced across three octave registers.
///
/// Returns `(aethyr_number, name, description, [freq0, freq1, freq2])`.
pub fn random_aethyr_chord() -> (u32, &'static str, &'static str, [f32; 3]) {
    let mut buf = [0u8; 1];
    getrandom::getrandom(&mut buf).unwrap_or(());
    let (num, name, desc) = AETHYRS[buf[0] as usize % AETHYRS.len()];

    const OCTAVE_BASES: [f32; 3] = [216.0, 432.0, 864.0];
    let mut freqs = [432.0f32; 3];
    for (i, c) in name.chars().enumerate().take(3) {
        let ordinal = enochian_lookup(c).map(|(_, o, _)| o).unwrap_or(1);
        freqs[i] = OCTAVE_BASES[i] * 2.0f32.powf((ordinal as f32 - 1.0) / 21.0);
    }

    (num, name, desc, freqs)
}
