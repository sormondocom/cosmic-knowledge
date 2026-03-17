//! Elder Futhark — the oldest attested Germanic runic alphabet.
//!
//! The Elder Futhark (*futark* from the phonemic values of the first six
//! runes: ᚠ ᚢ ᚦ ᚨ ᚱ ᚲ) is a 24-rune alphabet documented in inscriptions
//! across the Germanic world from roughly 150 CE to 800 CE.  Examples range
//! from the Vimose comb (Denmark, c. 160 CE) to the Ruthwell Cross
//! (Northumbria, c. 700 CE).  It is the ancestor of all later runic
//! alphabets and constitutes the primary tradition for modern runic study
//! and esoteric practice.
//!
//! ## Organisation — the three Aettir
//!
//! The 24 runes are divided into three groups of eight called *aettir*
//! ("families" or "eights").  This tripartite structure is attested in
//! manuscript sources and is foundational to runic esotericism:
//!
//! | Aett | Name              | Runes 1–8       | Deity patron |
//! |------|-------------------|-----------------|--------------|
//! | 1    | Freyr's Aett      | ᚠ ᚢ ᚦ ᚨ ᚱ ᚲ ᚷ ᚹ | Freyr/Freyja |
//! | 2    | Hagal's Aett      | ᚺ ᚾ ᛁ ᛃ ᛇ ᛈ ᛉ ᛊ | Heimdall/Hagal |
//! | 3    | Tyr's Aett        | ᛏ ᛒ ᛖ ᛗ ᛚ ᛜ ᛞ ᛟ | Tyr          |
//!
//! ## Sources
//!
//! - Antonsen, Elmer H. *Runes and Germanic Linguistics*. Berlin:
//!   Mouton de Gruyter, 2002.
//! - Elliott, Ralph W.V. *Runes: An Introduction*, 2nd ed. Manchester:
//!   Manchester University Press, 1989.
//! - Flowers, Stephen E. (Edred Thorsson). *Futhark: A Handbook of Rune
//!   Magic*. York Beach, ME: Weiser Books, 1984.
//! - Looijenga, Tineke. *Texts and Contexts of the Oldest Runic
//!   Inscriptions*. Leiden: Brill, 2003.
//! - Page, R.I. *An Introduction to English Runes*, 2nd ed. Woodbridge:
//!   Boydell Press, 1999.
//! - Paxson, Diana L. *Taking Up the Runes*. San Francisco: Weiser, 2005.
//! - Spurkland, Terje. *Norwegian Runes and Runic Inscriptions*.
//!   Woodbridge: Boydell Press, 2005.
//!
//! Rune poem translations follow Halsall (1981) for the Old English Rune
//! Poem, and Page (1999) for the Old Norwegian and Old Icelandic poems.
//! Stanzas are abbreviated; bracketed numbers indicate the stanza position
//! in the source poem.

use super::Rune;

/// The 24 runes of the Elder Futhark in canonical order.
pub static ELDER_FUTHARK: &[Rune] = &[
    // ═══════════════════════════════════════════════════════════════════════
    //  FREYR'S AETT  (runes 1–8)
    // ═══════════════════════════════════════════════════════════════════════
    Rune {
        number: 1,
        name: "Fehu",
        alt_names: "Feoh (OE), Fé (ON/OI), Fe (Gothic)",
        glyph: "ᚠ",
        phoneme: "f",
        etymology: "Proto-Germanic *fehu — 'cattle, mobile wealth, possessions.' \
            Cognate with Latin *pecus* (cattle) and *pecunia* (money), reflecting \
            the ancient equivalence of livestock and wealth.",
        aett: 1,
        aett_name: "Freyr's Aett",
        rune_poem_oe: "OE [st. 1]: 'Wealth (Feoh) is a comfort to every man, \
            yet every man must bestow it freely if he wish to gain honour in \
            the sight of the Lord.' (Halsall 1981)",
        rune_poem_on: "ON [st. 1]: 'Wealth (Fé) causes strife among kinsmen; \
            the wolf grows up in the forest.' (Page 1999)",
        rune_poem_oi: "OI [st. 1]: 'Wealth (Fé) — source of discord among \
            kinsmen, fire of the sea, path of the serpent.' (Page 1999)",
        deity: "Freyr, Njord",
        element: "Fire",
        world: "Muspellsheim / Vanaheim",
        esoteric: "Fehu is the primordial creative fire—the mobile, \
            self-generating energy that animates existence. As cattle, it \
            represents circulating wealth that creates community through gift \
            exchange; hoarded, it becomes stagnant and destructive. In runic \
            esotericism (Thorsson 1984) it corresponds to the solar fire of \
            Muspell and the fehu-force: the raw vital energy that initiates \
            all magical work. It opens the Elder Futhark as the first breath \
            of creation.",
        meaning_upright: "Prosperity, earned wealth, success in material endeavours, \
            new beginnings, creative energy, self-sufficiency, fulfilment.",
        meaning_reversed: "Financial loss, greed, the consequences of hoarding, \
            discord over property, energy squandered.",
    },
    Rune {
        number: 2,
        name: "Uruz",
        alt_names: "Ur (OE/ON), Úr (OI), Urus",
        glyph: "ᚢ",
        phoneme: "u",
        etymology: "Proto-Germanic *ūruz — 'aurochs' (*Bos primigenius*), the \
            massive wild ox of prehistoric Europe. The aurochs was extinct in \
            Scandinavia by the Viking Age; the rune preserved its archetype. \
            In the Younger Futhark the word *úr* shifted to mean 'drizzle, \
            slag' (a different word), illustrating semantic drift.",
        aett: 1,
        aett_name: "Freyr's Aett",
        rune_poem_oe: "OE [st. 2]: 'The aurochs (Ur) is proud and has great \
            horns; it is a very savage beast and fights with its horns; a \
            great ranger of the moors, it is a creature of mettle.' \
            (Halsall 1981)",
        rune_poem_on: "ON [st. 2]: 'Dross (Ur) comes from bad iron; the \
            reindeer often races over the frozen snow.' (Page 1999) \
            [Note: the ON poem reflects the semantic shift to *úr* = slag/drizzle.]",
        rune_poem_oi: "OI [st. 2]: 'Shower (Ur) — weeping of the clouds, \
            ruin of the hay-harvest, hatred of the herdsman.' (Page 1999)",
        deity: "Thor, the primordial Aurochs archetype",
        element: "Earth",
        world: "Midgard / Jotunheim",
        esoteric: "Uruz embodies *megin* — the primal physical vitality that \
            underlies all health and manifestation. The aurochs represented \
            the wild, untamed masculine power that must be overcome and \
            internalised. In Norse cosmology the first being, Ymir, was \
            nourished by the primordial cow Audhumbla; Uruz connects to this \
            originary nourishing force. Magically it is used for healing, \
            strengthening the body, and shaping new circumstances with \
            the strength of will.",
        meaning_upright: "Physical strength, health, raw vitality, primal power, \
            perseverance, courage, the ability to shape new realities.",
        meaning_reversed: "Weakness, illness, missed opportunities, impotence, \
            misdirected or uncontrolled force.",
    },
    Rune {
        number: 3,
        name: "Thurisaz",
        alt_names: "Thorn / Þorn (OE), Thurs (ON), Þurs (OI)",
        glyph: "ᚦ",
        phoneme: "þ (th, as in 'thorn')",
        etymology: "Proto-Germanic *þurisaz — 'giant, thurse.' The Norse *þurs* \
            (giant, particularly a destructive giant) is the primary meaning. \
            The Old English kenning *þorn* (thorn) is a secondary image. \
            The rune gave its name to the Old English letter Thorn (Þ), \
            which survived into Middle English.",
        aett: 1,
        aett_name: "Freyr's Aett",
        rune_poem_oe: "OE [st. 3]: 'The thorn (Thorn) is very sharp; for \
            every thane who seizes it, it is harmful, and uncommonly severe \
            to every man who lies among them.' (Halsall 1981)",
        rune_poem_on: "ON [st. 3]: 'Giant (Thurs) causes anguish to women; \
            misfortune makes few men cheerful.' (Page 1999)",
        rune_poem_oi: "OI [st. 3]: 'Giant (Thurs) — torment of women, \
            cliff-dweller, husband of a giantess, Saturn's thane.' \
            (Page 1999)",
        deity: "Thor (as slayer of the Thurses), Loki",
        element: "Fire / Ice (chaotic, boundary-crossing)",
        world: "Jotunheim",
        esoteric: "Thurisaz is the force of directed, potentially destructive \
            power — the thorn that wounds the unwary but protects the plant. \
            It embodies the principle of resistance, gateway-breaking, and \
            the active male principle in its most raw, unchannelled form. \
            Thor's hammer Mjolnir is a Thurisaz-force made purposeful. \
            In runic magic it is used to create barriers, drive away \
            harmful forces, and direct concentrated energy at obstacles.",
        meaning_upright: "Directed force, breaching obstacles, protection through \
            danger, catalytic change, the power of Thor, a gateway or \
            threshold moment.",
        meaning_reversed: "Uncontrolled aggression, danger, malice, vulnerability \
            to attack, recklessness.",
    },
    Rune {
        number: 4,
        name: "Ansuz",
        alt_names: "Os (OE), Óss / Áss (OI), Oss (ON)",
        glyph: "ᚨ",
        phoneme: "a",
        etymology: "Proto-Germanic *ansuz — 'a god, one of the Æsir.' Directly \
            cognate with Old Norse *áss* (pl. *æsir*), Old English *os* (god, \
            divine being). The rune is supremely associated with Odin, \
            the sovereign god of speech, wisdom, and runic knowledge.",
        aett: 1,
        aett_name: "Freyr's Aett",
        rune_poem_oe: "OE [st. 4]: 'The mouth (Os) is the source of all \
            language, a pillar of wisdom and a comfort to wise men, \
            a blessing and a joy to every nobleman.' (Halsall 1981)",
        rune_poem_on: "ON [st. 4]: 'Estuary (Óss) is the way of most \
            journeys; but a scabbard is of swords.' (Page 1999) \
            [Note: ON *óss* = river-mouth, illustrating the semantic drift \
            from 'god' to 'estuary' in the Younger Futhark.]",
        rune_poem_oi: "OI [st. 4]: 'God (Áss) — aged Gautr [Odin] and prince \
            of Ásgarðr, and lord of Valholl.' (Page 1999)",
        deity: "Odin (Gautr, Ygg, Allfather)",
        element: "Air",
        world: "Asgard",
        esoteric: "Ansuz is the rune of Odin and divine communication. It \
            represents the Word (*óðr*, inspiration) breathed into the first \
            humans by the gods. The *óðr* — the divine ecstatic force — is \
            the source of poetry, prophecy, and runic wisdom. Odin hung for \
            nine days on Yggdrasil to win the runes; Ansuz is the rune he \
            seized first. Magically it governs all work involving speech, \
            persuasion, consciousness expansion, ancestral communication, \
            and the awakening of divine intelligence.",
        meaning_upright: "Divine inspiration, communication, wisdom, poetic \
            insight, ancestral knowledge, the power of the spoken word, \
            Odin's gift.",
        meaning_reversed: "Miscommunication, deception, manipulation, abuse of \
            eloquence, blocked inspiration.",
    },
    Rune {
        number: 5,
        name: "Raidho",
        alt_names: "Rad (OE), Reið (ON/OI), Raido",
        glyph: "ᚱ",
        phoneme: "r",
        etymology: "Proto-Germanic *raidō — 'ride, journey, vehicle.' \
            Cognate with Old Norse *reið* (riding, vehicle), Old English \
            *rād* (road, riding), and ultimately with modern English 'road.' \
            The cosmological sense extends to the wheeling of the heavens.",
        aett: 1,
        aett_name: "Freyr's Aett",
        rune_poem_oe: "OE [st. 5]: 'Riding (Rad) seems easy to every warrior \
            while he is indoors and very courageous to him who traverses \
            the high-roads on the back of a stout horse.' (Halsall 1981)",
        rune_poem_on: "ON [st. 5]: 'Riding (Reið) is said to be the worst \
            thing for horses; Reginn forged the finest sword.' (Page 1999)",
        rune_poem_oi: "OI [st. 5]: 'Riding (Reið) — joy of the horsemen \
            and speedy journey and toil of the steed.' (Page 1999)",
        deity: "Thor (as sky-rider), Odin (as wanderer)",
        element: "Air",
        world: "Midgard (all roads, the cosmic wheel)",
        esoteric: "Raidho is the cosmic order of movement — not merely \
            physical travel but the right action (*recht*) that aligns \
            individual will with cosmic rhythm. The wheeling of the heavens, \
            the solar chariot, the journey of the soul — all fall under \
            Raidho. It is the rune of the *Thing* (assembly), of just \
            judgement, and of ritual procession. In magic it governs \
            journeys, correct timing, soul-travel, and alignment with \
            cosmic law.",
        meaning_upright: "Journey, movement, right action, cosmic order, \
            adventure, good judgement, ritual procession, alignment.",
        meaning_reversed: "Crisis during travel, poor timing, irrationality, \
            disrupted plans, injustice.",
    },
    Rune {
        number: 6,
        name: "Kenaz",
        alt_names: "Cen (OE), Kaun (ON/OI), Kaunan, Ken",
        glyph: "ᚲ",
        phoneme: "k",
        etymology: "Proto-Germanic *kaunan — 'torch, ulcer.' Two distinct \
            words converge here: Old English *cen* (torch, knowledge) and \
            Old Norse *kaun* (ulcer, sore). The former governs the Elder \
            Futhark esoteric tradition; the latter is attested in the \
            Younger Futhark poems, illustrating a dramatic semantic shift.",
        aett: 1,
        aett_name: "Freyr's Aett",
        rune_poem_oe: "OE [st. 6]: 'The torch (Cen) is known to every \
            living man by its pale, bright flame; it always burns where \
            princes sit within.' (Halsall 1981)",
        rune_poem_on: "ON [st. 6]: 'Ulcer (Kaun) is fatal to children; \
            death makes a corpse pale.' (Page 1999) \
            [Note: The ON poem reflects the shifted meaning to *kaun* = sore/ulcer.]",
        rune_poem_oi: "OI [st. 6]: 'Ulcer (Kaun) — disease fatal to children \
            and painful spot and abode of mortification.' (Page 1999)",
        deity: "Heimdall (as bringer of civilisational fire), Freyja",
        element: "Fire",
        world: "Midgard / Muspellsheim",
        esoteric: "Kenaz is the controlled fire of human civilisation — the \
            forge-fire, the hearth, and the inner light of knowledge. Where \
            Fehu is wild solar fire, Kenaz is fire harnessed: the torch \
            by which the craftsman works, the inner illumination of the \
            artist and lover. It governs craft (*list*), passionate love, \
            regeneration after illness, and the opening of blocked or hidden \
            knowledge. Its glyph, a simple angle, recalls both a torch and \
            a craftsman's chisel.",
        meaning_upright: "Illumination, knowledge, creative fire, passion, \
            craft, regeneration, the opening of hidden things, controlled \
            transformation.",
        meaning_reversed: "Loss of direction, extinguished inspiration, \
            false light, disease, arrogance.",
    },
    Rune {
        number: 7,
        name: "Gebo",
        alt_names: "Gyfu (OE), Gipt (Gothic), Geofu",
        glyph: "ᚷ",
        phoneme: "g",
        etymology: "Proto-Germanic *gebō — 'gift.' Cognate with Old English \
            *gyfu* (gift), Old High German *geба*. Note: Gebo does not appear \
            in the Younger Futhark (it was dropped as the alphabet contracted \
            from 24 to 16 runes) or in the Old Norwegian and Icelandic Rune \
            Poems. It does appear in the Old English Rune Poem.",
        aett: 1,
        aett_name: "Freyr's Aett",
        rune_poem_oe: "OE [st. 7]: 'Generosity (Gyfu) brings credit and \
            honour, which support one's dignity; it is an adornment and \
            a mark of esteem, the support and subsistence of all the \
            destitute.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Odin (as gift-giver), the Vanir (as gods of exchange)",
        element: "Air / All elements (gift crosses all boundaries)",
        world: "All Nine Worlds (the rune of connection between them)",
        esoteric: "Gebo is the sacred law of reciprocity. The Norse gift \
            economy (*gáfu*) was the structural basis of social bonds: gift \
            creates obligation, obligation creates relationship, relationship \
            creates community. The gods themselves operate by gift — Odin \
            sacrificed his eye for wisdom. Gebo's X-shape represents the \
            crossing of forces, the meeting of two trajectories, and \
            sacred union. It cannot be reversed, as a gift given is always \
            in motion. Magically it governs partnership, sacred sexuality, \
            initiation, and the equilibrium of exchange.",
        meaning_upright: "Gift, partnership, exchange, sacred union, \
            generosity, balance, magical initiation, self-sacrifice \
            for a higher purpose.",
        meaning_reversed: "(Gebo has no reversed meaning — it is \
            symmetrical and cannot be inverted.)",
    },
    Rune {
        number: 8,
        name: "Wunjo",
        alt_names: "Wynn (OE), Wunjo, Vend",
        glyph: "ᚹ",
        phoneme: "w",
        etymology: "Proto-Germanic *wunjō — 'joy, pleasure, perfection, \
            glory.' Cognate with Old English *wynn* (joy, delight), \
            Old High German *wunna*. The root connects to Proto-Germanic \
            *wunōną* (to accustom oneself, to dwell) — joy as the \
            rightness of being at home. Wunjo does not appear in the \
            Younger Futhark.",
        aett: 1,
        aett_name: "Freyr's Aett",
        rune_poem_oe: "OE [st. 8]: 'Bliss (Wynn) is had by one who \
            knows few troubles, pains and sorrows, and to him who himself \
            has power and blessedness and a good enough house.' \
            (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Odin (as *Wunsc*, the Wishing-God), Freyr",
        element: "Earth / Air",
        world: "Asgard / Vanaheim (the worlds of the blessed)",
        esoteric: "Wunjo is the rune of achieved harmony — the joy that \
            arises when the individual will aligns perfectly with cosmic \
            order and one's true kindred. It represents the fellowship of \
            the war-band (*comitatus*), the clan gathered in the mead-hall, \
            and the condition of spiritual fulfilment. Wunjo concludes \
            Freyr's Aett and resolves the tension of the preceding runes \
            into a state of radiant well-being. Magically it is used for \
            generating happiness, strengthening group bonds, and \
            manifesting wish-fulfilment.",
        meaning_upright: "Joy, harmony, fellowship, success, wish-fulfilment, \
            alignment with one's true will, spiritual well-being.",
        meaning_reversed: "Sorrow, alienation, frenzy, disharmony, \
            false happiness, delirium (*wod*).",
    },
    // ═══════════════════════════════════════════════════════════════════════
    //  HAGAL'S AETT  (runes 9–16)
    // ═══════════════════════════════════════════════════════════════════════
    Rune {
        number: 9,
        name: "Hagalaz",
        alt_names: "Hægl (OE), Hagall (ON), Hagall (OI)",
        glyph: "ᚺ",
        phoneme: "h",
        etymology: "Proto-Germanic *haglaz — 'hail (precipitation).' \
            Cognate with Old Norse *hagl*, Old English *hægl*. The \
            Younger Futhark form is *hagall* (hail). This rune opens \
            the second aett and represents the disruption that initiates \
            transformation.",
        aett: 2,
        aett_name: "Hagal's Aett",
        rune_poem_oe: "OE [st. 9]: 'Hail (Hægl) is the whitest of grains; \
            it comes whirling from the vault of heaven and is tossed about \
            by gusts of wind and then it melts into water.' (Halsall 1981)",
        rune_poem_on: "ON [st. 7]: 'Hail (Hagall) is the coldest of grains; \
            Odin shaped the world in ancient times.' (Page 1999)",
        rune_poem_oi: "OI [st. 7]: 'Hail (Hagall) — cold grain and shower \
            of sleet and sickness of serpents.' (Page 1999)",
        deity: "Heimdall, Ymir (as the cosmic pattern of ice)",
        element: "Ice / Water",
        world: "Niflheim",
        esoteric: "Hagalaz is the seed of cosmic pattern — the hailstone \
            that is hard, cold, and disruptive but melts to become \
            life-giving water. In Germanic cosmology, the interaction of \
            Muspellsheim (fire) and Niflheim (ice) produced the first \
            being; Hagalaz encodes this cosmic matrix. It is the rune of \
            the *hagal* model: the hexagonal pattern underlying all \
            natural form. Magically it initiates unavoidable but \
            transformative disruption — the clearing storm that makes \
            room for new growth.",
        meaning_upright: "Disruption, elemental forces, unavoidable change, \
            the cosmic matrix, transformation through crisis, the clearing \
            storm.",
        meaning_reversed: "(Hagalaz is often held to have no reversed position \
            due to its symmetry, though some practitioners read it as \
            intensified crisis or chaos.)",
    },
    Rune {
        number: 10,
        name: "Nauthiz",
        alt_names: "Nyd (OE), Nauðr (ON/OI), Naudiz",
        glyph: "ᚾ",
        phoneme: "n",
        etymology: "Proto-Germanic *naudiz — 'need, necessity, constraint, \
            distress.' Cognate with Old Norse *nauðr* (need, necessity, \
            compulsion), Old English *nyd* (need, compulsion). The rune \
            form depicts two crossed staves — the fire-bow drill — \
            evoking the generation of necessity-fire (*naudfire*), \
            used in ritual to drive out plague.",
        aett: 2,
        aett_name: "Hagal's Aett",
        rune_poem_oe: "OE [st. 10]: 'Need (Nyd) is oppressive to the heart; \
            yet often it proves a source of help and salvation to the sons \
            of men, to everyone who heeds it in time.' (Halsall 1981)",
        rune_poem_on: "ON [st. 8]: 'Constraint (Nauðr) gives scant choice; \
            a naked man is chilled by frost.' (Page 1999)",
        rune_poem_oi: "OI [st. 8]: 'Constraint (Nauðr) — grief of the \
            bond-maid and state of oppression and toilsome work.' \
            (Page 1999)",
        deity: "The Norns (Urðr, Verðandi, Skuld) — as weavers of fate",
        element: "Fire (necessity-fire, *naudfire*)",
        world: "Niflheim / the realm of the Norns",
        esoteric: "Nauthiz is the rune of fate-as-constraint: the need that \
            cannot be avoided, the friction that generates transformation. \
            The Norns weave *wyrd* (fate) at the Well of Urðr; Nauthiz is \
            their tool. Its glyph depicts the crossed fire-sticks used to \
            kindle *naudfire* — the emergency sacred fire lit during crises. \
            Paradoxically, Nauthiz teaches that recognising necessity \
            is the first step to freedom. Magically it is used to identify \
            what is truly needed, to resist temptation, and to work \
            patiently within constraint.",
        meaning_upright: "Need, necessity, constraint, patience under \
            pressure, the wisdom within hardship, recognising what \
            is truly required.",
        meaning_reversed: "Slavery to compulsion, wrong desire, misidentified \
            needs, destructive constraint, unrecognised pattern.",
    },
    Rune {
        number: 11,
        name: "Isa",
        alt_names: "Is (OE), Íss (OI), Iss (ON)",
        glyph: "ᛁ",
        phoneme: "i",
        etymology: "Proto-Germanic *isaz — 'ice.' Cognate with Old Norse \
            *íss*, Old English *is*, Gothic *eis*. The starkest, most \
            elemental rune — a single vertical stroke, like an icicle \
            or the world-axis frozen.",
        aett: 2,
        aett_name: "Hagal's Aett",
        rune_poem_oe: "OE [st. 11]: 'Ice (Is) is very cold and immeasurably \
            slippery; it glistens as clear as glass and most like to gems; \
            it is a floor wrought by the frost, fair to look upon.' \
            (Halsall 1981)",
        rune_poem_on: "ON [st. 9]: 'Ice (Iss) we call the broad bridge; \
            the blind man must be led.' (Page 1999)",
        rune_poem_oi: "OI [st. 9]: 'Ice (Íss) — bark of rivers and roof \
            of the wave and destruction of the doomed.' (Page 1999)",
        deity: "Rind (goddess of frozen earth), Niflheim's rulers",
        element: "Ice / Water (frozen)",
        world: "Niflheim",
        esoteric: "Isa is the primordial ice of Niflheim — pure, crystalline \
            stasis. It is the still point at the centre of the cosmic axis. \
            All dynamic forces freeze under Isa; it is concentration, \
            contraction, and the withdrawal of the ego into itself. \
            Like Hagalaz, it derives from the primordial ice, but where \
            Hagalaz is the disruptive hailstone, Isa is unmoving permanence. \
            Magically it halts unwanted processes, creates stillness for \
            meditation, and strengthens the self's boundaries. It cannot \
            be reversed — perfect ice has no orientation.",
        meaning_upright: "Stillness, halt, concentration, introspection, \
            self-discipline, the pause before action, winter.",
        meaning_reversed: "(Isa is symmetrical and is generally not reversed; \
            some traditions read it as blocked energy or dangerous \
            stagnation.)",
    },
    Rune {
        number: 12,
        name: "Jera",
        alt_names: "Ger (OE), Ár (ON/OI), Yer, Gaar",
        glyph: "ᛃ",
        phoneme: "j (y-sound)",
        etymology: "Proto-Germanic *jēran — 'year, (good) harvest.' \
            Cognate with Old Norse *ár* (year, harvest), Old English *gēr* \
            (year). The annual cycle of planting, tending, and harvest. \
            The Younger Futhark contracted this to *ár* (harvest, year), \
            preserving the agricultural sense.",
        aett: 2,
        aett_name: "Hagal's Aett",
        rune_poem_oe: "OE [st. 12]: 'Summer (Ger) is a joy to men, when \
            God, the holy King of Heaven, suffers the earth to bring \
            forth shining fruits for rich and poor alike.' (Halsall 1981)",
        rune_poem_on: "ON [st. 10]: 'Harvest (Ár) is a boon to men; \
            I say that Fróðr [the king of golden peace] was generous.' \
            (Page 1999)",
        rune_poem_oi: "OI [st. 10]: 'Harvest (Ár) — boon of men and good \
            summer and thriving crops.' (Page 1999)",
        deity: "Freyr (as harvest-god), Odin (as year-god)",
        element: "Earth",
        world: "Midgard / Vanaheim",
        esoteric: "Jera is the rune of the sacred year-cycle — the slow, \
            patient revolution that brings all things to fruition in their \
            proper time. It cannot be rushed; it cannot be reversed. \
            The two halves of the rune's glyph represent the interlocking \
            opposites — summer/winter, sowing/reaping — that together \
            constitute the living year. Magically it governs patience, \
            right timing, legal matters (the *þing* meets annually), \
            and manifesting the results of previous effort.",
        meaning_upright: "The harvest, reward for past effort, right timing, \
            patience, the yearly cycle, legal success, earned fruition.",
        meaning_reversed: "(Jera has no reversed meaning — as the year-cycle \
            it is complete in itself and turns without reversal.)",
    },
    Rune {
        number: 13,
        name: "Eihwaz",
        alt_names: "Eoh (OE), Yr (YF), Ihwaz, Eiwaz",
        glyph: "ᛇ",
        phoneme: "ei / ï (a long palatal vowel or ʀ in some analyses)",
        etymology: "Proto-Germanic *eihwaz — 'yew tree (*Taxus baccata*).' \
            Cognate with Old English *eoh* (yew), Old High German *iwa*. \
            The yew was the pre-eminent sacred tree of death and rebirth \
            in the Germanic world — evergreen, poisonous, yet extraordinarily \
            long-lived (some specimens exceed 5,000 years).",
        aett: 2,
        aett_name: "Hagal's Aett",
        rune_poem_oe: "OE [st. 13]: 'The yew (Eoh) is a tree with rough \
            bark, hard and fast in the earth, supported by its roots, \
            a guardian of flame and a joy upon an estate.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Odin (as Hangatyr, the Hanged God), Hel",
        element: "All elements (the yew connects all Nine Worlds)",
        world: "Yggdrasil itself — the World Tree",
        esoteric: "Eihwaz is the vertical axis of the cosmos — Yggdrasil, \
            the World Tree. The yew's paradox (evergreen yet deadly, \
            living yet associated with death) makes it the perfect \
            symbol for the axis connecting life and death, the worlds \
            above and below. Odin hung on Yggdrasil to win the runes; \
            Eihwaz is the tree itself. It is the rune of initiation \
            through ordeal, shamanic death-and-rebirth, and the mastery \
            of life's deepest mysteries. Magically it protects, aids \
            astral travel, and marks transitions between states.",
        meaning_upright: "Yggdrasil, endurance, the axis mundi, initiation \
            through ordeal, shamanic death-rebirth, protection, \
            deep mysteries.",
        meaning_reversed: "Confusion of levels, failed initiation, \
            vulnerability at transitions, entrapment between worlds.",
    },
    Rune {
        number: 14,
        name: "Perthro",
        alt_names: "Peordh (OE), Perthro, Perth",
        glyph: "ᛈ",
        phoneme: "p",
        etymology: "Proto-Germanic *perþō — meaning uncertain; the most \
            debated rune in scholarly literature. Proposed meanings include \
            'lot-cup, dice-cup' (the cup from which lots are cast), \
            'fruit tree,' 'vagina' (as a symbol of fate and birth), and \
            'chess piece.' The lot-cup interpretation aligns with the \
            divinatory tradition. Perthro does not appear in the Younger \
            Futhark or in the Norwegian and Icelandic Rune Poems.",
        aett: 2,
        aett_name: "Hagal's Aett",
        rune_poem_oe: "OE [st. 14]: 'Peordh is a source of recreation and \
            amusement to the great, where warriors sit blithely together \
            in the beer-hall.' (Halsall 1981) \
            [Note: the word *peordh* is untranslated in the manuscript; \
            its meaning is genuinely unknown — Page 1999, p. 72.]",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "The Norns (as dispensers of fate-lots), Frigg",
        element: "Water (the womb of fate)",
        world: "The Well of Urðr (Wyrd)",
        esoteric: "Perthro is the womb of fate — the cosmic lot-cup from \
            which all destinies are drawn. It is the mystery of *wyrd* \
            (fate as interconnected web), the hidden cause behind \
            apparent chance. Its glyph resembles an overturned cup or \
            an open womb. In the divinatory tradition it governs \
            revelation of hidden things, initiation into mysteries, \
            and the acceptance of what fate has decreed. Magically it \
            works with divination itself, probability, and accessing \
            hidden or ancestral knowledge.",
        meaning_upright: "Hidden things revealed, fate, the mystery of \
            wyrd, divination, initiation, the womb of becoming, \
            that which is cast or destined.",
        meaning_reversed: "Addiction, stagnation, hidden malevolence, \
            uncontrolled randomness, ill fate.",
    },
    Rune {
        number: 15,
        name: "Elhaz",
        alt_names: "Eolhx / Algiz (OE), Elhaz, Algiz, Z-rune",
        glyph: "ᛉ",
        phoneme: "z (or R, the Germanic *algiz*-sound, a semi-vowel)",
        etymology: "Proto-Germanic *algiz — 'elk' or possibly 'protection.' \
            The phoneme this rune represents (a final -z that became -R \
            in Proto-Norse) is called the *algiz*-sound. Old English \
            *eolhx* (OE Rune Poem) is a hapax legomenon. The form \
            also resembles a raised hand or an elk's antlers — both \
            protective imagery.",
        aett: 2,
        aett_name: "Hagal's Aett",
        rune_poem_oe: "OE [st. 15]: 'The elk-sedge (Eolhx) is mostly to be \
            found in the fen, growing in the water. It wounds severely, \
            staining with blood every man who makes a grab at it.' \
            (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Heimdall (as guardian of Bifrost), the Valkyries",
        element: "Air / Fire",
        world: "Bifrost (the Rainbow Bridge — threshold between worlds)",
        esoteric: "Elhaz is the rune of divine protection — the upraised \
            hand of the guardian, the elk's spreading antlers, the \
            sedge-grass that bleeds the grasping hand. It is the \
            *elhaz*-protection (*elhaz* = life-force protection in some \
            reconstructions). The glyph, like a person with arms raised, \
            represents the protective gesture (*vitki* — the runic \
            practitioner) calling on higher powers. It governs all \
            protective work, warding, sanctuary, connection to the divine, \
            and the safe traversal of dangerous thresholds.",
        meaning_upright: "Protection, warding, sanctuary, connection to the \
            divine, higher consciousness, the safe passage.",
        meaning_reversed: "Hidden danger, vulnerability, loss of divine \
            connection, sacrifice, a warning sign.",
    },
    Rune {
        number: 16,
        name: "Sowilo",
        alt_names: "Sigel (OE), Sól (ON/OI), Sowulo, Sol",
        glyph: "ᛊ",
        phoneme: "s",
        etymology: "Proto-Germanic *sōwilō — 'sun.' Cognate with Old Norse \
            *sól*, Old English *sigel*, Latin *sol*. The sun was a supreme \
            sacred force in Germanic religion, associated with victory, \
            clarity, and divine power. IMPORTANT HISTORICAL NOTE: This \
            rune was doubled (ᛊᛊ) as the insignia of the SS \
            (*Schutzstaffel*) in Nazi Germany, designed by Walter Heck \
            (1932). This appropriation is a historical atrocity with \
            no foundation in legitimate runic tradition.",
        aett: 2,
        aett_name: "Hagal's Aett",
        rune_poem_oe: "OE [st. 16]: 'The sun (Sigel) is ever a joy in the \
            hopes of seafarers when they journey away over the fishes' \
            bath, until the courser of the deep bears them to land.' \
            (Halsall 1981)",
        rune_poem_on: "ON [st. 11]: 'Sun (Sól) is the light of the lands; \
            I bow to the divine decree.' (Page 1999)",
        rune_poem_oi: "OI [st. 11]: 'Sun (Sól) — shield of the clouds and \
            shining ray and destroyer of ice.' (Page 1999)",
        deity: "Sól (the sun-goddess), Baldur (as solar hero)",
        element: "Fire",
        world: "Asgard (Sól's chariot-path)",
        esoteric: "Sowilo is the rune of the sun at the height of its \
            power — the lightning-bolt of the victorious divine will. \
            It represents the solar force that overcomes darkness, \
            the guiding star for travellers and heroes, and the \
            concentrated will that achieves its goal. Its *S*-shape \
            (or the Anglo-Saxon *sigil* form ᛋ) resembles a lightning \
            bolt. Magically it provides guidance, goal-seeking energy, \
            victory over obstacles, and healing of the soul.",
        meaning_upright: "The sun, victory, clarity, solar will, guidance, \
            success, life-force, wholeness, divine illumination.",
        meaning_reversed: "(Sowilo is generally not reversed due to its \
            rotational symmetry. When read reversed it can indicate \
            false clarity or the arrogance of unchecked solar ego.)",
    },
    // ═══════════════════════════════════════════════════════════════════════
    //  TYR'S AETT  (runes 17–24)
    // ═══════════════════════════════════════════════════════════════════════
    Rune {
        number: 17,
        name: "Tiwaz",
        alt_names: "Tir / Tir (OE), Týr (ON/OI), Teiwaz, Tiw",
        glyph: "ᛏ",
        phoneme: "t",
        etymology: "Proto-Germanic *tīwaz — 'the god Tyr,' from PIE *deywós \
            (sky-god, cf. Latin *deus*, Greek *Zeus*). Tyr/Tiw is the \
            ancient Germanic sky-god of law, justice, and righteous combat. \
            The English weekday 'Tuesday' (*Tīwesdæg*) preserves his name.",
        aett: 3,
        aett_name: "Tyr's Aett",
        rune_poem_oe: "OE [st. 17]: 'Tiw (Tir) is a guiding star; well \
            does it keep faith with princes; it is ever on its course \
            over the mists of night and never fails.' (Halsall 1981)",
        rune_poem_on: "ON [st. 12]: 'Tyr (Týr) is the one-handed god; \
            often the smith has to blow.' (Page 1999)",
        rune_poem_oi: "OI [st. 12]: 'Tyr (Týr) — one-handed god and \
            leavings of the wolf and prince of temples.' (Page 1999)",
        deity: "Tyr (Tiw) — the sky-god of law and right order",
        element: "Air / Fire",
        world: "Asgard (the courts of divine law)",
        esoteric: "Tiwaz is the rune of divine order, victory through \
            righteousness, and self-sacrifice in service of the collective. \
            Tyr sacrificed his hand in binding the wolf Fenrir — a \
            deliberate self-wounding to preserve cosmic order. The \
            rune's arrow-like form (*↑*) points to the Pole Star \
            (Tyr as a stellar deity), the fixed point by which all \
            navigation is oriented. Magically it governs just causes, \
            legal matters, victory in fair combat, and the courage \
            to sacrifice personal gain for right order.",
        meaning_upright: "Justice, righteous victory, self-sacrifice, \
            the warrior's honour, divine order, legal success, \
            strength of will.",
        meaning_reversed: "Injustice, defeat through lack of principle, \
            broken faith, a cause that is not right, blocked victory.",
    },
    Rune {
        number: 18,
        name: "Berkano",
        alt_names: "Beorc (OE), Bjarkan (ON/OI), Berkanan, Bercna",
        glyph: "ᛒ",
        phoneme: "b",
        etymology: "Proto-Germanic *berkanan — 'birch tree (*Betula*).' \
            Cognate with Old Norse *björk*, Old English *beorc* (birch). \
            The birch was among the first trees to re-colonise post-glacial \
            Europe and was associated with spring, new beginnings, \
            and feminine regeneration.",
        aett: 3,
        aett_name: "Tyr's Aett",
        rune_poem_oe: "OE [st. 18]: 'The poplar (Beorc) is without fruit; \
            yet without seed it brings forth suckers, for it is generated \
            from its leaves. Splendid are its branches and gloriously \
            adorned its lofty crown which reaches to the skies.' \
            (Halsall 1981) [Note: the OE poem may mean 'birch' or \
            'poplar' — the identification is debated; Page 1999, p. 78.]",
        rune_poem_on: "ON [st. 13]: 'Birch (Bjarkan) is the limb greenest \
            with leaves; Loki brought the luck of deceit.' (Page 1999)",
        rune_poem_oi: "OI [st. 13]: 'Birch (Bjarkan) — leafy twig and \
            little tree and fresh young shrub.' (Page 1999)",
        deity: "Frigg (as goddess of childbirth and the home), Nerthus",
        element: "Earth",
        world: "Midgard / Vanaheim",
        esoteric: "Berkano is the Great Mother rune — the birch-goddess of \
            birth, nurturing, concealment, and regeneration. Its glyph \
            (two breasts against a vertical trunk) is one of the most \
            overtly body-symbolic forms in the futhark. The birch \
            regenerates from its own roots; Berkano governs all forms \
            of beginning-again: birth, healing, the return of spring, \
            and the concealment of new things until they are ready \
            to emerge. Magically it governs birthing, healing, \
            earth-connection, and the nurturing of new projects.",
        meaning_upright: "Birth, new beginnings, nurturing, the Mother, \
            healing, regeneration, concealment of the sacred, \
            the return of spring.",
        meaning_reversed: "Failed birth, stunted growth, family troubles, \
            careless or destructive nurturing, the dark mother.",
    },
    Rune {
        number: 19,
        name: "Ehwaz",
        alt_names: "Eh (OE), Ehwaz, Eoh",
        glyph: "ᛖ",
        phoneme: "e",
        etymology: "Proto-Germanic *ehwaz — 'horse.' Cognate with Old English \
            *eoh* (horse, war-horse), Latin *equus*, Sanskrit *aśva*. \
            The horse was the most prestigious animal in Germanic culture — \
            a war-companion, a divine vehicle, and a creature of sacred \
            partnership. Ehwaz does not appear in the Younger Futhark.",
        aett: 3,
        aett_name: "Tyr's Aett",
        rune_poem_oe: "OE [st. 19]: 'The horse (Eh) is a joy to princes \
            in the presence of warriors. A steed in the pride of its hoofs, \
            when rich men on horseback bandy words about it; and it is \
            ever a source of comfort to the restless.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Odin (as rider of Sleipnir), the Divine Twins (Alcis)",
        element: "Earth / Air",
        world: "Midgard (all roads and journeys)",
        esoteric: "Ehwaz is the rune of partnership-in-motion — the perfect \
            collaboration between rider and horse, between the self and \
            its vehicle. The Divine Twins (*Alcis*, attested by Tacitus) \
            are an Ehwaz archetype: two beings moving as one. Magically \
            it governs astral travel (the horse as psychopomp vehicle), \
            harmonious partnerships, shape-shifting, and the trust \
            required for true collaboration.",
        meaning_upright: "Partnership, trust, harmonious movement, the \
            horse-rider relationship, astral travel, loyal \
            collaboration, progress.",
        meaning_reversed: "Broken trust, disharmony in partnership, \
            blocked movement, betrayal.",
    },
    Rune {
        number: 20,
        name: "Mannaz",
        alt_names: "Mann (OE), Maðr (ON/OI), Manna, Mannaz",
        glyph: "ᛗ",
        phoneme: "m",
        etymology: "Proto-Germanic *mannaz — 'human being, man.' \
            Cognate with Old Norse *maðr*, Old English *mann*, Gothic \
            *manna*. Also the name of the proto-ancestor Mannus, attested \
            by Tacitus (*Germania*, c. 98 CE) as the son of Tuisto, \
            the progenitor of the Germanic peoples.",
        aett: 3,
        aett_name: "Tyr's Aett",
        rune_poem_oe: "OE [st. 20]: 'The joyous man (Mann) is dear to his \
            kinsmen; yet every man is doomed to fail his fellow, since \
            the Lord by his decree will commit the vile carrion to the earth.' \
            (Halsall 1981)",
        rune_poem_on: "ON [st. 14]: 'Man (Maðr) is an augmentation of \
            the dust; great is the claw of the hawk.' (Page 1999)",
        rune_poem_oi: "OI [st. 14]: 'Man (Maðr) — delight of man and \
            augmentation of the earth and adorner of ships.' (Page 1999)",
        deity: "Heimdall (as father of the human classes; *Rígsþula*), \
            Odin (as creator of Ask and Embla)",
        element: "Air / All elements",
        world: "Midgard",
        esoteric: "Mannaz is the rune of the divine human — the being \
            crafted by the gods (Ask and Embla, the first man and woman, \
            were given breath by Odin, sense by Hœnir, and blood-colour \
            by Lóðurr). Mannaz represents humanity in its highest aspect: \
            the social being who participates in divine intelligence, \
            the individual embedded in and responsible to the community. \
            Magically it is used for enhancing intelligence, memory, \
            social success, and the awakening of the divine image within.",
        meaning_upright: "Humanity, the divine image in mankind, \
            intelligence, social cooperation, the self in relation \
            to community, memory, reason.",
        meaning_reversed: "Cunning without wisdom, isolation, self-deception, \
            the enemy within, mortal weakness.",
    },
    Rune {
        number: 21,
        name: "Laguz",
        alt_names: "Lagu (OE), Lögr (ON/OI), Lagaz, Laukaz",
        glyph: "ᛚ",
        phoneme: "l",
        etymology: "Proto-Germanic *laguz — 'water, lake, sea.' \
            Cognate with Old Norse *lögr* (water, sea), Old English \
            *lagu* (sea, water), Latin *lacus* (lake). An older \
            form, *laukaz* (leek), may also underlie this rune, \
            connecting it with growth, magic, and fertility.",
        aett: 3,
        aett_name: "Tyr's Aett",
        rune_poem_oe: "OE [st. 21]: 'The ocean (Lagu) seems interminable \
            to men, if they venture on the rolling bark and the waves \
            of the sea terrify them and the courser of the deep heeds \
            not its bridle.' (Halsall 1981)",
        rune_poem_on: "ON [st. 15]: 'A waterfall (Lögr) is that which \
            falls from the mountain-side; but gold objects are costly things.' \
            (Page 1999)",
        rune_poem_oi: "OI [st. 15]: 'Water (Lögr) — eddying stream and \
            broad geyser and land of fish.' (Page 1999)",
        deity: "Njord (sea-god), Nerthus (goddess of waters), Rán",
        element: "Water",
        world: "The World Ocean / Jotunheim (the unconscious depths)",
        esoteric: "Laguz is the rune of the ocean of the unconscious — \
            the primal waters that underlie all manifest reality. \
            Water flows, penetrates, and dissolves; it is the medium \
            of the life-force (*lagu*) and the magical power of the \
            *seiðr* practitioner. The *laukaz* (leek) reading connects \
            Laguz to magical growth, healing herbs, and the life-sap \
            of the plant kingdom. Magically it governs psychic development, \
            dreaming, ancestral memory, healing, and all work with \
            the unconscious.",
        meaning_upright: "Water, the unconscious, intuition, psychic \
            ability, flow, adaptability, the life-force in nature, \
            dreamwork.",
        meaning_reversed: "Being overwhelmed, losing direction at sea, \
            psychic attack, blocked intuition, drowning in emotion.",
    },
    Rune {
        number: 22,
        name: "Ingwaz",
        alt_names: "Ing (OE), Inguz, Enguz",
        glyph: "ᛜ",
        phoneme: "ng (velar nasal)",
        etymology: "Proto-Germanic *Ingwaz — the divine ancestor Ing \
            (Old Norse Yngvi-Freyr). Ing is the progenitor of the Yngling \
            dynasty of Swedish kings and a name or epithet of Freyr, \
            the god of fertility, prosperity, and sacred kingship. \
            Ingwaz does not appear in the Younger Futhark.",
        aett: 3,
        aett_name: "Tyr's Aett",
        rune_poem_oe: "OE [st. 22]: 'Ing was first seen by men among \
            the East-Danes, till, followed by his chariot, he departed \
            eastwards over the wave. So the Heardingas named the hero.' \
            (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Freyr / Yngvi-Freyr (Ing) — god of fertility and sacred kingship",
        element: "Earth / Fire (the stored seed-fire)",
        world: "Vanaheim",
        esoteric: "Ingwaz is the rune of the divine seed — the concentrated, \
            stored potential that explodes into manifestation at the \
            right moment. Its diamond glyph resembles a seed, a womb, \
            or the genetrix of the rune system. Ing was the progenitor \
            who departed mysteriously, leaving behind his generative \
            power. Magically Ingwaz governs the completion of projects, \
            the integration of a cycle, male fertility, gestation, \
            and the moment of sacred release when potential becomes actual.",
        meaning_upright: "The divine seed, completion of a cycle, \
            gestation, male fertility, stored potential, \
            the moment of release, sacred ancestor.",
        meaning_reversed: "(Ingwaz is symmetrical and has no reversed \
            position in most traditions.)",
    },
    Rune {
        number: 23,
        name: "Dagaz",
        alt_names: "Dæg (OE), Dagaz, Daeg",
        glyph: "ᛞ",
        phoneme: "d",
        etymology: "Proto-Germanic *dagaz — 'day.' Cognate with Old Norse \
            *dagr*, Old English *dæg*, Gothic *dags*. Dagaz does not \
            appear in the Younger Futhark. Note: the ordering of Dagaz \
            and Othala (23rd and 24th) is disputed — some manuscripts \
            place Othala last, others Dagaz.",
        aett: 3,
        aett_name: "Tyr's Aett",
        rune_poem_oe: "OE [st. 24]: 'Day (Dæg), the glorious light of \
            the Creator, is sent by the Lord; it is beloved of men, \
            a source of hope and happiness to rich and poor, and of \
            service to all.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Heimdall (as guardian of the twilight thresholds), \
            Baldur (as the bright day)",
        element: "Fire / Air (the light at the threshold)",
        world: "Asgard (the liminal space of dawn and dusk)",
        esoteric: "Dagaz is the paradox-rune — the moment of dawn or dusk \
            when darkness and light perfectly balance and interpenetrate. \
            Its hourglass or double-loop glyph encodes this balance of \
            opposites. It represents breakthrough, transformation at the \
            threshold, and the creative paradox that contains both \
            its own beginning and ending. Magically it is used for \
            invisibility (operating in the liminal space between states), \
            enlightenment, and the integration of apparent opposites.",
        meaning_upright: "Breakthrough, transformation, the threshold \
            moment, paradox resolved, clarity after darkness, \
            a new day, the meeting of opposites.",
        meaning_reversed: "(Dagaz is symmetrical — it looks the same \
            upside down — and is generally not reversed.)",
    },
    Rune {
        number: 24,
        name: "Othala",
        alt_names: "Œthel / Ethel (OE), Othalan, Odal, Oðal",
        glyph: "ᛟ",
        phoneme: "o (long ō)",
        etymology: "Proto-Germanic *ōþalan — 'ancestral estate, inherited \
            land, nobility.' Cognate with Old English *œðel*/*ēðel* \
            (native land, ancestral property), Old Norse *óðal* (allodial \
            land). The *óðal* right was the inalienable claim of a free \
            family to its ancestral lands under Germanic law. HISTORICAL \
            NOTE: This rune was appropriated by Himmler's SS as a symbol \
            of blood-and-soil racism. This appropriation is a historical \
            crime entirely foreign to the rune's legitimate meaning.",
        aett: 3,
        aett_name: "Tyr's Aett",
        rune_poem_oe: "OE [st. 23]: 'An estate (Œthel) is very dear to \
            every man, if he can enjoy there in his house whatever is \
            right and proper in constant prosperity.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Odin (as Allfather and ancestral progenitor), the \
            land-spirits (*landvœttir*)",
        element: "Earth",
        world: "Midgard (the ancestral homeland)",
        esoteric: "Othala closes the Elder Futhark as the rune of the \
            ancestral estate — not merely land, but the accumulated wisdom, \
            memory, and spiritual inheritance of one's bloodline and \
            people. It connects the individual to the chain of ancestral \
            consciousness, the collective *wyrd* of the clan. Magically \
            it is used for working with ancestors, protecting home and \
            family, accessing inherited gifts and abilities, and \
            establishing right relationship with land and lineage.",
        meaning_upright: "Ancestral inheritance, the family estate, \
            homeland, ancestral wisdom, collective memory, the gifts \
            of lineage, a secure foundation.",
        meaning_reversed: "Inheritance contested, exile, disconnection from \
            roots, ancestral trauma, clannishness or exclusion.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elder_futhark_has_24_runes() {
        assert_eq!(ELDER_FUTHARK.len(), 24);
    }

    #[test]
    fn elder_futhark_numbered_correctly() {
        for (i, rune) in ELDER_FUTHARK.iter().enumerate() {
            assert_eq!(rune.number as usize, i + 1, "Rune '{}' has wrong number", rune.name);
        }
    }

    #[test]
    fn elder_futhark_aett_assignments() {
        let aett1: Vec<_> = ELDER_FUTHARK.iter().filter(|r| r.aett == 1).collect();
        let aett2: Vec<_> = ELDER_FUTHARK.iter().filter(|r| r.aett == 2).collect();
        let aett3: Vec<_> = ELDER_FUTHARK.iter().filter(|r| r.aett == 3).collect();
        assert_eq!(aett1.len(), 8);
        assert_eq!(aett2.len(), 8);
        assert_eq!(aett3.len(), 8);
    }

    #[test]
    fn all_runes_have_glyphs() {
        for rune in ELDER_FUTHARK {
            assert!(!rune.glyph.is_empty(), "Rune '{}' has no glyph", rune.name);
        }
    }
}
