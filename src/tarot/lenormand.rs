//! Lenormand Oracle — 36 card petit jeu.
//!
//! ## Historical note
//!
//! Despite the name, the Lenormand deck was **not** created by the French
//! fortune-teller Marie Anne Lenormand (1772–1843), who used a 52-card playing
//! deck.  The petit jeu (small game) was designed in the early 19th century and
//! retroactively attributed to her after her death, capitalising on her fame.
//! The system derives from the earlier German *Spiel der Hoffnung* ("Game of
//! Hope", 1799 by Johann Kaspar Hechtel) and shares its roots with the
//! *piquet* playing-card tradition.
//!
//! **This module is intentionally placed inside `src/tarot/` for organisational
//! convenience, but Lenormand is a distinct cartomantic tradition, not a Tarot
//! system.** It predates the modern Tarot revival, carries no Kabbalistic or
//! Hermetic framework, and is read through a combinatorial folk-symbolic
//! language rather than the arcana structure.
//!
//! ## Reading style
//!
//! Lenormand cards are *always* read in combination.  A single card merely
//! names a topic; two or more cards form a sentence.  The deck is typically
//! read in spreads of 3, 5, or the full *Grand Tableau* (36 cards, 4 rows).
//!
//! ## Sources
//!
//! - **Hechtel**, *Spiel der Hoffnung* (1799) — the original 36-symbol
//!   playing-card insert from which the Lenormand tradition derives.
//! - **Rana George**, *The Essential Lenormand* (2013, Llewellyn) — authoritative
//!   modern reference for traditional meanings and the Grand Tableau.
//! - **Caitlín Matthews**, *The Complete Lenormand Oracle Handbook* (2014,
//!   Destiny Books) — card-by-card meanings and combinatorial reading.
//! - **Andy Boroveshengra**, *Lenormand Thirty Six Cards* (2013, Createspace) —
//!   strict traditional German meanings.

// ─── Data type ────────────────────────────────────────────────────────────────

/// One card of the 36-card Lenormand petit jeu.
pub struct LenormandCard {
    /// Sequential card number (1–36) as standardised in the tradition.
    pub number: u8,
    /// Traditional German / English card name.
    pub name: &'static str,
    /// The playing-card inset printed on the original Hechtel deck.
    pub playing_card: &'static str,
    /// Primary symbolic domain (person, object, event, or force).
    pub domain: &'static str,
    /// Core positive or neutral meanings.
    pub meaning_light: &'static str,
    /// Shadow or challenging aspect of the card.
    pub meaning_shadow: &'static str,
    /// Brief combinatorial note — how this card modifies its neighbours.
    pub reading_note: &'static str,
}

// ─── The thirty-six cards ─────────────────────────────────────────────────────

pub static LENORMAND: &[LenormandCard] = &[
    LenormandCard {
        number: 1,
        name: "The Rider",
        playing_card: "9 of Hearts",
        domain: "News · Arrival · Movement",
        meaning_light: "Swift news arriving, a dynamic young person, messages from afar",
        meaning_shadow: "Hasty decisions, news that unsettles, impulsive behaviour",
        reading_note: "Quickens whatever card it precedes; the message IS the next card",
    },
    LenormandCard {
        number: 2,
        name: "The Clover",
        playing_card: "6 of Diamonds",
        domain: "Luck · Opportunity · Spontaneity",
        meaning_light: "Small good luck, fleeting opportunity, lightheartedness, play",
        meaning_shadow: "Luck that passes quickly, gambling, trivial matters",
        reading_note: "Brightens whatever surrounds it; luck is small but real",
    },
    LenormandCard {
        number: 3,
        name: "The Ship",
        playing_card: "10 of Spades",
        domain: "Travel · Commerce · Distance · Foreign",
        meaning_light: "Journey, trade, ambition set on a distant horizon, the wider world",
        meaning_shadow: "Departure, separation, restlessness, ventures with unclear outcomes",
        reading_note: "Points toward distant or foreign matters; next card shows the destination",
    },
    LenormandCard {
        number: 4,
        name: "The House",
        playing_card: "King of Hearts",
        domain: "Home · Security · Family · Tradition",
        meaning_light: "Stability, the domestic sphere, property, family matters, privacy",
        meaning_shadow: "Insularity, resistance to change, family conflict",
        reading_note: "Domesticates surrounding cards; next card enters the querent's private life",
    },
    LenormandCard {
        number: 5,
        name: "The Tree",
        playing_card: "7 of Hearts",
        domain: "Health · Growth · Roots · Time",
        meaning_light: "Good health, slow steady growth, ancestral roots, spiritual grounding",
        meaning_shadow: "Illness (especially chronic), stagnation, karmic patterns",
        reading_note: "Slows the pace of surrounding cards; adds a health dimension to any topic",
    },
    LenormandCard {
        number: 6,
        name: "The Clouds",
        playing_card: "King of Clubs",
        domain: "Confusion · Uncertainty · Hidden things",
        meaning_light: "Temporary uncertainty, situations not yet clear, the need for patience",
        meaning_shadow: "Deception, confusion, depression, things obscured on purpose",
        reading_note: "Dark side faces the card it most affects; light side shows where clarity returns",
    },
    LenormandCard {
        number: 7,
        name: "The Snake",
        playing_card: "Queen of Clubs",
        domain: "Complexity · Desire · Detours",
        meaning_light: "Wisdom, a sophisticated woman, indirect routes, necessary detours",
        meaning_shadow: "Betrayal, manipulation, sexual complications, a rival",
        reading_note: "Introduces complexity and indirection; signals a secondary path or person",
    },
    LenormandCard {
        number: 8,
        name: "The Coffin",
        playing_card: "9 of Diamonds",
        domain: "Endings · Transformation · Illness",
        meaning_light: "Completion, putting something to rest, endings that free the querent",
        meaning_shadow: "Loss, severe illness, grief, a definitive closure",
        reading_note: "Terminates or transforms whatever it touches; neighbour shows what ends",
    },
    LenormandCard {
        number: 9,
        name: "The Bouquet",
        playing_card: "Queen of Spades",
        domain: "Beauty · Gifts · Happiness · Appreciation",
        meaning_light: "Pleasant surprise, appreciation, social grace, beauty, a kind woman",
        meaning_shadow: "Superficiality, hollow compliments, gifts with strings attached",
        reading_note: "Sweetens and beautifies whatever neighbours it; social harmony",
    },
    LenormandCard {
        number: 10,
        name: "The Scythe",
        playing_card: "Jack of Diamonds",
        domain: "Sudden cuts · Harvest · Decisions",
        meaning_light: "Decisive action, harvesting what was planted, surgical precision",
        meaning_shadow: "Accidents, abrupt endings, danger, a sudden severing",
        reading_note: "Blade direction matters: cuts toward the card it faces; brings swift change",
    },
    LenormandCard {
        number: 11,
        name: "The Whip",
        playing_card: "Jack of Clubs",
        domain: "Conflict · Repetition · Sport · Discussion",
        meaning_light: "Productive debate, physical training, repetitive work building mastery",
        meaning_shadow: "Arguments, aggression, punishment, repetitive problems, abuse",
        reading_note: "Introduces strife or repetition; with positive cards: vigorous activity",
    },
    LenormandCard {
        number: 12,
        name: "The Birds",
        playing_card: "7 of Diamonds",
        domain: "Conversation · Nervousness · Couple",
        meaning_light: "Lively talk, a couple, news exchanged, social chatter",
        meaning_shadow: "Gossip, anxiety, scattered thoughts, too much noise",
        reading_note: "Multiplies and amplifies; surrounding cards are discussed or spread about",
    },
    LenormandCard {
        number: 13,
        name: "The Child",
        playing_card: "Jack of Spades",
        domain: "Newness · Innocence · Small things",
        meaning_light: "A new beginning, childhood, something small and tender, playfulness",
        meaning_shadow: "Immaturity, naïveté, vulnerability, a child in difficulty",
        reading_note: "Diminishes and softens surrounding cards; next card is small or new",
    },
    LenormandCard {
        number: 14,
        name: "The Fox",
        playing_card: "9 of Clubs",
        domain: "Cunning · Self-interest · Work",
        meaning_light: "Cleverness, self-preservation, adaptation, work and practical affairs",
        meaning_shadow: "Deception, slyness, mistrust, someone pursuing their own agenda",
        reading_note: "Adds an element of caution or strategy to surrounding cards",
    },
    LenormandCard {
        number: 15,
        name: "The Bear",
        playing_card: "10 of Clubs",
        domain: "Strength · Leadership · Finances",
        meaning_light: "Power, authority, financial strength, a protective leader",
        meaning_shadow: "Dominance, possessiveness, financial pressure, an overbearing figure",
        reading_note: "Strengthens and enlarges; with financial cards, amplifies wealth or debt",
    },
    LenormandCard {
        number: 16,
        name: "The Stars",
        playing_card: "6 of Hearts",
        domain: "Hope · Guidance · Clarity · Wishes",
        meaning_light: "Inspiration, divine guidance, hope fulfilled, clarity of vision, fame",
        meaning_shadow: "Illusions, wishful thinking, ambitions beyond reach",
        reading_note: "Elevates and clarifies; the card it touches receives heavenly blessing",
    },
    LenormandCard {
        number: 17,
        name: "The Stork",
        playing_card: "Queen of Hearts",
        domain: "Change · Return · Pregnancy · Migration",
        meaning_light: "Positive change, return of something, pregnancy, transformation",
        meaning_shadow: "Unwanted changes, upheaval, restlessness, instability",
        reading_note: "Brings change; next card shows what changes or what arrives",
    },
    LenormandCard {
        number: 18,
        name: "The Dog",
        playing_card: "10 of Hearts",
        domain: "Friendship · Loyalty · Trust",
        meaning_light: "A trusted friend, loyalty, faithful support, companionship",
        meaning_shadow: "Sycophancy, blind loyalty, a dependent relationship",
        reading_note: "Adds trustworthiness and support; the Dog protects surrounding matters",
    },
    LenormandCard {
        number: 19,
        name: "The Tower",
        playing_card: "6 of Spades",
        domain: "Authority · Institutions · Isolation",
        meaning_light: "Established authority, government, career advancement, solitude chosen",
        meaning_shadow: "Loneliness, bureaucracy, rigid hierarchy, legal institutions as obstacles",
        reading_note: "Note: distinct from Tarot Tower. Represents authority, not sudden collapse",
    },
    LenormandCard {
        number: 20,
        name: "The Garden",
        playing_card: "8 of Spades",
        domain: "Society · Public life · Community",
        meaning_light: "Social events, the public sphere, networking, parties, gatherings",
        meaning_shadow: "Public exposure, social pressure, the performance of a public face",
        reading_note: "Makes surrounding cards public; the topic becomes known or social",
    },
    LenormandCard {
        number: 21,
        name: "The Mountain",
        playing_card: "8 of Clubs",
        domain: "Obstacles · Delays · Challenges",
        meaning_light: "A challenge that builds strength, the discipline of a long climb",
        meaning_shadow: "Serious blockage, enemy action, problems that do not yield easily",
        reading_note: "Blocks and delays; the card it faces is the obstacle or its nature",
    },
    LenormandCard {
        number: 22,
        name: "The Crossroads",
        playing_card: "Queen of Diamonds",
        domain: "Choices · Independence · Alternatives",
        meaning_light: "A decision point, freedom to choose, multiple valid paths available",
        meaning_shadow: "Indecision, divided loyalties, too many options leading to paralysis",
        reading_note: "The two flanking cards are the two paths; the querent must choose",
    },
    LenormandCard {
        number: 23,
        name: "The Mice",
        playing_card: "7 of Clubs",
        domain: "Loss · Depletion · Anxiety",
        meaning_light: "Small losses that are manageable, the clearing out of what is not needed",
        meaning_shadow: "Ongoing depletion, theft, worry, that which nibbles resources away",
        reading_note: "Reduces and erodes surrounding cards; the neighbour suffers slow loss",
    },
    LenormandCard {
        number: 24,
        name: "The Heart",
        playing_card: "Jack of Hearts",
        domain: "Love · Emotion · Generosity",
        meaning_light: "Romantic love, heartfelt generosity, emotional fulfilment",
        meaning_shadow: "Heartache, emotional vulnerability, love as neediness",
        reading_note: "Brings love and warm feeling to whatever it touches",
    },
    LenormandCard {
        number: 25,
        name: "The Ring",
        playing_card: "Ace of Clubs",
        domain: "Commitment · Contract · Cycles",
        meaning_light: "Binding agreements, marriage, promises kept, cyclical completion",
        meaning_shadow: "Binding contracts that constrain, relationships without exit",
        reading_note: "Formalises and commits; the flanking cards show what is being bound",
    },
    LenormandCard {
        number: 26,
        name: "The Book",
        playing_card: "10 of Diamonds",
        domain: "Secrets · Knowledge · Study",
        meaning_light: "Hidden knowledge, research, education, a secret about to be revealed",
        meaning_shadow: "Information withheld, mysteries not yet penetrated, hidden agendas",
        reading_note: "The closed Book = secret; the open Book = knowledge revealed",
    },
    LenormandCard {
        number: 27,
        name: "The Letter",
        playing_card: "7 of Spades",
        domain: "Communication · Documents · Written word",
        meaning_light: "Important written communication, documents, contracts, a letter",
        meaning_shadow: "Unwelcome correspondence, legal notices, communication problems",
        reading_note: "Next card describes the content; preceding card describes the sender",
    },
    LenormandCard {
        number: 28,
        name: "The Man",
        playing_card: "Ace of Hearts",
        domain: "The male querent · A significant man",
        meaning_light: "The querent (if male) or the main male person in the reading",
        meaning_shadow: "Context-dependent; surrounding cards carry the meaning",
        reading_note: "Personalises the reading; the cards around him describe his life",
    },
    LenormandCard {
        number: 29,
        name: "The Woman",
        playing_card: "Ace of Spades",
        domain: "The female querent · A significant woman",
        meaning_light: "The querent (if female) or the main female person in the reading",
        meaning_shadow: "Context-dependent; surrounding cards carry the meaning",
        reading_note: "Personalises the reading; the cards around her describe her life",
    },
    LenormandCard {
        number: 30,
        name: "The Lily",
        playing_card: "King of Spades",
        domain: "Virtue · Maturity · Sensuality · Winter",
        meaning_light: "Peace, wisdom of age, sexual pleasure within commitment, purity",
        meaning_shadow: "Cold detachment, prudishness, repressed desire",
        reading_note: "Adds maturity and stillness; cooling card that calms heated topics",
    },
    LenormandCard {
        number: 31,
        name: "The Sun",
        playing_card: "Ace of Diamonds",
        domain: "Success · Energy · Clarity · Summer",
        meaning_light: "Triumph, vitality, the best possible outcome, public success",
        meaning_shadow: "Over-confidence, arrogance, burning too bright",
        reading_note: "One of the two great blessings; illuminates and succeeds whatever it touches",
    },
    LenormandCard {
        number: 32,
        name: "The Moon",
        playing_card: "8 of Hearts",
        domain: "Recognition · Intuition · Emotional depth",
        meaning_light: "Public recognition, honour, intuitive gifts, the unconscious surfacing",
        meaning_shadow: "Emotional instability, illusions, dreams that mislead",
        reading_note: "Brings honour and emotional depth; adds intuitive or unconscious dimensions",
    },
    LenormandCard {
        number: 33,
        name: "The Key",
        playing_card: "8 of Diamonds",
        domain: "Solutions · Certainty · Unlocking",
        meaning_light: "The answer found, a definitive solution, fate that opens doors",
        meaning_shadow: "A solution that creates new problems; over-confidence in the outcome",
        reading_note: "Confirms and solves; the card it touches IS the answer or the opening",
    },
    LenormandCard {
        number: 34,
        name: "The Fish",
        playing_card: "King of Diamonds",
        domain: "Abundance · Flow · Commerce · Water",
        meaning_light: "Financial abundance, flowing resources, business, the emotional depths",
        meaning_shadow: "Excess, addiction, the inability to stop once started",
        reading_note: "Multiplies and enriches; with financial cards, great abundance",
    },
    LenormandCard {
        number: 35,
        name: "The Anchor",
        playing_card: "9 of Spades",
        domain: "Stability · Work · Perseverance",
        meaning_light: "Security, persistence, a steady job, long-term commitment that holds",
        meaning_shadow: "Stuck, unable to move on, work that has become a chain",
        reading_note: "Stabilises and grounds; surrounded topics are lasting and secure",
    },
    LenormandCard {
        number: 36,
        name: "The Cross",
        playing_card: "6 of Clubs",
        domain: "Fate · Burden · Suffering · Faith",
        meaning_light: "A karmic lesson, a burden carried with grace, faith amid hardship",
        meaning_shadow: "Unavoidable suffering, painful destiny, a heavy obligation",
        reading_note: "The final card; whatever it touches is fated or carries heavy significance",
    },
];

// ─── Lookup helpers ───────────────────────────────────────────────────────────

/// Find a Lenormand card by number (1–36).
pub fn lenormand_by_number(n: u8) -> Option<&'static LenormandCard> {
    LENORMAND.iter().find(|c| c.number == n)
}

/// Find a Lenormand card by name (case-insensitive, partial match).
pub fn lenormand_by_name(query: &str) -> Option<&'static LenormandCard> {
    let q = query.to_lowercase();
    LENORMAND
        .iter()
        .find(|c| c.name.to_lowercase().contains(&q))
}
