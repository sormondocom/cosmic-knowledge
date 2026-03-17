//! Angelic Tarot — Minor Arcana (56 cards) and the Shem HaMephorash (72 angels).
//!
//! ## The Shem HaMephorash
//!
//! The seventy-two names of God (*Shem HaMephorash*, שֵׁם הַמְפֹרָשׁ) are derived from
//! Exodus 14:19–21 — three verses of 72 Hebrew letters each — by a cipher called
//! *boustrophedon*.  Each three-letter name, suffixed with either *-iah* (יָה, divine)
//! or *-el* (אֵל, of God), becomes the name of a guardian angel governing 5° of the
//! zodiacal circle.  The full 360° divide into 72 × 5° sectors; each pair of angels
//! (10°) corresponds to one decan.
//!
//! ## Decan–card correspondences
//!
//! The Hermetic Order of the Golden Dawn assigned the numbered pip cards (2 through 10)
//! of each suit to one decan of their suit's element:
//!
//! | Suit      | Element | Signs (decans 1–3 each)               |
//! |-----------|---------|---------------------------------------|
//! | Wands     | Fire    | Aries · Leo · Sagittarius             |
//! | Cups      | Water   | Cancer · Scorpio · Pisces             |
//! | Swords    | Air     | Libra · Aquarius · Gemini             |
//! | Pentacles | Earth   | Capricorn · Taurus · Virgo            |
//!
//! Aces represent the pure undifferentiated element, governed by the elemental
//! archangel rather than a Shem HaMephorash pair.  Court cards bear elemental
//! sub-attributions (King=Fire, Queen=Water, Knight=Air, Page=Earth of the suit
//! element) and are governed by the suit's elemental archangel.
//!
//! ## Sources
//!
//! - **Exodus** 14:19–21 — the three 72-letter verses from which the Shem HaMephorash
//!   is derived by boustrophedon cipher.
//! - **Shimmushei Torah** / **Sefer Raziel HaMalakh** — early Kabbalistic sources
//!   giving the 72 angel names and their 5° zodiacal domains.
//! - **Agrippa**, *Three Books of Occult Philosophy* III:25 — tables of 72 angels.
//! - **Regardie**, *The Golden Dawn* (1937–40; Llewellyn 1989) — Tarot decan
//!   assignments and angelic correspondences.
//! - **Godwin**, *Godwin's Cabalistic Encyclopedia* (1979; 3rd ed. Llewellyn 1994) —
//!   comprehensive reference for Shem HaMephorash names and qualities.
//! - **Ambelain**, *La Kabbale Pratique* (1951; trans. Rankine & Barron as
//!   *Practical Kabbalah*, 2002) — French Kabbalistic tradition for divine qualities.

// ─── Shem HaMephorash ─────────────────────────────────────────────────────────

/// One of the 72 angels of the Shem HaMephorash.
pub struct ShemAngel {
    /// Sequential number 1–72.
    pub number: u8,
    /// Angel name (standard Latin/English transliteration).
    pub name: &'static str,
    /// Three-letter Hebrew root (the name's Exodus derivation).
    pub hebrew_root: &'static str,
    /// Zodiac degrees governed: e.g. "0°–5° Aries".
    pub degrees: &'static str,
    /// Divine quality or sphere of influence.
    pub quality: &'static str,
}

pub static SHEM_HAMEPHORASH: &[ShemAngel] = &[
    // ── Aries (0°–30°) ────────────────────────────────────────────────────────
    ShemAngel { number:  1, name: "Vehuiah",   hebrew_root: "וְהוּ",  degrees:  "0°–5° Aries",    quality: "Divine will and transformation; the spark that ignites sacred change" },
    ShemAngel { number:  2, name: "Jeliel",    hebrew_root: "יְלִי",  degrees:  "5°–10° Aries",   quality: "Love, wisdom, and the divine seed planted in the heart of creation" },
    ShemAngel { number:  3, name: "Sitael",    hebrew_root: "סִיט",   degrees: "10°–15° Aries",   quality: "Construction, shelter, and the building of lasting sacred structures" },
    ShemAngel { number:  4, name: "Elemiah",   hebrew_root: "עֶלֶם",  degrees: "15°–20° Aries",   quality: "Travel, inner journeying, and the healing of mental turmoil" },
    ShemAngel { number:  5, name: "Mahasiah",  hebrew_root: "מַהַשׁ", degrees: "20°–25° Aries",   quality: "Rectification and the grace to learn from error; sacred education" },
    ShemAngel { number:  6, name: "Lelahel",   hebrew_root: "לֶלֵה",  degrees: "25°–30° Aries",   quality: "Light, fortune, and the illumined love that reveals the divine plan" },
    // ── Taurus (0°–30°) ───────────────────────────────────────────────────────
    ShemAngel { number:  7, name: "Achaiah",   hebrew_root: "אַכָא",  degrees:  "0°–5° Taurus",   quality: "Patient discovery; the grace to uncover hidden truths through perseverance" },
    ShemAngel { number:  8, name: "Cahetel",   hebrew_root: "כַּהֵת", degrees:  "5°–10° Taurus",  quality: "Divine blessing of the earth; abundant harvest and sacred agriculture" },
    ShemAngel { number:  9, name: "Haziel",    hebrew_root: "הַזִי",  degrees: "10°–15° Taurus",  quality: "Divine mercy and forgiveness; the grace of friendship and reconciliation" },
    ShemAngel { number: 10, name: "Aladiah",   hebrew_root: "אַלַד",  degrees: "15°–20° Taurus",  quality: "Healing grace; restoration of health, fortune, and divine favour" },
    ShemAngel { number: 11, name: "Lauviah",   hebrew_root: "לָאוּ",  degrees: "20°–25° Taurus",  quality: "Victory, revelations, and wisdom received in sacred dreams" },
    ShemAngel { number: 12, name: "Hahaiah",   hebrew_root: "הַהַע",  degrees: "25°–30° Taurus",  quality: "Refuge in times of trial; the divine shelter within the storm" },
    // ── Gemini (0°–30°) ───────────────────────────────────────────────────────
    ShemAngel { number: 13, name: "Yezalel",   hebrew_root: "יֶזַל",  degrees:  "0°–5° Gemini",   quality: "Fidelity, memory, and the sacred bonds of loyal relationship" },
    ShemAngel { number: 14, name: "Mebahel",   hebrew_root: "מֶבַה",  degrees:  "5°–10° Gemini",  quality: "Truth, justice, and the divine protection of the innocent" },
    ShemAngel { number: 15, name: "Hariel",    hebrew_root: "הַרִי",  degrees: "10°–15° Gemini",  quality: "Purification and the sanctified arts, sciences, and sacred study" },
    ShemAngel { number: 16, name: "Hakamiah",  hebrew_root: "הַקֶם",  degrees: "15°–20° Gemini",  quality: "Loyalty, victory against adversity, and the defeat of treachery" },
    ShemAngel { number: 17, name: "Lauvuel",   hebrew_root: "לָאוּ",  degrees: "20°–25° Gemini",  quality: "Divine glory and inner illumination; the praise that rises to heaven" },
    ShemAngel { number: 18, name: "Caliel",    hebrew_root: "כַּלִי",  degrees: "25°–30° Gemini",  quality: "Divine justice in trial; the angel who cuts through illusion with truth" },
    // ── Cancer (0°–30°) ───────────────────────────────────────────────────────
    ShemAngel { number: 19, name: "Leuviah",   hebrew_root: "לֵאוּ",  degrees:  "0°–5° Cancer",   quality: "Expansive intelligence; memory, grace, and the gift of patience" },
    ShemAngel { number: 20, name: "Pahaliah",  hebrew_root: "פַּהַל",  degrees:  "5°–10° Cancer",  quality: "Redemption, vocation, and the courage to pursue the sacred calling" },
    ShemAngel { number: 21, name: "Nelkhael",  hebrew_root: "נֶלֶכ",  degrees: "10°–15° Cancer",  quality: "Liberation from ignorance; mastery in learning and the esoteric sciences" },
    ShemAngel { number: 22, name: "Yeiayel",   hebrew_root: "יֵיַי",  degrees: "15°–20° Cancer",  quality: "Fame, fortune, and divine aid for those who serve the greater good" },
    ShemAngel { number: 23, name: "Melahel",   hebrew_root: "מֵלַה",  degrees: "20°–25° Cancer",  quality: "Healing waters and the grace to cure; protection on journeys" },
    ShemAngel { number: 24, name: "Haheuiah",  hebrew_root: "חַהֲוּ", degrees: "25°–30° Cancer",  quality: "Divine protection and truth; refuge for those who seek honest counsel" },
    // ── Leo (0°–30°) ──────────────────────────────────────────────────────────
    ShemAngel { number: 25, name: "Nith-Haiah",hebrew_root: "נִיתֵ",  degrees:  "0°–5° Leo",      quality: "Wisdom magic and prophecy; the divine gift of inspired understanding" },
    ShemAngel { number: 26, name: "Haaiah",    hebrew_root: "הַאַי",  degrees:  "5°–10° Leo",     quality: "Political wisdom, sacred contemplation, and discernment in governance" },
    ShemAngel { number: 27, name: "Yeratel",   hebrew_root: "יֶרַת",  degrees: "10°–15° Leo",     quality: "Propagation of the light; sacred mission to spread divine knowledge" },
    ShemAngel { number: 28, name: "Seheiah",   hebrew_root: "שֵׁהֵי",  degrees: "15°–20° Leo",     quality: "Longevity and protection from illness, lightning, and sudden misfortune" },
    ShemAngel { number: 29, name: "Reiyel",    hebrew_root: "רֵייֵ",  degrees: "20°–25° Leo",     quality: "Liberation from enchantment; meditation that opens the inner eye" },
    ShemAngel { number: 30, name: "Omael",     hebrew_root: "עוֹמַ",  degrees: "25°–30° Leo",     quality: "Divine patience and the multiplication of life's sacred abundance" },
    // ── Virgo (0°–30°) ────────────────────────────────────────────────────────
    ShemAngel { number: 31, name: "Lecabel",   hebrew_root: "לֵיכַ",  degrees:  "0°–5° Virgo",    quality: "Sacred agriculture; intellectual mastery of crafts and natural cycles" },
    ShemAngel { number: 32, name: "Vasariah",  hebrew_root: "וַשַׁר",  degrees:  "5°–10° Virgo",   quality: "Clemency, divine justice, and the fair weighing of actions and souls" },
    ShemAngel { number: 33, name: "Yehuiah",   hebrew_root: "יֶהוּ",  degrees: "10°–15° Virgo",   quality: "Divine subordination; protection in loyal service to a greater purpose" },
    ShemAngel { number: 34, name: "Lehahiah",  hebrew_root: "לֵהַח",  degrees: "15°–20° Virgo",   quality: "Obedience to divine order; the peace that flows from harmonious discipline" },
    ShemAngel { number: 35, name: "Chavakiah", hebrew_root: "חַוָקִ", degrees: "20°–25° Virgo",   quality: "Reconciliation, family harmony, and recovery of what was lost or scattered" },
    ShemAngel { number: 36, name: "Menadel",   hebrew_root: "מֶנַד",  degrees: "25°–30° Virgo",   quality: "Sacred work and the liberation of those unjustly exiled or imprisoned" },
    // ── Libra (0°–30°) ────────────────────────────────────────────────────────
    ShemAngel { number: 37, name: "Aniel",     hebrew_root: "אַנִי",  degrees:  "0°–5° Libra",    quality: "Breaking vicious cycles; the courage to return to virtue after error" },
    ShemAngel { number: 38, name: "Haamiah",   hebrew_root: "חַעַמ",  degrees:  "5°–10° Libra",   quality: "Sacred ritual, truth-seeking, and the protocols of divine ceremony" },
    ShemAngel { number: 39, name: "Rehael",    hebrew_root: "רֵהַע",  degrees: "10°–15° Libra",   quality: "Health, longevity, and the restoration of harmony between generations" },
    ShemAngel { number: 40, name: "Ieiazel",   hebrew_root: "יֵיַז",  degrees: "15°–20° Libra",   quality: "Liberation from oppression; grace for artists and those who seek freedom" },
    ShemAngel { number: 41, name: "Hahahel",   hebrew_root: "הַהַח",  degrees: "20°–25° Libra",   quality: "Sacred mission, consecration, and unshakeable faith in the divine" },
    ShemAngel { number: 42, name: "Mikael",    hebrew_root: "מִיכַ",  degrees: "25°–30° Libra",   quality: "Divine order and political balance; protection of sacred hierarchies" },
    // ── Scorpio (0°–30°) ──────────────────────────────────────────────────────
    ShemAngel { number: 43, name: "Veuliah",   hebrew_root: "וֵאוּ",  degrees:  "0°–5° Scorpio",  quality: "Prosperity, freedom, and the transformation of poverty into abundance" },
    ShemAngel { number: 44, name: "Yelahiah",  hebrew_root: "יֵלַה",  degrees:  "5°–10° Scorpio", quality: "Sacred courage and victory; protection of warriors of righteousness" },
    ShemAngel { number: 45, name: "Sealiah",   hebrew_root: "סֵאַל",  degrees: "10°–15° Scorpio", quality: "Divine motivation; the exaltation of the humble and healing of the despairing" },
    ShemAngel { number: 46, name: "Ariel",     hebrew_root: "עַרִי",  degrees: "15°–20° Scorpio", quality: "Subtle perception, inspired revelation, and gratitude for divine gifts" },
    ShemAngel { number: 47, name: "Asaliah",   hebrew_root: "עַשַׁל",  degrees: "20°–25° Scorpio", quality: "Contemplation of divine law; the joy of conforming to cosmic truth" },
    ShemAngel { number: 48, name: "Mihael",    hebrew_root: "מִיהָ",  degrees: "25°–30° Scorpio", quality: "Sacred love, fertile peace, and the protection of loving bonds" },
    // ── Sagittarius (0°–30°) ──────────────────────────────────────────────────
    ShemAngel { number: 49, name: "Vehuel",    hebrew_root: "וֵהוּ",  degrees:  "0°–5° Sagittarius",  quality: "Elevation, grandeur, and the praise of God in moments of great glory" },
    ShemAngel { number: 50, name: "Daniel",    hebrew_root: "דַּנִי",  degrees:  "5°–10° Sagittarius", quality: "Divine eloquence, beauty, and the sacred gift of clear speech" },
    ShemAngel { number: 51, name: "Hahasiah",  hebrew_root: "הַחַשׁ", degrees: "10°–15° Sagittarius", quality: "Universal medicine; the hidden wisdom that heals body and soul alike" },
    ShemAngel { number: 52, name: "Imamiah",   hebrew_root: "עֲמַמ",  degrees: "15°–20° Sagittarius", quality: "Expiation of wrongs; the grace that eases difficult journeys and captivity" },
    ShemAngel { number: 53, name: "Nanael",    hebrew_root: "נַנַא",  degrees: "20°–25° Sagittarius", quality: "Spiritual communication and the contemplative silence in which God is heard" },
    ShemAngel { number: 54, name: "Nithael",   hebrew_root: "נִיתַ",  degrees: "25°–30° Sagittarius", quality: "Celestial kingship; divine fire that maintains the order of the cosmos" },
    // ── Capricorn (0°–30°) ────────────────────────────────────────────────────
    ShemAngel { number: 55, name: "Mebahiah",  hebrew_root: "מֵבַה",  degrees:  "0°–5° Capricorn",  quality: "Intellectual lucidity, moral clarity, and the consolation of reason" },
    ShemAngel { number: 56, name: "Poyel",     hebrew_root: "פּוֹיֵ",  degrees:  "5°–10° Capricorn", quality: "Fortune, renown, and the divine support that uplifts the deserving" },
    ShemAngel { number: 57, name: "Nemamiah",  hebrew_root: "נֵמַמ",  degrees: "10°–15° Capricorn", quality: "Discernment and righteous action; success for those who defend the just" },
    ShemAngel { number: 58, name: "Yeialel",   hebrew_root: "יֵיַל",  degrees: "15°–20° Capricorn", quality: "Mental fortitude; the healing of mind and eyes to see with divine clarity" },
    ShemAngel { number: 59, name: "Harahel",   hebrew_root: "חַרַה",  degrees: "20°–25° Capricorn", quality: "Intellectual riches; fertility of mind and the gifts of sacred knowledge" },
    ShemAngel { number: 60, name: "Mitzrael",  hebrew_root: "מִצְר",  degrees: "25°–30° Capricorn", quality: "Internal reparation; healing the rift between the self and its divine source" },
    // ── Aquarius (0°–30°) ─────────────────────────────────────────────────────
    ShemAngel { number: 61, name: "Umabel",    hebrew_root: "אוּמַ",  degrees:  "0°–5° Aquarius",  quality: "Friendship, affinity, and the sacred bonds of mutual recognition" },
    ShemAngel { number: 62, name: "Iah-Hel",   hebrew_root: "יָהֵ",  degrees:  "5°–10° Aquarius", quality: "Sacred wisdom, peaceful solitude, and the contemplative illumination" },
    ShemAngel { number: 63, name: "Anauel",    hebrew_root: "עֲנָוּ",  degrees: "10°–15° Aquarius", quality: "Unity of opposites; commerce made holy by integrity and mutual benefit" },
    ShemAngel { number: 64, name: "Mehiel",    hebrew_root: "מֵחִי",  degrees: "15°–20° Aquarius", quality: "Vivification and divine inspiration; protection of those who write and teach" },
    ShemAngel { number: 65, name: "Damabiah",  hebrew_root: "דַּמַב",  degrees: "20°–25° Aquarius", quality: "Fountain of divine wisdom; navigation, ships, and the overcoming of sorcery" },
    ShemAngel { number: 66, name: "Manakel",   hebrew_root: "מַנָקֵ",  degrees: "25°–30° Aquarius", quality: "Appeasement of the wrathful; the peace of God descending in healing dreams" },
    // ── Pisces (0°–30°) ───────────────────────────────────────────────────────
    ShemAngel { number: 67, name: "Eyael",     hebrew_root: "אֵיעַ",  degrees:  "0°–5° Pisces",   quality: "Transcendent consolation; the divine comfort that rises above all suffering" },
    ShemAngel { number: 68, name: "Habuhiah",  hebrew_root: "חַבוּ",  degrees:  "5°–10° Pisces",  quality: "Healing, fertility, and abundance flowing from divine generosity" },
    ShemAngel { number: 69, name: "Rochel",    hebrew_root: "רֵאהֵ",  degrees: "10°–15° Pisces",  quality: "Restitution of the lost; the angel who finds what has gone astray" },
    ShemAngel { number: 70, name: "Jabamiah",  hebrew_root: "יָבַמ",  degrees: "15°–20° Pisces",  quality: "Sacred alchemy; the great regeneration that transmutes base matter into spirit" },
    ShemAngel { number: 71, name: "Haiaiel",   hebrew_root: "הַיַי",  degrees: "20°–25° Pisces",  quality: "Divine armament; the celestial sword that defends the righteous" },
    ShemAngel { number: 72, name: "Mumiah",    hebrew_root: "מוּמֵ",  degrees: "25°–30° Pisces",  quality: "Sacred endings and new beginnings; the angel of health and the final mystery" },
];

// ─── Minor Arcana card type ───────────────────────────────────────────────────

/// One Minor Arcana card with angelic and decan correspondences.
pub struct MinorArcanum {
    /// "Wands", "Cups", "Swords", or "Pentacles".
    pub suit: &'static str,
    /// Card display name: "Ace", "Two" … "Ten", "Page", "Knight", "Queen", "King".
    pub rank_name: &'static str,
    /// Elemental nature of the suit.
    pub element: &'static str,
    /// Sub-elemental description for court cards (e.g. "Fire of Fire").
    /// Empty for aces and numbered pips.
    pub sub_element: &'static str,
    /// Primary angel or elemental archangel.
    pub angel: &'static str,
    /// Secondary Shem angel (second 5° of the decan).  Empty if not applicable.
    pub angel2: &'static str,
    /// Astrological decan (e.g. "Mars in Aries (0°–10°)").  Empty for aces/courts.
    pub decan: &'static str,
    /// Core interpretive keywords.
    pub keywords: &'static str,
    /// The divine quality or angelic blessing of this card.
    pub divine_quality: &'static str,
}

// ─── The fifty-six Minor Arcana ───────────────────────────────────────────────

pub static MINOR_ARCANA: &[MinorArcanum] = &[

    // ════════════════════════════════════════════════════════════════════════
    //  WANDS — Element: Fire
    //  Zodiac signs: Aries (cards 2–4) · Leo (5–7) · Sagittarius (8–10)
    // ════════════════════════════════════════════════════════════════════════

    MinorArcanum {
        suit: "Wands", rank_name: "Ace",
        element: "Fire", sub_element: "",
        angel: "Michael", angel2: "",
        decan: "",
        keywords: "Creative spark · new passion · sacred will · the divine flame",
        divine_quality: "The pure undivided fire of divine creation; Michael's solar sword",
    },
    MinorArcanum {
        suit: "Wands", rank_name: "Two",
        element: "Fire", sub_element: "",
        angel: "Vehuiah", angel2: "Jeliel",
        decan: "Mars in Aries (0°–10°)",
        keywords: "Bold vision · personal power · planning · courageous initiative",
        divine_quality: "Vehuiah's transforming will aligned with Jeliel's divine wisdom",
    },
    MinorArcanum {
        suit: "Wands", rank_name: "Three",
        element: "Fire", sub_element: "",
        angel: "Sitael", angel2: "Elemiah",
        decan: "Sun in Aries (10°–20°)",
        keywords: "Foresight · expansion · enterprise · the fruit of bold beginnings",
        divine_quality: "Sitael's sacred building illumined by Elemiah's healing journey",
    },
    MinorArcanum {
        suit: "Wands", rank_name: "Four",
        element: "Fire", sub_element: "",
        angel: "Mahasiah", angel2: "Lelahel",
        decan: "Venus in Aries (20°–30°)",
        keywords: "Celebration · homecoming · stability · the harvest of effort",
        divine_quality: "Mahasiah's rectifying grace crowned by Lelahel's fortunate light",
    },
    MinorArcanum {
        suit: "Wands", rank_name: "Five",
        element: "Fire", sub_element: "",
        angel: "Nith-Haiah", angel2: "Haaiah",
        decan: "Saturn in Leo (0°–10°)",
        keywords: "Conflict · competition · scattered energy · the test of will",
        divine_quality: "Nith-Haiah's prophetic wisdom and Haaiah's discernment amid strife",
    },
    MinorArcanum {
        suit: "Wands", rank_name: "Six",
        element: "Fire", sub_element: "",
        angel: "Yeratel", angel2: "Seheiah",
        decan: "Jupiter in Leo (10°–20°)",
        keywords: "Victory · public recognition · triumph · the procession of the conqueror",
        divine_quality: "Yeratel's light propagated, Seheiah's protection crowning success",
    },
    MinorArcanum {
        suit: "Wands", rank_name: "Seven",
        element: "Fire", sub_element: "",
        angel: "Reiyel", angel2: "Omael",
        decan: "Mars in Leo (20°–30°)",
        keywords: "Defiance · holding one's ground · courage under pressure · persistence",
        divine_quality: "Reiyel's liberation from enchantment fortified by Omael's patience",
    },
    MinorArcanum {
        suit: "Wands", rank_name: "Eight",
        element: "Fire", sub_element: "",
        angel: "Vehuel", angel2: "Daniel",
        decan: "Mercury in Sagittarius (0°–10°)",
        keywords: "Swift movement · messages in flight · momentum · approaching resolution",
        divine_quality: "Vehuel's elevation carrying Daniel's eloquent divine word swiftly",
    },
    MinorArcanum {
        suit: "Wands", rank_name: "Nine",
        element: "Fire", sub_element: "",
        angel: "Hahasiah", angel2: "Imamiah",
        decan: "Moon in Sagittarius (10°–20°)",
        keywords: "Resilience · defensive readiness · inner reserves · the last stand",
        divine_quality: "Hahasiah's hidden medicine and Imamiah's redemptive strength",
    },
    MinorArcanum {
        suit: "Wands", rank_name: "Ten",
        element: "Fire", sub_element: "",
        angel: "Nanael", angel2: "Nithael",
        decan: "Saturn in Sagittarius (20°–30°)",
        keywords: "Burdens · over-commitment · oppression · the weight of responsibility",
        divine_quality: "Nanael's contemplative counsel and Nithael's celestial order restore balance",
    },
    MinorArcanum {
        suit: "Wands", rank_name: "Page",
        element: "Fire", sub_element: "Earth of Fire",
        angel: "Uriel", angel2: "",
        decan: "",
        keywords: "Enthusiasm · new inspiration · creative potential · the eager messenger",
        divine_quality: "Uriel's earthly fire; the divine ember carried in eager hands",
    },
    MinorArcanum {
        suit: "Wands", rank_name: "Knight",
        element: "Fire", sub_element: "Air of Fire",
        angel: "Raphael", angel2: "",
        decan: "",
        keywords: "Impetuous action · passionate pursuit · adventure · dynamic movement",
        divine_quality: "Raphael's winged fire; the divine wind that fans the sacred flame",
    },
    MinorArcanum {
        suit: "Wands", rank_name: "Queen",
        element: "Fire", sub_element: "Water of Fire",
        angel: "Gabriel", angel2: "",
        decan: "",
        keywords: "Magnetic confidence · creative authority · warmth · intuitive power",
        divine_quality: "Gabriel's lunar fire; the divine intuition that directs creative force",
    },
    MinorArcanum {
        suit: "Wands", rank_name: "King",
        element: "Fire", sub_element: "Fire of Fire",
        angel: "Michael", angel2: "",
        decan: "",
        keywords: "Solar leadership · visionary mastery · entrepreneurial spirit · noble fire",
        divine_quality: "Michael's pure solar flame; divine leadership that illumines others",
    },

    // ════════════════════════════════════════════════════════════════════════
    //  CUPS — Element: Water
    //  Zodiac signs: Cancer (2–4) · Scorpio (5–7) · Pisces (8–10)
    // ════════════════════════════════════════════════════════════════════════

    MinorArcanum {
        suit: "Cups", rank_name: "Ace",
        element: "Water", sub_element: "",
        angel: "Gabriel", angel2: "",
        decan: "",
        keywords: "Emotional abundance · new love · spiritual gifts · the overflowing heart",
        divine_quality: "The pure undivided water of divine love; Gabriel's sacred chalice",
    },
    MinorArcanum {
        suit: "Cups", rank_name: "Two",
        element: "Water", sub_element: "",
        angel: "Leuviah", angel2: "Pahaliah",
        decan: "Venus in Cancer (0°–10°)",
        keywords: "Partnership · mutual attraction · sacred union · the pledge of love",
        divine_quality: "Leuviah's patient grace and Pahaliah's redemptive love sealed together",
    },
    MinorArcanum {
        suit: "Cups", rank_name: "Three",
        element: "Water", sub_element: "",
        angel: "Nelkhael", angel2: "Yeiayel",
        decan: "Mercury in Cancer (10°–20°)",
        keywords: "Celebration · friendship · community · the joy of shared abundance",
        divine_quality: "Nelkhael's liberation through learning with Yeiayel's fortunate fame",
    },
    MinorArcanum {
        suit: "Cups", rank_name: "Four",
        element: "Water", sub_element: "",
        angel: "Melahel", angel2: "Haheuiah",
        decan: "Moon in Cancer (20°–30°)",
        keywords: "Withdrawal · contemplation · emotional stagnation · the call to look inward",
        divine_quality: "Melahel's healing waters and Haheuiah's sheltering truth offer renewal",
    },
    MinorArcanum {
        suit: "Cups", rank_name: "Five",
        element: "Water", sub_element: "",
        angel: "Veuliah", angel2: "Yelahiah",
        decan: "Mars in Scorpio (0°–10°)",
        keywords: "Loss · grief · regret · the reminder that three cups remain upright",
        divine_quality: "Veuliah's liberation from poverty and Yelahiah's sacred courage in grief",
    },
    MinorArcanum {
        suit: "Cups", rank_name: "Six",
        element: "Water", sub_element: "",
        angel: "Sealiah", angel2: "Ariel",
        decan: "Sun in Scorpio (10°–20°)",
        keywords: "Nostalgia · innocence · past happiness · the gift of memory",
        divine_quality: "Sealiah's healing of the despairing and Ariel's subtle gift of gratitude",
    },
    MinorArcanum {
        suit: "Cups", rank_name: "Seven",
        element: "Water", sub_element: "",
        angel: "Asaliah", angel2: "Mihael",
        decan: "Venus in Scorpio (20°–30°)",
        keywords: "Illusion · wishful thinking · choices · the proliferation of desires",
        divine_quality: "Asaliah's contemplation of truth and Mihael's loving peace clarify the way",
    },
    MinorArcanum {
        suit: "Cups", rank_name: "Eight",
        element: "Water", sub_element: "",
        angel: "Eyael", angel2: "Habuhiah",
        decan: "Saturn in Pisces (0°–10°)",
        keywords: "Walking away · disillusionment · seeking deeper meaning · the sacred farewell",
        divine_quality: "Eyael's transcendent consolation and Habuhiah's healing abundance beckon",
    },
    MinorArcanum {
        suit: "Cups", rank_name: "Nine",
        element: "Water", sub_element: "",
        angel: "Rochel", angel2: "Jabamiah",
        decan: "Jupiter in Pisces (10°–20°)",
        keywords: "Contentment · wish fulfilment · emotional satisfaction · the good life",
        divine_quality: "Rochel's gift of restoration and Jabamiah's sacred alchemy of joy",
    },
    MinorArcanum {
        suit: "Cups", rank_name: "Ten",
        element: "Water", sub_element: "",
        angel: "Haiaiel", angel2: "Mumiah",
        decan: "Mars in Pisces (20°–30°)",
        keywords: "Bliss · family harmony · lasting happiness · the promise of love fulfilled",
        divine_quality: "Haiaiel's divine armament and Mumiah's sacred completion bless all",
    },
    MinorArcanum {
        suit: "Cups", rank_name: "Page",
        element: "Water", sub_element: "Earth of Water",
        angel: "Uriel", angel2: "",
        decan: "",
        keywords: "Intuitive messages · emotional openness · dreamy sensitivity · new feelings",
        divine_quality: "Uriel's earthly waters; the divine dream carried in innocent hands",
    },
    MinorArcanum {
        suit: "Cups", rank_name: "Knight",
        element: "Water", sub_element: "Air of Water",
        angel: "Raphael", angel2: "",
        decan: "",
        keywords: "Romantic pursuit · emotional intelligence · the arrival of opportunity",
        divine_quality: "Raphael's healing wind over still waters; the breath of sacred feeling",
    },
    MinorArcanum {
        suit: "Cups", rank_name: "Queen",
        element: "Water", sub_element: "Water of Water",
        angel: "Gabriel", angel2: "",
        decan: "",
        keywords: "Empathic mastery · nurturing wisdom · deep feeling · visionary heart",
        divine_quality: "Gabriel's purest lunar water; the divine empathy that heals all wounds",
    },
    MinorArcanum {
        suit: "Cups", rank_name: "King",
        element: "Water", sub_element: "Fire of Water",
        angel: "Michael", angel2: "",
        decan: "",
        keywords: "Emotional authority · compassionate leadership · diplomatic mastery",
        divine_quality: "Michael's solar warmth meeting water; divine compassion in command",
    },

    // ════════════════════════════════════════════════════════════════════════
    //  SWORDS — Element: Air
    //  Zodiac signs: Libra (2–4) · Aquarius (5–7) · Gemini (8–10)
    // ════════════════════════════════════════════════════════════════════════

    MinorArcanum {
        suit: "Swords", rank_name: "Ace",
        element: "Air", sub_element: "",
        angel: "Raphael", angel2: "",
        decan: "",
        keywords: "Mental clarity · truth · breakthrough · the sword of divine intellect",
        divine_quality: "The pure undivided air of divine mind; Raphael's sword of healing truth",
    },
    MinorArcanum {
        suit: "Swords", rank_name: "Two",
        element: "Air", sub_element: "",
        angel: "Aniel", angel2: "Haamiah",
        decan: "Moon in Libra (0°–10°)",
        keywords: "Stalemate · difficult decision · the blindfold of deliberate peace",
        divine_quality: "Aniel's cycle-breaking courage and Haamiah's ritual truth in balance",
    },
    MinorArcanum {
        suit: "Swords", rank_name: "Three",
        element: "Air", sub_element: "",
        angel: "Rehael", angel2: "Ieiazel",
        decan: "Saturn in Libra (10°–20°)",
        keywords: "Heartbreak · sorrow · grief · the purifying pain of loss",
        divine_quality: "Rehael's health-restoring mercy and Ieiazel's freedom from oppression",
    },
    MinorArcanum {
        suit: "Swords", rank_name: "Four",
        element: "Air", sub_element: "",
        angel: "Hahahel", angel2: "Mikael",
        decan: "Jupiter in Libra (20°–30°)",
        keywords: "Rest · recuperation · contemplation · the sacred truce after battle",
        divine_quality: "Hahahel's sacred mission resting, Mikael's divine order restoring peace",
    },
    MinorArcanum {
        suit: "Swords", rank_name: "Five",
        element: "Air", sub_element: "",
        angel: "Umabel", angel2: "Iah-Hel",
        decan: "Venus in Aquarius (0°–10°)",
        keywords: "Defeat · hollow victory · conflict's bitter aftermath · the lesson of pride",
        divine_quality: "Umabel's friendships fractured; Iah-Hel's wisdom silences the ego",
    },
    MinorArcanum {
        suit: "Swords", rank_name: "Six",
        element: "Air", sub_element: "",
        angel: "Anauel", angel2: "Mehiel",
        decan: "Mercury in Aquarius (10°–20°)",
        keywords: "Transition · passage · moving on · the healing journey toward calm",
        divine_quality: "Anauel's sacred commerce of souls and Mehiel's vivifying inspiration",
    },
    MinorArcanum {
        suit: "Swords", rank_name: "Seven",
        element: "Air", sub_element: "",
        angel: "Damabiah", angel2: "Manakel",
        decan: "Moon in Aquarius (20°–30°)",
        keywords: "Deception · strategy · theft of energy · the wisdom of tactical retreat",
        divine_quality: "Damabiah's deep-water wisdom and Manakel's dream-peace dissolve illusion",
    },
    MinorArcanum {
        suit: "Swords", rank_name: "Eight",
        element: "Air", sub_element: "",
        angel: "Yezalel", angel2: "Mebahel",
        decan: "Jupiter in Gemini (0°–10°)",
        keywords: "Self-imprisonment · limiting beliefs · the bondage of thought · victim patterns",
        divine_quality: "Yezalel's faithful memory and Mebahel's truth cut the mental bonds free",
    },
    MinorArcanum {
        suit: "Swords", rank_name: "Nine",
        element: "Air", sub_element: "",
        angel: "Hariel", angel2: "Hakamiah",
        decan: "Mars in Gemini (10°–20°)",
        keywords: "Nightmare · anxiety · despair · the darkest hour before dawn",
        divine_quality: "Hariel's purifying science and Hakamiah's loyal victory dispel the night",
    },
    MinorArcanum {
        suit: "Swords", rank_name: "Ten",
        element: "Air", sub_element: "",
        angel: "Lauvuel", angel2: "Caliel",
        decan: "Sun in Gemini (20°–30°)",
        keywords: "Rock bottom · painful ending · the finality that permits a new start",
        divine_quality: "Lauvuel's illumined glory and Caliel's divine justice clear the field",
    },
    MinorArcanum {
        suit: "Swords", rank_name: "Page",
        element: "Air", sub_element: "Earth of Air",
        angel: "Uriel", angel2: "",
        decan: "",
        keywords: "Curious intellect · vigilance · mental agility · the watchful messenger",
        divine_quality: "Uriel's grounded air; the divine observation carried in alert hands",
    },
    MinorArcanum {
        suit: "Swords", rank_name: "Knight",
        element: "Air", sub_element: "Air of Air",
        angel: "Raphael", angel2: "",
        decan: "",
        keywords: "Swift intellect · decisive action · charging forward · the storm of thought",
        divine_quality: "Raphael's pure wind; the divine mind at full gallop, cutting to truth",
    },
    MinorArcanum {
        suit: "Swords", rank_name: "Queen",
        element: "Air", sub_element: "Water of Air",
        angel: "Gabriel", angel2: "",
        decan: "",
        keywords: "Clear-minded authority · intellectual independence · sharp discernment",
        divine_quality: "Gabriel's lunar clarity in the realm of mind; divine truth without sentiment",
    },
    MinorArcanum {
        suit: "Swords", rank_name: "King",
        element: "Air", sub_element: "Fire of Air",
        angel: "Michael", angel2: "",
        decan: "",
        keywords: "Intellectual mastery · just authority · analytical command · the arbiter",
        divine_quality: "Michael's solar intellect; divine justice administered with clarity and fire",
    },

    // ════════════════════════════════════════════════════════════════════════
    //  PENTACLES — Element: Earth
    //  Zodiac signs: Capricorn (2–4) · Taurus (5–7) · Virgo (8–10)
    // ════════════════════════════════════════════════════════════════════════

    MinorArcanum {
        suit: "Pentacles", rank_name: "Ace",
        element: "Earth", sub_element: "",
        angel: "Uriel", angel2: "",
        decan: "",
        keywords: "Material gift · new prosperity · physical abundance · the sacred seed",
        divine_quality: "The pure undivided earth of divine manifestation; Uriel's golden coin",
    },
    MinorArcanum {
        suit: "Pentacles", rank_name: "Two",
        element: "Earth", sub_element: "",
        angel: "Mebahiah", angel2: "Poyel",
        decan: "Jupiter in Capricorn (0°–10°)",
        keywords: "Juggling resources · adaptability · balance in flux · playful management",
        divine_quality: "Mebahiah's lucidity and Poyel's fortunate support keep the dance aloft",
    },
    MinorArcanum {
        suit: "Pentacles", rank_name: "Three",
        element: "Earth", sub_element: "",
        angel: "Nemamiah", angel2: "Yeialel",
        decan: "Mars in Capricorn (10°–20°)",
        keywords: "Teamwork · skilled craft · collaboration · mastery built from study",
        divine_quality: "Nemamiah's discernment and Yeialel's mental fortitude forge excellent work",
    },
    MinorArcanum {
        suit: "Pentacles", rank_name: "Four",
        element: "Earth", sub_element: "",
        angel: "Harahel", angel2: "Mitzrael",
        decan: "Sun in Capricorn (20°–30°)",
        keywords: "Possessiveness · security · miserliness · the fear beneath material holding",
        divine_quality: "Harahel's intellectual wealth and Mitzrael's inner repair open the fist",
    },
    MinorArcanum {
        suit: "Pentacles", rank_name: "Five",
        element: "Earth", sub_element: "",
        angel: "Achaiah", angel2: "Cahetel",
        decan: "Mercury in Taurus (0°–10°)",
        keywords: "Hardship · poverty · exclusion · the grace found in desperate need",
        divine_quality: "Achaiah's patient discovery and Cahetel's divine blessing sustain the lost",
    },
    MinorArcanum {
        suit: "Pentacles", rank_name: "Six",
        element: "Earth", sub_element: "",
        angel: "Haziel", angel2: "Aladiah",
        decan: "Moon in Taurus (10°–20°)",
        keywords: "Generosity · charity · the balanced giving and receiving of material grace",
        divine_quality: "Haziel's divine mercy and Aladiah's healing favour flow freely to all",
    },
    MinorArcanum {
        suit: "Pentacles", rank_name: "Seven",
        element: "Earth", sub_element: "",
        angel: "Lauviah", angel2: "Hahaiah",
        decan: "Saturn in Taurus (20°–30°)",
        keywords: "Patience · long-term investment · assessment · the harvest not yet ripe",
        divine_quality: "Lauviah's victorious vision and Hahaiah's sheltering refuge sustain waiting",
    },
    MinorArcanum {
        suit: "Pentacles", rank_name: "Eight",
        element: "Earth", sub_element: "",
        angel: "Lecabel", angel2: "Vasariah",
        decan: "Sun in Virgo (0°–10°)",
        keywords: "Diligence · craftsmanship · apprenticeship · the mastery of humble practice",
        divine_quality: "Lecabel's sacred craft and Vasariah's divine clemency perfect the work",
    },
    MinorArcanum {
        suit: "Pentacles", rank_name: "Nine",
        element: "Earth", sub_element: "",
        angel: "Yehuiah", angel2: "Lehahiah",
        decan: "Venus in Virgo (10°–20°)",
        keywords: "Solitary abundance · self-sufficiency · refinement · earned luxury",
        divine_quality: "Yehuiah's protective service and Lehahiah's disciplined peace bear fruit",
    },
    MinorArcanum {
        suit: "Pentacles", rank_name: "Ten",
        element: "Earth", sub_element: "",
        angel: "Chavakiah", angel2: "Menadel",
        decan: "Mercury in Virgo (20°–30°)",
        keywords: "Legacy · family wealth · lasting prosperity · the fullness of the generations",
        divine_quality: "Chavakiah's reconciling harmony and Menadel's liberating work complete the cycle",
    },
    MinorArcanum {
        suit: "Pentacles", rank_name: "Page",
        element: "Earth", sub_element: "Earth of Earth",
        angel: "Uriel", angel2: "",
        decan: "",
        keywords: "Studious ambition · practical curiosity · grounded new beginnings · the scholar",
        divine_quality: "Uriel's purest earth; the divine seed of material wisdom in young hands",
    },
    MinorArcanum {
        suit: "Pentacles", rank_name: "Knight",
        element: "Earth", sub_element: "Air of Earth",
        angel: "Raphael", angel2: "",
        decan: "",
        keywords: "Methodical progress · reliable advance · deliberate effort · the steady worker",
        divine_quality: "Raphael's healing breath over fertile earth; the divine mind made tangible",
    },
    MinorArcanum {
        suit: "Pentacles", rank_name: "Queen",
        element: "Earth", sub_element: "Water of Earth",
        angel: "Gabriel", angel2: "",
        decan: "",
        keywords: "Practical nurturing · material generosity · the abundant provider",
        divine_quality: "Gabriel's lunar fertility in earth; the divine mother whose touch enriches",
    },
    MinorArcanum {
        suit: "Pentacles", rank_name: "King",
        element: "Earth", sub_element: "Fire of Earth",
        angel: "Michael", angel2: "",
        decan: "",
        keywords: "Material mastery · reliable abundance · the entrepreneur · secure prosperity",
        divine_quality: "Michael's solar vitality in earth; divine stewardship of created wealth",
    },
];

// ─── Lookup helpers ───────────────────────────────────────────────────────────

/// Return all Minor Arcana cards for one suit (case-insensitive).
pub fn suit_cards(suit: &str) -> Vec<&'static MinorArcanum> {
    let s = suit.to_lowercase();
    MINOR_ARCANA
        .iter()
        .filter(|c| c.suit.to_lowercase() == s)
        .collect()
}

/// Find a Shem HaMephorash angel by number (1–72).
pub fn shem_by_number(n: u8) -> Option<&'static ShemAngel> {
    SHEM_HAMEPHORASH.iter().find(|a| a.number == n)
}

/// Find a Minor Arcanum by suit and rank name (case-insensitive, exact rank match).
pub fn minor_by_suit_rank(suit: &str, rank: &str) -> Option<&'static MinorArcanum> {
    let s = suit.to_lowercase();
    let r = rank.to_lowercase();
    MINOR_ARCANA
        .iter()
        .find(|c| c.suit.to_lowercase() == s && c.rank_name.to_lowercase() == r)
}
