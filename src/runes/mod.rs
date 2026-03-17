//! Runic Traditions — Germanic and modern esoteric runic alphabets.
//!
//! This module covers five runic traditions, from the oldest attested
//! inscriptions to modern esoteric revivals:
//!
//! | Sub-module          | Tradition                     | Count | Period         |
//! |---------------------|-------------------------------|-------|----------------|
//! | [`elder_futhark`]   | Elder Futhark                 | 24    | c. 150–800 CE  |
//! | [`younger_futhark`] | Younger Futhark (Viking Age)  | 16    | c. 750–1100 CE |
//! | [`anglo_saxon`]     | Anglo-Saxon Futhorc           | 33    | c. 5th–11th c. |
//! | [`armanen`]         | Armanen Runes (modern)        | 18    | 1908 CE        |
//!
//! ## What is a rune?
//!
//! The word *rune* derives from Proto-Germanic \**rūnō* ("secret, mystery,
//! whisper"), cognate with Old Norse *rún*, Old English *rūn*, Old High
//! German *rūna*.  Runic writing systems were used across the Germanic
//! world from approximately the 2nd century CE and represent an adaptation
//! of a Mediterranean alphabet (most likely the Old Italic or Latin
//! alphabet) to the phonology of the Proto-Germanic language.
//!
//! Runes served as an everyday writing system (inscriptions on objects,
//! memorial stones, wood, bone) and in certain contexts carried magical
//! or ritual significance.  The association of runes with divination is
//! partially attested in ancient sources but has been greatly elaborated
//! in the modern esoteric tradition.
//!
//! ## Historical note on divination
//!
//! Tacitus (*Germania*, c. 98 CE) describes Germanic lot-casting using
//! marked staves cut from nut-bearing trees — but does not name the marks
//! as "runes."  Runic inscriptions on amulets (e.g., the Migration-Period
//! *bracteates*) and on objects like the Lindholm amulet clearly carry
//! magical intent.  However, a formalised system of divinatory rune
//! meanings comparable to Tarot or I Ching is largely a product of the
//! modern revival, systematised by Guido von List (1908), Ralph Blum
//! (1982), and Edred Thorsson/Stephen Flowers (1984 onward).  This
//! context is noted in each tradition's documentation and is clearly
//! distinguished from historical attestation.
//!
//! ## The three rune poems
//!
//! Three medieval rune poems preserve the traditional names and kennings:
//!
//! - **Old English Rune Poem** (*Rūnstæfas*, c. 8th–10th c. CE) — covers
//!   the 29 Anglo-Saxon futhorc runes.  The unique manuscript (MS Cotton
//!   Otho B.x) perished in the Cottonian fire of 1731; the text survives
//!   only through George Hickes's *Linguarum Veterum Septentrionalium
//!   Thesaurus* (Oxford, 1703–1705).
//!   *Modern edition:* Halsall, Maureen.  *The Old English Rune Poem:
//!   A Critical Edition* (Toronto: University of Toronto Press, 1981).
//!
//! - **Old Norwegian Rune Poem** (*Runatal*, c. 13th–15th c.) — covers the
//!   16 Younger Futhark runes.  Preserved in MS AM 461 12mo (Copenhagen).
//!   *Modern edition:* Page, R.I.  *An Introduction to English Runes*, 2nd
//!   ed. (Woodbridge: Boydell Press, 1999), pp. 67–70.
//!
//! - **Old Icelandic Rune Poem** (*Rúnakvæði*, c. 15th c.) — covers the 16
//!   Younger Futhark runes.  Preserved in MS AM 687d 4to (Copenhagen).
//!   *Modern edition:* Page (1999), pp. 70–73.
//!
//! Rune poem stanzas in this module are cited as abbreviated translations
//! following Page (1999) and Halsall (1981) unless otherwise noted.

pub mod anglo_saxon;
pub mod armanen;
pub mod elder_futhark;
pub mod younger_futhark;
mod session;

pub use session::run_runes_session;

// ─── Shared data type ─────────────────────────────────────────────────────────

/// A single rune from any tradition.
///
/// Fields that are not applicable to a given tradition are empty strings
/// (`""`).  The `aett` field is `0` when the tradition does not use aettir.
#[derive(Debug)]
pub struct Rune {
    /// Sequential position within the tradition's alphabet (1-based).
    pub number: u8,
    /// Common English/transliterated name.
    pub name: &'static str,
    /// Alternative names or spellings across traditions.
    pub alt_names: &'static str,
    /// Unicode rune character(s).
    pub glyph: &'static str,
    /// Phonetic value (IPA or conventional transcription).
    pub phoneme: &'static str,
    /// Proto-Germanic or etymological reconstruction and meaning.
    pub etymology: &'static str,
    /// Aett number (1–3 for Elder Futhark; 0 if not applicable).
    pub aett: u8,
    /// Aett name: "Freyr's Aett", "Hagal's Aett", "Tyr's Aett", or "".
    pub aett_name: &'static str,
    /// Stanza from the Old English Rune Poem (Halsall 1981; "" if absent).
    pub rune_poem_oe: &'static str,
    /// Stanza from the Old Norwegian Rune Poem (Page 1999; "" if absent).
    pub rune_poem_on: &'static str,
    /// Stanza from the Old Icelandic Rune Poem (Page 1999; "" if absent).
    pub rune_poem_oi: &'static str,
    /// Associated deity or mythological figure.
    pub deity: &'static str,
    /// Elemental correspondence.
    pub element: &'static str,
    /// Associated realm of Yggdrasil (Nine Worlds).
    pub world: &'static str,
    /// Esoteric or magical significance from the tradition.
    pub esoteric: &'static str,
    /// Divinatory meaning — upright.
    pub meaning_upright: &'static str,
    /// Divinatory meaning — reversed / merkstave ("" if tradition omits this).
    pub meaning_reversed: &'static str,
}
