//! Interactive runic traditions session — all terminal I/O for the Runes sub-menu.
//!
//! Functions:
//!  - [`run_runes_session`]       — top-level sub-menu loop
//!  - `browse_tradition`          — display all runes in a tradition table
//!  - `lookup_rune`               — look up a single rune by name, number, or glyph
//!  - `draw_reading`              — TRNG-based rune draw (1, 3, or 9 runes)
//!  - `show_aettir`               — display the three Elder Futhark aettir

use colored::*;
use std::io::{self, Write};

use rdrand::RdRand;

use crate::export::{handle_export, wrap_html};
use crate::menu::{Menu, MenuColor, MenuItem};
use crate::persistence::{get_or_create_user, open_db, record_reading};
use crate::reports::chrono_now;

use super::anglo_saxon::ANGLO_SAXON;
use super::armanen::ARMANEN;
use super::elder_futhark::ELDER_FUTHARK;
use super::younger_futhark::YOUNGER_FUTHARK;
use super::Rune;

// ─── Sub-menu definition ──────────────────────────────────────────────────────

static RUNES_ITEMS: &[MenuItem] = &[
    MenuItem {
        key: "1",
        icon: "ᚠ",
        label: "Elder Futhark  (24 runes, c. 150–800 CE)",
        hint: "Three aettir · Full rune poem citations · Esoteric meanings",
    },
    MenuItem {
        key: "2",
        icon: "ᚢ",
        label: "Younger Futhark  (16 runes, c. 750–1100 CE)",
        hint: "Viking Age · Long-branch & short-twig variants",
    },
    MenuItem {
        key: "3",
        icon: "ᚦ",
        label: "Anglo-Saxon Futhorc  (33 runes, c. 5th–11th c.)",
        hint: "28 core + 5 Northumbrian extensions · OE Rune Poem",
    },
    MenuItem {
        key: "4",
        icon: "ᚷ",
        label: "Armanen Runes  (18 runes, 1908 CE — modern esoteric)",
        hint: "Guido von List · Hávamál attributions · Historical warnings",
    },
    MenuItem {
        key: "5",
        icon: "ᛉ",
        label: "Elder Futhark — Browse by Aett",
        hint: "Freyr's · Hagal's · Tyr's — three families of eight",
    },
    MenuItem {
        key: "6",
        icon: "ᛟ",
        label: "Look Up a Rune",
        hint: "Search by name, number, or glyph — across all traditions",
    },
    MenuItem {
        key: "7",
        icon: "✦",
        label: "Draw a Rune Reading",
        hint: "1-rune · 3-rune (Nornir) · 9-rune cast · TRNG hardware randomness",
    },
    MenuItem {
        key: "8",
        icon: "📖",
        label: "View Reading History",
        hint: "Browse all recorded rune readings by user or globally",
    },
];

static RUNES_MENU: Menu = Menu {
    title: "ᚠ  RUNIC TRADITIONS  ᛟ",
    border_color: MenuColor::Cyan,
    items: RUNES_ITEMS,
    back_key: "0",
    back_label: "Back to main menu",
};

// ─── Entry point ──────────────────────────────────────────────────────────────

/// Run the runic traditions sub-menu loop.
pub fn run_runes_session() {
    loop {
        let choice = RUNES_MENU.show_and_read();
        match choice.trim() {
            "1" => tradition_session(ELDER_FUTHARK, "Elder Futhark", "runic"),
            "2" => tradition_session(YOUNGER_FUTHARK, "Younger Futhark", "runic"),
            "3" => tradition_session(ANGLO_SAXON, "Anglo-Saxon Futhorc", "runic"),
            "4" => tradition_session(ARMANEN, "Armanen Runes", "runic"),
            "5" => show_aettir(),
            "6" => lookup_rune(),
            "7" => draw_reading(),
            "8" => view_rune_history(),
            "0" | "" => break,
            _ => {}
        }
    }
}

// ─── Tradition session ────────────────────────────────────────────────────────

/// Interactive loop for a single tradition: browse, look up, or export.
fn tradition_session(runes: &[Rune], tradition: &str, theme: &str) {
    static SUB_ITEMS: &[MenuItem] = &[
        MenuItem {
            key: "1",
            icon: "▤",
            label: "Browse all runes (table)",
            hint: "",
        },
        MenuItem {
            key: "2",
            icon: "◎",
            label: "Look up by name or number",
            hint: "",
        },
        MenuItem {
            key: "3",
            icon: "↓",
            label: "Export full tradition",
            hint: "Text · HTML · PDF",
        },
    ];
    static SUB_MENU: Menu = Menu {
        title: "Tradition",
        border_color: MenuColor::Cyan,
        items: SUB_ITEMS,
        back_key: "0",
        back_label: "Back",
    };
    let sub_menu = &SUB_MENU;

    loop {
        let ch = sub_menu.show_and_read();
        match ch.trim() {
            "1" => browse_tradition(runes, tradition, theme),
            "2" => lookup_in_tradition(runes, tradition),
            "3" => {
                let stem = tradition.to_lowercase().replace(' ', "_");
                let trad = tradition.to_string();
                handle_export(
                    &stem,
                    || build_tradition_text(runes, &trad),
                    || {
                        let body = build_tradition_html_body(runes);
                        wrap_html(&trad, &body, "runic")
                    },
                );
            }
            "0" | "" => break,
            _ => {}
        }
    }
}

// ─── Browse ───────────────────────────────────────────────────────────────────

fn browse_tradition(runes: &[Rune], tradition: &str, _theme: &str) {
    println!();
    println!(
        "{}",
        format!("  ══  {}  ══", tradition)
            .bright_cyan()
            .bold()
    );
    println!(
        "  {}",
        format!("{} runes", runes.len()).dimmed()
    );
    println!();

    // Print header
    println!(
        "  {:<4} {:<6} {:<4} {:<24} {}",
        "#".bold(),
        "Glyph".bold(),
        "Ph.".bold(),
        "Name".bold(),
        "Upright meaning".bold(),
    );
    println!("  {}", "─".repeat(80).dimmed());

    for r in runes {
        println!(
            "  {:<4} {:<6} {:<4} {:<24} {}",
            r.number.to_string().bright_yellow(),
            r.glyph.bright_white().bold(),
            r.phoneme.chars().take(4).collect::<String>().dimmed(),
            r.name.cyan(),
            truncate(r.meaning_upright, 40).white(),
        );
    }
    println!();

    // Export prompt
    let stem = format!("{}_browse", tradition.to_lowercase().replace(' ', "_"));
    let trad = tradition.to_string();
    handle_export(
        &stem,
        || build_tradition_text(runes, &trad),
        || {
            let body = build_tradition_html_body(runes);
            wrap_html(&trad, &body, "runic")
        },
    );
}

// ─── Lookup ───────────────────────────────────────────────────────────────────

/// Cross-tradition look up: searches all four traditions.
fn lookup_rune() {
    println!();
    println!(
        "{}",
        "  ══  RUNE LOOKUP — all traditions  ══".bright_cyan().bold()
    );
    println!(
        "{}",
        "  Enter a name, number, or glyph character.  Empty line to return.\n".dimmed()
    );

    loop {
        print!("{}", "  ▸ Query: ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() || line.trim().is_empty() {
            return;
        }
        let q = line.trim().to_lowercase();

        // Search in order: EF → YF → AS → Armanen
        let sources: &[(&[Rune], &str)] = &[
            (ELDER_FUTHARK, "Elder Futhark"),
            (YOUNGER_FUTHARK, "Younger Futhark"),
            (ANGLO_SAXON, "Anglo-Saxon Futhorc"),
            (ARMANEN, "Armanen Runes"),
        ];

        let mut found = false;
        for (tradition, label) in sources {
            for r in *tradition {
                if rune_matches(r, &q) {
                    println!();
                    println!(
                        "  {}  —  {}",
                        label.bright_cyan().bold(),
                        r.glyph.bright_white().bold()
                    );
                    print_rune_detail(r);
                    found = true;

                    let stem = format!(
                        "rune_{}",
                        r.name.to_lowercase().replace(' ', "_")
                    );
                    let rune_ref = r;
                    let label_owned = label.to_string();
                    handle_export(
                        &stem,
                        || build_single_rune_text(rune_ref, &label_owned),
                        || {
                            let body = build_single_rune_html(rune_ref, &label_owned);
                            wrap_html(
                                &format!("{} — {}", rune_ref.name, label_owned),
                                &body,
                                "runic",
                            )
                        },
                    );
                }
            }
        }
        if !found {
            println!(
                "  {}",
                "  No rune found matching that query.".yellow()
            );
        }
        println!();
    }
}

/// Look up within a specific tradition.
fn lookup_in_tradition(runes: &[Rune], tradition: &str) {
    println!();
    println!(
        "{}",
        format!("  ══  Lookup in {}  ══", tradition)
            .bright_cyan()
            .bold()
    );
    println!(
        "{}",
        "  Enter name, number (1–N), or rune glyph. Empty line to return.\n".dimmed()
    );

    loop {
        print!("{}", "  ▸ Query: ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() || line.trim().is_empty() {
            return;
        }
        let q = line.trim().to_lowercase();

        let mut found = false;
        for r in runes {
            if rune_matches(r, &q) {
                println!();
                print_rune_detail(r);
                found = true;

                let stem = format!("rune_{}", r.name.to_lowercase().replace(' ', "_"));
                let trad = tradition.to_string();
                let rune_ref = r;
                handle_export(
                    &stem,
                    || build_single_rune_text(rune_ref, &trad),
                    || {
                        let body = build_single_rune_html(rune_ref, &trad);
                        wrap_html(
                            &format!("{} — {}", rune_ref.name, trad),
                            &body,
                            "runic",
                        )
                    },
                );
            }
        }
        if !found {
            println!(
                "  {}",
                "  No rune matches that query in this tradition.".yellow()
            );
        }
        println!();
    }
}

fn rune_matches(r: &Rune, q: &str) -> bool {
    r.name.to_lowercase().contains(q)
        || r.glyph.contains(q)
        || r.alt_names.to_lowercase().contains(q)
        || r.number.to_string() == q
}

// ─── Aettir display ───────────────────────────────────────────────────────────

fn show_aettir() {
    println!();
    println!(
        "{}",
        "  ══  ELDER FUTHARK — THREE AETTIR  ══".bright_cyan().bold()
    );
    println!(
        "{}",
        "  The 24 runes in three families (aettir) of eight.\n".dimmed()
    );

    let aett_names = ["Freyr's Aett", "Hagal's Aett", "Tyr's Aett"];
    let colors: [fn(&str) -> ColoredString; 3] = [
        |s| s.bright_yellow(),
        |s| s.bright_cyan(),
        |s| s.bright_magenta(),
    ];

    for aett_num in 1u8..=3 {
        let name = aett_names[(aett_num - 1) as usize];
        println!("  {}", name.bold());
        println!("  {}", "─".repeat(60).dimmed());

        let aett_runes: Vec<&Rune> = ELDER_FUTHARK
            .iter()
            .filter(|r| r.aett == aett_num)
            .collect();

        // Print glyph row
        let glyphs: String = aett_runes.iter().map(|r| format!(" {}  ", r.glyph)).collect();
        println!("  {}", colors[(aett_num - 1) as usize](&glyphs).bold());

        // Print name row
        for r in &aett_runes {
            let col = colors[(aett_num - 1) as usize];
            print!("  {:<10}", col(r.name));
        }
        println!();

        // Print phoneme row
        for r in &aett_runes {
            print!("  /{:<9}/", r.phoneme.chars().take(8).collect::<String>().dimmed());
        }
        println!("\n");
    }

    // Export
    handle_export(
        "elder_futhark_aettir",
        || build_aettir_text(),
        || {
            let body = build_aettir_html();
            wrap_html("Elder Futhark — Three Aettir", &body, "runic")
        },
    );
}

// ─── Draw a reading ───────────────────────────────────────────────────────────

fn draw_reading() {
    println!();
    println!(
        "{}",
        "  ══  DRAW A RUNE READING  ══".bright_cyan().bold()
    );

    static DRAW_ITEMS: &[MenuItem] = &[
        MenuItem {
            key: "1",
            icon: "◈",
            label: "Single rune — one insight",
            hint: "A single guiding rune drawn from the Elder Futhark",
        },
        MenuItem {
            key: "2",
            icon: "◈◈◈",
            label: "Three runes — the Norns  (Urðr · Verðandi · Skuld)",
            hint: "Past · Present · Future",
        },
        MenuItem {
            key: "3",
            icon: "✦✦✦",
            label: "Nine-rune cast — the Well of Urðr",
            hint: "Full cast across three registers: past · present · future",
        },
    ];
    let draw_menu = Menu {
        title: "Draw Type",
        border_color: MenuColor::Cyan,
        items: DRAW_ITEMS,
        back_key: "0",
        back_label: "Back",
    };

    let ch = draw_menu.show_and_read();
    let count = match ch.trim() {
        "1" => 1usize,
        "2" => 3,
        "3" => 9,
        _ => return,
    };

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

    // Draw from Elder Futhark (24 runes)
    let pool = ELDER_FUTHARK;
    let mut drawn: Vec<(&Rune, bool)> = Vec::with_capacity(count);
    let mut used: Vec<usize> = Vec::new();

    while drawn.len() < count {
        let raw: u64 = rune_rnd(&rng);
        let idx = (raw % pool.len() as u64) as usize;
        if used.contains(&idx) {
            continue;
        }
        used.push(idx);
        let raw2: u64 = rune_rnd(&rng);
        let reversed = raw2 & 1 != 0; // use LSB for orientation
        drawn.push((&pool[idx], reversed));
    }

    println!();
    let spread_name = match count {
        1 => "Single Rune",
        3 => "The Norns — Past · Present · Future",
        9 => "Nine-Rune Well of Urðr Cast",
        _ => "Reading",
    };
    println!(
        "  {}",
        format!("✦  {}  ✦", spread_name).bright_cyan().bold()
    );
    println!("  {}", chrono_now().dimmed());
    println!();

    let norn_labels = ["Past (Urðr)", "Present (Verðandi)", "Future (Skuld)"];
    let nine_labels = [
        "1 — Root cause",
        "2 — The past",
        "3 — Near future",
        "4 — Foundation",
        "5 — The present",
        "6 — The path",
        "7 — Hopes/fears",
        "8 — Environment",
        "9 — Outcome",
    ];

    for (i, (rune, reversed)) in drawn.iter().enumerate() {
        let pos_label = if count == 3 {
            norn_labels.get(i).copied().unwrap_or("")
        } else if count == 9 {
            nine_labels.get(i).copied().unwrap_or("")
        } else {
            ""
        };

        let orientation = if *reversed { " ↓ (reversed)" } else { "" };

        println!(
            "  {}  {}{}",
            rune.glyph.bright_white().bold(),
            rune.name.bright_yellow().bold(),
            orientation.dimmed(),
        );
        if !pos_label.is_empty() {
            println!("  {}", format!("  Position: {}", pos_label).dimmed());
        }
        if *reversed && !rune.meaning_reversed.is_empty()
            && !rune.meaning_reversed.starts_with('(')
        {
            println!(
                "  {}  {}",
                "  Reversed:".bright_red(),
                rune.meaning_reversed.white()
            );
        } else {
            println!(
                "  {}  {}",
                "  Meaning: ".bright_green(),
                truncate(rune.meaning_upright, 70).white()
            );
        }
        if count == 1 {
            println!("  {}", "  Esoteric:".bright_cyan());
            for line in word_wrap(rune.esoteric, 72) {
                println!("    {}", line.dimmed());
            }
        }
        println!();
    }

    // Persist the reading
    {
        let cards_text = drawn
            .iter()
            .map(|(r, rev)| {
                if *rev && !r.meaning_reversed.is_empty() && !r.meaning_reversed.starts_with('(') {
                    format!("{} {} (reversed)", r.glyph, r.name)
                } else {
                    format!("{} {}", r.glyph, r.name)
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        if let Ok(conn) = open_db() {
            if let Ok((user, _)) = get_or_create_user(&conn, &querent) {
                record_reading(&conn, &user.id, "Elder Futhark Runes", spread_name, &cards_text)
                    .ok();
            }
        }
    }

    // Export
    let text_snap = {
        let mut s = format!("RUNE READING — {}\n{}\n\n", spread_name, chrono_now());
        for (i, (rune, reversed)) in drawn.iter().enumerate() {
            let pos = if count == 3 {
                norn_labels.get(i).copied().unwrap_or("")
            } else if count == 9 {
                nine_labels.get(i).copied().unwrap_or("")
            } else {
                ""
            };
            s.push_str(&format!(
                "{} {}  {}{}\n  {}\n\n",
                rune.glyph,
                rune.name,
                if !pos.is_empty() {
                    format!("[{}]  ", pos)
                } else {
                    String::new()
                },
                if *reversed { "(reversed)" } else { "" },
                if *reversed && !rune.meaning_reversed.is_empty()
                    && !rune.meaning_reversed.starts_with('(')
                {
                    rune.meaning_reversed
                } else {
                    rune.meaning_upright
                },
            ));
        }
        s
    };

    let html_snap = {
        let mut rows = String::new();
        for (i, (rune, reversed)) in drawn.iter().enumerate() {
            let pos = if count == 3 {
                norn_labels.get(i).copied().unwrap_or("")
            } else if count == 9 {
                nine_labels.get(i).copied().unwrap_or("")
            } else {
                ""
            };
            let meaning = if *reversed
                && !rune.meaning_reversed.is_empty()
                && !rune.meaning_reversed.starts_with('(')
            {
                rune.meaning_reversed
            } else {
                rune.meaning_upright
            };
            rows.push_str(&format!(
                r#"<tr>
                  <td class="sys">{glyph} {name}</td>
                  <td>{pos}</td>
                  <td>{orient}</td>
                  <td class="meaning">{meaning}</td>
                </tr>"#,
                glyph = html_esc(rune.glyph),
                name = html_esc(rune.name),
                pos = html_esc(pos),
                orient = if *reversed { "Reversed" } else { "Upright" },
                meaning = html_esc(meaning),
            ));
        }
        format!(
            r#"<h2 style="color:var(--accent);margin-bottom:6pt;">{}</h2>
           <p class="meta">{}</p>
           <table>
             <thead>
               <tr>
                 <th>Rune</th><th>Position</th><th>Orientation</th><th>Meaning</th>
               </tr>
             </thead>
             <tbody>{rows}</tbody>
           </table>"#,
            html_esc(spread_name),
            html_esc(&chrono_now()),
            rows = rows,
        )
    };

    handle_export(
        &format!("rune_reading_{}", spread_name.to_lowercase().replace(' ', "_").replace('·', "").replace("—", "").replace("__", "_")),
        || text_snap.clone(),
        || wrap_html(&format!("Rune Reading — {}", spread_name), &html_snap, "runic"),
    );
}

// ─── Detail display ───────────────────────────────────────────────────────────

fn print_rune_detail(r: &Rune) {
    let divider = "─".repeat(60).dimmed();
    println!("  {}", divider);
    println!(
        "  {}  {}  {}",
        r.glyph.bright_white().bold(),
        r.name.bright_yellow().bold(),
        format!("({})", r.phoneme).dimmed(),
    );
    if !r.alt_names.is_empty() {
        println!("  {}", r.alt_names.dimmed());
    }
    if r.aett > 0 {
        println!(
            "  {} — rune {} of {}",
            r.aett_name.cyan(),
            r.number,
            r.aett_name.dimmed(),
        );
    }
    println!();

    field("Etymology", r.etymology);
    field("Deity", r.deity);
    field("Element", r.element);
    field("World", r.world);
    println!();

    if !r.rune_poem_oe.is_empty() {
        field_wrapped("OE Rune Poem", r.rune_poem_oe);
    }
    if !r.rune_poem_on.is_empty() {
        field_wrapped("ON Rune Poem", r.rune_poem_on);
    }
    if !r.rune_poem_oi.is_empty() {
        field_wrapped("OI Rune Poem", r.rune_poem_oi);
    }
    if !r.rune_poem_oe.is_empty() || !r.rune_poem_on.is_empty() || !r.rune_poem_oi.is_empty() {
        println!();
    }

    field_wrapped("Esoteric", r.esoteric);
    field_wrapped("Upright", r.meaning_upright);
    if !r.meaning_reversed.is_empty() {
        field_wrapped("Reversed", r.meaning_reversed);
    }
    println!("  {}", divider);
}

fn field(label: &str, value: &str) {
    if value.is_empty() {
        return;
    }
    println!("  {:<12} {}", format!("{}:", label).bright_cyan(), value.white());
}

fn field_wrapped(label: &str, value: &str) {
    if value.is_empty() {
        return;
    }
    let indent = "              "; // 14 chars to align with "  label:      "
    let lines = word_wrap(value, 64);
    for (i, l) in lines.iter().enumerate() {
        if i == 0 {
            println!(
                "  {:<12} {}",
                format!("{}:", label).bright_cyan(),
                l.white()
            );
        } else {
            println!("{}{}", indent, l.dimmed());
        }
    }
}

// ─── Text builders (for export) ──────────────────────────────────────────────

fn build_tradition_text(runes: &[Rune], tradition: &str) -> String {
    let mut out = format!(
        "{}\n{}\n\nGenerated: {}\n\n",
        tradition,
        "=".repeat(tradition.len()),
        chrono_now()
    );
    for r in runes {
        out.push_str(&build_single_rune_text(r, tradition));
        out.push('\n');
        out.push_str(&"─".repeat(60));
        out.push('\n');
    }
    out
}

fn build_single_rune_text(r: &Rune, tradition: &str) -> String {
    let mut s = format!(
        "{} {}  ({})\nTradition: {}\n",
        r.glyph, r.name, r.phoneme, tradition
    );
    if !r.alt_names.is_empty() {
        s.push_str(&format!("Also: {}\n", r.alt_names));
    }
    if r.aett > 0 {
        s.push_str(&format!("Aett: {} (rune {})\n", r.aett_name, r.number));
    }
    s.push_str(&format!("\nEtymology: {}\n", r.etymology));
    s.push_str(&format!("Deity: {}\nElement: {}\nWorld: {}\n", r.deity, r.element, r.world));
    if !r.rune_poem_oe.is_empty() {
        s.push_str(&format!("\n{}\n", r.rune_poem_oe));
    }
    if !r.rune_poem_on.is_empty() {
        s.push_str(&format!("\n{}\n", r.rune_poem_on));
    }
    if !r.rune_poem_oi.is_empty() {
        s.push_str(&format!("\n{}\n", r.rune_poem_oi));
    }
    s.push_str(&format!("\nEsoteric meaning:\n{}\n", r.esoteric));
    s.push_str(&format!("\nUpright: {}\n", r.meaning_upright));
    if !r.meaning_reversed.is_empty() {
        s.push_str(&format!("Reversed: {}\n", r.meaning_reversed));
    }
    s
}

fn build_aettir_text() -> String {
    let mut s = format!(
        "ELDER FUTHARK — THREE AETTIR\n{}\nGenerated: {}\n\n",
        "=".repeat(30),
        chrono_now()
    );
    for aett in 1u8..=3 {
        let name = ["Freyr's Aett", "Hagal's Aett", "Tyr's Aett"][(aett - 1) as usize];
        s.push_str(&format!("{}\n{}\n\n", name, "-".repeat(name.len())));
        for r in ELDER_FUTHARK.iter().filter(|r| r.aett == aett) {
            s.push_str(&format!("  {} {}  /{}/\n", r.glyph, r.name, r.phoneme));
            s.push_str(&format!("    {}\n\n", r.meaning_upright));
        }
    }
    s
}

// ─── HTML builders (for export) ───────────────────────────────────────────────

fn build_tradition_html_body(runes: &[Rune]) -> String {
    let mut rows = String::new();
    for r in runes {
        let poems = [r.rune_poem_oe, r.rune_poem_on, r.rune_poem_oi]
            .iter()
            .filter(|p| !p.is_empty())
            .map(|p| {
                format!(
                    "<div style=\"font-size:7.5pt;font-style:italic;margin-top:2pt;\">{}</div>",
                    html_esc(p)
                )
            })
            .collect::<Vec<_>>()
            .join("");

        rows.push_str(&format!(
            r#"<tr>
              <td class="num">{num}</td>
              <td class="sys" style="font-size:18pt;">{glyph}</td>
              <td class="sys">{name}<div style="font-size:7.5pt;color:var(--calc-color);">{alt}</div></td>
              <td><code>/{ph}/</code></td>
              <td class="meaning">{meaning}{poems}</td>
            </tr>"#,
            num = r.number,
            glyph = html_esc(r.glyph),
            name = html_esc(r.name),
            alt = html_esc(r.alt_names),
            ph = html_esc(r.phoneme),
            meaning = html_esc(r.meaning_upright),
            poems = poems,
        ));
    }

    format!(
        r#"<table>
      <thead>
        <tr>
          <th class="num">#</th>
          <th>Glyph</th>
          <th>Name</th>
          <th>Phoneme</th>
          <th>Upright Meaning &amp; Rune Poems</th>
        </tr>
      </thead>
      <tbody>{rows}</tbody>
    </table>"#,
        rows = rows,
    )
}

fn build_single_rune_html(r: &Rune, tradition: &str) -> String {
    let poems: String = [
        ("Old English Rune Poem", r.rune_poem_oe),
        ("Old Norwegian Rune Poem", r.rune_poem_on),
        ("Old Icelandic Rune Poem", r.rune_poem_oi),
    ]
    .iter()
    .filter(|(_, v)| !v.is_empty())
    .map(|(label, text)| {
        format!(
            r#"<tr><td class="sys">{}</td><td class="meaning" style="font-style:italic;">{}</td></tr>"#,
            html_esc(label),
            html_esc(text)
        )
    })
    .collect();

    let aett_row = if r.aett > 0 {
        format!(
            r#"<tr><td class="sys">Aett</td><td>{} (rune {})</td></tr>"#,
            html_esc(r.aett_name),
            r.number
        )
    } else {
        String::new()
    };

    format!(
        r#"<div style="font-size:48pt;color:var(--accent);margin-bottom:6pt;">{glyph}</div>
       <h2 style="color:var(--accent);">{name} <small style="font-size:14pt;">/{ph}/</small></h2>
       <p style="color:var(--calc-color);font-size:8.5pt;">{alt}</p>
       <table>
         <tbody>
           <tr><td class="sys">Tradition</td><td>{tradition}</td></tr>
           {aett}
           <tr><td class="sys">Etymology</td><td class="meaning">{etym}</td></tr>
           <tr><td class="sys">Deity</td><td>{deity}</td></tr>
           <tr><td class="sys">Element</td><td>{element}</td></tr>
           <tr><td class="sys">World</td><td>{world}</td></tr>
           {poems}
           <tr><td class="sys">Esoteric</td><td class="meaning">{esoteric}</td></tr>
           <tr><td class="sys">Upright</td><td class="meaning">{upright}</td></tr>
           <tr><td class="sys">Reversed</td><td class="meaning">{reversed}</td></tr>
         </tbody>
       </table>"#,
        glyph = html_esc(r.glyph),
        name = html_esc(r.name),
        ph = html_esc(r.phoneme),
        alt = html_esc(r.alt_names),
        tradition = html_esc(tradition),
        aett = aett_row,
        etym = html_esc(r.etymology),
        deity = html_esc(r.deity),
        element = html_esc(r.element),
        world = html_esc(r.world),
        poems = poems,
        esoteric = html_esc(r.esoteric),
        upright = html_esc(r.meaning_upright),
        reversed = html_esc(r.meaning_reversed),
    )
}

fn build_aettir_html() -> String {
    let colors = ["#D4AF37", "#4A9ECC", "#C87EC8"];
    let mut body = String::new();

    for aett in 1u8..=3 {
        let name = ["Freyr's Aett", "Hagal's Aett", "Tyr's Aett"][(aett - 1) as usize];
        let color = colors[(aett - 1) as usize];

        let mut rows = String::new();
        for r in ELDER_FUTHARK.iter().filter(|r| r.aett == aett) {
            rows.push_str(&format!(
                r#"<tr>
                  <td style="font-size:24pt;text-align:center;color:{color};">{glyph}</td>
                  <td class="sys">{name}</td>
                  <td><code>/{ph}/</code></td>
                  <td class="meaning">{meaning}</td>
                </tr>"#,
                color = color,
                glyph = html_esc(r.glyph),
                name = html_esc(r.name),
                ph = html_esc(r.phoneme),
                meaning = html_esc(r.meaning_upright),
            ));
        }
        body.push_str(&format!(
            r#"<h2 style="color:{color};margin-top:12pt;">{name}</h2>
           <table>
             <thead>
               <tr><th>Glyph</th><th>Name</th><th>Phoneme</th><th>Upright Meaning</th></tr>
             </thead>
             <tbody>{rows}</tbody>
           </table>"#,
            color = color,
            name = html_esc(name),
            rows = rows,
        ));
    }
    body
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let t: String = s.chars().take(max - 1).collect();
        format!("{}…", t)
    }
}

fn word_wrap(s: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in s.split_whitespace() {
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

// ─── Reading history ──────────────────────────────────────────────────────────

fn view_rune_history() {
    use crate::persistence::{get_all_readings, get_user_readings};

    static HIST_ITEMS: &[MenuItem] = &[
        MenuItem {
            key: "1",
            icon: "📖",
            label: "My readings (by name)",
            hint: "Look up all readings recorded for a specific name",
        },
        MenuItem {
            key: "2",
            icon: "🌐",
            label: "All readings (global)",
            hint: "Every reading across all users, newest first",
        },
    ];
    static HIST_MENU: Menu = Menu {
        title: "📖  RUNE READING HISTORY",
        border_color: MenuColor::Cyan,
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
                                let title = format!("Readings for {}", user.name);
                                print_rune_readings_table(&readings, &title);
                            }
                        },
                    },
                }
            }
            "2" => match open_db() {
                Err(e) => println!("  {}", format!("Database error: {e}").red()),
                Ok(conn) => match get_all_readings(&conn) {
                    Err(e) => println!("  {}", format!("Query error: {e}").red()),
                    Ok(readings) => print_rune_readings_table(&readings, "All Rune Readings"),
                },
            },
            "0" | "" => break,
            _ => {}
        }
    }
}

fn print_rune_readings_table(readings: &[crate::persistence::ReadingRecord], title: &str) {
    println!();
    println!("{}", format!("  ══  {}  ══", title).bright_cyan().bold());
    if readings.is_empty() {
        println!("  {}", "  No readings recorded yet.".dimmed());
        println!();
        return;
    }
    println!(
        "  {:<20} {:<24} {:<18} {:<16} {}",
        "User".bold(),
        "Date".bold(),
        "Tradition".bold(),
        "Spread".bold(),
        "Runes".bold(),
    );
    println!("  {}", "─".repeat(90).dimmed());
    for r in readings {
        let runes_preview: String = r.cards.lines().take(3).collect::<Vec<_>>().join(", ");
        let runes_preview = if r.cards.lines().count() > 3 {
            format!("{}, …", runes_preview)
        } else {
            runes_preview
        };
        println!(
            "  {:<20} {:<24} {:<18} {:<16} {}",
            r.user_name.bright_yellow(),
            r.drawn_at.dimmed(),
            r.tradition.cyan(),
            r.spread_type.white(),
            runes_preview.dimmed(),
        );
    }
    println!();

    // Export
    let readings_owned: Vec<crate::persistence::ReadingRecord> = readings.iter().map(|r| {
        crate::persistence::ReadingRecord {
            id: r.id.clone(),
            user_name: r.user_name.clone(),
            drawn_at: r.drawn_at.clone(),
            tradition: r.tradition.clone(),
            spread_type: r.spread_type.clone(),
            cards: r.cards.clone(),
        }
    }).collect();
    let title_owned = title.to_string();
    handle_export(
        &format!("rune_history_{}", title.to_lowercase().replace(' ', "_")),
        || {
            let mut s = format!("{}\n{}\nGenerated: {}\n\n", title_owned, "=".repeat(title_owned.len()), chrono_now());
            for r in &readings_owned {
                s.push_str(&format!("User:      {}\n", r.user_name));
                s.push_str(&format!("Date:      {}\n", r.drawn_at));
                s.push_str(&format!("Tradition: {}\n", r.tradition));
                s.push_str(&format!("Spread:    {}\n", r.spread_type));
                s.push_str("Runes:\n");
                for line in r.cards.lines() {
                    s.push_str(&format!("  {}\n", line));
                }
                s.push_str(&"─".repeat(60));
                s.push('\n');
            }
            s
        },
        || {
            let mut rows = String::new();
            for r in &readings_owned {
                rows.push_str(&format!(
                    "<tr><td class=\"sys\">{}</td><td>{}</td><td>{}</td><td>{}</td><td class=\"meaning\">{}</td></tr>",
                    html_esc(&r.user_name),
                    html_esc(&r.drawn_at),
                    html_esc(&r.tradition),
                    html_esc(&r.spread_type),
                    html_esc(&r.cards.replace('\n', "<br>")),
                ));
            }
            let body = format!(
                "<h2 style=\"color:var(--accent);\">{}</h2>\
                 <table><thead><tr><th>User</th><th>Date</th><th>Tradition</th><th>Spread</th><th>Runes</th></tr></thead>\
                 <tbody>{}</tbody></table>",
                html_esc(&title_owned),
                rows
            );
            wrap_html(&title_owned, &body, "runic")
        },
    );
}

// ─── Hardware RNG helper ──────────────────────────────────────────────────────

fn rune_rnd(rng: &Option<RdRand>) -> u64 {
    match rng {
        Some(r) => r.try_next_u64().unwrap_or_else(|_| os_rune_u64()),
        None => os_rune_u64(),
    }
}

fn os_rune_u64() -> u64 {
    let mut bytes = [0u8; 8];
    getrandom::getrandom(&mut bytes).unwrap_or(());
    u64::from_le_bytes(bytes)
}
