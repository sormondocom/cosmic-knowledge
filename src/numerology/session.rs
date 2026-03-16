//! Interactive numerology session — all terminal I/O for the numerology sub-menu.
//!
//! Functions:
//!  - [`run_numerology_session`] — main word-analysis loop
//!  - `choose_numerology_systems` — system-selection prompt

use std::io::{self, Write};
use colored::*;

use crate::audio::{AudioSystem, change_frequency, get_frequency_for_number, get_frequency_name};
use crate::enochian::{aethyr_lookup, enochian_angelic_message, enochian_meaning};
use crate::export::{build_numerology_html, export_html, export_pdf, export_text,
                    prompt_export_format, ExportChoice};
use crate::menu::show_help;
use crate::reports::build_numerology_report;

use super::{
    abjad_meaning, angelic_message, check_special_sequences,
    get_calculation_breakdown, isopsephy_meaning, master_numbers_message,
    meaning_of, numerology,
};

// ─── Session entry point ──────────────────────────────────────────────────────

/// Run the numerology word-analysis session until the user exits.
pub fn run_numerology_session(audio_system: &Option<AudioSystem>) {
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
            let breakdown = get_calculation_breakdown(&word, system);

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

        let stem = format!("numerology_{}", word.to_lowercase());
        match prompt_export_format() {
            ExportChoice::Text => export_text(&stem, &build_numerology_report(&word, &active_systems)),
            ExportChoice::Html => export_html(&stem, &build_numerology_html(&word, &active_systems)),
            ExportChoice::Pdf  => export_pdf(&stem, &build_numerology_report(&word, &active_systems)),
            ExportChoice::Skip => {}
        }
        println!();
    }
}

// ─── System selector ─────────────────────────────────────────────────────────

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
        println!("{}", "  ✓ All ten systems selected.".bright_green());
        return ALL.to_vec();
    }

    let mut selected: Vec<&'static str> = line
        .split_whitespace()
        .filter_map(|tok| tok.parse::<usize>().ok())
        .filter(|&n| n >= 1 && n <= ALL.len())
        .map(|n| ALL[n - 1])
        .collect();

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

