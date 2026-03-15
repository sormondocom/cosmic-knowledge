//! Report-building and file-export module.
//!
//! Contains:
//!  - `prompt_and_export`               — interactive save-to-file prompt
//!  - `build_numerology_report`         — multi-system numerology report
//!  - `build_enochian_translation_report` — letter-by-letter translation report
//!  - `build_enochian_gematria_report`  — Enochian-only gematria report
//!  - `strip_leading_emoji`             — emoji-strip for plain-text output
//!  - `chrono_now`                      — accurate UTC timestamp via chrono

use std::fs;
use std::io::{self, Write};
use colored::*;
use chrono::Utc;

use crate::enochian::{enochian_lookup, enochian_substitute, aethyr_lookup, enochian_meaning, enochian_angelic_message};
use crate::numerology::{
    numerology, digital_root, meaning_of, isopsephy_meaning, abjad_meaning,
    angelic_message, master_numbers_message, check_special_sequences, get_calculation_breakdown,
};

// ─── File export prompt ───────────────────────────────────────────────────────

/// Prompt the user for a filename (or accept a default stem), then write `content`
/// to `exports/<stem>.txt`.
///
/// The user may type `n` / `no` to skip saving entirely.
pub fn prompt_and_export(suggested_stem: &str, content: &str) {
    fs::create_dir_all("exports").ok();
    println!();
    println!("{}", "  Save this result to a text file?".bold());
    println!("  {}",
        format!("  Suggested filename: exports/{}.txt  (press Enter to accept, or type a new name)",
            suggested_stem).dimmed());
    print!("{}", "  ▸ Filename (or 'n' to skip): ".cyan());
    io::stdout().flush().unwrap_or(());

    let mut choice = String::new();
    if io::stdin().read_line(&mut choice).is_err() { return; }
    let choice = choice.trim();

    if choice.eq_ignore_ascii_case("n") || choice.eq_ignore_ascii_case("no") {
        return;
    }

    let raw = if choice.is_empty() { suggested_stem.to_string() } else { choice.to_string() };
    // Sanitize: keep only alphanumerics, hyphens, underscores — prevents path traversal.
    let stem: String = raw
        .trim_end_matches(".txt")
        .chars()
        .filter(|&c| c.is_alphanumeric() || c == '_' || c == '-')
        .collect();
    let stem = if stem.is_empty() { suggested_stem.to_string() } else { stem };
    let path = format!("exports/{}.txt", stem);

    match fs::write(&path, content) {
        Ok(_)  => println!("  {}", format!("✅  Saved to: {}", path).bright_green()),
        Err(e) => println!("  {}", format!("❌  Could not write file: {}", e).bright_red()),
    }
    println!();
}

// ─── Report builders ─────────────────────────────────────────────────────────

/// Build a plain-text multi-system numerology report for `word`.
pub fn build_numerology_report(word: &str, active_systems: &[&str]) -> String {
    let all_results = numerology(word);
    let results: Vec<_> = all_results
        .iter()
        .filter(|(name, _)| active_systems.iter().any(|s| s == name))
        .collect();

    let mut out = String::new();
    out.push_str("CELESTIAL NUMEROLOGY REPORT\n");
    out.push_str(&format!("Generated: {}\n", chrono_now()));
    out.push_str(&"═".repeat(60));
    out.push('\n');
    out.push_str(&format!("Word / Phrase : {}\n", word));
    out.push_str(&format!("Systems active: {}\n", active_systems.join(", ")));
    out.push_str(&"─".repeat(60));
    out.push('\n');

    for (system, (total, root)) in &results {
        let is_enochian = system.starts_with("Enochian");
        let meaning = match &system[..] {
            s if s.starts_with("Enochian") => enochian_meaning(*root),
            "Greek Isopsephy"              => isopsephy_meaning(*root),
            "Abjad"                        => abjad_meaning(*root),
            _                              => meaning_of(*root),
        };
        let breakdown   = get_calculation_breakdown(word, system);

        out.push_str(&format!("\n{:<20} total={:>5}  root={}\n", system, total, root));
        out.push_str(&format!("  Meaning : {}\n", strip_leading_emoji(meaning)));
        if !breakdown.is_empty() {
            out.push_str(&format!("  Calc    : {}\n", breakdown));
        }
        if is_enochian {
            let (anum, aname, adesc) = aethyr_lookup(*total);
            out.push_str(&format!("  Aethyr  : {} ({}) — {}\n", anum, aname, adesc));
        } else if let Some(master) = master_numbers_message(*total) {
            out.push_str(&format!("  Master  : {}\n", strip_leading_emoji(master)));
        }
        if let Some(special) = check_special_sequences(*total) {
            out.push_str(&format!("  Pattern : {}\n", special));
        }
    }

    out.push('\n');
    out.push_str(&"─".repeat(60));
    out.push('\n');

    if let Some((_, (_, root))) = results.first() {
        out.push_str(&format!("\nAngelic Message:\n  {}\n", angelic_message(*root)));
    }

    if active_systems.iter().any(|s| *s == "Enochian Ordinal") {
        if let Some((_, (_, er))) = all_results.iter().find(|(s, _)| *s == "Enochian Ordinal") {
            out.push_str(&format!("\nEnochian Call (John Dee):\n  {}\n", enochian_angelic_message(*er)));
        }
    }

    out.push('\n');
    out.push_str(&"═".repeat(60));
    out.push_str("\nGenerated by Celestial Numerology Analyzer\n");
    out
}

/// Build a plain-text report for an Enochian letter-by-letter translation.
pub fn build_enochian_translation_report(word: &str) -> String {
    let mut out = String::new();
    out.push_str("ENOCHIAN TRANSLATION REPORT\n");
    out.push_str(&format!("Generated: {}\n", chrono_now()));
    out.push_str(&"═".repeat(60));
    out.push('\n');
    out.push_str(&format!("Input word  : {}\n", word));
    out.push_str(&"─".repeat(60));
    out.push('\n');
    out.push_str(&format!(
        "{:<6}  {:<16}  {:<6}  {:>7}  {:>8}\n",
        "Char", "Enochian Name", "Sub?", "Ordinal", "GD Value"
    ));
    out.push_str(&"-".repeat(52));
    out.push('\n');

    let mut ordinal_total = 0u32;
    let mut gd_total      = 0u32;

    for ch in word.chars() {
        let sub        = enochian_substitute(ch);
        let sub_marker = if sub != ch { format!("->{}", sub) } else { "   ".to_string() };
        match enochian_lookup(ch) {
            Some((name, ord, gd)) => {
                ordinal_total += ord;
                gd_total      += gd;
                out.push_str(&format!(
                    "{:<6}  {:<16}  {:<6}  {:>7}  {:>8}\n",
                    ch, name, sub_marker, ord, gd
                ));
            }
            None => {
                out.push_str(&format!(
                    "{:<6}  {:<16}  {:<6}  {:>7}  {:>8}\n",
                    ch, "(no glyph)", "", "---", "-----"
                ));
            }
        }
    }

    out.push_str(&"-".repeat(52));
    out.push('\n');
    out.push_str(&format!("Ordinal total : {}  ->  root {}\n", ordinal_total, digital_root(ordinal_total)));
    out.push_str(&format!("G.D. total    : {}  ->  root {}\n", gd_total, digital_root(gd_total)));

    let (oa, on, od) = aethyr_lookup(ordinal_total);
    let (ga, gn, gd) = aethyr_lookup(gd_total);
    out.push_str(&format!("Ordinal Aethyr: {} ({}) — {}\n", oa, on, od));
    out.push_str(&format!("G.D. Aethyr   : {} ({}) — {}\n", ga, gn, gd));

    let root = digital_root(ordinal_total);
    out.push('\n');
    out.push_str(&"─".repeat(60));
    out.push('\n');
    out.push_str(&format!(
        "Enochian Call Fragment (root {}):\n  {}\n",
        root, enochian_angelic_message(root)
    ));

    out.push('\n');
    out.push_str(&"═".repeat(60));
    out.push_str("\nGenerated by Celestial Numerology Analyzer — Enochian Module\n");
    out.push_str("Source: Dee's Loagaeth (Sloane MS 3189); Laycock (2001)\n");
    out
}

/// Build a plain-text report for an Enochian-only gematria analysis.
pub fn build_enochian_gematria_report(word: &str) -> String {
    let all_results = numerology(word);
    let mut out = String::new();
    out.push_str("ENOCHIAN GEMATRIA REPORT\n");
    out.push_str(&format!("Generated: {}\n", chrono_now()));
    out.push_str(&"═".repeat(60));
    out.push('\n');
    out.push_str(&format!("Word / Phrase : {}\n", word));
    out.push_str(&"─".repeat(60));
    out.push('\n');

    for (system, (total, root)) in all_results.iter().filter(|(s, _)| s.starts_with("Enochian")) {
        let breakdown          = get_calculation_breakdown(word, system);
        let (anum, aname, adesc) = aethyr_lookup(*total);
        out.push_str(&format!("\n{:<20} total={:>5}  root={}\n", system, total, root));
        out.push_str(&format!("  Meaning : {}\n", strip_leading_emoji(enochian_meaning(*root))));
        if !breakdown.is_empty() {
            out.push_str(&format!("  Calc    : {}\n", breakdown));
        }
        out.push_str(&format!("  Aethyr  : {} ({}) — {}\n", anum, aname, adesc));
    }

    let enochian_root = all_results
        .iter()
        .find(|(s, _)| *s == "Enochian Ordinal")
        .map(|(_, (_, r))| *r)
        .unwrap_or(1);

    out.push('\n');
    out.push_str(&"─".repeat(60));
    out.push('\n');
    out.push_str(&format!(
        "Enochian Call (John Dee):\n  {}\n",
        enochian_angelic_message(enochian_root)
    ));

    out.push('\n');
    out.push_str(&"═".repeat(60));
    out.push_str("\nGenerated by Celestial Numerology Analyzer — Enochian Module\n");
    out
}

// ─── Utilities ────────────────────────────────────────────────────────────────

/// Skip any leading non-ASCII-alphabetic characters (emoji, symbols) for plain-text output.
pub fn strip_leading_emoji(s: &str) -> &str {
    let mut chars = s.char_indices();
    while let Some((i, c)) = chars.next() {
        if c.is_ascii_alphabetic() || c == '(' {
            return &s[i..];
        }
    }
    s
}

/// Return an accurate UTC timestamp string for report headers.
pub fn chrono_now() -> String {
    Utc::now().format("%Y-%m-%d  %H:%M:%S UTC").to_string()
}
