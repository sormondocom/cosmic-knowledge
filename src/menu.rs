//! Menu system — dynamic box-drawing menus and terminal display helpers.
//!
//! Replaces `ui.rs`.  Contains:
//!  - [`MenuItem`] / [`Menu`] — data-driven, 60-column box-drawing menus
//!  - [`MainMode`] — top-level navigation enum
//!  - [`show_main_menu`] — renders and reads the top-level menu
//!  - [`print_angel_banner`], [`show_loading_screen`], [`show_help`]

use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use colored::*;

use crate::audio::SOLFEGGIO_FREQUENCIES;

// ─── Primitives ───────────────────────────────────────────────────────────────

/// One selectable entry in a [`Menu`].
pub struct MenuItem {
    /// Key the user types to select this item (e.g. `"1"`, `"2"`).
    pub key:   &'static str,
    /// Emoji or symbol shown beside the label (assumed 2 display columns).
    pub icon:  &'static str,
    /// Primary label text (max ~47 chars; truncated if longer).
    pub label: &'static str,
    /// Dimmed sub-line hint below the label.  Empty string = no hint line.
    pub hint:  &'static str,
}

/// A renderable 60-column box-drawing menu.
///
/// Call [`Menu::show_and_read`] to render the menu and return the user's
/// trimmed input, or [`Menu::show`] just to print it.
pub struct Menu {
    /// Centred title in the header row.
    pub title:        &'static str,
    /// Colour applied to the box border lines.
    pub border_color: MenuColor,
    /// Selectable items rendered in order.
    pub items:        &'static [MenuItem],
    /// Key string for the back / exit footer entry.
    pub back_key:     &'static str,
    /// Label for the back / exit footer entry.
    pub back_label:   &'static str,
}

/// Border colour variants for [`Menu`].
#[allow(dead_code)]
pub enum MenuColor {
    Yellow,
    Cyan,
    White,
    Magenta,
    Green,
}

// Total display width of a menu line (border walls included).
const LINE_W: usize = 60;
// Inner display width between the two ║ walls.
const INNER:  usize = LINE_W - 2; // 58

impl Menu {
    /// Print the menu to stdout.
    pub fn show(&self) {
        let b = |s: &str| -> ColoredString {
            match self.border_color {
                MenuColor::Yellow  => s.bright_yellow(),
                MenuColor::Cyan    => s.bright_cyan(),
                MenuColor::White   => s.bright_white(),
                MenuColor::Magenta => s.bright_magenta(),
                MenuColor::Green   => s.bright_green(),
            }
        };

        let horiz = "═".repeat(INNER);
        let empty = format!("║{}║", " ".repeat(INNER));

        println!();
        println!("{}", b(&format!("╔{}╗", horiz)));
        println!("{}", b(&format!("║{}║", centre_pad(self.title, INNER))));
        println!("{}", b(&format!("╠{}╣", horiz)));

        for item in self.items {
            println!("{}", b(&empty));
            println!("{}", item_line(item.key, item.icon, item.label).bright_white());
            if !item.hint.is_empty() {
                println!("{}", hint_line(item.hint).dimmed());
            }
        }

        println!("{}", b(&empty));
        println!("{}", b(&format!("╠{}╣", horiz)));
        println!("{}", item_line(self.back_key, "←", self.back_label).dimmed());
        println!("{}", b(&empty));
        println!("{}", b(&format!("╚{}╝", horiz)));
        println!();
    }

    /// Print the menu and return the user's trimmed input.
    pub fn show_and_read(&self) -> String {
        self.show();
        print!("{}", "▸ Enter choice: ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap_or(0);
        buf.trim().to_string()
    }
}

// ─── Line-building helpers ────────────────────────────────────────────────────

/// Pad `text` symmetrically to exactly `width` Unicode chars (truncates if longer).
fn centre_pad(text: &str, width: usize) -> String {
    let len: usize = text.chars().count();
    if len >= width {
        return text.chars().take(width).collect();
    }
    let total = width - len;
    let left  = total / 2;
    let right = total - left;
    format!("{}{}{}", " ".repeat(left), text, " ".repeat(right))
}

/// `║   KEY.  ICON  LABEL<padding>║`  — 60 display columns total.
///
/// Inner layout (display cols):
/// 3(spaces) + 2(key+".") + 2(spaces) + 2(icon emoji) + 2(spaces) + 47(label) = 58
fn item_line(key: &str, icon: &str, label: &str) -> String {
    let label_truncated: String = label.chars().take(47).collect();
    format!("║   {}.  {}  {:<47}║", key, icon, label_truncated)
}

/// `║          HINT<padding>║`  — 60 display columns total.
fn hint_line(hint: &str) -> String {
    let hint_truncated: String = hint.chars().take(48).collect();
    format!("║          {:<48}║", hint_truncated)
}

// ─── Main menu ────────────────────────────────────────────────────────────────

/// Top-level navigation choices returned by [`show_main_menu`].
pub enum MainMode {
    Numerology,
    Enochian,
    WorldSystems,
    Frequencies,
    RngExperiment,
    Zodiac,
    Help,
    Quit,
}

static MAIN_ITEMS: &[MenuItem] = &[
    MenuItem { key: "1", icon: "🔢", label: "Gematria & Numerology",
               hint: "Analyze words across all ten systems" },
    MenuItem { key: "2", icon: "📜", label: "Enochian Angelology",
               hint: "Alphabet · Aethyrs · Translate · Keys" },
    MenuItem { key: "3", icon: "🎵", label: "Sacred Frequencies & Export",
               hint: "Binaural beats · Solfeggio · WAV export" },
    MenuItem { key: "4", icon: "🌏", label: "World Cosmologies",
               hint: "Chinese · African · Nine Star Ki · Ifa" },
    MenuItem { key: "5", icon: "🎲", label: "Psi–RNG Experiment",
               hint: "Can intention influence a hardware TRNG?" },
    MenuItem { key: "6", icon: "⭐", label: "Zodiac & Astrology",
               hint: "Hebrew Mazzaroth · Sefer Yetzirah · Twelve Tribes" },
    MenuItem { key: "7", icon: "❓", label: "Help & Reference",
               hint: "" },
];

static MAIN_MENU: Menu = Menu {
    title:        "✦  CHOOSE YOUR PATH  ✦",
    border_color: MenuColor::Yellow,
    items:        MAIN_ITEMS,
    back_key:     "0",
    back_label:   "Exit",
};

/// Display the top-level menu and return the user's selection.
pub fn show_main_menu() -> MainMode {
    loop {
        match MAIN_MENU.show_and_read().as_str() {
            "1"      => return MainMode::Numerology,
            "2"      => return MainMode::Enochian,
            "3"      => return MainMode::Frequencies,
            "4"      => return MainMode::WorldSystems,
            "5"      => return MainMode::RngExperiment,
            "6"      => return MainMode::Zodiac,
            "7"      => return MainMode::Help,
            "0" | "" => return MainMode::Quit,
            _        => println!("{}", "  Please enter 0–7.".yellow()),
        }
    }
}

// ─── Angel banner ─────────────────────────────────────────────────────────────

/// Print the ASCII angel figure and application title bar.
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

/// Play the animated startup sequence then clear the terminal.
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
        ("Connecting to celestial frequencies",  "🌌"),
        ("Aligning with divine numerology",       "⚡"),
        ("Awakening angelic consciousness",       "👼"),
        ("Tuning into universal vibrations",      "🎵"),
        ("Opening sacred numerical channels",     "🔢"),
        ("Downloading cosmic wisdom",             "📡"),
        ("Initializing spiritual algorithms",     "🧮"),
        ("Calibrating mystical sensors",          "🔮"),
    ];
    let spinners = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let bar_len  = 20;

    for (message, icon) in &messages {
        for frame in 0..25_usize {
            let spinner  = spinners[frame % spinners.len()];
            let progress = (frame * bar_len) / 25;
            let bar      = format!("[{}{}]", "█".repeat(progress), "░".repeat(bar_len - progress));
            print!("\r        {} {} {} {} {}",
                icon, spinner.bright_white(), message.bright_white(),
                bar.bright_blue(), format!("{}%", (frame * 100) / 25).dimmed(),
            );
            io::stdout().flush().unwrap_or(());
            thread::sleep(Duration::from_millis(50));
        }
        let full_bar = format!("[{}]", "█".repeat(bar_len));
        println!("\r        {} ✓ {} {} {}   ",
            icon, message.bright_white(), full_bar.bright_green(), "100%".bright_green());
        thread::sleep(Duration::from_millis(150));
    }

    println!();
    let mystical = [
        "🌟 Synchronizing with akashic records…",
        "⭐ Blessing your spiritual journey…",
        "✨ Opening gates of wisdom…",
        "🌙 Moon cycles aligned…",
        "☄️  Cosmic energies flowing…",
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
    println!("{}", "  Abjad             — Arabic/Islamic letter-number system (hisab al-jumal)".bright_white());
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
    println!("{}", "  African     — Yoruba Ifa (16 Odu), Akan day-soul names, Kemetic sacred numbers".bright_white());
    println!();
    println!("{}", "Visit the interactive mode to analyze words and export personalized frequencies!".italic().bright_blue());
}
