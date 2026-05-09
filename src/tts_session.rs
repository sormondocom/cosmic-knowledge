//! TTS configuration session — voice, rate, volume, and auto-read settings.

use std::io::{self, Write};

use colored::*;

use crate::tts_reader::{
    tts_adjust_rate, tts_adjust_volume, tts_auto_read, tts_available, tts_get_rate,
    tts_get_volume, tts_select_voice, tts_speak, tts_toggle_auto, tts_voice_label,
};

// ─── Test phrase ──────────────────────────────────────────────────────────────

const TEST_PHRASE: &str =
    "In the beginning was the Word. \
     The quick brown fox jumps over the lazy dog. \
     Testing one, two, three.";

// ─── Entry point ──────────────────────────────────────────────────────────────

pub fn run_tts_session() {
    if !tts_available() {
        println!();
        println!(
            "{}",
            "  ⚠  Text-to-speech is not available on this platform.".yellow()
        );
        println!(
            "{}",
            "     Requires Windows SAPI/WinRT, macOS AVSpeechSynthesizer,".dimmed()
        );
        println!(
            "{}",
            "     or Linux SpeechDispatcher (spd-say).".dimmed()
        );
        println!();
        return;
    }

    loop {
        print_status();
        print_menu();

        print!("{}", "▸ Choice: ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap_or(0);

        match buf.trim() {
            "1" => {
                println!();
                tts_select_voice();
                println!();
            }

            "2" => {
                tts_adjust_volume(true);
                let (cur, lo, hi) = tts_get_volume();
                println!("  {}", format!("Volume → {}%", pct(cur, lo, hi)).bright_yellow());
            }

            "3" => {
                tts_adjust_volume(false);
                let (cur, lo, hi) = tts_get_volume();
                println!("  {}", format!("Volume → {}%", pct(cur, lo, hi)).yellow());
            }

            "4" => {
                tts_adjust_rate(true);
                let (cur, lo, hi) = tts_get_rate();
                println!("  {}", format!("Rate   → {}%", pct(cur, lo, hi)).bright_yellow());
            }

            "5" => {
                tts_adjust_rate(false);
                let (cur, lo, hi) = tts_get_rate();
                println!("  {}", format!("Rate   → {}%", pct(cur, lo, hi)).yellow());
            }

            "6" => {
                let on = tts_toggle_auto();
                println!(
                    "  {}",
                    if on {
                        "Auto-read ON  — verses will be read aloud automatically.".bright_green()
                    } else {
                        "Auto-read OFF — press r to read on demand.".yellow()
                    }
                );
            }

            "7" => {
                println!("{}", "  Speaking test phrase…".dimmed());
                tts_speak(TEST_PHRASE);
            }

            "0" | "" => break,

            _ => println!("{}", "  Please enter 1–7 or 0.".yellow()),
        }
    }
}

// ─── Status panel ─────────────────────────────────────────────────────────────

fn print_status() {
    let voice     = tts_voice_label();
    let auto      = tts_auto_read();
    let (r, rl, rh) = tts_get_rate();
    let (v, vl, vh) = tts_get_volume();

    let rate_bar   = bar(r, rl, rh, 20);
    let vol_bar    = bar(v, vl, vh, 20);
    let rate_pct   = pct(r, rl, rh);
    let vol_pct    = pct(v, vl, vh);

    let sep = "  ─────────────────────────────────────────────────────";
    println!();
    println!("{}", "  ✦  TEXT-TO-SPEECH SETTINGS  ✦".bold().bright_cyan());
    println!("{}", sep.dimmed());
    println!(
        "  Voice      {}",
        voice.bright_white().bold()
    );
    println!(
        "  Volume     {} {}%",
        vol_bar.bright_yellow(),
        vol_pct.to_string().bright_yellow()
    );
    println!(
        "  Rate       {} {}%",
        rate_bar.bright_cyan(),
        rate_pct.to_string().bright_cyan()
    );
    println!(
        "  Auto-read  {}",
        if auto { "ON".bright_green().bold() } else { "OFF".dimmed() }
    );
    println!("{}", sep.dimmed());
    println!();
}

// ─── Menu ─────────────────────────────────────────────────────────────────────

fn print_menu() {
    let rows: &[(&str, &str, &str)] = &[
        ("1", "🎙 ", "Select voice           pick English female / male"),
        ("2", "🔊 ", "Volume louder          +10% of range"),
        ("3", "🔉 ", "Volume quieter         −10% of range"),
        ("4", "⏩ ", "Rate faster            +10% of range"),
        ("5", "⏪ ", "Rate slower            −10% of range"),
        ("6", "🔄 ", "Toggle auto-read       ON / OFF"),
        ("7", "🔈 ", "Test speech            hear current voice & settings"),
    ];
    for (key, icon, label) in rows {
        println!(
            "  {}  {}  {}",
            key.bright_yellow().bold(),
            icon,
            label.bright_white()
        );
    }
    println!("  {}  {}  {}", "0".dimmed(), "←  ", "Back".dimmed());
    println!();
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Percentage of `val` within [lo, hi], clamped 0–100.
fn pct(val: f32, lo: f32, hi: f32) -> u32 {
    if hi <= lo {
        return 50;
    }
    (((val - lo) / (hi - lo)).clamp(0.0, 1.0) * 100.0).round() as u32
}

/// ASCII progress bar of `width` chars showing where `val` sits in [lo, hi].
fn bar(val: f32, lo: f32, hi: f32, width: usize) -> String {
    let filled = if hi <= lo {
        width / 2
    } else {
        ((((val - lo) / (hi - lo)).clamp(0.0, 1.0)) * width as f32).round() as usize
    };
    let empty = width.saturating_sub(filled);
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}
