//! Interactive session for the Zohar (Bereshith to Lekh Lekha).
//! Translation: Nurho de Manhar [1900–14], public domain via sacred-texts.com.

use std::io::{self, Write};

use colored::*;

use crate::menu::{Menu, MenuColor, MenuItem};
use crate::persistence::{
    get_zohar_chapter, load_text_position, lookup_zohar_verse, open_db, save_text_position,
    search_zohar, seed_zohar_from_static, zohar_chapter_count, zohar_is_loaded, zohar_stats,
    zohar_verse_count, ZoharVerse,
};
use crate::tts_reader::{
    build_chapter_speech, clean_for_tts, tts_auto_read, tts_nav_hint, tts_speak, tts_stop,
    tts_toggle_auto, tts_toggle_pause,
};
use crate::utils::read_key;

use super::{resolve_section, SECTIONS};

// ─── Menu ─────────────────────────────────────────────────────────────────────

static ZOHAR_ITEMS: &[MenuItem] = &[
    MenuItem {
        key:   "1",
        icon:  "🔍",
        label: "Search the Zohar",
        hint:  "Keyword, phrase, or boolean — FTS5 full-text search",
    },
    MenuItem {
        key:   "2",
        icon:  "📖",
        label: "Look up a passage",
        hint:  "e.g. Bereshith 1:1  ·  Lekh Lekha 3:2  ·  intro 2:1",
    },
    MenuItem {
        key:   "3",
        icon:  "📜",
        label: "Browse a chapter",
        hint:  "e.g. Bereshith 5  ·  ber 10  ·  lekh 3  ·  intro 1",
    },
    MenuItem {
        key:   "4",
        icon:  "📚",
        label: "List sections",
        hint:  "Show available sections and chapter counts",
    },
];

static ZOHAR_MENU: Menu = Menu {
    title:        "✦  THE ZOHAR  (Bereshith · Lekh Lekha)  ✦",
    border_color: MenuColor::BrightMagenta,
    items:        ZOHAR_ITEMS,
    back_key:     "0",
    back_label:   "Back to main menu",
};

// ─── Entry point ──────────────────────────────────────────────────────────────

pub fn run_zohar_session() {
    let conn = match open_db() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", format!("  Database error: {e}").red());
            return;
        }
    };

    if !zohar_is_loaded(&conn) {
        println!();
        print!("{}", "  🔯 Seeding Zohar (one-time, ~1 300 passages) …".dimmed());
        io::stdout().flush().unwrap_or(());
        match seed_zohar_from_static(&conn) {
            Ok(()) => println!("{}", "  done.".bright_green()),
            Err(e) => {
                println!();
                println!("{}", format!("  Seed error: {e}").red());
                return;
            }
        }
    }

    let (para_total, section_count) = zohar_stats(&conn);
    println!();
    println!(
        "  {} Zohar — {} sections · {} passages  (Nurho de Manhar tr.)",
        "🔯".bright_white(),
        section_count,
        para_total,
    );

    // Offer to resume the last reading position
    if let Some((saved_book, saved_chap, saved_verse)) = load_text_position(&conn, "zohar") {
        println!(
            "  {}  Last read: {} {}.{}",
            "↩".bright_cyan(),
            saved_book.bright_yellow(),
            saved_chap.to_string().bright_cyan(),
            saved_verse.to_string().bright_cyan(),
        );
        print!("{}", "  Resume? Enter=yes / n=skip: ".dimmed());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap_or(0);
        if buf.trim().to_lowercase() != "n" {
            if let Ok(Some(v)) = lookup_zohar_verse(&conn, &saved_book, saved_chap, saved_verse) {
                navigate_verses(&conn, v);
            }
        }
    }

    loop {
        let choice = ZOHAR_MENU.show_and_read();
        match choice.trim() {
            "1" => search_session(&conn),
            "2" => lookup_session(&conn),
            "3" => browse_session(&conn),
            "4" => list_sections(),
            "0" | "" => { tts_stop(); break; }
            _ => println!("{}", "  Please enter 1–4 or 0.".yellow()),
        }
    }
}

// ─── Search ───────────────────────────────────────────────────────────────────

fn search_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  FTS5 search syntax:".bold().bright_cyan());
    println!("{}", "    sephira divine       — passages containing both words".dimmed());
    println!("{}", "    \"book of light\"      — exact phrase".dimmed());
    println!("{}", "    kabbali*             — prefix wildcard".dimmed());
    println!("{}", "    light AND NOT shadow — boolean".dimmed());
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

        match search_zohar(conn, query, 50) {
            Ok(results) if results.is_empty() => {
                println!("{}", "  No passages matched.".yellow());
            }
            Ok(results) => {
                let total = results.len();
                println!();
                println!(
                    "  {} {} result{}:",
                    "🔯".bright_white(),
                    total.to_string().bright_yellow().bold(),
                    if total == 1 { "" } else { "s" }
                );
                paginate_verses(&results);
            }
            Err(e) => println!("{}", format!("  Search error: {e}  (check FTS5 syntax)").red()),
        }
    }
}

// ─── Passage lookup with navigation ──────────────────────────────────────────

fn lookup_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  Enter a passage reference, e.g.:".dimmed());
    println!("{}", "    Bereshith 5:2   ·   Lekh Lekha 3:1   ·   intro 1:1   ·   ber 10:3".dimmed());
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
            Some((section, chap, verse)) => match lookup_zohar_verse(conn, section, chap, verse) {
                Ok(Some(v)) => navigate_verses(conn, v),
                Ok(None) => println!(
                    "{}",
                    format!("  Not found: {section} {chap}:{verse}").yellow()
                ),
                Err(e) => println!("{}", format!("  DB error: {e}").red()),
            },
            None => println!(
                "{}",
                "  Could not parse — try 'Bereshith 5:2' or 'lekh 3:1'".yellow()
            ),
        }
    }
}

fn navigate_verses(conn: &rusqlite::Connection, start: ZoharVerse) {
    let book      = start.book.clone();
    let mut chap  = start.chapter;
    let mut verse = start.verse;
    let mut cur_text = start.text.clone();
    print_verse_card(&start);
    if tts_auto_read() { tts_speak(&clean_for_tts(&cur_text)); }

    loop {
        let max_chap  = zohar_chapter_count(conn, &book);
        let max_verse = zohar_verse_count(conn, &book, chap);
        let at_start  = chap == 1 && verse <= 1;
        let at_end    = chap >= max_chap && verse >= max_verse;

        let mut hint = String::from("  ── ");
        if !at_start { hint.push_str("p=prev  "); }
        if !at_end   { hint.push_str("n=next  "); }
        hint.push_str("c=chapter  ");
        hint.push_str(&tts_nav_hint("s"));
        hint.push_str("q=back");

        print!("{}", hint.dimmed());

        match read_key() {
            'n' | '\n' if !at_end => {
                let (nc, nv) = if verse < max_verse {
                    (chap, verse + 1)
                } else {
                    (chap + 1, 1)
                };
                match lookup_zohar_verse(conn, &book, nc, nv) {
                    Ok(Some(v)) => {
                        chap = nc; verse = nv;
                        cur_text = v.text.clone();
                        print_verse_card(&v);
                        if tts_auto_read() { tts_speak(&clean_for_tts(&cur_text)); }
                    }
                    Ok(None) => println!("{}", "  (no next passage found)".yellow()),
                    Err(e)   => println!("{}", format!("  DB error: {e}").red()),
                }
            }
            'p' if !at_start => {
                let (nc, nv) = if verse > 1 {
                    (chap, verse - 1)
                } else if chap > 1 {
                    let pc = chap - 1;
                    let pv = zohar_verse_count(conn, &book, pc).max(1);
                    (pc, pv)
                } else {
                    (chap, verse)
                };
                match lookup_zohar_verse(conn, &book, nc, nv) {
                    Ok(Some(v)) => {
                        chap = nc; verse = nv;
                        cur_text = v.text.clone();
                        print_verse_card(&v);
                        if tts_auto_read() { tts_speak(&clean_for_tts(&cur_text)); }
                    }
                    Ok(None) => println!("{}", "  (no prev passage found)".yellow()),
                    Err(e)   => println!("{}", format!("  DB error: {e}").red()),
                }
            }
            'c' => {
                let book_clone = book.clone();
                read_chapter(conn, &book_clone, chap, max_chap);
            }
            'r' => tts_speak(&clean_for_tts(&cur_text)),
            'a' => {
                let on = tts_toggle_auto();
                println!("  {}", if on { "Auto-read ON".bright_green() } else { "Auto-read OFF".yellow() });
            }
            't' => { tts_toggle_pause(); }
            's' => tts_stop(),
            'q' | '\n' => {
                save_text_position(conn, "zohar", &book, chap, verse).ok();
                break;
            }
            _ => {}
        }
    }
}

// ─── Browse ───────────────────────────────────────────────────────────────────

fn browse_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  Enter section + chapter, e.g.:".dimmed());
    println!("{}", "    Bereshith 5   ·   ber 10   ·   Lekh Lekha 3   ·   intro 2".dimmed());
    println!();

    loop {
        print!("{}", "  Section + chapter (blank to exit): ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or(0);
        let raw = input.trim();

        if raw.is_empty() {
            break;
        }

        match parse_section_chapter(raw) {
            Some((section, chap)) => {
                let max_chap = zohar_chapter_count(conn, section);
                let mut cur  = chap;
                loop {
                    match read_chapter(conn, section, cur, max_chap) {
                        ChapNav::Next if cur < max_chap => cur += 1,
                        ChapNav::Prev if cur > 1        => cur -= 1,
                        _ => break,
                    }
                }
            }
            None => println!(
                "{}",
                "  Not recognised — try 'Bereshith 5' or 'lekh 3'".yellow()
            ),
        }
    }
}

// ─── Chapter reader ───────────────────────────────────────────────────────────

enum ChapNav { Prev, Next, Done }

fn read_chapter(
    conn: &rusqlite::Connection,
    section: &str,
    chapter: u32,
    max_chap: u32,
) -> ChapNav {
    let verses = match get_zohar_chapter(conn, section, chapter) {
        Ok(v) if !v.is_empty() => v,
        Ok(_) => {
            println!("{}", format!("  No content found for {section} ch.{chapter}.").yellow());
            return ChapNav::Done;
        }
        Err(e) => {
            println!("{}", format!("  DB error: {e}").red());
            return ChapNav::Done;
        }
    };

    println!();
    println!(
        "  {} {}  Chapter {}  ·  {} passages",
        "🔯".bright_white(),
        section.bright_yellow().bold(),
        chapter.to_string().bright_cyan(),
        verses.len(),
    );
    println!(
        "{}",
        "  ──────────────────────────────────────────────────────".dimmed()
    );

    if tts_auto_read() {
        let texts: Vec<&str> = verses.iter().map(|v| v.text.as_str()).collect();
        tts_speak(&build_chapter_speech(section, chapter, &texts));
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
            nav.push_str("── End of chapter ── ");
        } else {
            nav.push_str(&format!("── {start}/{total} ── Enter=more  "));
        }
        if chapter > 1        { nav.push_str("p=prev  "); }
        if chapter < max_chap { nav.push_str("n=next  "); }
        nav.push_str(&tts_nav_hint("s"));
        nav.push_str("q=back");

        print!("  {}", nav.dimmed());

        match read_key() {
            'n' if chapter < max_chap => return ChapNav::Next,
            'p' if chapter > 1        => return ChapNav::Prev,
            'r' => {
                let texts: Vec<&str> = verses.iter().map(|v| v.text.as_str()).collect();
                tts_speak(&build_chapter_speech(section, chapter, &texts));
            }
            'a' => {
                let on = tts_toggle_auto();
                println!("  {}", if on { "Auto-read ON".bright_green() } else { "Auto-read OFF".yellow() });
            }
            't' => { tts_toggle_pause(); }
            's' => tts_stop(),
            'q'            => return ChapNav::Done,
            '\n' if at_end => return ChapNav::Done,
            _ => {
                if at_end { return ChapNav::Done; }
            }
        }
    }
}

// ─── Section list ─────────────────────────────────────────────────────────────

fn list_sections() {
    println!();
    println!("{}", "  ── Zohar Sections ───────────────────────────────────────────".bright_yellow());
    for (i, s) in SECTIONS.iter().enumerate() {
        let line = format!(
            "{}.  {:<14}  {:>3} ch.  {}",
            i + 1, s.name, s.chapters, s.blurb
        );
        println!("  {}", line.bright_white());
    }
    println!();
    println!("{}", "  Short names: intro, ber, lekh  (use in Browse or Lookup)".dimmed());
    println!("{}", "  Translation: Nurho de Manhar [1900–14] — public domain".dimmed());
    println!();
}

// ─── Reference parsers ────────────────────────────────────────────────────────

/// Parse `"Bereshith 5:2"` or `"ber 10:3"` → `Some((section_name, chapter, verse))`.
fn parse_verse_ref(input: &str) -> Option<(&'static str, u32, u32)> {
    let s = input.trim();
    let colon = s.rfind(':')?;
    let verse: u32 = s[colon + 1..].trim().parse().ok()?;
    let prefix = s[..colon].trim();
    let space = prefix.rfind(|c: char| c.is_ascii_whitespace())?;
    let chapter: u32 = prefix[space + 1..].trim().parse().ok()?;
    let sec_str = &prefix[..space];
    let sec = resolve_section(sec_str)?;
    Some((sec.name, chapter, verse))
}

/// Parse `"Bereshith 5"` or `"lekh 3"` → `Some((section_name, chapter))`.
fn parse_section_chapter(input: &str) -> Option<(&'static str, u32)> {
    let s = input.trim();
    let space = s.rfind(|c: char| c.is_ascii_whitespace())?;
    let chapter: u32 = s[space + 1..].trim().parse().ok()?;
    let sec_str = &s[..space];
    let sec = resolve_section(sec_str)?;
    Some((sec.name, chapter))
}

// ─── Display helpers ──────────────────────────────────────────────────────────

fn print_verse_card(v: &ZoharVerse) {
    let sep = "  ──────────────────────────────────────────────────────";
    println!();
    println!("{}", sep.dimmed());
    println!(
        "  {} {}.{}  {}",
        v.book.bright_yellow().bold(),
        v.chapter.to_string().bright_cyan(),
        v.verse.to_string().bright_cyan(),
        "Nurho de Manhar tr. · sacred-texts.com".dimmed(),
    );
    println!("{}", sep.dimmed());
    for line in word_wrap(&v.text, 68) {
        println!("  {}", line.bright_white());
    }
    println!("{}", sep.dimmed());
    println!();
}

fn print_verse_full(v: &ZoharVerse) {
    let ref_tag = format!("{} {}.{}", v.book, v.chapter, v.verse);
    println!("  {}", ref_tag.bright_yellow().bold());
    for line in word_wrap(&v.text, 68) {
        println!("  {}", line.bright_white());
    }
    println!();
}

fn paginate_verses(verses: &[ZoharVerse]) {
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
