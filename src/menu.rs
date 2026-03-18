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
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::audio::SOLFEGGIO_FREQUENCIES;

// ─── Primitives ───────────────────────────────────────────────────────────────

/// One selectable entry in a [`Menu`].
pub struct MenuItem {
    /// Key the user types to select this item (e.g. `"1"`, `"2"`).
    pub key: &'static str,
    /// Emoji or symbol shown beside the label (assumed 2 display columns).
    pub icon: &'static str,
    /// Primary label text (max ~47 chars; truncated if longer).
    pub label: &'static str,
    /// Dimmed sub-line hint below the label.  Empty string = no hint line.
    pub hint: &'static str,
}

/// A renderable 60-column box-drawing menu.
///
/// Call [`Menu::show_and_read`] to render the menu and return the user's
/// trimmed input, or [`Menu::show`] just to print it.
pub struct Menu {
    /// Centred title in the header row.
    pub title: &'static str,
    /// Colour applied to the box border lines.
    pub border_color: MenuColor,
    /// Selectable items rendered in order.
    pub items: &'static [MenuItem],
    /// Key string for the back / exit footer entry.
    pub back_key: &'static str,
    /// Label for the back / exit footer entry.
    pub back_label: &'static str,
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
const INNER: usize = LINE_W - 2; // 58

impl Menu {
    /// Print the menu to stdout.
    pub fn show(&self) {
        let b = |s: &str| -> ColoredString {
            match self.border_color {
                MenuColor::Yellow => s.bright_yellow(),
                MenuColor::Cyan => s.bright_cyan(),
                MenuColor::White => s.bright_white(),
                MenuColor::Magenta => s.bright_magenta(),
                MenuColor::Green => s.bright_green(),
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
            println!(
                "{}",
                item_line(item.key, item.icon, item.label).bright_white()
            );
            if !item.hint.is_empty() {
                println!("{}", hint_line(item.hint).dimmed());
            }
        }

        println!("{}", b(&empty));
        println!("{}", b(&format!("╠{}╣", horiz)));
        println!(
            "{}",
            item_line(self.back_key, "←", self.back_label).dimmed()
        );
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
    let left = total / 2;
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
    Tarot,
    Runes,
    Urim,
    Help,
    Quit,
}

static MAIN_ITEMS: &[MenuItem] = &[
    MenuItem {
        key: "1",
        icon: "🔢",
        label: "Gematria & Numerology",
        hint: "Analyze words across all eleven systems",
    },
    MenuItem {
        key: "2",
        icon: "📜",
        label: "Enochian Angelology",
        hint: "Alphabet · Aethyrs · Translate · Keys",
    },
    MenuItem {
        key: "3",
        icon: "🎵",
        label: "Sacred Frequencies & Export",
        hint: "Binaural beats · Solfeggio · WAV export",
    },
    MenuItem {
        key: "4",
        icon: "🌏",
        label: "World Cosmologies",
        hint: "Chinese · African · Nine Star Ki · Ifa",
    },
    MenuItem {
        key: "5",
        icon: "🎲",
        label: "Psi–RNG Experiment",
        hint: "Can intention influence a hardware TRNG?",
    },
    MenuItem {
        key: "6",
        icon: "⭐",
        label: "Zodiac & Astrology",
        hint: "Hebrew Mazzaroth · Sefer Yetzirah · Twelve Tribes",
    },
    MenuItem {
        key: "7",
        icon: "🌟",
        label: "Angelic Tarot",
        hint: "Major Arcana · Minor Arcana · Shem HaMephorash · Sacred Draw",
    },
    MenuItem {
        key: "8",
        icon: "ᚠ",
        label: "Runic Traditions",
        hint: "Elder Futhark · Younger · Anglo-Saxon · Armanen · Readings",
    },
    MenuItem {
        key: "9",
        icon: "ℵ",
        label: "Urim & Thummim",
        hint: "Oracle of the High Priest · Breastplate · Binary Oracle",
    },
    MenuItem {
        key: "h",
        icon: "❓",
        label: "Help & Reference",
        hint: "",
    },
];

static MAIN_MENU: Menu = Menu {
    title: "✦  CHOOSE YOUR PATH  ✦",
    border_color: MenuColor::Yellow,
    items: MAIN_ITEMS,
    back_key: "0",
    back_label: "Exit",
};

/// Display the top-level menu and return the user's selection.
pub fn show_main_menu() -> MainMode {
    loop {
        match MAIN_MENU.show_and_read().as_str() {
            "1" => return MainMode::Numerology,
            "2" => return MainMode::Enochian,
            "3" => return MainMode::Frequencies,
            "4" => return MainMode::WorldSystems,
            "5" => return MainMode::RngExperiment,
            "6" => return MainMode::Zodiac,
            "7" => return MainMode::Tarot,
            "8" => return MainMode::Runes,
            "9" => return MainMode::Urim,
            "h" | "H" => return MainMode::Help,
            "0" | "" => return MainMode::Quit,
            _ => println!("{}", "  Please enter 0–9 or h.".yellow()),
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
    println!(
        "{}",
        "Hebrew · Pythagorean · Chaldean · Greek Isopsephy · Agrippan"
            .italic()
            .bright_blue()
    );
    println!(
        "{}",
        "Simple Ordinal · Reverse Ordinal · Abjad · Enochian (John Dee)"
            .italic()
            .bright_blue()
    );
    println!(
        "{}",
        "──────────────────────────────────────────────────────────".dimmed()
    );
}

// ─── Loading screen ───────────────────────────────────────────────────────────

/// Animated ASCII mandala loading screen — mirrors the SVG emblem structure:
///   outer ring  = 12 Mazzaroth / zodiac signs
///   middle ring = 22 Hebrew letters (א–ת)
///   inner ring  = 8 Ba Gua trigrams + 7 classical planet symbols
///   centre      = all-seeing eye / iris (◉)
pub fn show_loading_screen() {
    // ── Canvas geometry ───────────────────────────────────────────────────────
    // All coordinates are 0-indexed (col, row).
    // crossterm::MoveTo(col, row) is column-first — opposite of ANSI row;col.
    const CANVAS_W: i32 = 71;
    const CANVAS_H: i32 = 37;
    const CENTRE_COL: f64 = 35.0;
    const CENTRE_ROW: f64 = 18.0;
    // Ellipse semi-axes for each symbolic ring (col-radius, row-radius).
    const R_ZODIAC_COL: f64 = 13.5; // outer  — 12 Mazzaroth signs
    const R_ZODIAC_ROW: f64 = 6.5;
    const R_HEBREW_COL: f64 = 9.0; // middle — 22 Hebrew letters
    const R_HEBREW_ROW: f64 = 4.5;
    const R_INNER_COL: f64 = 6.0; // inner  — Ba Gua trigrams
    const R_INNER_ROW: f64 = 3.0;

    // Place `s` at 0-indexed (col, row); clips silently if out of bounds.
    let put = |row: i32, col: i32, s: &str| {
        if row >= 0 && col >= 0 && row < CANVAS_H && col < CANVAS_W {
            execute!(io::stdout(), MoveTo(col as u16, row as u16), Print(s)).ok();
        }
    };

    /// Compute (col, row) for symbol `i` of `n` evenly spaced around an
    /// ellipse with semi-axes (r_col, r_row), starting at the top (−π/2)
    /// and advancing clockwise.
    fn ring_pos(r_col: f64, r_row: f64, i: usize, n: usize) -> (i32, i32) {
        let a = -std::f64::consts::PI / 2.0 + i as f64 * 2.0 * std::f64::consts::PI / n as f64;
        let col = (CENTRE_COL + r_col * a.cos()).round() as i32;
        let row = (CENTRE_ROW + r_row * a.sin()).round() as i32;
        (col, row)
    }

    // Flush helper (called after every visual change)
    let flush = || io::stdout().flush().unwrap_or(());

    execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0), Hide).ok();
    flush();

    // ── PHASE 0 · Starfield (scattered background stars) ──────────────────
    let stars: &[(i32, i32, &str)] = &[
        (4, 0, "·"),
        (12, 1, "✦"),
        (23, 0, "·"),
        (34, 1, "✧"),
        (47, 0, "·"),
        (55, 1, "✦"),
        (64, 0, "·"),
        (68, 3, "·"),
        (0, 7, "✦"),
        (1, 18, "·"),
        (2, 27, "·"),
        (5, 35, "✧"),
        (65, 7, "·"),
        (69, 18, "✦"),
        (67, 27, "·"),
        (63, 35, "·"),
        (14, 36, "·"),
        (27, 36, "✧"),
        (44, 36, "·"),
        (54, 36, "✦"),
        (7, 4, "·"),
        (59, 4, "·"),
        (9, 31, "✧"),
        (58, 30, "·"),
        (18, 5, "·"),
        (49, 5, "✦"),
        (20, 30, "·"),
        (50, 30, "·"),
        (15, 11, "·"),
        (53, 11, "·"),
        (16, 24, "·"),
        (52, 24, "·"),
        (3, 14, "·"),
        (61, 22, "✦"),
        (10, 20, "·"),
        (58, 14, "·"),
    ];
    for &(col, row, s) in stars {
        put(row, col, &s.dimmed().to_string());
    }
    flush();
    thread::sleep(Duration::from_millis(350));

    // ── PHASE 1 · Outer zodiac ring (Mazzaroth) ────────────────────────────
    // 12 signs materialise clockwise, one per tick  (rcol=13.5, rrow=6.5)
    let zodiac: &[char] = &[
        '♈', '♉', '♊', '♋', '♌', '♍', '♎', '♏', '♐', '♑', '♒', '♓',
    ];
    for (i, &sign) in zodiac.iter().enumerate() {
        let (col, row) = ring_pos(R_ZODIAC_COL, R_ZODIAC_ROW, i, 12);
        put(row, col, &sign.to_string().bright_cyan().to_string());
        flush();
        thread::sleep(Duration::from_millis(75));
    }
    thread::sleep(Duration::from_millis(180));

    // ── PHASE 2 · Hebrew ring (22 letters, א–ת) ────────────────────────────
    // (rcol=9.0, rrow=4.5)
    let hebrew: &[char] = &[
        'א', 'ב', 'ג', 'ד', 'ה', 'ו', 'ז', 'ח', 'ט', 'י', 'כ', 'ל', 'מ', 'נ', 'ס', 'ע', 'פ', 'צ',
        'ק', 'ר', 'ש', 'ת',
    ];
    for (i, &letter) in hebrew.iter().enumerate() {
        let (col, row) = ring_pos(R_HEBREW_COL, R_HEBREW_ROW, i, 22);
        put(row, col, &letter.to_string().bright_magenta().to_string());
        flush();
        thread::sleep(Duration::from_millis(48));
    }
    thread::sleep(Duration::from_millis(180));

    // ── PHASE 3a · Ba Gua inner ring (rcol=6.0, rrow=3.0) ──────────────────
    let bagua: &[char] = &['☰', '☱', '☲', '☳', '☴', '☵', '☶', '☷'];
    for (i, &tri) in bagua.iter().enumerate() {
        let (col, row) = ring_pos(R_INNER_COL, R_INNER_ROW, i, 8);
        put(row, col, &tri.to_string().bright_yellow().to_string());
        flush();
        thread::sleep(Duration::from_millis(80));
    }

    // ── PHASE 3b · Classical planet symbols (SVG-proportional positions) ───
    // Derived from the SVG's r≈147 px ring, scaled to terminal coords.
    let planets: &[(i32, i32, char)] = &[
        (37, 15, '☉'), // Sun      — upper-right
        (40, 17, '☽'), // Moon     — right
        (39, 19, '♂'), // Mars     — lower-right
        (36, 20, '♀'), // Venus    — lower-centre-right
        (31, 20, '♃'), // Jupiter  — lower-centre-left
        (28, 19, '♄'), // Saturn   — lower-left
        (27, 17, '☿'), // Mercury  — left
    ];
    for &(col, row, sym) in planets {
        put(row, col, &sym.to_string().red().to_string());
        flush();
        thread::sleep(Duration::from_millis(80));
    }
    thread::sleep(Duration::from_millis(200));

    // ── PHASE 4 · Central all-seeing eye ───────────────────────────────────
    // Faint Merkabah triangles (Star of David geometry inside the iris)
    put(15, 35, &"▲".dimmed().to_string());
    put(19, 35, &"▽".dimmed().to_string());
    flush();
    thread::sleep(Duration::from_millis(150));

    // Eye box: 7 wide (cols 32–38), 3 tall (rows 16–18).  Centre = (35, 17).
    // Draw the frame once, then pulse only the iris glyph.
    put(16, 32, &"╭─────╮".bright_white().bold().to_string());
    put(17, 32, &"│".bright_white().bold().to_string());
    put(17, 38, &"│".bright_white().bold().to_string());
    put(18, 32, &"╰─────╯".bright_white().bold().to_string());

    for (glyph, delay) in [("·", 200u64), ("○", 180), ("◉", 160)] {
        put(17, 33, "  "); // clear inner
        put(17, 36, "  ");
        put(17, 35, &glyph.bold().bright_yellow().to_string());
        flush();
        thread::sleep(Duration::from_millis(delay));
    }

    // Settled iris: radial spokes flanking the pupil
    put(17, 33, &"·".dimmed().to_string());
    put(17, 37, &"·".dimmed().to_string());
    put(17, 35, &"◉".bold().bright_yellow().to_string());
    flush();
    thread::sleep(Duration::from_millis(400));

    // ── PHASE 5 · Title ────────────────────────────────────────────────────
    let title = "✦  C O S M I C   K N O W L E D G E  ✦";
    let subtitle = "Celestial Numerology · Sacred Wisdom";
    let t_col = ((CANVAS_W - title.chars().count() as i32) / 2).max(0);
    let s_col = ((CANVAS_W - subtitle.chars().count() as i32) / 2).max(0);
    put(34, t_col, &title.bold().bright_magenta().to_string());
    flush();
    thread::sleep(Duration::from_millis(280));
    put(35, s_col, &subtitle.italic().dimmed().to_string());
    flush();
    thread::sleep(Duration::from_millis(1300));

    // ── Teardown ───────────────────────────────────────────────────────────
    execute!(io::stdout(), Show, Clear(ClearType::All), MoveTo(0, 0)).ok();
    flush();
}

// ─── Help screen ──────────────────────────────────────────────────────────────

/// Print the full help and reference screen.
pub fn show_help() {
    println!(
        "{}",
        "╔════════════════════════════════════════════════════════╗".bright_cyan()
    );
    println!(
        "{}",
        "║          🌟 CELESTIAL NUMEROLOGY ANALYZER 🌟           ║".bright_cyan()
    );
    println!(
        "{}",
        "║              Sacred Frequency Generator               ║".bright_cyan()
    );
    println!(
        "{}",
        "╚════════════════════════════════════════════════════════╝".bright_cyan()
    );
    println!();
    println!("{}", "USAGE:".bold());
    println!("  cargo run [FLAGS]");
    println!();
    println!("{}", "FLAGS:".bold());
    println!(
        "{}",
        "  -f, --fast              Skip the mystical loading animation".bright_white()
    );
    println!(
        "{}",
        "  -s, --silent            Disable audio frequencies".bright_white()
    );
    println!(
        "{}",
        "  --export-all            Export all Solfeggio frequencies to WAV files".bright_white()
    );
    println!(
        "{}",
        "  --aethyr <n|name>       Look up a specific Aethyr (e.g. --aethyr 10 or --aethyr ZAX)"
            .bright_white()
    );
    println!(
        "{}",
        "  --aethyr                Show all 30 Aethyrs".bright_white()
    );
    println!(
        "{}",
        "  -h, --help              Show this help message".bright_white()
    );
    println!();
    println!("{}", "NUMEROLOGICAL SYSTEMS:".bold());
    println!(
        "{}",
        "  Hebrew Gematria   — Traditional letter-values (A=1 … Z=900, Mispar Hechrachi)"
            .bright_white()
    );
    println!(
        "{}",
        "  Pythagorean       — Western 1-9 cyclical mapping (mod-9 of alphabet position)"
            .bright_white()
    );
    println!(
        "{}",
        "  Chaldean          — Ancient Babylonian vibrational system (9 sacred, unassigned)"
            .bright_white()
    );
    println!(
        "{}",
        "  Greek Isopsephy   — Classical Greek letter-number system (Neoplatonic tradition)"
            .bright_white()
    );
    println!(
        "{}",
        "  Agrippan          — Cornelius Agrippa/Francis Barrett Latin gematria (c.1531/1801)"
            .bright_white()
    );
    println!(
        "{}",
        "  Simple Ordinal    — Direct alphabetical position 1-26 (modern English Gematria)"
            .bright_white()
    );
    println!(
        "{}",
        "  Reverse Ordinal   — Mirror complement: Z=1 … A=26".bright_white()
    );
    println!(
        "{}",
        "  Abjad             — Arabic/Islamic letter-number system (hisab al-jumal)".bright_white()
    );
    println!(
        "{}",
        "  Enochian Ordinal  — John Dee's angelic alphabet, positional values 1-21".bright_white()
    );
    println!(
        "{}",
        "  Enochian G.D.     — Golden Dawn's Hebrew-mapped Enochian values".bright_white()
    );
    println!();
    println!("{}", "SOLFEGGIO FREQUENCIES:".bold());
    for (freq, _, icon) in SOLFEGGIO_FREQUENCIES {
        let name = crate::audio::get_frequency_name(*freq);
        println!(
            "{}",
            format!("  {} {} Hz — {}", icon, *freq as u32, name).bright_magenta()
        );
    }
    println!();
    println!("{}", "EXPORT FORMATS:".bold());
    println!("{}", "  • WAV files (44.1 kHz, 16-bit)".bright_white());
    println!(
        "{}",
        "  • Pure tones (mono) or Binaural beats (stereo)".bright_white()
    );
    println!("{}", "  • Durations: 5, 10, or 30 minutes".bright_white());
    println!(
        "{}",
        "  • Custom frequency combinations available".bright_white()
    );
    println!();
    println!("{}", "WORLD COSMOLOGIES:".bold());
    println!(
        "{}",
        "  Chinese     — Nine Star Ki natal star, Wu Xing Five Elements, lucky/unlucky numbers"
            .bright_white()
    );
    println!(
        "{}",
        "  African     — Yoruba Ifa (16 Odu), Akan day-soul names, Kemetic sacred numbers"
            .bright_white()
    );
    println!();
    println!(
        "{}",
        "Visit the interactive mode to analyze words and export personalized frequencies!"
            .italic()
            .bright_blue()
    );
}
