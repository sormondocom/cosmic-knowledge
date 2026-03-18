//! Interactive Urim & Thummim session — all terminal I/O for the oracle sub-menu.
//!
//! Functions:
//!  - [`run_urim_session`]      — top-level sub-menu loop
//!  - `browse_breastplate`     — display the 12 stones as a 4×3 breastplate grid
//!  - `cast_oracle`            — TRNG-based binary oracle with three outcomes
//!  - `show_lore`              — detailed historical and scholarly summary
//!  - `view_history`           — reading history by user or global

use colored::*;
use std::io::{self, Write};

use rdrand::RdRand;

use crate::export::{handle_export, wrap_html};
use crate::menu::{Menu, MenuColor, MenuItem};
use crate::persistence::{get_all_readings, get_or_create_user, get_user_readings, open_db,
                         record_reading};
use crate::reports::chrono_now;

use super::breastplate::{BreastplateStone, BREASTPLATE};

// ─── Oracle outcome categories ────────────────────────────────────────────────

/// The three possible oracular outcomes of the Urim and Thummim.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome {
    /// Urim — Light affirms (~40 % probability).
    Urim,
    /// Thummim — Perfection completes / negates (~40 % probability).
    Thummim,
    /// Silence — The LORD withholds (~20 % probability).
    Silence,
}

// ─── Oracle utterances ────────────────────────────────────────────────────────

static URIM_UTTERANCES: &[&str] = &[
    "The path before thee is lit.  Go forward in the light of the Eternal.",
    "As the stones of the breastplate gleam, so shines the answer: proceed.",
    "The Urim speaks — the way is open.  Trust the voice of the Living God.",
    "Light affirms.  The LORD has made thy way prosperous — go and do.",
    "The sacred light descends upon thy question.  The answer is yes.",
    "Even as the priest stood before the Ark and the stones blazed, so \
     blazes the answer: the way is clear.",
    "Thy petition has been heard before the Mercy Seat.  Rise and act in faith.",
    "As the sun breaks upon the eastern hills, so the Urim illuminates: \
     the door stands open before thee.",
    "The breastplate gleams.  Numbers 27:21 is fulfilled — the Eternal has \
     answered by Urim.  Proceed.",
];

static THUMMIM_UTTERANCES: &[&str] = &[
    "This thing shall not be accomplished.  Seek another path in wisdom.",
    "The Thummim seals the way.  In completion, turn and find the truer road.",
    "Not this — the Perfection of the Eternal closes this door for thy good.",
    "The oracle is complete.  What thou seekest here is not for thee in this \
     hour.  Turn aside.",
    "As Saul was answered not by Urim in the hour of his transgression, so \
     this path is sealed.  Repent and return.",
    "The Thummim lies still.  The answer is no.  Let this be enough.",
    "Perfection speaks: the design of the Eternal runs otherwise.  Submit \
     to the higher wisdom.",
    "The Kohen Gadol turns from the Ark.  The answer is written in closure: \
     this is not thy appointed way.",
    "As Ezra wrote of the priest who could not stand with Urim and Thummim \
     (Ezra 2:63), so thou must wait for a cleaner hour and a clearer question.",
];

static SILENCE_UTTERANCES: &[&str] = &[
    "The sacred stones are still.  Return when thy heart is prepared.",
    "The LORD answers not by Urim this hour.  Pray, fast, and return.",
    "As in the days of Saul (1 Samuel 28:6), silence is itself the answer. \
     Wait upon the Eternal.",
    "The breastplate is still.  The oracle withholds.  This question belongs \
     to a later season.",
    "The stones do not illuminate.  The Eternal bids thee be patient \
     and humble thy seeking.",
    "As the Urim and Thummim fell silent after the First Temple (Ezra 2:63), \
     so silence descends.  Dwell in trust.",
    "Heaven answers not.  Seek first the kingdom, then return with thy \
     petition refined.",
    "The stones rest in darkness.  This is not the hour of the oracle. \
     Withdraw into prayer.",
];

// ─── Sub-menu definition ──────────────────────────────────────────────────────

static URIM_ITEMS: &[MenuItem] = &[
    MenuItem {
        key: "1",
        icon: "✦",
        label: "Browse the Breastplate  (12 sacred stones)",
        hint: "4×3 grid · Hebrew names · Tribes · Gem identifications",
    },
    MenuItem {
        key: "2",
        icon: "☀",
        label: "Cast the Oracle",
        hint: "Binary oracle: Urim · Thummim · Silence — TRNG entropy",
    },
    MenuItem {
        key: "3",
        icon: "📜",
        label: "Historical Lore & Scholarship",
        hint: "Sources · scholarly debates · significance",
    },
    MenuItem {
        key: "4",
        icon: "📖",
        label: "Reading History",
        hint: "Browse all recorded oracle consultations",
    },
];

static URIM_MENU: Menu = Menu {
    title: "ℵ  URIM & THUMMIM  ·  The Oracle of the High Priest",
    border_color: MenuColor::Yellow,
    items: URIM_ITEMS,
    back_key: "0",
    back_label: "Back to main menu",
};

// ─── Entry point ──────────────────────────────────────────────────────────────

/// Run the Urim & Thummim sub-menu loop.
pub fn run_urim_session() {
    loop {
        let choice = URIM_MENU.show_and_read();
        match choice.trim() {
            "1" => browse_breastplate(),
            "2" => cast_oracle(),
            "3" => show_lore(),
            "4" => view_history(),
            "0" | "" => break,
            _ => {}
        }
    }
}

// ─── Browse breastplate ───────────────────────────────────────────────────────

fn browse_breastplate() {
    println!();
    println!(
        "{}",
        "  ══  THE CHOSHEN MISHPAT — BREASTPLATE OF JUDGMENT  ══"
            .bright_yellow()
            .bold()
    );
    println!(
        "  {}",
        "Exodus 28:15–30 · Josephus, Antiquities 3.7.5 · Maimonides, Mishneh Torah 9"
            .dimmed()
    );
    println!();
    println!(
        "  {}",
        "The breastplate bore twelve stones, one for each tribe of Israel.  Set in \
         four rows of three, they rested over the heart of the High Priest."
            .white()
    );
    println!();

    // Print as a 4×3 grid mirroring the physical breastplate
    for row in 1u8..=4 {
        let stones: Vec<&BreastplateStone> =
            BREASTPLATE.iter().filter(|s| s.row == row).collect();

        // ── Glyph / position header row ──────────────────────────────────────
        print!("  ");
        for s in &stones {
            print!(
                "  {:^22}",
                format!("[ {} ]", s.position).bright_yellow().bold()
            );
        }
        println!();

        // ── Hebrew name row ──────────────────────────────────────────────────
        print!("  ");
        for s in &stones {
            print!("  {:^22}", s.hebrew_name.bright_white().bold());
        }
        println!();

        // ── Stone name row ───────────────────────────────────────────────────
        print!("  ");
        for s in &stones {
            print!("  {:^22}", s.stone_name.cyan());
        }
        println!();

        // ── Tribe row ────────────────────────────────────────────────────────
        print!("  ");
        for s in &stones {
            print!("  {:^22}", s.tribe.bright_magenta());
        }
        println!();

        // ── Tribe meaning row ────────────────────────────────────────────────
        print!("  ");
        for s in &stones {
            let meaning = format!("\"{}\"", s.tribe_meaning);
            print!("  {:^22}", meaning.dimmed());
        }
        println!();

        // ── Color row ────────────────────────────────────────────────────────
        print!("  ");
        for s in &stones {
            print!("  {:^22}", s.color.yellow());
        }
        println!();

        println!(
            "  {}",
            "  ─────────────────────────────────────────────────────────────────"
                .dimmed()
        );
    }

    println!();
    println!(
        "  {}",
        "Enter a stone number (1–12) for full detail, or press Enter to continue."
            .dimmed()
    );
    print!("{}", "  ▸ Stone number: ".bold().bright_yellow());
    io::stdout().flush().unwrap_or(());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap_or(0);
    let trimmed = buf.trim();
    if let Ok(n) = trimmed.parse::<u8>() {
        if let Some(stone) = BREASTPLATE.iter().find(|s| s.position == n) {
            print_stone_detail(stone);
        }
    }

    // Export
    handle_export(
        "urim_breastplate",
        || build_breastplate_text(),
        || {
            let body = build_breastplate_html();
            wrap_html("Urim & Thummim — The Breastplate of Judgment", &body, "angelic")
        },
    );
}

fn print_stone_detail(s: &BreastplateStone) {
    let divider = "─".repeat(60).dimmed();
    println!();
    println!("  {}", divider);
    println!(
        "  {}  —  Stone {} of the Choshen Mishpat",
        s.hebrew_name.bright_yellow().bold(),
        s.position.to_string().bright_white()
    );
    println!(
        "  {}  Row {}, Column {}",
        s.stone_name.cyan().bold(),
        s.row,
        s.col
    );
    println!();
    println!(
        "  {:<14} {}",
        "Tribe:".bright_yellow(),
        s.tribe.bright_white()
    );
    println!(
        "  {:<14} {}",
        "Meaning:".bright_yellow(),
        format!("\"{}\"", s.tribe_meaning).white()
    );
    println!(
        "  {:<14} {}",
        "Color:".bright_yellow(),
        s.color.white()
    );
    println!(
        "  {:<14} {}",
        "Scripture:".bright_yellow(),
        s.scripture.white()
    );
    println!();
    println!("  {}", "Attributes:".bright_yellow());
    for line in word_wrap(s.attributes, 64) {
        println!("    {}", line.white());
    }
    println!("  {}", divider);
    println!();
}

// ─── Cast the oracle ──────────────────────────────────────────────────────────

fn cast_oracle() {
    println!();
    println!(
        "{}",
        "  ══  CAST THE ORACLE — URIM & THUMMIM  ══"
            .bright_yellow()
            .bold()
    );
    println!(
        "  {}",
        "Numbers 27:21 · 1 Samuel 14:41 · Cornelius van Dam, The Urim and Thummim (1997)"
            .dimmed()
    );
    println!();
    println!(
        "  {}",
        "The High Priest stands before the Ark.  The Choshen Mishpat rests upon his \
         heart.  You may now bring your question before the Eternal."
            .white()
    );
    println!();

    // Gather question
    print!("{}", "  ▸ State your question: ".bold().bright_yellow());
    io::stdout().flush().unwrap_or(());
    let mut question_buf = String::new();
    io::stdin().read_line(&mut question_buf).unwrap_or(0);
    let question = question_buf.trim().to_string();
    if question.is_empty() {
        println!("  {}", "No question given.  Returning to menu.".dimmed());
        return;
    }

    // Gather querent name
    print!(
        "{}",
        "  ▸ Your name for the record (Enter for anonymous): "
            .bold()
            .bright_yellow()
    );
    io::stdout().flush().unwrap_or(());
    let mut name_buf = String::new();
    io::stdin().read_line(&mut name_buf).unwrap_or(0);
    let querent = {
        let t = name_buf.trim();
        if t.is_empty() { "Anonymous" } else { t }.to_string()
    };

    // Use hardware RNG (RDRAND) with OS-entropy fallback
    let rng = RdRand::new().ok();

    // Determine the illuminating stone (random from 12)
    let stone_idx = (oracle_rnd(&rng) % 12) as usize;
    let illuminated = &BREASTPLATE[stone_idx];

    // Determine outcome using weighted probability:
    //   0..=39  → Urim    (40%)
    //  40..=79  → Thummim (40%)
    //  80..=99  → Silence (20%)
    let weight_raw = oracle_rnd(&rng) % 100;
    let outcome = if weight_raw < 40 {
        Outcome::Urim
    } else if weight_raw < 80 {
        Outcome::Thummim
    } else {
        Outcome::Silence
    };

    // Select utterance
    let utterances: &[&str] = match outcome {
        Outcome::Urim => URIM_UTTERANCES,
        Outcome::Thummim => THUMMIM_UTTERANCES,
        Outcome::Silence => SILENCE_UTTERANCES,
    };
    let utterance_idx = (oracle_rnd(&rng) % utterances.len() as u64) as usize;
    let utterance = utterances[utterance_idx];

    // ── Display ceremony ──────────────────────────────────────────────────────
    println!();
    println!(
        "  {}",
        "══════════════════════════════════════════════════════════════"
            .bright_yellow()
            .dimmed()
    );
    println!();
    println!(
        "  {} {}",
        "The High Priest consults the Urim and Thummim".bright_yellow().italic(),
        "(Numbers 27:21)".dimmed()
    );
    println!(
        "  {}",
        format!("  Querent: {}  ·  {}", querent, chrono_now()).dimmed()
    );
    println!();
    println!(
        "  {}",
        format!("  Question: {}", question).white().italic()
    );
    println!();

    // Illuminated stone
    println!(
        "  {}",
        "  — The stone that illuminated: —".bright_yellow()
    );
    println!(
        "  {}  {}  ·  Tribe of {}",
        illuminated.hebrew_name.bright_white().bold(),
        illuminated.stone_name.cyan(),
        illuminated.tribe.bright_magenta()
    );
    println!();

    // Outcome header
    match outcome {
        Outcome::Urim => {
            println!(
                "  {}",
                "  ☀  URIM  ·  Light affirms  ☀".bright_yellow().bold()
            );
        }
        Outcome::Thummim => {
            println!(
                "  {}",
                "  ✦  THUMMIM  ·  Perfection completes  ✦"
                    .bright_white()
                    .bold()
            );
        }
        Outcome::Silence => {
            println!(
                "  {}",
                "  ···  SILENCE  ·  The LORD withholds  ···".dimmed().bold()
            );
        }
    }
    println!();

    // Oracle utterance
    for line in word_wrap(utterance, 64) {
        println!("    {}", line.bright_yellow().italic());
    }
    println!();
    println!(
        "  {}",
        "══════════════════════════════════════════════════════════════"
            .bright_yellow()
            .dimmed()
    );
    println!();

    // Outcome description
    let outcome_label = match outcome {
        Outcome::Urim => "URIM — Light affirms",
        Outcome::Thummim => "THUMMIM — Perfection completes",
        Outcome::Silence => "SILENCE — The LORD withholds",
    };

    // ── Persist the reading ───────────────────────────────────────────────────
    let cards_text = format!(
        "Question: {}\nOutcome: {}\nIlluminated stone: {} ({}) — Tribe of {}\nOracle: {}",
        question,
        outcome_label,
        illuminated.hebrew_name,
        illuminated.stone_name,
        illuminated.tribe,
        utterance,
    );

    if let Ok(conn) = open_db() {
        if let Ok((user, _)) = get_or_create_user(&conn, &querent) {
            record_reading(
                &conn,
                &user.id,
                "Urim & Thummim",
                "Binary Oracle",
                &cards_text,
            )
            .ok();
        }
    }

    // ── Export ────────────────────────────────────────────────────────────────
    let timestamp = chrono_now();
    let text_snap = build_reading_text(
        &querent,
        &question,
        outcome_label,
        illuminated,
        utterance,
        &timestamp,
    );
    let html_snap = build_reading_html(
        &querent,
        &question,
        outcome_label,
        illuminated,
        utterance,
        &timestamp,
    );

    handle_export(
        "urim_oracle_reading",
        || text_snap.clone(),
        || wrap_html("Urim & Thummim — Oracle Reading", &html_snap, "angelic"),
    );
}

// ─── Historical lore ─────────────────────────────────────────────────────────

fn show_lore() {
    println!();
    println!(
        "{}",
        "  ══  URIM & THUMMIM — HISTORICAL LORE & SCHOLARSHIP  ══"
            .bright_yellow()
            .bold()
    );
    println!();

    let sections: &[(&str, &str)] = &[
        (
            "What Were the Urim and Thummim?",
            "The Urim (אוּרִים, 'Lights') and Thummim (תּוּמִים, 'Perfections' or \
             'Completions') were a sacred oracular device entrusted to the High Priest \
             (Kohen Gadol) of ancient Israel.  They were placed within the Choshen \
             Mishpat — the breastplate of judgment — which was itself set with twelve \
             precious stones, one for each tribe (Exodus 28:15–30).  The breastplate was \
             worn over the High Priest's heart when he entered the Tabernacle or Temple \
             to inquire of the LORD on behalf of the nation.",
        ),
        (
            "Primary Scripture References",
            "• Exodus 28:15–30 — The full description of the breastplate and the \
             commandment: 'you shall put in the breastplate of judgment the Urim and \
             the Thummim; and they shall be upon Aaron's heart when he goes in before \
             the LORD.'\n\
             • Leviticus 8:8 — At the consecration of Aaron: 'He placed the \
             breastplate upon him, and he put the Urim and Thummim in the breastplate.'\n\
             • Numbers 27:21 — Moses commands Joshua to stand before Eleazar the priest \
             'who shall inquire for him by the judgment of the Urim before the LORD.'\n\
             • 1 Samuel 14:41 — Saul's famous consultation; the Septuagint (LXX) text \
             preserves the clearest binary reading: 'Give Thummim' / 'Give Urim,' \
             reflecting the two-valued oracle structure.\n\
             • 1 Samuel 28:6 — 'And when Saul inquired of the LORD, the LORD answered \
             him not, neither by dreams, nor by Urim, nor by prophets.' (Divine silence \
             as the third state.)\n\
             • Ezra 2:63 / Nehemiah 7:65 — After the Babylonian exile, returnees whose \
             priestly lineage was uncertain were told to wait 'until a priest stood up \
             with Urim and Thummim' — acknowledging the device was absent from the \
             Second Temple.",
        ),
        (
            "The Scholarly Consensus: A Binary Oracle",
            "Modern scholarship (van Dam, 1997) generally understands the Urim and \
             Thummim as a binary lot-oracle, capable of three states: affirmation, \
             negation, and silence.  This reading is supported by the LXX text of \
             1 Samuel 14:41, which presents the clearest ancient evidence for a \
             two-object system yielding yes/no answers.  The three-outcome model \
             (including divine silence, as in 1 Samuel 28:6) is widely accepted.\n\n\
             Cornelius van Dam's monograph, The Urim and Thummim: A Means of Revelation \
             in Ancient Israel (Eisenbrauns, 1997), remains the most comprehensive \
             modern study.  Van Dam argues that the device functioned as a direct \
             divine communication channel, not merely a random lot, because the High \
             Priest's approaching the LORD was itself a theologically significant act.",
        ),
        (
            "The Illuminating Stones Theory",
            "Some ancient sources and mediaeval commentators suggest a more elaborate \
             mechanism: the gemstones on the breastplate would illuminate sequentially \
             to spell out answers letter by letter.  Josephus (Antiquities 3.8.9) \
             records that the stones 'shone out' before victories in battle — a sign \
             of divine assent.  The Talmud (Yoma 73b) discusses the phenomenon in the \
             context of the breastplate glowing in response to priestly inquiry.  \
             Maimonides (Mishneh Torah, Hilchot Klei HaMikdash 10) treats the device \
             as part of the Kohen Gadol's required vestments without specifying \
             mechanism, focusing instead on its theological necessity.",
        ),
        (
            "Occult and Esoteric Reception",
            "Cornelius Agrippa (Three Books of Occult Philosophy, Book III, c. 1531) \
             treated the Choshen Mishpat as a mantic device of the highest order, \
             situating it within his broader theory of divine and angelic \
             communication through symbolic media.  The breastplate's twelve stones \
             were connected in Agrippan numerology to the twelve tribes, the twelve \
             signs of the Mazzaroth (zodiac), and the angelic guardians of the \
             Israelite camp.  This tradition influenced later Kabbalistic writers and \
             Rosicrucian interpretations of the priestly vestments as a living \
             cosmological map.",
        ),
        (
            "Absence from the Second Temple",
            "A remarkable feature of the Second Temple period was the acknowledged \
             absence of the Urim and Thummim.  Ezra 2:63 and Nehemiah 7:65 both \
             describe priestly lineage disputes in which the community resolved to \
             wait until a future priest could consult the Urim and Thummim — \
             implying the device was lost or inactive after the Babylonian destruction \
             of Solomon's Temple.  The Talmud lists the Urim and Thummim among the \
             five things present in the First Temple but absent from the Second \
             (Yoma 21b).  This absence lent the device an eschatological dimension: \
             its restoration was associated with messianic expectation.",
        ),
        (
            "The Name: Etymological Notes",
            "Urim (אוּרִים) is generally derived from the Hebrew root אוֹר (or, \
             'light'), giving the translation 'Lights' or 'the illuminating ones.'  \
             Thummim (תּוּמִים) derives from תָּם (tam, 'complete, whole, perfect'), \
             yielding 'Perfections' or 'Completions.'  The pairing Light / Perfection \
             maps naturally onto yes / no in a binary oracle: the Lights affirm, the \
             Perfections complete and close.  Some scholars (following Driver, 1913) \
             derive Thummim from a root meaning 'guilt' (עָוֹן), yielding the pair \
             Innocence / Guilt, but this reading is less commonly accepted in \
             contemporary scholarship.",
        ),
    ];

    for (title, body) in sections {
        println!("  {}", title.bright_yellow().bold());
        println!("  {}", "─".repeat(60).dimmed());
        for line in word_wrap(body, 70) {
            println!("  {}", line.white());
        }
        println!();
    }

    // Export
    handle_export(
        "urim_thummim_lore",
        || build_lore_text(sections),
        || {
            let body = build_lore_html(sections);
            wrap_html("Urim & Thummim — Historical Lore & Scholarship", &body, "angelic")
        },
    );
}

// ─── Reading history ──────────────────────────────────────────────────────────

fn view_history() {
    static HIST_ITEMS: &[MenuItem] = &[
        MenuItem {
            key: "1",
            icon: "📖",
            label: "My readings (by name)",
            hint: "Look up all oracle readings recorded for a specific name",
        },
        MenuItem {
            key: "2",
            icon: "🌐",
            label: "All readings (global)",
            hint: "Every oracle consultation across all users, newest first",
        },
    ];
    static HIST_MENU: Menu = Menu {
        title: "📖  URIM & THUMMIM — READING HISTORY",
        border_color: MenuColor::Yellow,
        items: HIST_ITEMS,
        back_key: "0",
        back_label: "Back",
    };

    loop {
        match HIST_MENU.show_and_read().as_str() {
            "1" => {
                print!("{}", "  ▸ Enter your name: ".bold().bright_yellow());
                io::stdout().flush().unwrap_or(());
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap_or(0);
                let name = buf.trim().to_string();
                if name.is_empty() {
                    continue;
                }
                match open_db() {
                    Err(e) => println!("  {}", format!("Database error: {e}").red()),
                    Ok(conn) => match get_or_create_user(&conn, &name) {
                        Err(e) => println!("  {}", format!("User error: {e}").red()),
                        Ok((user, _)) => match get_user_readings(&conn, &user.id) {
                            Err(e) => println!("  {}", format!("Query error: {e}").red()),
                            Ok(readings) => {
                                let urim_readings: Vec<_> = readings
                                    .iter()
                                    .filter(|r| r.tradition == "Urim & Thummim")
                                    .collect();
                                let title = format!("Oracle Readings for {}", user.name);
                                print_readings_table(&urim_readings, &title);
                            }
                        },
                    },
                }
            }
            "2" => match open_db() {
                Err(e) => println!("  {}", format!("Database error: {e}").red()),
                Ok(conn) => match get_all_readings(&conn) {
                    Err(e) => println!("  {}", format!("Query error: {e}").red()),
                    Ok(readings) => {
                        let urim_readings: Vec<_> = readings
                            .iter()
                            .filter(|r| r.tradition == "Urim & Thummim")
                            .collect();
                        print_readings_table(&urim_readings, "All Urim & Thummim Readings");
                    }
                },
            },
            "0" | "" => break,
            _ => {}
        }
    }
}

fn print_readings_table(
    readings: &[&crate::persistence::ReadingRecord],
    title: &str,
) {
    println!();
    println!(
        "  {}",
        format!("══  {}  ══", title).bright_yellow().bold()
    );
    println!(
        "  {}",
        format!("{} reading(s) found", readings.len()).dimmed()
    );
    println!();

    if readings.is_empty() {
        println!(
            "  {}",
            "No oracle readings recorded yet.  Cast the Oracle to begin.".dimmed()
        );
        println!();
        return;
    }

    for (i, rec) in readings.iter().enumerate() {
        println!(
            "  {}  {}  —  {}",
            format!("{:>3}.", i + 1).dimmed(),
            rec.drawn_at.bright_yellow(),
            rec.user_name.bright_white().bold()
        );
        // Show first two lines of the stored cards text
        for line in rec.cards.lines().take(3) {
            println!("       {}", line.trim().dimmed());
        }
        println!();
    }

    // Export
    let title_owned = title.to_string();
    let export_text: String = {
        let mut s = format!(
            "URIM & THUMMIM — {}\nGenerated: {}\n\n",
            title_owned,
            chrono_now()
        );
        for (i, rec) in readings.iter().enumerate() {
            s.push_str(&format!(
                "{}.  {}  —  {}\n{}\n\n",
                i + 1,
                rec.drawn_at,
                rec.user_name,
                rec.cards
            ));
        }
        s
    };
    let export_html: String = {
        let mut rows = String::new();
        for rec in readings.iter() {
            rows.push_str(&format!(
                r#"<tr>
                  <td class="sys">{user}</td>
                  <td>{date}</td>
                  <td class="meaning">{cards}</td>
                </tr>"#,
                user = html_esc(&rec.user_name),
                date = html_esc(&rec.drawn_at),
                cards = html_esc(&rec.cards).replace('\n', "<br/>"),
            ));
        }
        format!(
            r#"<h2 style="color:var(--accent);">{title}</h2>
           <table>
             <thead>
               <tr><th>Querent</th><th>Date</th><th>Reading</th></tr>
             </thead>
             <tbody>{rows}</tbody>
           </table>"#,
            title = html_esc(&title_owned),
            rows = rows,
        )
    };

    handle_export(
        "urim_history",
        || export_text.clone(),
        || wrap_html(&format!("Urim & Thummim — {}", title_owned), &export_html, "angelic"),
    );
}

// ─── RNG helpers ─────────────────────────────────────────────────────────────

fn oracle_rnd(rng: &Option<RdRand>) -> u64 {
    match rng {
        Some(r) => r.try_next_u64().unwrap_or_else(|_| os_u64()),
        None => os_u64(),
    }
}

fn os_u64() -> u64 {
    let mut bytes = [0u8; 8];
    getrandom::getrandom(&mut bytes).unwrap_or(());
    u64::from_le_bytes(bytes)
}

// ─── Text / HTML builders ─────────────────────────────────────────────────────

fn build_breastplate_text() -> String {
    let mut s = format!(
        "THE CHOSHEN MISHPAT — BREASTPLATE OF JUDGMENT\n\
         ==============================================\n\
         Exodus 28:15–30 · Josephus, Antiquities 3.7.5\n\
         Generated: {}\n\n",
        chrono_now()
    );
    for stone in &BREASTPLATE {
        s.push_str(&format!(
            "Stone {pos}  —  {hebrew}  ({stone_name})\n\
             Tribe: {tribe} (\"{meaning}\")\n\
             Row {row}, Column {col}  ·  Color: {color}\n\
             Scripture: {scripture}\n\
             Attributes:\n  {attrs}\n\n{sep}\n\n",
            pos = stone.position,
            hebrew = stone.hebrew_name,
            stone_name = stone.stone_name,
            tribe = stone.tribe,
            meaning = stone.tribe_meaning,
            row = stone.row,
            col = stone.col,
            color = stone.color,
            scripture = stone.scripture,
            attrs = stone.attributes,
            sep = "─".repeat(60),
        ));
    }
    s
}

fn build_breastplate_html() -> String {
    let mut rows = String::new();
    for s in &BREASTPLATE {
        rows.push_str(&format!(
            r#"<tr>
              <td class="num">{pos}</td>
              <td class="sys">{hebrew}</td>
              <td class="sys">{stone}</td>
              <td class="meaning">{tribe} <small>"{meaning}"</small></td>
              <td>{color}</td>
              <td class="meaning">{attrs}</td>
            </tr>"#,
            pos = s.position,
            hebrew = html_esc(s.hebrew_name),
            stone = html_esc(s.stone_name),
            tribe = html_esc(s.tribe),
            meaning = html_esc(s.tribe_meaning),
            color = html_esc(s.color),
            attrs = html_esc(s.attributes),
        ));
    }
    format!(
        r#"<h2 style="color:var(--accent);">The Choshen Mishpat — Breastplate of Judgment</h2>
       <p class="meta">Exodus 28:15–30 · Josephus, Antiquities 3.7.5 · Maimonides, Mishneh Torah 9</p>
       <table>
         <thead>
           <tr>
             <th>#</th><th>Hebrew Name</th><th>Stone</th>
             <th>Tribe</th><th>Color</th><th>Attributes</th>
           </tr>
         </thead>
         <tbody>{rows}</tbody>
       </table>"#,
        rows = rows,
    )
}

fn build_reading_text(
    querent: &str,
    question: &str,
    outcome_label: &str,
    stone: &BreastplateStone,
    utterance: &str,
    timestamp: &str,
) -> String {
    format!(
        "URIM & THUMMIM — ORACLE READING\n\
         ================================\n\
         Querent:  {querent}\n\
         Date:     {timestamp}\n\n\
         Question:\n  {question}\n\n\
         Illuminated Stone:\n\
           {hebrew}  ({stone_name})  —  Tribe of {tribe} (\"{tribe_meaning}\")\n\n\
         Outcome:  {outcome}\n\n\
         Oracle:\n  {utterance}\n\n\
         Source: Numbers 27:21 · Cornelius van Dam, The Urim and Thummim (1997)\n",
        querent = querent,
        timestamp = timestamp,
        question = question,
        hebrew = stone.hebrew_name,
        stone_name = stone.stone_name,
        tribe = stone.tribe,
        tribe_meaning = stone.tribe_meaning,
        outcome = outcome_label,
        utterance = utterance,
    )
}

fn build_reading_html(
    querent: &str,
    question: &str,
    outcome_label: &str,
    stone: &BreastplateStone,
    utterance: &str,
    timestamp: &str,
) -> String {
    format!(
        r#"<h2 style="color:var(--accent);margin-bottom:6pt;">Urim &amp; Thummim — Oracle Reading</h2>
       <p class="meta">{timestamp} · Querent: {querent}</p>
       <table>
         <tbody>
           <tr><td class="sys">Question</td>
               <td class="meaning" style="font-style:italic;">{question}</td></tr>
           <tr><td class="sys">Illuminated Stone</td>
               <td>{hebrew} ({stone_name}) — Tribe of {tribe} "{tribe_meaning}"</td></tr>
           <tr><td class="sys">Outcome</td>
               <td class="meaning" style="font-weight:bold;color:var(--accent);">{outcome}</td></tr>
           <tr><td class="sys">Oracle</td>
               <td class="meaning" style="font-style:italic;">{utterance}</td></tr>
           <tr><td class="sys">Source</td>
               <td>Numbers 27:21 · Cornelius van Dam, <em>The Urim and Thummim</em> (1997)</td></tr>
         </tbody>
       </table>"#,
        timestamp = html_esc(timestamp),
        querent = html_esc(querent),
        question = html_esc(question),
        hebrew = html_esc(stone.hebrew_name),
        stone_name = html_esc(stone.stone_name),
        tribe = html_esc(stone.tribe),
        tribe_meaning = html_esc(stone.tribe_meaning),
        outcome = html_esc(outcome_label),
        utterance = html_esc(utterance),
    )
}

fn build_lore_text(sections: &[(&str, &str)]) -> String {
    let mut s = format!(
        "URIM & THUMMIM — HISTORICAL LORE & SCHOLARSHIP\n\
         ===============================================\n\
         Generated: {}\n\n",
        chrono_now()
    );
    for (title, body) in sections {
        s.push_str(&format!(
            "{}\n{}\n\n{}\n\n{}\n\n",
            title,
            "─".repeat(title.len()),
            body,
            "─".repeat(60),
        ));
    }
    s
}

fn build_lore_html(sections: &[(&str, &str)]) -> String {
    let mut body = String::new();
    for (title, text) in sections {
        let paragraphs = text
            .split("\n\n")
            .map(|p| {
                format!(
                    "<p class=\"meaning\">{}</p>",
                    html_esc(p).replace('\n', "<br/>")
                )
            })
            .collect::<Vec<_>>()
            .join("");
        body.push_str(&format!(
            r#"<h3 style="color:var(--accent);margin-top:12pt;">{title}</h3>
           {paragraphs}"#,
            title = html_esc(title),
            paragraphs = paragraphs,
        ));
    }
    body
}

// ─── Utility helpers ──────────────────────────────────────────────────────────

fn word_wrap(s: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in s.split('\n') {
        let mut current = String::new();
        for word in paragraph.split_whitespace() {
            if current.is_empty() {
                current.push_str(word);
            } else if current.len() + 1 + word.len() <= width {
                current.push(' ');
                current.push_str(word);
            } else {
                lines.push(current.clone());
                current = word.to_string();
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
        // Preserve blank paragraph separators
        if paragraph.trim().is_empty() && !lines.last().map(|l: &String| l.is_empty()).unwrap_or(false) {
            lines.push(String::new());
        }
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

fn html_esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
