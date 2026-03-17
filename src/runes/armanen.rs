//! Armanen Runes — the modern esoteric system of Guido von List (1908).
//!
//! ## ⚠ IMPORTANT HISTORICAL NOTICE
//!
//! The Armanen runes are **not a historically attested runic alphabet**.
//! They were developed by the Austrian occultist and pan-Germanic
//! nationalist Guido von List (1848–1919), who claimed to have received
//! the 18 runes in a mystical vision during a period of temporary
//! blindness following cataract surgery (1902).  He published them in
//! *Das Geheimnis der Runen* (The Secret of the Runes; Vienna, 1908;
//! English trans. Stephen Flowers, Destiny Books, 1988).
//!
//! List claimed these 18 runes were the original "Ur-runes" — the
//! primordial alphabet from which all others derived.  This claim has
//! **no support in academic runology**.  The 18-rune count was derived
//! from the 18 *ljóðatal* stanzas of the Eddic poem *Hávamál* (the
//! "Song of the High One"), which describe 18 magical songs — not runes.
//!
//! ## The Shadow of Nazism
//!
//! List's Armanen system became foundational for the "Ario-Germanic"
//! occultism of early 20th-century German-speaking Europe.  Several
//! Armanen runes were adapted for use by the SS (*Schutzstaffel*) and
//! related organisations under the direction of Heinrich Himmler and
//! the SS Ahnenerbe (1935–1945):
//!
//! - **Sig rune** (doubled): The SS insignia (ᛋᛋ), designed by
//!   Walter Heck (1932).
//! - **Lebensrune** (Life rune, inverted Yr): Used in SS birth
//!   announcements and gravestones.
//! - **Todesrune** (Death rune, Yr): Used in SS death notices.
//! - **Odal rune**: Used by the SS racial office and 7th SS Division.
//!
//! These appropriations are historical crimes.  The legitimate runic
//! tradition has no connection with Nazi ideology.  Scholars, pagans,
//! and runologists have consistently and emphatically rejected this
//! appropriation.  See: Goodrick-Clarke, Nicholas. *The Occult Roots
//! of Nazism* (1985; 2nd ed. 2004).
//!
//! ## Why study the Armanen runes?
//!
//! Despite their fictional origins, the Armanen system is significant:
//!
//! 1. It is historically important as the source of the most widespread
//!    modern runic esoteric tradition.
//! 2. It influenced virtually all subsequent 20th-century runic revival
//!    movements (including those that explicitly reject its racism).
//! 3. Understanding it is essential for critically evaluating modern
//!    esoteric runic literature.
//! 4. The system's magical attributions — whatever their origin — have
//!    been extensively developed by subsequent practitioners.
//!
//! ## Sources
//!
//! - List, Guido von. *Das Geheimnis der Runen* (Vienna: Guido-von-List-
//!   Gesellschaft, 1908).  English trans.: *The Secret of the Runes*,
//!   trans. Stephen E. Flowers. Rochester, VT: Destiny Books, 1988.
//! - Flowers, Stephen E. *The Galdrabók: An Icelandic Grimoire*.
//!   York Beach, ME: Weiser, 1989.
//! - Goodrick-Clarke, Nicholas. *The Occult Roots of Nazism: Secret
//!   Aryan Cults and Their Influence on Nazi Ideology*. London:
//!   I.B. Tauris, 2004.
//! - Pringle, Heather. *The Master Plan: Himmler's Scholars and the
//!   Holocaust*. New York: Hyperion, 2006.

use super::Rune;

/// The 18 Armanen runes as enumerated by Guido von List (1908).
///
/// These are presented for historical and educational completeness.
/// The esoteric meanings given are List's own attributions (via Flowers 1988)
/// and have no historical authority in the pre-modern runic tradition.
pub static ARMANEN: &[Rune] = &[
    Rune {
        number: 1,
        name: "Fa",
        alt_names: "Fa (List), cf. Elder Futhark Fehu",
        glyph: "ᚠ",
        phoneme: "f",
        etymology: "List derived this from *Fehu* (cattle, wealth) but \
            assigned it to the first *ljóðatal* stanza of the Hávamál. \
            His interpretation: 'the primal fire,' the creative beginning.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 1 — \
            'I know that I hung on the wind-tossed tree / nine long nights.' \
            List associated this with the primal creative fire \
            (List 1908; Flowers trans. 1988).]",
        rune_poem_oi: "",
        deity: "Odin (as initiatory sacrificer)",
        element: "Fire",
        world: "Muspellsheim",
        esoteric: "The primal fire of creation; the beginning of all \
            magical work. List's attribution connects the wealth-rune \
            to the sacrificial fire that initiates the runic mystery.",
        meaning_upright: "Creation, beginning, primal fire, initiation.",
        meaning_reversed: "Destruction, wasted potential.",
    },
    Rune {
        number: 2,
        name: "Ur",
        alt_names: "Ur (List), cf. Elder Futhark Uruz",
        glyph: "ᚢ",
        phoneme: "u",
        etymology: "List associated *ur-* (the Proto-Germanic prefix \
            meaning 'primal, original') with this rune — giving it \
            the meaning of primal origins and the primordial world. \
            The academic word *ur-* is a real German prefix (cf. \
            *Ur-sprung*, 'origin') but List's assignment is not \
            historically attested.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 2 \
            (List 1908; Flowers 1988).]",
        rune_poem_oi: "",
        deity: "The primordial Ur-world (Ymir, the primal giant)",
        element: "Earth",
        world: "The primal world before creation",
        esoteric: "The primal origins, the Ur-world, the source behind \
            all beginnings. Memory of the ancestral world.",
        meaning_upright: "Primal origins, the beginning of all, the eternal \
            return to source.",
        meaning_reversed: "Forgetfulness of origins, lost roots.",
    },
    Rune {
        number: 3,
        name: "Thorn",
        alt_names: "Thorn / Dorn (List), cf. Elder Futhark Thurisaz",
        glyph: "ᚦ",
        phoneme: "th / d",
        etymology: "List retained the 'thorn' meaning but reinterpreted \
            it as the 'Armanist' principle of protection — the defensive \
            spine of the Ar-man (the 'noble man,' List's racial-spiritual \
            ideal). This racial reinterpretation has no academic basis.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 3 \
            (List 1908; Flowers 1988).]",
        rune_poem_oi: "",
        deity: "Thor (as protective force)",
        element: "Fire",
        world: "Midgard / Jotunheim",
        esoteric: "Protection, the active defensive principle, \
            resistance to hostile forces.",
        meaning_upright: "Protection, defensive strength, directed force.",
        meaning_reversed: "Uncontrolled aggression, harm turned inward.",
    },
    Rune {
        number: 4,
        name: "Os",
        alt_names: "Os (List), cf. Elder Futhark Ansuz",
        glyph: "ᚩ",
        phoneme: "o / a",
        etymology: "List connected *os* to 'mouth, speech, divine \
            utterance,' consistent with the OE Rune Poem's use of \
            *ōs* as 'mouth.' His Armanen attribution, however, \
            added a racial-linguistic layer (the 'divine Aryan speech') \
            that is entirely a modern invention.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 4 \
            (List 1908; Flowers 1988).]",
        rune_poem_oi: "",
        deity: "Odin (as lord of divine speech)",
        element: "Air",
        world: "Asgard",
        esoteric: "Divine speech, the power of the word, magical \
            incantation, the breath of Odin.",
        meaning_upright: "Communication, divine inspiration, the power \
            of the spoken word.",
        meaning_reversed: "False speech, manipulation.",
    },
    Rune {
        number: 5,
        name: "Rit",
        alt_names: "Rit (List), cf. Elder Futhark Raidho",
        glyph: "ᚱ",
        phoneme: "r",
        etymology: "List renamed Raidho to *Rit*, connecting it to \
            '*Recht*' (right, law) and '*Rit-ual*' (ritual). His \
            interpretation was of the rune as 'divine right order' — \
            a concept he used to argue for a pre-historical Germanic \
            legal and religious system. The etymology is speculative.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 5 \
            (List 1908; Flowers 1988).]",
        rune_poem_oi: "",
        deity: "Tyr (as god of law)",
        element: "Air",
        world: "Midgard (the law-courts, the Thing)",
        esoteric: "Divine right order, cosmic law, ritual correctness, \
            the way that is right.",
        meaning_upright: "Right action, cosmic law, justice, ritual order.",
        meaning_reversed: "Lawlessness, deviation from right order.",
    },
    Rune {
        number: 6,
        name: "Ka",
        alt_names: "Ka (List), cf. Elder Futhark Kenaz",
        glyph: "ᚲ",
        phoneme: "k / c",
        etymology: "List connected this to the concept of the \
            '*Kausalität*' (causality) and the bold, decisive \
            creative act. Like other Armanen attributions, this \
            is List's own construction without historical precedent.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 6 \
            (List 1908; Flowers 1988).]",
        rune_poem_oi: "",
        deity: "Heimdall (as bringer of civilisational fire)",
        element: "Fire",
        world: "Midgard",
        esoteric: "The torch of causality, the decisive creative act, \
            illumination of the way ahead.",
        meaning_upright: "Decisive action, illumination, the creative spark.",
        meaning_reversed: "Recklessness, burning out.",
    },
    Rune {
        number: 7,
        name: "Hagal",
        alt_names: "Hagal (List), cf. Elder Futhark Hagalaz",
        glyph: "ᚼ",
        phoneme: "h",
        etymology: "List's 'Hagal' became the central rune of his \
            system — the 'all-rune,' the *Mutterrune* (mother-rune) \
            from which all other runes can be derived geometrically. \
            This is a key Armanen doctrine: the hexagonal snowflake \
            pattern of Hagal contains within it the forms of all \
            other runes. Page (1999) notes this claim has no historical \
            basis but is geometrically interesting.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 7. \
            List's Hagal is the central 'all-rune' (*Mutterrune*) \
            from which all others are geometrically derived \
            (List 1908; Flowers 1988).]",
        rune_poem_oi: "",
        deity: "Odin (as the cosmic pattern-setter)",
        element: "All elements (the all-rune)",
        world: "All Nine Worlds (as the matrix)",
        esoteric: "The all-rune, the mother of all runes, the cosmic \
            hexagonal matrix. In List's system, Hagal is the key to \
            the entire Armanen system — all other rune forms can be \
            found within the hexagonal snowflake.",
        meaning_upright: "The cosmic matrix, all possibilities, the \
            all-containing pattern.",
        meaning_reversed: "Chaos without pattern, the matrix disrupted.",
    },
    Rune {
        number: 8,
        name: "Not",
        alt_names: "Not (List), cf. Elder Futhark Nauthiz",
        glyph: "ᚾ",
        phoneme: "n",
        etymology: "List connected this to German *Not* (need, \
            distress, emergency) — consistent with the historical \
            meaning of Nauthiz. His interpretation emphasised \
            compulsion and emergency as transformative forces.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 8 \
            (List 1908; Flowers 1988).]",
        rune_poem_oi: "",
        deity: "The Norns",
        element: "Fire",
        world: "The Well of Urðr",
        esoteric: "Compulsion, the emergency of destiny, the need-fire \
            that purifies through crisis.",
        meaning_upright: "Need, compulsion, emergency transformed, \
            the necessity that frees.",
        meaning_reversed: "Slavery to compulsion, trapped in necessity.",
    },
    Rune {
        number: 9,
        name: "Is",
        alt_names: "Is (List), cf. Elder Futhark Isa",
        glyph: "ᛁ",
        phoneme: "i",
        etymology: "List connected *Is* to the principle of the \
            self-contained ego — the 'I' that stands alone. \
            German *ich* (I) and Old Norse *ís* (ice) are brought \
            together in his interpretation of this rune as the \
            sovereign, crystalline self.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 9 \
            (List 1908; Flowers 1988).]",
        rune_poem_oi: "",
        deity: "The sovereign self (the 'I')",
        element: "Ice",
        world: "Niflheim",
        esoteric: "The crystalline ego, the self as sovereign, \
            the 'I' in its most pure concentrated form.",
        meaning_upright: "Self-mastery, the sovereign ego, concentrated will.",
        meaning_reversed: "Ego-prison, frozen self.",
    },
    Rune {
        number: 10,
        name: "Ar",
        alt_names: "Ar (List), cf. Jera / Ár",
        glyph: "ᛅ",
        phoneme: "a (long)",
        etymology: "List connected *Ar* to *Aryan* and *Ehre* (honour), \
            making this rune a racial-mystical symbol of 'Aryan' \
            nobility. This is entirely List's invention with no \
            historical basis whatsoever. The actual proto-Germanic \
            form means 'year/harvest' (as in Jera). List's racial \
            etymology of 'Aryan' from *ar-* is rejected by all \
            modern linguistics.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 10. \
            NOTE: List's racial interpretation of this rune as \
            an 'Aryan' symbol is entirely fictional and has \
            no historical basis (List 1908; Flowers 1988).]",
        rune_poem_oi: "",
        deity: "The sun (List's 'Aryan' solar symbolism)",
        element: "Fire (solar)",
        world: "Asgard (solar realm)",
        esoteric: "In List's system: the solar principle, the year-cycle, \
            honour. In the legitimate tradition the year/harvest meaning \
            from Jera/Ár applies. The racial overlay is rejected.",
        meaning_upright: "The year-cycle, honour, the solar principle, \
            harvest of effort.",
        meaning_reversed: "Dishonour, the cycle inverted.",
    },
    Rune {
        number: 11,
        name: "Sol",
        alt_names: "Sol (List), cf. Elder Futhark Sowilo",
        glyph: "ᛋ",
        phoneme: "s",
        etymology: "List's Sol rune corresponds to the sun. \
            **CRITICAL HISTORICAL NOTE**: This rune, doubled (ᛋᛋ), \
            was used as the insignia of the SS (*Schutzstaffel*), \
            designed by graphic artist Walter Heck in 1932. \
            This represents one of the most notorious appropriations \
            of runic symbolism in history. The SS rune bears no \
            relationship to legitimate solar or runic traditions.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 11. \
            CRITICAL NOTE: This rune doubled (ᛋᛋ) was the SS insignia. \
            See module documentation (List 1908; Goodrick-Clarke 2004).]",
        rune_poem_oi: "",
        deity: "Sól (the sun-goddess)",
        element: "Fire",
        world: "Asgard",
        esoteric: "The sun, solar victory, divine illumination. \
            In the legitimate tradition see Elder Futhark Sowilo.",
        meaning_upright: "Victory, solar will, success, divine illumination.",
        meaning_reversed: "Overweening ego, the arrogance of the false sun.",
    },
    Rune {
        number: 12,
        name: "Tyr",
        alt_names: "Tyr (List), cf. Elder Futhark Tiwaz",
        glyph: "ᛏ",
        phoneme: "t",
        etymology: "List's Tyr corresponds closely to the historical \
            Tiwaz. His interpretation emphasised the warrior-sacrifice \
            aspect and connected it to the concept of the 'Armanist \
            warrior-priest' — a romanticised figure with no historical \
            basis.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 12 \
            (List 1908; Flowers 1988).]",
        rune_poem_oi: "",
        deity: "Tyr",
        element: "Air",
        world: "Asgard",
        esoteric: "Justice, righteous victory, self-sacrifice, \
            the warrior's honour.",
        meaning_upright: "Justice, victory, honour, self-sacrifice.",
        meaning_reversed: "Injustice, broken honour.",
    },
    Rune {
        number: 13,
        name: "Bar",
        alt_names: "Bar (List), cf. Elder Futhark Berkano",
        glyph: "ᛒ",
        phoneme: "b",
        etymology: "List connected *Bar* to *Geburt* (birth) and \
            the birch-goddess as mother of the race. His concept \
            of the birch as 'mother of the Ario-Germanic people' \
            is a racial elaboration of the legitimate Berkano \
            birth/nurturing meaning.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 13 \
            (List 1908; Flowers 1988).]",
        rune_poem_oi: "",
        deity: "Frigg, Nerthus",
        element: "Earth",
        world: "Midgard / Vanaheim",
        esoteric: "Birth, the mother, the regenerative principle.",
        meaning_upright: "Birth, nurturing, regeneration, the mother.",
        meaning_reversed: "Failed nurturing, destructive mother-complex.",
    },
    Rune {
        number: 14,
        name: "Laf",
        alt_names: "Laf (List), cf. Elder Futhark Laguz",
        glyph: "ᛚ",
        phoneme: "l",
        etymology: "List connected this to *Laf* (remnant, legacy, \
            what is left behind) rather than the historical 'water/lake' \
            meaning. His interpretation emphasises the life-law \
            (*Lebensgesetz*) — the universal law of existence.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 14 \
            (List 1908; Flowers 1988).]",
        rune_poem_oi: "",
        deity: "Njord, Nerthus",
        element: "Water",
        world: "The World Ocean",
        esoteric: "The life-law, the legacy of the ancestors, the waters \
            of destiny.",
        meaning_upright: "Life-law, ancestral legacy, the waters of fate.",
        meaning_reversed: "Violation of life-law, destructive inheritance.",
    },
    Rune {
        number: 15,
        name: "Man",
        alt_names: "Man (List), cf. Elder Futhark Mannaz",
        glyph: "ᛗ",
        phoneme: "m",
        etymology: "List's Man rune represents the divine human in \
            his racial-mystical framework — the 'Aryo-Germanic' man \
            as the highest human type. The legitimate meaning of \
            Mannaz (humanity in general, the divine image in all \
            humans) is preferable.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 15 \
            (List 1908; Flowers 1988).]",
        rune_poem_oi: "",
        deity: "Heimdall, Odin",
        element: "Air",
        world: "Midgard",
        esoteric: "The divine human, the god within man, \
            humanity in its highest aspect.",
        meaning_upright: "Humanity, the divine image within, \
            intelligence, community.",
        meaning_reversed: "Self-deception, the human fallen from \
            divine potential.",
    },
    Rune {
        number: 16,
        name: "Yr",
        alt_names: "Yr (List) — 'Yew / Bow / Death rune'",
        glyph: "ᛦ",
        phoneme: "ü (rounded front vowel) / r",
        etymology: "List's Yr rune became notorious for its dual use \
            in the SS: the upright form (*Lebensrune*, life-rune) and \
            its inverted form (*Todesrune*, death-rune) appeared on \
            SS birth announcements and grave markers respectively. \
            **These uses are SS inventions with no basis in runic \
            tradition.** The legitimate meaning relates to the yew \
            tree and its death-rebirth paradox.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 16. \
            NOTE: The inverted form was used as the SS *Todesrune* \
            (death-rune); the upright form as the *Lebensrune* \
            (life-rune). Both SS usages are modern inventions with \
            no historical basis (List 1908; Goodrick-Clarke 2004).]",
        rune_poem_oi: "",
        deity: "Hel (death aspect), the yew tree as death-rebirth symbol",
        element: "Earth / Death",
        world: "Helheim / Midgard",
        esoteric: "The yew's paradox: death and life intertwined. \
            The bow bent in tension. Endurance through extremity.",
        meaning_upright: "Death-wisdom, the yew's endurance, life through \
            death, the archer's patience.",
        meaning_reversed: "Fear of death, destructive clinging.",
    },
    Rune {
        number: 17,
        name: "Eh",
        alt_names: "Eh (List), cf. Elder Futhark Ehwaz",
        glyph: "ᛖ",
        phoneme: "e",
        etymology: "List associated Eh with marriage (*Ehe* in German), \
            the sacred union of complementary forces. This is a \
            plausible if speculative connection — the horse-partnership \
            of Ehwaz and the marriage-partnership share the root \
            concept of complementary union.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 17 \
            (List 1908; Flowers 1988).]",
        rune_poem_oi: "",
        deity: "Frigg (as goddess of marriage), Odin",
        element: "Air / Earth",
        world: "Midgard",
        esoteric: "Sacred marriage, the union of complementary forces, \
            partnership as a sacred covenant.",
        meaning_upright: "Sacred union, partnership, the marriage of \
            complementary forces.",
        meaning_reversed: "Broken union, incompatible partnership.",
    },
    Rune {
        number: 18,
        name: "Gibor",
        alt_names: "Gibor (List), cf. Elder Futhark Gebo",
        glyph: "ᚷ",
        phoneme: "g",
        etymology: "List's final rune *Gibor* corresponds to \
            Gebo (gift) but he attributed it to the final *ljóðatal* \
            stanza of the Hávamál, connecting it to the mystery of \
            the divine gift — the *Gibu* (gift of the gods to \
            humanity, cf. Gothic *giba*). As the 18th and final \
            Armanen rune, Gibor represents the culmination of the \
            runic mystery: the gift of consciousness itself.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "[Armanen attribution to Hávamál st. 18 — \
            the final magical song. List's Gibor represents the \
            culminating gift of divine consciousness \
            (List 1908; Flowers 1988).]",
        rune_poem_oi: "",
        deity: "Odin (as the gift-giver of consciousness), the Allfather",
        element: "All elements",
        world: "All Nine Worlds",
        esoteric: "The final and supreme gift — consciousness itself, \
            the divine awareness breathed into the human by the gods. \
            Gibor closes the Armanen cycle as the gift that makes all \
            other gifts possible.",
        meaning_upright: "The divine gift, consciousness, the culmination \
            of the runic mystery, sacred reciprocity.",
        meaning_reversed: "The gift refused or squandered, unconsciousness.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armanen_has_18_runes() {
        assert_eq!(ARMANEN.len(), 18);
    }

    #[test]
    fn armanen_numbered_correctly() {
        for (i, rune) in ARMANEN.iter().enumerate() {
            assert_eq!(rune.number as usize, i + 1, "Rune '{}' has wrong number", rune.name);
        }
    }
}
