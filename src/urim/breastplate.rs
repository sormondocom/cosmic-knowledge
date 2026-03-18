//! The twelve breastplate stones (Choshen Mishpat) — static data.
//!
//! Sources: Exodus 28:17–20 (Hebrew names and arrangement); gem identifications
//! from the Septuagint (LXX) translation, Josephus (*Antiquities* 3.7.5), and
//! classical tradition.  The stones are arranged in four rows of three, read
//! left-to-right, top-to-bottom, as described in Exodus 28:17–20.
//!
//! Tribe assignments follow the rabbinic tradition codified in the Talmud
//! (*Yoma* 73b) and Maimonides (*Mishneh Torah*, Hilchot Klei HaMikdash 9:7).

/// One of the twelve sacred stones of the Choshen Mishpat.
pub struct BreastplateStone {
    /// Sequential position across the breastplate, 1–12, row-major order.
    pub position: u8,
    /// Row number (1–4); three stones per row.
    pub row: u8,
    /// Column number (1–3) within the row.
    pub col: u8,
    /// Hebrew name with transliteration, e.g. "אֹדֶם · Odem".
    pub hebrew_name: &'static str,
    /// English stone identification(s) with classical variant.
    pub stone_name: &'static str,
    /// Tribe of Israel associated with this stone.
    pub tribe: &'static str,
    /// Meaning of the tribe name.
    pub tribe_meaning: &'static str,
    /// Dominant colour description.
    pub color: &'static str,
    /// Spiritual, priestly, and metaphysical attributes.
    pub attributes: &'static str,
    /// Primary scripture reference for this stone's placement.
    pub scripture: &'static str,
}

/// The complete breastplate of judgment — all twelve stones in canonical order.
///
/// Row 1 (top): Reuben · Simeon · Levi
/// Row 2:       Judah  · Issachar · Zebulun
/// Row 3:       Dan    · Naphtali · Gad
/// Row 4 (bot): Asher  · Joseph   · Benjamin
pub static BREASTPLATE: [BreastplateStone; 12] = [
    // ── Row 1 ────────────────────────────────────────────────────────────────
    BreastplateStone {
        position: 1,
        row: 1,
        col: 1,
        hebrew_name: "אֹדֶם · Odem",
        stone_name: "Ruby / Carnelian",
        tribe: "Reuben",
        tribe_meaning: "Behold, a son",
        color: "Deep red / scarlet",
        attributes: "Firstborn dignity; vitality; passion tempered by \
                      repentance.  The red fire of Odem mirrors Reuben's \
                      turbulent nature and ultimate restoration.  Associated \
                      with courage in battle and the fire of divine justice.",
        scripture: "Exodus 28:17",
    },
    BreastplateStone {
        position: 2,
        row: 1,
        col: 2,
        hebrew_name: "פִּטְדָה · Pitdah",
        stone_name: "Topaz / Chrysolite",
        tribe: "Simeon",
        tribe_meaning: "He who hears",
        color: "Yellow-green / golden-green",
        attributes: "Attentiveness to the divine voice; zeal for righteousness; \
                      the danger of unchecked passion.  Simeon's ear is attuned \
                      to the Eternal — whether for wrath or for mercy is the \
                      test of the stone.",
        scripture: "Exodus 28:17",
    },
    BreastplateStone {
        position: 3,
        row: 1,
        col: 3,
        hebrew_name: "בָּרֶקֶת · Bareketh",
        stone_name: "Emerald / Carbuncle",
        tribe: "Levi",
        tribe_meaning: "Attached / joined",
        color: "Vivid green / deep red flash",
        attributes: "Sacred service; the priestly calling; union with the \
                      divine through liturgy and sacrifice.  The Levites were \
                      attached to the sanctuary as the Bareketh is attached to \
                      the gold of the breastplate — inseparable from holiness.",
        scripture: "Exodus 28:17",
    },

    // ── Row 2 ────────────────────────────────────────────────────────────────
    BreastplateStone {
        position: 4,
        row: 2,
        col: 1,
        hebrew_name: "נֹפֶךְ · Nofekh",
        stone_name: "Carbuncle / Garnet",
        tribe: "Judah",
        tribe_meaning: "Praise / thanksgiving",
        color: "Deep crimson / dark red",
        attributes: "Royal authority; the messianic lineage; the lion's \
                      courage.  Judah's stone blazes like a king's seal — the \
                      sceptre that shall not depart (Genesis 49:10).  Emblem \
                      of sovereignty under the Most High.",
        scripture: "Exodus 28:18",
    },
    BreastplateStone {
        position: 5,
        row: 2,
        col: 2,
        hebrew_name: "סַפִּיר · Sapir",
        stone_name: "Sapphire / Lapis Lazuli",
        tribe: "Issachar",
        tribe_meaning: "There is reward",
        color: "Celestial blue / deep blue with gold flecks",
        attributes: "Heavenly wisdom; the contemplative life; reward for \
                      faithful study.  Issachar dwelt in his tents studying \
                      the Torah; his stone is the colour of the vault of heaven \
                      — a window into the divine mind (Exodus 24:10).",
        scripture: "Exodus 28:18",
    },
    BreastplateStone {
        position: 6,
        row: 2,
        col: 3,
        hebrew_name: "יַהֲלֹם · Yahalom",
        stone_name: "Diamond / Onyx",
        tribe: "Zebulun",
        tribe_meaning: "Dwelling / honour",
        color: "Clear / brilliant white, or banded black-white",
        attributes: "Commerce and generosity; the partnership between \
                      worldly means and sacred study.  Zebulun traded by sea \
                      to sustain Issachar's Torah; his stone is diamond-hard \
                      in integrity and brilliant in transparent dealing.",
        scripture: "Exodus 28:18",
    },

    // ── Row 3 ────────────────────────────────────────────────────────────────
    BreastplateStone {
        position: 7,
        row: 3,
        col: 1,
        hebrew_name: "לֶשֶׁם · Leshem",
        stone_name: "Jacinth / Amber",
        tribe: "Dan",
        tribe_meaning: "He judged",
        color: "Orange-red / amber",
        attributes: "Justice; the capacity to judge with discernment.  Dan \
                      shall judge his people (Genesis 49:16).  The amber light \
                      of Leshem illuminates hidden truths and penetrates \
                      darkness — the lantern of the magistrate.",
        scripture: "Exodus 28:19",
    },
    BreastplateStone {
        position: 8,
        row: 3,
        col: 2,
        hebrew_name: "שְׁבוֹ · Shevo",
        stone_name: "Agate / Turquoise",
        tribe: "Naphtali",
        tribe_meaning: "My wrestling",
        color: "Banded grey-white / sky blue-green",
        attributes: "Swiftness; eloquence; the beauty of natural revelation. \
                      Naphtali is a swift deer who brings beautiful words \
                      (Genesis 49:21).  Shevo carries both the suppleness of \
                      water and the firmness of stone — speech that heals.",
        scripture: "Exodus 28:19",
    },
    BreastplateStone {
        position: 9,
        row: 3,
        col: 3,
        hebrew_name: "אַחְלָמָה · Ahlamah",
        stone_name: "Amethyst",
        tribe: "Gad",
        tribe_meaning: "Fortune / a troop",
        color: "Purple / violet",
        attributes: "Fortitude in adversity; the warrior's purple; resistance \
                      to intoxication by worldly things.  Gad the troop-leader \
                      forded the Jordan at the head of the armies.  Amethyst \
                      guards the mind in the heat of battle.",
        scripture: "Exodus 28:19",
    },

    // ── Row 4 ────────────────────────────────────────────────────────────────
    BreastplateStone {
        position: 10,
        row: 4,
        col: 1,
        hebrew_name: "תַּרְשִׁישׁ · Tarshish",
        stone_name: "Beryl / Chrysolite",
        tribe: "Asher",
        tribe_meaning: "Happy / blessed",
        color: "Sea-green / aquamarine",
        attributes: "Abundance; the blessings of the earth; hospitality and \
                      joy.  Asher's bread shall be fat and he shall yield \
                      royal dainties (Genesis 49:20).  Tarshish glows like \
                      the sea at the edge of the world — wealth flowing from \
                      the favour of heaven.",
        scripture: "Exodus 28:20",
    },
    BreastplateStone {
        position: 11,
        row: 4,
        col: 2,
        hebrew_name: "שֹׁהַם · Shoham",
        stone_name: "Onyx / Malachite",
        tribe: "Joseph (Ephraim & Manasseh)",
        tribe_meaning: "May He add",
        color: "Banded black-white / deep green",
        attributes: "Dreams and their interpretation; fruitfulness; endurance \
                      through suffering toward triumph.  Joseph wore two \
                      crowns through Ephraim and Manasseh — the double \
                      portion of the dreamer.  Shoham is the stone also set \
                      in the High Priest's shoulder pieces (Exodus 28:9).",
        scripture: "Exodus 28:20",
    },
    BreastplateStone {
        position: 12,
        row: 4,
        col: 3,
        hebrew_name: "יָשְׁפֶה · Yashfeh",
        stone_name: "Jasper",
        tribe: "Benjamin",
        tribe_meaning: "Son of the right hand",
        color: "Red-brown / variegated",
        attributes: "Beloved proximity to the divine; the Temple's home \
                      territory; the ravenous wolf redeemed to generous \
                      hospitality.  Benjamin received the largest priestly \
                      blessing (Deuteronomy 33:12): 'The beloved of the LORD \
                      shall dwell in safety by Him.'  Jasper is the final \
                      stone and the foundation of the New Jerusalem \
                      (Revelation 21:19).",
        scripture: "Exodus 28:20",
    },
];
