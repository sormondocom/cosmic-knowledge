//! The 30 Aethyrs — lookup table and display routines.

use colored::*;

// ─── The 30 Aethyrs ───────────────────────────────────────────────────────────
//
// Listed outer → inner (30/TEX → 1/LIL).
// Sources: Dee's diaries; Crowley, "The Vision and the Voice" (1909).
pub const AETHYRS: &[(u32, &str, &str)] = &[
    (30, "TEX", "The outermost Aethyr; threshold of the material world; guarded by 4 Governors"),
    (29, "RII", "The realm of vision and scattered light; first awakening of the scrying sight"),
    (28, "BAG", "Seat of the angels of transformation; where illusion begins to dissolve"),
    (27, "ZAA", "The house of lamentation; the angel ZAMFRES teaches through sorrow"),
    (26, "DES", "The Aethyr of dispersion; scattered forces reassemble by divine will"),
    (25, "VTI", "Victory over the lower elements; the angels of fortitude dwell here"),
    (24, "NIA", "Heaven of joy; the celestial rose blooms; beauty uncontaminated by matter"),
    (23, "TOR", "The chariot and the wheel; the vision of eternal recurrence and liberation"),
    (22, "LIN", "The Aethyr of the cup; the Grail of understanding; chaos before order"),
    (21, "ASP", "The sphinx of the elements; intellect and will in perfect combat"),
    (20, "KHR", "The vision of the dawn; the Aethyr before which Crowley's camel kneels"),
    (19, "POP", "The throne of the spiritual sun; the angel LEXARPH blazes within"),
    (18, "ZEN", "The Aethyr of the child; pure innocence beyond all conditioning"),
    (17, "TAN", "The vision of the balance of karma; TANACH the divine magistrate presides"),
    (16, "LEA", "The lion-serpent; fierce divine love that consumes duality"),
    (15, "OXO", "The ox; patient endurance; the magical memory is revealed here"),
    (14, "UTA", "The Aethyr of the great angel TZADQIEL; justice and mercy intertwined"),
    (13, "ZIM", "The beehive; industrious spirits of creation; the Word is sung in chorus"),
    (12, "LOE", "The vision of the Stooping Dragon; the hidden light within the abyss"),
    (11, "ICH", "The great sea of understanding; the mother of all angels bathes in radiance"),
    (10, "ZAX", "The Abyss; CHORONZON dwells here; annihilation of the ego-self"),
    ( 9, "ZIP", "Above the Abyss; the first Aethyr of the City of the Pyramids"),
    ( 8, "ZID", "The vision of God face to face; Dee and Kelley trembled at its gate"),
    ( 7, "DEO", "The voice of the Aethyr is Love; the name of God revealed in seven letters"),
    ( 6, "MAZ", "The Sword and the Serpent; the reconciling intelligence"),
    ( 5, "LIT", "The vision of the Middle Pillar; the universe a single star"),
    ( 4, "PAZ", "The vision of the holy guardian angel in blinding glory"),
    ( 3, "ZOM", "The vision of the Universal Peacock; all colours in one light"),
    ( 2, "ARN", "The vision of the Tao; the silence before creation"),
    ( 1, "LIL", "The highest Aethyr; LIL means 'night'; the limitless light beyond the first veil"),
];

// ─── Aethyr lookup ────────────────────────────────────────────────────────────

/// Map a gematria total to its resonant Aethyr.
///
/// Uses modular correspondence: remainder 0 → TEX (30), remainder 1 → RII (29) … remainder 29 → LIL (1).
pub fn aethyr_lookup(total: u32) -> (u32, &'static str, &'static str) {
    if total == 0 {
        return (30, "TEX", AETHYRS[0].2);
    }
    let remainder = (total % 30) as usize; // 0..29
    let aethyr_num = if remainder == 0 { 30 } else { 30 - remainder as u32 };
    let idx = 30 - aethyr_num as usize;
    (aethyr_num, AETHYRS[idx].1, AETHYRS[idx].2)
}

// ─── Display routines ─────────────────────────────────────────────────────────

/// Print the table of all 30 Aethyrs to stdout.
pub fn show_aethyr_table() {
    println!();
    println!("{}", "╔═══════════════════════════════════════════════════════════════════════════════════╗".bright_magenta());
    println!("{}", "║          🌌 THE 30 AETHYRS — John Dee's Enochian Angelic System (1582-1587)       ║".bold().bright_magenta());
    println!("{}", "║   Outer (30=TEX) → Inner (1=LIL)   ·   Governors per Aethyr: 4, 3, or 2        ║".dimmed());
    println!("{}", "╠═══════════════════════════════════════════════════════════════════════════════════╣".bright_magenta());
    println!("{}",
        format!("║  {:<4}  {:<5}  {}  ║", "#", "Name", "Description (truncated to 60 chars)")
        .bold().bright_white());
    println!("{}", "╠═══════════════════════════════════════════════════════════════════════════════════╣".bright_magenta());
    for (num, name, desc) in AETHYRS.iter() {
        let short_desc: String = desc.chars().take(58).collect();
        let ellipsis = if desc.len() > 58 { "…" } else { " " };
        let line = format!("║  {:>2}.  {:<5}  {}{} ║", num, name, short_desc, ellipsis);
        if *num <= 10 {
            println!("{}", line.bright_cyan());
        } else if *num <= 20 {
            println!("{}", line.bright_white());
        } else {
            println!("{}", line.white());
        }
    }
    println!("{}", "╠═══════════════════════════════════════════════════════════════════════════════════╣".bright_magenta());
    println!("{}", "║  Tip: use '--aethyr <number or name>' on the CLI for full details              ║".italic().dimmed());
    println!("{}", "╚═══════════════════════════════════════════════════════════════════════════════════╝".bright_magenta());
    println!();
}

/// Display full detail for a single Aethyr, looked up by number or three-letter name.
pub fn show_aethyr_info(query: &str) {
    let found = if let Ok(n) = query.parse::<u32>() {
        AETHYRS.iter().find(|(num, _, _)| *num == n)
    } else {
        let q = query.to_uppercase();
        AETHYRS.iter().find(|(_, name, _)| *name == q.as_str())
    };

    match found {
        None => {
            println!("{}", format!("⚠️  Aethyr '{}' not found. Valid: 1-30 or TEX, RII, BAG … LIL", query).yellow());
        }
        Some((num, name, desc)) => {
            println!();
            println!("{}", "╔══════════════════════════════════════════════╗".bright_cyan());
            println!("{}",
                format!("║  🌌 Aethyr {:>2} — {}  ║", num, name)
                .bold().bright_cyan());
            println!("{}", "╠══════════════════════════════════════════════╣".bright_cyan());

            // Word-wrap description at ~44 chars
            let words: Vec<&str> = desc.split_whitespace().collect();
            let mut line = String::new();
            for word in &words {
                if line.len() + word.len() + 1 > 44 {
                    println!("{}", format!("║  {:<44}  ║", line).bright_white());
                    line = word.to_string();
                } else {
                    if !line.is_empty() { line.push(' '); }
                    line.push_str(word);
                }
            }
            if !line.is_empty() {
                println!("{}", format!("║  {:<44}  ║", line).bright_white());
            }

            let tier = match num {
                1..=9   => "Above the Abyss (City of Pyramids)",
                10      => "The Abyss (ZAX — CHORONZON)",
                11..=18 => "Below the Abyss — Inner Aethyrs",
                19..=25 => "Middle Aethyrs",
                _       => "Outer Aethyrs (material threshold)",
            };
            println!("{}", "╠══════════════════════════════════════════════╣".bright_cyan());
            println!("{}", format!("║  Tier: {:<38}  ║", tier).italic().bright_yellow());
            println!("{}", "╚══════════════════════════════════════════════╝".bright_cyan());
            println!();
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aethyr_lookup_zero_returns_tex() {
        let (num, name, _) = aethyr_lookup(0);
        assert_eq!(num, 30);
        assert_eq!(name, "TEX");
    }

    #[test]
    fn aethyr_lookup_30_returns_tex() {
        let (num, name, _) = aethyr_lookup(30);
        assert_eq!(num, 30);
        assert_eq!(name, "TEX");
    }

    #[test]
    fn aethyr_lookup_1_returns_rii() {
        // 1 % 30 = 1 → remainder 1 → Aethyr 30-1 = 29 (RII)
        let (num, name, _) = aethyr_lookup(1);
        assert_eq!(num, 29);
        assert_eq!(name, "RII");
    }

    #[test]
    fn aethyr_lookup_29_returns_lil() {
        // 29 % 30 = 29 → Aethyr 30-29 = 1 (LIL)
        let (num, name, _) = aethyr_lookup(29);
        assert_eq!(num, 1);
        assert_eq!(name, "LIL");
    }

    #[test]
    fn aethyr_lookup_modulo_wraps_correctly() {
        // 60 % 30 = 0 → TEX; 61 % 30 = 1 → RII
        let (n60, _, _) = aethyr_lookup(60);
        assert_eq!(n60, 30);
        let (n61, _, _) = aethyr_lookup(61);
        assert_eq!(n61, 29);
    }
}
