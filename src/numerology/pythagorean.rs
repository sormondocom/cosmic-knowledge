//! Pythagorean / Western cyclical numerology system.
//!
//! Source: Traditional Western numerology; cyclical mod-9 mapping of alphabet position.

use once_cell::sync::Lazy;
use std::collections::HashMap;

pub(super) static MAP: Lazy<HashMap<char, u32>> = Lazy::new(|| {
    [
        ('A', 1),
        ('B', 2),
        ('C', 3),
        ('D', 4),
        ('E', 5),
        ('F', 6),
        ('G', 7),
        ('H', 8),
        ('I', 9),
        ('J', 1),
        ('K', 2),
        ('L', 3),
        ('M', 4),
        ('N', 5),
        ('O', 6),
        ('P', 7),
        ('Q', 8),
        ('R', 9),
        ('S', 1),
        ('T', 2),
        ('U', 3),
        ('V', 4),
        ('W', 5),
        ('X', 6),
        ('Y', 7),
        ('Z', 8),
    ]
    .into_iter()
    .collect()
});
