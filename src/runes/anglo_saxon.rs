//! Anglo-Saxon Futhorc — the expanded Old English runic alphabet.
//!
//! The Anglo-Saxon Futhorc (*fuþorc*, from the phonemic values of its
//! first six runes) developed from the Elder Futhark as Germanic-speaking
//! peoples settled in Britain (c. 5th century CE).  Where the Younger
//! Futhark *contracted* the Elder Futhark, the Futhorc *expanded* it,
//! adding new runes to represent sounds distinctive to Old English.
//!
//! ## Structure
//!
//! The core Futhorc has 28 runes, attested in the *Old English Rune Poem*
//! (which covers 29 stanzas, the last being Ear).  Manuscript traditions
//! — especially the Northumbrian extension — add runes to bring the total
//! to 33.  This module presents all 33:
//!
//! - **Runes 1–24**: Futhorc equivalents of the Elder Futhark (with
//!   Old English name-forms and significant graphic changes).
//! - **Runes 25–29**: The five additional runes of the standard Futhorc
//!   (*Ac*, *Æsc*, *Yr*, *Ior*, *Ear*).
//! - **Runes 30–33**: Northumbrian extensions (*Cweorth*, *Calc*,
//!   *Stan*, *Gar*), attested primarily in the
//!   *Vienna Codex* (MS Cod. Vindob. 795) and related manuscripts.
//!
//! ## Key graphic differences from Elder Futhark
//!
//! Several runes have substantially different forms in the Futhorc:
//! *Hægl* (ᚻ) uses a single-stem H-form; *Ger* (ᛄ) has a distinct
//! shape; *Sigel* (ᛋ) is an S-form; *Ing* (ᛝ) has a double-diamond form.
//!
//! ## Sources
//!
//! - Halsall, Maureen. *The Old English Rune Poem: A Critical Edition*.
//!   Toronto: University of Toronto Press, 1981.
//! - Page, R.I. *An Introduction to English Runes*, 2nd ed.
//!   Woodbridge: Boydell Press, 1999.
//! - Derolez, René. *Runica Manuscripta: The English Tradition*.
//!   Bruges: De Tempel, 1954.
//! - Elliott, Ralph W.V. *Runes: An Introduction*, 2nd ed.
//!   Manchester: Manchester University Press, 1989.
//! - Odenstedt, Bengt. *On the Origin and Early History of the Runic Script*.
//!   Uppsala: Almqvist & Wiksell, 1990.
//!
//! All stanza translations follow Halsall (1981) unless noted.
//! The Northumbrian rune names follow Page (1999) and Derolez (1954).

use super::Rune;

/// The 33 runes of the Anglo-Saxon Futhorc in canonical order.
pub static ANGLO_SAXON: &[Rune] = &[
    // ─── Core Futhorc (runes 1–24, OE equivalents of Elder Futhark) ─────────
    Rune {
        number: 1,
        name: "Feoh",
        alt_names: "Feoh (OE); = EF Fehu",
        glyph: "ᚠ",
        phoneme: "f",
        etymology: "Old English *feoh* — 'cattle, property, money.' \
            Direct cognate of Elder Futhark Fehu. In Old English law, \
            *feoh* was the standard term for movable wealth and monetary \
            compensation (*wergild*).",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 1]: 'Wealth (Feoh) is a comfort to every \
            man; yet every man must bestow it freely if he wish to gain \
            honour in the sight of the Lord.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Freyr, Njord",
        element: "Fire",
        world: "Vanaheim / Midgard",
        esoteric: "Circulating wealth, creative energy, the fire of \
            material prosperity.",
        meaning_upright: "Wealth, prosperity, generosity, success.",
        meaning_reversed: "Financial discord, greed, hoarded energy.",
    },
    Rune {
        number: 2,
        name: "Ūr",
        alt_names: "Ur (OE); = EF Uruz",
        glyph: "ᚢ",
        phoneme: "u",
        etymology: "Old English *ūr* — 'aurochs (*Bos primigenius*).' \
            The OE poem confirms the 'aurochs' meaning (unlike the Younger \
            Futhark, which shifted to 'drizzle'). The aurochs was hunted \
            in continental Europe but may have already been absent from \
            Britain by the Anglo-Saxon period.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 2]: 'The aurochs (Ur) is proud and has \
            great horns; it is a very savage beast and fights with its \
            horns; a great ranger of the moors, it is a creature of mettle.' \
            (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Thor",
        element: "Earth",
        world: "Midgard / Jotunheim",
        esoteric: "Primal vitality, physical strength, untamed power.",
        meaning_upright: "Strength, health, vitality, primal force.",
        meaning_reversed: "Weakness, missed opportunities.",
    },
    Rune {
        number: 3,
        name: "Þorn",
        alt_names: "Thorn (OE); = EF Thurisaz",
        glyph: "ᚦ",
        phoneme: "þ (th)",
        etymology: "Old English *þorn* — 'thorn.' The shift from *þurs* \
            (giant) in Elder Futhark to *þorn* (thorn) in Old English \
            emphasises the protective/wounding aspect over the giant \
            association. The Thorn letterform (Þ) descends directly \
            from this rune and survived as an English letter until \
            the 15th century.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 3]: 'The thorn (Thorn) is very sharp; \
            for every thane who seizes it, it is harmful, and uncommonly \
            severe to every man who lies among them.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Thor",
        element: "Fire",
        world: "Jotunheim",
        esoteric: "Directed force, protection through the capacity \
            to wound, gateway-breaking.",
        meaning_upright: "Directed force, protection, catalytic change.",
        meaning_reversed: "Uncontrolled aggression, vulnerability.",
    },
    Rune {
        number: 4,
        name: "Ōs",
        alt_names: "Os (OE); cf. EF Ansuz",
        glyph: "ᚩ",
        phoneme: "o",
        etymology: "Old English *ōs* — 'god (one of the Æsir)' or \
            'mouth, speech.' The OE Rune Poem uses the 'mouth' meaning: \
            *ōs* as the source of language — showing both the Christian \
            overlay (wisdom is God-given) and the Germanic sense of \
            divine speech. Distinct graphic form from Elder Futhark Ansuz.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 4]: 'The mouth (Os) is the source of \
            all language, a pillar of wisdom and a comfort to wise men, \
            a blessing and a joy to every nobleman.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Odin",
        element: "Air",
        world: "Asgard",
        esoteric: "Divine speech, wisdom, the Word as creative force.",
        meaning_upright: "Communication, wisdom, divine inspiration.",
        meaning_reversed: "Miscommunication, deception.",
    },
    Rune {
        number: 5,
        name: "Rād",
        alt_names: "Rad (OE); = EF Raidho",
        glyph: "ᚱ",
        phoneme: "r",
        etymology: "Old English *rād* — 'riding, road, journey.' \
            The same root as modern English 'road.' The OE poem's \
            contrast between the easy-seeming indoor view of riding \
            and the actual hardship is a common Germanic wisdom trope.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 5]: 'Riding (Rad) seems easy to every \
            warrior while he is indoors and very courageous to him who \
            traverses the high-roads on the back of a stout horse.' \
            (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Thor, Odin",
        element: "Air",
        world: "Midgard",
        esoteric: "Journey, cosmic order, right action, ritual procession.",
        meaning_upright: "Journey, right action, good timing, alignment.",
        meaning_reversed: "Crisis in travel, disrupted plans.",
    },
    Rune {
        number: 6,
        name: "Cēn",
        alt_names: "Ken (OE); = EF Kenaz",
        glyph: "ᚳ",
        phoneme: "c / k",
        etymology: "Old English *cen* — 'torch, pine-torch.' Retains the \
            Elder Futhark 'torch' meaning (unlike Younger Futhark Kaun, \
            which shifted to 'ulcer'). The OE poem emphasises the hearth-fire \
            that distinguishes civilised dwelling from the dark outside.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 6]: 'The torch (Cen) is known to every \
            living man by its pale, bright flame; it always burns where \
            princes sit within.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Heimdall, Freyja",
        element: "Fire",
        world: "Midgard",
        esoteric: "Illumination, knowledge, creative fire, craft.",
        meaning_upright: "Illumination, knowledge, creative fire, passion.",
        meaning_reversed: "Lost direction, extinguished inspiration.",
    },
    Rune {
        number: 7,
        name: "Gyfu",
        alt_names: "Geofu (OE); = EF Gebo",
        glyph: "ᚷ",
        phoneme: "g",
        etymology: "Old English *gyfu* — 'gift, generosity.' The law \
            of gift-exchange (*gáfu*) was central to Anglo-Saxon social \
            structure; *gyfu* in OE also carries the sense of God's grace \
            (a Christian semantic layer). The X-shape cannot be reversed.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 7]: 'Generosity (Gyfu) brings credit and \
            honour, which support one's dignity; it is an adornment and a \
            mark of esteem, the support and subsistence of all the \
            destitute.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Odin (gift-giver), the Vanir",
        element: "Air",
        world: "All worlds",
        esoteric: "Sacred reciprocity, partnership, initiation. \
            The crossing of forces in exchange.",
        meaning_upright: "Gift, partnership, generosity, sacred union.",
        meaning_reversed: "(Symmetrical — no reversal.)",
    },
    Rune {
        number: 8,
        name: "Wynn",
        alt_names: "Wen (OE); = EF Wunjo",
        glyph: "ᚹ",
        phoneme: "w",
        etymology: "Old English *wynn* — 'joy, bliss, pleasure.' \
            The Wynn letterform (Ƿ) was used as the Old English letter W \
            until replaced by the digraph 'ww' > 'w' in Middle English. \
            The OE poem emphasises the conditional nature of joy: it \
            depends on health and sufficient material support.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 8]: 'Bliss (Wynn) is had by one who \
            knows few troubles, pains and sorrows, and to him who himself \
            has power and blessedness and a good enough house.' \
            (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Odin (as *Wunsc*), Freyr",
        element: "Earth / Air",
        world: "Asgard / Vanaheim",
        esoteric: "Achieved harmony, fellowship, spiritual well-being.",
        meaning_upright: "Joy, harmony, fellowship, wish-fulfilment.",
        meaning_reversed: "Sorrow, alienation, disharmony.",
    },
    Rune {
        number: 9,
        name: "Hægl",
        alt_names: "Haegel (OE); = EF Hagalaz",
        glyph: "ᚻ",
        phoneme: "h",
        etymology: "Old English *hægl* — 'hail.' The Futhorc graphic \
            form differs from the Elder Futhark — an H-shaped stave. \
            The OE poem is unusually beautiful, describing hail as \
            'whitest of grains' that 'melts into water' — emphasising \
            the transformative cycle.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 9]: 'Hail (Hægl) is the whitest of \
            grains; it comes whirling from the vault of heaven and is \
            tossed about by gusts of wind and then it melts into water.' \
            (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Heimdall, Odin",
        element: "Ice",
        world: "Niflheim",
        esoteric: "Disruption as the seed of transformation. The cosmic matrix.",
        meaning_upright: "Disruption, transformation through crisis, elemental force.",
        meaning_reversed: "(Symmetrical — no standard reversal.)",
    },
    Rune {
        number: 10,
        name: "Nȳd",
        alt_names: "Nyd (OE); = EF Nauthiz",
        glyph: "ᚾ",
        phoneme: "n",
        etymology: "Old English *nȳd* — 'need, compulsion, necessity.' \
            The crossed-stave form remains. The OE poem's 'yet often it \
            proves a source of help' distinguishes the Anglo-Saxon view \
            from the harsher ON/OI poems — need as also potentially \
            redemptive.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 10]: 'Need (Nyd) is oppressive to the heart; \
            yet often it proves a source of help and salvation to the sons \
            of men, to everyone who heeds it in time.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "The Norns",
        element: "Fire (necessity-fire)",
        world: "The Well of Wyrd",
        esoteric: "Fate-as-constraint. The friction that generates \
            transformation.",
        meaning_upright: "Need, necessity, patience, wisdom within hardship.",
        meaning_reversed: "Slavery to compulsion, wrong desire.",
    },
    Rune {
        number: 11,
        name: "Īs",
        alt_names: "Is (OE); = EF Isa",
        glyph: "ᛁ",
        phoneme: "i",
        etymology: "Old English *īs* — 'ice.' Unchanged. The OE poem is \
            the most visually evocative of all: 'glistens as clear as \
            glass and most like to gems; it is a floor wrought by the frost, \
            fair to look upon.'",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 11]: 'Ice (Is) is very cold and immeasurably \
            slippery; it glistens as clear as glass and most like to gems; \
            it is a floor wrought by the frost, fair to look upon.' \
            (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Niflheim rulers",
        element: "Ice",
        world: "Niflheim",
        esoteric: "Stillness, halt, concentration, the frozen axis.",
        meaning_upright: "Stillness, halt, introspection, self-discipline.",
        meaning_reversed: "(Symmetrical — no reversal.)",
    },
    Rune {
        number: 12,
        name: "Gēr",
        alt_names: "Ger (OE); = EF Jera",
        glyph: "ᛄ",
        phoneme: "g / y (palatal)",
        etymology: "Old English *gēr* / *gēar* — 'year, annual harvest.' \
            Unchanged in meaning. The OE poem's invocation of 'God, the \
            holy King of Heaven' shows Christian overlay on the pre-Christian \
            harvest theology.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 12]: 'Summer (Ger) is a joy to men, when \
            God, the holy King of Heaven, suffers the earth to bring forth \
            shining fruits for rich and poor alike.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Freyr, Odin",
        element: "Earth",
        world: "Midgard",
        esoteric: "The sacred year-cycle, patient fruition. Right timing.",
        meaning_upright: "The harvest, reward for effort, patience.",
        meaning_reversed: "(No reversal — the year turns without inversion.)",
    },
    Rune {
        number: 13,
        name: "Ēoh",
        alt_names: "Eoh (OE); = EF Eihwaz",
        glyph: "ᛇ",
        phoneme: "eo / io",
        etymology: "Old English *ēoh* — 'yew tree (*Taxus baccata*).' \
            The yew's paradox is carefully noted in the OE poem: 'without \
            fruit' yet life-sustaining 'on an estate.' Note: this rune \
            (Ēoh) is distinct from the later Futhorc rune Yr (rune 27), \
            which also relates to yew.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 13]: 'The yew (Eoh) is a tree with rough \
            bark, hard and fast in the earth, supported by its roots, \
            a guardian of flame and a joy upon an estate.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Odin (as Hangatyr), Hel",
        element: "All elements",
        world: "Yggdrasil",
        esoteric: "The World Tree axis. Initiation through ordeal. \
            Shamanic death-rebirth.",
        meaning_upright: "Endurance, the axis mundi, initiation, deep mysteries.",
        meaning_reversed: "Confusion of levels, failed initiation.",
    },
    Rune {
        number: 14,
        name: "Peorð",
        alt_names: "Peordh (OE); = EF Perthro",
        glyph: "ᛈ",
        phoneme: "p",
        etymology: "Old English *peorð* — meaning genuinely unknown; \
            the most debated rune name in the entire corpus. The OE poem \
            provides no clear lexical identification. Page (1999, p. 72) \
            notes the word '*peordh*' is simply left untranslated in \
            the Hickes text. Proposed interpretations include 'lot-cup,' \
            'chess piece,' 'pear tree,' and others.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 14]: 'Peordh is a source of recreation and \
            amusement to the great, where warriors sit blithely together \
            in the beer-hall.' (Halsall 1981) \
            [The word *peordh* is a hapax legomenon — its meaning is \
            genuinely unknown; Page 1999, p. 72.]",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "The Norns, Frigg",
        element: "Water",
        world: "The Well of Wyrd",
        esoteric: "Hidden things, fate, the mystery of wyrd, divination.",
        meaning_upright: "Hidden things revealed, fate, initiation into mystery.",
        meaning_reversed: "Addiction, hidden malevolence, ill fate.",
    },
    Rune {
        number: 15,
        name: "Eolhx",
        alt_names: "Algiz (OE); = EF Elhaz",
        glyph: "ᛉ",
        phoneme: "x / z (Anglian) or ea (West Saxon)",
        etymology: "Old English *eolhx* — a hapax legomenon translated \
            tentatively as 'elk-sedge' (*Carex*) or related to 'elk.' \
            The protective grass whose sharp edges draw blood is the \
            key image — protection through the capacity to wound those \
            who grasp carelessly.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 15]: 'The elk-sedge (Eolhx) is mostly to \
            be found in the fen, growing in the water. It wounds severely, \
            staining with blood every man who makes a grab at it.' \
            (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Heimdall, Valkyries",
        element: "Air",
        world: "Bifrost",
        esoteric: "Protection, warding, sanctuary, connection to the divine.",
        meaning_upright: "Protection, warding, higher consciousness.",
        meaning_reversed: "Hidden danger, vulnerability.",
    },
    Rune {
        number: 16,
        name: "Sigel",
        alt_names: "Sigil (OE); = EF Sowilo",
        glyph: "ᛋ",
        phoneme: "s",
        etymology: "Old English *sigel* — 'sun, jewel.' The double \
            meaning (sun and jewel) reflects the Old English kenning \
            tradition in which *sigel* (sun) and *sigle* (brooch, jewel) \
            overlap. The OE poem uses the seafarer's perspective — the \
            sun as navigator's guide. SEE HISTORICAL NOTE in Elder Futhark \
            Sowilo regarding Nazi SS appropriation.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 16]: 'The sun (Sigel) is ever a joy in \
            the hopes of seafarers when they journey away over the fishes' \
            bath, until the courser of the deep bears them to land.' \
            (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Sól, Baldur",
        element: "Fire",
        world: "Asgard",
        esoteric: "Solar will, victory, guidance, life-force.",
        meaning_upright: "Victory, clarity, solar will, success.",
        meaning_reversed: "(Symmetrical — no standard reversal.)",
    },
    Rune {
        number: 17,
        name: "Tīr",
        alt_names: "Tir (OE); = EF Tiwaz",
        glyph: "ᛏ",
        phoneme: "t",
        etymology: "Old English *Tīr* — 'the god Tiw (Tyr),' also \
            'glory, honour.' The OE poem describes Tīr as a guiding \
            star ('*tācna sum*' — 'a certain sign'), identifying it with \
            the Pole Star (as in the OI poem). Tuesday (*Tīwesdæg*) \
            preserves the name.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 17]: 'Tiw (Tir) is a guiding star; well \
            does it keep faith with princes; it is ever on its course over \
            the mists of night and never fails.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Tiw / Tyr",
        element: "Air",
        world: "Asgard",
        esoteric: "Justice, righteous victory, the Pole Star of cosmic law.",
        meaning_upright: "Justice, victory, self-sacrifice, divine order.",
        meaning_reversed: "Injustice, broken faith.",
    },
    Rune {
        number: 18,
        name: "Beorc",
        alt_names: "Beorc (OE); = EF Berkano",
        glyph: "ᛒ",
        phoneme: "b",
        etymology: "Old English *beorc* — 'birch (or poplar) tree.' \
            The OE poem's botanical identification is debated: Halsall \
            (1981) argues for birch; others suggest poplar (*Populus*). \
            The image of a tree that 'brings forth suckers' without \
            seeds from its leaves is unusual and possibly a scribal error \
            or deliberate mystification.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 18]: 'The poplar (Beorc) is without fruit; \
            yet without seed it brings forth suckers, for it is generated \
            from its leaves. Splendid are its branches and gloriously \
            adorned its lofty crown which reaches to the skies.' \
            (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Frigg, Nerthus",
        element: "Earth",
        world: "Midgard",
        esoteric: "Birth, regeneration, the Great Mother, the return of spring.",
        meaning_upright: "Birth, new beginnings, healing, nurturing.",
        meaning_reversed: "Failed growth, family troubles.",
    },
    Rune {
        number: 19,
        name: "Eh",
        alt_names: "Eh (OE); = EF Ehwaz",
        glyph: "ᛖ",
        phoneme: "e",
        etymology: "Old English *eoh* — 'horse, war-horse.' The OE poem \
            reflects the social prestige of the horse in the *comitatus* \
            (warrior fellowship): horses are 'a source of comfort to \
            the restless.' Note: *eoh* (horse, rune 19) is distinct from \
            *ēoh* (yew, rune 13) — same spelling, different vowel length \
            and meaning.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 19]: 'The horse (Eh) is a joy to princes \
            in the presence of warriors. A steed in the pride of its hoofs, \
            when rich men on horseback bandy words about it; and it is ever \
            a source of comfort to the restless.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Odin (Sleipnir), Divine Twins",
        element: "Earth / Air",
        world: "Midgard",
        esoteric: "Partnership in motion, trust, harmonious movement, \
            astral travel.",
        meaning_upright: "Partnership, trust, harmonious movement, progress.",
        meaning_reversed: "Broken trust, disharmony.",
    },
    Rune {
        number: 20,
        name: "Mann",
        alt_names: "Man (OE); = EF Mannaz",
        glyph: "ᛗ",
        phoneme: "m",
        etymology: "Old English *mann* — 'human being, person.' The OE \
            poem's meditation on mortality ('every man is doomed to fail \
            his fellow') has a Christian overlay but retains the Germanic \
            awareness of *wyrd*. The phrase 'vile carrion to the earth' \
            reflects the Old English elegiac tradition.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 20]: 'The joyous man (Mann) is dear to \
            his kinsmen; yet every man is doomed to fail his fellow, since \
            the Lord by his decree will commit the vile carrion to the earth.' \
            (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Heimdall, Odin",
        element: "Air",
        world: "Midgard",
        esoteric: "The divine human, intelligence, social cooperation.",
        meaning_upright: "Humanity, intelligence, social bonds, the self \
            in community.",
        meaning_reversed: "Isolation, self-deception.",
    },
    Rune {
        number: 21,
        name: "Lagu",
        alt_names: "Lagu (OE); = EF Laguz",
        glyph: "ᛚ",
        phoneme: "l",
        etymology: "Old English *lagu* — 'sea, water, flood.' The OE \
            poem uses the seafarer's perspective — the ocean that 'seems \
            interminable' and whose waves 'terrify.' This was lived \
            experience for the Anglo-Saxons of the North Sea coast.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 21]: 'The ocean (Lagu) seems interminable \
            to men, if they venture on the rolling bark and the waves of \
            the sea terrify them and the courser of the deep heeds not its \
            bridle.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Njord, Nerthus, Rán",
        element: "Water",
        world: "The World Ocean",
        esoteric: "The unconscious, intuition, psychic ability, flow.",
        meaning_upright: "Water, intuition, psychic ability, adaptability.",
        meaning_reversed: "Being overwhelmed, psychic attack.",
    },
    Rune {
        number: 22,
        name: "Ing",
        alt_names: "Ing (OE); = EF Ingwaz",
        glyph: "ᛝ",
        phoneme: "ng",
        etymology: "Old English *Ing* — the divine ancestor Ing. \
            The OE poem is unique in being narrative rather than \
            descriptive: 'Ing was first seen by men among the East-Danes' \
            — a mythological fragment of a migration legend, perhaps \
            relating to the Ingvaeones (a tribal confederation attested \
            by Tacitus). The 'Heardingas' who named the hero are otherwise \
            unknown.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 22]: 'Ing was first seen by men among \
            the East-Danes, till, followed by his chariot, he departed \
            eastwards over the wave. So the Heardingas named the hero.' \
            (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Freyr / Yngvi-Freyr",
        element: "Earth / Fire",
        world: "Vanaheim",
        esoteric: "The divine seed, completion of a cycle, male fertility, \
            stored potential.",
        meaning_upright: "The divine seed, gestation, completion, release.",
        meaning_reversed: "(Symmetrical — no reversal.)",
    },
    Rune {
        number: 23,
        name: "Ēðel",
        alt_names: "Ethel / Odal (OE); = EF Othala",
        glyph: "ᛟ",
        phoneme: "œ / oe",
        etymology: "Old English *ēðel* / *œðel* — 'native land, ancestral \
            property, noble estate.' The OE poem emphasises the conditional \
            joy of property: the estate is precious *if* the owner can \
            'enjoy there in his house whatever is right and proper.' \
            The *óðal* right was an inalienable ancestral land-claim. \
            SEE HISTORICAL NOTE in Elder Futhark Othala regarding Nazi \
            appropriation.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 23]: 'An estate (Œthel) is very dear to \
            every man, if he can enjoy there in his house whatever is \
            right and proper in constant prosperity.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Odin, land-spirits (*landwights*)",
        element: "Earth",
        world: "Midgard",
        esoteric: "Ancestral inheritance, homeland, collective memory.",
        meaning_upright: "Ancestral inheritance, homeland, family estate.",
        meaning_reversed: "Exile, disconnection from roots.",
    },
    Rune {
        number: 24,
        name: "Dæg",
        alt_names: "Daeg (OE); = EF Dagaz",
        glyph: "ᛞ",
        phoneme: "d",
        etymology: "Old English *dæg* — 'day.' Note: the OE Rune Poem \
            places Dæg at stanza 24 and Œthel at stanza 23, the reverse \
            of the most common Elder Futhark ordering (where Dagaz is \
            23rd and Othala 24th). The Christian overlay ('the glorious \
            light of the Creator') is strong here.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 24]: 'Day (Dæg), the glorious light of \
            the Creator, is sent by the Lord; it is beloved of men, a \
            source of hope and happiness to rich and poor, and of service \
            to all.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Heimdall, Baldur",
        element: "Fire / Air",
        world: "Asgard (the liminal threshold)",
        esoteric: "Breakthrough, the threshold moment, paradox resolved, \
            clarity after darkness.",
        meaning_upright: "Breakthrough, transformation, a new day.",
        meaning_reversed: "(Symmetrical — no reversal.)",
    },
    // ─── Five additional Futhorc runes (25–29) ──────────────────────────────
    Rune {
        number: 25,
        name: "Āc",
        alt_names: "Ac (OE) — 'Oak'",
        glyph: "ᚪ",
        phoneme: "a (short)",
        etymology: "Old English *āc* — 'oak tree (*Quercus robur*).' \
            One of the five runes added to the core Elder Futhark to \
            represent Old English sounds without Elder Futhark equivalents. \
            The oak was the tree of the Anglo-Saxon warrior aristocracy — \
            its timber built ships and halls, and its acorns fed the swine \
            of an estate's economy.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 25]: 'The oak (Ac) fattens the flesh of \
            pigs for the children of men. Often it travels over the \
            gannet's bath, and the ocean proves whether the oak keeps \
            noble faith.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Þunor / Thor (oak is thunder-god's tree across Indo-European tradition)",
        element: "Earth",
        world: "Midgard",
        esoteric: "Strength, endurance, the long-lived foundation. The oak \
            as the material basis of civilisation — ship and hall. \
            Steady, grounded power.",
        meaning_upright: "Strength, endurance, solid foundations, the \
            long-term view, material security.",
        meaning_reversed: "Rigidity, inflexibility, refusal to change.",
    },
    Rune {
        number: 26,
        name: "Æsc",
        alt_names: "Aesc (OE) — 'Ash tree'",
        glyph: "ᚫ",
        phoneme: "æ (ash-sound, a front vowel between a and e)",
        etymology: "Old English *æsc* — 'ash tree (*Fraxinus excelsior*).' \
            The Æsc letterform (Æ) is still used in English and Scandinavian \
            orthography. The ash tree was the primary wood for spear-shafts, \
            and *æsc* could mean 'spear' by metonymy. In Norse cosmology, \
            Yggdrasil is often identified as an ash (*askr*).",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 26]: 'The ash (Æsc) is exceedingly high \
            and precious to men. With its sturdy trunk it offers a stubborn \
            resistance, though attacked by many a man.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Odin (Yggdrasil as ash), the World Tree",
        element: "Air / Earth",
        world: "Yggdrasil (the World Tree itself)",
        esoteric: "The World Tree, the cosmic axis, resilience under \
            pressure. The ash's height connects earth to the heavens; \
            its resistance to attack makes it the warrior's tree.",
        meaning_upright: "Resilience, the World Tree, cosmic connection, \
            strength under pressure, high aspiration.",
        meaning_reversed: "Brittleness, false pride, overreach.",
    },
    Rune {
        number: 27,
        name: "Yr",
        alt_names: "Yr (OE) — 'Bow / Yew bow'",
        glyph: "ᚣ",
        phoneme: "y (rounded front vowel, like German ü)",
        etymology: "Old English *yr* — 'yew-wood bow' or 'horn-bow.' \
            Distinct from rune 13 (Ēoh = yew tree), this rune refers \
            specifically to the bow made of yew. The yew bow was the \
            pre-eminent weapon of the Germanic world before the advent \
            of the longbow. The OE poem's 'joy of princes and knights' \
            reflects its military prestige.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 27]: 'The bow (Yr) is a joy to princes \
            and knights. It is fitting for a warrior, a fine piece of \
            battle-equipment, when riding at speed.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Ullr (god of the bow and the hunt)",
        element: "Earth / Air",
        world: "Midgard",
        esoteric: "Precision, the warrior's craft, the archer's patience. \
            The bow that must bend to project — power through controlled \
            tension.",
        meaning_upright: "Skill, precision, the warrior's path, patience \
            before release.",
        meaning_reversed: "Misdirected energy, the arrow gone astray.",
    },
    Rune {
        number: 28,
        name: "Ior",
        alt_names: "Ior (OE) — 'Eel / Beaver / World-serpent'",
        glyph: "ᛡ",
        phoneme: "io (diphthong)",
        etymology: "Old English *ior* — possibly 'eel' or 'beaver,' though \
            the OE poem's identification is debated. The image of a \
            creature that lives in both water and land (amphibious) is \
            the interpretive key. Some scholars see a reference to \
            Jormungandr (the World Serpent), the great serpent that \
            encircles Midgard.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 28]: 'Ior is a river fish and yet it \
            always feeds on land; it has a fair abode encompassed by \
            water, where it lives in happiness.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Jormungandr (World Serpent), Nerthus",
        element: "Water / Earth",
        world: "The World Ocean / Midgard",
        esoteric: "The amphibious principle — movement between realms, \
            the creature that is at home in two worlds. Threshold-crossing \
            and dual nature.",
        meaning_upright: "Adaptability, dual nature, living between two worlds, \
            happiness despite constraints.",
        meaning_reversed: "Being trapped between two worlds, indecision.",
    },
    Rune {
        number: 29,
        name: "Ear",
        alt_names: "Ear (OE) — 'Grave / Earth'",
        glyph: "ᛠ",
        phoneme: "ea (diphthong)",
        etymology: "Old English *ear* — 'earth, grave, dust.' The \
            final rune of the standard Futhorc is a meditation on death \
            and burial — the return to earth. The OE poem is one of \
            the starkest in the sequence, with no comfort or redemption: \
            'the body is cold, the earth-fellow.' This rune has no \
            Elder Futhark equivalent.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "OE [st. 29]: 'Grave (Ear) is loathsome to every \
            knight, when the corpse quickly begins to cool and is laid in \
            the bosom of the dark earth. Prosperity declines, happiness \
            passes away and covenants are broken.' (Halsall 1981)",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Hel (goddess of the dead)",
        element: "Earth",
        world: "Helheim",
        esoteric: "Death as return to earth, the dissolution of all earthly \
            things. Ear completes the Futhorc as the inevitable end. \
            Magically used to understand mortality, ancestor-working, \
            and the composting of the past.",
        meaning_upright: "Death, ending, return to earth, the dissolution \
            of what was. Completion through dissolution.",
        meaning_reversed: "Fear of death, clinging to what must end.",
    },
    // ─── Northumbrian extensions (30–33) ────────────────────────────────────
    Rune {
        number: 30,
        name: "Cweorth",
        alt_names: "Cweorth (OE) — 'Sacred fire / Ritual fire'",
        glyph: "ᚸ",
        phoneme: "qu / cw",
        etymology: "Old English *cweorth* — proposed meaning 'fire, \
            cremation fire, ritual fire.' A Northumbrian rune attested \
            primarily in manuscript traditions (MS Vienna Cod. 795, \
            Cod. Vindob. 795; Derolez 1954). The cremation-fire \
            interpretation connects it with funeral pyres. This rune \
            is rare and its meaning genuinely uncertain.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "[No rune poem stanza — Northumbrian extension, \
            attested in manuscript runic alphabets rather than the \
            OE Rune Poem; Page 1999, pp. 81–84.]",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Odin (as master of the cremation fire), Baldur \
            (whose funeral pyre was the supreme Germanic cremation)",
        element: "Fire",
        world: "The liminal space between Midgard and Helheim",
        esoteric: "The sacred cremation fire that transforms the dead \
            and releases the soul. Cweorth is the rune of ritual \
            purification through fire and the proper completion of \
            the life-cycle.",
        meaning_upright: "Ritual fire, transformation through destruction, \
            purification, the proper completion of endings.",
        meaning_reversed: "Impure fire, desecration, incomplete transformation.",
    },
    Rune {
        number: 31,
        name: "Calc",
        alt_names: "Calc (OE) — 'Chalice / Ritual vessel'",
        glyph: "ᛣ",
        phoneme: "k (velar stop)",
        etymology: "Old English *calc* — 'chalice, cup, sandal.' \
            A Northumbrian rune of uncertain meaning. The chalice \
            interpretation connects it to ritual vessel use. \
            Some scholars relate it to Latin *calx* (lime, chalk) \
            or *calix* (cup). Page (1999, p. 83) notes the rune \
            appears in a small number of manuscript alphabets only.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "[No rune poem stanza — Northumbrian extension; \
            attested in MS Cod. Vindob. 795; Derolez 1954, pp. 416–418.]",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Mimir (guardian of the sacred well), Odin",
        element: "Water / Earth",
        world: "The Well of Urðr",
        esoteric: "The sacred vessel that holds the waters of wisdom. \
            Calc is the rune of the ritual container — the cup that \
            must be empty to receive.",
        meaning_upright: "The sacred container, receptivity, the vessel \
            of wisdom, ritual space.",
        meaning_reversed: "The empty vessel misused, receptivity turned \
            to passivity.",
    },
    Rune {
        number: 32,
        name: "Stan",
        alt_names: "Stan (OE) — 'Stone'",
        glyph: "ᛥ",
        phoneme: "st (cluster)",
        etymology: "Old English *stān* — 'stone.' A rare Northumbrian \
            rune, its phonemic value represents the *st-* cluster. \
            Stone in the Germanic world carried associations of permanence, \
            boundary-marking, and the monumental — the runestone itself \
            is a *stān* inscribed with runes. Page (1999) notes it \
            appears in very few manuscript sources.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "[No rune poem stanza — Northumbrian extension; \
            rare manuscript attestation; Page 1999, p. 83.]",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "The land-spirits (*landwights*), dwarves (as stone-shapers)",
        element: "Earth",
        world: "Midgard / Svartalfheim",
        esoteric: "Permanence, the boundary-marker, the monumental. \
            Stan is the rune of what endures — carved in stone, \
            the record outlasts the carver.",
        meaning_upright: "Permanence, endurance, solid boundaries, \
            the monument, what is carved in stone.",
        meaning_reversed: "Immovability, petrification, refusal to change.",
    },
    Rune {
        number: 33,
        name: "Gar",
        alt_names: "Gar (OE) — 'Spear (of Odin)'",
        glyph: "ᚸ",
        phoneme: "g (in some analyses) / a catch-all",
        etymology: "Old English *gār* — 'spear.' In the Northumbrian \
            tradition, Gar is sometimes placed outside the main sequence \
            as a 'rune of Odin' — the spear Gungnir with which Odin \
            sacrificed himself on Yggdrasil. Some scholars treat it as \
            a 33rd rune; others as a separate symbol. Its form combines \
            elements of other runes (it appears as a diamond or a cross \
            within a circle in manuscript sources), suggesting it may \
            be a synthesis-rune (Derolez 1954, pp. 418–420).",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "[No rune poem stanza — Gar is an extra-sequence \
            rune in Northumbrian tradition, possibly representing Odin's \
            spear Gungnir; Derolez 1954; Page 1999, pp. 83–84.]",
        rune_poem_on: "",
        rune_poem_oi: "",
        deity: "Odin (as Gungnir's wielder, as the self-sacrificed god)",
        element: "Air / Fire (the spear cast by the divine will)",
        world: "Yggdrasil / Asgard",
        esoteric: "Odin's self-sacrifice on Yggdrasil, the divine will \
            projected into the world, the rune beyond the runes. \
            As a synthesis-rune, Gar may represent the totality of \
            the Futhorc held in a single point of divine intention.",
        meaning_upright: "Divine will, Odin's sacrifice, the spear-point \
            of focused intention, the totality of the runic mystery.",
        meaning_reversed: "Scattered will, the sacrifice not made, \
            unfocused power.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anglo_saxon_has_33_runes() {
        assert_eq!(ANGLO_SAXON.len(), 33);
    }

    #[test]
    fn anglo_saxon_numbered_correctly() {
        for (i, rune) in ANGLO_SAXON.iter().enumerate() {
            assert_eq!(rune.number as usize, i + 1, "Rune '{}' has wrong number", rune.name);
        }
    }

    #[test]
    fn northumbrian_runes_are_last_four() {
        assert_eq!(ANGLO_SAXON[29].name, "Cweorth");
        assert_eq!(ANGLO_SAXON[30].name, "Calc");
        assert_eq!(ANGLO_SAXON[31].name, "Stan");
        assert_eq!(ANGLO_SAXON[32].name, "Gar");
    }
}
