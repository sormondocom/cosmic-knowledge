//! Interactive session for the Trimorphic Protennoia (John D. Turner tr.).
//! Three discourses (NHC XIII,1): Thought · Voice · Word.

use std::io::{self, Write};

use colored::*;

use crate::menu::{Menu, MenuColor, MenuItem};
use crate::persistence::{
    get_trimorphic_discourse, load_text_position, lookup_trimorphic_verse, open_db,
    save_text_position, search_trimorphic, seed_trimorphic_from_static, trimorphic_is_loaded,
    trimorphic_stats, trimorphic_verse_count, TrimorphicVerse,
};
use crate::tts_reader::{
    build_chapter_speech, clean_for_tts, tts_auto_read, tts_nav_hint, tts_speak, tts_stop,
    tts_toggle_auto, tts_toggle_pause,
};
use crate::utils::read_key;

use super::{resolve_discourse, DISCOURSES};

// ─── Menu ─────────────────────────────────────────────────────────────────────

static TRIM_ITEMS: &[MenuItem] = &[
    MenuItem {
        key:   "1",
        icon:  "🔍",
        label: "Search the Trimorphic Protennoia",
        hint:  "Keyword, phrase, or boolean — FTS5 full-text search",
    },
    MenuItem {
        key:   "2",
        icon:  "📖",
        label: "Look up a passage",
        hint:  "Discourse:paragraph — e.g. 1:3  ·  2:5  ·  3:1",
    },
    MenuItem {
        key:   "3",
        icon:  "📜",
        label: "Browse a discourse",
        hint:  "1 = The Thought  ·  2 = The Voice  ·  3 = The Word",
    },
    MenuItem {
        key:   "4",
        icon:  "📚",
        label: "List discourses",
        hint:  "Show the three hypostatic discourses with descriptions",
    },
];

static TRIM_MENU: Menu = Menu {
    title:        "✦  TRIMORPHIC PROTENNOIA  (John D. Turner tr.)  ✦",
    border_color: MenuColor::BrightMagenta,
    items:        TRIM_ITEMS,
    back_key:     "0",
    back_label:   "Back to main menu",
};

// ─── Entry point ──────────────────────────────────────────────────────────────

pub fn run_trimorphic_session() {
    let conn = match open_db() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", format!("  Database error: {e}").red());
            return;
        }
    };

    if !trimorphic_is_loaded(&conn) {
        println!();
        print!("{}", "  🕊  Seeding Trimorphic Protennoia (one-time, ~37 passages) …".dimmed());
        io::stdout().flush().unwrap_or(());
        match seed_trimorphic_from_static(&conn) {
            Ok(()) => println!("{}", "  done.".bright_green()),
            Err(e) => {
                println!();
                println!("{}", format!("  Seed error: {e}").red());
                return;
            }
        }
    }

    let (para_total, disc_count) = trimorphic_stats(&conn);
    println!();
    println!(
        "  {} Trimorphic Protennoia — {} discourses · {} passages  (J.D. Turner tr.)",
        "🕊".bright_white(),
        disc_count,
        para_total,
    );

    // Offer to resume last reading position
    if let Some((_book, saved_chap, saved_verse)) = load_text_position(&conn, "trimorphic") {
        let disc_name = DISCOURSES
            .iter()
            .find(|d| d.num == saved_chap)
            .map(|d| d.name)
            .unwrap_or("?");
        println!(
            "  {}  Last read: Discourse {} ({}) ¶{}",
            "↩".bright_cyan(),
            saved_chap.to_string().bright_cyan(),
            disc_name.bright_yellow(),
            saved_verse.to_string().bright_cyan(),
        );
        print!("{}", "  Resume? Enter=yes / n=skip: ".dimmed());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap_or(0);
        if buf.trim().to_lowercase() != "n" {
            if let Ok(Some(v)) = lookup_trimorphic_verse(&conn, saved_chap, saved_verse) {
                navigate_verses(&conn, v);
            }
        }
    }

    loop {
        let choice = TRIM_MENU.show_and_read();
        match choice.trim() {
            "1" => search_session(&conn),
            "2" => lookup_session(&conn),
            "3" => browse_session(&conn),
            "4" => list_discourses(),
            "0" | "" => { tts_stop(); break; }
            _ => println!("{}", "  Please enter 1–4 or 0.".yellow()),
        }
    }
}

// ─── Search ───────────────────────────────────────────────────────────────────

fn search_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  FTS5 search syntax:".bold().bright_cyan());
    println!("{}", "    thought light        — passages containing both words".dimmed());
    println!("{}", "    \"perfect light\"      — exact phrase".dimmed());
    println!("{}", "    logos*               — prefix wildcard".dimmed());
    println!("{}", "    voice AND NOT chaos  — boolean".dimmed());
    println!();

    loop {
        print!("{}", "  Search query (blank to exit): ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or(0);
        let query = input.trim();

        if query.is_empty() {
            break;
        }

        match search_trimorphic(conn, query, 50) {
            Ok(results) if results.is_empty() => {
                println!("{}", "  No passages matched.".yellow());
            }
            Ok(results) => {
                let total = results.len();
                println!();
                println!(
                    "  {} {} result{}:",
                    "🕊".bright_white(),
                    total.to_string().bright_yellow().bold(),
                    if total == 1 { "" } else { "s" }
                );
                paginate_verses(&results);
            }
            Err(e) => println!("{}", format!("  Search error: {e}  (check FTS5 syntax)").red()),
        }
    }
}

// ─── Passage lookup ───────────────────────────────────────────────────────────

fn lookup_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  Enter a passage reference, e.g.:".dimmed());
    println!("{}", "    1:3   (discourse 1, paragraph 3)".dimmed());
    println!("{}", "    2:5   (discourse 2, paragraph 5)".dimmed());
    println!("{}", "    3:1   (discourse 3, paragraph 1)".dimmed());
    println!();

    loop {
        print!("{}", "  Reference (blank to exit): ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or(0);
        let raw = input.trim();

        if raw.is_empty() {
            break;
        }

        match parse_verse_ref(raw) {
            Some((disc, para)) => match lookup_trimorphic_verse(conn, disc, para) {
                Ok(Some(v)) => navigate_verses(conn, v),
                Ok(None) => println!(
                    "{}",
                    format!("  Not found: discourse {disc} ¶{para}").yellow()
                ),
                Err(e) => println!("{}", format!("  DB error: {e}").red()),
            },
            None => println!(
                "{}",
                "  Could not parse — try '1:3' (discourse:paragraph)".yellow()
            ),
        }
    }
}

fn navigate_verses(conn: &rusqlite::Connection, start: TrimorphicVerse) {
    let mut disc  = start.chapter;
    let mut para  = start.verse;
    let mut cur_text = start.text.clone();
    print_verse_card(&start);
    if tts_auto_read() { tts_speak(&clean_for_tts(&cur_text)); }

    loop {
        let max_para = trimorphic_verse_count(conn, disc);
        let at_start = disc <= 1 && para <= 1;
        let at_end   = disc >= 3 && para >= max_para;

        let mut hint = String::from("  ── ");
        if !at_start { hint.push_str("p=prev  "); }
        if !at_end   { hint.push_str("n=next  "); }
        hint.push_str("d=discourse  ");
        hint.push_str(&tts_nav_hint("s"));
        hint.push_str("q=back");

        print!("{}", hint.dimmed());

        match read_key() {
            'n' | '\n' if !at_end => {
                let (nd, np) = if para < max_para {
                    (disc, para + 1)
                } else {
                    (disc + 1, 1)
                };
                match lookup_trimorphic_verse(conn, nd, np) {
                    Ok(Some(v)) => {
                        disc = nd; para = np;
                        cur_text = v.text.clone();
                        print_verse_card(&v);
                        if tts_auto_read() { tts_speak(&clean_for_tts(&cur_text)); }
                    }
                    Ok(None) => println!("{}", "  (no next passage found)".yellow()),
                    Err(e)   => println!("{}", format!("  DB error: {e}").red()),
                }
            }
            'p' if !at_start => {
                let (nd, np) = if para > 1 {
                    (disc, para - 1)
                } else if disc > 1 {
                    let pd = disc - 1;
                    let pp = trimorphic_verse_count(conn, pd).max(1);
                    (pd, pp)
                } else {
                    (disc, para)
                };
                match lookup_trimorphic_verse(conn, nd, np) {
                    Ok(Some(v)) => {
                        disc = nd; para = np;
                        cur_text = v.text.clone();
                        print_verse_card(&v);
                        if tts_auto_read() { tts_speak(&clean_for_tts(&cur_text)); }
                    }
                    Ok(None) => println!("{}", "  (no prev passage found)".yellow()),
                    Err(e)   => println!("{}", format!("  DB error: {e}").red()),
                }
            }
            'd' => {
                read_discourse(conn, disc);
            }
            'r' => tts_speak(&clean_for_tts(&cur_text)),
            'a' => {
                let on = tts_toggle_auto();
                println!("  {}", if on { "Auto-read ON".bright_green() } else { "Auto-read OFF".yellow() });
            }
            't' => { tts_toggle_pause(); }
            's' => tts_stop(),
            'q' | '\n' => {
                save_text_position(conn, "trimorphic", "Trimorphic Protennoia", disc, para).ok();
                break;
            }
            _ => {}
        }
    }
}

// ─── Browse ───────────────────────────────────────────────────────────────────

fn browse_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  Enter a discourse number or name:".dimmed());
    println!("{}", "    1  ·  thought  ·  voice  ·  3  ·  word".dimmed());
    println!();

    loop {
        print!("{}", "  Discourse (blank to exit): ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or(0);
        let raw = input.trim();

        if raw.is_empty() {
            break;
        }

        match resolve_discourse(raw) {
            Some(d) => {
                let mut cur = d.num;
                loop {
                    match read_discourse(conn, cur) {
                        DiscNav::Next if cur < 3 => cur += 1,
                        DiscNav::Prev if cur > 1 => cur -= 1,
                        _ => break,
                    }
                }
            }
            None => println!(
                "{}",
                "  Not recognised — enter 1, 2, or 3 (or thought / voice / word)".yellow()
            ),
        }
    }
}

// ─── Discourse reader ─────────────────────────────────────────────────────────

enum DiscNav { Prev, Next, Done }

fn read_discourse(conn: &rusqlite::Connection, disc: u32) -> DiscNav {
    let verses = match get_trimorphic_discourse(conn, disc) {
        Ok(v) if !v.is_empty() => v,
        Ok(_) => {
            println!("{}", format!("  No content found for discourse {disc}.").yellow());
            return DiscNav::Done;
        }
        Err(e) => {
            println!("{}", format!("  DB error: {e}").red());
            return DiscNav::Done;
        }
    };

    let disc_name = DISCOURSES
        .iter()
        .find(|d| d.num == disc)
        .map(|d| d.name)
        .unwrap_or("?");

    println!();
    println!(
        "  {} Discourse {}  ·  {}  ·  {} passages",
        "🕊".bright_white(),
        disc.to_string().bright_cyan(),
        disc_name.bright_yellow().bold(),
        verses.len(),
    );
    println!(
        "{}",
        "  ──────────────────────────────────────────────────────".dimmed()
    );

    if tts_auto_read() {
        let texts: Vec<&str> = verses.iter().map(|v| v.text.as_str()).collect();
        tts_speak(&build_chapter_speech(disc_name, disc, &texts));
    }

    const PAGE: usize = 3;
    let total  = verses.len();
    let mut start = 0;

    loop {
        let end = (start + PAGE).min(total);
        for v in &verses[start..end] {
            print_verse_full(v);
        }
        start = end;

        let at_end = start >= total;
        let mut nav = String::new();
        if at_end {
            nav.push_str("── End of discourse ── ");
        } else {
            nav.push_str(&format!("── {start}/{total} ── Enter=more  "));
        }
        if disc > 1 { nav.push_str("p=prev  "); }
        if disc < 3 { nav.push_str("n=next  "); }
        nav.push_str(&tts_nav_hint("s"));
        nav.push_str("q=back");

        print!("  {}", nav.dimmed());

        match read_key() {
            'n' if disc < 3 => return DiscNav::Next,
            'p' if disc > 1 => return DiscNav::Prev,
            'r' => {
                let texts: Vec<&str> = verses.iter().map(|v| v.text.as_str()).collect();
                tts_speak(&build_chapter_speech(disc_name, disc, &texts));
            }
            'a' => {
                let on = tts_toggle_auto();
                println!("  {}", if on { "Auto-read ON".bright_green() } else { "Auto-read OFF".yellow() });
            }
            't' => { tts_toggle_pause(); }
            's' => tts_stop(),
            'q'            => return DiscNav::Done,
            '\n' if at_end => return DiscNav::Done,
            _ => {
                if at_end { return DiscNav::Done; }
            }
        }
    }
}

// ─── Discourse list ───────────────────────────────────────────────────────────

fn list_discourses() {
    println!();
    println!("{}", "  ── Trimorphic Protennoia (NHC XIII,1) ───────────────────────".bright_yellow());
    for d in DISCOURSES {
        let line = format!("{}.  {:<12}  {}", d.num, d.name, d.blurb);
        println!("  {}", line.bright_white());
    }
    println!();
    println!("{}", "  Translation: John D. Turner — Nag Hammadi Library".dimmed());
    println!("{}", "  Source: earlychristianwritings.com".dimmed());
    println!();
}

// ─── Reference parser ─────────────────────────────────────────────────────────

/// Parse `"1:3"` → `Some((discourse, paragraph))`.
fn parse_verse_ref(input: &str) -> Option<(u32, u32)> {
    let s = input.trim();
    let colon = s.find(':')?;
    let disc: u32 = s[..colon].trim().parse().ok()?;
    let para: u32 = s[colon + 1..].trim().parse().ok()?;
    if disc < 1 || disc > 3 { return None; }
    Some((disc, para))
}

// ─── Display helpers ──────────────────────────────────────────────────────────

fn discourse_name(disc: u32) -> &'static str {
    DISCOURSES
        .iter()
        .find(|d| d.num == disc)
        .map(|d| d.name)
        .unwrap_or("?")
}

fn print_verse_card(v: &TrimorphicVerse) {
    let sep = "  ──────────────────────────────────────────────────────";
    println!();
    println!("{}", sep.dimmed());
    println!(
        "  Discourse {} ({}) ¶{}  {}",
        v.chapter.to_string().bright_cyan(),
        discourse_name(v.chapter).bright_yellow().bold(),
        v.verse.to_string().bright_cyan(),
        "J.D. Turner tr. · NHC XIII,1".dimmed(),
    );
    println!("{}", sep.dimmed());
    for line in word_wrap(&v.text, 68) {
        println!("  {}", line.bright_white());
    }
    println!("{}", sep.dimmed());
    println!();
}

fn print_verse_full(v: &TrimorphicVerse) {
    let ref_tag = format!(
        "Discourse {} ({}) ¶{}",
        v.chapter,
        discourse_name(v.chapter),
        v.verse
    );
    println!("  {}", ref_tag.bright_yellow().bold());
    for line in word_wrap(&v.text, 68) {
        println!("  {}", line.bright_white());
    }
    println!();
}

fn paginate_verses(verses: &[TrimorphicVerse]) {
    const PAGE: usize = 3;
    let mut start = 0;
    let total     = verses.len();
    loop {
        let end = (start + PAGE).min(total);
        for v in &verses[start..end] {
            print_verse_full(v);
        }
        start = end;
        if start >= total {
            break;
        }
        print!(
            "{}",
            format!("  ── {} of {} shown — Enter=more, q=stop: ", start, total).dimmed()
        );
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap_or(0);
        if buf.trim().eq_ignore_ascii_case("q") {
            break;
        }
    }
    println!();
}

fn word_wrap(text: &str, max_cols: usize) -> Vec<String> {
    let mut lines   = Vec::new();
    let mut current = String::new();
    for word in text.split_ascii_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= max_cols {
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
    lines
}
