//! Chinese cosmological systems.
//!
//! **Chinese:**
//!  - Nine Star Ki (九星気学): birth-year → natal star 1–9 with element and trigram
//!  - Wu Xing (五行): numbers 1–9 mapped to the Five Elements
//!  - Ba Gua (八卦): the eight trigrams with Lo Shu positions
//!  - Lucky/unlucky numbers: phonetic homophony in Mandarin and Cantonese
//!
//! Sources:
//!  - Kushi, M. *Nine Star Ki* (1991, One Peaceful World Press)
//!  - Yoshikawa, T. *The Ki* (1986, St. Martin's Press)
//!  - Wilhelm, R. (trans.) *I Ching* (1950, Princeton / Bollingen)
//!  - Needham, J. *Science and Civilisation in China*, Vol. 2 (1956, Cambridge)
//!  - Eberhard, W. *A Dictionary of Chinese Symbols* (1983, Routledge)
//!  - Bruun, O. *Fengshui in China* (2003, NIAS Press)

use std::io::{self, Write};
use colored::*;

// ─── Chinese: Nine Star Ki ─────────────────────────────────────────────────────

/// Complete information for a Nine Star Ki natal star.
pub struct NineStarInfo {
    pub star:      u32,
    pub name:      &'static str,
    pub element:   &'static str,
    pub direction: &'static str,
    pub trigram:   &'static str,
    pub symbol:    &'static str,
    pub qualities: &'static str,
}

/// Compute the Nine Star Ki natal star from a birth year.
///
/// **Risshun adjustment**: births before approximately February 4 of any year
/// should pass `year - 1` — the Chinese solar year begins at Risshun (立春).
///
/// Anchor: 1991 = star 9.  The cycle descends 9→8→…→1→9 one step per year.
///
/// Source: Yoshikawa (1986), Kushi (1991), Sachs (1992).
pub fn nine_star_ki_natal(year: i32) -> u32 {
    let delta = (year - 1991).rem_euclid(9) as u32;
    if delta == 0 { 9 } else { 9 - delta }
}

/// Full Nine Star Ki profile for a natal star 1–9.
pub fn nine_star_info(star: u32) -> NineStarInfo {
    match star {
        1 => NineStarInfo {
            star: 1, name: "One Water",
            element: "Water", direction: "North",
            trigram: "Kan (坎)", symbol: "☵",
            qualities: "Depth, adaptability, wisdom, potential, the creative abyss; \
                        independent and introspective; diplomatic under pressure",
        },
        2 => NineStarInfo {
            star: 2, name: "Two Soil",
            element: "Earth", direction: "Southwest",
            trigram: "Kun (坤)", symbol: "☷",
            qualities: "Nurturing, diligence, receptivity, the devoted mother; \
                        reliable and methodical; builds strong foundations",
        },
        3 => NineStarInfo {
            star: 3, name: "Three Tree",
            element: "Wood (Thunder)", direction: "East",
            trigram: "Zhen (震)", symbol: "☳",
            qualities: "Initiative, enthusiasm, direct action, the eldest son; \
                        pioneering energy; honest to the point of bluntness",
        },
        4 => NineStarInfo {
            star: 4, name: "Four Tree",
            element: "Wood (Wind)", direction: "Southeast",
            trigram: "Xun (巽)", symbol: "☴",
            qualities: "Gentle penetration, communication, creativity, the eldest daughter; \
                        persuasive and artistic; attentive to detail",
        },
        5 => NineStarInfo {
            star: 5, name: "Five Soil",
            element: "Earth (Center)", direction: "Center",
            trigram: "(No trigram — the axis)", symbol: "⊕",
            qualities: "Power, leadership, the axis of all nine stars; intensifies \
                        surrounding influences; strongest and most extreme natal star",
        },
        6 => NineStarInfo {
            star: 6, name: "Six Metal",
            element: "Metal (Heaven)", direction: "Northwest",
            trigram: "Qian (乾)", symbol: "☰",
            qualities: "Authority, discipline, the creative father, heaven; \
                        natural leader; high standards; can be rigid but just",
        },
        7 => NineStarInfo {
            star: 7, name: "Seven Metal",
            element: "Metal (Lake)", direction: "West",
            trigram: "Dui (兌)", symbol: "☱",
            qualities: "Joy, socializing, eloquence, the youngest daughter; \
                        magnetic and charming; appreciates beauty and pleasure",
        },
        8 => NineStarInfo {
            star: 8, name: "Eight Soil",
            element: "Earth (Mountain)", direction: "Northeast",
            trigram: "Gen (艮)", symbol: "☶",
            qualities: "Stillness, patience, perseverance, the youngest son; \
                        reliable and accumulating; pauses before leaping",
        },
        9 => NineStarInfo {
            star: 9, name: "Nine Fire",
            element: "Fire", direction: "South",
            trigram: "Li (離)", symbol: "☲",
            qualities: "Brilliance, clarity, recognition, the middle daughter; \
                        expressive and passionate; illuminates everything around it",
        },
        _ => NineStarInfo {
            star: 0, name: "Unknown", element: "—", direction: "—",
            trigram: "—", symbol: "?", qualities: "Star out of range (1–9 only)",
        },
    }
}

// ─── Chinese: Wu Xing (Five Elements) ─────────────────────────────────────────

/// The Five Elements in their Lo Shu / Nine Star Ki assignment.
pub struct WuXingInfo {
    pub element_zh:  &'static str,   // Chinese name (Pinyin)
    pub element_en:  &'static str,   // English
    pub symbol:      &'static str,
    pub direction:   &'static str,
    pub season:      &'static str,
    pub qualities:   &'static str,
}

/// Map a Lo Shu position (1–9) to its Wu Xing element.
pub fn wu_xing(number: u32) -> Option<WuXingInfo> {
    match number {
        1 => Some(WuXingInfo {
            element_zh: "Shui (水)", element_en: "Water", symbol: "💧",
            direction: "North", season: "Winter",
            qualities: "Flow, wisdom, adaptability, introspection; governs the \
                        kidneys and bones; associated with the colour black/dark blue",
        }),
        2 | 5 | 8 => Some(WuXingInfo {
            element_zh: "Tu (土)", element_en: "Earth", symbol: "🌏",
            direction: "Center / SW / NE", season: "Late summer / transitions",
            qualities: "Stability, nourishment, practicality; governs spleen and \
                        digestion; associated with yellow; the pivot among elements",
        }),
        3 => Some(WuXingInfo {
            element_zh: "Mu (木)", element_en: "Wood (Thunder)", symbol: "🌳",
            direction: "East", season: "Spring",
            qualities: "Growth, initiative, flexibility; governs the liver and eyes; \
                        associated with green; expansive upward energy",
        }),
        4 => Some(WuXingInfo {
            element_zh: "Mu (木)", element_en: "Wood (Wind)", symbol: "🌳",
            direction: "Southeast", season: "Spring",
            qualities: "Growth, initiative, flexibility; governs the liver and eyes; \
                        associated with green; expansive upward energy",
        }),
        6 | 7 => Some(WuXingInfo {
            element_zh: "Jin (金)", element_en: "Metal", symbol: "⚙️",
            direction: "Northwest / West", season: "Autumn",
            qualities: "Clarity, precision, letting go; governs the lungs and skin; \
                        associated with white/gold; refining and structuring energy",
        }),
        9 => Some(WuXingInfo {
            element_zh: "Huo (火)", element_en: "Fire", symbol: "🔥",
            direction: "South", season: "Summer",
            qualities: "Illumination, passion, communication; governs the heart; \
                        associated with red; the most expansive and visible energy",
        }),
        _ => None,
    }
}

// ─── Chinese: Lucky/Unlucky numbers ───────────────────────────────────────────

pub struct ChineseLuckyInfo {
    pub number:      u32,
    pub mandarin:    &'static str,   // pronunciation
    pub cantonese:   &'static str,   // pronunciation
    pub luck:        &'static str,   // "Very Lucky" / "Lucky" / "Mixed" / "Unlucky" / "Very Unlucky"
    pub meaning:     &'static str,
}

/// Look up Chinese lucky/unlucky meaning for a number.
///
/// Covers 1–9 and key compound numbers documented in Eberhard (1983), Bruun (2003).
pub fn chinese_lucky_meaning(n: u32) -> Option<ChineseLuckyInfo> {
    match n {
        1 => Some(ChineseLuckyInfo { number: 1,
            mandarin: "yī (一)", cantonese: "yat (一)",
            luck: "Neutral / Good",
            meaning: "Unity, beginning, certainty (一定 yídìng — 'certainly'); stands alone but \
                      points toward wholeness; the Tao expressed as number",
        }),
        2 => Some(ChineseLuckyInfo { number: 2,
            mandarin: "èr (二)", cantonese: "yi (二)",
            luck: "Lucky",
            meaning: "Good things come in pairs (好事成双); Cantonese *yi* ~ 'easy'; \
                      doubles of lucky numbers amplify their power",
        }),
        3 => Some(ChineseLuckyInfo { number: 3,
            mandarin: "sān (三)", cantonese: "saam (三)",
            luck: "Mixed",
            meaning: "Mandarin: sounds like 'alive/grow' (生 shēng) — positive. \
                      Cantonese: *saam* sounds like 'scatter' (散 saan) — slightly unlucky. \
                      Associated with the Three Purities in Taoism",
        }),
        4 => Some(ChineseLuckyInfo { number: 4,
            mandarin: "sì (四)", cantonese: "sei (四)",
            luck: "Very Unlucky",
            meaning: "Sounds almost identically to 'death' (死 sǐ / *sei²*) in both Mandarin \
                      and Cantonese. Avoided in addresses, hospital floors, phone numbers, \
                      gift amounts. Tetraphobia is widespread across East Asia",
        }),
        5 => Some(ChineseLuckyInfo { number: 5,
            mandarin: "wǔ (五)", cantonese: "ng (五)",
            luck: "Mixed",
            meaning: "Mandarin: sounds like 'nothing/without' (无 wú) — ambiguous. Also the \
                      Five Elements (五行), which is auspicious. Cantonese *ǹg* lacks strong \
                      homophony either way",
        }),
        6 => Some(ChineseLuckyInfo { number: 6,
            mandarin: "liù (六)", cantonese: "luk (六)",
            luck: "Very Lucky",
            meaning: "Sounds like 'smooth flow / prosperity' (流/禄 liú/lù). Cantonese *luk⁶* \
                      ~ 'road/path' (路 lou⁶). Represents smooth progress in all endeavours. \
                      Triple 6 (666) is extremely lucky in Chinese culture",
        }),
        7 => Some(ChineseLuckyInfo { number: 7,
            mandarin: "qī (七)", cantonese: "cat (七)",
            luck: "Mixed",
            meaning: "Mandarin: sounds like 'arise/stand up' (起 qǐ) — positive. Also the \
                      7th lunar month (Ghost Month) — negative. Cantonese *cat¹* ~ 'surely' \
                      (實 sat⁶) — positive context-dependent",
        }),
        8 => Some(ChineseLuckyInfo { number: 8,
            mandarin: "bā (八)", cantonese: "baat (八)",
            luck: "Extremely Lucky",
            meaning: "Sounds like 'prosper/wealth' (發 fā/faat³). The most prized number in \
                      Chinese culture. Beijing Olympics opened 2008-08-08 at 08:08:08 PM. \
                      Premium prices paid for addresses, phone numbers, and license plates \
                      containing 8. Triple 8 (888) = ultimate prosperity",
        }),
        9 => Some(ChineseLuckyInfo { number: 9,
            mandarin: "jiǔ (九)", cantonese: "gau (九)",
            luck: "Lucky",
            meaning: "Sounds identically to 'long-lasting/eternal' (久 jiǔ). The Emperor's \
                      number: 9 dragons, 9999 rooms in the Forbidden City, 9-tiered pagodas. \
                      Double 9 (99) = 'everlasting'; used on anniversaries and weddings",
        }),
        18 => Some(ChineseLuckyInfo { number: 18,
            mandarin: "shí-bā (十八)", cantonese: "sap-baat",
            luck: "Lucky",
            meaning: "Sounds like 'definitely prosper' (实发 shí fā). A fortunate number \
                      combining the reliable '1' with the prosperous '8'",
        }),
        28 => Some(ChineseLuckyInfo { number: 28,
            mandarin: "èr-bā (二八)", cantonese: "yi-baat",
            luck: "Lucky",
            meaning: "'Easy to prosper' (易發 yì fā). Cantonese: *yi baat* ~ 'easy to get rich'",
        }),
        68 => Some(ChineseLuckyInfo { number: 68,
            mandarin: "liù-bā (六八)", cantonese: "luk-baat",
            luck: "Very Lucky",
            meaning: "'Road to prosperity' — smooth (6) wealth (8). Highly desirable in \
                      business names and phone numbers",
        }),
        88 => Some(ChineseLuckyInfo { number: 88,
            mandarin: "bā-bā (八八)", cantonese: "baat-baat",
            luck: "Extremely Lucky",
            meaning: "Double prosperity. Visually resembles 囍 (double happiness). Also \
                      sounds like *bàba* (爸爸, father) — filial piety. The most auspicious \
                      two-digit number",
        }),
        99 => Some(ChineseLuckyInfo { number: 99,
            mandarin: "jiǔ-jiǔ (九九)", cantonese: "gau-gau",
            luck: "Very Lucky",
            meaning: "'Eternal/everlasting' (久久 jiǔjiǔ). Used on wedding gifts and \
                      anniversary dates. Represents the Emperor's doubled eternal reign",
        }),
        168 => Some(ChineseLuckyInfo { number: 168,
            mandarin: "yī-liù-bā (一六八)", cantonese: "yat-luk-baat",
            luck: "Extremely Lucky",
            meaning: "'All the way to prosperity' (一路發 yī lù fā). One of the most \
                      auspicious phone-number sequences in Chinese business culture",
        }),
        518 => Some(ChineseLuckyInfo { number: 518,
            mandarin: "wǔ-yī-bā (五一八)", cantonese: "ng-yat-baat",
            luck: "Lucky",
            meaning: "'I want to prosper' (我要發 wǒ yào fā / 我要發 ǹg yiu faat). \
                      Popular in business contexts",
        }),
        666 => Some(ChineseLuckyInfo { number: 666,
            mandarin: "liù-liù-liù (六六六)", cantonese: "luk-luk-luk",
            luck: "Very Lucky",
            meaning: "Triple smooth progress. Premium phone numbers and car plates. \
                      The opposite of Western connotations — in Chinese culture, \
                      666 = maximum smoothness and flow in all endeavours",
        }),
        888 => Some(ChineseLuckyInfo { number: 888,
            mandarin: "bā-bā-bā (八八八)", cantonese: "baat-baat-baat",
            luck: "Extremely Lucky",
            meaning: "Triple prosperity. The ultimate lucky number sequence. \
                      Associated with the highest prices for phone numbers, \
                      property, and luxury goods",
        }),
        _ => None,
    }
}

// ─── Display / session functions ───────────────────────────────────────────────

/// Interactive Chinese cosmology session (Nine Star Ki, Wu Xing, Lucky Numbers).
pub fn run_chinese_session() {
    loop {
        println!();
        println!("{}", "╔══════════════════════════════════════════════════════════╗".bright_yellow());
        println!("{}", "║          ☯  CHINESE COSMOLOGICAL SYSTEMS                 ║".bold().bright_yellow());
        println!("{}", "╠══════════════════════════════════════════════════════════╣".bright_yellow());
        println!("{}", "║   1.  🌟  Nine Star Ki — Natal Star from Birth Year      ║".bright_white());
        println!("{}", "║          Ba Gua trigram · Wu Xing element · qualities    ║".dimmed());
        println!("{}", "║                                                          ║".bright_yellow());
        println!("{}", "║   2.  🍀  Lucky / Unlucky Number Lookup                  ║".bright_white());
        println!("{}", "║          Mandarin & Cantonese homophony meanings         ║".dimmed());
        println!("{}", "║                                                          ║".bright_yellow());
        println!("{}", "║   3.  🌊  Wu Xing — Five Elements for a Lo Shu Number    ║".bright_white());
        println!("{}", "║          Element, direction, season, qualities           ║".dimmed());
        println!("{}", "║                                                          ║".bright_yellow());
        println!("{}", "║   0.  ←  Back                                            ║".dimmed());
        println!("{}", "╚══════════════════════════════════════════════════════════╝".bright_yellow());
        println!();

        print!("{}", "▸ Enter choice: ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { break; }

        match input.trim() {
            "1" => chinese_nine_star_session(),
            "2" => chinese_lucky_session(),
            "3" => chinese_wu_xing_session(),
            "0" | "" => break,
            _ => println!("{}", "  Please enter 0–3.".yellow()),
        }
    }
}

fn chinese_nine_star_session() {
    println!();
    print!("{}", "  ▸ Enter birth year (e.g. 1987): ".cyan());
    io::stdout().flush().unwrap_or(());
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_err() { return; }

    let Ok(year) = buf.trim().parse::<i32>() else {
        println!("{}", "  ⚠️  Please enter a 4-digit year.".yellow());
        return;
    };

    print!("{}", "  ▸ Born before February 4? (y/n): ".cyan());
    io::stdout().flush().unwrap_or(());
    let mut early_buf = String::new();
    io::stdin().read_line(&mut early_buf).ok();
    let adjusted_year = if early_buf.trim().eq_ignore_ascii_case("y") { year - 1 } else { year };

    let star = nine_star_ki_natal(adjusted_year);
    let info = nine_star_info(star);
    let wx   = wu_xing(star).expect("star 1–9 always maps to an element");

    println!();
    println!("{}", "  ╔═══════════════════════════════════════════════════════╗".bright_yellow());
    println!("  {}", format!("║  🌟  Nine Star Ki for birth year {}{}             ║",
        year, if adjusted_year != year { " (adj.)" } else { "         " }).bold().bright_yellow());
    println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_yellow());
    println!("  {}", format!("║  Natal Star  : {}  —  {}                    ║", star, info.name)
        .bold().bright_white());
    println!("  {}", format!("║  Trigram     : {} {}                              ║", info.symbol, info.trigram)
        .bright_cyan());
    println!("  {}", format!("║  Element     : {} {}                         ║", wx.symbol, info.element)
        .bright_green());
    println!("  {}", format!("║  Direction   : {}                                  ║", info.direction)
        .bright_white());
    println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_yellow());

    // Word-wrap qualities
    let words: Vec<&str> = info.qualities.split_whitespace().collect();
    let mut line = String::new();
    for word in &words {
        if line.len() + word.len() + 1 > 51 {
            println!("  {}", format!("║  {:<53}║", line).bright_white());
            line = word.to_string();
        } else {
            if !line.is_empty() { line.push(' '); }
            line.push_str(word);
        }
    }
    if !line.is_empty() {
        println!("  {}", format!("║  {:<53}║", line).bright_white());
    }

    println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_yellow());
    println!("  {}", format!("║  Wu Xing: {}   Direction: {}                    ║",
        wx.element_en, wx.direction).italic().dimmed());
    println!("{}", "  ╚═══════════════════════════════════════════════════════╝".bright_yellow());
    println!();
}

fn chinese_lucky_session() {
    println!();
    println!("{}", "  Enter a number to check its Chinese lucky/unlucky meaning.".dimmed());
    println!("{}", "  (1–9 and compound numbers 18, 28, 68, 88, 99, 168, 518, 666, 888)".dimmed());
    println!("{}", "  (empty line → back)".dimmed());

    loop {
        print!("{}", "\n  ▸ Enter number: ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() { break; }
        let trimmed = buf.trim();
        if trimmed.is_empty() { break; }

        let Ok(n) = trimmed.parse::<u32>() else {
            println!("{}", "  ⚠️  Please enter a positive integer.".yellow());
            continue;
        };

        match chinese_lucky_meaning(n) {
            None => println!("{}", format!("  ℹ️  {} has no specific documented homophony meaning in this database.", n).dimmed()),
            Some(info) => {
                let luck_color = match info.luck {
                    "Extremely Lucky" | "Very Lucky" => info.luck.bright_green().bold().to_string(),
                    "Lucky"           => info.luck.green().to_string(),
                    "Mixed"           => info.luck.yellow().to_string(),
                    _                 => info.luck.bright_red().to_string(),
                };
                println!();
                println!("  {}", format!("Number {} — {}", n, luck_color).bold());
                println!("  {}", format!("Mandarin: {}  |  Cantonese: {}", info.mandarin, info.cantonese).dimmed());
                println!("  {}", info.meaning.bright_white());
            }
        }
    }
}

fn chinese_wu_xing_session() {
    println!();
    println!("{}", "  Enter a Lo Shu position (1–9) to see its Wu Xing element.".dimmed());
    println!("{}", "  (empty line → back)".dimmed());

    loop {
        print!("{}", "\n  ▸ Enter Lo Shu number (1–9): ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() { break; }
        let trimmed = buf.trim();
        if trimmed.is_empty() { break; }

        let Ok(n) = trimmed.parse::<u32>() else {
            println!("{}", "  ⚠️  Enter a number 1–9.".yellow());
            continue;
        };

        match wu_xing(n) {
            None => println!("{}", format!("  ⚠️  {} is out of range (1–9).", n).yellow()),
            Some(wx) => {
                println!();
                println!("  {} {} — {}", wx.symbol, wx.element_zh.bold(), wx.element_en.bright_green().bold());
                println!("  Direction: {}  |  Season: {}", wx.direction.cyan(), wx.season.cyan());
                println!("  {}", wx.qualities.bright_white());
            }
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Nine Star Ki ──────────────────────────────────────────────────────────

    #[test]
    fn nine_star_ki_anchor_1991_is_9() {
        assert_eq!(nine_star_ki_natal(1991), 9);
    }

    #[test]
    fn nine_star_ki_descends_by_one() {
        assert_eq!(nine_star_ki_natal(1992), 8);
        assert_eq!(nine_star_ki_natal(1993), 7);
        assert_eq!(nine_star_ki_natal(1999), 1);
    }

    #[test]
    fn nine_star_ki_wraps_correctly() {
        assert_eq!(nine_star_ki_natal(2000), 9); // new cycle
        assert_eq!(nine_star_ki_natal(2001), 8);
    }

    #[test]
    fn nine_star_ki_before_anchor() {
        assert_eq!(nine_star_ki_natal(1990), 1);
        assert_eq!(nine_star_ki_natal(1982), 9); // 9 years before 1991
    }

    #[test]
    fn nine_star_ki_always_in_range() {
        for year in 1900..=2100 {
            let s = nine_star_ki_natal(year);
            assert!(s >= 1 && s <= 9, "Star {} out of range for year {}", s, year);
        }
    }

    // ── Wu Xing ───────────────────────────────────────────────────────────────

    #[test]
    fn wu_xing_one_is_water() {
        assert_eq!(wu_xing(1).unwrap().element_en, "Water");
    }

    #[test]
    fn wu_xing_nine_is_fire() {
        assert_eq!(wu_xing(9).unwrap().element_en, "Fire");
    }

    #[test]
    fn wu_xing_three_four_are_wood() {
        assert_eq!(wu_xing(3).unwrap().element_en, "Wood (Thunder)");
        assert_eq!(wu_xing(4).unwrap().element_en, "Wood (Wind)");
    }

    #[test]
    fn wu_xing_out_of_range_returns_none() {
        assert!(wu_xing(0).is_none());
        assert!(wu_xing(10).is_none());
    }

    // ── Chinese lucky numbers ─────────────────────────────────────────────────

    #[test]
    fn lucky_eight_is_extremely_lucky() {
        let info = chinese_lucky_meaning(8).unwrap();
        assert!(info.luck.contains("Extremely Lucky"));
    }

    #[test]
    fn unlucky_four() {
        let info = chinese_lucky_meaning(4).unwrap();
        assert!(info.luck.contains("Unlucky"));
    }

    #[test]
    fn triple_eight_exists() {
        assert!(chinese_lucky_meaning(888).is_some());
    }
}
