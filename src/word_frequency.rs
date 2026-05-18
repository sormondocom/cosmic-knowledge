//! Word & Title Frequency — derive an audible Hz from any word's gematria value.
//!
//! Formula: octave-transpose the gematria total into [110, 880 Hz] (A2–A5).
//! Values that differ by a power of 2 share the same pitch class (octave equivalents),
//! which is musically meaningful — "the same note in a different register."
//!
//! Alignment with known sacred frequencies is detected after normalising both
//! values to the same one-octave window [110, 220 Hz].  Matching means the
//! word and the sacred frequency are the same note, possibly in different octaves.

use std::io::{self, Write};

use colored::*;

use crate::audio::{change_frequency, stop_audio, AudioSystem};
use crate::numerology::numerology;

// ─── Audible target range ─────────────────────────────────────────────────────

const LO: f32   = 110.0;   // A2 — bottom of range
const HI: f32   = 880.0;   // A5 — top of range  (3 octaves)
const NORM: f32 = 110.0;   // base for one-octave normalization window [110, 220)

const TOLERANCE: f32 = 4.0; // Hz (after octave-normalisation) that counts as "aligned"

// ─── Known sacred frequencies ─────────────────────────────────────────────────

static KNOWN: &[(&str, f32)] = &[
    ("174 Hz — Solfeggio Ut¹ (Foundation / Earth)", 174.0),
    ("285 Hz — Solfeggio (Regeneration / Energy)", 285.0),
    ("396 Hz — Solfeggio Ut² (Liberation from Fear)", 396.0),
    ("417 Hz — Solfeggio Re (Undoing / Change)", 417.0),
    ("432 Hz — Verdi's A (natural tuning)", 432.0),
    ("440 Hz — Concert A (standard tuning)", 440.0),
    ("528 Hz — Solfeggio Mi (Transformation / Love)", 528.0),
    ("639 Hz — Solfeggio Fa (Connection / Harmony)", 639.0),
    ("741 Hz — Solfeggio Sol (Expression / Awakening)", 741.0),
    ("852 Hz — Solfeggio La (Return to Spiritual Order)", 852.0),
    ("963 Hz — Solfeggio Si (Unity / Crown)", 963.0),
];

// ─── Core maths ───────────────────────────────────────────────────────────────

/// Octave-transpose `value` into [LO, HI).
pub fn gematria_to_hz(value: u32) -> f32 {
    if value == 0 {
        return LO;
    }
    let mut f = value as f32;
    while f < LO  { f *= 2.0; }
    while f >= HI { f /= 2.0; }
    f
}

/// Nearest equal-temperament note name (A4 = 440 Hz reference).
pub fn hz_to_note(hz: f32) -> String {
    const NAMES: &[&str] = &[
        "C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B",
    ];
    let semi = 12.0 * (hz / 440.0).log2();
    let midi = (69.0 + semi).round() as i32;
    let idx  = ((midi % 12) + 12) as usize % 12;
    let oct  = midi / 12 - 1;
    format!("{}{}", NAMES[idx], oct)
}

/// Check whether `hz` aligns (octave-normalised) with any known sacred frequency.
/// Returns a Vec of `(display_name, octave_relationship_string)`.
pub fn known_alignments(hz: f32) -> Vec<(&'static str, String)> {
    let word_norm = octave_norm(hz);
    KNOWN
        .iter()
        .filter_map(|(name, kf)| {
            if (octave_norm(*kf) - word_norm).abs() < TOLERANCE {
                Some((*name, octave_rel(hz, *kf)))
            } else {
                None
            }
        })
        .collect()
}

/// Normalise f into [NORM, NORM*2) for comparison.
fn octave_norm(f: f32) -> f32 {
    let hi = NORM * 2.0;
    let mut x = f;
    while x < NORM { x *= 2.0; }
    while x >= hi  { x /= 2.0; }
    x
}

/// Human-readable octave relationship between `word_hz` and a `known_hz`.
fn octave_rel(word_hz: f32, known_hz: f32) -> String {
    let n = (word_hz / known_hz).log2().round() as i32;
    match n {
        0  => format!("direct (±{:.0} Hz)", (word_hz - known_hz).abs()),
        1  => "1 oct. above".to_string(),
        -1 => "1 oct. below".to_string(),
        k if k > 0 => format!("{} oct. above", k),
        k           => format!("{} oct. below", -k),
    }
}

// ─── Session ──────────────────────────────────────────────────────────────────

pub fn run_word_frequency_session(audio: &Option<AudioSystem>) {
    println!();
    println!("{}", "  ╔══════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "  ║       🔉  WORD & TITLE FREQUENCY                        ║".bold().bright_cyan());
    println!("{}", "  ╠══════════════════════════════════════════════════════════╣".bright_cyan());
    println!("{}", "  ║  Gematria total → octave-transposed to [110–880 Hz]      ║".dimmed());
    println!("{}", "  ║  Alignment with known sacred frequencies is shown (✦)    ║".dimmed());
    println!("{}", "  ║  Values an octave apart share the same pitch class       ║".dimmed());
    println!("{}", "  ╚══════════════════════════════════════════════════════════╝".bright_cyan());
    println!();

    loop {
        print!("{}", "  ▸ Word or phrase (blank to return): ".bold().cyan());
        io::stdout().flush().unwrap_or(());

        let mut raw = String::new();
        io::stdin().read_line(&mut raw).unwrap_or(0);
        let raw = raw.trim();
        if raw.is_empty() {
            break;
        }

        // Normalise to uppercase alpha only (same as numerology session)
        let word: String = raw
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .flat_map(|c| c.to_uppercase())
            .collect();

        if word.is_empty() {
            println!("{}", "  ⚠  No alphabetic characters.".yellow());
            continue;
        }

        let results = numerology(&word);

        // Build rows: (system_name, value, hz)
        let rows: Vec<(&str, u32, f32)> = results
            .iter()
            .map(|(name, (total, _))| (*name, *total, gematria_to_hz(*total)))
            .collect();

        print_table(&word, &rows);

        // Let the user play frequencies if audio is available
        if let Some(sys) = audio {
            play_loop(sys, &rows);
        } else {
            println!("{}", "  (audio not available — run without --silent to play tones)".dimmed());
            println!();
        }
    }

    // Stop any playing tone on exit
    if let Some(sys) = audio {
        stop_audio(sys);
    }
}

// ─── Display ──────────────────────────────────────────────────────────────────

fn print_table(word: &str, rows: &[(&str, u32, f32)]) {
    println!();
    println!(
        "  {} {}",
        "Word:".bright_cyan(),
        word.bold().bright_white()
    );
    println!("{}", "  ──────────────────────────────────────────────────────────────────".dimmed());
    println!(
        "  {:<3}  {:<18}  {:>7}  {:>9}  {:>5}",
        " # ".dimmed(),
        "System".dimmed(),
        "Value".dimmed(),
        "Hz".dimmed(),
        "Note".dimmed(),
    );
    println!("{}", "  ──────────────────────────────────────────────────────────────────".dimmed());

    for (i, (name, value, hz)) in rows.iter().enumerate() {
        let note     = hz_to_note(*hz);
        let aligns   = known_alignments(*hz);

        let row = format!(
            "  {:<3}  {:<18}  {:>7}  {:>8.1} Hz  {:>5}",
            (i + 1).to_string().bright_yellow().bold(),
            name.bright_white(),
            value.to_string().bright_blue(),
            hz,
            note.bright_cyan(),
        );
        print!("{}", row);

        if aligns.is_empty() {
            println!();
        } else {
            // Show first alignment inline; extra ones on continuation lines
            let (aname, rel) = &aligns[0];
            println!(
                "  {}",
                format!("✦ {} [{}]", aname, rel).bright_magenta()
            );
            for (aname2, rel2) in aligns.iter().skip(1) {
                println!(
                    "  {:<3}  {:<18}  {:>7}  {:>9}  {:>5}  {}",
                    "", "", "", "", "",
                    format!("  ✦ {} [{}]", aname2, rel2).bright_magenta()
                );
            }
        }
    }

    println!("{}", "  ──────────────────────────────────────────────────────────────────".dimmed());
    println!();
    println!("{}", "  Note: values an octave apart (×2 / ÷2) share the same pitch class.".dimmed());
    println!("{}", "  Alignment is checked after normalising both to the same octave window.".dimmed());
    println!();
}

// ─── Play loop ────────────────────────────────────────────────────────────────

fn play_loop(sys: &AudioSystem, rows: &[(&str, u32, f32)]) {
    let n = rows.len();
    loop {
        print!(
            "{}",
            format!("  ▸ Play 1–{} (row) · Enter = new word · 0 = back: ", n)
                .bold()
                .cyan()
        );
        io::stdout().flush().unwrap_or(());

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap_or(0);
        let s = buf.trim();

        if s.is_empty() { break; }
        if s == "0"     { stop_audio(sys); return; }

        if let Ok(idx) = s.parse::<usize>() {
            if idx >= 1 && idx <= n {
                let (name, value, hz) = &rows[idx - 1];
                change_frequency(sys, *hz);
                println!(
                    "  {} {} → {} Hz ({})  [{}]",
                    "♪".bright_magenta(),
                    name.bright_white(),
                    format!("{:.1}", hz).bright_yellow(),
                    hz_to_note(*hz).bright_cyan(),
                    format!("gematria value: {}", value).dimmed(),
                );
            } else {
                println!("{}", format!("  Enter 1–{}.", n).yellow());
            }
        } else {
            println!("{}", format!("  Enter 1–{}, 0, or blank.", n).yellow());
        }
    }

    stop_audio(sys);
}
