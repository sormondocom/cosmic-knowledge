//! Interactive Enochian session — all terminal I/O for the Enochian sub-menu.
//!
//! Functions:
//!  - [`run_enochian_session`]  — sub-menu loop
//!  - `enochian_translate_word` — letter-by-letter rendering
//!  - `enochian_gematria_lookup`— Ordinal + G.D. analysis loop
//!  - `browse_enochian_keys`    — 19-key browser

use std::io::{self, Write};
use colored::*;

use crate::export::{self, handle_export, prompt_export_format, export_text, export_html, export_pdf, ExportChoice};
use crate::menu::{Menu, MenuItem, MenuColor};
use crate::numerology::{digital_root, numerology, get_calculation_breakdown};
use crate::reports::{
    build_enochian_alphabet_report, build_aethyr_table_report,
    build_aethyr_info_report, build_enochian_key_report,
    build_enochian_translation_report, build_enochian_gematria_report,
};

use super::{
    aethyr_lookup, enochian_angelic_message, enochian_lookup, enochian_meaning,
    enochian_substitute, show_aethyr_info, show_aethyr_table, show_enochian_table,
};

// ─── Sub-menu definition ──────────────────────────────────────────────────────

static ENOCHIAN_ITEMS: &[MenuItem] = &[
    MenuItem { key: "1", icon: "🔤", label: "Enochian Alphabet & Gematria Table",
               hint: "All 21 letters · Ordinal & GD values" },
    MenuItem { key: "2", icon: "🌌", label: "Browse the 30 Aethyrs",
               hint: "Full table of all celestial realms" },
    MenuItem { key: "3", icon: "🔍", label: "Look Up a Specific Aethyr",
               hint: "Search by number (1-30) or name (TEX, LIL...)" },
    MenuItem { key: "4", icon: "🔤", label: "Translate Word into Enochian",
               hint: "Letter-by-letter Enochian name rendering" },
    MenuItem { key: "5", icon: "🗝", label: "Browse the Enochian Keys (Calls)",
               hint: "The 19 angelic calls as recorded by Dee" },
    MenuItem { key: "6", icon: "🔢", label: "Enochian Gematria — Analyze a Word",
               hint: "Ordinal + G.D. values with Aethyr resonance" },
];

static ENOCHIAN_MENU: Menu = Menu {
    title:        "📜  ENOCHIAN ANGELOLOGY  (John Dee)",
    border_color: MenuColor::Cyan,
    items:        ENOCHIAN_ITEMS,
    back_key:     "0",
    back_label:   "Back to Main Menu",
};

// ─── Session entry point ──────────────────────────────────────────────────────

/// Run the Enochian sub-menu loop until the user exits.
pub fn run_enochian_session() {
    loop {
        match ENOCHIAN_MENU.show_and_read().as_str() {
            "1" => {
                show_enochian_table();
                handle_export(
                    "enochian_alphabet",
                    || build_enochian_alphabet_report(),
                    || export::build_enochian_alphabet_html(),
                );
            }
            "2" => {
                show_aethyr_table();
                handle_export(
                    "aethyr_table",
                    || build_aethyr_table_report(),
                    || export::build_aethyr_table_html(),
                );
            }
            "3" => {
                print!("{}", "\n  ▸ Enter Aethyr number (1-30) or name (e.g. ZAX): ".cyan());
                io::stdout().flush().unwrap_or(());
                let mut q = String::new();
                if io::stdin().read_line(&mut q).is_ok() && !q.trim().is_empty() {
                    let qt = q.trim().to_string();
                    show_aethyr_info(&qt);
                    handle_export(
                        &format!("aethyr_{}", qt.to_lowercase()),
                        || build_aethyr_info_report(&qt),
                        || export::build_aethyr_info_html(&qt),
                    );
                }
            }
            "4" => enochian_translate_word(),
            "5" => browse_enochian_keys(),
            "6" => enochian_gematria_lookup(),
            "0" | "" => break,
            _ => println!("{}", "  Please enter 0–6.".yellow()),
        }
    }
}

// ─── Letter-by-letter translation ────────────────────────────────────────────

fn enochian_translate_word() {
    println!();
    print!("{}", "  ▸ Enter a word or phrase to render in Enochian letter names: ".cyan());
    io::stdout().flush().unwrap_or(());
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() { return; }
    let input = input.trim();
    if input.is_empty() { return; }

    let word: String = input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .flat_map(|c| c.to_uppercase())
        .collect();

    if word.is_empty() {
        println!("{}", "  ⚠️  No alphabetic characters found.".yellow());
        return;
    }

    println!();
    println!("{}", "  ╔═══════════════════════════════════════════════════════╗".bright_cyan());
    println!("  {}", format!("║  🔤  Enochian rendering of  '{}'", word).bold().bright_cyan());
    println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_cyan());
    println!("{}", "  ║  Char   Enochian Name   Sub?   Ordinal   GD Value     ║".bold().bright_white());
    println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_cyan());

    let mut ordinal_total = 0u32;
    let mut gd_total      = 0u32;

    for ch in word.chars() {
        let sub        = enochian_substitute(ch);
        let sub_marker = if sub != ch { format!("→{}", sub) } else { "   ".to_string() };
        match enochian_lookup(ch) {
            Some((name, ord, gd)) => {
                ordinal_total += ord;
                gd_total      += gd;
                println!("  {}",
                    format!("║   {:<5}  {:<16} {:<6}    {:>3}       {:>5}      ║",
                        ch, name, sub_marker, ord, gd)
                    .bright_white());
            }
            None => {
                println!("  {}",
                    format!("║   {:<5}  {:<16} {:<6}    ---       -----      ║",
                        ch, "(no glyph)", "")
                    .dimmed());
            }
        }
    }

    println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_cyan());
    println!("  {}",
        format!("║  Ordinal total: {:>5}   ->  root {}                      ║",
            ordinal_total, digital_root(ordinal_total))
        .bold().bright_yellow());
    println!("  {}",
        format!("║  G.D. total:    {:>5}   ->  root {}                      ║",
            gd_total, digital_root(gd_total))
        .bold().bright_yellow());

    let (oa, on, od) = aethyr_lookup(ordinal_total);
    let (ga, gn, gd) = aethyr_lookup(gd_total);
    println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_cyan());
    println!("  {}",
        format!("║  Ordinal Aethyr: {:>2} ({})                               ║", oa, on)
        .italic().bright_cyan());
    let ds: String = od.chars().take(46).collect();
    println!("  {}", format!("║    {}  ║", format!("{:<49}", ds)).dimmed());
    println!("  {}",
        format!("║  G.D. Aethyr:    {:>2} ({})                               ║", ga, gn)
        .italic().bright_cyan());
    let ds2: String = gd.chars().take(46).collect();
    println!("  {}", format!("║    {}  ║", format!("{:<49}", ds2)).dimmed());
    println!("{}", "  ╚═══════════════════════════════════════════════════════╝".bright_cyan());

    let root = digital_root(ordinal_total);
    println!();
    println!("  {}", "📜 Enochian Call Fragment:".bold().yellow());
    println!("  {}", format!("   {}", enochian_angelic_message(root)).italic().bright_yellow());
    println!();

    let stem            = format!("enochian_translation_{}", word.to_lowercase());
    let enochian_systems: &[&str] = &["Enochian Ordinal", "Enochian G.D."];
    match prompt_export_format() {
        ExportChoice::Text => export_text(&stem, &build_enochian_translation_report(&word)),
        ExportChoice::Html => export_html(&stem, &export::build_numerology_html(&word, enochian_systems)),
        ExportChoice::Pdf  => export_pdf(&stem, &build_enochian_translation_report(&word)),
        ExportChoice::Skip => {}
    }
}

// ─── Gematria analysis loop ───────────────────────────────────────────────────

fn enochian_gematria_lookup() {
    println!();
    println!("{}", "  ─────────────────────────────────────────────────────────".dimmed());
    println!("{}", "  🔢  Enochian Gematria — Analyze a Word".bold().bright_white());
    println!("{}", "  ─────────────────────────────────────────────────────────".dimmed());
    println!("{}", "  (empty line → back)".dimmed());
    println!();

    loop {
        print!("{}", "  ▸ Enter word or phrase: ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { break; }
        let input = input.trim();
        if input.is_empty() { break; }

        let word: String = input
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .flat_map(|c| c.to_uppercase())
            .collect();

        if word.is_empty() {
            println!("{}", "  ⚠️  No alphabetic characters found.".yellow());
            continue;
        }

        let all_results = numerology(&word);
        println!("\n  {}", format!("Enochian results for '{}':", word).bold().green());
        println!("{}", "  ─────────────────────────────────────────".dimmed());

        for (system, (total, root)) in all_results.iter().filter(|(s, _)| s.starts_with("Enochian")) {
            let breakdown          = get_calculation_breakdown(&word, system);
            let (an, aname, adesc) = aethyr_lookup(*total);
            println!(
                "  {} {:>5} → root {}: {}",
                format!("{:<16}", system).bold(),
                total.to_string().blue(),
                root.to_string().magenta(),
                enochian_meaning(*root)
            );
            if !breakdown.is_empty() {
                println!("      {}", format!("Calculation: {}", breakdown).dimmed());
            }
            println!("      {}",
                format!("⟁ Aethyr {:>2} ({}) — {}", an, aname, adesc).italic().bright_cyan());
        }

        let enochian_root = all_results
            .iter()
            .find(|(s, _)| *s == "Enochian Ordinal")
            .map(|(_, (_, r))| *r)
            .unwrap_or(1);

        println!("\n  {}", "📜 Enochian Call:".bold().yellow());
        println!("  {}", format!("   {}", enochian_angelic_message(enochian_root)).italic().bright_yellow());
        println!("{}", "  ─────────────────────────────────────────".dimmed());

        let stem            = format!("enochian_gematria_{}", word.to_lowercase());
        let enochian_systems: &[&str] = &["Enochian Ordinal", "Enochian G.D."];
        match prompt_export_format() {
            ExportChoice::Text => export_text(&stem, &build_enochian_gematria_report(&word)),
            ExportChoice::Html => export_html(&stem, &export::build_numerology_html(&word, enochian_systems)),
            ExportChoice::Pdf  => export_pdf(&stem, &build_enochian_gematria_report(&word)),
            ExportChoice::Skip => {}
        }
        println!();
    }
}

// ─── Enochian Keys browser ────────────────────────────────────────────────────

fn browse_enochian_keys() {
    // Source: Dee's diaries (Cotton MS Appendix XLVI); Crowley's Liber Chanokh (1909).
    const KEYS: &[(u32, &str, &str, &str)] = &[
        ( 1, "The First Key",     "MICMA GOHO MAD ZIR",
          "I reign over you, saith the God of Justice. In power exalted above the firmaments of wrath; in whose hands the Sun is as a sword and the Moon as a through-thrusting fire." ),
        ( 2, "The Second Key",    "ADGT VPAAH ZONG OM",
          "Can the wings of the winds understand your voices of wonder? O you the second flame, the house of justice, who art mightier than the evening wolves." ),
        ( 3, "The Third Key",     "MICAOLI BERANUSAJI",
          "Behold, saith your God, I am a circle on whose hands stand twelve kingdoms. Six are the seats of living breath; the rest are as sharp sickles, or the horns of death." ),
        ( 4, "The Fourth Key",    "OTHIL LASDI BABAGE",
          "I have set my feet in the South, and have looked about me, saying: Are not the thunders of increase numbered 33 which reign in the second angle?" ),
        ( 5, "The Fifth Key",     "SAPAH ZIMII DU-I-BE",
          "The mighty sounds have entered into the third angle, and are become as olives in the olive mount, looking with gladness upon the earth." ),
        ( 6, "The Sixth Key",     "GAHE SADiv FIEN",
          "The spirits of the fourth angle are nine, mighty in the firmament of waters; whom the first hath planted a torment to the wicked and a garland to the righteous." ),
        ( 7, "The Seventh Key",   "RAAS I SALMAN",
          "The East is a house of virgins singing praises among the flames of first glory, wherein the Lord hath opened his mouth; and they are become 28 living dwellings." ),
        ( 8, "The Eighth Key",    "BAZMELO I TA PIRIPSON",
          "The midday, the first, is as the third heaven made of hyacinth pillars 26; in whom the second beginning of things are and wax strong, which also successively are the number of time." ),
        ( 9, "The Ninth Key",     "MICAOLI BRANSG PIAD",
          "A mighty guard of fire with two-edged swords flaming (which have vials 8 of wrath for two times and a half, whose wings are of wormwood and of the marrow of salt)." ),
        (10, "The Tenth Key",     "CORAXO CHIS CORMP",
          "The thunders of judgment and wrath are numbered and are harboured in the North, in the likeness of an oak whose branches are nests of lamentation and weeping." ),
        (11, "The Eleventh Key",  "OXIAYAL HOLDO",
          "The mighty seat groaned and they were five thunders which flew into the East; and the Eagle spake and cried aloud: Come away from the house of death." ),
        (12, "The Twelfth Key",   "NONCI DSONF BABAGE",
          "O you that reign in the South, and are 28; the lanterns of sorrow — bind up your girdles and visit us! Bring down your train 3663 that the Lord may be magnified." ),
        (13, "The Thirteenth Key","NAPEAI BABAGEN",
          "O you swords of the South which have 42 eyes to stir up the wrath of sin, making men drunken which are empty: Behold the Promise of God and his power." ),
        (14, "The Fourteenth Key","NOROMI BAGLE",
          "O you sons of fury, the daughters of the Just One! That sit upon 24 seats, vexing all creatures of the Earth with age — that have under you 1636." ),
        (15, "The Fifteenth Key", "ILS TABAAN LIXIPSP",
          "O thou, the governor of the first flame, under whose wings are 6739; that weave the Earth with dryness; which knowest the great name Righteousness." ),
        (16, "The Sixteenth Key", "ILS VIVIALPRT SALMAN",
          "O thou second flame, the house of justice, which hast thy beginning in glory and shalt comfort the just — which walkest on the earth with feet 8763." ),
        (17, "The Seventeenth Key","ILS DIAL PEOC",
          "O thou third flame, whose wings are thorns to stir up vexation, and who hast 7336 living lamps going before thee — whose God is wrath in anger." ),
        (18, "The Eighteenth Key","ILS MICAOLI CHIS",
          "O thou mighty light and burning flame of comfort, that unveilest the glory of God to the centre of the Earth, in whom the secrets of truth have their abiding." ),
        (19, "The Nineteenth Key (The Aethyr Call)", "MADRIAAX DS PRAF",
          "O you heavens which dwell in the first air, ye are mighty in the parts of the earth and execute the judgment of the highest! To you it is said: Behold the face of your God." ),
    ];

    loop {
        println!();
        println!("{}", "  ╔═══════════════════════════════════════════════════════╗".bright_yellow());
        println!("{}", "  ║      🗝  THE 19 ENOCHIAN KEYS (CALLS) — John Dee      ║".bold().bright_yellow());
        println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_yellow());
        for (num, title, _, _) in KEYS {
            println!("  {}", format!("║   {:>2}.  {:<49}║", num, title).bright_white());
        }
        println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_yellow());
        println!("{}", "  ║    0.  <- Back                                        ║".dimmed());
        println!("{}", "  ╚═══════════════════════════════════════════════════════╝".bright_yellow());
        println!();

        print!("{}", "  ▸ Enter key number (1-19) to read, or 0 to go back: ".cyan());
        io::stdout().flush().unwrap_or(());
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { break; }
        let trimmed = input.trim();
        if trimmed == "0" || trimmed.is_empty() { break; }

        if let Ok(n) = trimmed.parse::<usize>() {
            if n >= 1 && n <= KEYS.len() {
                let (num, title, opening, body) = KEYS[n - 1];
                println!();
                println!("{}", "  ╔═══════════════════════════════════════════════════════╗".bright_cyan());
                println!("  {}", format!("║  🗝  Key {:>2} — {}  ║", num, title).bold().bright_cyan());
                println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_cyan());
                println!("  {}", "║  Opening words (Enochian):                            ║".dimmed());
                println!("  {}", format!("║    {:<51}║", opening).italic().bright_magenta());
                println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_cyan());
                println!("  {}", "║  Translation / Dee's English rendering:               ║".dimmed());

                let words: Vec<&str> = body.split_whitespace().collect();
                let mut line = String::new();
                for word in &words {
                    if line.len() + word.len() + 1 > 50 {
                        println!("  {}", format!("║  {:<53}║", line).bright_white());
                        line = word.to_string();
                    } else {
                        if !line.is_empty() { line.push(' '); }
                        line.push_str(word);
                    }
                }
                if !line.is_empty() {
                    println!("  {}", format!("║  {:<53}║", line).bright_white());
                }

                let root = ((num - 1) % 9 + 1) as u32;
                println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_cyan());
                println!("  {}", format!("║  Enochian meaning for root {}:                         ║", root).italic().dimmed());
                let em_short: String = enochian_meaning(root).chars().take(51).collect();
                println!("  {}", format!("║    {:<51}║", em_short).italic().bright_yellow());
                println!("{}", "  ╚═══════════════════════════════════════════════════════╝".bright_cyan());
                println!();
                let key_stem = format!("enochian_key_{}", num);
                handle_export(
                    &key_stem,
                    || build_enochian_key_report(num, title, opening, body),
                    || export::build_enochian_key_html(num, title, opening, body),
                );
            } else {
                println!("{}", "  Enter a number between 1 and 19.".yellow());
            }
        } else {
            println!("{}", "  Enter a number between 1 and 19.".yellow());
        }
    }
}
