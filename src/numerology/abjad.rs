//! Arabic/Islamic Abjad numerals — Eastern/Mashriqī phonetic Latin approximation.
//!
//! Source: Ibn Khaldun, "Muqaddimah" (14th c.); Schimmel, "The Mystery of
//!         Numbers" (OUP, 1993).  The Eastern Abjad order is used (standard in
//!         Persian, Ottoman, and South Asian traditions).
//! This is a phonetic-approximation mapping: canonical usage operates on Arabic
//! Unicode directly.  Collisions are expected (C=G=J=3; H=E=5; P=F=80; U=V=W=6)
//! because Arabic has 28 letters with no clean 1:1 Latin correspondence.
//! The number 9 maps to Ṭa (ط) — the emphatic /t/ — approximated here by T.

use std::collections::HashMap;
use once_cell::sync::Lazy;

pub(super) static MAP: Lazy<HashMap<char, u32>> = Lazy::new(|| [
    ('A',  1), ('B',  2), ('C',  3), ('D',  4), ('E',  5), ('F', 80),
    ('G',  3), ('H',  5), ('I', 10), ('J',  3), ('K', 20), ('L', 30),
    ('M', 40), ('N', 50), ('O', 70), ('P', 80), ('Q',100), ('R',200),
    ('S', 60), ('T',400), ('U',  6), ('V',  6), ('W',  6), ('X',600),
    ('Y', 10), ('Z',  7),
].into_iter().collect());

/// Abjad root meanings — Sufi / Islamic mystical tradition.
///
/// Drawn from ʿIlm al-ḥurūf (science of letters) as systematized by
/// Ibn ʿArabī and the broader Sufi tradition; planetary correspondences
/// from Islamic astrology (falak) and ʿIlm al-ʿadad (science of number).
pub fn abjad_meaning(root: u32) -> &'static str {
    match root {
        1 => "ا Alif — Divine Unity (Tawḥīd); the Absolute; the first breath of Allāh; all things return to One",
        2 => "ب Bā — The revealed world; the first letter of Bismillāh; duality as the ground of creation",
        3 => "ج Jīm — The three worlds (physical, astral, divine); triangulation of the cosmic order",
        4 => "د Dāl — The four archangels (Jibrīl, Mīkāʾīl, Isrāfīl, ʿIzrāʾīl); the four pillars of the Throne",
        5 => "ه Hāʾ — The five pillars of Islām; the five senses; the divine presence in manifestation",
        6 => "و Wāw — The six directions of space; the six days of creation; perfect equilibrium",
        7 => "ز Zayn — The seven heavens (sabʿ samāwāt); the seven attributes of Allāh; sacred completion",
        8 => "ح Ḥāʾ — The eight bearers of the Throne (ʿArsh); abundance; the gardens of paradise",
        9 => "ط Ṭāʾ — The nine celestial spheres; completion before the return to divine Unity",
        _ => "🌙 Beyond the nine — the ineffable name; enter the silence of Fanāʾ (annihilation in the Divine)",
    }
}
