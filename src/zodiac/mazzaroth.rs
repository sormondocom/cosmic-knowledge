//! Hebrew Mazzaroth — the twelve signs of the sacred zodiac.
//!
//! Sources:
//!  - **Sefer Yetzirah** 5:1–6 — assigns one of the twelve simple letters
//!    (ה ו ז ח ט י ל נ ס ע צ ק) to each Hebrew month and its ruling sense.
//!  - **Bamidbar Rabbah** 2:7 — relates each tribe of Israel to a
//!    constellation and month in the context of the Wilderness camp arrangement.
//!  - **Exodus** 28:17–20 / **Talmud Yerushalmi** *Yoma* 3:7 — the Hoshen
//!    (High Priest's breastplate) twelve gemstones matched to the twelve tribes.
//!  - **Bereshit Rabbah** 10:6 — planetary rulership over the days and signs.
//!  - **Sefer ha-Razim** / **Pirke de-Rabbi Eliezer** ch. 6 — Mazzaroth as the
//!    forty-nine "faces" of celestial intelligence.

use colored::*;
use std::io::{self, Write};

use crate::export::{handle_export, wrap_html};
use crate::reports::chrono_now;

// ─── Data types ───────────────────────────────────────────────────────────────

pub struct MazzarothSign {
    pub number: u8,
    /// Transliterated Hebrew name (e.g. "Taleh").
    pub name: &'static str,
    /// Hebrew script.
    pub hebrew: &'static str,
    /// Common English zodiac name.
    pub english: &'static str,
    /// Unicode astrological symbol.
    pub symbol: &'static str,
    /// Approximate Gregorian date range.
    pub dates: &'static str,
    /// One of: Fire, Earth, Air, Water.
    pub element: &'static str,
    /// Ruling planet — Hebrew name (transliterated) + script + English.
    pub planet: &'static str,
    /// Tribe of Israel (Bamidbar Rabbah 2:7).
    pub tribe: &'static str,
    /// Hebrew month name (transliterated + script).
    pub hebrew_month: &'static str,
    /// Sefer Yetzirah simple letter (transliterated + script).
    pub letter: &'static str,
    /// Hoshen breastplate gemstone (Hebrew name + English translation).
    pub hoshen_stone: &'static str,
    /// Modality and spiritual quality.
    pub quality: &'static str,
}

// ─── The twelve signs ─────────────────────────────────────────────────────────

pub static MAZZAROTH: &[MazzarothSign] = &[
    MazzarothSign {
        number: 1,
        name: "Taleh",
        hebrew: "טָלֶה",
        english: "Aries",
        symbol: "♈",
        dates: "21 March – 19 April",
        element: "Fire",
        planet: "Ma'adim (מַאֲדִים) — Mars",
        tribe: "Judah (יְהוּדָה)",
        hebrew_month: "Nisan (נִיסָן)",
        letter: "Heh (ה)",
        hoshen_stone: "Nofekh (נֹפֶךְ) — Turquoise / Carbuncle",
        quality: "Cardinal · Initiative, leadership, the spark of creation",
    },
    MazzarothSign {
        number: 2,
        name: "Shor",
        hebrew: "שׁוֹר",
        english: "Taurus",
        symbol: "♉",
        dates: "20 April – 20 May",
        element: "Earth",
        planet: "Nogah (נֹגַהּ) — Venus",
        tribe: "Issachar (יִשָּׂשכָר)",
        hebrew_month: "Iyar (אִיָּר)",
        letter: "Vav (ו)",
        hoshen_stone: "Sapir (סַפִּיר) — Sapphire / Lapis Lazuli",
        quality: "Fixed · Steadfastness, earthly abundance, sacred study",
    },
    MazzarothSign {
        number: 3,
        name: "Te'omim",
        hebrew: "תְּאוֹמִים",
        english: "Gemini",
        symbol: "♊",
        dates: "21 May – 20 June",
        element: "Air",
        planet: "Kokhav (כּוֹכָב) — Mercury",
        tribe: "Zebulun (זְבוּלֻן)",
        hebrew_month: "Sivan (סִיוָן)",
        letter: "Zayin (ז)",
        hoshen_stone: "Yahalom (יַהֲלֹם) — Diamond",
        quality: "Mutable · Duality, commerce, the gift of language and trade",
    },
    MazzarothSign {
        number: 4,
        name: "Sartan",
        hebrew: "סַרְטָן",
        english: "Cancer",
        symbol: "♋",
        dates: "21 June – 22 July",
        element: "Water",
        planet: "Levanah (לְבָנָה) — Moon",
        tribe: "Reuben (רְאוּבֵן)",
        hebrew_month: "Tammuz (תַּמּוּז)",
        letter: "Chet (ח)",
        hoshen_stone: "Odem (אֹדֶם) — Ruby / Carnelian",
        quality: "Cardinal · Memory, ancestry, the hidden waters of emotion",
    },
    MazzarothSign {
        number: 5,
        name: "Aryeh",
        hebrew: "אַרְיֵה",
        english: "Leo",
        symbol: "♌",
        dates: "23 July – 22 August",
        element: "Fire",
        planet: "Shemesh (שֶׁמֶשׁ) — Sun",
        tribe: "Simeon (שִׁמְעוֹן)",
        hebrew_month: "Av (אָב)",
        letter: "Tet (ט)",
        hoshen_stone: "Pitdah (פִּטְדָה) — Topaz / Peridot",
        quality: "Fixed · Royalty, divine radiance, the courage of the heart",
    },
    MazzarothSign {
        number: 6,
        name: "Betulah",
        hebrew: "בְּתוּלָה",
        english: "Virgo",
        symbol: "♍",
        dates: "23 August – 22 September",
        element: "Earth",
        planet: "Kokhav (כּוֹכָב) — Mercury",
        tribe: "Gad (גָּד)",
        hebrew_month: "Elul (אֱלוּל)",
        letter: "Yod (י)",
        hoshen_stone: "Shevo (שְׁבוֹ) — Agate / Banded Quartz",
        quality: "Mutable · Discernment, teshuvah (return), the harvest of refinement",
    },
    MazzarothSign {
        number: 7,
        name: "Moznayim",
        hebrew: "מֹאזְנַיִם",
        english: "Libra",
        symbol: "♎",
        dates: "23 September – 22 October",
        element: "Air",
        planet: "Nogah (נֹגַהּ) — Venus",
        tribe: "Ephraim (אֶפְרַיִם)",
        hebrew_month: "Tishrei (תִּשְׁרֵי)",
        letter: "Lamed (ל)",
        hoshen_stone: "Leshem (לֶשֶׁם) — Ligure / Jacinth",
        quality: "Cardinal · Justice, divine balance, the scales of Rosh Hashanah",
    },
    MazzarothSign {
        number: 8,
        name: "Akrav",
        hebrew: "עַקְרָב",
        english: "Scorpio",
        symbol: "♏",
        dates: "23 October – 21 November",
        element: "Water",
        planet: "Ma'adim (מַאֲדִים) — Mars",
        tribe: "Manasseh (מְנַשֶּׁה)",
        hebrew_month: "Cheshvan (חֶשְׁוָן)",
        letter: "Nun (נ)",
        hoshen_stone: "Shoham (שֹׁהַם) — Onyx / Beryl",
        quality: "Fixed · Transformation, hidden depths, death and renewal",
    },
    MazzarothSign {
        number: 9,
        name: "Keshet",
        hebrew: "קֶשֶׁת",
        english: "Sagittarius",
        symbol: "♐",
        dates: "22 November – 21 December",
        element: "Fire",
        planet: "Tzedek (צֶדֶק) — Jupiter",
        tribe: "Benjamin (בִּנְיָמִין)",
        hebrew_month: "Kislev (כִּסְלֵו)",
        letter: "Samech (ס)",
        hoshen_stone: "Yashfeh (יָשְׁפֵה) — Jasper",
        quality: "Mutable · Vision, the bow of prophecy, Chanukah light in darkness",
    },
    MazzarothSign {
        number: 10,
        name: "Gedi",
        hebrew: "גְּדִי",
        english: "Capricorn",
        symbol: "♑",
        dates: "22 December – 19 January",
        element: "Earth",
        planet: "Shabbatai (שַׁבְּתַאי) — Saturn",
        tribe: "Dan (דָּן)",
        hebrew_month: "Tevet (טֵבֵת)",
        letter: "Ayin (ע)",
        hoshen_stone: "Achlamah (אַחְלָמָה) — Amethyst",
        quality: "Cardinal · Discipline, the mountain of ascent, winter solstice",
    },
    MazzarothSign {
        number: 11,
        name: "Deli",
        hebrew: "דְּלִי",
        english: "Aquarius",
        symbol: "♒",
        dates: "20 January – 18 February",
        element: "Air",
        planet: "Shabbatai (שַׁבְּתַאי) — Saturn",
        tribe: "Asher (אָשֵׁר)",
        hebrew_month: "Shevat (שְׁבָט)",
        letter: "Tzade (צ)",
        hoshen_stone: "Tarshish (תַּרְשִׁישׁ) — Beryl / Chrysolite",
        quality: "Fixed · Universal vision, the water-bearer of living Torah",
    },
    MazzarothSign {
        number: 12,
        name: "Dagim",
        hebrew: "דָּגִים",
        english: "Pisces",
        symbol: "♓",
        dates: "19 February – 20 March",
        element: "Water",
        planet: "Tzedek (צֶדֶק) — Jupiter",
        tribe: "Naphtali (נַפְתָּלִי)",
        hebrew_month: "Adar (אֲדָר)",
        letter: "Kuf (ק)",
        hoshen_stone: "Bareket (בָּרֶקֶת) — Emerald",
        quality: "Mutable · Compassion, hidden faith, the sea of Adar joy",
    },
];

// ─── Date lookup ──────────────────────────────────────────────────────────────

/// Return the sign for a Gregorian calendar date (1-indexed month, day).
/// Returns `None` only if month/day values are out of range.
pub fn sign_for_date(month: u8, day: u8) -> Option<&'static MazzarothSign> {
    let idx = match (month, day) {
        (3, 21..=31) | (4, 1..=19) => 0,   // Taleh   / Aries
        (4, 20..=30) | (5, 1..=20) => 1,   // Shor    / Taurus
        (5, 21..=31) | (6, 1..=20) => 2,   // Te'omim / Gemini
        (6, 21..=30) | (7, 1..=22) => 3,   // Sartan  / Cancer
        (7, 23..=31) | (8, 1..=22) => 4,   // Aryeh   / Leo
        (8, 23..=31) | (9, 1..=22) => 5,   // Betulah / Virgo
        (9, 23..=30) | (10, 1..=22) => 6,  // Moznayim/ Libra
        (10, 23..=31) | (11, 1..=21) => 7, // Akrav   / Scorpio
        (11, 22..=30) | (12, 1..=21) => 8, // Keshet  / Sagittarius
        (12, 22..=31) | (1, 1..=19) => 9,  // Gedi    / Capricorn
        (1, 20..=31) | (2, 1..=18) => 10,  // Deli    / Aquarius
        (2, 19..=29) | (3, 1..=20) => 11,  // Dagim   / Pisces
        _ => return None,
    };
    MAZZAROTH.get(idx)
}

// ─── Display helpers ──────────────────────────────────────────────────────────

fn print_sign_card(s: &MazzarothSign) {
    let sep = "  ══════════════════════════════════════════════════════════";
    println!();
    println!("{}", sep.bright_yellow());
    println!(
        "  {} {} {}  {}  —  {}",
        format!("{:>2}.", s.number).bright_yellow(),
        s.symbol.bright_white(),
        s.name.bold().bright_yellow(),
        s.hebrew.bright_cyan(),
        s.english.bright_white(),
    );
    println!("{}", sep.dimmed());
    println!("  {:<22} {}", "Dates:".bold(), s.dates.bright_white());
    println!("  {:<22} {}", "Element:".bold(), s.element.bright_white());
    println!(
        "  {:<22} {}",
        "Ruling Planet:".bold(),
        s.planet.bright_white()
    );
    println!(
        "  {:<22} {}",
        "Tribe of Israel:".bold(),
        s.tribe.bright_white()
    );
    println!(
        "  {:<22} {}",
        "Hebrew Month:".bold(),
        s.hebrew_month.bright_white()
    );
    println!(
        "  {:<22} {}",
        "Sefer Yetzirah Letter:".bold(),
        s.letter.bright_white()
    );
    println!(
        "  {:<22} {}",
        "Hoshen Gemstone:".bold(),
        s.hoshen_stone.bright_white()
    );
    println!("  {:<22} {}", "Quality:".bold(), s.quality.dimmed());
    println!("{}", sep.dimmed());
}

fn print_summary_table() {
    println!();
    println!(
        "{}",
        "  ╔═══╦══╦════════════╦══════════════╦════════╦══════════════╗".bright_yellow()
    );
    println!(
        "{}",
        "  ║ # ║  ║  Hebrew     ║  English      ║ Elem   ║  Dates        ║".bright_yellow()
    );
    println!(
        "{}",
        "  ╠═══╬══╬════════════╬══════════════╬════════╬══════════════╣".bright_yellow()
    );
    for s in MAZZAROTH {
        println!(
            "  ║ {:<2}║{}║ {:<12}║ {:<14}║ {:<7}║ {:<14}║",
            s.number.to_string().bright_cyan(),
            s.symbol.bright_white(),
            s.name.bright_yellow(),
            s.english.bright_white(),
            s.element.dimmed(),
            s.dates.dimmed(),
        );
    }
    println!(
        "{}",
        "  ╚═══╩══╩════════════╩══════════════╩════════╩══════════════╝".bright_yellow()
    );
    println!();
    handle_export(
        "mazzaroth_twelve_signs",
        || {
            let mut s = format!("HEBREW MAZZAROTH — TWELVE SIGNS\nGenerated: {}\n\n", chrono_now());
            s.push_str(&format!("{:<3} {:<3} {:<14} {:<16} {:<10} {:<16} {:<12} {}\n",
                "#", "", "Hebrew", "English", "Element", "Dates", "Tribe", "Hoshen Stone"));
            s.push_str(&"-".repeat(90));
            s.push('\n');
            for sg in MAZZAROTH {
                s.push_str(&format!("{:<3} {:<3} {:<14} {:<16} {:<10} {:<16} {:<12} {}\n",
                    sg.number, sg.symbol, sg.name, sg.english,
                    sg.element, sg.dates, sg.tribe, sg.hoshen_stone));
            }
            s
        },
        || {
            let mut rows = String::new();
            for sg in MAZZAROTH {
                rows.push_str(&format!(
                    "<tr><td class=\"num\">{}</td><td style=\"font-size:18pt;\">{}</td>\
                     <td class=\"sys\">{}</td><td>{}</td><td>{}</td><td>{}</td>\
                     <td>{}</td><td class=\"meaning\">{}</td></tr>",
                    sg.number, maz_esc(sg.symbol), maz_esc(sg.name),
                    maz_esc(sg.english), maz_esc(sg.element), maz_esc(sg.dates),
                    maz_esc(sg.tribe), maz_esc(sg.hoshen_stone)
                ));
            }
            let body = format!(
                "<table><thead><tr><th>#</th><th>Symbol</th><th>Hebrew</th>\
                 <th>English</th><th>Element</th><th>Dates</th>\
                 <th>Tribe</th><th>Hoshen Stone</th></tr></thead><tbody>{}</tbody></table>",
                rows
            );
            wrap_html("Hebrew Mazzaroth — Twelve Signs", &body, "hebrew")
        },
    );
}

fn maz_esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

// ─── Session ──────────────────────────────────────────────────────────────────

pub fn run_mazzaroth_session() {
    println!();
    println!(
        "{}",
        "╔════════════════════════════════════════════════════════════╗".bright_yellow()
    );
    println!(
        "{}",
        "║          ✡  HEBREW MAZZAROTH — מַזָּרוֹת  ✡              ║"
            .bold()
            .bright_yellow()
    );
    println!(
        "{}",
        "╠════════════════════════════════════════════════════════════╣".bright_yellow()
    );
    println!(
        "{}",
        "║  The twelve constellations of the sacred zodiac as seen   ║".dimmed()
    );
    println!(
        "{}",
        "║  through Torah, Sefer Yetzirah, and the Twelve Tribes.    ║".dimmed()
    );
    println!(
        "{}",
        "╚════════════════════════════════════════════════════════════╝".bright_yellow()
    );

    loop {
        println!();
        println!(
            "  {}",
            "─ Choose an action ───────────────────────────────────────".dimmed()
        );
        println!("  {}  Browse all twelve signs", "1.".cyan());
        println!("  {}  Look up a sign by number (1–12)", "2.".cyan());
        println!("  {}  Find my sign by birth date", "3.".cyan());
        println!("  {}  Return to Zodiac menu", "0.".dimmed());
        println!();
        print!("  {} ", "▸ Choice:".bold().cyan());
        io::stdout().flush().unwrap_or(());

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap_or(0);

        match buf.trim() {
            "1" => {
                println!();
                println!(
                    "{}",
                    "  ── All Twelve Signs of the Mazzaroth ──"
                        .bold()
                        .bright_yellow()
                );
                print_summary_table();
                println!();
                print!("  {} ", "Press Enter to continue…".dimmed());
                io::stdout().flush().unwrap_or(());
                let mut _p = String::new();
                io::stdin().read_line(&mut _p).unwrap_or(0);
            }
            "2" => {
                print!("  {} ", "Enter sign number (1–12):".bold().cyan());
                io::stdout().flush().unwrap_or(());
                let mut nbuf = String::new();
                io::stdin().read_line(&mut nbuf).unwrap_or(0);
                match nbuf.trim().parse::<u8>() {
                    Ok(n) if (1..=12).contains(&n) => {
                        print_sign_card(&MAZZAROTH[(n - 1) as usize]);
                    }
                    _ => println!("{}", "  Please enter a number from 1 to 12.".yellow()),
                }
            }
            "3" => {
                print!("  {} ", "Enter birth month (1–12):".bold().cyan());
                io::stdout().flush().unwrap_or(());
                let mut mbuf = String::new();
                io::stdin().read_line(&mut mbuf).unwrap_or(0);
                let month = mbuf.trim().parse::<u8>().unwrap_or(0);

                print!("  {} ", "Enter birth day (1–31):".bold().cyan());
                io::stdout().flush().unwrap_or(());
                let mut dbuf = String::new();
                io::stdin().read_line(&mut dbuf).unwrap_or(0);
                let day = dbuf.trim().parse::<u8>().unwrap_or(0);

                match sign_for_date(month, day) {
                    Some(s) => print_sign_card(s),
                    None => println!(
                        "{}",
                        "  Invalid date — please enter a valid month (1–12) and day.".yellow()
                    ),
                }
            }
            "0" | "" => break,
            _ => println!("{}", "  Please enter 0–3.".yellow()),
        }
    }
}
