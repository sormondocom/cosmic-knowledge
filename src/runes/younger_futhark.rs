//! Younger Futhark — the Viking Age runic alphabet (c. 750–1100 CE).
//!
//! The Younger Futhark is a reduction of the Elder Futhark from 24 to 16
//! runes, paradoxically occurring at the same time that Old Norse was
//! *expanding* its phoneme inventory.  The result is that a single rune
//! must represent multiple sounds.  This reduction is not fully explained
//! by scholarship; the most commonly accepted theory is that the
//! simplification served the practical needs of rapid carving.
//!
//! There are two main graphic variants:
//!
//! - **Long-branch runes** (Danish): the standard textbook form, used
//!   primarily in Denmark and coastal areas.
//! - **Short-twig runes** (Swedish-Norwegian / *stavlösa*): a simplified
//!   form used in Sweden and Norway, with shorter, angular strokes.
//!
//! The Younger Futhark is attested on thousands of runic inscriptions,
//! most famously the approximately 2,500 surviving Viking Age runestones.
//!
//! ## Notable semantic shifts from Elder Futhark
//!
//! Several runes changed meaning dramatically as the alphabet contracted:
//!
//! | Rune | Elder Futhark meaning | Younger Futhark meaning |
//! |------|-----------------------|------------------------|
//! | ᚢ Úr  | Aurochs (*ūruz*)     | Drizzle/slag (*úr*)    |
//! | ᚴ Kaun | Torch (*kaunan*)    | Ulcer/sore (*kaun*)    |
//!
//! ## Sources
//!
//! - Page, R.I. *An Introduction to English Runes*, 2nd ed. Woodbridge:
//!   Boydell Press, 1999.
//! - Spurkland, Terje. *Norwegian Runes and Runic Inscriptions*.
//!   Woodbridge: Boydell Press, 2005.
//! - Jansson, Sven B.F. *Runes in Sweden*, trans. P. Foote. Stockholm:
//!   Gidlunds, 1987.
//! - Barnes, Michael P. *Runes: A Handbook*. Woodbridge: Boydell Press, 2012.
//!
//! Rune poem stanzas follow Page (1999) translations of the Old Norwegian
//! (ON) and Old Icelandic (OI) Rune Poems.

use super::Rune;

/// The 16 runes of the Younger Futhark in canonical order.
pub static YOUNGER_FUTHARK: &[Rune] = &[
    Rune {
        number: 1,
        name: "Fé",
        alt_names: "Fe (ON), Fé (OI); = Elder Futhark Fehu",
        glyph: "ᚠ",
        phoneme: "f",
        etymology: "Old Norse *fé* — 'cattle, wealth, money.' \
            Unchanged in both form and primary meaning from Elder Futhark Fehu. \
            The social tension between wealth-as-gift and wealth-as-strife \
            is highlighted in both rune poems.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "ON [st. 1]: 'Wealth (Fé) causes strife among kinsmen; \
            the wolf grows up in the forest.' (Page 1999)",
        rune_poem_oi: "OI [st. 1]: 'Wealth (Fé) — source of discord among \
            kinsmen, fire of the sea, path of the serpent.' (Page 1999)",
        deity: "Freyr, Njord",
        element: "Fire",
        world: "Vanaheim",
        esoteric: "Mobile wealth and its double nature — generosity and \
            strife. The wolf growing in the forest is the consequence \
            of hoarded rather than distributed wealth.",
        meaning_upright: "Prosperity, earned wealth, success, abundance.",
        meaning_reversed: "",
    },
    Rune {
        number: 2,
        name: "Úr",
        alt_names: "Ur (ON/OI); cf. Elder Futhark Uruz",
        glyph: "ᚢ",
        phoneme: "u",
        etymology: "Old Norse *úr* — in the Younger Futhark context primarily \
            'drizzle, slag (from iron-smelting),' though the word 'aurochs' \
            (*ūruz*) may still underlie the rune name. The ON poem's \
            stanza is ambiguous, and the OI poem clearly uses *úr* = \
            'shower/rain.' This semantic shift (from the aurochs of the \
            Elder Futhark to rain/dross) is historically significant.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "ON [st. 2]: 'Dross (Ur) comes from bad iron; \
            the reindeer often races over the frozen snow.' (Page 1999)",
        rune_poem_oi: "OI [st. 2]: 'Shower (Ur) — weeping of the clouds, \
            ruin of the hay-harvest, hatred of the herdsman.' (Page 1999)",
        deity: "Thor (storm-aspect)",
        element: "Water / Earth",
        world: "Midgard",
        esoteric: "Primal vitality and the paradox of strength — the aurochs \
            gone, leaving only rain and slag, yet the memory of primal \
            power remains. Used in healing and strengthening.",
        meaning_upright: "Vitality, strength, health, primal force.",
        meaning_reversed: "",
    },
    Rune {
        number: 3,
        name: "Þurs",
        alt_names: "Thurs (ON), Þurs (OI); = Elder Futhark Thurisaz",
        glyph: "ᚦ",
        phoneme: "þ/ð (th)",
        etymology: "Old Norse *þurs* — 'giant, thurse, harmful being.' \
            The ON and OI poems are unusually explicit about misogynistic \
            harm, reflecting Viking Age attitudes toward giant-kind. \
            The form and basic meaning are unchanged from Elder Futhark.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "ON [st. 3]: 'Giant (Thurs) causes anguish to women; \
            misfortune makes few men cheerful.' (Page 1999)",
        rune_poem_oi: "OI [st. 3]: 'Giant (Þurs) — torment of women, \
            cliff-dweller, husband of a giantess, Saturn's thane.' \
            (Page 1999)",
        deity: "Thor (as slayer of giants), Loki",
        element: "Fire / Ice",
        world: "Jotunheim",
        esoteric: "Directed, potentially destructive force. Gateway-breaking. \
            Protection through controlled aggression.",
        meaning_upright: "Directed force, obstacles overcome, protection.",
        meaning_reversed: "",
    },
    Rune {
        number: 4,
        name: "Óss",
        alt_names: "Oss (ON), Áss (OI); cf. Elder Futhark Ansuz",
        glyph: "ᚬ",
        phoneme: "o / a",
        etymology: "Old Norse *óss* — '(river-)mouth, estuary.' A dramatic \
            semantic shift from Elder Futhark *ansuz* (a god, divine being). \
            In the Viking Age, *áss* (god) was still a living word, but \
            the ON Rune Poem uses *óss* (estuary); the OI poem reverts \
            to the older 'god' (*áss*) meaning. The rune thus carries \
            both senses simultaneously.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "ON [st. 4]: 'Estuary (Óss) is the way of most \
            journeys; but a scabbard is of swords.' (Page 1999)",
        rune_poem_oi: "OI [st. 4]: 'God (Áss) — aged Gautr [Odin] and \
            prince of Ásgarðr, and lord of Valholl.' (Page 1999)",
        deity: "Odin",
        element: "Air",
        world: "Asgard / the estuary of worlds",
        esoteric: "Divine communication, the spoken word, Odin's wisdom. \
            The estuary sense adds the meaning of confluence — where \
            rivers meet the sea, disparate forces merge.",
        meaning_upright: "Communication, divine inspiration, wisdom, \
            Odin's blessing.",
        meaning_reversed: "",
    },
    Rune {
        number: 5,
        name: "Reið",
        alt_names: "Reid (ON), Reið (OI); = Elder Futhark Raidho",
        glyph: "ᚱ",
        phoneme: "r",
        etymology: "Old Norse *reið* — 'riding, vehicle, chariot.' \
            Unchanged from Elder Futhark Raidho in both form and meaning. \
            Reginn the smith (mentioned in the ON poem) is the dwarf who \
            forged the sword Gramr for Sigurðr.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "ON [st. 5]: 'Riding (Reið) is said to be the worst \
            thing for horses; Reginn forged the finest sword.' (Page 1999)",
        rune_poem_oi: "OI [st. 5]: 'Riding (Reið) — joy of the horsemen \
            and speedy journey and toil of the steed.' (Page 1999)",
        deity: "Thor (sky-rider), Odin (wanderer)",
        element: "Air",
        world: "Midgard (all roads)",
        esoteric: "Journey, right action, cosmic order, alignment with \
            the rhythm of time.",
        meaning_upright: "Journey, right action, good timing, orderly progress.",
        meaning_reversed: "",
    },
    Rune {
        number: 6,
        name: "Kaun",
        alt_names: "Kaunar (ON), Kaun (OI); cf. Elder Futhark Kenaz",
        glyph: "ᚴ",
        phoneme: "k / g",
        etymology: "Old Norse *kaun* — 'ulcer, sore, boil.' A striking \
            semantic shift from Elder Futhark *kaunan* (torch). The \
            torch sense (illumination, creative fire) was replaced by \
            its dark mirror — the festering wound, disease, and death. \
            Both poems emphasise mortality. The rune's phoneme also \
            broadened to cover both /k/ and /g/ sounds.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "ON [st. 6]: 'Ulcer (Kaun) is fatal to children; \
            death makes a corpse pale.' (Page 1999)",
        rune_poem_oi: "OI [st. 6]: 'Ulcer (Kaun) — disease fatal to \
            children and painful spot and abode of mortification.' \
            (Page 1999)",
        deity: "Hel (as goddess of sickness and death)",
        element: "Fire (turned sour — infection rather than illumination)",
        world: "Helheim",
        esoteric: "The shadow of Kenaz — where creative fire turns to \
            consuming infection. A rune of crisis, illness, and the \
            need for purification. Magically used to understand and \
            neutralise corrupting forces.",
        meaning_upright: "Disease, crisis, a wound that must be examined, \
            hidden corruption brought to light.",
        meaning_reversed: "",
    },
    Rune {
        number: 7,
        name: "Hagall",
        alt_names: "Hagall (ON/OI); = Elder Futhark Hagalaz",
        glyph: "ᚼ",
        phoneme: "h",
        etymology: "Old Norse *hagl* — 'hail.' The Younger Futhark \
            form often differs graphically from the Elder (an H-shape \
            rather than the two-stave form). The ON poem adds a \
            cosmological dimension by referencing Odin's role in shaping \
            the world.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "ON [st. 7]: 'Hail (Hagall) is the coldest of \
            grains; Odin shaped the world in ancient times.' (Page 1999)",
        rune_poem_oi: "OI [st. 7]: 'Hail (Hagall) — cold grain and \
            shower of sleet and sickness of serpents.' (Page 1999)",
        deity: "Odin (as shaper of the world from primordial ice)",
        element: "Ice",
        world: "Niflheim",
        esoteric: "The cosmic matrix, disruption as creative force, \
            the seed pattern of all transformation.",
        meaning_upright: "Disruption, unavoidable change, elemental force, \
            transformation through crisis.",
        meaning_reversed: "",
    },
    Rune {
        number: 8,
        name: "Nauðr",
        alt_names: "Naudr (ON), Nauð (OI); = Elder Futhark Nauthiz",
        glyph: "ᚾ",
        phoneme: "n",
        etymology: "Old Norse *nauðr* — 'need, necessity, compulsion, \
            distress.' Unchanged from Elder Futhark Nauthiz. The crossed- \
            stave glyph represents the fire-drill (*nauðfire*).",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "ON [st. 8]: 'Constraint (Nauðr) gives scant choice; \
            a naked man is chilled by frost.' (Page 1999)",
        rune_poem_oi: "OI [st. 8]: 'Constraint (Nauð) — grief of the \
            bond-maid and state of oppression and toilsome work.' \
            (Page 1999)",
        deity: "The Norns",
        element: "Fire (necessity-fire)",
        world: "The Well of Urðr",
        esoteric: "Fate-as-constraint. The friction that generates \
            transformation. Patient work within limitation.",
        meaning_upright: "Need, necessity, patience, wisdom in hardship, \
            recognising what is truly required.",
        meaning_reversed: "",
    },
    Rune {
        number: 9,
        name: "Íss",
        alt_names: "Iss (ON), Íss (OI); = Elder Futhark Isa",
        glyph: "ᛁ",
        phoneme: "i",
        etymology: "Old Norse *íss* — 'ice.' Unchanged from Elder Futhark. \
            The ON poem's 'broad bridge' refers to a frozen river — ice \
            as a dangerous but traversable path, requiring careful guidance.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "ON [st. 9]: 'Ice (Iss) we call the broad bridge; \
            the blind man must be led.' (Page 1999)",
        rune_poem_oi: "OI [st. 9]: 'Ice (Íss) — bark of rivers and roof \
            of the wave and destruction of the doomed.' (Page 1999)",
        deity: "Niflheim's rulers, Rind",
        element: "Ice",
        world: "Niflheim",
        esoteric: "Stillness, concentration, the frozen axis of the world. \
            Halts unwanted processes; strengthens boundaries.",
        meaning_upright: "Stillness, halt, introspection, the still point.",
        meaning_reversed: "",
    },
    Rune {
        number: 10,
        name: "Ár",
        alt_names: "Ar (ON), Ár (OI); = Elder Futhark Jera",
        glyph: "ᛅ",
        phoneme: "a (long)",
        etymology: "Old Norse *ár* — 'year, (good) harvest, bountiful \
            season.' The harvest meaning of Elder Futhark Jera is \
            preserved, with the cyclical aspect emphasised. The reference \
            to Fróðr (Fróði) in the ON poem invokes the mythological \
            'Peace of Fróðr' — a golden age of abundance.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "ON [st. 10]: 'Harvest (Ár) is a boon to men; \
            I say that Fróðr was generous.' (Page 1999)",
        rune_poem_oi: "OI [st. 10]: 'Harvest (Ár) — boon of men and \
            good summer and thriving crops.' (Page 1999)",
        deity: "Freyr (harvest-god)",
        element: "Earth",
        world: "Midgard / Vanaheim",
        esoteric: "The sacred year-cycle, patient fruition, the harvest \
            of past effort. Right timing.",
        meaning_upright: "The harvest, reward for effort, good timing, \
            patience, earned fruition.",
        meaning_reversed: "",
    },
    Rune {
        number: 11,
        name: "Sól",
        alt_names: "Sol (ON/OI); = Elder Futhark Sowilo",
        glyph: "ᛋ",
        phoneme: "s",
        etymology: "Old Norse *sól* — 'sun.' Unchanged in meaning from \
            Elder Futhark Sowilo. The graphic form of the Younger Futhark \
            Sól (ᛋ) is a single S-stroke, lighter than the Elder Futhark's \
            zigzag. The 'divine decree' of the ON poem reflects the \
            ordered movement of the sun.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "ON [st. 11]: 'Sun (Sól) is the light of the lands; \
            I bow to the divine decree.' (Page 1999)",
        rune_poem_oi: "OI [st. 11]: 'Sun (Sól) — shield of the clouds \
            and shining ray and destroyer of ice.' (Page 1999)",
        deity: "Sól (sun-goddess), Baldur",
        element: "Fire",
        world: "Asgard",
        esoteric: "Victory, solar will, guidance, life-force. The destroyer \
            of ice — Sól and Íss as cosmic opposites.",
        meaning_upright: "Victory, clarity, solar will, success, life-force.",
        meaning_reversed: "",
    },
    Rune {
        number: 12,
        name: "Týr",
        alt_names: "Tyr (ON/OI); = Elder Futhark Tiwaz",
        glyph: "ᛏ",
        phoneme: "t",
        etymology: "Old Norse *Týr* — the god Tyr. The 'one-handed god' \
            reference is to Tyr's sacrifice of his sword-hand in binding \
            Fenrir. The smith's bellows in the ON poem are unexplained \
            by scholars — possibly a kenning for warfare's tools.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "ON [st. 12]: 'Tyr (Týr) is the one-handed god; \
            often the smith has to blow.' (Page 1999)",
        rune_poem_oi: "OI [st. 12]: 'Tyr (Týr) — one-handed god and \
            leavings of the wolf and prince of temples.' (Page 1999)",
        deity: "Tyr",
        element: "Air / Fire",
        world: "Asgard",
        esoteric: "Justice, righteous victory, self-sacrifice for right \
            order. The Pole Star — the fixed point of cosmic law.",
        meaning_upright: "Justice, victory, self-sacrifice, divine order.",
        meaning_reversed: "",
    },
    Rune {
        number: 13,
        name: "Bjarkan",
        alt_names: "Bjaur (ON), Bjarkan (OI); = Elder Futhark Berkano",
        glyph: "ᛒ",
        phoneme: "b",
        etymology: "Old Norse *björk* — 'birch tree.' The OI poem's \
            'leafy twig' (*laufgrœnn limar*) is a kenning for spring and \
            renewal. Loki's mention in the ON poem is obscure — possibly \
            referencing trickery associated with the spring thaw.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "ON [st. 13]: 'Birch (Bjarkan) is the limb \
            greenest with leaves; Loki brought the luck of deceit.' \
            (Page 1999)",
        rune_poem_oi: "OI [st. 13]: 'Birch (Bjarkan) — leafy twig and \
            little tree and fresh young shrub.' (Page 1999)",
        deity: "Frigg, Nerthus",
        element: "Earth",
        world: "Midgard / Vanaheim",
        esoteric: "Birth, regeneration, nurturing, the Great Mother. \
            The return of spring.",
        meaning_upright: "Birth, new beginnings, healing, nurturing, regeneration.",
        meaning_reversed: "",
    },
    Rune {
        number: 14,
        name: "Maðr",
        alt_names: "Madr (ON), Maðr (OI); = Elder Futhark Mannaz",
        glyph: "ᛘ",
        phoneme: "m",
        etymology: "Old Norse *maðr* — 'man, human being.' The OI poem's \
            'delight of man' (*mannfeginn maðr*) emphasises humanity's \
            social nature; 'adorner of ships' connects the human to the \
            sea-going culture of the Viking Age.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "ON [st. 14]: 'Man (Maðr) is an augmentation of \
            the dust; great is the claw of the hawk.' (Page 1999)",
        rune_poem_oi: "OI [st. 14]: 'Man (Maðr) — delight of man and \
            augmentation of the earth and adorner of ships.' (Page 1999)",
        deity: "Heimdall (as father of the classes), Odin",
        element: "Air",
        world: "Midgard",
        esoteric: "The divine human, intelligence, social cooperation, \
            the self in community.",
        meaning_upright: "Humanity, intelligence, social bonds, the self \
            within community.",
        meaning_reversed: "",
    },
    Rune {
        number: 15,
        name: "Lögr",
        alt_names: "Logr (ON), Lögr (OI); = Elder Futhark Laguz",
        glyph: "ᛚ",
        phoneme: "l",
        etymology: "Old Norse *lögr* — 'water, sea, liquid.' The ON poem's \
            'waterfall' (*fors*) and 'gold objects' (*gullr) kenning is \
            cryptic — gold cast into water was a known votive practice. \
            The OI 'land of fish' (*fiskr byggvir*) emphasises the sea \
            as a distinct, inhabited realm.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "ON [st. 15]: 'A waterfall (Lögr) is that which \
            falls from the mountain-side; but gold objects are costly things.' \
            (Page 1999)",
        rune_poem_oi: "OI [st. 15]: 'Water (Lögr) — eddying stream and \
            broad geyser and land of fish.' (Page 1999)",
        deity: "Njord, Rán, Nerthus",
        element: "Water",
        world: "The World Ocean",
        esoteric: "The unconscious, intuition, psychic ability, the life-force \
            flowing through all things.",
        meaning_upright: "Water, intuition, psychic ability, flow, adaptability.",
        meaning_reversed: "",
    },
    Rune {
        number: 16,
        name: "Ýr",
        alt_names: "Yr (ON/OI); cf. Elder Futhark Eihwaz / OE Yr",
        glyph: "ᛦ",
        phoneme: "ʀ (R, a back-of-mouth rhotic distinct from /r/)",
        etymology: "Old Norse *ýr* — 'yew tree (*Taxus baccata*),' \
            or 'yew bow.' The Younger Futhark Ýr represents the \
            phoneme *ʀ* (a distinct sound from *r*), which derives \
            from Proto-Germanic final *-z*. The yew meaning connects \
            it to Elder Futhark Eihwaz (also yew), though the two \
            runes are distinct. The OI poem's 'bent bow' (*boginn \
            bogr*) references the war-bow made of yew.",
        aett: 0,
        aett_name: "",
        rune_poem_oe: "",
        rune_poem_on: "ON [st. 16]: 'Yew (Yr) is the greenest of \
            trees in winter; it is wont to crackle when it burns.' \
            (Page 1999)",
        rune_poem_oi: "OI [st. 16]: 'Yew (Ýr) — bent bow and brittle \
            iron and giant of the arrow.' (Page 1999)",
        deity: "Ullr (god of the bow and hunt)",
        element: "Earth / Fire",
        world: "All Nine Worlds (the yew grows across the boundary \
            of life and death)",
        esoteric: "The yew's paradox — evergreen yet poisonous, living \
            beside the dead — makes Ýr a rune of death-wisdom, \
            the bow-shot of fate, and endurance through extremity.",
        meaning_upright: "Endurance, death-wisdom, the archer's precision, \
            the yew's paradox of death-in-life.",
        meaning_reversed: "",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn younger_futhark_has_16_runes() {
        assert_eq!(YOUNGER_FUTHARK.len(), 16);
    }

    #[test]
    fn younger_futhark_numbered_correctly() {
        for (i, rune) in YOUNGER_FUTHARK.iter().enumerate() {
            assert_eq!(rune.number as usize, i + 1, "Rune '{}' has wrong number", rune.name);
        }
    }
}
