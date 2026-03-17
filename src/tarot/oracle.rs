//! Angelic Oracle Deck — 44 cards of divine guidance.
//!
//! ## What is an Oracle deck?
//!
//! Unlike Tarot (which has a fixed 78-card structure governed by Kabbalistic
//! and Hermetic frameworks) or Lenormand (which has a fixed 36-card
//! combinatorial folk system), an **Oracle deck** is a free-form cartomantic
//! system whose card count, imagery, and meaning framework are entirely at the
//! discretion of its creator.  There is no canonical "Oracle deck" — every
//! designer builds their own structure.
//!
//! This implementation presents a **44-card Angelic Oracle** grounded in the
//! Western angelic hierarchy and the classical divine attributes (*Sefirot*,
//! elemental forces, planetary intelligences).  The number 44 is chosen as a
//! master number in numerology (4×11, or 2×22) representing the Spiritual
//! Builder who bridges earthly form and divine pattern.
//!
//! ## Reading style
//!
//! Oracle cards are read individually rather than combinatorially.  Each card
//! is a self-contained teaching, an angelic message, or a divine quality for
//! the querent to embody.  Spreads of 1, 3, or 5 are most common; the card's
//! meaning is complete in itself.
//!
//! ## Structure of this deck
//!
//! The 44 cards are organised into four **suits of eleven** (the master number
//! 11 as a gateway):
//!
//! | Suit            | Domain                    | Elemental angel |
//! |-----------------|---------------------------|-----------------|
//! | Seraphic Fire   | Divine will, inspiration  | Michael         |
//! | Celestial Water | Divine love, healing       | Gabriel         |
//! | Sacred Air      | Divine wisdom, truth       | Raphael         |
//! | Holy Earth      | Divine order, manifestation| Uriel           |
//!
//! ## Sources
//!
//! - **Pseudo-Dionysius**, *The Celestial Hierarchy* (c. 5th c. CE) — nine
//!   angelic orders and their divine functions.
//! - **Agrippa**, *Three Books of Occult Philosophy* (1531) — angelic names,
//!   divine attributes, and elemental correspondences.
//! - **Swedenborg**, *Heaven and Hell* (1758; Swedenborg Foundation 2000) —
//!   the quality of angelic spheres and their divine correspondence.
//! - **Steiner**, *The Spiritual Hierarchies* (1909; Anthroposophic Press 1996) —
//!   the cosmic tasks of the nine hierarchies.
//! - **Virtue**, Doreen, *Healing with the Angels* (1999, Hay House) —
//!   popular modern angelic framework cross-referenced for completeness.

// ─── Data type ────────────────────────────────────────────────────────────────

/// One card of the 44-card Angelic Oracle deck.
pub struct OracleCard {
    /// Card number (1–44).
    pub number: u8,
    /// Suit name: "Seraphic Fire", "Celestial Water", "Sacred Air", "Holy Earth".
    pub suit: &'static str,
    /// Card title — the angelic quality or divine teaching named on this card.
    pub title: &'static str,
    /// The angel or angelic order most closely associated with this card.
    pub angel: &'static str,
    /// The primary divine quality or cosmic function.
    pub divine_quality: &'static str,
    /// Guidance message — what the card asks of the querent.
    pub guidance: &'static str,
    /// Affirmation — a first-person statement to embody the card's energy.
    pub affirmation: &'static str,
}

// ─── The forty-four cards ─────────────────────────────────────────────────────

pub static ORACLE: &[OracleCard] = &[

    // ══════════════════════════════════════════════════════════
    //  SERAPHIC FIRE (1–11)
    //  Element: Fire · Archangel: Michael · Domain: Divine Will
    // ══════════════════════════════════════════════════════════

    OracleCard {
        number: 1, suit: "Seraphic Fire",
        title: "Sacred Flame",
        angel: "Michael",
        divine_quality: "The original creative fire; the first thought of God made energy",
        guidance: "Ignite your sacred purpose. You carry a divine spark no circumstance can extinguish.",
        affirmation: "I am a vessel of the divine flame. My will aligns with sacred purpose.",
    },
    OracleCard {
        number: 2, suit: "Seraphic Fire",
        title: "Courage of the Archangel",
        angel: "Michael",
        divine_quality: "The fearless protector; the sword of truth that guards the light",
        guidance: "Stand firm. Michael stands with you; what must be confronted is already overcome.",
        affirmation: "I face what must be faced. The light protects me and I protect the light.",
    },
    OracleCard {
        number: 3, suit: "Seraphic Fire",
        title: "The Purifying Fire",
        angel: "Uriel",
        divine_quality: "The fire that refines without destroying; sacred alchemical transformation",
        guidance: "Allow the fire to work. What is being burned away is making room for pure gold.",
        affirmation: "I welcome transformation. What I release makes space for what I truly am.",
    },
    OracleCard {
        number: 4, suit: "Seraphic Fire",
        title: "Divine Mission",
        angel: "Chamuel",
        divine_quality: "The sacred task given before birth; the soul's unique contribution to creation",
        guidance: "Your purpose is not accidental. Follow what sets your soul alight.",
        affirmation: "I am here for a reason. I fulfil my sacred mission with joy.",
    },
    OracleCard {
        number: 5, suit: "Seraphic Fire",
        title: "The Holy Warrior",
        angel: "Michael",
        divine_quality: "Divine strength mobilised in defence of truth and the defenceless",
        guidance: "Fight only what truly needs fighting. Discriminate between fear and genuine call to action.",
        affirmation: "I wield my strength in service of truth and love.",
    },
    OracleCard {
        number: 6, suit: "Seraphic Fire",
        title: "Inspired Vision",
        angel: "Raziel",
        divine_quality: "The divine imagination; creative fire that builds in the mind before the hand",
        guidance: "Your vision is the seed of what will be. Tend it as sacred.",
        affirmation: "My imagination is a divine faculty. I see what can be, and I help it become real.",
    },
    OracleCard {
        number: 7, suit: "Seraphic Fire",
        title: "Solar Glory",
        angel: "Michael",
        divine_quality: "The life-giving radiance that nourishes all without partiality",
        guidance: "Let your light shine without apology. The sun does not dim itself for comfort.",
        affirmation: "I shine. My light is a gift to everyone who meets it.",
    },
    OracleCard {
        number: 8, suit: "Seraphic Fire",
        title: "The Phoenix",
        angel: "Azrael",
        divine_quality: "Sacred death and rebirth; the fires of ending that birth a new self",
        guidance: "You are rising. The ashes beneath you are not your ending — they are your foundation.",
        affirmation: "From every ending I rise renewed, carrying wisdom rather than wounds.",
    },
    OracleCard {
        number: 9, suit: "Seraphic Fire",
        title: "Zeal",
        angel: "Zadkiel",
        divine_quality: "Sacred enthusiasm; the divine energy that flows when the soul is aligned",
        guidance: "Follow what excites you without guilt. Zeal is the soul's compass.",
        affirmation: "My enthusiasm is sacred. I follow what lights me up.",
    },
    OracleCard {
        number: 10, suit: "Seraphic Fire",
        title: "Divine Protection",
        angel: "Michael",
        divine_quality: "The cosmic shield; the angelic guard placed around what is sacred",
        guidance: "You are protected. Release the vigilance that is actually fear, and trust the shield.",
        affirmation: "I am surrounded by divine light. Only love may enter; all else departs.",
    },
    OracleCard {
        number: 11, suit: "Seraphic Fire",
        title: "The Gateway of Fire",
        angel: "Metatron",
        divine_quality: "The transformative threshold; the master-number portal between human and divine",
        guidance: "You stand at a threshold. Walking through it requires you to leave behind an old self.",
        affirmation: "I cross into my higher purpose. I carry only what is true.",
    },

    // ══════════════════════════════════════════════════════════
    //  CELESTIAL WATER (12–22)
    //  Element: Water · Archangel: Gabriel · Domain: Divine Love
    // ══════════════════════════════════════════════════════════

    OracleCard {
        number: 12, suit: "Celestial Water",
        title: "The Healing Spring",
        angel: "Raphael",
        divine_quality: "The living waters that restore body, mind, and spirit",
        guidance: "Allow yourself to receive healing. The water asks only that you open to it.",
        affirmation: "I am open to healing in all its forms. I receive and I give in equal measure.",
    },
    OracleCard {
        number: 13, suit: "Celestial Water",
        title: "Unconditional Love",
        angel: "Chamuel",
        divine_quality: "The love that does not depend on merit; the grace beneath all things",
        guidance: "You are loved not for what you do but for what you are. Rest in this.",
        affirmation: "I am loved beyond condition. I extend that same love to myself and others.",
    },
    OracleCard {
        number: 14, suit: "Celestial Water",
        title: "The Annunciation",
        angel: "Gabriel",
        divine_quality: "The divine message that arrives to announce a new birth or calling",
        guidance: "Something new is being announced within you. Listen for the angel's voice.",
        affirmation: "I am receptive to divine messages. Great news is on its way.",
    },
    OracleCard {
        number: 15, suit: "Celestial Water",
        title: "Compassion",
        angel: "Gabriel",
        divine_quality: "The capacity to feel with another; the heart open to all suffering",
        guidance: "Lead with empathy. Understanding is more powerful than being right.",
        affirmation: "My heart is open. I feel with others and that feeling is sacred strength.",
    },
    OracleCard {
        number: 16, suit: "Celestial Water",
        title: "Sacred Tears",
        angel: "Sandalphon",
        divine_quality: "The divine meaning in grief; tears as the body's prayer",
        guidance: "Let yourself feel. Grief honours what mattered. The angels collect every tear.",
        affirmation: "My feelings are sacred. I let myself grieve so that I can truly heal.",
    },
    OracleCard {
        number: 17, suit: "Celestial Water",
        title: "Intuition",
        angel: "Gabriel",
        divine_quality: "The inner knowing that bypasses reason; the soul's direct perception",
        guidance: "Trust what you know before you know why you know it.",
        affirmation: "My intuition is divine guidance. I trust what I feel without needing proof.",
    },
    OracleCard {
        number: 18, suit: "Celestial Water",
        title: "The Lunar Mirror",
        angel: "Gabriel",
        divine_quality: "Reflection, receptivity, and the wisdom that comes from looking inward",
        guidance: "Be still. What you seek is already within. The mirror only works in silence.",
        affirmation: "In stillness I see clearly. My inner world reveals what my outer world needs.",
    },
    OracleCard {
        number: 19, suit: "Celestial Water",
        title: "Forgiveness",
        angel: "Zadkiel",
        divine_quality: "The release of what was owed; the act that frees both giver and receiver",
        guidance: "Forgiveness is not for them — it is for you. Release the debt and reclaim your peace.",
        affirmation: "I release the past with compassion. I am free and I set others free.",
    },
    OracleCard {
        number: 20, suit: "Celestial Water",
        title: "The Ocean of God",
        angel: "Gabriel",
        divine_quality: "The infinite divine love in which all beings swim without knowing it",
        guidance: "You are immersed in love you cannot see because you are inside it. Feel it now.",
        affirmation: "I swim in an ocean of divine love. I cannot drown because love holds me up.",
    },
    OracleCard {
        number: 21, suit: "Celestial Water",
        title: "Sacred Relationship",
        angel: "Chamuel",
        divine_quality: "The holy mirror of another soul; relationship as the school of divine love",
        guidance: "Your relationships are your greatest teachers. See the divine in the face before you.",
        affirmation: "I see the divine in others. Every relationship is a sacred encounter.",
    },
    OracleCard {
        number: 22, suit: "Celestial Water",
        title: "The Gateway of Water",
        angel: "Metatron",
        divine_quality: "The master-number threshold of feeling; deep surrender to divine love",
        guidance: "This is a moment to surrender completely to what the heart knows.",
        affirmation: "I surrender to love. I trust the river of life to carry me where I need to be.",
    },

    // ══════════════════════════════════════════════════════════
    //  SACRED AIR (23–33)
    //  Element: Air · Archangel: Raphael · Domain: Divine Wisdom
    // ══════════════════════════════════════════════════════════

    OracleCard {
        number: 23, suit: "Sacred Air",
        title: "The Healing Word",
        angel: "Raphael",
        divine_quality: "The spoken divine name; language as the instrument of creation and cure",
        guidance: "Choose your words with care. What you speak, you call into being.",
        affirmation: "My words carry divine power. I speak truth, love, and healing.",
    },
    OracleCard {
        number: 24, suit: "Sacred Air",
        title: "Divine Truth",
        angel: "Raphael",
        divine_quality: "The light that cannot be distorted; the fact that exists beyond argument",
        guidance: "The truth you are avoiding is the one that will set you free.",
        affirmation: "I live in truth. What is true for me I speak and embody with courage.",
    },
    OracleCard {
        number: 25, suit: "Sacred Air",
        title: "The Breath of God",
        angel: "Raphael",
        divine_quality: "The animating spirit; the divine inbreathing that makes matter alive",
        guidance: "Breathe consciously. You are breathing the same breath that animated the first life.",
        affirmation: "With every breath I receive divine life. I am fully alive and fully present.",
    },
    OracleCard {
        number: 26, suit: "Sacred Air",
        title: "Clarity of Mind",
        angel: "Raphael",
        divine_quality: "The undistorted perception; the mind that sees what is rather than what is feared",
        guidance: "Still the chatter. Beneath the noise is a clear knowing waiting for you.",
        affirmation: "My mind is clear and receptive. I see situations as they truly are.",
    },
    OracleCard {
        number: 27, suit: "Sacred Air",
        title: "The Messenger",
        angel: "Raphael",
        divine_quality: "The angelic function of carrying divine communication between realms",
        guidance: "You are either the messenger or the recipient. Either way, what arrives is important.",
        affirmation: "I deliver and receive sacred messages. I am a clear channel.",
    },
    OracleCard {
        number: 28, suit: "Sacred Air",
        title: "Sacred Study",
        angel: "Raziel",
        divine_quality: "The devotion to understanding; learning as a form of worship",
        guidance: "Go deeper. The surface has been explored; now dive into the mystery beneath.",
        affirmation: "I am a devoted student of life and wisdom. Every experience teaches me.",
    },
    OracleCard {
        number: 29, suit: "Sacred Air",
        title: "Freedom",
        angel: "Raphael",
        divine_quality: "The birthright of every soul; the open sky that belongs to all beings",
        guidance: "What are you allowing to confine you? The cage door may already be open.",
        affirmation: "I am free. I choose my responses and I live with an open horizon.",
    },
    OracleCard {
        number: 30, suit: "Sacred Air",
        title: "Higher Perspective",
        angel: "Raphael",
        divine_quality: "The eagle's view; seeing the whole pattern from above the immediate",
        guidance: "Rise above the situation. From altitude, the path through becomes visible.",
        affirmation: "I see the bigger picture. My challenges are part of a larger, sacred story.",
    },
    OracleCard {
        number: 31, suit: "Sacred Air",
        title: "The Still Small Voice",
        angel: "Elijah",
        divine_quality: "The gentle divine whisper that can only be heard in silence",
        guidance: "The loudest voice in the room is rarely the wisest. Seek the quiet one.",
        affirmation: "I listen for the still small voice. In silence I find the truest guidance.",
    },
    OracleCard {
        number: 32, suit: "Sacred Air",
        title: "Sacred Science",
        angel: "Uriel",
        divine_quality: "The divine intelligence behind natural law; creation as a book to be read",
        guidance: "Look for the pattern. Every phenomenon is an angel speaking in disguise.",
        affirmation: "I see the sacred in the rational. Wisdom and science are two languages of the same truth.",
    },
    OracleCard {
        number: 33, suit: "Sacred Air",
        title: "The Gateway of Air",
        angel: "Metatron",
        divine_quality: "The master-number threshold of mind; divine revelation through pure thought",
        guidance: "Your thinking is about to change entirely. Allow the old map to dissolve.",
        affirmation: "I am open to a new understanding. My mind expands to receive divine truth.",
    },

    // ══════════════════════════════════════════════════════════
    //  HOLY EARTH (34–44)
    //  Element: Earth · Archangel: Uriel · Domain: Divine Order
    // ══════════════════════════════════════════════════════════

    OracleCard {
        number: 34, suit: "Holy Earth",
        title: "The Garden of God",
        angel: "Uriel",
        divine_quality: "The created world as a sacred garden tended by angelic gardeners",
        guidance: "Tend what is in your care. Small faithful tending brings the greatest harvest.",
        affirmation: "I tend my life as a sacred garden. Every act of care is an act of love.",
    },
    OracleCard {
        number: 35, suit: "Holy Earth",
        title: "Abundance",
        angel: "Uriel",
        divine_quality: "The divine generosity expressed in the fecundity of creation",
        guidance: "Abundance is your natural state. Scarcity is a thought; let it go.",
        affirmation: "I live in divine abundance. I receive generously and give generously.",
    },
    OracleCard {
        number: 36, suit: "Holy Earth",
        title: "Sacred Body",
        angel: "Raphael",
        divine_quality: "The body as a temple of the divine; flesh as sanctified matter",
        guidance: "Honour your body. It is the instrument through which your spirit touches the world.",
        affirmation: "My body is sacred. I care for it with love and gratitude.",
    },
    OracleCard {
        number: 37, suit: "Holy Earth",
        title: "Divine Timing",
        angel: "Uriel",
        divine_quality: "The cosmic clock; the perfect moment for each thing to unfold",
        guidance: "Trust the timing. What is coming is on its way; pushing does not speed what ripens.",
        affirmation: "Everything in my life unfolds in perfect divine timing.",
    },
    OracleCard {
        number: 38, suit: "Holy Earth",
        title: "Ancestral Wisdom",
        angel: "Sandalphon",
        divine_quality: "The encoded knowledge of the lineage; the wisdom carried in the blood and bone",
        guidance: "Your ancestors walked difficult ground so you could stand where you stand. Honour them.",
        affirmation: "I carry the wisdom of my lineage. I honour the past and I build a better future.",
    },
    OracleCard {
        number: 39, suit: "Holy Earth",
        title: "Sacred Work",
        angel: "Uriel",
        divine_quality: "Labour as worship; the divine principle embedded in purposeful craft",
        guidance: "Do the work with full presence. The act of making is itself a prayer.",
        affirmation: "My work is my offering. I bring skill, care, and presence to everything I do.",
    },
    OracleCard {
        number: 40, suit: "Holy Earth",
        title: "Grounding",
        angel: "Uriel",
        divine_quality: "The return to the body and earth; the downward current that stabilises spirit",
        guidance: "Come down from the heights. The earth needs your presence, not your transcendence.",
        affirmation: "I am grounded in the present moment. My roots run deep and hold me steady.",
    },
    OracleCard {
        number: 41, suit: "Holy Earth",
        title: "The Covenant",
        angel: "Ariel",
        divine_quality: "The sacred agreement between heaven and earth; the promise that holds creation",
        guidance: "Keep your word. Sacred commitments are the bones of integrity.",
        affirmation: "I honour my commitments. I am trustworthy and my word is my bond.",
    },
    OracleCard {
        number: 42, suit: "Holy Earth",
        title: "Natural Magic",
        angel: "Uriel",
        divine_quality: "The intelligence in seed, stone, and season; the sacred in the ordinary",
        guidance: "The magic you seek is already present in the world around you. Look again.",
        affirmation: "I see the sacred in the natural world. I live in a world alive with wonder.",
    },
    OracleCard {
        number: 43, suit: "Holy Earth",
        title: "Patience",
        angel: "Uriel",
        divine_quality: "The deep calm of the earth that does not rush the seasons",
        guidance: "The oak does not hurry. Neither must you. What is growing in you is growing well.",
        affirmation: "I am patient with my process. Great things take time, and I trust the timing.",
    },
    OracleCard {
        number: 44, suit: "Holy Earth",
        title: "The Gateway of Earth",
        angel: "Metatron",
        divine_quality: "The master-number completion; spirit fully embodied in matter, heaven on earth",
        guidance: "You are called to bring heaven into the everyday — not to escape earth, but to consecrate it.",
        affirmation: "I am spirit in form. I make the sacred practical and the practical sacred.",
    },
];

// ─── Lookup helpers ───────────────────────────────────────────────────────────

/// Find an Oracle card by number (1–44).
pub fn oracle_by_number(n: u8) -> Option<&'static OracleCard> {
    ORACLE.iter().find(|c| c.number == n)
}

/// Find an Oracle card by title (case-insensitive, partial match).
#[allow(dead_code)]
pub fn oracle_by_title(query: &str) -> Option<&'static OracleCard> {
    let q = query.to_lowercase();
    ORACLE
        .iter()
        .find(|c| c.title.to_lowercase().contains(&q))
}

/// Return all Oracle cards for one suit (case-insensitive).
pub fn oracle_suit_cards(suit: &str) -> Vec<&'static OracleCard> {
    let s = suit.to_lowercase();
    ORACLE
        .iter()
        .filter(|c| c.suit.to_lowercase().contains(&s))
        .collect()
}
