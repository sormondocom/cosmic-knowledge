//! Celestial Numerology Analyzer — binary entry point.
//!
//! This file is intentionally thin: it owns only the top-level application
//! flow (argument parsing, session loops, sub-menu dispatch).  All domain
//! logic lives in the library modules declared below.

// ─── Module declarations ──────────────────────────────────────────────────────
mod audio;
mod cosmology;
mod enochian;
mod numerology;
mod print;
mod reports;
mod ui;

// ─── Selective imports ────────────────────────────────────────────────────────
use std::io::{self, Write};

use colored::*;

use audio::{
    change_frequency, create_custom_binaural, export_all_frequencies,
    export_all_frequencies_cli, export_frequency, get_frequency_for_number,
    get_frequency_name, initialize_audio, start_ambient_frequency, stop_audio,
    AudioSystem, SOLFEGGIO_FREQUENCIES,
};
use enochian::{
    aethyr_lookup, enochian_angelic_message, enochian_lookup, enochian_meaning,
    enochian_substitute, show_aethyr_info, show_aethyr_table, show_enochian_table,
};
use numerology::{
    abjad_meaning, angelic_message, check_special_sequences, digital_root,
    get_calculation_breakdown, isopsephy_meaning, master_numbers_message, meaning_of, numerology,
};
use print::{print_numerology_report, export_pdf_report};
use reports::{
    build_enochian_gematria_report, build_enochian_translation_report,
    build_numerology_report, prompt_and_export,
};
use cosmology::run_world_systems_session;
use ui::{print_angel_banner, show_help, show_loading_screen, show_main_menu, MainMode};

// ─── Entry point ──────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let skip_loading = args.contains(&"--fast".to_string())    || args.contains(&"-f".to_string());
    let enable_audio = !args.contains(&"--silent".to_string()) && !args.contains(&"-s".to_string());

    // ── CLI commands (non-interactive) ────────────────────────────────────────
    if args.contains(&"--export-all".to_string()) {
        println!("{}", "🎵 Exporting all Solfeggio frequencies…".bold().bright_magenta());
        export_all_frequencies_cli();
        return;
    }

    if let Some(pos) = args.iter().position(|a| a == "--aethyr") {
        if let Some(query) = args.get(pos + 1) {
            show_aethyr_info(query);
        } else {
            show_aethyr_table();
        }
        return;
    }

    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        show_help();
        return;
    }

    // ── Audio initialisation ──────────────────────────────────────────────────
    let audio_system: Option<AudioSystem> = if enable_audio {
        match initialize_audio() {
            Ok(sys) => {
                println!("{}", "🎵 Sacred frequencies activated".dimmed());
                Some(sys)
            }
            Err(_) => {
                println!("{}", "⚠️  Audio unavailable — continuing in silent mode".yellow());
                None
            }
        }
    } else {
        None
    };

    if !skip_loading {
        show_loading_screen();
    }

    if let Some(ref sys) = audio_system {
        start_ambient_frequency(sys, 432.0);
        println!("{}", "🎶 Ambient frequency: 432 Hz (Universal Harmony)".dimmed());
    }

    print_angel_banner();

    // ── Main menu loop ────────────────────────────────────────────────────────
    loop {
        match show_main_menu() {
            MainMode::Numerology  => run_numerology_session(&audio_system),
            MainMode::Enochian    => run_enochian_session(),
            MainMode::WorldSystems => run_world_systems_session(),
            MainMode::Frequencies => {
                if audio_system.is_some() {
                    show_export_menu();
                } else {
                    println!("{}", "⚠️  Audio unavailable — run without --silent to enable exports.".yellow());
                }
            }
            MainMode::Help => show_help(),
            MainMode::Quit => {
                if let Some(ref sys) = audio_system {
                    stop_audio(sys);
                }
                println!("{}", "\n✨ Farewell, child of light and logic. ✨\n".italic().bright_magenta());
                break;
            }
        }
    }
}

// ─── Numerology session ───────────────────────────────────────────────────────

fn run_numerology_session(audio_system: &Option<AudioSystem>) {
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════╗".bright_white());
    println!("{}", "║         🔢  GEMATRIA & NUMEROLOGY SESSION                ║".bold().bright_white());
    println!("{}", "╠══════════════════════════════════════════════════════════╣".bright_white());
    println!("{}", "║  Hebrew · Pythagorean · Chaldean · Greek Isopsephy       ║".dimmed());
    println!("{}", "║  Agrippan · Simple Ordinal · Reverse Ordinal · Abjad     ║".dimmed());
    println!("{}", "║  Enochian Ordinal · Enochian G.D.                        ║".dimmed());
    println!("{}", "╚══════════════════════════════════════════════════════════╝".bright_white());
    println!();

    let active_systems = choose_numerology_systems();
    println!();

    if audio_system.is_some() {
        println!("{}", "🎵 Frequencies will attune to each word's root number.".italic().bright_blue());
    }
    println!("{}", "   (empty line → back to main menu)\n".dimmed());

    loop {
        print!("{}", "▸ Enter word or phrase: ".bold().cyan());
        io::stdout().flush().unwrap_or(());

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { break; }
        let input = input.trim();
        if input.is_empty() { break; }

        match input.to_lowercase().as_str() {
            "systems" => {
                println!("{}", "  Active systems:".bold());
                for s in &active_systems { println!("    • {}", s.bright_white()); }
                println!("{}", "  (restart session from main menu to change)".dimmed());
                continue;
            }
            "help" => { show_help(); continue; }
            _ => {}
        }

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
        let results: Vec<_> = all_results
            .iter()
            .filter(|(name, _)| active_systems.iter().any(|s| s == name))
            .collect();

        let first_root = results.first().map(|(_, (_, r))| *r);

        println!("\n{}", format!("  Results for '{}':", word).bold().green());
        println!("{}", "  ─".repeat(23).dimmed());

        let mut special_patterns: Vec<(&&str, &str)> = Vec::new();

        for (system, (total, root)) in &results {
            let is_enochian = system.starts_with("Enochian");
            let meaning = match *system {
                s if s.starts_with("Enochian") => enochian_meaning(*root),
                "Greek Isopsephy"              => isopsephy_meaning(*root),
                "Abjad"                        => abjad_meaning(*root),
                _                              => meaning_of(*root),
            };
            let breakdown   = get_calculation_breakdown(&word, system);

            println!(
                "  {} {:>5} → root {}: {}",
                format!("{:<18}", system).bold(),
                total.to_string().blue(),
                root.to_string().magenta(),
                meaning
            );
            if !breakdown.is_empty() {
                println!("      {}", format!("Calculation: {}", breakdown).dimmed());
            }
            if is_enochian {
                let (an, aname, adesc) = aethyr_lookup(*total);
                println!("      {}",
                    format!("⟁ Aethyr {:>2} ({}) — {}", an, aname, adesc)
                        .italic().bright_cyan());
            }
            if !is_enochian {
                if let Some(msg) = master_numbers_message(*total) {
                    println!("      {}", msg.bright_yellow());
                }
            }
            if let Some(msg) = check_special_sequences(*total) {
                special_patterns.push((system, msg));
            }
        }

        if let Some(root) = first_root {
            println!("\n  {}", "💫 Angelic Message:".bold().bright_cyan());
            println!("  {}", format!("   {}", angelic_message(root)).italic().bright_white());
        }

        if active_systems.iter().any(|s| *s == "Enochian Ordinal") {
            if let Some((_, (_, er))) = all_results.iter().find(|(s, _)| *s == "Enochian Ordinal") {
                println!("\n  {}", "📜 Enochian Call (John Dee):".bold().yellow());
                println!("  {}", format!("   {}", enochian_angelic_message(*er)).italic().bright_yellow());
            }
        }

        for (system, msg) in special_patterns {
            println!("\n  {}", format!("🌈 Special Pattern in {}: {}", system, msg).bright_magenta());
        }

        if let (Some(ref sys), Some(root)) = (audio_system, first_root) {
            let freq  = get_frequency_for_number(root);
            let fname = get_frequency_name(freq);
            change_frequency(sys, freq);
            println!("\n  {}", format!("🎵 Frequency attuned to: {} Hz ({})", freq, fname).italic().bright_magenta());
        }

        println!("{}", "  ─".repeat(23).dimmed());

        print!("{}", "  ▸ Save (s), Print (p), PDF (d), or skip (Enter):".cyan());
        io::stdout().flush().unwrap_or(());
        let mut save_choice = String::new();
        if io::stdin().read_line(&mut save_choice).is_err() { continue; }
        match save_choice.trim().to_lowercase().as_str() {
            "s" | "save" | "y" | "yes" => {
                let report = build_numerology_report(&word, &active_systems);
                let stem   = format!("numerology_{}", word.to_lowercase());
                prompt_and_export(&stem, &report);
            }
            "p" | "print" => {
                print_numerology_report(&word, &active_systems);
            }
            "d" | "pdf" => {
                export_pdf_report(&word, &active_systems);
            }
            _ => {}
        }
        println!();
    }
}

fn choose_numerology_systems() -> Vec<&'static str> {
    const ALL: &[&str] = &[
        "Hebrew Gematria",
        "Pythagorean",
        "Chaldean",
        "Greek Isopsephy",
        "Agrippan",
        "Simple Ordinal",
        "Reverse Ordinal",
        "Abjad",
        "Enochian Ordinal",
        "Enochian G.D.",
    ];

    println!("{}", "  Select systems to include in this session:".bold());
    println!("{}", "  ─────────────────────────────────────────".dimmed());
    for (i, name) in ALL.iter().enumerate() {
        println!("    {}  {}", format!("{}.", i + 1).cyan(), name.bright_white());
    }
    println!("    {}  {}", "A.".cyan(), "All systems (default)".bright_white());
    println!();
    print!("{}", "  ▸ Enter numbers separated by spaces (e.g. 1 3 4) or A for all: ".cyan());
    io::stdout().flush().unwrap_or(());

    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_err() {
        return ALL.to_vec();
    }
    let line = line.trim().to_uppercase();

    if line.is_empty() || line.contains('A') {
        println!("{}", "  ✓ All five systems selected.".bright_green());
        return ALL.to_vec();
    }

    let mut selected: Vec<&'static str> = line
        .split_whitespace()
        .filter_map(|tok| tok.parse::<usize>().ok())
        .filter(|&n| n >= 1 && n <= ALL.len())
        .map(|n| ALL[n - 1])
        .collect();

    // Deduplicate while preserving order
    let mut seen = std::collections::HashSet::new();
    selected.retain(|s| seen.insert(*s));

    if selected.is_empty() {
        println!("{}", "  No valid selection — defaulting to all systems.".yellow());
        ALL.to_vec()
    } else {
        println!("  {}", format!("✓ {} system(s) selected.", selected.len()).bright_green());
        selected
    }
}

// ─── Enochian session ─────────────────────────────────────────────────────────

fn run_enochian_session() {
    loop {
        println!();
        println!("{}", "╔══════════════════════════════════════════════════════════╗".bright_cyan());
        println!("{}", "║           📜  ENOCHIAN ANGELOLOGY  (John Dee)            ║".bold().bright_cyan());
        println!("{}", "╠══════════════════════════════════════════════════════════╣".bright_cyan());
        println!("{}", "║                                                          ║".bright_cyan());
        println!("{}", "║   1.  🔤  Enochian Alphabet & Gematria Table             ║".bright_white());
        println!("{}", "║          All 21 letters · Ordinal & GD values            ║".dimmed());
        println!("{}", "║                                                          ║".bright_cyan());
        println!("{}", "║   2.  🌌  Browse the 30 Aethyrs                          ║".bright_white());
        println!("{}", "║          Full table of all celestial realms              ║".dimmed());
        println!("{}", "║                                                          ║".bright_cyan());
        println!("{}", "║   3.  🔍  Look Up a Specific Aethyr                      ║".bright_white());
        println!("{}", "║          Search by number (1-30) or name (TEX, LIL…)    ║".dimmed());
        println!("{}", "║                                                          ║".bright_cyan());
        println!("{}", "║   4.  🔤  Translate Word into Enochian                   ║".bright_white());
        println!("{}", "║          Letter-by-letter Enochian name rendering        ║".dimmed());
        println!("{}", "║                                                          ║".bright_cyan());
        println!("{}", "║   5.  🗝️   Browse the Enochian Keys (Calls)              ║".bright_white());
        println!("{}", "║          The 19 angelic calls as recorded by Dee         ║".dimmed());
        println!("{}", "║                                                          ║".bright_cyan());
        println!("{}", "║   6.  🔢  Enochian Gematria — Analyze a Word             ║".bright_white());
        println!("{}", "║          Ordinal + G.D. values with Aethyr resonance     ║".dimmed());
        println!("{}", "║                                                          ║".bright_cyan());
        println!("{}", "║   0.  ←  Back to Main Menu                              ║".dimmed());
        println!("{}", "║                                                          ║".bright_cyan());
        println!("{}", "╚══════════════════════════════════════════════════════════╝".bright_cyan());
        println!();

        print!("{}", "▸ Enter choice: ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { break; }

        match input.trim() {
            "1" => show_enochian_table(),
            "2" => show_aethyr_table(),
            "3" => {
                print!("{}", "\n  ▸ Enter Aethyr number (1-30) or name (e.g. ZAX): ".cyan());
                io::stdout().flush().unwrap_or(());
                let mut q = String::new();
                if io::stdin().read_line(&mut q).is_ok() && !q.trim().is_empty() {
                    show_aethyr_info(q.trim());
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
                    format!("║   {:<5}  {:<16} {:<6}    ---       -----      ║", ch, "(no glyph)", "")
                    .dimmed());
            }
        }
    }

    println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_cyan());
    println!("  {}",
        format!("║  Ordinal total: {:>5}   →  root {}                      ║",
            ordinal_total, digital_root(ordinal_total))
        .bold().bright_yellow());
    println!("  {}",
        format!("║  G.D. total:    {:>5}   →  root {}                      ║",
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

    print!("{}", "  ▸ Save (s), Print (p), PDF (d), or skip (Enter):".cyan());
    io::stdout().flush().unwrap_or(());
    let mut save_choice = String::new();
    if io::stdin().read_line(&mut save_choice).is_err() { return; }
    match save_choice.trim().to_lowercase().as_str() {
        "s" | "save" | "y" | "yes" => {
            let report = build_enochian_translation_report(&word);
            let stem   = format!("enochian_translation_{}", word.to_lowercase());
            prompt_and_export(&stem, &report);
        }
        "p" | "print" => {
            // For Enochian translations, print the full numerology report over Enochian systems.
            let enochian_systems = &["Enochian Ordinal", "Enochian G.D."];
            print_numerology_report(&word, enochian_systems);
        }
        "d" | "pdf" => {
            let enochian_systems = &["Enochian Ordinal", "Enochian G.D."];
            export_pdf_report(&word, enochian_systems);
        }
        _ => {}
    }
}

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

        print!("{}", "  ▸ Save this result to a file? (y/n): ".cyan());
        io::stdout().flush().unwrap_or(());
        let mut save_choice = String::new();
        if io::stdin().read_line(&mut save_choice).is_err() { continue; }
        if save_choice.trim().eq_ignore_ascii_case("y") {
            let report = build_enochian_gematria_report(&word);
            let stem   = format!("enochian_gematria_{}", word.to_lowercase());
            prompt_and_export(&stem, &report);
        }
        println!();
    }
}

fn browse_enochian_keys() {
    // The 19 Enochian Keys / Calls — opening lines as recorded by Dee/Kelley.
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
        println!("{}", "  ║      🗝️  THE 19 ENOCHIAN KEYS (CALLS) — John Dee      ║".bold().bright_yellow());
        println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_yellow());
        for (num, title, _, _) in KEYS {
            println!("  {}", format!("║   {:>2}.  {:<49}║", num, title).bright_white());
        }
        println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_yellow());
        println!("{}", "  ║    0.  ← Back                                         ║".dimmed());
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
                println!("  {}", format!("║  🗝️  Key {:>2} — {}  ║", num, title).bold().bright_cyan());
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
            } else {
                println!("{}", "  Enter a number between 1 and 19.".yellow());
            }
        } else {
            println!("{}", "  Enter a number between 1 and 19.".yellow());
        }
    }
}

// ─── Export / frequency menu ──────────────────────────────────────────────────

fn show_export_menu() {
    println!("\n{}", "╔════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║             🎵 FREQUENCY EXPORT 🎵             ║".bright_cyan());
    println!("{}", "╚════════════════════════════════════════════════╝".bright_cyan());
    println!();
    println!("{}", "Choose frequencies to export as binaural beats:".bold());
    println!();

    for (i, (freq, _, icon)) in SOLFEGGIO_FREQUENCIES.iter().enumerate() {
        println!("{} {} Hz — {} {}",
            format!("{}.", i + 1).cyan(),
            (*freq as u32).to_string().bright_blue(),
            get_frequency_name(*freq).bright_white(),
            icon,
        );
    }

    println!();
    println!("{}", "10. Export All Solfeggio Frequencies".bright_magenta());
    println!("{}", "11. Create Custom Binaural Beat".bright_yellow());
    println!("{}", "0.  Return to main menu".dimmed());

    print!("\n{}", "Enter choice (0-11): ".cyan());
    io::stdout().flush().unwrap_or(());

    let mut choice = String::new();
    if io::stdin().read_line(&mut choice).is_err() { return; }

    match choice.trim() {
        "10"     => export_all_frequencies(),
        "11"     => create_custom_binaural(),
        "0" | "" => {}
        chosen   => {
            if let Ok(n) = chosen.parse::<usize>() {
                if n >= 1 && n <= SOLFEGGIO_FREQUENCIES.len() {
                    let (freq, name, _) = SOLFEGGIO_FREQUENCIES[n - 1];
                    export_frequency(freq, name);
                    return;
                }
            }
            println!("{}", "Invalid choice. Returning to main menu.".yellow());
        }
    }
}
