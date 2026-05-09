//! Quran search — Pickthall English translation (1930), public domain.
//!
//! Source: Project Gutenberg EBook #16955.
//!
//! ## Reference syntax
//!
//! - By surah number and ayah: `2:255`
//! - By surah name:            `Al-Baqara 255`  or  `baqara 255`
//! - Browse a whole surah:     `2`  or  `Al-Baqara`
//!
//! ## Search syntax (FTS5)
//!
//! - Plain words: `mercy guidance`
//! - Exact phrase: `"straight path"`
//! - Prefix: `mercif*`
//! - Boolean: `light AND NOT darkness`

mod session;
pub mod verses_data;
pub use session::run_quran_session;

// ─── Surah table ─────────────────────────────────────────────────────────────

/// One row of the surah reference table.
pub struct Surah {
    pub number: u32,
    pub name: &'static str,
    pub english: &'static str,
    pub ayahs: u32,
}

/// All 114 surahs in canonical order.
pub static SURAHS: &[Surah] = &[
    Surah { number:   1, name: "Al-Fatiha",     english: "The Opening",               ayahs:   7 },
    Surah { number:   2, name: "Al-Baqara",     english: "The Cow",                   ayahs: 286 },
    Surah { number:   3, name: "Al-Imran",      english: "The Family of Imran",       ayahs: 200 },
    Surah { number:   4, name: "An-Nisa",       english: "The Women",                 ayahs: 176 },
    Surah { number:   5, name: "Al-Maida",      english: "The Table",                 ayahs: 120 },
    Surah { number:   6, name: "Al-Anam",       english: "The Cattle",                ayahs: 165 },
    Surah { number:   7, name: "Al-Araf",       english: "The Heights",               ayahs: 206 },
    Surah { number:   8, name: "Al-Anfal",      english: "The Spoils of War",         ayahs:  75 },
    Surah { number:   9, name: "At-Tawba",      english: "The Repentance",            ayahs: 129 },
    Surah { number:  10, name: "Yunus",         english: "Jonah",                     ayahs: 109 },
    Surah { number:  11, name: "Hud",           english: "Hud",                       ayahs: 123 },
    Surah { number:  12, name: "Yusuf",         english: "Joseph",                    ayahs: 111 },
    Surah { number:  13, name: "Ar-Rad",        english: "The Thunder",               ayahs:  43 },
    Surah { number:  14, name: "Ibrahim",       english: "Abraham",                   ayahs:  52 },
    Surah { number:  15, name: "Al-Hijr",       english: "The Rock",                  ayahs:  99 },
    Surah { number:  16, name: "An-Nahl",       english: "The Bee",                   ayahs: 128 },
    Surah { number:  17, name: "Al-Isra",       english: "The Night Journey",         ayahs: 111 },
    Surah { number:  18, name: "Al-Kahf",       english: "The Cave",                  ayahs: 110 },
    Surah { number:  19, name: "Maryam",        english: "Mary",                      ayahs:  98 },
    Surah { number:  20, name: "Ta-Ha",         english: "Ta-Ha",                     ayahs: 135 },
    Surah { number:  21, name: "Al-Anbiya",     english: "The Prophets",              ayahs: 112 },
    Surah { number:  22, name: "Al-Hajj",       english: "The Pilgrimage",            ayahs:  78 },
    Surah { number:  23, name: "Al-Muminun",    english: "The Believers",             ayahs: 118 },
    Surah { number:  24, name: "An-Nur",        english: "The Light",                 ayahs:  64 },
    Surah { number:  25, name: "Al-Furqan",     english: "The Criterion",             ayahs:  77 },
    Surah { number:  26, name: "Ash-Shuara",    english: "The Poets",                 ayahs: 227 },
    Surah { number:  27, name: "An-Naml",       english: "The Ant",                   ayahs:  93 },
    Surah { number:  28, name: "Al-Qasas",      english: "The Stories",               ayahs:  88 },
    Surah { number:  29, name: "Al-Ankabut",    english: "The Spider",                ayahs:  69 },
    Surah { number:  30, name: "Ar-Rum",        english: "The Romans",                ayahs:  60 },
    Surah { number:  31, name: "Luqman",        english: "Luqman",                    ayahs:  34 },
    Surah { number:  32, name: "As-Sajda",      english: "The Prostration",           ayahs:  30 },
    Surah { number:  33, name: "Al-Ahzab",      english: "The Confederates",          ayahs:  73 },
    Surah { number:  34, name: "Saba",          english: "Sheba",                     ayahs:  54 },
    Surah { number:  35, name: "Fatir",         english: "The Originator",            ayahs:  45 },
    Surah { number:  36, name: "Ya-Sin",        english: "Ya-Sin",                    ayahs:  83 },
    Surah { number:  37, name: "As-Saffat",     english: "Those Ranged in Ranks",     ayahs: 182 },
    Surah { number:  38, name: "Sad",           english: "Sad",                       ayahs:  88 },
    Surah { number:  39, name: "Az-Zumar",      english: "The Groups",                ayahs:  75 },
    Surah { number:  40, name: "Ghafir",        english: "The Forgiver",              ayahs:  85 },
    Surah { number:  41, name: "Fussilat",      english: "Explained in Detail",       ayahs:  54 },
    Surah { number:  42, name: "Ash-Shura",     english: "The Consultation",          ayahs:  53 },
    Surah { number:  43, name: "Az-Zukhruf",    english: "The Ornaments",             ayahs:  89 },
    Surah { number:  44, name: "Ad-Dukhan",     english: "The Smoke",                 ayahs:  59 },
    Surah { number:  45, name: "Al-Jathiya",    english: "The Crouching",             ayahs:  37 },
    Surah { number:  46, name: "Al-Ahqaf",      english: "The Wind-Curved Sandhills", ayahs:  35 },
    Surah { number:  47, name: "Muhammad",      english: "Muhammad",                  ayahs:  38 },
    Surah { number:  48, name: "Al-Fath",       english: "The Victory",               ayahs:  29 },
    Surah { number:  49, name: "Al-Hujurat",    english: "The Rooms",                 ayahs:  18 },
    Surah { number:  50, name: "Qaf",           english: "Qaf",                       ayahs:  45 },
    Surah { number:  51, name: "Adh-Dhariyat",  english: "The Winds",                 ayahs:  60 },
    Surah { number:  52, name: "At-Tur",        english: "The Mount",                 ayahs:  49 },
    Surah { number:  53, name: "An-Najm",       english: "The Star",                  ayahs:  62 },
    Surah { number:  54, name: "Al-Qamar",      english: "The Moon",                  ayahs:  55 },
    Surah { number:  55, name: "Ar-Rahman",     english: "The Most Merciful",         ayahs:  78 },
    Surah { number:  56, name: "Al-Waqia",      english: "The Event",                 ayahs:  96 },
    Surah { number:  57, name: "Al-Hadid",      english: "The Iron",                  ayahs:  29 },
    Surah { number:  58, name: "Al-Mujadila",   english: "The Pleading Woman",        ayahs:  22 },
    Surah { number:  59, name: "Al-Hashr",      english: "The Exile",                 ayahs:  24 },
    Surah { number:  60, name: "Al-Mumtahana",  english: "The Woman to be Examined",  ayahs:  13 },
    Surah { number:  61, name: "As-Saf",        english: "The Ranks",                 ayahs:  14 },
    Surah { number:  62, name: "Al-Jumua",      english: "Friday",                    ayahs:  11 },
    Surah { number:  63, name: "Al-Munafiqun",  english: "The Hypocrites",            ayahs:  11 },
    Surah { number:  64, name: "At-Taghabun",   english: "The Mutual Disillusion",    ayahs:  18 },
    Surah { number:  65, name: "At-Talaq",      english: "Divorce",                   ayahs:  12 },
    Surah { number:  66, name: "At-Tahrim",     english: "The Prohibition",           ayahs:  12 },
    Surah { number:  67, name: "Al-Mulk",       english: "The Sovereignty",           ayahs:  30 },
    Surah { number:  68, name: "Al-Qalam",      english: "The Pen",                   ayahs:  52 },
    Surah { number:  69, name: "Al-Haqqa",      english: "The Reality",               ayahs:  52 },
    Surah { number:  70, name: "Al-Maarij",     english: "The Ascending Stairways",   ayahs:  44 },
    Surah { number:  71, name: "Nuh",           english: "Noah",                      ayahs:  28 },
    Surah { number:  72, name: "Al-Jinn",       english: "The Jinn",                  ayahs:  28 },
    Surah { number:  73, name: "Al-Muzzammil",  english: "The Wrapped One",           ayahs:  20 },
    Surah { number:  74, name: "Al-Muddaththir",english: "The Cloaked One",           ayahs:  56 },
    Surah { number:  75, name: "Al-Qiyama",     english: "The Resurrection",          ayahs:  40 },
    Surah { number:  76, name: "Al-Insan",      english: "The Human",                 ayahs:  31 },
    Surah { number:  77, name: "Al-Mursalat",   english: "The Emissaries",            ayahs:  50 },
    Surah { number:  78, name: "An-Naba",       english: "The Announcement",          ayahs:  40 },
    Surah { number:  79, name: "An-Naziat",     english: "Those Who Pull Out",        ayahs:  46 },
    Surah { number:  80, name: "Abasa",         english: "He Frowned",                ayahs:  42 },
    Surah { number:  81, name: "At-Takwir",     english: "The Overthrowing",          ayahs:  29 },
    Surah { number:  82, name: "Al-Infitar",    english: "The Cleaving",              ayahs:  19 },
    Surah { number:  83, name: "Al-Mutaffifin", english: "The Defrauding",            ayahs:  36 },
    Surah { number:  84, name: "Al-Inshiqaq",   english: "The Splitting Open",        ayahs:  25 },
    Surah { number:  85, name: "Al-Buruj",      english: "The Constellations",        ayahs:  22 },
    Surah { number:  86, name: "At-Tariq",      english: "The Morning Star",          ayahs:  17 },
    Surah { number:  87, name: "Al-Ala",        english: "The Most High",             ayahs:  19 },
    Surah { number:  88, name: "Al-Ghashiya",   english: "The Overwhelming",          ayahs:  26 },
    Surah { number:  89, name: "Al-Fajr",       english: "The Dawn",                  ayahs:  30 },
    Surah { number:  90, name: "Al-Balad",      english: "The City",                  ayahs:  20 },
    Surah { number:  91, name: "Ash-Shams",     english: "The Sun",                   ayahs:  15 },
    Surah { number:  92, name: "Al-Layl",       english: "The Night",                 ayahs:  21 },
    Surah { number:  93, name: "Ad-Duha",       english: "The Morning Hours",         ayahs:  11 },
    Surah { number:  94, name: "Ash-Sharh",     english: "The Relief",                ayahs:   8 },
    Surah { number:  95, name: "At-Tin",        english: "The Fig",                   ayahs:   8 },
    Surah { number:  96, name: "Al-Alaq",       english: "The Clot",                  ayahs:  19 },
    Surah { number:  97, name: "Al-Qadr",       english: "The Night of Power",        ayahs:   5 },
    Surah { number:  98, name: "Al-Bayyina",    english: "The Clear Proof",           ayahs:   8 },
    Surah { number:  99, name: "Az-Zalzala",    english: "The Earthquake",            ayahs:   8 },
    Surah { number: 100, name: "Al-Adiyat",     english: "The Chargers",              ayahs:  11 },
    Surah { number: 101, name: "Al-Qaria",      english: "The Calamity",              ayahs:  11 },
    Surah { number: 102, name: "At-Takathur",   english: "The Rivalry in Increase",   ayahs:   8 },
    Surah { number: 103, name: "Al-Asr",        english: "The Declining Day",         ayahs:   3 },
    Surah { number: 104, name: "Al-Humaza",     english: "The Traducer",              ayahs:   9 },
    Surah { number: 105, name: "Al-Fil",        english: "The Elephant",              ayahs:   5 },
    Surah { number: 106, name: "Quraysh",       english: "Quraysh",                   ayahs:   4 },
    Surah { number: 107, name: "Al-Maun",       english: "The Small Kindnesses",      ayahs:   7 },
    Surah { number: 108, name: "Al-Kawthar",    english: "Abundance",                 ayahs:   3 },
    Surah { number: 109, name: "Al-Kafirun",    english: "The Unbelievers",           ayahs:   6 },
    Surah { number: 110, name: "An-Nasr",       english: "The Divine Support",        ayahs:   3 },
    Surah { number: 111, name: "Al-Masad",      english: "The Palm Fiber",            ayahs:   5 },
    Surah { number: 112, name: "Al-Ikhlas",     english: "Sincerity",                 ayahs:   4 },
    Surah { number: 113, name: "Al-Falaq",      english: "The Daybreak",              ayahs:   5 },
    Surah { number: 114, name: "An-Nas",        english: "Mankind",                   ayahs:   6 },
];

/// Resolve a surah reference to its entry in `SURAHS`.
///
/// Accepts: a bare number (`"2"`), a name (`"baqara"`, `"Al-Baqara"`),
/// or a name prefix (`"rahman"`).
pub fn resolve_surah(input: &str) -> Option<&'static Surah> {
    let s = input.trim();

    // Try numeric first
    if let Ok(n) = s.parse::<u32>() {
        return SURAHS.iter().find(|su| su.number == n);
    }

    let lower = s.to_lowercase();
    // Strip common prefixes for fuzzy matching
    let stripped = lower
        .trim_start_matches("al-")
        .trim_start_matches("an-")
        .trim_start_matches("at-")
        .trim_start_matches("as-")
        .trim_start_matches("adh-")
        .trim_start_matches("az-")
        .trim_start_matches("ad-")
        .trim_start_matches("ash-");

    // Exact name match (case-insensitive)
    for su in SURAHS {
        if su.name.to_lowercase() == lower {
            return Some(su);
        }
    }

    // Prefix / contains match on name (after stripping articles)
    for su in SURAHS {
        let name_lower = su.name.to_lowercase();
        let name_stripped = name_lower
            .trim_start_matches("al-")
            .trim_start_matches("an-")
            .trim_start_matches("at-")
            .trim_start_matches("as-")
            .trim_start_matches("adh-")
            .trim_start_matches("az-")
            .trim_start_matches("ad-")
            .trim_start_matches("ash-");
        if name_stripped.starts_with(stripped) || name_stripped.contains(stripped) {
            return Some(su);
        }
    }

    None
}
