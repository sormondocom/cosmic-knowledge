//! Simple Ordinal (A=1…Z=26) and Reverse Ordinal (Z=1…A=26) systems.
//!
//! Simple Ordinal: widely used in modern English Gematria communities (e.g.
//! Gematria Effect News, Marty Leeds).  Distinct from Pythagorean because there
//! is no mod-9 cycle — the raw position 1–26 is used as the letter value.
//!
//! Reverse Ordinal: the mirror complement of Simple Ordinal.  Commonly paired
//! with it for comparative analysis.  A=26, B=25 … Z=1.

use std::collections::HashMap;
use once_cell::sync::Lazy;

pub(super) static SIMPLE_MAP: Lazy<HashMap<char, u32>> = Lazy::new(|| {
    ('A'..='Z').enumerate().map(|(i, c)| (c, i as u32 + 1)).collect()
});

pub(super) static REVERSE_MAP: Lazy<HashMap<char, u32>> = Lazy::new(|| {
    ('A'..='Z').enumerate().map(|(i, c)| (c, 26 - i as u32)).collect()
});
