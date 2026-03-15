//! Enochian root meanings and angelic call messages.

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
