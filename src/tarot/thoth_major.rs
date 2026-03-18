//! Thoth Tarot — 22 Major Arcana
//!
//! Data sources:
//!   Crowley, A. (1944). *The Book of Thoth*. O.T.O.
//!   DuQuette, L.M. (2003). *Understanding Aleister Crowley's Thoth Tarot*. Weiser Books.
//!   Crowley, A. (1909). *Liber 777*. Walter Scott Publishing.
//!
//! Notable Crowley divergences from RWS tradition:
//!   • Strength → "Lust" (XI); Justice → "Adjustment" (VIII)
//!   • Temperance → "Art" (XIV); Judgement → "The Aeon" (XX); World → "The Universe" (XXI)
//!   • Tzaddi / He swap: The Emperor bears Tzaddi (path 28); The Star bears He (path 15).
//!     Crowley announced this in *The Book of the Law* I:57 ("All these old letters of my
//!     Book are aright; but Tzaddi is not the Star").

// ─── Struct ──────────────────────────────────────────────────────────────────

/// One Major Arcanum of Crowley's Thoth Tarot deck, painted by Lady Frieda Harris.
#[derive(Debug)]
pub struct ThothMajor {
    /// Card number 0–21.
    pub number: u8,
    /// Thoth Tarot name as given in *The Book of Thoth*.
    pub name: &'static str,
    /// Equivalent Rider-Waite-Smith card name.
    pub rws_name: &'static str,
    /// Hebrew letter: Unicode glyph · transliteration · traditional meaning.
    pub hebrew_letter: &'static str,
    /// Tree of Life path number (11–32).
    pub path: u8,
    /// Sephiroth connected by this path.
    pub sephiroth: &'static str,
    /// Planetary, zodiacal, or elemental attribution.
    pub astrology: &'static str,
    /// Role or title within Thelemic cosmology and *The Book of Thoth*.
    pub thelemic_title: &'static str,
    /// Key visual symbols Lady Frieda Harris incorporated.
    pub harris_symbolism: &'static str,
    /// Divinatory meaning, upright.
    pub meaning_upright: &'static str,
    /// Divinatory meaning, reversed or ill-dignified.
    pub meaning_reversed: &'static str,
}

// ─── Card data ───────────────────────────────────────────────────────────────

pub static THOTH_MAJOR: [ThothMajor; 22] = [
    // ── 0 · The Fool ─────────────────────────────────────────────────────────
    ThothMajor {
        number: 0,
        name: "The Fool",
        rws_name: "The Fool",
        hebrew_letter: "א · Aleph · Ox",
        path: 11,
        sephiroth: "Kether → Chokmah",
        astrology: "Air",
        thelemic_title: "The Spirit of Aethyr; pure Hadit potential before manifestation; zero = infinite",
        harris_symbolism: "Leaping figure in motley, white sun, crocodile below, horned god mask, grapes and tiger",
        meaning_upright: "Absolute beginnings, divine folly, spontaneous trust in the unknown; pure potential leaping into being",
        meaning_reversed: "Recklessness, mania, poor judgment, inability to ground inspired impulse in practical reality",
    },
    // ── 1 · The Magus ────────────────────────────────────────────────────────
    ThothMajor {
        number: 1,
        name: "The Magus",
        rws_name: "The Magician",
        hebrew_letter: "ב · Beth · House",
        path: 12,
        sephiroth: "Kether → Binah",
        astrology: "Mercury",
        thelemic_title: "Logos; the Word of creation; Thoth-Hermes; the transmitter of divine will into form",
        harris_symbolism: "Mercury figure juggling symbols of the four elements, caduceus, dove and serpent intertwined",
        meaning_upright: "Skilled will, communication, magical mastery; the ability to translate divine inspiration into action",
        meaning_reversed: "Trickery, cunning without wisdom, will misdirected; the Magus becomes the Juggler or con-man",
    },
    // ── 2 · The Priestess ────────────────────────────────────────────────────
    ThothMajor {
        number: 2,
        name: "The Priestess",
        rws_name: "The High Priestess",
        hebrew_letter: "ג · Gimel · Camel",
        path: 13,
        sephiroth: "Kether → Tiphareth",
        astrology: "Moon",
        thelemic_title: "Initiated Womb of the Universe; guardian of the Veil of the Abyss; Babalon above the Abyss",
        harris_symbolism: "Seated goddess behind gossamer veil, bow and arrows, crystal sphere, pillar of light and dark",
        meaning_upright: "Hidden wisdom, intuition, the unconscious; mysteries that yield only to patient inner work",
        meaning_reversed: "Secrets withheld harmfully, surface knowledge mistaken for depth, emotional repression",
    },
    // ── 3 · The Empress ──────────────────────────────────────────────────────
    ThothMajor {
        number: 3,
        name: "The Empress",
        rws_name: "The Empress",
        hebrew_letter: "ד · Daleth · Door",
        path: 14,
        sephiroth: "Chokmah → Binah",
        astrology: "Venus",
        thelemic_title: "Great Mother; Isis-Aphrodite; Nature in her generative and sexual aspect; Nuit as manifest love",
        harris_symbolism: "Crowned queen surrounded by lotus blossoms, girdle of zodiac, pelican feeding young, moon at feet",
        meaning_upright: "Fertility, sensual abundance, creative power, natural beauty; love flowing freely into manifestation",
        meaning_reversed: "Sterility, smothering, creative block, vanity; love turned possessive or dissipated",
    },
    // ── 4 · The Emperor ──────────────────────────────────────────────────────
    ThothMajor {
        number: 4,
        name: "The Emperor",
        rws_name: "The Emperor",
        hebrew_letter: "צ · Tzaddi · Fish-hook",
        path: 28,
        sephiroth: "Netzach → Yesod",
        astrology: "Aries",
        thelemic_title: "Constituted Authority; Sun in Aries; Crowley assigns Tzaddi here per Liber AL I:57 swap",
        harris_symbolism: "Armored ruler on ram-headed throne, orb and scepter, eagles on shield, lamb at feet",
        meaning_upright: "Order, authority, rational structure, leadership; the will to establish and defend civilization",
        meaning_reversed: "Tyranny, rigidity, domination without wisdom; force without flexibility collapses",
    },
    // ── 5 · The Hierophant ───────────────────────────────────────────────────
    ThothMajor {
        number: 5,
        name: "The Hierophant",
        rws_name: "The Hierophant",
        hebrew_letter: "ו · Vav · Nail",
        path: 16,
        sephiroth: "Chokmah → Chesed",
        astrology: "Taurus",
        thelemic_title: "Magus of the Eternal; Osiris enthroned; the revealed mysteries; outer court of initiation",
        harris_symbolism: "Enthroned pontiff with five-pointed crown, two initiates kneeling, pentagram overlapping cards",
        meaning_upright: "Tradition, spiritual teaching, initiation, institutional wisdom; the outer form of inner mystery",
        meaning_reversed: "Dogma, spiritual tyranny, blind adherence to tradition; the letter kills the spirit",
    },
    // ── 6 · The Lovers ───────────────────────────────────────────────────────
    ThothMajor {
        number: 6,
        name: "The Lovers",
        rws_name: "The Lovers",
        hebrew_letter: "ז · Zayin · Sword",
        path: 17,
        sephiroth: "Binah → Tiphareth",
        astrology: "Gemini",
        thelemic_title: "Children of the Voice; the oracle of the Mighty Gods; Cain and Abel as dual will; Royal Marriage",
        harris_symbolism: "Hermit presiding over royal marriage of King and Queen, figures of the four elements, Orphic egg",
        meaning_upright: "Union of opposites, conscious choice, sacred partnership, alchemical marriage of self",
        meaning_reversed: "Indecision, inner conflict, false union, choosing from fear rather than love",
    },
    // ── 7 · The Chariot ──────────────────────────────────────────────────────
    ThothMajor {
        number: 7,
        name: "The Chariot",
        rws_name: "The Chariot",
        hebrew_letter: "ח · Cheth · Fence",
        path: 18,
        sephiroth: "Binah → Geburah",
        astrology: "Cancer",
        thelemic_title: "Child of the Powers of the Waters; Holy Grail borne by the victorious Warrior; Graal knight",
        harris_symbolism: "Armored warrior in starry canopy chariot, Holy Grail at chest, four sphinxes drawing vehicle",
        meaning_upright: "Victory through will, disciplined mastery of forces; the Grail knight conquers by surrender",
        meaning_reversed: "Inner conflict destroying momentum, scattered will, domination by instincts or circumstance",
    },
    // ── 8 · Adjustment ───────────────────────────────────────────────────────
    ThothMajor {
        number: 8,
        name: "Adjustment",
        rws_name: "Justice",
        hebrew_letter: "ל · Lamed · Ox-goad",
        path: 22,
        sephiroth: "Geburah → Tiphareth",
        astrology: "Libra",
        thelemic_title: "Daughter of the Lords of Truth; cosmic equilibrium; complement to The Fool on the Tree",
        harris_symbolism: "Masked goddess en pointe, double sword and scales balanced on a pin, diamond lozenge body",
        meaning_upright: "Perfect equilibrium, karma, cause and effect in precise operation; truth as structural balance",
        meaning_reversed: "Injustice, bias, karmic imbalance, refusal to acknowledge consequences of one's actions",
    },
    // ── 9 · The Hermit ───────────────────────────────────────────────────────
    ThothMajor {
        number: 9,
        name: "The Hermit",
        rws_name: "The Hermit",
        hebrew_letter: "י · Yod · Hand",
        path: 20,
        sephiroth: "Chesed → Tiphareth",
        astrology: "Virgo",
        thelemic_title: "Magus of the Voice of Power; Hermes as the Aged One; the divine sperm of cosmic generation",
        harris_symbolism: "Cloaked elder bearing lantern of sun, three-headed Cerberus, staff entwined with serpent, wheat",
        meaning_upright: "Solitary inner wisdom, withdrawal to find light; the guide who illumines path for others",
        meaning_reversed: "Isolation as avoidance, false humility, wisdom hoarded, refusal to emerge from introversion",
    },
    // ── 10 · Fortune ─────────────────────────────────────────────────────────
    ThothMajor {
        number: 10,
        name: "Fortune",
        rws_name: "Wheel of Fortune",
        hebrew_letter: "כ · Kaph · Palm of Hand",
        path: 21,
        sephiroth: "Chesed → Netzach",
        astrology: "Jupiter",
        thelemic_title: "Lord of the Forces of Life; three Gunas spinning; Sphinx, Hermanubis, and Typhon on the Wheel",
        harris_symbolism: "Ten-spoked wheel with three figures: Sphinx atop, Hermanubis rising, Typhon descending; Tao hub",
        meaning_upright: "Inevitable change, cycles of fate, expansion; embrace the turning Wheel rather than resist it",
        meaning_reversed: "Clinging to the past, bad luck from resistance to change, victimhood under fortune's wheel",
    },
    // ── 11 · Lust ────────────────────────────────────────────────────────────
    ThothMajor {
        number: 11,
        name: "Lust",
        rws_name: "Strength",
        hebrew_letter: "ט · Teth · Serpent",
        path: 19,
        sephiroth: "Chesed → Geburah",
        astrology: "Leo",
        thelemic_title: "Daughter of the Flaming Sword; Babalon riding the Beast 666; Kundalini force consciously ridden",
        harris_symbolism: "Scarlet Woman riding seven-headed lion-serpent Beast, Holy Grail held aloft, ten horns on Beast",
        meaning_upright: "Raw power channeled through joy; ecstatic strength; the courage to love the wild within",
        meaning_reversed: "Force misdirected by lust for power, excess, fanaticism, or repression of vital instinct",
    },
    // ── 12 · The Hanged Man ──────────────────────────────────────────────────
    ThothMajor {
        number: 12,
        name: "The Hanged Man",
        rws_name: "The Hanged Man",
        hebrew_letter: "מ · Mem · Water",
        path: 23,
        sephiroth: "Geburah → Hod",
        astrology: "Water / Neptune",
        thelemic_title: "Spirit of the Mighty Waters; Osiris slain and suspended; Odin on Yggdrasil; sacrificed redeemer",
        harris_symbolism: "Figure hanging from ankh cross, spiral serpent of light, legs crossed in tau, serene face",
        meaning_upright: "Voluntary surrender, reversal of perspective revealing hidden truth; sacrifice that brings gnosis",
        meaning_reversed: "Futile sacrifice, martyrdom without purpose, unwillingness to release what must be released",
    },
    // ── 13 · Death ───────────────────────────────────────────────────────────
    ThothMajor {
        number: 13,
        name: "Death",
        rws_name: "Death",
        hebrew_letter: "נ · Nun · Fish",
        path: 24,
        sephiroth: "Tiphareth → Netzach",
        astrology: "Scorpio",
        thelemic_title: "Child of the Great Transformers; scorpion, serpent, eagle, and scorpion's sting; Osiris reborn",
        harris_symbolism: "Skeleton warrior wielding scythe, bubbling life-forms emerging from black earth, serpent rising",
        meaning_upright: "Transformation through ending; the clearing away that allows new life; death as rebirth vector",
        meaning_reversed: "Stagnation, fear of necessary change, clinging to the dead, refusing transformation",
    },
    // ── 14 · Art ─────────────────────────────────────────────────────────────
    ThothMajor {
        number: 14,
        name: "Art",
        rws_name: "Temperance",
        hebrew_letter: "ס · Samekh · Prop",
        path: 25,
        sephiroth: "Tiphareth → Yesod",
        astrology: "Sagittarius",
        thelemic_title: "Daughter of the Reconcilers; alchemical Great Work; Solve et Coagula; the quintessence sought",
        harris_symbolism: "Two-headed androgyne pouring fire and water between cups, raven and skull, green and red lion",
        meaning_upright: "Integration of opposites, alchemy, artistic vision perfecting raw material into the gold of self",
        meaning_reversed: "Imbalance, failed synthesis, creativity without discipline, poisonous mixing of incompatibles",
    },
    // ── 15 · The Devil ───────────────────────────────────────────────────────
    ThothMajor {
        number: 15,
        name: "The Devil",
        rws_name: "The Devil",
        hebrew_letter: "ע · Ayin · Eye",
        path: 26,
        sephiroth: "Tiphareth → Hod",
        astrology: "Capricorn",
        thelemic_title: "Lord of the Gates of Matter; Pan; creative energy as matter's blind generative drive; Set-Typhon",
        harris_symbolism: "Baphomet figure as Pan, phallic wand, goat horns, third eye, creative spiral energy, caduceus",
        meaning_upright: "Primal creative force, material power, ecstatic union with matter; Pan as cosmic laughter",
        meaning_reversed: "Bondage to appetite, materialism, addiction, obsession; the eye that sees only surfaces",
    },
    // ── 16 · The Tower ───────────────────────────────────────────────────────
    ThothMajor {
        number: 16,
        name: "The Tower",
        rws_name: "The Tower",
        hebrew_letter: "פ · Peh · Mouth",
        path: 27,
        sephiroth: "Netzach → Hod",
        astrology: "Mars",
        thelemic_title: "Lord of the Hosts of the Mighty; the War-engine of Mars; Liber AL's Hadit as the lightning flash",
        harris_symbolism: "Tower struck by divine mouth-lightning, figures falling, eye of Shiva, dove and serpent expelled",
        meaning_upright: "Sudden revelation shattering false structures; liberating destruction; the ego's necessary ruin",
        meaning_reversed: "Catastrophe without enlightenment, violence, oppression; walls rebuilt faster than truth enters",
    },
    // ── 17 · The Star ────────────────────────────────────────────────────────
    ThothMajor {
        number: 17,
        name: "The Star",
        rws_name: "The Star",
        hebrew_letter: "ה · He · Window",
        path: 15,
        sephiroth: "Chokmah → Tiphareth",
        astrology: "Aquarius",
        thelemic_title: "Daughter of the Firmament; Nuit as star-goddess; Nuith pouring infinite space as nourishment",
        harris_symbolism: "Nude goddess kneeling by water, seven-pointed star of Babalon, fluid poured from two cups to earth",
        meaning_upright: "Cosmic hope, Nuit's infinite gift, inspiration pouring from the stars; naked truth revealed",
        meaning_reversed: "Disillusionment, false hope, disconnection from the stars; guidance obscured by doubt",
    },
    // ── 18 · The Moon ────────────────────────────────────────────────────────
    ThothMajor {
        number: 18,
        name: "The Moon",
        rws_name: "The Moon",
        hebrew_letter: "ק · Qoph · Back of Head",
        path: 29,
        sephiroth: "Netzach → Malkuth",
        astrology: "Pisces",
        thelemic_title: "Ruler of Flux and Reflux; Qoph as the subconscious mind; path of the Qliphoth emerging",
        harris_symbolism: "Scarab beetle, twin towers, jackal and wolf howling, crayfish in pool, dark and light pillars",
        meaning_upright: "The unconscious surfacing, illusion, psychic flux; navigate deep waters without ego's lantern",
        meaning_reversed: "Delusion, psychic confusion, hidden enemies, neurosis, the dark night refusing to pass",
    },
    // ── 19 · The Sun ─────────────────────────────────────────────────────────
    ThothMajor {
        number: 19,
        name: "The Sun",
        rws_name: "The Sun",
        hebrew_letter: "ר · Resh · Head",
        path: 30,
        sephiroth: "Hod → Yesod",
        astrology: "Sun",
        thelemic_title: "Lord of the Fire of the World; Ra-Hoor-Khuit triumphant; solar consciousness restored to Horus",
        harris_symbolism: "Twin children dancing before golden sun, butterfly wings, twelve rays of zodiac, rose garland",
        meaning_upright: "Vitality, illumination, conscious success; the Thelemic sun of Ra-Hoor-Khuit fully risen",
        meaning_reversed: "Arrogance, superficiality, blocked vitality; the solar ego eclipsing its own deeper light",
    },
    // ── 20 · The Aeon ────────────────────────────────────────────────────────
    ThothMajor {
        number: 20,
        name: "The Aeon",
        rws_name: "Judgement",
        hebrew_letter: "ש · Shin · Tooth",
        path: 31,
        sephiroth: "Hod → Malkuth",
        astrology: "Fire / Pluto",
        thelemic_title: "Spirit of the Primal Fire; Aeon of Horus begun 1904; Nuit, Hadit, Ra-Hoor-Khuit in cosmic triad",
        harris_symbolism: "Nuit arched above, Hadit as winged globe, Ra-Hoor-Khuit enthroned, Harpocrates in oval below",
        meaning_upright: "New Aeon dawning, cosmic judgment based in love and will; awakening to True Will's full scope",
        meaning_reversed: "Old-Aeon thinking, condemnation without understanding, failure to hear the cosmic call",
    },
    // ── 21 · The Universe ────────────────────────────────────────────────────
    ThothMajor {
        number: 21,
        name: "The Universe",
        rws_name: "The World",
        hebrew_letter: "ת · Tau · Cross / Mark",
        path: 32,
        sephiroth: "Yesod → Malkuth",
        astrology: "Saturn / Earth",
        thelemic_title: "Great One of the Night of Time; Malkuth completed; the 32nd path as final gate of incarnation",
        harris_symbolism: "Dancing nude goddess in oval of seventy-two circles, four kerubic figures, serene cosmic dancer",
        meaning_upright: "Completion, cosmic attainment, the Great Work accomplished; matter as sacred as spirit",
        meaning_reversed: "Incompletion, fear of finality, inability to ground vision in earthly reality; stagnation",
    },
];

// ─── Lookup functions ─────────────────────────────────────────────────────────

/// Return a Major Arcanum by its number (0–21).
pub fn thoth_major_by_number(n: u8) -> Option<&'static ThothMajor> {
    THOTH_MAJOR.iter().find(|c| c.number == n)
}

/// Return a Major Arcanum by name — case-insensitive partial match on either
/// `name` (Thoth) or `rws_name` (RWS equivalent).
pub fn thoth_major_by_name(q: &str) -> Option<&'static ThothMajor> {
    let q_lower = q.to_lowercase();
    THOTH_MAJOR
        .iter()
        .find(|c| {
            c.name.to_lowercase().contains(&q_lower)
                || c.rws_name.to_lowercase().contains(&q_lower)
        })
}
