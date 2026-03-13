//! Numerology systems module.
//!
//! Contains:
//!  - Five gematria/numerology letter-value systems
//!  - `digital_root` reduction
//!  - Root meanings, angelic messages, master-number detection
//!  - Special sequence detection
//!  - Per-system calculation breakdown strings

use std::collections::HashMap;
use crate::enochian::{enochian_lookup, enochian_substitute};

// ─── Internal helper ──────────────────────────────────────────────────────────

/// Build a `HashMap<char, u32>` from a const array of pairs.
pub fn hashmap_from<const N: usize>(entries: [(char, u32); N]) -> HashMap<char, u32> {
    entries.into_iter().collect()
}

// ─── Core computation ─────────────────────────────────────────────────────────

/// Reduce `n` to a single digit by summing its decimal digits repeatedly.
pub fn digital_root(mut n: u32) -> u32 {
    while n > 9 {
        n = n.to_string().chars().filter_map(|c| c.to_digit(10)).sum();
    }
    n
}

/// Compute totals and digital roots for all five numerology systems.
///
/// Returns a `Vec` of `(system_name, (total, root))` in a stable order:
/// Hebrew Gematria, Pythagorean, Chaldean, Enochian Ordinal, Enochian G.D.
pub fn numerology(word: &str) -> Vec<(&'static str, (u32, u32))> {
    let hebrew = hashmap_from([
        ('A',  1), ('B',  2), ('C',  3), ('D',  4), ('E',   5),
        ('F',  6), ('G',  7), ('H',  8), ('I',  9), ('J',  10),
        ('K', 20), ('L', 30), ('M', 40), ('N', 50), ('O',  60),
        ('P', 70), ('Q', 80), ('R',100), ('S',200), ('T', 300),
        ('U',400), ('V',500), ('W',600), ('X',700), ('Y', 800),
        ('Z',900),
    ]);

    let pythagorean = hashmap_from([
        ('A',1),('B',2),('C',3),('D',4),('E',5),('F',6),('G',7),('H',8),('I',9),
        ('J',1),('K',2),('L',3),('M',4),('N',5),('O',6),('P',7),('Q',8),('R',9),
        ('S',1),('T',2),('U',3),('V',4),('W',5),('X',6),('Y',7),('Z',8),
    ]);

    let chaldean = hashmap_from([
        ('A',1),('B',2),('C',3),('D',4),('E',5),('F',8),('G',3),('H',5),('I',1),
        ('J',1),('K',2),('L',3),('M',4),('N',5),('O',7),('P',8),('Q',1),('R',2),
        ('S',3),('T',4),('U',6),('V',6),('W',6),('X',5),('Y',1),('Z',7),
    ]);

    // Standard map-based systems
    let mut results: Vec<(&'static str, (u32, u32))> = [
        ("Hebrew Gematria", &hebrew),
        ("Pythagorean",     &pythagorean),
        ("Chaldean",        &chaldean),
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

    // Standard map-based systems — rebuild map locally (cheap for short words)
    let hebrew = hashmap_from([
        ('A',  1), ('B',  2), ('C',  3), ('D',  4), ('E',   5),
        ('F',  6), ('G',  7), ('H',  8), ('I',  9), ('J',  10),
        ('K', 20), ('L', 30), ('M', 40), ('N', 50), ('O',  60),
        ('P', 70), ('Q', 80), ('R',100), ('S',200), ('T', 300),
        ('U',400), ('V',500), ('W',600), ('X',700), ('Y', 800),
        ('Z',900),
    ]);
    let pythagorean = hashmap_from([
        ('A',1),('B',2),('C',3),('D',4),('E',5),('F',6),('G',7),('H',8),('I',9),
        ('J',1),('K',2),('L',3),('M',4),('N',5),('O',6),('P',7),('Q',8),('R',9),
        ('S',1),('T',2),('U',3),('V',4),('W',5),('X',6),('Y',7),('Z',8),
    ]);
    let chaldean = hashmap_from([
        ('A',1),('B',2),('C',3),('D',4),('E',5),('F',8),('G',3),('H',5),('I',1),
        ('J',1),('K',2),('L',3),('M',4),('N',5),('O',7),('P',8),('Q',1),('R',2),
        ('S',3),('T',4),('U',6),('V',6),('W',6),('X',5),('Y',1),('Z',7),
    ]);

    let map = match system_name {
        "Hebrew Gematria" => &hebrew,
        "Pythagorean"     => &pythagorean,
        "Chaldean"        => &chaldean,
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
