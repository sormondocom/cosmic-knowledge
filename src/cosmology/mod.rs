//! World Cosmologies — Chinese and African numerological traditions.
//!
//! Sub-modules:
//!  - `chinese` — Nine Star Ki, Wu Xing, Ba Gua, Chinese lucky/unlucky numbers
//!  - `african` — Yoruba Ifá, Akan day names, Kemetic sacred numbers

pub mod chinese;
pub mod african;

pub use chinese::run_chinese_session;
pub use african::run_african_session;

use std::io::{self, Write};
use colored::*;

/// Top-level dispatcher for the World Cosmologies session.
pub fn run_world_systems_session() {
    loop {
        println!();
        println!("{}", "╔══════════════════════════════════════════════════════════╗".bright_green());
        println!("{}", "║        🌏  WORLD COSMOLOGIES & NUMBER TRADITIONS         ║".bold().bright_green());
        println!("{}", "╠══════════════════════════════════════════════════════════╣".bright_green());
        println!("{}", "║                                                          ║".bright_green());
        println!("{}", "║   1.  🐉  Chinese Cosmology                              ║".bright_yellow());
        println!("{}", "║          Nine Star Ki · Wu Xing · Ba Gua · Lucky numbers ║".dimmed());
        println!("{}", "║                                                          ║".bright_green());
        println!("{}", "║   2.  🌍  African Traditions                             ║".bright_red());
        println!("{}", "║          Yoruba Ifá · Akan Day Names · Kemetic Numbers   ║".dimmed());
        println!("{}", "║                                                          ║".bright_green());
        println!("{}", "║   0.  ←  Back to Main Menu                              ║".dimmed());
        println!("{}", "║                                                          ║".bright_green());
        println!("{}", "╚══════════════════════════════════════════════════════════╝".bright_green());
        println!();

        print!("{}", "▸ Enter choice: ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { break; }
        match input.trim() {
            "1" => run_chinese_session(),
            "2" => run_african_session(),
            "0" | "" => break,
            _ => println!("{}", "  Please enter 0–2.".yellow()),
        }
    }
}
