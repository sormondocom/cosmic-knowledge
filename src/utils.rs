//! Shared utility functions used across multiple modules.

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
