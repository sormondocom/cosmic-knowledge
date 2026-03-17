//! Celestial Numerology Analyzer — binary entry point.
//!
//! This file is intentionally thin: it owns only the top-level application
//! flow (argument parsing and the main session loop).  All domain logic,
//! interactive sessions, and sub-menus live in the modules declared below.

// ─── Module declarations ──────────────────────────────────────────────────────
mod audio;
mod cosmology;
mod enochian;
mod export;
mod menu;
mod numerology;
mod persistence;
mod reports;
mod rng;
mod utils;
mod zodiac;

// ─── Selective imports ────────────────────────────────────────────────────────
use std::io;

use colored::*;
use crossterm::execute;
use crossterm::cursor::Show;

use audio::{
    export_all_frequencies_cli, initialize_audio, play_intro_chord, show_frequency_menu,
    stop_audio, AudioSystem,
};
use enochian::{run_enochian_session, show_aethyr_info, show_aethyr_table, random_aethyr_chord};
use numerology::run_numerology_session;
use cosmology::run_world_systems_session;
use rng::run_rng_session;
use zodiac::run_zodiac_session;
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

    // ── Cursor safety: restore cursor on panic so the terminal isn't left blind ──
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        execute!(io::stdout(), Show).ok();
        prev_hook(info);
    }));

    // ── Ctrl+C: restore cursor before exiting ─────────────────────────────────
    ctrlc::set_handler(|| {
        execute!(io::stdout(), Show).ok();
        std::process::exit(0);
    }).ok();

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
        let (num, name, desc, freqs) = random_aethyr_chord();
        println!("  {} Aethyr {} ({}) — {}",
            "🌌".bright_white(),
            num.to_string().bright_cyan(),
            name.bright_yellow(),
            desc.dimmed(),
        );
        play_intro_chord(sys, freqs);
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
            MainMode::RngExperiment => run_rng_session(),
            MainMode::Zodiac        => run_zodiac_session(),
            MainMode::Help          => show_help(),
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
