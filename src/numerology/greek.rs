//! Greek Isopsephy — Latin/English phonetic approximation of classical Greek values.
//!
//! Source: Dornseiff, "Das Alphabet in Mystik und Magie" (1925); Piperakis.
//! Convention B maps each Latin letter to its closest Greek phonetic equivalent
//! then uses that letter's classical value.  Ambiguities are noted:
//!   C/G → Gamma(3)  |  O → Omicron(70)  |  X → Xi(60)
//!   W → Omega(800)  |  F → Digamma(6)   |  Q → Qoppa(90)
//! For native Greek Unicode input see the module-level doc comment.

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
        ('G', 3),
        ('H', 8),
        ('I', 9),
        ('J', 10),
        ('K', 20),
        ('L', 30),
        ('M', 40),
        ('N', 50),
        ('O', 70),
        ('P', 80),
        ('Q', 90),
        ('R', 100),
        ('S', 200),
        ('T', 300),
        ('U', 400),
        ('V', 400),
        ('W', 800),
        ('X', 60),
        ('Y', 400),
        ('Z', 7),
    ]
    .into_iter()
    .collect()
});

/// Greek Isopsephy root meanings — Neoplatonic / Pythagorean cosmology.
///
/// Rooted in the Pythagorean-Neoplatonic number mysticism of Iamblichus,
/// Nicomachus of Gerasa, and the Gnostic traditions.
pub fn isopsephy_meaning(root: u32) -> &'static str {
    match root {
        1 => "⊙ Monad — the One, the indivisible source; all things proceed from and return to unity",
        2 => "☯ Dyad — duality and polarity; the first division of the Monad; receptive principle",
        3 => "△ Triad — the World Soul; harmony arising from tension of opposites; divine proportion",
        4 => "□ Tetrad — the four elements; cosmic order and stability; the Pythagorean oath sworn by 4",
        5 => "✦ Pentad — life and marriage (2+3); the fifth element Aether; the human microcosm",
        6 => "✡ Hexad — perfection (1+2+3=6); cosmic love; the soul in equilibrium between matter and spirit",
        7 => "🌙 Heptad — the divine; seven celestial spheres; the virgin number (no factor within decad)",
        8 => "∞ Ogdoad — the higher heavens in Gnostic cosmology; transformation beyond the seven spheres",
        9 => "✿ Ennead — the Muses; universal harmony; the horizon of the single-digit cosmos",
        _ => "🌌 Beyond the Ennead — the ineffable mystery of the Decad and its return to unity",
    }
}
