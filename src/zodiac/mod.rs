//! Zodiac & Astrology — sacred celestial traditions.
//!
//! Current traditions:
//!  - [`mazzaroth`] — Hebrew Mazzaroth: the twelve signs as interpreted
//!    through Sefer Yetzirah, the Twelve Tribes, and the Hoshen breastplate.
//!
//! More traditions (Tropical Western, Vedic Jyotish, Chinese Shengxiao, …)
//! may be added as sub-modules following the same pattern.

mod mazzaroth;

use std::io::{self, Write};
use colored::*;

use mazzaroth::run_mazzaroth_session;

// ─── Public entry point ───────────────────────────────────────────────────────

/// Display the Zodiac traditions menu and dispatch to the chosen session.
pub fn run_zodiac_session() {
    loop {
        println!();
        println!("{}", "╔════════════════════════════════════════════════════════════╗".bright_yellow());
        println!("{}", "║             ✦  ZODIAC & ASTROLOGY  ✦                      ║".bold().bright_yellow());
        println!("{}", "╠════════════════════════════════════════════════════════════╣".bright_yellow());
        println!("{}", "║  Sacred celestial traditions of the world.                ║".dimmed());
        println!("{}", "╚════════════════════════════════════════════════════════════╝".bright_yellow());
        println!();
        println!("  {}  Hebrew Mazzaroth  —  מַזָּרוֹת", "1.".cyan());
        println!("{}", "          Sefer Yetzirah · Twelve Tribes · Hoshen gemstones".dimmed());
        println!();
        println!("  {}  Return to main menu", "0.".dimmed());
        println!();
        print!("  {} ", "▸ Choice:".bold().cyan());
        io::stdout().flush().unwrap_or(());

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap_or(0);

        match buf.trim() {
            "1"      => run_mazzaroth_session(),
            "0" | "" => break,
            _        => println!("{}", "  Please enter 0 or 1.".yellow()),
        }
    }
}
