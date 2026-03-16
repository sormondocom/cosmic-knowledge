//! Vedic (Anka Vidya) numerology — the Indian tradition of letter-number correspondences.
//!
//! ## Letter values
//!
//! The Latin-alphabet phonetic mapping follows the standard documented in:
//!  - Johari, H. *Numerology: with Tantra, Ayurveda, and Astrology* (1990, Destiny Books)
//!  - Chaudhry, J.C. *Numerology for Success* (2002, Sterling)
//!
//! Each letter value corresponds to one of the **Navagraha** (nine celestial bodies of
//! Jyotish): Sun, Moon, Jupiter, Rahu, Mercury, Venus, Ketu, Saturn, Mars.  The number 9
//! (Mars / Mangal) is considered sacred and is never assigned to an individual letter — the
//! same convention as the Babylonian/Chaldean tradition from which the Indian system draws.
//! Totals that reduce to 9 are valid and carry the Mars / Mangal interpretation.
//!
//! ## Interpretive framework
//!
//! Unlike Western numerology systems (Pythagorean, Chaldean), each digit in Vedic
//! numerology carries a full planetary identity that links to:
//!  - **Navagraha** planetary deity and archetype
//!  - **Bīja mantra** (seed syllable) used to invoke the planet's energy
//!  - **Navaratna** gemstone prescribed to strengthen or balance the planet
//!  - **Ayurvedic dosha** (Vāta, Pitta, Kapha) associated with the planetary force
//!  - **Day of the week** sacred to the ruling graha
//!
//! **Primary scholarly sources:**
//!  - Johari, H. *Numerology: with Tantra, Ayurveda, and Astrology* (1990, Destiny Books)
//!  - Defouw, H. & Svoboda, R. *Light on Life: An Introduction to the Astrology of India*
//!    (1996, Arkana / Penguin) — Navagraha attributes
//!  - Frawley, D. *Astrology of the Seers: A Guide to Vedic/Hindu Astrology*
//!    (1990, Passage Press) — planetary meanings and doshas
//!  - Svoboda, R. *Aghora: At the Left Hand of God* (1986, Brotherhood of Life) —
//!    bīja mantras and gemstone lore
//!  - Rao, P.V.R.N. *Vedic Astrology: An Integrated Approach* (2000) — Navagraha overview
//!  - *Bṛhat Parāśara Horā Śāstra* (ancient; trans. Santhanam, 1984, Ranjan) —
//!    the foundational Jyotish text for planetary natures
//!  - Lad, V. *Textbook of Ayurveda*, Vol. 1 (2002, Ayurvedic Press) — dosha attributes

use std::collections::HashMap;
use once_cell::sync::Lazy;

// ─── Letter map ───────────────────────────────────────────────────────────────

/// Vedic letter-to-number map (Latin phonetic approximation, Johari 1990).
///
/// Values 1–8 correspond to the eight visible Navagraha; 9 (Mars) is held
/// sacred and unassigned to individual letters.
pub(super) static MAP: Lazy<HashMap<char, u32>> = Lazy::new(|| [
    ('A',1),('B',2),('C',3),('D',4),('E',5),('F',8),('G',3),('H',5),('I',1),
    ('J',1),('K',2),('L',3),('M',4),('N',5),('O',7),('P',8),('Q',1),('R',2),
    ('S',3),('T',4),('U',6),('V',6),('W',6),('X',5),('Y',1),('Z',7),
].into_iter().collect());

// ─── Planetary reading ────────────────────────────────────────────────────────

/// Full Vedic numerological reading for a single root number (1–9).
#[allow(dead_code)]
pub struct VedicReading {
    /// Root number this reading belongs to.
    pub root:    u32,
    /// Graha name in English and transliterated Sanskrit.
    pub planet:  &'static str,
    /// Graha name in Devanāgarī script.
    pub sanskrit: &'static str,
    /// Bīja (seed) mantra used to invoke or pacify the graha.
    pub bija:    &'static str,
    /// Navaratna gemstone name (English).
    pub gem:     &'static str,
    /// Navaratna gemstone name (transliterated Sanskrit + Devanāgarī).
    pub gem_skt: &'static str,
    /// Ayurvedic dosha(s) associated with this graha.
    pub dosha:   &'static str,
    /// Auspicious colour(s) linked to this graha.
    pub color:   &'static str,
    /// Sacred day of the week.
    pub day:     &'static str,
    /// Primary Vedic interpretive meaning.
    pub meaning: &'static str,
}

static VEDIC_DATA: &[VedicReading] = &[
    // Index 0 unused (roots run 1–9)
    VedicReading {
        root: 0, planet: "", sanskrit: "", bija: "", gem: "", gem_skt: "",
        dosha: "", color: "", day: "", meaning: "",
    },
    // 1 — Sun (Sūrya)
    VedicReading {
        root:    1,
        planet:  "Sun (Surya)",
        sanskrit: "सूर्य",
        bija:    "Oṃ Hrāṃ Hrīṃ Hrauṃ Saḥ Sūryāya Namaḥ",
        gem:     "Ruby",
        gem_skt: "Māṇikya (माणिक्य)",
        dosha:   "Pitta",
        color:   "Gold, Deep Red, Orange",
        day:     "Sunday (Ravivār)",
        meaning: "The Ātman — the individual self and solar intelligence. Leadership, \
                  vitality, authority, and the power to illuminate. The soul's direct \
                  expression in the world. Governs the father principle, the spine, \
                  the heart, and the eyes. Strong 1s are born leaders whose path \
                  demands self-knowledge and fearless originality.",
    },
    // 2 — Moon (Chandra)
    VedicReading {
        root:    2,
        planet:  "Moon (Chandra)",
        sanskrit: "चन्द्र",
        bija:    "Oṃ Śrāṃ Śrīṃ Śrauṃ Saḥ Chandrāya Namaḥ",
        gem:     "Pearl",
        gem_skt: "Moti (मोती)",
        dosha:   "Kapha, Vāta",
        color:   "White, Silver, Pale Blue",
        day:     "Monday (Somavār)",
        meaning: "Manas — the receptive mind and the seat of emotion. The mother \
                  principle, memory, imagination, and all that is fluid and cyclical. \
                  Governs the emotions, the stomach, the left eye, and the chest. \
                  Chandra's energy is nurturing, psychic, and deeply sensitive to \
                  the rhythms of time and relationship.",
    },
    // 3 — Jupiter (Guru / Bṛhaspati)
    VedicReading {
        root:    3,
        planet:  "Jupiter (Guru / Brhaspati)",
        sanskrit: "गुरु",
        bija:    "Oṃ Brāṃ Brīṃ Brauṃ Saḥ Guruve Namaḥ",
        gem:     "Yellow Sapphire",
        gem_skt: "Pushyarāga (पुष्पराग)",
        dosha:   "Kapha",
        color:   "Yellow, Golden, Saffron",
        day:     "Thursday (Guruvār)",
        meaning: "Dharma, wisdom, and divine grace — the preceptor of the gods. \
                  Bṛhaspati bestows knowledge of sacred scripture, righteous conduct, \
                  and the capacity for spiritual expansion. Governs the liver, the \
                  hips, and higher learning. Number 3 carries the blessing of the \
                  teacher archetype: generous, optimistic, and oriented toward truth.",
    },
    // 4 — Rāhu (North Node)
    VedicReading {
        root:    4,
        planet:  "Rahu (North Node)",
        sanskrit: "राहु",
        bija:    "Oṃ Bhrāṃ Bhrīṃ Bhrauṃ Saḥ Rāhave Namaḥ",
        gem:     "Hessonite Garnet",
        gem_skt: "Gomeda (गोमेद)",
        dosha:   "Vāta",
        color:   "Smoky Brown, Dark Blue, Ultraviolet",
        day:     "Saturday (Śanivār) — shares rulership with Śani",
        meaning: "The shadow planet of worldly obsession, karmic momentum, and the \
                  uncharted frontier. Rāhu amplifies desire, drives innovation, and \
                  pulls consciousness toward the unfamiliar and foreign. It governs \
                  sudden events, epidemics, technology, and collective illusion (Māyā). \
                  Number 4 natives are unconventional, intensely driven, and carry \
                  a karmic imperative to confront the shadow of materialism.",
    },
    // 5 — Mercury (Budha)
    VedicReading {
        root:    5,
        planet:  "Mercury (Budha)",
        sanskrit: "बुध",
        bija:    "Oṃ Brāṃ Brīṃ Brauṃ Saḥ Budhāya Namaḥ",
        gem:     "Emerald",
        gem_skt: "Pannā (पन्ना)",
        dosha:   "Tridoshic (all three in balance)",
        color:   "Green, Variegated",
        day:     "Wednesday (Budhavār)",
        meaning: "Intellect, discernment, and the power of communication. Budha is \
                  the prince of the grahas — quick, adaptable, mercurial, and skilled \
                  in trade, language, mathematics, and analysis. Governs the nervous \
                  system, speech organs, and skin. Number 5 natives are versatile, \
                  witty, and perpetually curious; their spiritual path lies in refining \
                  discrimination (viveka) from mere cleverness into wisdom.",
    },
    // 6 — Venus (Śukra)
    VedicReading {
        root:    6,
        planet:  "Venus (Shukra)",
        sanskrit: "शुक्र",
        bija:    "Oṃ Drāṃ Drīṃ Drauṃ Saḥ Śukrāya Namaḥ",
        gem:     "Diamond",
        gem_skt: "Hīrā (हीरा) or White Sapphire",
        dosha:   "Kapha, Pitta",
        color:   "White, Pink, Iridescent",
        day:     "Friday (Śukravār)",
        meaning: "Beauty, refined pleasure, artistic sensibility, and the power of \
                  devotion. Śukra is the preceptor of the asuras — possessing a \
                  profound knowledge of regeneration, desire, and the life-force (ojas). \
                  Governs the reproductive system, kidneys, and the arts. Number 6 \
                  carries the energy of Lakṣmī: abundance flows through beauty, \
                  harmony, and the cultivation of love in its many forms.",
    },
    // 7 — Ketu (South Node)
    VedicReading {
        root:    7,
        planet:  "Ketu (South Node)",
        sanskrit: "केतु",
        bija:    "Oṃ Srāṃ Srīṃ Srauṃ Saḥ Ketave Namaḥ",
        gem:     "Cat's Eye",
        gem_skt: "Lahsuniyā (लहसुनिया)",
        dosha:   "Pitta, Vāta",
        color:   "Grey, Smoky, Mottled",
        day:     "Thursday (Guruvār) — shares rulership with Guru",
        meaning: "Liberation (mokṣa), renunciation, and accumulated spiritual merit \
                  from past lives. Ketu dissolves material attachment and opens the \
                  inner eye of psychic perception. Governs mysterious illnesses, \
                  sudden spiritual awakenings, and the occult. Number 7 natives carry \
                  deep wisdom from prior incarnations; their path moves naturally \
                  toward mysticism, solitude, and the direct experience of the \
                  formless.",
    },
    // 8 — Saturn (Śani)
    VedicReading {
        root:    8,
        planet:  "Saturn (Shani)",
        sanskrit: "शनि",
        bija:    "Oṃ Prāṃ Prīṃ Prauṃ Saḥ Śanaiścarāya Namaḥ",
        gem:     "Blue Sapphire",
        gem_skt: "Nīlam (नीलम)",
        dosha:   "Vāta",
        color:   "Black, Dark Blue, Indigo",
        day:     "Saturday (Śanivār)",
        meaning: "The great karmic adjudicator — slowness, discipline, perseverance, \
                  and the weight of accumulated action. Śani rules the bones, teeth, \
                  the span of life, and all that is built through sustained effort \
                  over time. He rewards integrity and brings suffering proportional \
                  to past transgressions. Number 8 natives are old souls whose path \
                  demands humility, patience, and service; mastery — when earned — is \
                  lasting and profound.",
    },
    // 9 — Mars (Maṅgala)
    VedicReading {
        root:    9,
        planet:  "Mars (Mangala)",
        sanskrit: "मंगल",
        bija:    "Oṃ Krāṃ Krīṃ Krauṃ Saḥ Bhauma Namaḥ",
        gem:     "Red Coral",
        gem_skt: "Moṅgā (मूँगा)",
        dosha:   "Pitta",
        color:   "Red, Crimson, Scarlet",
        day:     "Tuesday (Maṅgalavār)",
        meaning: "Pure energy, courage, and the warrior principle. Maṅgala governs \
                  the blood, the muscles, the capacity for decisive action, and the \
                  martial arts. He is the protector of dharma who acts without \
                  hesitation. In the Vedic tradition 9 is held sacred — the number \
                  of completion and divine totality, the sum of all single digits, \
                  not assigned to individual letters yet pervading all things as \
                  the universal life-force (prāṇa).",
    },
];

// ─── Public accessors ─────────────────────────────────────────────────────────

/// Return the [`VedicReading`] for a root number (1–9).
/// Returns the Mars (9) entry for 0 or any value > 9.
pub fn vedic_reading(root: u32) -> &'static VedicReading {
    let idx = if root >= 1 && root <= 9 { root as usize } else { 9 };
    &VEDIC_DATA[idx]
}

