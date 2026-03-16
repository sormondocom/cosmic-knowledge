//! Celestial Numerology Analyzer — binary entry point.
//!
//! This file is intentionally thin: it owns only the top-level application
//! flow (argument parsing, the main session loop, and the frequency export
//! sub-menu).  All domain logic and interactive sessions live in the modules
//! declared below.

// ─── Module declarations ──────────────────────────────────────────────────────
mod audio;
mod cosmology;
mod enochian;
mod export;
mod menu;
mod numerology;
mod reports;

// ─── Selective imports ────────────────────────────────────────────────────────
use std::io::{self, Write};

use colored::*;

use audio::{
    create_custom_binaural, export_all_frequencies, export_all_frequencies_cli,
    export_frequency, get_frequency_name, initialize_audio, start_ambient_frequency,
    stop_audio, AudioSystem, SOLFEGGIO_FREQUENCIES,
};
use enochian::{run_enochian_session, show_aethyr_info, show_aethyr_table};
use numerology::run_numerology_session;
use cosmology::run_world_systems_session;
use menu::{print_angel_banner, show_help, show_loading_screen, show_main_menu, MainMode};

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
            MainMode::Numerology   => run_numerology_session(&audio_system),
            MainMode::Enochian     => run_enochian_session(),
            MainMode::WorldSystems => run_world_systems_session(),
            MainMode::Frequencies  => {
                if audio_system.is_some() {
                    show_frequency_menu();
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

// ─── Frequency export menu ────────────────────────────────────────────────────

fn show_frequency_menu() {
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
