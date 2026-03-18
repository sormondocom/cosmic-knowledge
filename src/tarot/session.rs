//! Interactive Angelic Tarot session — all terminal I/O for the Tarot sub-menu.
//!
//! Functions:
//!  - [`run_tarot_session`]         — top-level sub-menu loop
//!  - `browse_major_arcana`         — display all 22 Major Arcana
//!  - `lookup_major`                — look up one card by number or name
//!  - `browse_minor_by_suit`        — display all 14 cards in a chosen suit
//!  - `browse_shem_hamephorash`     — display all 72 Shem HaMephorash angels
//!  - `draw_reading`                — 1- or 3-card sacred draw using hardware RNG

use colored::*;
use std::io::{self, Write};

use rdrand::RdRand;

use crate::export::{handle_export, wrap_html};
use crate::menu::{Menu, MenuColor, MenuItem};
use crate::persistence::{get_or_create_user, open_db, record_reading};
use crate::reports::chrono_now;

use super::lenormand::{lenormand_by_name, lenormand_by_number, LENORMAND};
use super::major::{major_by_name, major_by_number, MajorArcanum, MAJOR_ARCANA};
use super::minor::{suit_cards, MinorArcanum, MINOR_ARCANA, SHEM_HAMEPHORASH};
use super::oh_cards::{oh_image_by_number, oh_word_by_number, OH_IMAGES, OH_WORDS};
use super::oracle::{oracle_by_number, oracle_suit_cards, OracleCard, ORACLE};

// ─── Colour palette ───────────────────────────────────────────────────────────
// Gold / violet / soft-cyan palette to evoke illuminated manuscript aesthetics.

const SUIT_ICONS: &[&str] = &["🔥", "💧", "⚡", "🌿"];

// ─── Sub-menu definition ──────────────────────────────────────────────────────

static TAROT_ITEMS: &[MenuItem] = &[
    MenuItem {
        key: "1",
        icon: "🌟",
        label: "Angelic Tarot — Major Arcana (22 cards)",
        hint: "Browse · Look up · Kabbalistic Tree paths · Angels",
    },
    MenuItem {
        key: "2",
        icon: "🃏",
        label: "Angelic Tarot — Minor Arcana (56 cards)",
        hint: "Wands · Cups · Swords · Pentacles · Shem HaMephorash",
    },
    MenuItem {
        key: "3",
        icon: "✨",
        label: "The 72 Shem HaMephorash Angels",
        hint: "God's seventy-two names · 5° zodiacal rulers",
    },
    MenuItem {
        key: "4",
        icon: "🔮",
        label: "Draw a Sacred Reading (Tarot)",
        hint: "1-card or 3-card spread · Full 78-card deck · TRNG",
    },
    MenuItem {
        key: "5",
        icon: "🌿",
        label: "Lenormand Oracle (36 cards)",
        hint: "Petit jeu · Folk cartomancy · Combinatorial tradition",
    },
    MenuItem {
        key: "6",
        icon: "💫",
        label: "Angelic Oracle Deck (44 cards)",
        hint: "Four elemental suits · Divine guidance · Affirmations",
    },
    MenuItem {
        key: "7",
        icon: "🖼",
        label: "OH Cards — Image & Word Draw (88 cards)",
        hint: "Projective psychology tool · Image + Word pairs",
    },
    MenuItem {
        key: "8",
        icon: "📖",
        label: "View Reading History",
        hint: "Browse all recorded readings by user or globally",
    },
    MenuItem {
        key: "9",
        icon: "🔯",
        label: "Crowley Thoth Tarot  (78 cards)",
        hint: "Major Arcana · Minor Arcana (Disks) · Draw · Book of Thoth",
    },
];

static TAROT_MENU: Menu = Menu {
    title: "🌟  CARD TRADITIONS & ORACLE TOOLS  🌟",
    border_color: MenuColor::Magenta,
    items: TAROT_ITEMS,
    back_key: "0",
    back_label: "Back to Main Menu",
};

// ─── Major Arcana sub-menu items ──────────────────────────────────────────────

static MAJOR_ITEMS: &[MenuItem] = &[
    MenuItem {
        key: "1",
        icon: "📜",
        label: "Browse All 22 Major Arcana",
        hint: "Full table with angelic correspondences",
    },
    MenuItem {
        key: "2",
        icon: "🔢",
        label: "Look Up by Card Number (0–21)",
        hint: "Enter a number to see the full card profile",
    },
    MenuItem {
        key: "3",
        icon: "🔍",
        label: "Look Up by Card Name",
        hint: "Partial match — e.g. \"fool\", \"tower\", \"hermit\"",
    },
];

static MAJOR_MENU: Menu = Menu {
    title: "🌟  MAJOR ARCANA  ·  The 22 Trumps",
    border_color: MenuColor::Yellow,
    items: MAJOR_ITEMS,
    back_key: "0",
    back_label: "Back to Tarot Menu",
};

// ─── Session entry point ──────────────────────────────────────────────────────

/// Run the card traditions sub-menu loop until the user exits.
pub fn run_tarot_session() {
    loop {
        match TAROT_MENU.show_and_read().as_str() {
            "1" => major_arcana_menu(),
            "2" => minor_arcana_menu(),
            "3" => browse_shem_hamephorash(),
            "4" => draw_reading(),
            "5" => lenormand_session(),
            "6" => oracle_session(),
            "7" => oh_cards_session(),
            "8" => view_reading_history(),
            "9" => thoth_session(),
            "0" | "" => break,
            _ => println!("{}", "  Please enter 0–9.".yellow()),
        }
    }
}

// ─── Major Arcana section ─────────────────────────────────────────────────────

fn major_arcana_menu() {
    loop {
        match MAJOR_MENU.show_and_read().as_str() {
            "1" => browse_major_arcana(),
            "2" => lookup_major_by_number(),
            "3" => lookup_major_by_name(),
            "0" | "" => break,
            _ => println!("{}", "  Please enter 0–3.".yellow()),
        }
    }
}

fn browse_major_arcana() {
    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_magenta()
    );
    println!(
        "{}",
        "  ║        🌟  THE TWENTY-TWO MAJOR ARCANA  🌟              ║"
            .bold()
            .bright_magenta()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_magenta()
    );
    println!(
        "{}",
        "  ║  #   Card Name              Angel        Element        ║".dimmed()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_magenta()
    );

    for card in MAJOR_ARCANA {
        let name_short: String = card.name.chars().take(22).collect();
        let angel_short: String = card.angel.chars().take(12).collect();
        let elem_short: String = card.element.chars().take(14).collect();
        println!(
            "  {}",
            format!(
                "║  {:>2}. {:<22}  {:<12}  {:<14} ║",
                card.number, name_short, angel_short, elem_short
            )
            .bright_white()
        );
    }

    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_magenta()
    );
    println!();
    // Export prompt
    handle_export(
        "major_arcana_table",
        || {
            let mut s = format!("THE TWENTY-TWO MAJOR ARCANA\nGenerated: {}\n\n", chrono_now());
            s.push_str(&format!("{:<4} {:<24} {:<14} {}\n", "#", "Name", "Angel", "Element"));
            s.push_str(&"-".repeat(70));
            s.push('\n');
            for c in MAJOR_ARCANA {
                s.push_str(&format!("{:<4} {:<24} {:<14} {}\n", c.number, c.name, c.angel, c.element));
            }
            s
        },
        || {
            let mut rows = String::new();
            for c in MAJOR_ARCANA {
                rows.push_str(&format!(
                    "<tr><td class=\"num\">{}</td><td class=\"sys\">{}</td>\
                     <td>{}</td><td>{}</td><td class=\"meaning\">{}</td></tr>",
                    c.number, tarot_esc(c.name), tarot_esc(c.angel),
                    tarot_esc(c.element), tarot_esc(c.keywords)
                ));
            }
            let body = format!(
                "<table><thead><tr><th>#</th><th>Name</th><th>Angel</th>\
                 <th>Element</th><th>Keywords</th></tr></thead><tbody>{}</tbody></table>",
                rows
            );
            wrap_html("The Twenty-Two Major Arcana", &body, "")
        },
    );
    println!(
        "{}",
        "  Enter a card number (0–21) for the full profile, or press Enter to return:".cyan()
    );
    print!("  {} ", "▸".bold().cyan());
    io::stdout().flush().unwrap_or(());

    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_ok() {
        let t = buf.trim();
        if let Ok(n) = t.parse::<u8>() {
            if n <= 21 {
                if let Some(card) = major_by_number(n) {
                    print_major_card(card);
                }
            }
        }
    }
}

fn lookup_major_by_number() {
    loop {
        println!();
        print!(
            "{}",
            "  ▸ Enter card number (0–21), or press Enter to go back: ".cyan()
        );
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() {
            break;
        }
        let t = buf.trim();
        if t.is_empty() {
            break;
        }
        match t.parse::<u8>() {
            Ok(n) if n <= 21 => match major_by_number(n) {
                Some(card) => print_major_card(card),
                None => println!("{}", "  Card not found.".yellow()),
            },
            _ => println!("{}", "  Please enter a number between 0 and 21.".yellow()),
        }
    }
}

fn lookup_major_by_name() {
    loop {
        println!();
        print!(
            "{}",
            "  ▸ Enter card name (partial match), or press Enter to go back: ".cyan()
        );
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() {
            break;
        }
        let t = buf.trim();
        if t.is_empty() {
            break;
        }
        match major_by_name(t) {
            Some(card) => print_major_card(card),
            None => println!(
                "  {}",
                format!("No Major Arcana card matching '{}'.", t).yellow()
            ),
        }
    }
}

/// Print the full profile of one Major Arcana card.
pub fn print_major_card(card: &MajorArcanum) {
    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_magenta()
    );
    println!(
        "  {}",
        format!(
            "║  {:<2}. ✦ {:<46}   ║",
            card.number,
            card.name.to_uppercase()
        )
        .bold()
        .bright_yellow()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_magenta()
    );

    let rows: &[(&str, &str)] = &[
        ("Hebrew Letter", card.hebrew_letter),
        ("Tree of Life", &format!("Path {:>2} — {}", card.path, card.sephiroth)),
        ("Element / Star", card.element),
        ("Angel", card.angel),
        ("Angelic Order", card.angelic_order),
    ];

    for (label, value) in rows {
        let val_short: String = value.chars().take(43).collect();
        println!(
            "  {}",
            format!("║  {:<14}: {:<43}  ║", label, val_short).bright_white()
        );
        // Wrap long values
        if value.chars().count() > 43 {
            let rest: String = value.chars().skip(43).collect();
            let rest_short: String = rest.chars().take(43).collect();
            println!(
                "  {}",
                format!("║  {:<14}  {:<43}  ║", "", rest_short).dimmed()
            );
        }
    }

    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_magenta()
    );
    println!(
        "  {}",
        "║  Divine Quality:                                          ║".dimmed()
    );
    wrap_box_line(card.divine_quality, 56);

    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_magenta()
    );
    println!(
        "  {}",
        "║  Keywords:                                                ║".dimmed()
    );
    wrap_box_line(card.keywords, 56);

    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_magenta()
    );
    println!();
}

// ─── Minor Arcana section ─────────────────────────────────────────────────────

static MINOR_ITEMS: &[MenuItem] = &[
    MenuItem {
        key: "1",
        icon: "🔥",
        label: "Wands — Fire · Aries · Leo · Sagittarius",
        hint: "Ace through King · Shem HaMephorash decans",
    },
    MenuItem {
        key: "2",
        icon: "💧",
        label: "Cups — Water · Cancer · Scorpio · Pisces",
        hint: "Ace through King · Shem HaMephorash decans",
    },
    MenuItem {
        key: "3",
        icon: "⚡",
        label: "Swords — Air · Libra · Aquarius · Gemini",
        hint: "Ace through King · Shem HaMephorash decans",
    },
    MenuItem {
        key: "4",
        icon: "🌿",
        label: "Pentacles — Earth · Capricorn · Taurus · Virgo",
        hint: "Ace through King · Shem HaMephorash decans",
    },
];

static MINOR_MENU: Menu = Menu {
    title: "🃏  MINOR ARCANA  ·  Choose a Suit",
    border_color: MenuColor::Cyan,
    items: MINOR_ITEMS,
    back_key: "0",
    back_label: "Back to Tarot Menu",
};

fn minor_arcana_menu() {
    loop {
        match MINOR_MENU.show_and_read().as_str() {
            "1" => browse_suit("Wands"),
            "2" => browse_suit("Cups"),
            "3" => browse_suit("Swords"),
            "4" => browse_suit("Pentacles"),
            "0" | "" => break,
            _ => println!("{}", "  Please enter 0–4.".yellow()),
        }
    }
}

fn browse_suit(suit: &str) {
    let cards = suit_cards(suit);
    if cards.is_empty() {
        println!("  {}", format!("No cards found for suit '{}'.", suit).yellow());
        return;
    }

    let (icon, color_fn): (&&str, fn(&str) -> ColoredString) = match suit {
        "Wands" => (&&SUIT_ICONS[0], |s: &str| s.bright_red()),
        "Cups" => (&&SUIT_ICONS[1], |s: &str| s.bright_cyan()),
        "Swords" => (&&SUIT_ICONS[2], |s: &str| s.bright_white()),
        _ => (&&SUIT_ICONS[3], |s: &str| s.bright_green()),
    };

    println!();
    println!(
        "  {}",
        color_fn(&format!(
            "╔══════════════════════════════════════════════════════════╗"
        ))
    );
    println!(
        "  {}",
        color_fn(&format!(
            "║  {}  {}  —  {} element                                ║",
            icon,
            suit.to_uppercase(),
            cards[0].element
        ))
        .bold()
    );
    println!(
        "  {}",
        color_fn("╠══════════════════════════════════════════════════════════╣")
    );
    println!(
        "  {}",
        "║  Rank          Angel              Decan / Notes          ║".dimmed()
    );
    println!(
        "  {}",
        color_fn("╠══════════════════════════════════════════════════════════╣")
    );

    for card in &cards {
        let rank_field: String = card.rank_name.chars().take(10).collect();
        let angel_field = if card.angel2.is_empty() {
            card.angel.chars().take(18).collect::<String>()
        } else {
            format!("{} & {}", card.angel, card.angel2)
                .chars()
                .take(18)
                .collect()
        };
        let decan_field: String = if card.sub_element.is_empty() {
            card.decan.chars().take(22).collect()
        } else {
            card.sub_element.chars().take(22).collect()
        };
        println!(
            "  {}",
            color_fn(&format!(
                "║  {:<10}  {:<18}  {:<22} ║",
                rank_field, angel_field, decan_field
            ))
        );
    }

    println!(
        "  {}",
        color_fn("╚══════════════════════════════════════════════════════════╝")
    );
    println!();
    println!(
        "  {}",
        "Enter a rank name for full details (e.g. \"Ace\", \"Five\", \"Queen\"), or Enter to return:"
            .cyan()
    );

    loop {
        print!("  {} ", "▸".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() {
            break;
        }
        let t = buf.trim();
        if t.is_empty() {
            break;
        }
        match crate::tarot::minor::minor_by_suit_rank(suit, t) {
            Some(card) => print_minor_card(card),
            None => println!(
                "  {}",
                format!("No {} card matching '{}'.", suit, t).yellow()
            ),
        }
    }
}

/// Print the full profile of one Minor Arcana card.
pub fn print_minor_card(card: &MinorArcanum) {
    let (icon, border) = match card.suit {
        "Wands" => ("🔥", "bright_red"),
        "Cups" => ("💧", "bright_cyan"),
        "Swords" => ("⚡", "bright_white"),
        _ => ("🌿", "bright_green"),
    };

    let colorize = |s: &str| -> ColoredString {
        match border {
            "bright_red" => s.bright_red(),
            "bright_cyan" => s.bright_cyan(),
            "bright_white" => s.bright_white(),
            _ => s.bright_green(),
        }
    };

    let angel_display = if card.angel2.is_empty() {
        card.angel.to_string()
    } else {
        format!("{} & {}", card.angel, card.angel2)
    };

    let decan_display = if card.sub_element.is_empty() && card.decan.is_empty() {
        "Pure elemental force".to_string()
    } else if !card.sub_element.is_empty() {
        format!("{} ({})", card.sub_element, card.element)
    } else {
        card.decan.to_string()
    };

    println!();
    println!(
        "  {}",
        colorize("╔══════════════════════════════════════════════════════════╗")
    );
    println!(
        "  {}",
        colorize(&format!(
            "║  {}  {} of {}  {:<39} ║",
            icon,
            card.rank_name.to_uppercase(),
            card.suit.to_uppercase(),
            ""
        ))
        .bold()
        .bright_yellow()
    );
    println!(
        "  {}",
        colorize("╠══════════════════════════════════════════════════════════╣")
    );

    let rows: &[(&str, String)] = &[
        ("Element", card.element.to_string()),
        ("Angel(s)", angel_display),
        ("Decan / Nature", decan_display),
    ];

    for (label, value) in rows {
        let val_short: String = value.chars().take(43).collect();
        println!(
            "  {}",
            format!("║  {:<14}: {:<43}  ║", label, val_short).bright_white()
        );
    }

    println!(
        "  {}",
        colorize("╠══════════════════════════════════════════════════════════╣")
    );
    println!(
        "  {}",
        "║  Divine Quality:                                          ║".dimmed()
    );
    wrap_box_line(card.divine_quality, 56);

    println!(
        "  {}",
        colorize("╠══════════════════════════════════════════════════════════╣")
    );
    println!(
        "  {}",
        "║  Keywords:                                                ║".dimmed()
    );
    wrap_box_line(card.keywords, 56);

    println!(
        "  {}",
        colorize("╚══════════════════════════════════════════════════════════╝")
    );
    println!();
}

// ─── Shem HaMephorash ─────────────────────────────────────────────────────────

fn browse_shem_hamephorash() {
    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_yellow()
    );
    println!(
        "{}",
        "  ║  ✦  THE 72 ANGELS OF THE SHEM HAMEPHORASH  ✦           ║"
            .bold()
            .bright_yellow()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_yellow()
    );
    println!(
        "{}",
        "  ║  Derived from Exodus 14:19–21 via boustrophedon cipher  ║".dimmed()
    );
    println!(
        "{}",
        "  ║  Each angel governs 5° of the zodiacal circle           ║".dimmed()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_yellow()
    );
    println!(
        "{}",
        "  ║  #   Name          Root   Degrees                       ║".dimmed()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_yellow()
    );

    for angel in SHEM_HAMEPHORASH {
        let name_f: String = angel.name.chars().take(13).collect();
        let root_f: String = angel.hebrew_root.chars().take(4).collect();
        let deg_f: String = angel.degrees.chars().take(24).collect();
        println!(
            "  {}",
            format!(
                "║  {:>2}. {:<13}  {:<4}  {:<24}         ║",
                angel.number, name_f, root_f, deg_f
            )
            .bright_white()
        );
    }

    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_yellow()
    );
    println!();

    // Export the Shem HaMephorash table
    handle_export(
        "shem_hamephorash_72_angels",
        || {
            let mut s = format!("THE 72 ANGELS OF THE SHEM HAMEPHORASH\nGenerated: {}\n\n", chrono_now());
            s.push_str(&format!("{:<4} {:<15} {:<6} {}\n", "#", "Name", "Root", "Degrees"));
            s.push_str(&"-".repeat(70));
            s.push('\n');
            for a in SHEM_HAMEPHORASH {
                s.push_str(&format!("{:<4} {:<15} {:<6} {}\n", a.number, a.name, a.hebrew_root, a.degrees));
            }
            s
        },
        || {
            let mut rows = String::new();
            for a in SHEM_HAMEPHORASH {
                rows.push_str(&format!(
                    "<tr><td class=\"num\">{}</td><td class=\"sys\">{}</td>\
                     <td>{}</td><td>{}</td><td class=\"meaning\">{}</td></tr>",
                    a.number, tarot_esc(a.name), tarot_esc(a.hebrew_root),
                    tarot_esc(a.degrees), tarot_esc(a.quality)
                ));
            }
            let body = format!(
                "<p style=\"font-size:8.5pt;margin-bottom:6pt;\">Derived from Exodus \
                 14:19&ndash;21 via boustrophedon cipher. Each angel governs 5&deg; \
                 of the zodiacal circle.</p>\
                 <table><thead><tr><th>#</th><th>Name</th><th>Hebrew Root</th>\
                 <th>Degrees</th><th>Quality</th></tr></thead><tbody>{}</tbody></table>",
                rows
            );
            wrap_html("The 72 Angels of the Shem HaMephorash", &body, "hebrew")
        },
    );

    println!(
        "  {}",
        "Enter an angel number (1–72) for full details, or press Enter to return:".cyan()
    );

    loop {
        print!("  {} ", "▸".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() {
            break;
        }
        let t = buf.trim();
        if t.is_empty() {
            break;
        }
        match t.parse::<u8>() {
            Ok(n) if n >= 1 && n <= 72 => match crate::tarot::minor::shem_by_number(n) {
                Some(angel) => {
                    println!();
                    println!(
                        "  {}",
                        "╔══════════════════════════════════════════════════════════╗"
                            .bright_yellow()
                    );
                    println!(
                        "  {}",
                        format!(
                            "║  {:>2}. ✦  {}  ({})                        ║",
                            angel.number, angel.name, angel.hebrew_root
                        )
                        .bold()
                        .bright_yellow()
                    );
                    println!(
                        "  {}",
                        "╠══════════════════════════════════════════════════════════╣"
                            .bright_yellow()
                    );
                    println!(
                        "  {}",
                        format!("║  Degrees:  {:<47} ║", angel.degrees).bright_white()
                    );
                    println!(
                        "  {}",
                        "╠══════════════════════════════════════════════════════════╣"
                            .bright_yellow()
                    );
                    println!(
                        "  {}",
                        "║  Divine Quality:                                          ║".dimmed()
                    );
                    wrap_box_line(angel.quality, 56);
                    println!(
                        "  {}",
                        "╚══════════════════════════════════════════════════════════╝"
                            .bright_yellow()
                    );
                    println!();
                }
                None => println!("{}", "  Angel not found.".yellow()),
            },
            _ => println!("{}", "  Please enter a number between 1 and 72.".yellow()),
        }
    }
}

// ─── Sacred reading ───────────────────────────────────────────────────────────

static READING_ITEMS: &[MenuItem] = &[
    MenuItem {
        key: "1",
        icon: "🕯",
        label: "Single Card — One Sacred Insight",
        hint: "One card drawn from the full 78-card deck",
    },
    MenuItem {
        key: "2",
        icon: "🌙",
        label: "Three-Card Spread — Past · Present · Future",
        hint: "Three cards drawn without replacement",
    },
];

static READING_MENU: Menu = Menu {
    title: "🔮  SACRED CARD READING",
    border_color: MenuColor::Magenta,
    items: READING_ITEMS,
    back_key: "0",
    back_label: "Back to Tarot Menu",
};

fn draw_reading() {
    // Ask for the querent's name before drawing
    print!(
        "{}",
        "  ▸ Your name for the record (Enter for anonymous): "
            .bold()
            .bright_yellow()
    );
    io::stdout().flush().unwrap_or(());
    let mut name_buf = String::new();
    io::stdin().read_line(&mut name_buf).unwrap_or(0);
    let querent = name_buf.trim().to_string();
    let querent = if querent.is_empty() {
        "Anonymous".to_string()
    } else {
        querent
    };

    loop {
        match READING_MENU.show_and_read().as_str() {
            "1" => do_draw(1, &querent),
            "2" => do_draw(3, &querent),
            "0" | "" => break,
            _ => println!("{}", "  Please enter 0–2.".yellow()),
        }
    }
}

/// Draw `count` unique cards from the full 78-card deck using the hardware TRNG.
fn do_draw(count: usize, querent: &str) {
    // Build a flat index over the 78 cards (22 Major + 56 Minor).
    // We represent them as indices: 0–21 = Major, 22–77 = Minor.
    let total = 22 + MINOR_ARCANA.len(); // always 78
    let mut indices: Vec<usize> = (0..total).collect();

    // Use hardware TRNG if available, fall back to OS entropy.
    let rng_hw = RdRand::new().ok();

    // Fisher-Yates shuffle for the first `count` positions.
    for i in 0..count.min(total) {
        let rand_val = next_rnd(&rng_hw);
        let j = i + (rand_val as usize % (total - i));
        indices.swap(i, j);
    }

    let labels = ["Past", "Present", "Future"];

    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_magenta()
    );
    let header = if count == 1 {
        "  ║           🔮  YOUR SACRED CARD  🔮                       ║"
    } else {
        "  ║        🔮  YOUR THREE-CARD READING  🔮                    ║"
    };
    println!("{}", header.bold().bright_magenta());
    println!(
        "{}",
        "  ║         Drawn by sacred hardware randomness               ║".dimmed()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_magenta()
    );
    println!();

    // Sing an angelic hymn before the reading is revealed
    let hymn_title = display_angelic_hymn(&rng_hw);

    print!(
        "{}",
        "  ✦  Press Enter to reveal your reading...  ✦  "
            .italic()
            .bright_yellow()
    );
    io::stdout().flush().unwrap_or(());
    let mut _pause = String::new();
    io::stdin().read_line(&mut _pause).unwrap_or(0);
    println!();

    for (draw_i, &card_idx) in indices[..count].iter().enumerate() {
        if count > 1 {
            println!(
                "  {}",
                format!("── {} ──────────────────────────────", labels[draw_i])
                    .bold()
                    .bright_cyan()
            );
            println!();
        }

        if card_idx < 22 {
            // Major Arcana
            if let Some(card) = major_by_number(card_idx as u8) {
                print_major_card(card);
            }
        } else {
            // Minor Arcana
            let minor_idx = card_idx - 22;
            if minor_idx < MINOR_ARCANA.len() {
                print_minor_card(&MINOR_ARCANA[minor_idx]);
            }
        }
    }

    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_magenta()
    );
    println!(
        "{}",
        "  ║  ✦  May the angels illumine the path revealed here  ✦   ║"
            .italic()
            .bright_yellow()
    );
    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_magenta()
    );
    println!();

    // Persist the reading
    let drawn_indices: Vec<usize> = indices[..count].to_vec();
    let spread = if count == 1 { "Single Card" } else { "Three-Card Reading" };
    {
        let cards_text = drawn_indices
            .iter()
            .map(|&idx| {
                if idx < 22 {
                    major_by_number(idx as u8)
                        .map(|c| format!("Major #{}: {}", c.number, c.name))
                        .unwrap_or_default()
                } else if idx - 22 < MINOR_ARCANA.len() {
                    let m = &MINOR_ARCANA[idx - 22];
                    format!("{} of {}", m.rank_name, m.suit)
                } else {
                    String::new()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        if let Ok(conn) = open_db() {
            if let Ok((user, _)) = get_or_create_user(&conn, querent) {
                record_reading(&conn, &user.id, "Angelic Tarot", spread, &cards_text).ok();
            }
        }
    }

    // Export the reading
    handle_export(
        &format!("tarot_reading_{}", spread.to_lowercase().replace(' ', "_").replace('-', "_")),
        || {
            let lbs = ["Past", "Present", "Future"];
            let mut s = format!("TAROT READING — {}\nHymn: {}\nGenerated: {}\n\n", spread, hymn_title, chrono_now());
            for (i, &idx) in drawn_indices.iter().enumerate() {
                if count > 1 { s.push_str(&format!("── {} ──\n", lbs[i])); }
                if idx < 22 {
                    if let Some(c) = major_by_number(idx as u8) {
                        s.push_str(&format!("MAJOR ARCANA #{}: {}\n", c.number, c.name));
                        s.push_str(&format!("Angel: {} | Element: {}\n", c.angel, c.element));
                        s.push_str(&format!("Keywords: {}\n\n", c.keywords));
                    }
                } else if idx - 22 < MINOR_ARCANA.len() {
                    let m = &MINOR_ARCANA[idx - 22];
                    s.push_str(&format!("{} of {}\n", m.rank_name, m.suit));
                    s.push_str(&format!("Angel: {} | Element: {}\n", m.angel, m.element));
                    s.push_str(&format!("Keywords: {}\n\n", m.keywords));
                }
            }
            s
        },
        || {
            let lbs = ["Past", "Present", "Future"];
            let mut rows = String::new();
            for (i, &idx) in drawn_indices.iter().enumerate() {
                let pos = if count > 1 { lbs.get(i).copied().unwrap_or("") } else { "" };
                if idx < 22 {
                    if let Some(c) = major_by_number(idx as u8) {
                        rows.push_str(&format!(
                            "<tr><td class=\"sys\">Major #{}: {}</td><td>{}</td>\
                             <td>{}</td><td class=\"meaning\">{}</td></tr>",
                            c.number, tarot_esc(c.name), tarot_esc(pos),
                            tarot_esc(c.angel), tarot_esc(c.keywords)
                        ));
                    }
                } else if idx - 22 < MINOR_ARCANA.len() {
                    let m = &MINOR_ARCANA[idx - 22];
                    rows.push_str(&format!(
                        "<tr><td class=\"sys\">{} of {}</td><td>{}</td>\
                         <td>{}</td><td class=\"meaning\">{}</td></tr>",
                        tarot_esc(m.rank_name), tarot_esc(m.suit), tarot_esc(pos),
                        tarot_esc(m.angel), tarot_esc(m.keywords)
                    ));
                }
            }
            let body = format!(
                "<h2 style=\"color:var(--accent);\">{}</h2>\
                 <p class=\"meta\">{}</p>\
                 <table><thead><tr><th>Card</th><th>Position</th>\
                 <th>Angel</th><th>Keywords</th></tr></thead>\
                 <tbody>{}</tbody></table>",
                tarot_esc(spread), tarot_esc(&chrono_now()), rows
            );
            wrap_html(&format!("Tarot Reading — {}", spread), &body, "")
        },
    );
}

// ─── Lenormand session ────────────────────────────────────────────────────────

static LENORMAND_ITEMS: &[MenuItem] = &[
    MenuItem {
        key: "1",
        icon: "📋",
        label: "Browse All 36 Lenormand Cards",
        hint: "Full table with playing-card insets and domains",
    },
    MenuItem {
        key: "2",
        icon: "🔢",
        label: "Look Up by Card Number (1–36)",
        hint: "Full card profile with meanings and reading note",
    },
    MenuItem {
        key: "3",
        icon: "🔍",
        label: "Look Up by Card Name",
        hint: "Partial match — e.g. \"ship\", \"key\", \"clover\"",
    },
    MenuItem {
        key: "4",
        icon: "🎴",
        label: "Draw a Lenormand Reading",
        hint: "1, 3, or 5 cards · Combinatorial folk tradition",
    },
];

static LENORMAND_MENU: Menu = Menu {
    title: "🌿  LENORMAND  ·  36-Card Petit Jeu",
    border_color: MenuColor::Green,
    items: LENORMAND_ITEMS,
    back_key: "0",
    back_label: "Back to Card Menu",
};

fn lenormand_session() {
    // Print the historical disclaimer before the menu
    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_green()
    );
    println!(
        "{}",
        "  ║  ⚠  Note: Lenormand is NOT a Tarot system               ║"
            .bold()
            .bright_green()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_green()
    );
    println!(
        "{}",
        "  ║  The Lenormand petit jeu is a distinct 36-card folk      ║".dimmed()
    );
    println!(
        "{}",
        "  ║  cartomantic tradition, not a Tarot system.  It carries  ║".dimmed()
    );
    println!(
        "{}",
        "  ║  no Kabbalistic or Hermetic framework; cards are read     ║".dimmed()
    );
    println!(
        "{}",
        "  ║  combinatorially in pairs and sequences rather than       ║".dimmed()
    );
    println!(
        "{}",
        "  ║  individually.  Despite its name, it was NOT designed by  ║".dimmed()
    );
    println!(
        "{}",
        "  ║  Marie Anne Lenormand — it was attributed to her post-    ║".dimmed()
    );
    println!(
        "{}",
        "  ║  humously (Hechtel, Spiel der Hoffnung, 1799).            ║".dimmed()
    );
    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_green()
    );

    loop {
        match LENORMAND_MENU.show_and_read().as_str() {
            "1" => browse_lenormand(),
            "2" => lookup_lenormand_by_number(),
            "3" => lookup_lenormand_by_name(),
            "4" => draw_lenormand(),
            "0" | "" => break,
            _ => println!("{}", "  Please enter 0–4.".yellow()),
        }
    }
}

fn browse_lenormand() {
    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_green()
    );
    println!(
        "{}",
        "  ║        🌿  THE 36 LENORMAND CARDS  (Petit Jeu)  🌿       ║"
            .bold()
            .bright_green()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_green()
    );
    println!(
        "{}",
        "  ║  #   Card Name         Playing Card   Domain             ║".dimmed()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_green()
    );

    for card in LENORMAND {
        let name_f: String = card.name.chars().take(17).collect();
        let pc_f: String = card.playing_card.chars().take(13).collect();
        let dom_f: String = card.domain.chars().take(18).collect();
        println!(
            "  {}",
            format!(
                "║  {:>2}. {:<17}  {:<13}  {:<18} ║",
                card.number, name_f, pc_f, dom_f
            )
            .bright_white()
        );
    }

    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_green()
    );
    println!();
    println!(
        "  {}",
        "Enter a card number (1–36) for full profile, or press Enter to return:".cyan()
    );
    print!("  {} ", "▸".bold().cyan());
    io::stdout().flush().unwrap_or(());
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_ok() {
        if let Ok(n) = buf.trim().parse::<u8>() {
            if n >= 1 && n <= 36 {
                if let Some(card) = lenormand_by_number(n) {
                    print_lenormand_card(card);
                }
            }
        }
    }
}

fn lookup_lenormand_by_number() {
    loop {
        println!();
        print!(
            "{}",
            "  ▸ Enter card number (1–36), or press Enter to go back: ".cyan()
        );
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() { break; }
        let t = buf.trim();
        if t.is_empty() { break; }
        match t.parse::<u8>() {
            Ok(n) if n >= 1 && n <= 36 => match lenormand_by_number(n) {
                Some(c) => print_lenormand_card(c),
                None => println!("{}", "  Card not found.".yellow()),
            },
            _ => println!("{}", "  Please enter a number between 1 and 36.".yellow()),
        }
    }
}

fn lookup_lenormand_by_name() {
    loop {
        println!();
        print!(
            "{}",
            "  ▸ Enter card name (partial match), or press Enter to go back: ".cyan()
        );
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() { break; }
        let t = buf.trim();
        if t.is_empty() { break; }
        match lenormand_by_name(t) {
            Some(c) => print_lenormand_card(c),
            None => println!("  {}", format!("No Lenormand card matching '{}'.", t).yellow()),
        }
    }
}

fn print_lenormand_card(card: &crate::tarot::lenormand::LenormandCard) {
    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_green()
    );
    println!(
        "  {}",
        format!(
            "║  {:>2}. ✦ {}  ({}){:<width$}║",
            card.number,
            card.name,
            card.playing_card,
            " ",
            width = (56usize).saturating_sub(6 + card.name.len() + card.playing_card.len())
        )
        .bold()
        .bright_yellow()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_green()
    );
    println!(
        "  {}",
        format!("║  Domain:  {:<47} ║", card.domain).bright_white()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_green()
    );
    println!(
        "  {}",
        "║  Light meaning:                                           ║".dimmed()
    );
    wrap_box_line(card.meaning_light, 54);
    println!(
        "  {}",
        "║  Shadow meaning:                                          ║".dimmed()
    );
    wrap_box_line(card.meaning_shadow, 54);
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_green()
    );
    println!(
        "  {}",
        "║  Reading note (combinatorial):                            ║".dimmed()
    );
    wrap_box_line(card.reading_note, 54);
    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_green()
    );
    println!();
}

static LENORMAND_DRAW_ITEMS: &[MenuItem] = &[
    MenuItem { key: "1", icon: "🕯", label: "One Card — Single Theme", hint: "Names the topic at hand" },
    MenuItem { key: "2", icon: "🌙", label: "Three Cards — Subject · Verb · Object", hint: "Combinatorial sentence" },
    MenuItem { key: "3", icon: "⭐", label: "Five Cards — Line of Five", hint: "Past · Context · Core · Context · Future" },
];

static LENORMAND_DRAW_MENU: Menu = Menu {
    title: "🎴  LENORMAND DRAW",
    border_color: MenuColor::Green,
    items: LENORMAND_DRAW_ITEMS,
    back_key: "0",
    back_label: "Back",
};

fn draw_lenormand() {
    loop {
        match LENORMAND_DRAW_MENU.show_and_read().as_str() {
            "1" => do_lenormand_draw(1),
            "2" => do_lenormand_draw(3),
            "3" => do_lenormand_draw(5),
            "0" | "" => break,
            _ => println!("{}", "  Please enter 0–3.".yellow()),
        }
    }
}

fn do_lenormand_draw(count: usize) {
    let total = LENORMAND.len();
    let mut indices: Vec<usize> = (0..total).collect();
    let rng_hw = RdRand::new().ok();
    for i in 0..count.min(total) {
        let j = i + (next_rnd(&rng_hw) as usize % (total - i));
        indices.swap(i, j);
    }

    let pos_labels = ["", "Past", "Context", "Core", "Context", "Future"];

    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_green()
    );
    println!(
        "{}",
        "  ║           🌿  LENORMAND READING  🌿                      ║"
            .bold()
            .bright_green()
    );
    println!(
        "{}",
        "  ║  Remember: read cards as a sentence, not individually.   ║".dimmed()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_green()
    );
    println!();

    for (i, &idx) in indices[..count].iter().enumerate() {
        if count > 1 {
            let label = if count == 5 { pos_labels[i + 1] } else { "" };
            println!(
                "  {}",
                format!("── Card {} {} ───────────────────────────────────", i + 1, label)
                    .bold()
                    .bright_green()
            );
            println!();
        }
        if idx < LENORMAND.len() {
            print_lenormand_card(&LENORMAND[idx]);
        }
    }

    if count > 1 {
        println!(
            "{}",
            "  ╔══════════════════════════════════════════════════════════╗"
                .bright_green()
        );
        println!(
            "{}",
            "  ║  Combine the cards into a sentence or story.             ║"
                .italic()
                .bright_yellow()
        );
        println!(
            "{}",
            "  ║  Each card modifies the others; no card stands alone.    ║"
                .italic()
                .bright_yellow()
        );
        println!(
            "{}",
            "  ╚══════════════════════════════════════════════════════════╝"
                .bright_green()
        );
        println!();
    }
}

// ─── Oracle session ───────────────────────────────────────────────────────────

static ORACLE_ITEMS: &[MenuItem] = &[
    MenuItem {
        key: "1",
        icon: "📋",
        label: "Browse All 44 Oracle Cards",
        hint: "Full table with suits and angels",
    },
    MenuItem {
        key: "2",
        icon: "🔥",
        label: "Seraphic Fire (Cards 1–11)",
        hint: "Divine will · Inspiration · Michael",
    },
    MenuItem {
        key: "3",
        icon: "💧",
        label: "Celestial Water (Cards 12–22)",
        hint: "Divine love · Healing · Gabriel",
    },
    MenuItem {
        key: "4",
        icon: "⚡",
        label: "Sacred Air (Cards 23–33)",
        hint: "Divine wisdom · Truth · Raphael",
    },
    MenuItem {
        key: "5",
        icon: "🌿",
        label: "Holy Earth (Cards 34–44)",
        hint: "Divine order · Manifestation · Uriel",
    },
    MenuItem {
        key: "6",
        icon: "💫",
        label: "Draw an Oracle Card",
        hint: "Single card of divine guidance · TRNG",
    },
];

static ORACLE_MENU: Menu = Menu {
    title: "💫  ANGELIC ORACLE  ·  44 Cards",
    border_color: MenuColor::Magenta,
    items: ORACLE_ITEMS,
    back_key: "0",
    back_label: "Back to Card Menu",
};

fn oracle_session() {
    loop {
        match ORACLE_MENU.show_and_read().as_str() {
            "1" => browse_oracle_all(),
            "2" => browse_oracle_suit("Seraphic Fire"),
            "3" => browse_oracle_suit("Celestial Water"),
            "4" => browse_oracle_suit("Sacred Air"),
            "5" => browse_oracle_suit("Holy Earth"),
            "6" => draw_oracle(),
            "0" | "" => break,
            _ => println!("{}", "  Please enter 0–6.".yellow()),
        }
    }
}

fn browse_oracle_all() {
    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_magenta()
    );
    println!(
        "{}",
        "  ║        💫  THE 44 ANGELIC ORACLE CARDS  💫               ║"
            .bold()
            .bright_magenta()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_magenta()
    );
    println!(
        "{}",
        "  ║  #   Title                    Angel         Suit         ║".dimmed()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_magenta()
    );

    for card in ORACLE {
        let title_f: String = card.title.chars().take(22).collect();
        let angel_f: String = card.angel.chars().take(13).collect();
        let suit_f: String = card.suit.chars().take(14).collect();
        println!(
            "  {}",
            format!(
                "║  {:>2}. {:<22}  {:<13}  {:<14}║",
                card.number, title_f, angel_f, suit_f
            )
            .bright_white()
        );
    }

    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_magenta()
    );
    println!();
    println!(
        "  {}",
        "Enter a card number (1–44) for full profile, or press Enter to return:".cyan()
    );
    print!("  {} ", "▸".bold().cyan());
    io::stdout().flush().unwrap_or(());
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_ok() {
        if let Ok(n) = buf.trim().parse::<u8>() {
            if n >= 1 && n <= 44 {
                if let Some(card) = oracle_by_number(n) {
                    print_oracle_card(card);
                }
            }
        }
    }
}

fn browse_oracle_suit(suit: &str) {
    let cards = oracle_suit_cards(suit);
    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_magenta()
    );
    println!(
        "  {}",
        format!("║  💫  {}  {:<37}║", suit.to_uppercase(), "").bold().bright_magenta()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_magenta()
    );

    for card in &cards {
        let title_f: String = card.title.chars().take(28).collect();
        let angel_f: String = card.angel.chars().take(18).collect();
        println!(
            "  {}",
            format!("║  {:>2}. {:<28}  {:<18}    ║", card.number, title_f, angel_f)
                .bright_white()
        );
    }

    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_magenta()
    );
    println!();
    println!(
        "  {}",
        "Enter a card number for the full profile, or press Enter to return:".cyan()
    );

    loop {
        print!("  {} ", "▸".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() { break; }
        let t = buf.trim();
        if t.is_empty() { break; }
        match t.parse::<u8>() {
            Ok(n) if n >= 1 && n <= 44 => match oracle_by_number(n) {
                Some(c) => print_oracle_card(c),
                None => println!("{}", "  Card not found.".yellow()),
            },
            _ => println!("{}", "  Please enter a number between 1 and 44.".yellow()),
        }
    }
}

fn print_oracle_card(card: &OracleCard) {
    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_magenta()
    );
    println!(
        "  {}",
        format!(
            "║  {:>2}. ✦ {}  {:<width$}║",
            card.number, card.title, " ",
            width = (56usize).saturating_sub(6 + card.title.len())
        )
        .bold()
        .bright_yellow()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_magenta()
    );
    println!(
        "  {}",
        format!("║  Suit:   {:<48} ║", card.suit).bright_white()
    );
    println!(
        "  {}",
        format!("║  Angel:  {:<48} ║", card.angel).bright_white()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_magenta()
    );
    println!(
        "  {}",
        "║  Divine Quality:                                          ║".dimmed()
    );
    wrap_box_line(card.divine_quality, 54);
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_magenta()
    );
    println!(
        "  {}",
        "║  Guidance:                                                ║".dimmed()
    );
    wrap_box_line(card.guidance, 54);
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_magenta()
    );
    println!(
        "  {}",
        "║  Affirmation:                                             ║".dimmed()
    );
    wrap_box_line(card.affirmation, 54);
    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_magenta()
    );
    println!();
}

fn draw_oracle() {
    let total = ORACLE.len();
    let rng_hw = RdRand::new().ok();
    let idx = next_rnd(&rng_hw) as usize % total;
    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_magenta()
    );
    println!(
        "{}",
        "  ║        💫  YOUR ORACLE CARD  💫                          ║"
            .bold()
            .bright_magenta()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_magenta()
    );
    println!();

    display_angelic_hymn(&rng_hw);

    print!(
        "{}",
        "  ✦  Press Enter to reveal your oracle card...  ✦  "
            .italic()
            .bright_yellow()
    );
    io::stdout().flush().unwrap_or(());
    let mut _pause = String::new();
    io::stdin().read_line(&mut _pause).unwrap_or(0);
    println!();

    print_oracle_card(&ORACLE[idx]);
}

// ─── OH Cards session ─────────────────────────────────────────────────────────

static OH_ITEMS: &[MenuItem] = &[
    MenuItem {
        key: "1",
        icon: "🖼",
        label: "Draw an Image + Word Pair",
        hint: "One image card + one word card · Pure projection",
    },
    MenuItem {
        key: "2",
        icon: "📋",
        label: "Browse Image Cards (1–44)",
        hint: "Scene descriptions as projective prompts",
    },
    MenuItem {
        key: "3",
        icon: "🔤",
        label: "Browse Word Cards (1–44)",
        hint: "Archetypal words and their psychological dimensions",
    },
];

static OH_MENU: Menu = Menu {
    title: "🖼  OH CARDS  ·  Image + Word Projection",
    border_color: MenuColor::White,
    items: OH_ITEMS,
    back_key: "0",
    back_label: "Back to Card Menu",
};

fn oh_cards_session() {
    // Disclaimer before menu
    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_white()
    );
    println!(
        "{}",
        "  ║  ⚠  OH Cards are NOT a Tarot or divination system       ║"
            .bold()
            .bright_white()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_white()
    );
    println!(
        "{}",
        "  ║  OH Cards (©1981 Ely Raman & Joseph Schlichter; Eos      ║".dimmed()
    );
    println!(
        "{}",
        "  ║  Games) are a projective psychological image + word       ║".dimmed()
    );
    println!(
        "{}",
        "  ║  tool.  They carry no fixed meanings — all interpretation ║".dimmed()
    );
    println!(
        "{}",
        "  ║  comes from the user's own free association.  The 44      ║".dimmed()
    );
    println!(
        "{}",
        "  ║  scene descriptions here are open-ended prompts rather    ║".dimmed()
    );
    println!(
        "{}",
        "  ║  than copies of the proprietary artwork.                  ║".dimmed()
    );
    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_white()
    );

    loop {
        match OH_MENU.show_and_read().as_str() {
            "1" => draw_oh_pair(),
            "2" => browse_oh_images(),
            "3" => browse_oh_words(),
            "0" | "" => break,
            _ => println!("{}", "  Please enter 0–3.".yellow()),
        }
    }
}

fn draw_oh_pair() {
    let rng_hw = RdRand::new().ok();
    let img_idx = next_rnd(&rng_hw) as usize % OH_IMAGES.len();
    let word_idx = next_rnd(&rng_hw) as usize % OH_WORDS.len();
    let img = &OH_IMAGES[img_idx];
    let word = &OH_WORDS[word_idx];

    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_white()
    );
    println!(
        "{}",
        "  ║  🖼  YOUR OH CARDS DRAW  ·  Image + Word                 ║"
            .bold()
            .bright_white()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_white()
    );
    println!();

    display_contemplative_opening(&rng_hw);

    print!(
        "{}",
        "  ✦  Press Enter to reveal your cards...  ✦  "
            .italic()
            .white()
    );
    io::stdout().flush().unwrap_or(());
    let mut _pause = String::new();
    io::stdin().read_line(&mut _pause).unwrap_or(0);
    println!();

    // Image card
    println!(
        "  {}",
        format!("║  IMAGE CARD #{:<45} ║", img.number).bold().bright_cyan()
    );
    println!(
        "  {}",
        "║  Scene:                                                   ║".dimmed()
    );
    wrap_box_line(img.scene, 54);
    println!(
        "  {}",
        "║  Themes often evoked:                                     ║".dimmed()
    );
    wrap_box_line(img.themes, 54);
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_white()
    );

    // Word card (face-down then revealed)
    println!(
        "  {}",
        format!("║  WORD CARD #{:<46} ║", word.number).bold().bright_yellow()
    );
    println!(
        "  {}",
        format!("║  Word:  {:<49} ║", word.word)
            .bold()
            .bright_yellow()
    );
    println!(
        "  {}",
        "║  Psychological dimension:                                 ║".dimmed()
    );
    wrap_box_line(word.dimension, 54);
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_white()
    );

    // Prompts
    println!(
        "  {}",
        "║  Reflection prompts:                                      ║".dimmed()
    );
    println!(
        "  {}",
        "║  · What is happening in the scene?                        ║".dimmed()
    );
    println!(
        "  {}",
        "║  · How does the word connect to the image for you?        ║".dimmed()
    );
    println!(
        "  {}",
        "║  · Where do you see this in your own life right now?      ║".dimmed()
    );
    println!(
        "  {}",
        "║  · What feeling arises?                                   ║".dimmed()
    );
    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_white()
    );
    println!();
}

fn browse_oh_images() {
    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_white()
    );
    println!(
        "{}",
        "  ║  🖼  OH IMAGE CARDS (1–44) — Scene Prompts               ║"
            .bold()
            .bright_white()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_white()
    );

    for img in OH_IMAGES {
        let scene_f: String = img.scene.chars().take(50).collect();
        println!(
            "  {}",
            format!("║  {:>2}. {:<52} ║", img.number, scene_f).bright_white()
        );
    }

    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_white()
    );
    println!();
    println!(
        "  {}",
        "Enter a card number (1–44) to see full scene + themes, or press Enter:".cyan()
    );

    loop {
        print!("  {} ", "▸".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() { break; }
        let t = buf.trim();
        if t.is_empty() { break; }
        if let Ok(n) = t.parse::<u8>() {
            if let Some(img) = oh_image_by_number(n) {
                println!();
                println!("  {}", format!("Image #{}: {}", img.number, img.scene).bold().bright_cyan());
                println!("  {}", format!("Themes: {}", img.themes).dimmed());
                println!();
            } else {
                println!("{}", "  Please enter 1–44.".yellow());
            }
        } else {
            println!("{}", "  Please enter a number.".yellow());
        }
    }
}

fn browse_oh_words() {
    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_white()
    );
    println!(
        "{}",
        "  ║  🔤  OH WORD CARDS (1–44) — Archetypal Words             ║"
            .bold()
            .bright_white()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_white()
    );
    println!(
        "{}",
        "  ║  #   Word              Psychological Dimension           ║".dimmed()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_white()
    );

    for word in OH_WORDS {
        let word_f: String = word.word.chars().take(15).collect();
        let dim_f: String = word.dimension.chars().take(34).collect();
        println!(
            "  {}",
            format!("║  {:>2}. {:<15}  {:<34}  ║", word.number, word_f, dim_f).bright_white()
        );
    }

    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_white()
    );
    println!();
    println!(
        "  {}",
        "Enter a card number (1–44) to see the full dimension, or press Enter:".cyan()
    );

    loop {
        print!("  {} ", "▸".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() { break; }
        let t = buf.trim();
        if t.is_empty() { break; }
        if let Ok(n) = t.parse::<u8>() {
            if let Some(w) = oh_word_by_number(n) {
                println!();
                println!("  {}", format!("Word #{}: \"{}\"", w.number, w.word).bold().bright_yellow());
                println!("  {}", w.dimension.italic().dimmed());
                println!();
            } else {
                println!("{}", "  Please enter 1–44.".yellow());
            }
        } else {
            println!("{}", "  Please enter a number.".yellow());
        }
    }
}

// ─── Angelic Hymns ────────────────────────────────────────────────────────────

struct AngelicHymn {
    title: &'static str,
    source: &'static str,
    verses: &'static [&'static str],
}

static ANGELIC_HYMNS: &[AngelicHymn] = &[
    AngelicHymn {
        title: "Sanctus",
        source: "Roman Mass · Isaiah 6:2–3",
        verses: &[
            "Sanctus, Sanctus, Sanctus",
            "Dominus Deus Sabaoth.",
            "Pleni sunt caeli et terra gloria tua.",
            "Hosanna in excelsis.",
            "",
            "Holy, Holy, Holy,",
            "Lord God of Hosts.",
            "Heaven and earth are full of Thy glory.",
            "Hosanna in the highest.",
        ],
    },
    AngelicHymn {
        title: "Gloria in Excelsis Deo",
        source: "Luke 2:14 · Greater Doxology",
        verses: &[
            "Gloria in excelsis Deo",
            "et in terra pax hominibus bonae voluntatis.",
            "",
            "Glory to God in the highest,",
            "and on earth peace to people of good will.",
            "We praise Thee, we bless Thee, we adore Thee,",
            "we glorify Thee, we give Thee thanks for Thy great glory.",
        ],
    },
    AngelicHymn {
        title: "Trisagion",
        source: "Eastern Liturgy · 5th century",
        verses: &[
            "Agios o Theos,",
            "Agios Ischyros,",
            "Agios Athanatos,",
            "eleison imas.",
            "",
            "Holy God,",
            "Holy Mighty,",
            "Holy Immortal,",
            "have mercy on us.",
        ],
    },
    AngelicHymn {
        title: "Te Deum Laudamus",
        source: "Nicetas of Remesiana · c. 400 CE",
        verses: &[
            "Te Deum laudamus: te Dominum confitemur.",
            "Te aeternum Patrem omnis terra veneratur.",
            "",
            "Tibi omnes Angeli, tibi Caeli et universae Potestates,",
            "Tibi Cherubim et Seraphim incessabili voce proclamant:",
            "Sanctus, Sanctus, Sanctus, Dominus Deus Sabaoth.",
            "",
            "We praise Thee, O God; we acknowledge Thee to be the Lord.",
            "All the Angels cry aloud, the Heavens and all the Powers therein:",
            "Holy, Holy, Holy, Lord God of Sabaoth.",
        ],
    },
    AngelicHymn {
        title: "Agnus Dei",
        source: "Roman Mass · 7th century",
        verses: &[
            "Agnus Dei, qui tollis peccata mundi,",
            "miserere nobis.",
            "Agnus Dei, qui tollis peccata mundi,",
            "miserere nobis.",
            "Agnus Dei, qui tollis peccata mundi,",
            "dona nobis pacem.",
            "",
            "Lamb of God, who takest away the sins of the world,",
            "have mercy on us.",
            "Lamb of God, who takest away the sins of the world,",
            "grant us peace.",
        ],
    },
    AngelicHymn {
        title: "Veni Creator Spiritus",
        source: "Hrabanus Maurus · c. 9th century",
        verses: &[
            "Veni, Creator Spiritus,",
            "mentes tuorum visita,",
            "imple superna gratia",
            "quae tu creasti pectora.",
            "",
            "Come, Holy Spirit, Creator blest,",
            "and in our souls take up Thy rest;",
            "come with Thy grace and heavenly aid",
            "to fill the hearts which Thou hast made.",
        ],
    },
    AngelicHymn {
        title: "Alma Redemptoris Mater",
        source: "Hermann Contractus · c. 1050 CE",
        verses: &[
            "Alma Redemptoris Mater, quae pervia caeli",
            "porta manes, et stella maris, succurre cadenti,",
            "surgere qui curat, populo:",
            "",
            "Loving Mother of the Redeemer,",
            "gate of heaven, star of the sea,",
            "assist your people who have fallen yet strive to rise again.",
            "To the wonderment of nature you bore your Creator,",
            "yet remained a virgin after as before.",
        ],
    },
    AngelicHymn {
        title: "O Lux Beata Trinitas",
        source: "St. Ambrose of Milan · 4th century",
        verses: &[
            "O lux beata Trinitas",
            "et principalis Unitas,",
            "iam sol recedit igneus:",
            "infunde lumen cordibus.",
            "",
            "O Trinity of blessed light,",
            "O Unity of primal right,",
            "the fiery sun now goes his way;",
            "shed thou within our hearts thy ray.",
        ],
    },
];

/// Select a hymn using the shared RNG, display it, and start chant synthesis.
/// Returns the hymn title for inclusion in the export.
fn display_angelic_hymn(rng: &Option<RdRand>) -> &'static str {
    let raw = match rng {
        Some(r) => r.try_next_u64().unwrap_or_else(|_| os_u64()),
        None => os_u64(),
    };
    let hymn_idx = raw as usize % ANGELIC_HYMNS.len();
    let hymn = &ANGELIC_HYMNS[hymn_idx];

    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_yellow()
    );
    println!(
        "  ║  {}  ║",
        format!("✦  {}  ✦", hymn.title)
            .bright_yellow()
            .bold()
    );
    println!(
        "  ║  {}  ║",
        format!("   {}   ", hymn.source)
            .italic()
            .dimmed()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_yellow()
    );
    println!();
    for verse in hymn.verses {
        if verse.is_empty() {
            println!();
        } else {
            println!("  {}  {}", "║".bright_yellow(), verse.italic().bright_white());
        }
    }
    println!();
    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_yellow()
    );
    println!();

    // Start Gregorian chant synthesis in background (non-blocking, cross-platform).
    crate::hymn_synth::play_gregorian_chant(hymn_idx);

    hymn.title
}

// ─── OH Cards — Contemplative Openings ───────────────────────────────────────

struct ContemplativeOpening {
    attribution: &'static str,
    source: &'static str,
    lines: &'static [&'static str],
}

static CONTEMPLATIVE_OPENINGS: &[ContemplativeOpening] = &[
    ContemplativeOpening {
        attribution: "C. G. Jung",
        source: "Psychological Types, 1921",
        lines: &[
            "\"Your vision will become clear only when you look",
            "into your own heart.",
            "Who looks outside, dreams;",
            "who looks inside, awakes.\"",
        ],
    },
    ContemplativeOpening {
        attribution: "Rainer Maria Rilke",
        source: "Letters to a Young Poet, 1903",
        lines: &[
            "\"I would like to beg you, dear friend,",
            "to have patience with everything unresolved in your heart",
            "and try to love the questions themselves",
            "as if they were locked rooms or books written",
            "in a very foreign language.\"",
        ],
    },
    ContemplativeOpening {
        attribution: "C. G. Jung",
        source: "Memories, Dreams, Reflections, 1962",
        lines: &[
            "\"The privilege of a lifetime is to become",
            "who you truly are.",
            "That which we do not bring to consciousness",
            "appears in our lives as fate.\"",
        ],
    },
    ContemplativeOpening {
        attribution: "Simone Weil",
        source: "Waiting for God, 1951",
        lines: &[
            "\"Attention is the rarest and purest form of generosity.",
            "To pay attention, this is our endless",
            "and proper work.\"",
        ],
    },
    ContemplativeOpening {
        attribution: "Martin Buber",
        source: "I and Thou, 1923",
        lines: &[
            "\"All actual life is encounter.",
            "The world is not an obstacle on your path to God —",
            "it is the path itself.\"",
        ],
    },
    ContemplativeOpening {
        attribution: "Thich Nhất Hạnh",
        source: "The Miracle of Mindfulness, 1975",
        lines: &[
            "\"The present moment is the only moment",
            "available to us, and it is the door",
            "to all moments.",
            "Breathe. You have arrived.\"",
        ],
    },
    ContemplativeOpening {
        attribution: "D. W. Winnicott",
        source: "Playing and Reality, 1971",
        lines: &[
            "\"It is in playing and only in playing",
            "that the individual child or adult is able to be creative",
            "and to use the whole personality,",
            "and it is only in being creative",
            "that the individual discovers the self.\"",
        ],
    },
    ContemplativeOpening {
        attribution: "Meister Eckhart",
        source: "Sermons, c. 1300",
        lines: &[
            "\"The eye through which I see God",
            "is the same eye through which God sees me;",
            "my eye and God's eye are one eye,",
            "one seeing, one knowing, one love.\"",
        ],
    },
];

/// Display a contemplative opening suited to the secular, projective OH Cards.
fn display_contemplative_opening(rng: &Option<RdRand>) -> &'static str {
    let raw = match rng {
        Some(r) => r.try_next_u64().unwrap_or_else(|_| os_u64()),
        None => os_u64(),
    };
    let opening_idx = raw as usize % CONTEMPLATIVE_OPENINGS.len();
    let opening = &CONTEMPLATIVE_OPENINGS[opening_idx];

    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_white()
    );
    println!(
        "  ║  {}  ║",
        format!("✦  {}  ✦", opening.attribution)
            .bright_white()
            .bold()
    );
    println!(
        "  ║  {}  ║",
        format!("   {}   ", opening.source).italic().dimmed()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_white()
    );
    println!();
    for line in opening.lines {
        if line.is_empty() {
            println!();
        } else {
            println!("  {}  {}", "║".bright_white(), line.italic().white());
        }
    }
    println!();
    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_white()
    );
    println!();

    // Start meditative modal tone in background (non-blocking, cross-platform).
    crate::hymn_synth::play_contemplative_tone(opening_idx);

    opening.attribution
}

// ─── Shared RNG helper ────────────────────────────────────────────────────────

fn next_rnd(rng: &Option<RdRand>) -> u64 {
    match rng {
        Some(r) => r.try_next_u64().unwrap_or_else(|_| os_u64()),
        None => os_u64(),
    }
}

// ─── OS-entropy fallback ──────────────────────────────────────────────────────

fn os_u64() -> u64 {
    let mut bytes = [0u8; 8];
    getrandom::getrandom(&mut bytes).unwrap_or(());
    u64::from_le_bytes(bytes)
}

// ─── HTML escape helper ───────────────────────────────────────────────────────

// ─── Crowley Thoth Tarot session ─────────────────────────────────────────────

use super::thoth_major::{thoth_major_by_name, thoth_major_by_number, ThothMajor, THOTH_MAJOR};
use super::thoth_minor::{thoth_minor_by_suit_rank, thoth_suit_cards, ThothMinor, THOTH_MINOR};

static THOTH_ITEMS: &[MenuItem] = &[
    MenuItem {
        key: "1",
        icon: "✦",
        label: "Major Arcana — The 22 Trumps",
        hint: "Adjustment · Lust · Art · The Aeon · The Universe",
    },
    MenuItem {
        key: "2",
        icon: "🔥",
        label: "Minor Arcana — Wands (Fire)",
        hint: "Dominion · Virtue · Completion · Strife · Victory …",
    },
    MenuItem {
        key: "3",
        icon: "💧",
        label: "Minor Arcana — Cups (Water)",
        hint: "Love · Abundance · Luxury · Debauch · Happiness …",
    },
    MenuItem {
        key: "4",
        icon: "⚡",
        label: "Minor Arcana — Swords (Air)",
        hint: "Peace · Sorrow · Defeat · Science · Ruin …",
    },
    MenuItem {
        key: "5",
        icon: "🌍",
        label: "Minor Arcana — Disks (Earth)",
        hint: "Change · Works · Power · Failure · Wealth …",
    },
    MenuItem {
        key: "6",
        icon: "🔮",
        label: "Draw a Thoth Reading",
        hint: "1-card or 3-card draw from the full 78-card deck · TRNG",
    },
];

static THOTH_MENU: Menu = Menu {
    title: "🔯  CROWLEY THOTH TAROT  ·  Book of Thoth",
    border_color: MenuColor::Yellow,
    items: THOTH_ITEMS,
    back_key: "0",
    back_label: "Back to Card Menu",
};

fn thoth_session() {
    // Brief orientation note
    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_yellow()
    );
    println!(
        "{}",
        "  ║  🔯  CROWLEY THOTH TAROT  ·  The Book of Thoth         ║"
            .bold()
            .bright_yellow()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_yellow()
    );
    println!(
        "{}",
        "  ║  Designed by Aleister Crowley · Painted by Lady Frieda  ║".dimmed()
    );
    println!(
        "{}",
        "  ║  Harris · 1938–1943 · Published posthumously 1969.       ║".dimmed()
    );
    println!(
        "{}",
        "  ║  Based on Thelema, Hermetic Qabalah & projective         ║".dimmed()
    );
    println!(
        "{}",
        "  ║  geometry.  Source: The Book of Thoth (O.T.O., 1944).   ║".dimmed()
    );
    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_yellow()
    );

    loop {
        match THOTH_MENU.show_and_read().as_str() {
            "1" => thoth_major_session(),
            "2" => thoth_minor_session("Wands"),
            "3" => thoth_minor_session("Cups"),
            "4" => thoth_minor_session("Swords"),
            "5" => thoth_minor_session("Disks"),
            "6" => thoth_draw(),
            "0" | "" => break,
            _ => println!("{}", "  Please enter 0–6.".yellow()),
        }
    }
}

// ── Thoth Major Arcana ──────────────────────────────────────────────────────

static THOTH_MAJOR_ITEMS: &[MenuItem] = &[
    MenuItem { key: "1", icon: "📜", label: "Browse all 22 Trumps", hint: "Full table" },
    MenuItem { key: "2", icon: "🔢", label: "Look up by number (0–21)", hint: "" },
    MenuItem { key: "3", icon: "🔍", label: "Look up by name", hint: "Partial match works" },
];
static THOTH_MAJOR_MENU: Menu = Menu {
    title: "🔯  THOTH MAJOR ARCANA  ·  22 Trumps",
    border_color: MenuColor::Yellow,
    items: THOTH_MAJOR_ITEMS,
    back_key: "0",
    back_label: "Back",
};

fn thoth_major_session() {
    loop {
        match THOTH_MAJOR_MENU.show_and_read().as_str() {
            "1" => browse_thoth_major(),
            "2" => lookup_thoth_major_by_number(),
            "3" => lookup_thoth_major_by_name(),
            "0" | "" => break,
            _ => {}
        }
    }
}

fn browse_thoth_major() {
    println!();
    println!("{}", "  ══  THOTH MAJOR ARCANA  ══".bright_yellow().bold());
    println!(
        "  {:<4} {:<24} {:<20} {:<14} {}",
        "#".bold(), "Name".bold(), "Hebrew".bold(), "Astrology".bold(), "Upright".bold()
    );
    println!("  {}", "─".repeat(90).dimmed());
    for c in &THOTH_MAJOR {
        let rws = if c.name != c.rws_name {
            format!(" ({})", c.rws_name)
        } else {
            String::new()
        };
        println!(
            "  {:<4} {:<24} {:<20} {:<14} {}",
            c.number.to_string().bright_cyan(),
            format!("{}{}", c.name, rws).bright_yellow(),
            c.hebrew_letter.dimmed(),
            c.astrology.cyan(),
            tarot_esc(c.meaning_upright).chars().take(38).collect::<String>().white(),
        );
    }
    println!();

    let stem = "thoth_major_arcana";
    handle_export(
        stem,
        || {
            let mut s = format!("THOTH TAROT — MAJOR ARCANA\nGenerated: {}\n\n", chrono_now());
            for c in &THOTH_MAJOR {
                s.push_str(&format!("[{}] {}", c.number, c.name));
                if c.name != c.rws_name { s.push_str(&format!(" (RWS: {})", c.rws_name)); }
                s.push('\n');
                s.push_str(&format!("  Hebrew: {}\n  Path {}: {}\n  Astrology: {}\n", c.hebrew_letter, c.path, c.sephiroth, c.astrology));
                s.push_str(&format!("  Thelema: {}\n  Upright: {}\n\n", c.thelemic_title, c.meaning_upright));
            }
            s
        },
        || {
            let mut rows = String::new();
            for c in &THOTH_MAJOR {
                rows.push_str(&format!(
                    "<tr><td class=\"num\">{}</td>\
                     <td class=\"sys\">{}{}</td>\
                     <td>{}</td><td>{}</td>\
                     <td class=\"meaning\">{}</td></tr>",
                    c.number,
                    tarot_esc(c.name),
                    if c.name != c.rws_name { format!("<br><small style=\"color:var(--calc-color);\">RWS: {}</small>", tarot_esc(c.rws_name)) } else { String::new() },
                    tarot_esc(c.hebrew_letter),
                    tarot_esc(c.astrology),
                    tarot_esc(c.meaning_upright),
                ));
            }
            let body = format!(
                "<h2 style=\"color:var(--accent);\">Thoth Tarot — Major Arcana</h2>\
                 <table><thead><tr><th>#</th><th>Name</th><th>Hebrew</th><th>Astrology</th><th>Upright Meaning</th></tr></thead>\
                 <tbody>{}</tbody></table>",
                rows
            );
            wrap_html("Thoth Tarot — Major Arcana", &body, "angelic")
        },
    );
}

fn lookup_thoth_major_by_number() {
    print!("{}", "  ▸ Enter card number (0–21): ".bold().bright_yellow());
    io::stdout().flush().unwrap_or(());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap_or(0);
    if let Ok(n) = buf.trim().parse::<u8>() {
        if let Some(c) = thoth_major_by_number(n) {
            print_thoth_major_card(c);
        } else {
            println!("  {}", "No card at that number.".yellow());
        }
    }
}

fn lookup_thoth_major_by_name() {
    print!("{}", "  ▸ Enter card name (partial ok): ".bold().bright_yellow());
    io::stdout().flush().unwrap_or(());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap_or(0);
    match thoth_major_by_name(buf.trim()) {
        Some(c) => print_thoth_major_card(c),
        None => println!("  {}", "No matching Thoth card found.".yellow()),
    }
}

pub fn print_thoth_major_card(c: &ThothMajor) {
    println!();
    println!(
        "  {}",
        format!("── Trump {}:  {}  ──────────────────────────────", c.number, c.name)
            .bright_yellow()
            .bold()
    );
    if c.name != c.rws_name {
        println!("  {}", format!("  RWS equivalent: {}", c.rws_name).dimmed());
    }
    println!("  {} {}", "Hebrew:".bright_cyan(), c.hebrew_letter.white());
    println!("  {} Path {} · {}", "Tree of Life:".bright_cyan(), c.path, c.sephiroth.white());
    println!("  {} {}", "Astrology:".bright_cyan(), c.astrology.white());
    println!("  {} {}", "Thelema:".bright_cyan(), c.thelemic_title.white());
    println!();
    println!("  {} {}", "Harris:".bright_yellow(), c.harris_symbolism.dimmed());
    println!();
    println!("  {} {}", "Upright:".bright_green(), c.meaning_upright.white());
    println!("  {} {}", "Reversed:".bright_red(), c.meaning_reversed.white());
    println!();
}

// ── Thoth Minor Arcana ──────────────────────────────────────────────────────

fn thoth_minor_session(suit: &str) {
    let cards = thoth_suit_cards(suit);
    println!();
    println!(
        "{}",
        format!("  ══  THOTH — {} ({} cards)  ══", suit.to_uppercase(), cards.len())
            .bright_yellow()
            .bold()
    );
    println!(
        "  {:<14} {:<22} {:<20} {}",
        "Rank".bold(), "Title".bold(), "Astrology".bold(), "Meaning".bold()
    );
    println!("  {}", "─".repeat(80).dimmed());
    for c in &cards {
        println!(
            "  {:<14} {:<22} {:<20} {}",
            c.rank_name.bright_yellow(),
            c.title.bright_cyan(),
            c.astrology.dimmed(),
            tarot_esc(c.meaning).chars().take(36).collect::<String>().white(),
        );
    }
    println!();

    println!("{}", "  ▸ Enter rank to see full detail (e.g. \"Ace\", \"Three\", \"Queen\"), or Enter to skip: ".dimmed());
    print!("  {} ", "▸".bold().bright_yellow());
    io::stdout().flush().unwrap_or(());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap_or(0);
    let rank = buf.trim();
    if !rank.is_empty() {
        if let Some(c) = thoth_minor_by_suit_rank(suit, rank) {
            print_thoth_minor_card(c);
        } else {
            println!("  {}", format!("No {} card matching '{}'.", suit, rank).yellow());
        }
    }

    let suit_owned = suit.to_string();
    handle_export(
        &format!("thoth_{}", suit.to_lowercase()),
        || {
            let mut s = format!("THOTH TAROT — {}\nGenerated: {}\n\n", suit_owned.to_uppercase(), chrono_now());
            for c in &cards {
                s.push_str(&format!("{} of {} — {}\n", c.rank_name, c.suit, c.title));
                s.push_str(&format!("  Sephirah: {}  ·  Astrology: {}\n", c.sephirah, c.astrology));
                s.push_str(&format!("  Upright: {}\n  Reversed: {}\n\n", c.meaning, c.reversed));
            }
            s
        },
        || {
            let mut rows = String::new();
            for c in &cards {
                rows.push_str(&format!(
                    "<tr><td class=\"sys\">{} of {}</td><td class=\"sys\">{}</td>\
                     <td>{}</td><td>{}</td><td class=\"meaning\">{}</td></tr>",
                    tarot_esc(c.rank_name), tarot_esc(c.suit),
                    tarot_esc(c.title),
                    tarot_esc(c.sephirah), tarot_esc(c.astrology),
                    tarot_esc(c.meaning),
                ));
            }
            let body = format!(
                "<h2 style=\"color:var(--accent);\">Thoth Tarot — {}</h2>\
                 <table><thead><tr><th>Card</th><th>Title</th><th>Sephirah</th><th>Astrology</th><th>Meaning</th></tr></thead>\
                 <tbody>{}</tbody></table>",
                tarot_esc(&suit_owned), rows
            );
            wrap_html(&format!("Thoth Tarot — {}", suit_owned), &body, "angelic")
        },
    );
}

fn print_thoth_minor_card(c: &ThothMinor) {
    println!();
    println!(
        "  {}",
        format!("── {} of {}  ·  \"{}\"  ──", c.rank_name, c.suit, c.title)
            .bright_yellow()
            .bold()
    );
    if !c.sephirah.is_empty() {
        println!("  {} {}", "Sephirah:".bright_cyan(), c.sephirah.white());
    }
    println!("  {} {}", "Astrology:".bright_cyan(), c.astrology.white());
    println!("  {} {}", "Element:".bright_cyan(), c.element.white());
    println!();
    println!("  {} {}", "Upright:".bright_green(), c.meaning.white());
    println!("  {} {}", "Reversed:".bright_red(), c.reversed.white());
    println!();
}

// ── Thoth Draw ──────────────────────────────────────────────────────────────

fn thoth_draw() {
    print!(
        "{}",
        "  ▸ Your name for the record (Enter for anonymous): "
            .bold()
            .bright_yellow()
    );
    io::stdout().flush().unwrap_or(());
    let mut name_buf = String::new();
    io::stdin().read_line(&mut name_buf).unwrap_or(0);
    let querent = {
        let t = name_buf.trim();
        if t.is_empty() { "Anonymous" } else { t }.to_string()
    };

    static TDRAW_ITEMS: &[MenuItem] = &[
        MenuItem { key: "1", icon: "◈", label: "Single card", hint: "" },
        MenuItem { key: "2", icon: "◈◈◈", label: "Three-card reading", hint: "Past · Present · Future" },
    ];
    let draw_menu = Menu {
        title: "Thoth Draw",
        border_color: MenuColor::Yellow,
        items: TDRAW_ITEMS,
        back_key: "0",
        back_label: "Back",
    };
    let count = match draw_menu.show_and_read().as_str() {
        "1" => 1usize,
        "2" => 3,
        _ => return,
    };

    // Shuffle the 78-card Thoth deck: 0–21 = Major, 22–77 = THOTH_MINOR
    let total = 22 + THOTH_MINOR.len();
    let mut indices: Vec<usize> = (0..total).collect();
    let rng_hw = RdRand::new().ok();
    for i in 0..count.min(total) {
        let r = next_rnd(&rng_hw);
        let j = i + (r as usize % (total - i));
        indices.swap(i, j);
    }

    // Display hymn + reveal prompt
    let hymn_title = display_angelic_hymn(&rng_hw);
    print!(
        "{}",
        "  ✦  Press Enter to reveal your Thoth reading...  ✦  "
            .italic()
            .bright_yellow()
    );
    io::stdout().flush().unwrap_or(());
    let mut _p = String::new();
    io::stdin().read_line(&mut _p).unwrap_or(0);
    println!();

    let labels = ["Past", "Present", "Future"];
    println!();
    println!("{}", "  ╔══════════════════════════════════════════════════════════╗".bright_yellow());
    let hdr = if count == 1 {
        "  ║        🔯  YOUR THOTH CARD  🔯                           ║"
    } else {
        "  ║      🔯  YOUR THOTH THREE-CARD READING  🔯               ║"
    };
    println!("{}", hdr.bold().bright_yellow());
    println!("{}", "  ╠══════════════════════════════════════════════════════════╣".bright_yellow());
    println!();

    let drawn: Vec<usize> = indices[..count].to_vec();
    for (i, &idx) in drawn.iter().enumerate() {
        if count > 1 {
            println!(
                "  {}",
                format!("── {} ──────────────────────────────", labels[i])
                    .bold()
                    .bright_cyan()
            );
            println!();
        }
        if idx < 22 {
            if let Some(c) = thoth_major_by_number(idx as u8) {
                print_thoth_major_card(c);
            }
        } else {
            let mi = idx - 22;
            if mi < THOTH_MINOR.len() {
                print_thoth_minor_card(&THOTH_MINOR[mi]);
            }
        }
    }

    println!("{}", "  ╔══════════════════════════════════════════════════════════╗".bright_yellow());
    println!("{}", "  ║  ✦  Do what thou wilt shall be the whole of the Law  ✦  ║".italic().bright_yellow());
    println!("{}", "  ╚══════════════════════════════════════════════════════════╝".bright_yellow());
    println!();

    // Persist
    let spread = if count == 1 { "Single Card" } else { "Three-Card Reading" };
    {
        let cards_text = drawn.iter().map(|&idx| {
            if idx < 22 {
                thoth_major_by_number(idx as u8)
                    .map(|c| format!("Trump {}: {}", c.number, c.name))
                    .unwrap_or_default()
            } else {
                let mi = idx - 22;
                if mi < THOTH_MINOR.len() {
                    let m = &THOTH_MINOR[mi];
                    format!("{} of {} — {}", m.rank_name, m.suit, m.title)
                } else { String::new() }
            }
        }).collect::<Vec<_>>().join("\n");
        if let Ok(conn) = open_db() {
            if let Ok((user, _)) = get_or_create_user(&conn, &querent) {
                record_reading(&conn, &user.id, "Thoth Tarot", spread, &cards_text).ok();
            }
        }
    }

    // Export
    let drawn_snap = drawn.clone();
    let querent_snap = querent.clone();
    let hymn_snap = hymn_title.to_string();
    let spread_snap = spread.to_string();
    handle_export(
        &format!("thoth_reading_{}", spread.to_lowercase().replace(' ', "_")),
        || {
            let mut s = format!("THOTH READING — {}\nHymn: {}\nQuerent: {}\nGenerated: {}\n\n",
                spread_snap, hymn_snap, querent_snap, chrono_now());
            for (i, &idx) in drawn_snap.iter().enumerate() {
                if count > 1 { s.push_str(&format!("── {} ──\n", labels[i])); }
                if idx < 22 {
                    if let Some(c) = thoth_major_by_number(idx as u8) {
                        s.push_str(&format!("Trump {}: {}\n  Hebrew: {}\n  {}\n  Upright: {}\n\n",
                            c.number, c.name, c.hebrew_letter, c.astrology, c.meaning_upright));
                    }
                } else if idx - 22 < THOTH_MINOR.len() {
                    let m = &THOTH_MINOR[idx - 22];
                    s.push_str(&format!("{} of {} — \"{}\"\n  {}\n  Upright: {}\n\n",
                        m.rank_name, m.suit, m.title, m.astrology, m.meaning));
                }
            }
            s
        },
        || {
            let mut rows = String::new();
            for (i, &idx) in drawn_snap.iter().enumerate() {
                let pos = if count > 1 { labels.get(i).copied().unwrap_or("") } else { "" };
                if idx < 22 {
                    if let Some(c) = thoth_major_by_number(idx as u8) {
                        rows.push_str(&format!(
                            "<tr><td class=\"sys\">Trump {}: {}</td><td>{}</td><td>{}</td><td class=\"meaning\">{}</td></tr>",
                            c.number, tarot_esc(c.name), tarot_esc(pos), tarot_esc(c.astrology), tarot_esc(c.meaning_upright)
                        ));
                    }
                } else if idx - 22 < THOTH_MINOR.len() {
                    let m = &THOTH_MINOR[idx - 22];
                    rows.push_str(&format!(
                        "<tr><td class=\"sys\">{} of {} — {}</td><td>{}</td><td>{}</td><td class=\"meaning\">{}</td></tr>",
                        tarot_esc(m.rank_name), tarot_esc(m.suit), tarot_esc(m.title),
                        tarot_esc(pos), tarot_esc(m.astrology), tarot_esc(m.meaning)
                    ));
                }
            }
            let body = format!(
                "<h2 style=\"color:var(--accent);\">Thoth Reading — {}</h2>\
                 <p class=\"meta\">Hymn: {} · Querent: {} · {}</p>\
                 <table><thead><tr><th>Card</th><th>Position</th><th>Astrology</th><th>Meaning</th></tr></thead>\
                 <tbody>{}</tbody></table>",
                tarot_esc(spread), tarot_esc(&hymn_snap), tarot_esc(&querent_snap),
                tarot_esc(&chrono_now()), rows
            );
            wrap_html(&format!("Thoth Reading — {}", spread), &body, "angelic")
        },
    );
}

// ─── Reading history ──────────────────────────────────────────────────────────

fn view_reading_history() {
    use crate::persistence::{get_all_readings, get_user_readings};

    static HIST_ITEMS: &[MenuItem] = &[
        MenuItem {
            key: "1",
            icon: "📖",
            label: "My readings (by name)",
            hint: "Look up all readings recorded for a specific name",
        },
        MenuItem {
            key: "2",
            icon: "🌐",
            label: "All readings (global)",
            hint: "Every reading across all users, newest first",
        },
    ];
    static HIST_MENU: Menu = Menu {
        title: "📖  READING HISTORY",
        border_color: MenuColor::Magenta,
        items: HIST_ITEMS,
        back_key: "0",
        back_label: "Back",
    };

    loop {
        match HIST_MENU.show_and_read().as_str() {
            "1" => {
                print!("{}", "  ▸ Enter your name: ".bold().bright_yellow());
                io::stdout().flush().unwrap_or(());
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap_or(0);
                let name = buf.trim();
                if name.is_empty() {
                    continue;
                }
                match open_db() {
                    Err(e) => println!("  {}", format!("Database error: {e}").red()),
                    Ok(conn) => match get_or_create_user(&conn, name) {
                        Err(e) => println!("  {}", format!("User error: {e}").red()),
                        Ok((user, _)) => match get_user_readings(&conn, &user.id) {
                            Err(e) => println!("  {}", format!("Query error: {e}").red()),
                            Ok(readings) => {
                                let title = format!("Readings for {}", user.name);
                                print_readings_table(&readings, &title);
                            }
                        },
                    },
                }
            }
            "2" => match open_db() {
                Err(e) => println!("  {}", format!("Database error: {e}").red()),
                Ok(conn) => match get_all_readings(&conn) {
                    Err(e) => println!("  {}", format!("Query error: {e}").red()),
                    Ok(readings) => print_readings_table(&readings, "All Readings"),
                },
            },
            "0" | "" => break,
            _ => {}
        }
    }
}

/// Parse a stored card line back to a card object and render it at full detail.
///
/// Stored formats:
///   Major Arcana  — `"Major #0: The Fool"`
///   Minor Arcana  — `"Ace of Wands"`
fn recall_tarot_reading(record: &crate::persistence::ReadingRecord) {
    let card_lines: Vec<&str> = record.cards.lines().collect();
    let count = card_lines.len();

    println!();
    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_magenta()
    );
    let header = if count == 1 {
        "  ║           🔮  RECALLED READING  🔮                       ║"
    } else {
        "  ║         🔮  RECALLED READING (3-CARD)  🔮                ║"
    };
    println!("{}", header.bold().bright_magenta());
    println!(
        "  {}",
        format!("  ║  {}  ·  {}", record.user_name, record.drawn_at)
            .dimmed()
    );
    println!(
        "{}",
        "  ╠══════════════════════════════════════════════════════════╣"
            .bright_magenta()
    );
    println!();

    let labels = ["Past", "Present", "Future"];
    for (i, line) in card_lines.iter().enumerate() {
        if count > 1 {
            let label = labels.get(i).copied().unwrap_or("Card");
            println!(
                "  {}",
                format!("── {} ──────────────────────────────", label)
                    .bold()
                    .bright_cyan()
            );
            println!();
        }

        let line = line.trim();
        if line.starts_with("Major #") {
            // Parse "Major #N: Name"
            let num_str = line
                .trim_start_matches("Major #")
                .splitn(2, ':')
                .next()
                .unwrap_or("")
                .trim();
            if let Ok(num) = num_str.parse::<u8>() {
                if let Some(card) = major_by_number(num) {
                    print_major_card(card);
                }
            }
        } else if let Some(pos) = line.find(" of ") {
            // Parse "Rank of Suit"
            let rank = &line[..pos];
            let suit = &line[pos + 4..];
            if let Some(card) = crate::tarot::minor::minor_by_suit_rank(suit, rank) {
                print_minor_card(card);
            }
        } else {
            println!("  {}", line.dimmed());
        }
    }

    println!(
        "{}",
        "  ╔══════════════════════════════════════════════════════════╗"
            .bright_magenta()
    );
    println!(
        "{}",
        "  ║  ✦  May the angels illumine the path revealed here  ✦   ║"
            .italic()
            .bright_yellow()
    );
    println!(
        "{}",
        "  ╚══════════════════════════════════════════════════════════╝"
            .bright_magenta()
    );
    println!();

    // Export the recalled reading
    let querent = record.user_name.clone();
    let drawn_at = record.drawn_at.clone();
    let spread = record.spread_type.clone();
    let cards_text = record.cards.clone();
    handle_export(
        &format!(
            "recalled_reading_{}",
            record.drawn_at.replace(' ', "_").replace(':', "")
        ),
        || {
            let mut s = format!(
                "RECALLED READING — {}\nQuerent: {} · {}\n\n",
                spread, querent, drawn_at
            );
            for line in cards_text.lines() {
                let line = line.trim();
                if line.starts_with("Major #") {
                    let num_str = line
                        .trim_start_matches("Major #")
                        .splitn(2, ':')
                        .next()
                        .unwrap_or("")
                        .trim();
                    if let Ok(num) = num_str.parse::<u8>() {
                        if let Some(c) = major_by_number(num) {
                            s.push_str(&format!("MAJOR #{}: {}\n", c.number, c.name));
                            s.push_str(&format!("Angel: {} | Element: {}\n", c.angel, c.element));
                            s.push_str(&format!("Keywords: {}\n\n", c.keywords));
                        }
                    }
                } else if let Some(pos) = line.find(" of ") {
                    let rank = &line[..pos];
                    let suit = &line[pos + 4..];
                    if let Some(m) = crate::tarot::minor::minor_by_suit_rank(suit, rank) {
                        s.push_str(&format!("{} of {}\n", m.rank_name, m.suit));
                        s.push_str(&format!("Angel: {} | Element: {}\n", m.angel, m.element));
                        s.push_str(&format!("Keywords: {}\n\n", m.keywords));
                    }
                }
            }
            s
        },
        || {
            let mut rows = String::new();
            for (i, line) in cards_text.lines().enumerate() {
                let line = line.trim();
                let pos_label = if count > 1 {
                    labels.get(i).copied().unwrap_or("")
                } else {
                    ""
                };
                if line.starts_with("Major #") {
                    let num_str = line
                        .trim_start_matches("Major #")
                        .splitn(2, ':')
                        .next()
                        .unwrap_or("")
                        .trim();
                    if let Ok(num) = num_str.parse::<u8>() {
                        if let Some(c) = major_by_number(num) {
                            rows.push_str(&format!(
                                "<tr><td class=\"sys\">Major #{}: {}</td><td>{}</td>\
                                 <td>{}</td><td class=\"meaning\">{}</td></tr>",
                                c.number,
                                tarot_esc(c.name),
                                tarot_esc(pos_label),
                                tarot_esc(c.angel),
                                tarot_esc(c.keywords)
                            ));
                        }
                    }
                } else if let Some(p) = line.find(" of ") {
                    let rank = &line[..p];
                    let suit = &line[p + 4..];
                    if let Some(m) = crate::tarot::minor::minor_by_suit_rank(suit, rank) {
                        rows.push_str(&format!(
                            "<tr><td class=\"sys\">{} of {}</td><td>{}</td>\
                             <td>{}</td><td class=\"meaning\">{}</td></tr>",
                            tarot_esc(m.rank_name),
                            tarot_esc(m.suit),
                            tarot_esc(pos_label),
                            tarot_esc(m.angel),
                            tarot_esc(m.keywords)
                        ));
                    }
                }
            }
            let body = format!(
                "<h2 style=\"color:var(--accent);\">Recalled Reading — {}</h2>\
                 <p class=\"meta\">Querent: {} · {}</p>\
                 <table><thead><tr><th>Card</th><th>Position</th><th>Angel</th><th>Keywords</th></tr></thead>\
                 <tbody>{}</tbody></table>",
                tarot_esc(&spread),
                tarot_esc(&querent),
                tarot_esc(&drawn_at),
                rows
            );
            wrap_html(&format!("Recalled Reading — {}", spread), &body, "angelic")
        },
    );
}

fn print_readings_table(readings: &[crate::persistence::ReadingRecord], title: &str) {
    println!();
    println!("{}", format!("  ══  {}  ══", title).bright_magenta().bold());
    if readings.is_empty() {
        println!("  {}", "  No readings recorded yet.".dimmed());
        println!();
        return;
    }
    println!(
        "  {:<4} {:<20} {:<24} {:<18} {:<16} {}",
        "#".bold(),
        "User".bold(),
        "Date".bold(),
        "Tradition".bold(),
        "Spread".bold(),
        "Cards".bold(),
    );
    println!("  {}", "─".repeat(94).dimmed());
    for (i, r) in readings.iter().enumerate() {
        let cards_preview: String = r.cards.lines().take(2).collect::<Vec<_>>().join(", ");
        let cards_preview = if r.cards.lines().count() > 2 {
            format!("{}, …", cards_preview)
        } else {
            cards_preview
        };
        println!(
            "  {:<4} {:<20} {:<24} {:<18} {:<16} {}",
            (i + 1).to_string().bright_cyan(),
            r.user_name.bright_yellow(),
            r.drawn_at.dimmed(),
            r.tradition.cyan(),
            r.spread_type.white(),
            cards_preview.dimmed(),
        );
    }
    println!();

    // Offer recall of individual reading
    print!(
        "{}",
        "  ▸ Enter number to recall a reading in full detail (Enter to skip): "
            .bold()
            .bright_yellow()
    );
    io::stdout().flush().unwrap_or(());
    let mut sel_buf = String::new();
    io::stdin().read_line(&mut sel_buf).unwrap_or(0);
    if let Ok(n) = sel_buf.trim().parse::<usize>() {
        if n >= 1 && n <= readings.len() {
            recall_tarot_reading(&readings[n - 1]);
            return; // skip summary export after a recall
        }
    }

    // Export
    let readings_owned: Vec<crate::persistence::ReadingRecord> = readings.iter().map(|r| {
        crate::persistence::ReadingRecord {
            id: r.id.clone(),
            user_name: r.user_name.clone(),
            drawn_at: r.drawn_at.clone(),
            tradition: r.tradition.clone(),
            spread_type: r.spread_type.clone(),
            cards: r.cards.clone(),
        }
    }).collect();
    let title_owned = title.to_string();
    handle_export(
        &format!("reading_history_{}", title.to_lowercase().replace(' ', "_")),
        || {
            let mut s = format!("{}\n{}\nGenerated: {}\n\n", title_owned, "=".repeat(title_owned.len()), chrono_now());
            for r in &readings_owned {
                s.push_str(&format!("User:      {}\n", r.user_name));
                s.push_str(&format!("Date:      {}\n", r.drawn_at));
                s.push_str(&format!("Tradition: {}\n", r.tradition));
                s.push_str(&format!("Spread:    {}\n", r.spread_type));
                s.push_str(&format!("Cards:\n"));
                for line in r.cards.lines() {
                    s.push_str(&format!("  {}\n", line));
                }
                s.push_str(&"─".repeat(60));
                s.push('\n');
            }
            s
        },
        || {
            let mut rows = String::new();
            for r in &readings_owned {
                rows.push_str(&format!(
                    "<tr><td class=\"sys\">{}</td><td>{}</td><td>{}</td><td>{}</td><td class=\"meaning\">{}</td></tr>",
                    tarot_esc(&r.user_name),
                    tarot_esc(&r.drawn_at),
                    tarot_esc(&r.tradition),
                    tarot_esc(&r.spread_type),
                    tarot_esc(&r.cards.replace('\n', "<br>")),
                ));
            }
            let body = format!(
                "<h2 style=\"color:var(--accent);\">{}</h2>\
                 <table><thead><tr><th>User</th><th>Date</th><th>Tradition</th><th>Spread</th><th>Cards</th></tr></thead>\
                 <tbody>{}</tbody></table>",
                tarot_esc(&title_owned),
                rows
            );
            wrap_html(&title_owned, &body, "angelic")
        },
    );
}

fn tarot_esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

// ─── Box-drawing word wrap ────────────────────────────────────────────────────

/// Print `text` wrapped into ║  …  ║ lines of `inner_width` characters.
fn wrap_box_line(text: &str, inner_width: usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut line = String::new();
    for word in &words {
        if !line.is_empty() && line.len() + 1 + word.len() > inner_width {
            println!(
                "  {}",
                format!("║  {:<width$}  ║", line, width = inner_width).italic().bright_yellow()
            );
            line.clear();
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        println!(
            "  {}",
            format!("║  {:<width$}  ║", line, width = inner_width).italic().bright_yellow()
        );
    }
}
