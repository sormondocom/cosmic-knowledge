//! Chaldean / Babylonian vibrational numerology system.
//!
//! Source: Ancient Babylonian tradition; 9 is considered sacred and unassigned.

use std::collections::HashMap;
use once_cell::sync::Lazy;

pub(super) static MAP: Lazy<HashMap<char, u32>> = Lazy::new(|| [
    ('A',1),('B',2),('C',3),('D',4),('E',5),('F',8),('G',3),('H',5),('I',1),
    ('J',1),('K',2),('L',3),('M',4),('N',5),('O',7),('P',8),('Q',1),('R',2),
    ('S',3),('T',4),('U',6),('V',6),('W',6),('X',5),('Y',1),('Z',7),
].into_iter().collect());
