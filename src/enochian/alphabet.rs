//! Enochian alphabet — letter table, substitution rules, and lookup helpers.

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

// ─── Display routine ─────────────────────────────────────────────────────────

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

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── enochian_substitute ───────────────────────────────────────────────────

    #[test]
    fn substitute_j_to_i() { assert_eq!(enochian_substitute('J'), 'I'); }

    #[test]
    fn substitute_k_to_c() { assert_eq!(enochian_substitute('K'), 'C'); }

    #[test]
    fn substitute_w_to_v() { assert_eq!(enochian_substitute('W'), 'V'); }

    #[test]
    fn substitute_others_unchanged() {
        for c in ('A'..='Z').filter(|&c| c != 'J' && c != 'K' && c != 'W') {
            assert_eq!(enochian_substitute(c), c);
        }
    }

    // ── enochian_lookup ───────────────────────────────────────────────────────

    #[test]
    fn lookup_a_returns_un() {
        let (name, ord, gd) = enochian_lookup('A').unwrap();
        assert_eq!(name, "Un");
        assert_eq!(ord, 1);
        assert_eq!(gd, 1);
    }

    #[test]
    fn lookup_z_returns_ceph() {
        let (name, ord, _) = enochian_lookup('Z').unwrap();
        assert_eq!(name, "Ceph");
        assert_eq!(ord, 21);
    }

    #[test]
    fn lookup_j_via_substitution() {
        // J → I (Gon)
        let (name, ord, _) = enochian_lookup('J').unwrap();
        assert_eq!(name, "Gon");
        assert_eq!(ord, 9);
    }

    #[test]
    fn lookup_returns_none_for_digit() {
        assert!(enochian_lookup('1').is_none());
    }

    // ── coverage sanity ───────────────────────────────────────────────────────

    #[test]
    fn all_21_letters_have_lookups() {
        // Every letter in ENOCHIAN_LETTERS must be reachable via enochian_lookup.
        for (_, chars, _, _) in ENOCHIAN_LETTERS {
            for &c in *chars {
                assert!(
                    enochian_lookup(c).is_some(),
                    "enochian_lookup({}) returned None", c
                );
            }
        }
    }
}
