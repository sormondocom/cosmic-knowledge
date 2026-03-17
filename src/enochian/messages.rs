//! Enochian root meanings, angelic call messages, and the 19 Enochian Keys.
//!
//! Source: Dee's diaries (Cotton MS Appendix XLVI); Crowley's *Liber Chanokh* (1909).

/// The 19 Enochian Keys (Calls) as recorded by John Dee.
///
/// Each entry is `(key_number, title, opening_enochian_words, english_rendering)`.
pub const ENOCHIAN_KEYS: &[(u32, &str, &str, &str)] = &[
    ( 1, "The First Key",     "MICMA GOHO MAD ZIR",
      "I reign over you, saith the God of Justice. In power exalted above the firmaments of wrath; in whose hands the Sun is as a sword and the Moon as a through-thrusting fire." ),
    ( 2, "The Second Key",    "ADGT VPAAH ZONG OM",
      "Can the wings of the winds understand your voices of wonder? O you the second flame, the house of justice, who art mightier than the evening wolves." ),
    ( 3, "The Third Key",     "MICAOLI BERANUSAJI",
      "Behold, saith your God, I am a circle on whose hands stand twelve kingdoms. Six are the seats of living breath; the rest are as sharp sickles, or the horns of death." ),
    ( 4, "The Fourth Key",    "OTHIL LASDI BABAGE",
      "I have set my feet in the South, and have looked about me, saying: Are not the thunders of increase numbered 33 which reign in the second angle?" ),
    ( 5, "The Fifth Key",     "SAPAH ZIMII DU-I-BE",
      "The mighty sounds have entered into the third angle, and are become as olives in the olive mount, looking with gladness upon the earth." ),
    ( 6, "The Sixth Key",     "GAHE SADiv FIEN",
      "The spirits of the fourth angle are nine, mighty in the firmament of waters; whom the first hath planted a torment to the wicked and a garland to the righteous." ),
    ( 7, "The Seventh Key",   "RAAS I SALMAN",
      "The East is a house of virgins singing praises among the flames of first glory, wherein the Lord hath opened his mouth; and they are become 28 living dwellings." ),
    ( 8, "The Eighth Key",    "BAZMELO I TA PIRIPSON",
      "The midday, the first, is as the third heaven made of hyacinth pillars 26; in whom the second beginning of things are and wax strong, which also successively are the number of time." ),
    ( 9, "The Ninth Key",     "MICAOLI BRANSG PIAD",
      "A mighty guard of fire with two-edged swords flaming (which have vials 8 of wrath for two times and a half, whose wings are of wormwood and of the marrow of salt)." ),
    (10, "The Tenth Key",     "CORAXO CHIS CORMP",
      "The thunders of judgment and wrath are numbered and are harboured in the North, in the likeness of an oak whose branches are nests of lamentation and weeping." ),
    (11, "The Eleventh Key",  "OXIAYAL HOLDO",
      "The mighty seat groaned and they were five thunders which flew into the East; and the Eagle spake and cried aloud: Come away from the house of death." ),
    (12, "The Twelfth Key",   "NONCI DSONF BABAGE",
      "O you that reign in the South, and are 28; the lanterns of sorrow — bind up your girdles and visit us! Bring down your train 3663 that the Lord may be magnified." ),
    (13, "The Thirteenth Key","NAPEAI BABAGEN",
      "O you swords of the South which have 42 eyes to stir up the wrath of sin, making men drunken which are empty: Behold the Promise of God and his power." ),
    (14, "The Fourteenth Key","NOROMI BAGLE",
      "O you sons of fury, the daughters of the Just One! That sit upon 24 seats, vexing all creatures of the Earth with age — that have under you 1636." ),
    (15, "The Fifteenth Key", "ILS TABAAN LIXIPSP",
      "O thou, the governor of the first flame, under whose wings are 6739; that weave the Earth with dryness; which knowest the great name Righteousness." ),
    (16, "The Sixteenth Key", "ILS VIVIALPRT SALMAN",
      "O thou second flame, the house of justice, which hast thy beginning in glory and shalt comfort the just — which walkest on the earth with feet 8763." ),
    (17, "The Seventeenth Key","ILS DIAL PEOC",
      "O thou third flame, whose wings are thorns to stir up vexation, and who hast 7336 living lamps going before thee — whose God is wrath in anger." ),
    (18, "The Eighteenth Key","ILS MICAOLI CHIS",
      "O thou mighty light and burning flame of comfort, that unveilest the glory of God to the centre of the Earth, in whom the secrets of truth have their abiding." ),
    (19, "The Nineteenth Key (The Aethyr Call)", "MADRIAAX DS PRAF",
      "O you heavens which dwell in the first air, ye are mighty in the parts of the earth and execute the judgment of the highest! To you it is said: Behold the face of your God." ),
];

/// Root-number meanings framed through John Dee's Enochian / angelic-call cosmology.
pub fn enochian_meaning(root: u32) -> &'static str {
    match root {
        1 => "🜁 Un — The Primal Fire; the First Aethyr speaks; a new cycle of angelic emanation begins",
        2 => "🜂 Pa — The Dual Watchtowers; balance between the elemental tablets; duality of creation",
        3 => "🜃 Veh — The Trinitarian Call; Dee's Three Books of Mystery; mind, will, and word aligned",
        4 => "🜄 Gal — The Four Watchtowers (Earth, Air, Fire, Water); the Great Table of the Universe",
        5 => "⊕ Graph — The Fifth element, Spirit; the Tablet of Union (EXARP·HCOMA·NANTA·BITOM)",
        6 => "✡ Or — Six-fold symmetry of the Sigillum Dei Aemeth; harmony of the celestial spheres",
        7 => "☽ Ged — The Seven Heptarchic Kings; Dee's seven planetary governors of divine mystery",
        8 => "♄ Na — The Eight Temples of Heptarchia Mystica; the infinite angels of the lower aethyrs",
        9 => "☉ Gon — The 91 Governors across the 30 Aethyrs (91=7×13); completion of the divine plan",
        _ => "🌌 Beyond the 30th Aethyr — TEX dissolves into the Limitless Light",
    }
}

/// Enochian angelic call message, inspired by the 19 Enochian Keys.
pub fn enochian_angelic_message(root: u32) -> &'static str {
    match root {
        1 => "MICMA — I reign over you, saith the God of Justice. The first call opens the gates of the celestial city.",
        2 => "ADGT — The wings of the winds understand your voices of wonder. Two pillars sustain the heavens.",
        3 => "MICAOLI — Behold, saith your God, I am a circle on whose hands stand Twelve Kingdoms. Three-fold is the light.",
        4 => "OTHIL — I have set my feet in the South, and have looked about me, saying: are not the thunders of increase numbered 33?",
        5 => "SAPAH — The mighty sounds have entered into the third angle, and are become as olives in the olive mount.",
        6 => "GAHE — The spirits of the fourth angle are nine mighty in the firmament of waters; they frame the earth with 46 voices of creation.",
        7 => "RAAS — The East is a house of virgins singing praises among the flames of first glory. Seek the seventh Aethyr.",
        8 => "BAZMELO — The midday of the first is as the third heaven made of hyacinth pillars, 26 in number.",
        9 => "TELOCH — A mighty guard of fire with two-edged swords of ice; the name of their God is Baligon. All is accomplished.",
        _ => "OL SONF VORSG — I reign over you in power exalted above the firmaments of wrath. Enter into the silence.",
    }
}
