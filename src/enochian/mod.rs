//! Enochian angelology module — John Dee / Edward Kelley (1582–1587)
//!
//! Sub-modules:
//!  - `alphabet`  — The 21-letter Enochian alphabet, substitution rules, gematria lookup
//!  - `aethyrs`   — The 30 celestial Aethyrs with lookup and display
//!  - `messages`  — Root-number meanings and angelic call fragments

pub mod alphabet;
pub mod aethyrs;
pub mod messages;
pub mod session;

pub use alphabet::{ENOCHIAN_LETTERS, enochian_substitute, enochian_lookup, show_enochian_table};
pub use aethyrs::{AETHYRS, aethyr_lookup, show_aethyr_table, show_aethyr_info};
pub use messages::{enochian_meaning, enochian_angelic_message};
pub use session::run_enochian_session;
