//! Numerology systems — letter-to-number mappings and interpretive functions.
//!
//! Sub-modules (one per tradition):
//!  - `hebrew`      — Hebrew Gematria (Mispar Hechrachi)
//!  - `pythagorean` — Pythagorean / Western cyclical system
//!  - `chaldean`    — Chaldean / Babylonian vibrational system
//!  - `greek`       — Greek Isopsephy (Neoplatonic tradition)
//!  - `agrippan`    — Agrippan / Barrett English extension
//!  - `ordinal`     — Simple Ordinal and Reverse Ordinal
//!  - `abjad`       — Arabic/Islamic Abjad numerals

pub mod hebrew;
pub mod pythagorean;
pub mod chaldean;
pub mod greek;
pub mod agrippan;
pub mod ordinal;
pub mod abjad;

pub use greek::isopsephy_meaning;
pub use abjad::abjad_meaning;

use std::collections::HashMap;
use crate::enochian::{enochian_lookup, enochian_substitute};

// ─── Core computation ─────────────────────────────────────────────────────────

/// Reduce `n` to a single digit by summing its decimal digits repeatedly.
pub fn digital_root(mut n: u32) -> u32 {
    while n > 9 {
        n = n.to_string().chars().filter_map(|c| c.to_digit(10)).sum();
    }
    n
}

/// Compute totals and digital roots for all ten numerology systems.
///
/// Returns a `Vec` of `(system_name, (total, root))` in stable order:
/// Hebrew Gematria, Pythagorean, Chaldean,
/// Greek Isopsephy, Agrippan, Simple Ordinal, Reverse Ordinal, Abjad,
/// Enochian Ordinal, Enochian G.D.
pub fn numerology(word: &str) -> Vec<(&'static str, (u32, u32))> {
    // Standard map-based systems — shared static tables, no per-call allocation.
    let mut results: Vec<(&'static str, (u32, u32))> = [
        ("Hebrew Gematria", &*hebrew::MAP),
        ("Pythagorean",     &*pythagorean::MAP),
        ("Chaldean",        &*chaldean::MAP),
        ("Greek Isopsephy", &*greek::MAP),
        ("Agrippan",        &*agrippan::MAP),
        ("Simple Ordinal",  &*ordinal::SIMPLE_MAP),
        ("Reverse Ordinal", &*ordinal::REVERSE_MAP),
        ("Abjad",           &*abjad::MAP),
    ]
    .iter()
    .map(|(name, map)| {
        let total = word.chars().filter_map(|c| map.get(&c)).sum::<u32>();
        let root  = digital_root(total);
        (*name, (total, root))
    })
    .collect();

    // Enochian Ordinal (positional 1-21, per Dee's letter order)
    let enochian_ordinal_total: u32 = word
        .chars()
        .filter_map(|c| enochian_lookup(c).map(|(_, ord, _)| ord))
        .sum();
    results.push((
        "Enochian Ordinal",
        (enochian_ordinal_total, digital_root(enochian_ordinal_total)),
    ));

    // Enochian Golden Dawn (Hebrew-mapped values, GD/Schueler tradition)
    let enochian_gd_total: u32 = word
        .chars()
        .filter_map(|c| enochian_lookup(c).map(|(_, _, gd)| gd))
        .sum();
    results.push((
        "Enochian G.D.",
        (enochian_gd_total, digital_root(enochian_gd_total)),
    ));

    results
}

// ─── Interpretive text ────────────────────────────────────────────────────────

/// Western root-number meanings (Hebrew / Pythagorean / Chaldean context).
pub fn meaning_of(root: u32) -> &'static str {
    match root {
        1 => "🌟 New beginnings, leadership, independence, manifestation power",
        2 => "⚖️ Balance, cooperation, divine partnerships, faith and trust",
        3 => "✨ Creativity, self-expression, ascended masters' guidance",
        4 => "🏗️ Stability, hard work, building solid foundations, angels' support",
        5 => "🦋 Freedom, positive change, life lessons, personal transformation",
        6 => "💝 Love, family, nurturing, responsibility, healing energy",
        7 => "🔮 Spiritual awakening, inner wisdom, mystical knowledge, enlightenment",
        8 => "♾️ Material abundance, karma, personal power, infinite possibilities",
        9 => "🕊️ Universal love, spiritual completion, humanitarian service, lightworker",
        _ => "🌌 Beyond the veil of ordinary understanding",
    }
}

/// Brief angelic guidance message keyed to a root number.
pub fn angelic_message(root: u32) -> &'static str {
    match root {
        1 => "The angels encourage you to step into your leadership role. Trust your instincts and take initiative.",
        2 => "Divine timing is at work. Have faith and trust the process. Partnership opportunities are coming.",
        3 => "Your creativity is divinely inspired. Express yourself authentically and share your gifts with the world.",
        4 => "The angels are supporting your efforts. Stay disciplined and focused on building your dreams.",
        5 => "Embrace the changes coming your way. They are divinely guided and lead to your highest good.",
        6 => "Focus on love, family, and healing. You have the power to nurture and heal others.",
        7 => "You are awakening to your spiritual gifts. Trust your intuition and seek deeper wisdom.",
        8 => "Abundance flows to you as you align with your soul's purpose. Balance material and spiritual goals.",
        9 => "You are called to serve humanity. Your compassion and wisdom can heal the world.",
        _ => "The universe holds mysteries beyond current understanding. Stay open to divine guidance.",
    }
}

/// Return a master-number message if `total` is a recognised master number, else `None`.
pub fn master_numbers_message(total: u32) -> Option<&'static str> {
    match total {
        11 => Some("🔥 MASTER NUMBER 11: Spiritual messenger, intuitive insights, enlightenment"),
        22 => Some("🏛️ MASTER NUMBER 22: Master builder, turning dreams into reality, divine architect"),
        33 => Some("👼 MASTER NUMBER 33: Master teacher, Christ consciousness, divine love in action"),
        44 => Some("⚡ MASTER NUMBER 44: Master healer, spiritual warrior, earthly and cosmic balance"),
        55 => Some("🌊 MASTER NUMBER 55: Freedom seeker, life path changes, spiritual transformation"),
        66 => Some("💖 MASTER NUMBER 66: Cosmic parent, unconditional love, healing for all"),
        77 => Some("🌟 MASTER NUMBER 77: Spiritual perfection, higher wisdom, divine knowledge"),
        88 => Some("♾️ MASTER NUMBER 88: Material mastery, karmic balance, infinite abundance"),
        99 => Some("🌍 MASTER NUMBER 99: Universal consciousness, completion, humanitarian leadership"),
        _  => None,
    }
}

/// Detect angelic / repeating / sequential patterns in a total.  Returns `None` when
/// nothing notable is found.
pub fn check_special_sequences(total: u32) -> Option<&'static str> {
    let digits: Vec<char> = total.to_string().chars().collect();

    // Repeating digit sequences
    if digits.len() >= 2 && digits.iter().all(|&d| d == digits[0]) {
        return match digits[0] {
            '1' => Some("Pure manifestation energy - your thoughts are rapidly becoming reality"),
            '2' => Some("Divine partnerships and balance are highlighted"),
            '3' => Some("Ascended masters are near, offering guidance and support"),
            '4' => Some("Angels are surrounding you with love and protection"),
            '5' => Some("Major life changes are coming - embrace transformation"),
            '6' => Some("Focus on home, family, and nurturing relationships"),
            '7' => Some("Spiritual awakening and mystical experiences await"),
            '8' => Some("Material and financial abundance is flowing to you"),
            '9' => Some("A cycle is completing - prepare for new beginnings"),
            _   => None,
        };
    }

    // Sequential sub-patterns
    let num_str = total.to_string();
    if num_str.contains("123") { return Some("Step-by-step progress - you're on the right path!"); }
    if num_str.contains("321") { return Some("Release and let go - clearing the way for new opportunities"); }
    if num_str.contains("111") { return Some("Gateway opening - make a wish, manifestation is powerful now!"); }
    if num_str.contains("222") { return Some("Everything is in perfect divine order - have faith!"); }
    if num_str.contains("333") { return Some("Mind, body, spirit alignment - you are fully supported!"); }
    if num_str.contains("444") { return Some("Angels are everywhere around you - you are safe and loved!"); }
    if num_str.contains("555") { return Some("Buckle up! Major positive changes are accelerating!"); }

    None
}

// ─── Calculation breakdown ────────────────────────────────────────────────────

/// Return a human-readable step-by-step breakdown string such as `"A=1 + B=2 + …"`.
///
/// Returns an empty string for single-character words (nothing to add up).
pub fn get_calculation_breakdown(word: &str, system_name: &str) -> String {
    // Enochian systems: show letter names rather than bare numbers
    if system_name == "Enochian Ordinal" {
        return enochian_breakdown(word, |_, ord, _| ord);
    }
    if system_name == "Enochian G.D." {
        return enochian_breakdown(word, |_, _, gd| gd);
    }

    let map: &HashMap<char, u32> = match system_name {
        "Hebrew Gematria" => &hebrew::MAP,
        "Pythagorean"     => &pythagorean::MAP,
        "Chaldean"        => &chaldean::MAP,
        "Greek Isopsephy" => &greek::MAP,
        "Agrippan"        => &agrippan::MAP,
        "Simple Ordinal"  => &ordinal::SIMPLE_MAP,
        "Reverse Ordinal" => &ordinal::REVERSE_MAP,
        "Abjad"           => &abjad::MAP,
        _                 => return String::new(),
    };

    let parts: Vec<String> = word
        .chars()
        .filter_map(|c| map.get(&c).map(|&v| format!("{}={}", c, v)))
        .collect();

    if parts.len() > 1 { parts.join(" + ") } else { String::new() }
}

/// Internal helper: build an Enochian breakdown using a value-selector closure.
fn enochian_breakdown<F>(word: &str, pick: F) -> String
where
    F: Fn(&'static str, u32, u32) -> u32,
{
    let parts: Vec<String> = word
        .chars()
        .filter_map(|c| {
            enochian_lookup(c).map(|(name, ord, gd)| {
                let val = pick(name, ord, gd);
                let sub = enochian_substitute(c);
                if sub != c {
                    format!("{}→{}({})={}", c, sub, name, val)
                } else {
                    format!("{}({})={}", c, name, val)
                }
            })
        })
        .collect();
    if parts.len() > 1 { parts.join(" + ") } else { String::new() }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── digital_root ──────────────────────────────────────────────────────────

    #[test]
    fn digital_root_single_digits_unchanged() {
        for n in 0..=9 {
            assert_eq!(digital_root(n), n);
        }
    }

    #[test]
    fn digital_root_two_digits() {
        assert_eq!(digital_root(10), 1);  // 1+0=1
        assert_eq!(digital_root(19), 1);  // 1+9=10 → 1+0=1
        assert_eq!(digital_root(29), 2);  // 2+9=11 → 1+1=2
        assert_eq!(digital_root(99), 9);  // 9+9=18 → 1+8=9
    }

    #[test]
    fn digital_root_three_digits() {
        assert_eq!(digital_root(100), 1);
        assert_eq!(digital_root(999), 9);
        assert_eq!(digital_root(123), 6); // 1+2+3=6
    }

    #[test]
    fn digital_root_master_numbers_reduce() {
        // Master numbers are meaningful as totals, but digital_root still reduces them.
        assert_eq!(digital_root(11), 2);
        assert_eq!(digital_root(22), 4);
        assert_eq!(digital_root(33), 6);
    }

    // ── numerology ────────────────────────────────────────────────────────────

    #[test]
    fn numerology_returns_ten_systems() {
        let results = numerology("LOVE");
        assert_eq!(results.len(), 10);
        let names: Vec<&str> = results.iter().map(|(n, _)| *n).collect();
        for expected in &[
            "Hebrew Gematria", "Pythagorean", "Chaldean",
            "Enochian Ordinal", "Enochian G.D.",
            "Greek Isopsephy", "Agrippan", "Simple Ordinal",
            "Reverse Ordinal", "Abjad",
        ] {
            assert!(names.contains(expected), "Missing system: {}", expected);
        }
    }

    // ── new systems ───────────────────────────────────────────────────────────

    #[test]
    fn simple_ordinal_abc() {
        // A=1, B=2, C=3 → total=6, root=6
        let results = numerology("ABC");
        let so = results.iter().find(|(n, _)| *n == "Simple Ordinal").unwrap();
        assert_eq!(so.1.0, 6);
        assert_eq!(so.1.1, 6);
    }

    #[test]
    fn simple_ordinal_z_is_26() {
        let results = numerology("Z");
        let so = results.iter().find(|(n, _)| *n == "Simple Ordinal").unwrap();
        assert_eq!(so.1.0, 26);
    }

    #[test]
    fn reverse_ordinal_a_is_26() {
        let results = numerology("A");
        let ro = results.iter().find(|(n, _)| *n == "Reverse Ordinal").unwrap();
        assert_eq!(ro.1.0, 26);
    }

    #[test]
    fn reverse_ordinal_z_is_1() {
        let results = numerology("Z");
        let ro = results.iter().find(|(n, _)| *n == "Reverse Ordinal").unwrap();
        assert_eq!(ro.1.0, 1);
    }

    #[test]
    fn simple_and_reverse_ordinal_sum_to_27_per_letter() {
        // For any single letter, simple + reverse = 27 (1+26, 2+25, ..., 26+1)
        for c in 'A'..='Z' {
            let word = c.to_string();
            let r = numerology(&word);
            let so = r.iter().find(|(n, _)| *n == "Simple Ordinal").unwrap().1.0;
            let ro = r.iter().find(|(n, _)| *n == "Reverse Ordinal").unwrap().1.0;
            assert_eq!(so + ro, 27, "Letter {} sums to {} not 27", c, so + ro);
        }
    }

    #[test]
    fn agrippan_a_through_i_matches_hebrew_one_through_nine() {
        // A–I in Agrippan are 1–9, same as in Hebrew Gematria.
        for (c, expected) in ('A'..='I').zip(1u32..=9) {
            let results = numerology(&c.to_string());
            let ag = results.iter().find(|(n, _)| *n == "Agrippan").unwrap().1.0;
            assert_eq!(ag, expected, "Agrippan {} = {}, expected {}", c, ag, expected);
        }
    }

    #[test]
    fn isopsephy_o_is_70() {
        // O → Omicron = 70 (not 60 as in Pythagorean mod-9)
        let results = numerology("O");
        let iso = results.iter().find(|(n, _)| *n == "Greek Isopsephy").unwrap();
        assert_eq!(iso.1.0, 70);
    }

    #[test]
    fn abjad_l_is_30() {
        // L → Lam = 30
        let results = numerology("L");
        let ab = results.iter().find(|(n, _)| *n == "Abjad").unwrap();
        assert_eq!(ab.1.0, 30);
    }

    #[test]
    fn numerology_pythagorean_love() {
        // L=3, O=6, V=4, E=5 → total=18 → root=9
        let results = numerology("LOVE");
        let py = results.iter().find(|(n, _)| *n == "Pythagorean").unwrap();
        assert_eq!(py.1.0, 18);
        assert_eq!(py.1.1, 9);
    }

    #[test]
    fn numerology_hebrew_single_letter() {
        // A=1 in Hebrew
        let results = numerology("A");
        let heb = results.iter().find(|(n, _)| *n == "Hebrew Gematria").unwrap();
        assert_eq!(heb.1.0, 1);
        assert_eq!(heb.1.1, 1);
    }

    #[test]
    fn numerology_chaldean_known_values() {
        // A=1, B=2, C=3 → total=6, root=6
        let results = numerology("ABC");
        let ch = results.iter().find(|(n, _)| *n == "Chaldean").unwrap();
        assert_eq!(ch.1.0, 6);
        assert_eq!(ch.1.1, 6);
    }

    // ── special sequences ─────────────────────────────────────────────────────

    #[test]
    fn check_special_sequences_repeating_ones() {
        assert!(check_special_sequences(111).is_some());
        assert!(check_special_sequences(1111).is_some());
    }

    #[test]
    fn check_special_sequences_ascending() {
        assert!(check_special_sequences(123).is_some());
    }

    #[test]
    fn check_special_sequences_none_for_ordinary() {
        assert!(check_special_sequences(42).is_none());
        assert!(check_special_sequences(7).is_none());
    }

    // ── master numbers ────────────────────────────────────────────────────────

    #[test]
    fn master_numbers_all_recognised() {
        for n in [11u32, 22, 33, 44, 55, 66, 77, 88, 99] {
            assert!(master_numbers_message(n).is_some(), "Master number {} not recognised", n);
        }
    }

    #[test]
    fn master_numbers_non_master_returns_none() {
        assert!(master_numbers_message(10).is_none());
        assert!(master_numbers_message(23).is_none());
        assert!(master_numbers_message(100).is_none());
    }
}
