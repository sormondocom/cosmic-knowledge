//! African cosmological systems.
//!
//! **African:**
//!  - Yoruba Ifá: the 16 principal Odù with qualities and domains
//!  - Akan day names: birth day-of-week → Kra soul name
//!  - Kemetic / Ancient Egyptian: sacred number meanings from Ennead/Ogdoad tradition
//!
//! Sources:
//!  - Abimbola, W. *Ifá: An Exposition of Ifá Literary Corpus* (1976, OUP Nigeria)
//!  - Gyekye, K. *An Essay on African Philosophical Thought* (1987, Cambridge)
//!  - Opoku, K.A. *West African Traditional Religion* (1978, FEP International)
//!  - Morenz, S. *Egyptian Religion* (1973, Cornell University Press)
//!  - Wilkinson, R. *The Complete Gods and Goddesses of Ancient Egypt* (2003, Thames & Hudson)
//!  - Faulkner, R. *The Ancient Egyptian Book of the Dead* (1972, British Museum Press)

use std::io::{self, Write};
use colored::*;

// ─── African: Yoruba Ifá — 16 Principal Odù ───────────────────────────────────

pub struct IfaOdu {
    pub index:    u8,         // 1–16
    pub name:     &'static str,
    pub orisha:   &'static str,   // associated deity
    pub domain:   &'static str,
    pub quality:  &'static str,
}

/// Return the Ifá Odù for an index 1–16.
///
/// Sequence follows Abimbola (1976) Oju Odù ordering.
/// Source: Wande Abimbola, *Ifá: An Exposition of Ifá Literary Corpus* (1976, OUP Nigeria).
pub fn ifa_odu(index: u8) -> Option<IfaOdu> {
    match index {
        1  => Some(IfaOdu { index: 1,  name: "Ogbe (Ògbè)",
            orisha: "Orunmila / Ọrunmìlà (Ifá himself)",
            domain: "Prosperity, dawn, new beginnings, divine wisdom",
            quality: "The first and most senior Odù; associated with light, \
                      abundance, and the direct voice of Orunmila. All single \
                      marks (||||): maximum yang energy",
        }),
        2  => Some(IfaOdu { index: 2,  name: "Oyeku (Òyèkú)",
            orisha: "Iku (Death) / Egungun (Ancestors)",
            domain: "Death, endings, transformation, ancestral wisdom",
            quality: "The counterpart of Ogbe — all double marks (!!). Represents \
                      night, the ancestors, and the completion of cycles. Feared \
                      but also revered as the wisdom that comes from endings",
        }),
        3  => Some(IfaOdu { index: 3,  name: "Iwori (Ìwòrì)",
            orisha: "Ogún (Iron) and Elégba (Crossroads)",
            domain: "Inner sight, introspection, self-knowledge, hidden truth",
            quality: "The Odù of seeing within. Governs dreams, \
                      meditation, and the exploration of one's own nature. \
                      Warns against self-deception",
        }),
        4  => Some(IfaOdu { index: 4,  name: "Odi (Òdí)",
            orisha: "Yemoja (Waters / Mother of Waters)",
            domain: "Secrets, gestation, the womb, hidden power",
            quality: "The Odù of that which is concealed and growing. \
                      Associated with pregnancy, plans not yet revealed, \
                      and the deep creative potential before manifestation",
        }),
        5  => Some(IfaOdu { index: 5,  name: "Irosun (Ìròsùn)",
            orisha: "Ọṣun (Love, Fertility, Sweet Water)",
            domain: "Blood, women, fertility, love, wealth",
            quality: "The Odù of Osun — governs relationships, women's power, \
                      sensuality, abundance, and the flow of vital force. \
                      Named for the red dye *osun* (camwood)",
        }),
        6  => Some(IfaOdu { index: 6,  name: "Owonrin (Òwónrín)",
            orisha: "Eṣu / Elégba (Trickster, Divine Messenger)",
            domain: "Unpredictability, chaos, sudden change, crossroads",
            quality: "The most unpredictable Odù — governs sudden reversals, \
                      unexpected opportunities, and the role of the trickster. \
                      Associated with Eshu and lightning",
        }),
        7  => Some(IfaOdu { index: 7,  name: "Obara (Òbàrà)",
            orisha: "Ṣàngó (Thunder, Lightning, Justice)",
            domain: "Royalty, pride, leadership, charisma",
            quality: "The Odù of kings. Governs power, authority, and the \
                      responsibility that comes with greatness. \
                      Warns against arrogance",
        }),
        8  => Some(IfaOdu { index: 8,  name: "Okanran (Òkànràn)",
            orisha: "Ògún (Iron, War, Labour)",
            domain: "Conflict, fire, impatience, raw power",
            quality: "The Odù of sudden action and conflict. Governs battles, \
                      arguments, and impulsive decisions. Advises patience \
                      and careful planning before acting",
        }),
        9  => Some(IfaOdu { index: 9,  name: "Ogunda (Ògúndá)",
            orisha: "Ògún (Iron, Clearing the Path)",
            domain: "Iron, clearing obstacles, new pathways, surgery",
            quality: "The path-clearing Odù. Governs the removal of obstacles \
                      through direct action. Associated with surgery, farming, \
                      and any work with iron or sharp implements",
        }),
        10 => Some(IfaOdu { index: 10, name: "Osa (Òsá)",
            orisha: "Ọya (Wind, Storms, Change)",
            domain: "Sudden reversals, storms, witchcraft, transformation",
            quality: "The Odù of sudden and often violent change. \
                      Associated with Oya — the wind that strips leaves \
                      from trees. Danger from enemies; but also radical \
                      liberation",
        }),
        11 => Some(IfaOdu { index: 11, name: "Ika (Ìká)",
            orisha: "Ọbàtálá (Creation, Purity, White Cloth)",
            domain: "Stubbornness, pride, creative power, wisdom",
            quality: "The Odù governing creative but inflexible energy. \
                      Warns against stubbornness blocking one's own blessing. \
                      Associated with the artist and the visionary",
        }),
        12 => Some(IfaOdu { index: 12, name: "Oturupon (Òtúrúpọ̀n)",
            orisha: "Ọbàtálá and Ọya",
            domain: "Transformation, liminality, what is hidden becoming revealed",
            quality: "The Odù of threshold states — between life and death, \
                      waking and dreaming. Governs radical transformation and \
                      the point of no return",
        }),
        13 => Some(IfaOdu { index: 13, name: "Otura (Òtúrá)",
            orisha: "Ọbàtálá (the Creator)",
            domain: "Creation, Obatala, white cloth, the sky, purity",
            quality: "The Odù of divine creation and purity. \
                      Associated with Obatala who sculpts human form. \
                      Governs purity of intent, spiritual elevation",
        }),
        14 => Some(IfaOdu { index: 14, name: "Irete (Ìrẹtẹ̀)",
            orisha: "Ọlọdùmarè (the Supreme Being) and Orunmila",
            domain: "Long life, stability, spiritual mastery, the elder",
            quality: "The Odù of the wise elder. Governs long life, \
                      accumulated wisdom, and spiritual mastery. \
                      Associated with patience and the long view",
        }),
        15 => Some(IfaOdu { index: 15, name: "Ose (Òsẹ̀)",
            orisha: "Ọṣun (Love) and Orisa Oko (Agriculture)",
            domain: "Fertility, wealth, children, abundance of the earth",
            quality: "The Odù of material and spiritual abundance. \
                      Governs fertility of the land, women, and business. \
                      Associated with the sweetness of life",
        }),
        16 => Some(IfaOdu { index: 16, name: "Ofun (Òfún)",
            orisha: "Yemọja (Ocean, Mother of Waters)",
            domain: "Death, rebirth, the deep ocean, the supreme mystery",
            quality: "The 16th and final principal Odù — the complete cycle \
                      from Ogbe (birth) to Ofun (death and return). \
                      Governs the deepest mysteries and the threshold \
                      of transformation",
        }),
        _ => None,
    }
}

// ─── African: Akan Day Names ───────────────────────────────────────────────────

pub struct AkanDay {
    pub day_en:        &'static str,
    pub day_akan:      &'static str,
    pub male_name:     &'static str,
    pub female_name:   &'static str,
    pub orisa:         &'static str,   // associated deity/spiritual force
    pub qualities:     &'static str,
}

/// Return the Akan day name for a day-of-week index.
/// `weekday`: 0 = Sunday, 1 = Monday, … 6 = Saturday.
///
/// Source: Gyekye, K. *An Essay on African Philosophical Thought* (1987, Cambridge);
///         Opoku, K.A. *West African Traditional Religion* (1978, FEP International).
pub fn akan_day_name(weekday: u8) -> AkanDay {
    match weekday % 7 {
        0 => AkanDay { day_en: "Sunday",    day_akan: "Kwasida",
            male_name: "Kwasi",  female_name: "Akosua",
            orisa: "Nyankopong (the High God, the Sun)",
            qualities: "Noble, spiritual, creative, solar energy; natural leaders \
                        with strong moral compass",
        },
        1 => AkanDay { day_en: "Monday",    day_akan: "Dwowda",
            male_name: "Kwadwo", female_name: "Adwoa",
            orisa: "The Moon",
            qualities: "Peaceful, calm, diplomatic, introspective; gifted \
                        mediators and peacemakers",
        },
        2 => AkanDay { day_en: "Tuesday",   day_akan: "Benada",
            male_name: "Kwabena", female_name: "Abena",
            orisa: "The Ocean / Sea spirits",
            qualities: "Bold, warrior-spirit, energetic, courageous; \
                        natural athletes and defenders",
        },
        3 => AkanDay { day_en: "Wednesday", day_akan: "Wukuda",
            male_name: "Kwaku",  female_name: "Akua",
            orisa: "Anansi (Spider) — the divine trickster",
            qualities: "Clever, versatile, mischievous, quick-witted; \
                        natural storytellers and problem-solvers. Wednesday \
                        is Anansi's day — expect the unexpected",
        },
        4 => AkanDay { day_en: "Thursday",  day_akan: "Yawda",
            male_name: "Yaw",    female_name: "Yaa",
            orisa: "Earth Goddess (Asase Yaa)",
            qualities: "Patient, powerful, generous, connected to the earth; \
                        strong sense of justice and perseverance",
        },
        5 => AkanDay { day_en: "Friday",    day_akan: "Fida",
            male_name: "Kofi",   female_name: "Afua / Efua",
            orisa: "The River",
            qualities: "Adventurous, warm, loving, sociable; diplomatic \
                        and versatile; known for their charm and charisma. \
                        (Kofi Annan — born Friday)",
        },
        _ => AkanDay { day_en: "Saturday",  day_akan: "Memenda",
            male_name: "Kwame",  female_name: "Ama",
            orisa: "Nyame (the Supreme Being, sky god)",
            qualities: "Responsible, spiritual, deeply connected to the divine; \
                        Saturday is Nyame's day — those born on it carry \
                        special spiritual authority. (Kwame Nkrumah — born Saturday)",
        },
    }
}

// ─── African: Kemetic / Ancient Egyptian sacred numbers ────────────────────────

/// Return the Kemetic / Ancient Egyptian significance of a number.
///
/// Source: Morenz (1973); Wilkinson (2003); Faulkner (1972).
pub fn kemetic_meaning(number: u32) -> Option<&'static str> {
    match number {
        1 => Some("☀️ Atum / Ra — the self-created One; the primordial mound emerging from Nun; \
                   unity before the first division; the source from which all gods and creation proceed"),
        2 => Some("⚖️ Ma'at — the Two Truths (truth and justice); duality of Upper and Lower Egypt; \
                   Shu (air) and Tefnut (moisture) as the first paired creation; the scales of divine balance"),
        3 => Some("△ The divine triad — Osiris, Isis, Horus; the sacred family; \
                   plurality in Egyptian grammar (three strokes = many); triads at every major temple"),
        4 => Some("✦ The four cardinal directions; the Four Sons of Horus guarding the canopic jars \
                   (Imseti, Hapy, Duamutef, Qebehsenuef); the four pillars of the sky goddess Nut"),
        5 => Some("✦ The five epagomenal days (the days 'outside the year'); birthdays of \
                   Osiris, Horus the Elder, Set, Isis, and Nephthys; the number of life in \
                   certain medical papyri"),
        6 => Some("☽ Six months of Akhet (flood season); the six-rayed star of Sopdet (Sirius); \
                   the number of completeness in certain temple ritual counts"),
        7 => Some("🌟 The seven celestial bodies (classical planets) governing the week; \
                   seven sacred lakes; seven gates of the Duat (underworld); \
                   the seven Hathors who determine a child's fate"),
        8 => Some("∞ The Ogdoad — eight primordial deities of Hermopolis: Nun & Naunet (water), \
                   Huh & Hauhet (infinity), Kuk & Kauket (darkness), Amun & Amaunet (hiddenness); \
                   total pre-creation chaos in its complete (eight-fold) form"),
        9 => Some("⊙ The Ennead of Heliopolis — nine gods: Atum, Shu, Tefnut, Geb, Nut, \
                   Osiris, Isis, Set, Nephthys; cosmic completion; \
                   also the 'Nine Bows' representing the totality of Egypt's enemies"),
        42 => Some("⚖️ The 42 Assessors of the Dead — one for each nome of Egypt (22 Upper + 20 Lower); \
                    the 42 Negative Confessions in Spell 125 of the Book of the Dead; \
                    the measure of divine judgment: 42 = 6×7, combining cosmic and sacred numbers"),
        360 => Some("☀️ The 360-day administrative sacred year + 5 epagomenal days = 365; \
                     the 36 decans of the Egyptian zodiac (each covering 10° = 10 days); \
                     Thoth won 1/72 of moonlight in a dice game to create the five extra days. \
                     360 days × 36 decans forms the backbone of Egyptian astronomical time"),
        _ => None,
    }
}

// ─── Display / session functions ───────────────────────────────────────────────

/// Interactive African cosmology session (Ifá Odù, Akan day names, Kemetic numbers).
pub fn run_african_session() {
    loop {
        println!();
        println!("{}", "╔══════════════════════════════════════════════════════════╗".bright_cyan());
        println!("{}", "║           🌍  AFRICAN COSMOLOGICAL SYSTEMS               ║".bold().bright_cyan());
        println!("{}", "╠══════════════════════════════════════════════════════════╣".bright_cyan());
        println!("{}", "║   1.  🎋  Yoruba Ifá — The 16 Principal Odù              ║".bright_white());
        println!("{}", "║          Orisha · domain · divinatory quality            ║".dimmed());
        println!("{}", "║                                                          ║".bright_cyan());
        println!("{}", "║   2.  🌅  Akan Day Names — Birth Day Soul Name           ║".bright_white());
        println!("{}", "║          Kra name, qualities, and spiritual guardian     ║".dimmed());
        println!("{}", "║                                                          ║".bright_cyan());
        println!("{}", "║   3.  ☥  Kemetic Numbers — Ancient Egyptian Meanings     ║".bright_white());
        println!("{}", "║          Ogdoad, Ennead, Ma'at, and sacred numbers       ║".dimmed());
        println!("{}", "║                                                          ║".bright_cyan());
        println!("{}", "║   0.  ←  Back                                            ║".dimmed());
        println!("{}", "╚══════════════════════════════════════════════════════════╝".bright_cyan());
        println!();

        print!("{}", "▸ Enter choice: ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { break; }

        match input.trim() {
            "1" => ifa_odu_session(),
            "2" => akan_day_session(),
            "3" => kemetic_session(),
            "0" | "" => break,
            _ => println!("{}", "  Please enter 0–3.".yellow()),
        }
    }
}

fn ifa_odu_session() {
    println!();
    println!("{}", "  ╔═══════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "  ║    🎋 THE 16 PRINCIPAL ODÙ OF IFÁ — Wande Abimbola    ║".bold().bright_cyan());
    println!("{}", "  ║    Source: Yoruba, Benin, Nigeria · UNESCO ICH 2008   ║".dimmed());
    println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_cyan());
    for i in 1u8..=16 {
        if let Some(odu) = ifa_odu(i) {
            println!("  {}", format!("║  {:>2}.  {:<49}║", i, odu.name).bright_white());
        }
    }
    println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_cyan());
    println!("{}", "  ║    0.  ← Back                                         ║".dimmed());
    println!("{}", "  ╚═══════════════════════════════════════════════════════╝".bright_cyan());
    println!();

    loop {
        print!("{}", "  ▸ Enter Odù number (1-16) or 0 to go back: ".cyan());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() { break; }
        let trimmed = buf.trim();
        if trimmed == "0" || trimmed.is_empty() { break; }

        if let Ok(n) = trimmed.parse::<u8>() {
            if let Some(odu) = ifa_odu(n) {
                println!();
                println!("{}", "  ╔═══════════════════════════════════════════════════════╗".bright_yellow());
                println!("  {}", format!("║  🎋 {:>2}. {}                                    ║", odu.index, odu.name).bold().bright_yellow());
                println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_yellow());
                println!("  {}", format!("║  Orisha : {:<45}║", odu.orisha).bright_cyan());
                println!("  {}", format!("║  Domain : {:<45}║", odu.domain).bright_white());
                println!("{}", "  ╠═══════════════════════════════════════════════════════╣".bright_yellow());
                let words: Vec<&str> = odu.quality.split_whitespace().collect();
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
                println!("{}", "  ╚═══════════════════════════════════════════════════════╝".bright_yellow());
                println!();
            } else {
                println!("{}", "  Enter a number between 1 and 16.".yellow());
            }
        } else {
            println!("{}", "  Enter a number between 1 and 16.".yellow());
        }
    }
}

fn akan_day_session() {
    println!();
    println!("{}", "  Enter a day of the week to find the Akan soul name (Kra din).".dimmed());
    println!("{}", "  (0=Sunday  1=Monday  2=Tuesday  3=Wednesday".dimmed());
    println!("{}", "   4=Thursday  5=Friday  6=Saturday)".dimmed());
    println!("{}", "  (empty line → back)".dimmed());

    loop {
        print!("{}", "\n  ▸ Enter day number (0–6): ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() { break; }
        let trimmed = buf.trim();
        if trimmed.is_empty() { break; }

        let Ok(d) = trimmed.parse::<u8>() else {
            println!("{}", "  ⚠️  Enter 0 (Sunday) through 6 (Saturday).".yellow());
            continue;
        };

        let day = akan_day_name(d);
        println!();
        println!("  {} — {}", day.day_en.bold().bright_yellow(), day.day_akan.bright_cyan());
        println!("  Male name   : {}", day.male_name.bold().bright_white());
        println!("  Female name : {}", day.female_name.bold().bright_white());
        println!("  Guardian    : {}", day.orisa.italic().bright_magenta());
        println!("  Qualities   : {}", day.qualities.bright_white());
    }
}

fn kemetic_session() {
    println!();
    println!("{}", "  Enter a number to see its Ancient Egyptian / Kemetic significance.".dimmed());
    println!("{}", "  Notable numbers: 1 2 3 4 5 6 7 8 9 42 360".dimmed());
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

        match kemetic_meaning(n) {
            None => println!("{}", format!("  ℹ️  {} has no specific Kemetic significance in this database.", n).dimmed()),
            Some(meaning) => {
                println!();
                println!("  {} {}", "☥ ".bold().bright_yellow(), format!("Kemetic significance of {}:", n).bold().bright_yellow());
                println!("  {}", meaning.bright_white());
            }
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Ifá Odù ───────────────────────────────────────────────────────────────

    #[test]
    fn ifa_first_odu_is_ogbe() {
        let odu = ifa_odu(1).unwrap();
        assert!(odu.name.contains("Ogbe"));
    }

    #[test]
    fn ifa_last_odu_is_ofun() {
        let odu = ifa_odu(16).unwrap();
        assert!(odu.name.contains("Ofun"));
    }

    #[test]
    fn ifa_all_16_exist() {
        for i in 1u8..=16 {
            assert!(ifa_odu(i).is_some(), "Odù {} missing", i);
        }
    }

    #[test]
    fn ifa_out_of_range_returns_none() {
        assert!(ifa_odu(0).is_none());
        assert!(ifa_odu(17).is_none());
    }

    // ── Akan day names ────────────────────────────────────────────────────────

    #[test]
    fn akan_sunday_is_kwasi() {
        let day = akan_day_name(0);
        assert_eq!(day.male_name, "Kwasi");
    }

    #[test]
    fn akan_friday_is_kofi() {
        let day = akan_day_name(5);
        assert_eq!(day.male_name, "Kofi");
    }

    #[test]
    fn akan_wraps_mod7() {
        // 7 % 7 = 0 → Sunday
        assert_eq!(akan_day_name(7).male_name, akan_day_name(0).male_name);
    }

    // ── Kemetic numbers ───────────────────────────────────────────────────────

    #[test]
    fn kemetic_eight_is_ogdoad() {
        let m = kemetic_meaning(8).unwrap();
        assert!(m.contains("Ogdoad"));
    }

    #[test]
    fn kemetic_nine_is_ennead() {
        let m = kemetic_meaning(9).unwrap();
        assert!(m.contains("Ennead"));
    }

    #[test]
    fn kemetic_unknown_returns_none() {
        assert!(kemetic_meaning(100).is_none());
    }
}
