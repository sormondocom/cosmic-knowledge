//! Agrippan English (Barrett / Francis Barrett "The Magus" 1801).
//!
//! Source: Cornelius Agrippa, "De Occulta Philosophia" (1531), Book III;
//!         Francis Barrett, "The Magus" (1801) English extension.
//! The Barrett variant separates I(9) from J(10) and assigns sequential tens
//! thereafter, extending Agrippa's original 24-letter Latin table to 26 English
//! letters.  Agrippa original (I=J=9, U=V=200, no W) is a distinct variant.

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
        ('R', 90),
        ('S', 100),
        ('T', 200),
        ('U', 300),
        ('V', 400),
        ('W', 500),
        ('X', 600),
        ('Y', 700),
        ('Z', 800),
    ]
    .into_iter()
    .collect()
});
