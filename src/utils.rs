//! Shared utility functions used across multiple modules.

use std::io::Write;

/// Parse a 1-based menu input string.
///
/// Returns `Some(n)` if `s` trims to a decimal integer in the range `[1, max]`,
/// `None` otherwise.
pub fn parse_menu_choice(s: &str, max: usize) -> Option<usize> {
    s.trim()
        .parse::<usize>()
        .ok()
        .filter(|&n| n >= 1 && n <= max)
}

/// Read a single keypress without requiring Enter.
///
/// Enables raw mode, waits for exactly one key event, restores cooked mode, then
/// returns the character (lower-cased for letters), `'\n'` for Enter/Return, and
/// `'q'` for Escape.  Ctrl+C exits the process cleanly.
///
/// Call this after printing and flushing a prompt so the terminal is ready.
pub fn read_key() -> char {
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
    use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

    // Flush any pending output before stealing the terminal
    std::io::stdout().flush().unwrap_or(());

    let _ = enable_raw_mode();
    let key = loop {
        match event::read() {
            Ok(Event::Key(KeyEvent { code, modifiers, kind, .. })) => {
                // Ignore key-release and key-repeat events
                if kind != KeyEventKind::Press {
                    continue;
                }
                // Ctrl+C → restore terminal and exit cleanly
                if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                    let _ = disable_raw_mode();
                    let _ = crossterm::execute!(
                        std::io::stdout(),
                        crossterm::cursor::Show
                    );
                    std::process::exit(0);
                }
                match code {
                    KeyCode::Enter | KeyCode::Char('\n') => break '\n',
                    KeyCode::Esc                         => break 'q',
                    KeyCode::Char(c)                     => break c.to_ascii_lowercase(),
                    _                                    => continue,
                }
            }
            _ => continue,
        }
    };
    let _ = disable_raw_mode();
    // Move cursor to a fresh line (raw mode suppresses the terminal's own newline)
    println!();
    key
}
