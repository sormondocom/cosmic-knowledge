//! Configuration session — TTS voice/rate/volume, export directory, and other settings.
//! All changes are persisted to the `meta` table on exit.

use std::io::{self, Write};

use colored::*;

use crate::audio::{get_export_dir, set_export_dir};
use crate::persistence::{open_db, set_setting};
use crate::tts_reader::{
    tts_adjust_rate, tts_adjust_volume, tts_auto_read, tts_available, tts_get_rate,
    tts_get_volume, tts_save_settings, tts_select_voice, tts_speak, tts_toggle_auto,
    tts_voice_label,
};

// ─── Test phrase ──────────────────────────────────────────────────────────────

const TEST_PHRASE: &str =
    "In the beginning was the Word. \
     The quick brown fox jumps over the lazy dog. \
     Testing one, two, three.";

// ─── Entry point ──────────────────────────────────────────────────────────────

pub fn run_config_session() {
    let conn = match open_db() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", format!("  Database error: {e}").red());
            return;
        }
    };

    loop {
        print_status();
        print_menu();

        print!("{}", "▸ Choice: ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap_or(0);

        match buf.trim() {
            "1" => {
                if tts_available() {
                    println!();
                    tts_select_voice();
                    println!();
                } else {
                    println!("{}", "  TTS is not available on this platform.".yellow());
                }
            }

            "2" => {
                if tts_available() {
                    tts_adjust_volume(true);
                    let (cur, lo, hi) = tts_get_volume();
                    println!("  {}", format!("Volume → {}%", pct(cur, lo, hi)).bright_yellow());
                }
            }

            "3" => {
                if tts_available() {
                    tts_adjust_volume(false);
                    let (cur, lo, hi) = tts_get_volume();
                    println!("  {}", format!("Volume → {}%", pct(cur, lo, hi)).yellow());
                }
            }

            "4" => {
                if tts_available() {
                    tts_adjust_rate(true);
                    let (cur, lo, hi) = tts_get_rate();
                    println!("  {}", format!("Rate   → {}%", pct(cur, lo, hi)).bright_yellow());
                }
            }

            "5" => {
                if tts_available() {
                    tts_adjust_rate(false);
                    let (cur, lo, hi) = tts_get_rate();
                    println!("  {}", format!("Rate   → {}%", pct(cur, lo, hi)).yellow());
                }
            }

            "6" => {
                if tts_available() {
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
            }

            "7" => {
                if tts_available() {
                    println!("{}", "  Speaking test phrase…".dimmed());
                    tts_speak(TEST_PHRASE);
                }
            }

            "8" => set_export_dir_interactive(&conn),

            "0" | "" => {
                // Persist TTS settings and export directory before leaving
                tts_save_settings(&conn);
                set_setting(&conn, "export_dir", &get_export_dir());
                break;
            }

            _ => println!("{}", "  Please enter 1–8 or 0.".yellow()),
        }
    }
}

// ─── Export directory ─────────────────────────────────────────────────────────

fn set_export_dir_interactive(conn: &rusqlite::Connection) {
    println!();
    println!(
        "  Current export directory: {}",
        get_export_dir().bright_white().bold()
    );
    println!("{}", "  Leave blank to keep current.".dimmed());
    print!("{}", "  New path: ".bold().cyan());
    io::stdout().flush().unwrap_or(());

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap_or(0);
    let new_dir = buf.trim();

    if new_dir.is_empty() {
        return;
    }

    set_export_dir(new_dir);
    set_setting(conn, "export_dir", new_dir);
    println!(
        "  {}",
        format!("Export directory set to: {}", new_dir).bright_green()
    );
}

// ─── Status panel ─────────────────────────────────────────────────────────────

fn print_status() {
    let sep = "  ─────────────────────────────────────────────────────";
    println!();
    println!("{}", "  ⚙  CONFIGURATION  ⚙".bold().bright_cyan());
    println!("{}", sep.dimmed());

    if tts_available() {
        let voice        = tts_voice_label();
        let auto         = tts_auto_read();
        let (r, rl, rh)  = tts_get_rate();
        let (v, vl, vh)  = tts_get_volume();
        let rate_bar     = bar(r, rl, rh, 20);
        let vol_bar      = bar(v, vl, vh, 20);

        println!("  Voice      {}", voice.bright_white().bold());
        println!(
            "  Volume     {} {}%",
            vol_bar.bright_yellow(),
            pct(v, vl, vh).to_string().bright_yellow()
        );
        println!(
            "  Rate       {} {}%",
            rate_bar.bright_cyan(),
            pct(r, rl, rh).to_string().bright_cyan()
        );
        println!(
            "  Auto-read  {}",
            if auto { "ON".bright_green().bold() } else { "OFF".dimmed() }
        );
    } else {
        println!(
            "  {}",
            "TTS  (unavailable on this platform)".dimmed()
        );
    }

    println!(
        "  Export dir {}",
        get_export_dir().bright_white()
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
        ("8", "📁 ", "Set export directory   where WAV files are saved"),
    ];
    for (key, icon, label) in rows {
        println!(
            "  {}  {}  {}",
            key.bright_yellow().bold(),
            icon,
            label.bright_white()
        );
    }
    println!("  {}  {}  {}", "0".dimmed(), "←  ", "Back  (settings saved)".dimmed());
    println!();
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn pct(val: f32, lo: f32, hi: f32) -> u32 {
    if hi <= lo { return 50; }
    (((val - lo) / (hi - lo)).clamp(0.0, 1.0) * 100.0).round() as u32
}

fn bar(val: f32, lo: f32, hi: f32, width: usize) -> String {
    let filled = if hi <= lo {
        width / 2
    } else {
        ((((val - lo) / (hi - lo)).clamp(0.0, 1.0)) * width as f32).round() as usize
    };
    let empty = width.saturating_sub(filled);
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}
