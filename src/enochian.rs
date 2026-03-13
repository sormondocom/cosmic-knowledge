//! Enochian angelology module — John Dee / Edward Kelley (1582–1587)
//!
//! Contains:
//!  - The Enochian alphabet with ordinal and Golden Dawn gematria values
//!  - The 30 Aethyrs
//!  - Lookup helpers
//!  - Root-number meanings framed in Enochian cosmology
//!  - Display routines for the alphabet and Aethyr tables

use colored::*;

// ─── Enochian alphabet ────────────────────────────────────────────────────────
//
// Each entry: (letter_name, english_chars_it_covers, ordinal_value, golden_dawn_value)
//
// Sources:
//  • Dee's Loagaeth (Sloane MS 3189)
//  • Laycock, "The Complete Enochian Dictionary" (2001)
//  • Regardie / Schueler — Golden Dawn tradition (GD column)
//
// J, K, W have NO Enochian equivalent; they are substituted at call-site:
//   J → I,  K → C,  W → V  (Elizabethan / scholarly consensus)
pub const ENOCHIAN_LETTERS: &[(&str, &[char], u32, u32)] = &[
    // name     chars          ordinal  GD/Hebrew-mapped
    ("Un",    &['A'],         1,    1 ),  // Aleph  = 1
    ("Pa",    &['B'],         2,    2 ),  // Beth   = 2
    ("Veh",   &['C'],         3,    3 ),  // Gimel  = 3
    ("Gal",   &['D'],         4,    4 ),  // Daleth = 4
    ("Graph", &['E'],         5,    5 ),  // Heh    = 5
    ("Or",    &['F'],         6,    6 ),  // Vav    = 6
    ("Ged",   &['G'],         7,    3 ),  // Gimel  = 3 (GD: G maps to Gimel like C)
    ("Na",    &['H'],         8,    8 ),  // Cheth  = 8
    ("Gon",   &['I', 'Y'],    9,   10 ),  // Yod    = 10
    ("Ur",    &['L'],        10,   30 ),  // Lamed  = 30
    ("Tal",   &['M'],        11,   40 ),  // Mem    = 40
    ("Drux",  &['N'],        12,   50 ),  // Nun    = 50
    ("Med",   &['O'],        13,   70 ),  // Ayin   = 70
    ("Mals",  &['P'],        14,   80 ),  // Peh    = 80
    ("Ger",   &['Q'],        15,  100 ),  // Qoph   = 100
    ("Don",   &['R'],        16,  200 ),  // Resh   = 200
    ("Fam",   &['S'],        17,   60 ),  // Samech = 60
    ("Gisg",  &['T'],        18,    9 ),  // Tet    = 9
    ("Van",   &['U', 'V'],   19,    6 ),  // Vav    = 6
    ("Pal",   &['X'],        20,   60 ),  // Samech = 60 (no direct Hebrew equiv.)
    ("Ceph",  &['Z'],        21,    7 ),  // Zayin  = 7
];

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

// ─── Letter helpers ───────────────────────────────────────────────────────────

/// Substitute letters that have no Enochian glyph before lookup.
///
/// `J → I`,  `K → C`,  `W → V`  (Elizabethan / scholarly consensus)
#[inline]
pub fn enochian_substitute(c: char) -> char {
    match c {
        'J' => 'I',
        'K' => 'C',
        'W' => 'V',
        other => other,
    }
}

/// Return `(letter_name, ordinal_value, gd_value)` for an English character, or `None`.
///
/// The substitution table is applied automatically.
pub fn enochian_lookup(c: char) -> Option<(&'static str, u32, u32)> {
    let c = enochian_substitute(c);
    ENOCHIAN_LETTERS.iter().find_map(|(name, chars, ord, gd)| {
        if chars.contains(&c) {
            Some((*name, *ord, *gd))
        } else {
            None
        }
    })
}

// ─── Interpretive functions ───────────────────────────────────────────────────

/// Root-number meanings framed through John Dee's Enochian / angelic-call cosmology.
pub fn enochian_meaning(root: u32) -> &'static str {
    match root {
        1 => "🜁 Un — The Primal Fire; the First Aethyr speaks; a new cycle of angelic emanation begins",
        2 => "🜂 Pa — The Dual Watchtowers; balance between the elemental tablets; duality of creation",
        3 => "🜃 Veh — The Trinitarian Call; Dee's Three Books of Mystery; mind, will, and word aligned",
        4 => "🜄 Gal — The Four Watchtowers (Earth, Air, Fire, Water); the Great Table of the Universe",
        5 => "⊕ Graph — The Fifth element, Spirit; the Tablet of Union (EXARP·HCOMA·NANTA·BITOM)",
        6 => "✡ Or — Six-fold symmetry of the Sigillum Dei Aemeth; harmony of the celestial spheres",
        7 => "☽ Ged — The Seven Heptarchic Kings; Dee's seven planetary governors of divine mystery",
        8 => "♄ Na — The Eight Temples of Heptarchia Mystica; the infinite angels of the lower aethyrs",
        9 => "☉ Gon — The 91 Governors across the 30 Aethyrs (91=7×13); completion of the divine plan",
        _ => "🌌 Beyond the 30th Aethyr — TEX dissolves into the Limitless Light",
    }
}

/// Enochian angelic call message, inspired by the 19 Enochian Keys.
pub fn enochian_angelic_message(root: u32) -> &'static str {
    match root {
        1 => "MICMA — I reign over you, saith the God of Justice. The first call opens the gates of the celestial city.",
        2 => "ADGT — The wings of the winds understand your voices of wonder. Two pillars sustain the heavens.",
        3 => "MICAOLI — Behold, saith your God, I am a circle on whose hands stand Twelve Kingdoms. Three-fold is the light.",
        4 => "OTHIL — I have set my feet in the South, and have looked about me, saying: are not the thunders of increase numbered 33?",
        5 => "SAPAH — The mighty sounds have entered into the third angle, and are become as olives in the olive mount.",
        6 => "GAHE — The spirits of the fourth angle are nine mighty in the firmament of waters; they frame the earth with 46 voices of creation.",
        7 => "RAAS — The East is a house of virgins singing praises among the flames of first glory. Seek the seventh Aethyr.",
        8 => "BAZMELO — The midday of the first is as the third heaven made of hyacinth pillars, 26 in number.",
        9 => "TELOCH — A mighty guard of fire with two-edged swords of ice; the name of their God is Baligon. All is accomplished.",
        _ => "OL SONF VORSG — I reign over you in power exalted above the firmaments of wrath. Enter into the silence.",
    }
}

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

/// Print the full Enochian alphabet table to stdout.
pub fn show_enochian_table() {
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════════════════════╗".bright_yellow());
    println!("{}", "║          📜 THE ENOCHIAN ALPHABET — John Dee / Edward Kelley (1582)      ║".bold().bright_yellow());
    println!("{}", "║    Source: Sloane MS 3189 (Loagaeth) · Laycock, Complete Enochian Dict.  ║".dimmed());
    println!("{}", "╠══════════════════════════════════════════════════════════════════════════╣".bright_yellow());
    println!("{}",
        format!("║  {:<6}  {:<8}  {:<10}  {:>8}  {:>10}  ║",
            "#", "Name", "English", "Ordinal", "GD Value")
        .bold().bright_white());
    println!("{}", "╠══════════════════════════════════════════════════════════════════════════╣".bright_yellow());
    for (i, (name, chars, ord, gd)) in ENOCHIAN_LETTERS.iter().enumerate() {
        let eng: String = chars.iter().map(|c| c.to_string()).collect::<Vec<_>>().join("/");
        println!("{}",
            format!("║  {:<6}  {:<8}  {:<10}  {:>8}  {:>10}  ║",
                format!("{:>2}.", i + 1), name, eng, ord, gd)
            .bright_white());
    }
    println!("{}", "╠══════════════════════════════════════════════════════════════════════════╣".bright_yellow());
    println!("{}", "║  Substitutions (no Enochian glyph):                                      ║".italic().dimmed());
    println!("{}", "║    J → I (Gon, ord=9, GD=10)                                             ║".italic().dimmed());
    println!("{}", "║    K → C (Veh, ord=3, GD= 3)                                             ║".italic().dimmed());
    println!("{}", "║    W → V (Van, ord=19, GD= 6)                                            ║".italic().dimmed());
    println!("{}", "║  Note: Dee's originals contain NO explicit gematria table.               ║".italic().dimmed());
    println!("{}", "║  The Ordinal system (pos. 1-21) is most defensible; the GD values        ║".italic().dimmed());
    println!("{}", "║  are a 19th-century Golden Dawn retrofitting (Mathers/Regardie).         ║".italic().dimmed());
    println!("{}", "╚══════════════════════════════════════════════════════════════════════════╝".bright_yellow());
    println!();
}

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
