//! Hebrew Gematria — Mispar Hechrachi (absolute value) letter table.
//!
//! Source: Traditional; Kaplan, A. *Sefer Yetzirah* (1990, Weiser).

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
        ('J', 10),
        ('K', 20),
        ('L', 30),
        ('M', 40),
        ('N', 50),
        ('O', 60),
        ('P', 70),
        ('Q', 80),
        ('R', 100),
        ('S', 200),
        ('T', 300),
        ('U', 400),
        ('V', 500),
        ('W', 600),
        ('X', 700),
        ('Y', 800),
        ('Z', 900),
    ]
    .into_iter()
    .collect()
});
