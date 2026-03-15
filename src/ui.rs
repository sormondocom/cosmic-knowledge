//! UI / display module — terminal rendering helpers that are pure presentation.
//!
//! Contains:
//!  - `MainMode` enum
//!  - `show_main_menu` — top-level menu loop
//!  - `print_angel_banner` — ASCII angel and title bar
//!  - `show_loading_screen` — animated startup sequence
//!  - `show_help` — usage reference

use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use colored::*;

use crate::audio::SOLFEGGIO_FREQUENCIES;

// ─── Main-menu enum ───────────────────────────────────────────────────────────

/// Top-level navigation choices.
pub enum MainMode {
    Numerology,
    Enochian,
    WorldSystems,
    Frequencies,
    Help,
    Quit,
}

// ─── Main menu ────────────────────────────────────────────────────────────────

/// Display the top-level menu and return the user's selection.
pub fn show_main_menu() -> MainMode {
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════╗".bright_yellow());
    println!("{}", "║                  ✦  CHOOSE YOUR PATH  ✦                  ║".bold().bright_yellow());
    println!("{}", "╠══════════════════════════════════════════════════════════╣".bright_yellow());
    println!("{}", "║                                                          ║".bright_yellow());
    println!("{}", "║   1.  🔢  Gematria & Numerology                          ║".bright_white());
    println!("{}", "║          Analyze words across all five systems           ║".dimmed());
    println!("{}", "║                                                          ║".bright_yellow());
    println!("{}", "║   2.  📜  Enochian Angelology                            ║".bright_cyan());
    println!("{}", "║          Alphabet · Aethyrs · Translate · Keys           ║".dimmed());
    println!("{}", "║                                                          ║".bright_yellow());
    println!("{}", "║   3.  🎵  Sacred Frequencies & Export                    ║".bright_magenta());
    println!("{}", "║          Binaural beats · Solfeggio · WAV export         ║".dimmed());
    println!("{}", "║                                                          ║".bright_yellow());
    println!("{}", "║   4.  ❓  Help & Reference                               ║".white());
    println!("{}", "║                                                          ║".bright_yellow());
    println!("{}", "║   5.  🌏  World Cosmologies                              ║".bright_green());
    println!("{}", "║          Chinese · African · Nine Star Ki · Ifá          ║".dimmed());
    println!("{}", "║                                                          ║".bright_yellow());
    println!("{}", "║   0.  ✦  Exit                                            ║".dimmed());
    println!("{}", "║                                                          ║".bright_yellow());
    println!("{}", "╚══════════════════════════════════════════════════════════╝".bright_yellow());
    println!();

    loop {
        print!("{}", "▸ Enter choice: ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { return MainMode::Quit; }
        match input.trim() {
            "1"       => return MainMode::Numerology,
            "2"       => return MainMode::Enochian,
            "3"       => return MainMode::Frequencies,
            "4"       => return MainMode::Help,
            "5"       => return MainMode::WorldSystems,
            "0" | ""  => return MainMode::Quit,
            _         => println!("{}", "  Please enter 0–5.".yellow()),
        }
    }
}

// ─── Angel banner ─────────────────────────────────────────────────────────────

/// Print the ASCII angel figure and application title.
pub fn print_angel_banner() {
    let angel = r#"
          ⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀
          ⠀⠀⠀⠀⣀⣴⣾⣿⣿⣿⣿⣿⣷⣦⣀⠀⠀⠀⠀⠀
          ⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀
          ⠀⢀⣾⣿⣿⣿⣿⠿⠿⠿⠿⣿⣿⣿⣿⣿⣷⡀⠀⠀
          ⠀⣼⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣧⠀⠀
          ⢸⣿⣿⠏⠀⠀⣀⣤⣶⣶⣦⣄⠀⠀⠹⣿⣿⣿⡇⠀
          ⠸⣿⣿⠀⠀⢰⣿⣿⣿⣿⣿⣿⡆⠀⠀⣿⣿⣿⠇⠀
          ⠀⠹⣿⣧⠀⠈⠻⣿⣿⣿⣿⡿⠋⠀⣰⣿⣿⠏⠀⠀
          ⠀⠀⠹⣿⣧⡀⠀⠈⠉⠉⠉⠀⢀⣾⣿⠟⠀⠀⠀⠀
          ⠀⠀⠀⠈⠻⢿⣶⣤⣀⣀⣤⣶⡿⠋⠁⠀⠀⠀⠀⠀
          ⠀⠀⠀⠀⠀⠀⠈⠉⠛⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀
    "#;
    println!("{}", angel.bright_white());
    println!("{}", "CELESTIAL NUMEROLOGY ANALYZER".bold().bright_yellow());
    println!("{}", "Hebrew · Pythagorean · Chaldean · Greek Isopsephy · Agrippan".italic().bright_blue());
    println!("{}", "Simple Ordinal · Reverse Ordinal · Abjad · Enochian (John Dee)".italic().bright_blue());
    println!("{}", "──────────────────────────────────────────────────────────".dimmed());
}

// ─── Loading screen ───────────────────────────────────────────────────────────

/// Play the animated startup sequence, then clear the terminal.
pub fn show_loading_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap_or(());

    println!("{}", "\n".repeat(6));
    println!("{}", "            ╭─────────────────────────────────────╮".bright_magenta());
    println!("{}", "            │        🌟 AWAKENING 🌟             │".bright_magenta());
    println!("{}", "            │    CELESTIAL CONSCIOUSNESS        │".bright_magenta());
    println!("{}", "            ╰─────────────────────────────────────╯".bright_magenta());
    println!();

    for pulse in 0..3_u8 {
        let energy = if pulse % 2 == 0 { ["◊", "◈", "◉", "◈", "◊"] } else { ["◉", "◈", "◊", "◈", "◉"] };
        print!("\r                ");
        for s in energy { print!("{} ", s.bright_cyan()); }
        print!(" LOADING ");
        for s in energy.iter().rev() { print!("{} ", s.bright_cyan()); }
        io::stdout().flush().unwrap_or(());
        thread::sleep(Duration::from_millis(400));
    }
    println!("\n");

    let messages = [
        ("Connecting to celestial frequencies",   "🌌"),
        ("Aligning with divine numerology",        "⚡"),
        ("Awakening angelic consciousness",        "👼"),
        ("Tuning into universal vibrations",       "🎵"),
        ("Opening sacred numerical channels",      "🔢"),
        ("Downloading cosmic wisdom",              "📡"),
        ("Initializing spiritual algorithms",      "🧮"),
        ("Calibrating mystical sensors",           "🔮"),
    ];
    let spinners = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let bar_len = 20;

    for (message, icon) in &messages {
        for frame in 0..25_usize {
            let spinner  = spinners[frame % spinners.len()];
            let progress = (frame * bar_len) / 25;
            let bar      = format!("[{}{}]", "█".repeat(progress), "░".repeat(bar_len - progress));
            print!("\r        {} {} {} {} {}",
                icon,
                spinner.bright_white(),
                message.bright_white(),
                bar.bright_blue(),
                format!("{}%", (frame * 100) / 25).dimmed(),
            );
            io::stdout().flush().unwrap_or(());
            thread::sleep(Duration::from_millis(50));
        }
        let full_bar = format!("[{}]", "█".repeat(bar_len));
        println!("\r        {} ✓ {} {} {}   ",
            icon,
            message.bright_white(),
            full_bar.bright_green(),
            "100%".bright_green(),
        );
        thread::sleep(Duration::from_millis(150));
    }

    println!();
    let mystical = [
        "🌟 Synchronizing with akashic records…",
        "⭐ Blessing your spiritual journey…",
        "✨ Opening gates of wisdom…",
        "🌙 Moon cycles aligned…",
        "☄️ Cosmic energies flowing…",
        "🪐 Planetary influences calibrated…",
        "🌌 Universe ready to speak…",
    ];
    for text in mystical {
        println!("{}", format!("           {}", text).italic().bright_magenta());
        thread::sleep(Duration::from_millis(300));
    }

    println!("\n");
    println!("{}", "              ╭───────────────────────╮".bright_yellow());
    println!("{}", "              │  🔮 CONNECTION LIVE 🔮  │".bold().bright_magenta());
    println!("{}", "              │   Sacred portal open   │".bright_yellow());
    println!("{}", "              ╰───────────────────────╯".bright_yellow());

    let sparkles = ["✦", "✧", "✩", "✪", "✫"];
    print!("                ");
    for s in sparkles {
        print!("{} ", s.bright_yellow());
        io::stdout().flush().unwrap_or(());
        thread::sleep(Duration::from_millis(100));
    }

    println!("\n{}", "            Ready to decode the universe…".italic().bright_blue());
    thread::sleep(Duration::from_millis(1000));

    // Single clear — avoids the visible flicker caused by rapid repeated clears.
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap_or(());
}

// ─── Help screen ──────────────────────────────────────────────────────────────

/// Print the full help and reference screen.
pub fn show_help() {
    println!("{}", "╔════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║          🌟 CELESTIAL NUMEROLOGY ANALYZER 🌟           ║".bright_cyan());
    println!("{}", "║              Sacred Frequency Generator               ║".bright_cyan());
    println!("{}", "╚════════════════════════════════════════════════════════╝".bright_cyan());
    println!();
    println!("{}", "USAGE:".bold());
    println!("{}", "  cargo run [FLAGS]");
    println!();
    println!("{}", "FLAGS:".bold());
    println!("{}", "  -f, --fast              Skip the mystical loading animation".bright_white());
    println!("{}", "  -s, --silent            Disable audio frequencies".bright_white());
    println!("{}", "  --export-all            Export all Solfeggio frequencies to WAV files".bright_white());
    println!("{}", "  --aethyr <n|name>       Look up a specific Aethyr (e.g. --aethyr 10 or --aethyr ZAX)".bright_white());
    println!("{}", "  --aethyr                Show all 30 Aethyrs".bright_white());
    println!("{}", "  -h, --help              Show this help message".bright_white());
    println!();
    println!("{}", "NUMEROLOGICAL SYSTEMS:".bold());
    println!("{}", "  Hebrew Gematria   — Traditional letter-values (A=1 … Z=900, Mispar Hechrachi)".bright_white());
    println!("{}", "  Pythagorean       — Western 1-9 cyclical mapping (mod-9 of alphabet position)".bright_white());
    println!("{}", "  Chaldean          — Ancient Babylonian vibrational system (9 sacred, unassigned)".bright_white());
    println!("{}", "  Greek Isopsephy   — Classical Greek letter-number system (Neoplatonic tradition)".bright_white());
    println!("{}", "  Agrippan          — Cornelius Agrippa/Francis Barrett Latin gematria (c.1531/1801)".bright_white());
    println!("{}", "  Simple Ordinal    — Direct alphabetical position 1-26 (modern English Gematria)".bright_white());
    println!("{}", "  Reverse Ordinal   — Mirror complement: Z=1 … A=26".bright_white());
    println!("{}", "  Abjad             — Arabic/Islamic letter-number system (ḥisāb al-jumal)".bright_white());
    println!("{}", "  Enochian Ordinal  — John Dee's angelic alphabet, positional values 1-21".bright_white());
    println!("{}", "  Enochian G.D.     — Golden Dawn's Hebrew-mapped Enochian values".bright_white());
    println!();
    println!("{}", "SOLFEGGIO FREQUENCIES:".bold());
    for (freq, _, icon) in SOLFEGGIO_FREQUENCIES {
        let name = crate::audio::get_frequency_name(*freq);
        println!("{}", format!("  {} {} Hz — {}", icon, *freq as u32, name).bright_magenta());
    }
    println!();
    println!("{}", "EXPORT FORMATS:".bold());
    println!("{}", "  • WAV files (44.1 kHz, 16-bit)".bright_white());
    println!("{}", "  • Pure tones (mono) or Binaural beats (stereo)".bright_white());
    println!("{}", "  • Durations: 5, 10, or 30 minutes".bright_white());
    println!("{}", "  • Custom frequency combinations available".bright_white());
    println!();
    println!("{}", "WORLD COSMOLOGIES:".bold());
    println!("{}", "  Chinese     — Nine Star Ki natal star, Wu Xing Five Elements, lucky/unlucky numbers".bright_white());
    println!("{}", "  African     — Yoruba Ifá (16 Odù), Akan day-soul names, Kemetic sacred numbers".bright_white());
    println!();
    println!("{}", "Visit the interactive mode to analyze words and export personalized frequencies!".italic().bright_blue());
}
