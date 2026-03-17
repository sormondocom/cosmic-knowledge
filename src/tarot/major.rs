//! Angelic Tarot — Major Arcana (22 Trump cards).
//!
//! Each card of the Major Arcana encodes a complete metaphysical teaching: a
//! Hebrew letter, a path on the Kabbalistic Tree of Life, an elemental or
//! planetary or zodiacal force, and a presiding angelic intelligence drawn from
//! the Western Hermetic tradition.
//!
//! ## Sources
//!
//! - **Sefer Yetzirah** (attributed to Abraham; *ed.* Kaplan 1997, Aryeh Kaplan) —
//!   the foundational text that assigns one of the twenty-two Hebrew letters to
//!   each of the three elements, seven planets, and twelve zodiac signs.
//! - **Pseudo-Dionysius the Areopagite**, *The Celestial Hierarchy* (c. 5th c. CE;
//!   trans. Luibheid, Paulist Press 1987) — the nine angelic orders: Seraphim,
//!   Cherubim, Thrones, Dominions, Virtues, Powers, Principalities,
//!   Archangels, Angels.
//! - **Cornelius Agrippa**, *Three Books of Occult Philosophy* (1531; trans. Tyson
//!   1993, Llewellyn) — planetary archangels and zodiacal angels (Book III,
//!   ch. 24–28).
//! - **Francis Barrett**, *The Magus* (1801; reprint Weiser 2000) — angelic
//!   attributions for planets and signs, drawing on Agrippa.
//! - **Israel Regardie**, *The Golden Dawn* (1937–40; 6th ed. Llewellyn 1989) —
//!   Tarot-path-Sephiroth assignments and angelic correspondences as
//!   systematised by the Hermetic Order of the Golden Dawn.
//! - **Paul Foster Case**, *The Tarot: A Key to the Wisdom of the Ages* (1947;
//!   reprint Tarcher/Penguin 2006) — B.O.T.A. angelic and symbolic commentary.
//! - **Lon Milo DuQuette**, *Understanding Aleister Crowley's Thoth Tarot*
//!   (2003, Weiser) — modern scholarly synthesis cross-referenced here.
//!
//! ## Card numbering
//!
//! Rider-Waite-Smith (RWS) ordering is used throughout:
//! - Card **8** = Strength (Teth / Leo)
//! - Card **11** = Justice (Lamed / Libra)
//!
//! The Golden Dawn original reversed these two; Waite corrected to astrological
//! sequence.  The Thoth deck retains the GD ordering (8=Justice/Adjustment,
//! 11=Strength/Lust) — noted in the `keywords` field where relevant.

// ─── Data type ────────────────────────────────────────────────────────────────

/// One Major Arcana card with complete angelic correspondences.
pub struct MajorArcanum {
    /// Card number in RWS order (0 = The Fool … 21 = The World).
    pub number: u8,
    /// Traditional English name.
    pub name: &'static str,
    /// Hebrew letter: transliterated name · script glyph · meaning.
    pub hebrew_letter: &'static str,
    /// Path number on the Kabbalistic Tree of Life (paths 11–32).
    pub path: u8,
    /// The two Sephiroth joined by this path.
    pub sephiroth: &'static str,
    /// Astrological / elemental correspondence (element, planet, or sign).
    pub element: &'static str,
    /// Primary ruling angel name.
    pub angel: &'static str,
    /// Angelic choir / order (Pseudo-Dionysian hierarchy).
    pub angelic_order: &'static str,
    /// The divine quality or cosmic function this angel embodies.
    pub divine_quality: &'static str,
    /// Core interpretive keywords for upright reading.
    pub keywords: &'static str,
}

// ─── The twenty-two Trump cards ───────────────────────────────────────────────

pub static MAJOR_ARCANA: &[MajorArcanum] = &[
    // ── 0. The Fool ──────────────────────────────────────────────────────────
    // Aleph = ox, first letter, primal breath.  Mother letter of Air.
    // Path 11: from Kether (the Crown) to Chokmah (Wisdom) — the first
    // emanation, the divine leap before form.  Angel: Raphael, the great
    // healer, whose name means "God heals"; in the Zohar he is lord of
    // the winds and stands at the east, the quarter of Air and sunrise.
    MajorArcanum {
        number: 0,
        name: "The Fool",
        hebrew_letter: "Aleph (אָלֶף) · א · Ox",
        path: 11,
        sephiroth: "Kether (Crown) ─── Chokmah (Wisdom)",
        element: "Air (Mother letter)",
        angel: "Raphael",
        angelic_order: "Seraphim",
        divine_quality: "The breath of God; healing winds; the sacred leap into the unknown",
        keywords: "Innocence · new beginnings · spontaneity · divine trust · the holy fool",
    },

    // ── 1. The Magician ───────────────────────────────────────────────────────
    // Beth = house; first double letter; Mercury (Kokav כּוֹכָב).
    // Path 12: Kether to Binah — pure will descending into form.
    // Raphael governs Mercury in the planetary hierarchy of both Agrippa
    // (Book III, ch. 24) and Barrett; the Magician wields the caduceus of
    // the divine messenger.
    MajorArcanum {
        number: 1,
        name: "The Magician",
        hebrew_letter: "Beth (בֵּית) · ב · House",
        path: 12,
        sephiroth: "Kether (Crown) ─── Binah (Understanding)",
        element: "Mercury (Kokav — כּוֹכָב)",
        angel: "Raphael",
        angelic_order: "Cherubim",
        divine_quality: "Divine messenger, healing intelligence, the as-above-so-below principle",
        keywords: "Will · skill · focused power · manifestation · the sacred arts",
    },

    // ── 2. The High Priestess ─────────────────────────────────────────────────
    // Gimel = camel; double letter; Moon (Levanah לְבָנָה).
    // Path 13: Kether to Tiphareth — the silver thread of lunar mystery
    // connecting Crown to Heart.  Gabriel is the Archangel of the Moon and
    // of Water; his name means "God is my strength."  He guards the veil
    // between seen and unseen.
    MajorArcanum {
        number: 2,
        name: "The High Priestess",
        hebrew_letter: "Gimel (גִּימֶל) · ג · Camel",
        path: 13,
        sephiroth: "Kether (Crown) ─── Tiphareth (Beauty)",
        element: "Moon (Levanah — לְבָנָה)",
        angel: "Gabriel",
        angelic_order: "Thrones",
        divine_quality: "Guardian of the lunar veil; keeper of sacred memory and divine mystery",
        keywords: "Intuition · mystery · the unconscious · sacred knowledge · the veil",
    },

    // ── 3. The Empress ────────────────────────────────────────────────────────
    // Daleth = door; double letter; Venus (Nogah נֹגַהּ).
    // Path 14: Chokmah to Binah — the Supernal Feminine, the door between
    // Father and Mother.  Haniel (Anael) is the Archangel of Venus, whose
    // name means "Grace of God"; he governs love, beauty, and abundance.
    MajorArcanum {
        number: 3,
        name: "The Empress",
        hebrew_letter: "Daleth (דָּלֶת) · ד · Door",
        path: 14,
        sephiroth: "Chokmah (Wisdom) ─── Binah (Understanding)",
        element: "Venus (Nogah — נֹגַהּ)",
        angel: "Haniel",
        angelic_order: "Dominions",
        divine_quality: "Grace of God; sacred love, beauty, and the abundance of Nature",
        keywords: "Fertility · abundance · nature · nurturing · creative power",
    },

    // ── 4. The Emperor ────────────────────────────────────────────────────────
    // Heh = window; simple letter; Aries (Taleh טָלֶה).
    // Path 15: Chokmah to Tiphareth — the archetypal Father, the window
    // through which heavenly light orders earthly form.
    // Malkhidael (Machidiel) governs Aries in Agrippa's and Barrett's
    // lists of zodiacal rulers; the name derives from "Kingdom of God."
    MajorArcanum {
        number: 4,
        name: "The Emperor",
        hebrew_letter: "Heh (הֵא) · ה · Window",
        path: 15,
        sephiroth: "Chokmah (Wisdom) ─── Tiphareth (Beauty)",
        element: "Aries ♈ (Taleh — טָלֶה)",
        angel: "Malkhidael",
        angelic_order: "Powers",
        divine_quality: "Kingdom of God; divine authority that structures chaos into cosmos",
        keywords: "Authority · structure · stability · leadership · the cosmic law",
    },

    // ── 5. The Hierophant ─────────────────────────────────────────────────────
    // Vav = nail/hook; simple letter; Taurus (Shor שׁוֹר).
    // Path 16: Chokmah to Chesed — the nail of tradition that connects
    // heavenly wisdom to earthly mercy.
    // Asmodel governs Taurus; the name is found in Barrett's *The Magus*
    // and in the Ars Almadel and Sefer Raziel traditions.
    MajorArcanum {
        number: 5,
        name: "The Hierophant",
        hebrew_letter: "Vav (וָו) · ו · Nail / Hook",
        path: 16,
        sephiroth: "Chokmah (Wisdom) ─── Chesed (Mercy)",
        element: "Taurus ♉ (Shor — שׁוֹר)",
        angel: "Asmodel",
        angelic_order: "Virtues",
        divine_quality: "Guardian of sacred tradition; transmitter of celestial wisdom to earth",
        keywords: "Tradition · spiritual teaching · sacred ritual · divine law · initiation",
    },

    // ── 6. The Lovers ─────────────────────────────────────────────────────────
    // Zayin = sword; simple letter; Gemini (Te'omim תְּאוֹמִים).
    // Path 17: Binah to Tiphareth — the sword of discernment that passes
    // through Understanding to illumine the Heart.
    // Ambriel (Amriel) is the traditional angelic ruler of Gemini in
    // Agrippa and Barrett; the name relates to "speech of God."
    MajorArcanum {
        number: 6,
        name: "The Lovers",
        hebrew_letter: "Zayin (זַיִן) · ז · Sword",
        path: 17,
        sephiroth: "Binah (Understanding) ─── Tiphareth (Beauty)",
        element: "Gemini ♊ (Te'omim — תְּאוֹמִים)",
        angel: "Ambriel",
        angelic_order: "Principalities",
        divine_quality: "Speech of God; the sacred bond that aligns opposites in divine choice",
        keywords: "Love · alignment · sacred choice · duality · conscious union",
    },

    // ── 7. The Chariot ────────────────────────────────────────────────────────
    // Cheth = fence/field; simple letter; Cancer (Sartan סַרְטָן).
    // Path 18: Binah to Geburah — the protective fence channelling divine
    // Understanding into divine Severity.
    // Muriel governs Cancer; Muriel's name means "myrrh of God" and the
    // angel is associated with water, memory, and maternal protection.
    MajorArcanum {
        number: 7,
        name: "The Chariot",
        hebrew_letter: "Cheth (חֵית) · ח · Fence / Field",
        path: 18,
        sephiroth: "Binah (Understanding) ─── Geburah (Severity)",
        element: "Cancer ♋ (Sartan — סַרְטָן)",
        angel: "Muriel",
        angelic_order: "Archangels",
        divine_quality: "Myrrh of God; divine will marshalling opposites toward victory",
        keywords: "Determination · victory · self-mastery · directed will · sacred journey",
    },

    // ── 8. Strength ───────────────────────────────────────────────────────────
    // Teth = serpent; simple letter; Leo (Aryeh אַרְיֵה).
    // Path 19: Chesed to Geburah — the serpentine force that mediates
    // divine Mercy and divine Severity.  (Note: GD/Thoth assigns this to
    // Justice; Waite restored astrological order so Leo = Strength.)
    // Verchiel governs Leo; from the Latin Virgo Caeli ("maiden of heaven")
    // corrupted, Verchiel relates to radiance and charitable power.
    MajorArcanum {
        number: 8,
        name: "Strength",
        hebrew_letter: "Teth (טֵית) · ט · Serpent",
        path: 19,
        sephiroth: "Chesed (Mercy) ─── Geburah (Severity)",
        element: "Leo ♌ (Aryeh — אַרְיֵה)",
        angel: "Verchiel",
        angelic_order: "Angels",
        divine_quality: "Radiant charitable power; courage tempered by compassion",
        keywords: "Inner strength · courage · patience · gentle authority · sacred fortitude",
    },

    // ── 9. The Hermit ─────────────────────────────────────────────────────────
    // Yod = hand; simple letter; Virgo (Betulah בְּתוּלָה).
    // Path 20: Chesed to Tiphareth — the hand of God reaching from Mercy
    // to illuminate the Heart.
    // Hamaliel governs Virgo; the name means "grace and harmony of God"
    // and the angel is associated with analytical wisdom and purification.
    MajorArcanum {
        number: 9,
        name: "The Hermit",
        hebrew_letter: "Yod (יוֹד) · י · Hand",
        path: 20,
        sephiroth: "Chesed (Mercy) ─── Tiphareth (Beauty)",
        element: "Virgo ♍ (Betulah — בְּתוּלָה)",
        angel: "Hamaliel",
        angelic_order: "Seraphim",
        divine_quality: "Grace of analytical wisdom; the inner lantern that guides seekers",
        keywords: "Solitude · inner guidance · prudence · illumination · the sacred lamp",
    },

    // ── 10. Wheel of Fortune ──────────────────────────────────────────────────
    // Kaph = palm of hand; double letter; Jupiter (Tzedek צֶדֶק).
    // Path 21: Chesed to Netzach — the turning palm of providence, moving
    // from Mercy to Victory.
    // Sachiel (also Zadkiel / Tzadkiel) is the Archangel of Jupiter; the
    // name means "covering of God" and he governs abundance and good fortune.
    MajorArcanum {
        number: 10,
        name: "Wheel of Fortune",
        hebrew_letter: "Kaph (כַּף) · כ · Palm of Hand",
        path: 21,
        sephiroth: "Chesed (Mercy) ─── Netzach (Victory)",
        element: "Jupiter (Tzedek — צֶדֶק)",
        angel: "Sachiel",
        angelic_order: "Cherubim",
        divine_quality: "Covering of God; the cosmic providence that turns all cycles",
        keywords: "Fate · cosmic cycles · turning points · fortune · divine providence",
    },

    // ── 11. Justice ───────────────────────────────────────────────────────────
    // Lamed = ox-goad; simple letter; Libra (Moznaim מֹאזְנַיִם).
    // Path 22: Geburah to Tiphareth — the ox-goad of divine law, directing
    // Severity toward the illumined Heart.
    // Zuriel governs Libra; the name means "God is my rock / foundation."
    MajorArcanum {
        number: 11,
        name: "Justice",
        hebrew_letter: "Lamed (לָמֶד) · ל · Ox-goad",
        path: 22,
        sephiroth: "Geburah (Severity) ─── Tiphareth (Beauty)",
        element: "Libra ♎ (Moznaim — מֹאזְנַיִם)",
        angel: "Zuriel",
        angelic_order: "Thrones",
        divine_quality: "God is my rock; divine equilibrium, the scales of cosmic truth",
        keywords: "Justice · truth · cause and effect · balance · the law of return",
    },

    // ── 12. The Hanged Man ────────────────────────────────────────────────────
    // Mem = water; second Mother letter; Water (Mayim מַיִם).
    // Path 23: Geburah to Hod — the waters of surrender flowing from
    // Severity to Splendour.
    // Gabriel as Archangel of Water and Moon presides; in the Zohar
    // (III:215a) Gabriel stands at the left, the side of judgement made
    // gentle by surrender.
    MajorArcanum {
        number: 12,
        name: "The Hanged Man",
        hebrew_letter: "Mem (מֵם) · מ · Water",
        path: 23,
        sephiroth: "Geburah (Severity) ─── Hod (Splendour)",
        element: "Water (Mother letter — Mayim מַיִם)",
        angel: "Gabriel",
        angelic_order: "Dominions",
        divine_quality: "Strength of God; the gift of holy suspension, seeing anew in stillness",
        keywords: "Surrender · new perspective · sacred pause · sacrifice · reversal",
    },

    // ── 13. Death ─────────────────────────────────────────────────────────────
    // Nun = fish; simple letter; Scorpio (Akrav עַקְרָב).
    // Path 24: Tiphareth to Netzach — the fish swimming from Beauty toward
    // Victory, traversing the depths of transformation.
    // Azrael is the Angel of Death in Islamic and later Kabbalistic sources;
    // in Hermetic Tarot the transformer is sometimes Samael, but Azrael's
    // role as the gentle escort of souls is most consonant with this card's
    // emphasis on transition rather than destruction.
    MajorArcanum {
        number: 13,
        name: "Death",
        hebrew_letter: "Nun (נוּן) · נ · Fish",
        path: 24,
        sephiroth: "Tiphareth (Beauty) ─── Netzach (Victory)",
        element: "Scorpio ♏ (Akrav — עַקְרָב)",
        angel: "Azrael",
        angelic_order: "Powers",
        divine_quality: "The divine escort; sacred transformation that liberates the immortal soul",
        keywords: "Transformation · endings and beginnings · rebirth · the eternal cycle",
    },

    // ── 14. Temperance ────────────────────────────────────────────────────────
    // Samekh = prop / tent-peg; simple letter; Sagittarius (Keshet קֶשֶׁת).
    // Path 25: Tiphareth to Yesod — the prop supporting the Heart above
    // the Foundation; the arrow of the centaur aimed upward.
    // Adnachiel (Advachiel / Adnakhiel) governs Sagittarius; named in
    // Barrett's *The Magus* (1801) and Agrippa; associated with temperance,
    // aspiration, and righteous travel.
    MajorArcanum {
        number: 14,
        name: "Temperance",
        hebrew_letter: "Samekh (סָמֶךְ) · ס · Prop / Tent-peg",
        path: 25,
        sephiroth: "Tiphareth (Beauty) ─── Yesod (Foundation)",
        element: "Sagittarius ♐ (Keshet — קֶשֶׁת)",
        angel: "Adnachiel",
        angelic_order: "Virtues",
        divine_quality: "Sacred alchemy; the flowing blend of fire and water into holy harmony",
        keywords: "Balance · moderation · patience · alchemy · the middle path",
    },

    // ── 15. The Devil ─────────────────────────────────────────────────────────
    // Ayin = eye; simple letter; Capricorn (Gedi גְּדִי).
    // Path 26: Tiphareth to Hod — the eye of matter, the descent from
    // Beauty to Splendour through the illusion of material bondage.
    // Hanael (Hamael) governs Capricorn; the name is related to "grace of
    // God" in a contractive, Saturnine aspect.  The card reveals the
    // angel behind the illusion of bondage — the chains of matter are
    // self-forged and self-released.
    MajorArcanum {
        number: 15,
        name: "The Devil",
        hebrew_letter: "Ayin (עַיִן) · ע · Eye",
        path: 26,
        sephiroth: "Tiphareth (Beauty) ─── Hod (Splendour)",
        element: "Capricorn ♑ (Gedi — גְּדִי)",
        angel: "Hanael",
        angelic_order: "Principalities",
        divine_quality: "The eye that sees through illusion; guardian of the shadow and its gifts",
        keywords: "Shadow self · material bondage · illusion · raw power · liberation through awareness",
    },

    // ── 16. The Tower ─────────────────────────────────────────────────────────
    // Peh = mouth; double letter; Mars (Ma'adim מַאֲדִים).
    // Path 27: Netzach to Hod — the mouth of God that utters a word of
    // sudden truth, shattering the tower of false certainty.
    // Camael (Khamael) is the Archangel of Mars/Geburah in Agrippa
    // (Book III, ch. 24); the name means "he who sees God" — divine
    // vision that demolishes illusion with a single stroke.
    MajorArcanum {
        number: 16,
        name: "The Tower",
        hebrew_letter: "Peh (פֵּא) · פ · Mouth",
        path: 27,
        sephiroth: "Netzach (Victory) ─── Hod (Splendour)",
        element: "Mars (Ma'adim — מַאֲדִים)",
        angel: "Camael",
        angelic_order: "Archangels",
        divine_quality: "He who sees God; the divine lightning that demolishes false structures",
        keywords: "Sudden change · upheaval · revelation · liberation · the flash of truth",
    },

    // ── 17. The Star ──────────────────────────────────────────────────────────
    // Tzaddi = fish-hook; simple letter; Aquarius (Deli דְּלִי).
    // Path 28: Netzach to Yesod — the hook that draws from Victory into
    // the Foundation, filling the vessel with starlight.
    // Cambriel (Cambiel) governs Aquarius; associated with inspiration,
    // humanism, and the gift of hope carried on stellar winds.
    MajorArcanum {
        number: 17,
        name: "The Star",
        hebrew_letter: "Tzaddi (צַדִּי) · צ · Fish-hook",
        path: 28,
        sephiroth: "Netzach (Victory) ─── Yesod (Foundation)",
        element: "Aquarius ♒ (Deli — דְּלִי)",
        angel: "Cambriel",
        angelic_order: "Angels",
        divine_quality: "Celestial inspiration; the endless outpouring of divine hope and renewal",
        keywords: "Hope · renewal · inspiration · divine guidance · the healing light",
    },

    // ── 18. The Moon ──────────────────────────────────────────────────────────
    // Qoph = back of head; simple letter; Pisces (Dagim דָּגִים).
    // Path 29: Netzach to Malkuth — the deepest inner pool, the primal
    // ocean from Victory down into the Kingdom of the manifest world.
    // Amnitziel (Amnediel) governs Pisces in the GD system; presides over
    // the tides of illusion and the dissolution into the oceanic unconscious.
    MajorArcanum {
        number: 18,
        name: "The Moon",
        hebrew_letter: "Qoph (קוֹף) · ק · Back of the Head",
        path: 29,
        sephiroth: "Netzach (Victory) ─── Malkuth (Kingdom)",
        element: "Pisces ♓ (Dagim — דָּגִים)",
        angel: "Amnitziel",
        angelic_order: "Seraphim",
        divine_quality: "Guardian of the lunar tides; angel of sacred dreams and deep waters",
        keywords: "Illusion · the unconscious · hidden fears · dreams · the veil of Maya",
    },

    // ── 19. The Sun ───────────────────────────────────────────────────────────
    // Resh = head; double letter; Sun (Shemesh שֶׁמֶשׁ).
    // Path 30: Hod to Yesod — the great radiant head illumining the way
    // from Splendour to Foundation.
    // Michael is the Archangel of the Sun and of Tiphareth (the Heart of
    // the Tree); his name means "Who is like God?" — a declaration of the
    // divine image radiating through created beings.
    MajorArcanum {
        number: 19,
        name: "The Sun",
        hebrew_letter: "Resh (רֵישׁ) · ר · Head",
        path: 30,
        sephiroth: "Hod (Splendour) ─── Yesod (Foundation)",
        element: "Sun (Shemesh — שֶׁמֶשׁ)",
        angel: "Michael",
        angelic_order: "Cherubim",
        divine_quality: "Who is like God? The divine radiance burning away shadow with pure light",
        keywords: "Joy · vitality · clarity · success · the life-giving solar truth",
    },

    // ── 20. Judgement ─────────────────────────────────────────────────────────
    // Shin = tooth; third Mother letter; Fire (Esh אֵשׁ).
    // Path 31: Hod to Malkuth — the divine fire descending from Splendour
    // into the Kingdom, sounding the trumpet of awakening.
    // Uriel ("Fire of God") is often named as the archangel of the South
    // (Fire) in the Lesser Banishing Ritual of the Pentagram and in
    // Pseudo-Dionysius; he is the angel of revelation, repentance, and
    // the final call that raises the dead to new life.
    MajorArcanum {
        number: 20,
        name: "Judgement",
        hebrew_letter: "Shin (שִׁין) · ש · Tooth / Flame",
        path: 31,
        sephiroth: "Hod (Splendour) ─── Malkuth (Kingdom)",
        element: "Fire (Mother letter — Esh אֵשׁ)",
        angel: "Uriel",
        angelic_order: "Thrones",
        divine_quality: "Fire of God; the trumpet of divine awakening, calling all souls home",
        keywords: "Awakening · rebirth · inner calling · absolution · the sacred summons",
    },

    // ── 21. The World ─────────────────────────────────────────────────────────
    // Tav = cross / mark; double letter; Saturn (Shabbatai שַׁבְּתַאי).
    // Path 32: Yesod to Malkuth — the final seal, the mark of completion,
    // the crossing from Foundation into the fully manifested Kingdom.
    // Cassiel (Tzaphkiel in some systems) is the Archangel of Saturn and
    // Binah; the name means "speed of God" or "tears of God."  As lord of
    // time and completion Cassiel witnesses the great dancer at the centre
    // of creation.
    MajorArcanum {
        number: 21,
        name: "The World",
        hebrew_letter: "Tav (תָּו) · ת · Cross / Mark",
        path: 32,
        sephiroth: "Yesod (Foundation) ─── Malkuth (Kingdom)",
        element: "Saturn (Shabbatai — שַׁבְּתַאי)",
        angel: "Cassiel",
        angelic_order: "Dominions",
        divine_quality: "Speed of God; witness of completion, the cosmic dance at the centre of all",
        keywords: "Completion · integration · cosmic wholeness · the perfected soul · fulfilment",
    },
];

// ─── Lookup helpers ───────────────────────────────────────────────────────────

/// Find a Major Arcanum by card number (0–21).
pub fn major_by_number(n: u8) -> Option<&'static MajorArcanum> {
    MAJOR_ARCANA.iter().find(|c| c.number == n)
}

/// Find a Major Arcanum by name (case-insensitive, partial match).
pub fn major_by_name(query: &str) -> Option<&'static MajorArcanum> {
    let q = query.to_lowercase();
    MAJOR_ARCANA
        .iter()
        .find(|c| c.name.to_lowercase().contains(&q))
}
