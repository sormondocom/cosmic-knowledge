//! Interactive session for the Pistis Sophia (G.R.S. Mead tr., 1921).
//! Six books, 148 chapters of Gnostic discourse between Jesus and his disciples.

use std::io::{self, Write};

use colored::*;

use crate::menu::{Menu, MenuColor, MenuItem};
use crate::persistence::{
    get_ps_chapter, load_text_position, lookup_ps_verse, open_db, ps_chapter_count,
    ps_chapter_min, ps_is_loaded, ps_stats, ps_verse_count, save_text_position,
    search_ps, seed_ps_from_static, PsVerse,
};
use crate::tts_reader::{
    build_chapter_speech, clean_for_tts, tts_auto_read, tts_nav_hint, tts_speak, tts_stop,
    tts_toggle_auto, tts_toggle_pause,
};
use crate::utils::read_key;

use super::{book_for_chapter, resolve_book, BOOKS};

// ─── Menu ─────────────────────────────────────────────────────────────────────

static PS_ITEMS: &[MenuItem] = &[
    MenuItem {
        key:   "1",
        icon:  "🔍",
        label: "Search the Pistis Sophia",
        hint:  "Keyword, phrase, or boolean — FTS5 full-text search",
    },
    MenuItem {
        key:   "2",
        icon:  "📖",
        label: "Look up a passage",
        hint:  "e.g. Book 1 1:2  ·  b2 70:1  ·  3 110:3",
    },
    MenuItem {
        key:   "3",
        icon:  "📜",
        label: "Browse a chapter",
        hint:  "e.g. Book 1 5  ·  b2 70  ·  3 110  ·  chapter 42",
    },
    MenuItem {
        key:   "4",
        icon:  "📚",
        label: "List books",
        hint:  "Show all six books and their chapter ranges",
    },
];

static PS_MENU: Menu = Menu {
    title:        "✦  PISTIS SOPHIA  (G.R.S. Mead, 1921)  ✦",
    border_color: MenuColor::BrightMagenta,
    items:        PS_ITEMS,
    back_key:     "0",
    back_label:   "Back to main menu",
};

// ─── Entry point ──────────────────────────────────────────────────────────────

pub fn run_ps_session() {
    let conn = match open_db() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", format!("  Database error: {e}").red());
            return;
        }
    };

    if !ps_is_loaded(&conn) {
        println!();
        print!("{}", "  ✨ Seeding Pistis Sophia (one-time, ~2 200 passages) …".dimmed());
        io::stdout().flush().unwrap_or(());
        match seed_ps_from_static(&conn) {
            Ok(()) => println!("{}", "  done.".bright_green()),
            Err(e) => {
                println!();
                println!("{}", format!("  Seed error: {e}").red());
                return;
            }
        }
    }

    let (para_total, book_count) = ps_stats(&conn);
    println!();
    println!(
        "  {} Pistis Sophia — {} books · {} passages  (G.R.S. Mead tr.)",
        "✨".bright_white(),
        book_count,
        para_total,
    );

    // Offer to resume the last reading position
    if let Some((saved_book, saved_chap, saved_verse)) = load_text_position(&conn, "ps") {
        println!(
            "  {}  Last read: {} ch.{} ¶{}",
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
            if let Ok(Some(v)) = lookup_ps_verse(&conn, &saved_book, saved_chap, saved_verse) {
                navigate_verses(&conn, v);
            }
        }
    }

    loop {
        let choice = PS_MENU.show_and_read();
        match choice.trim() {
            "1" => search_session(&conn),
            "2" => lookup_session(&conn),
            "3" => browse_session(&conn),
            "4" => list_books(),
            "0" | "" => { tts_stop(); break; }
            _ => println!("{}", "  Please enter 1–4 or 0.".yellow()),
        }
    }
}

// ─── Search ───────────────────────────────────────────────────────────────────

fn search_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  FTS5 search syntax:".bold().bright_cyan());
    println!("{}", "    sophia light         — passages containing both words".dimmed());
    println!("{}", "    \"treasury of light\"  — exact phrase".dimmed());
    println!("{}", "    pistis*              — prefix wildcard".dimmed());
    println!("{}", "    Mary AND NOT Peter   — boolean".dimmed());
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

        match search_ps(conn, query, 50) {
            Ok(results) if results.is_empty() => {
                println!("{}", "  No passages matched.".yellow());
            }
            Ok(results) => {
                let total = results.len();
                println!();
                println!(
                    "  {} {} result{}:",
                    "✨".bright_white(),
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
    println!("{}", "    Book 1 1:3   ·   b2 70:1   ·   3 110:2   ·   chapter 42 ¶1".dimmed());
    println!("{}", "  (book + global-chapter : paragraph)".dimmed());
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
            Some((book, chap, verse)) => match lookup_ps_verse(conn, book, chap, verse) {
                Ok(Some(v)) => navigate_verses(conn, v),
                Ok(None) => println!(
                    "{}",
                    format!("  Not found: {book} ch.{chap} ¶{verse}").yellow()
                ),
                Err(e) => println!("{}", format!("  DB error: {e}").red()),
            },
            None => println!(
                "{}",
                "  Could not parse — try 'Book 1 5:2' or 'b2 70:1'".yellow()
            ),
        }
    }
}

fn navigate_verses(conn: &rusqlite::Connection, start: PsVerse) {
    let book      = start.book.clone();
    let mut chap  = start.chapter;
    let mut verse = start.verse;
    let mut cur_text = start.text.clone();
    print_verse_card(&start);
    if tts_auto_read() { tts_speak(&clean_for_tts(&cur_text)); }

    loop {
        let max_chap  = ps_chapter_count(conn, &book);
        let min_chap  = ps_chapter_min(conn, &book);
        let max_verse = ps_verse_count(conn, &book, chap);
        let at_start  = chap <= min_chap && verse <= 1;
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
                match lookup_ps_verse(conn, &book, nc, nv) {
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
                } else if chap > min_chap {
                    let pc = chap - 1;
                    let pv = ps_verse_count(conn, &book, pc).max(1);
                    (pc, pv)
                } else {
                    (chap, verse)
                };
                match lookup_ps_verse(conn, &book, nc, nv) {
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
                read_chapter(conn, &book_clone, chap, ps_chapter_count(conn, &book), ps_chapter_min(conn, &book));
            }
            'r' => tts_speak(&clean_for_tts(&cur_text)),
            'a' => {
                let on = tts_toggle_auto();
                println!("  {}", if on { "Auto-read ON".bright_green() } else { "Auto-read OFF".yellow() });
            }
            't' => { tts_toggle_pause(); }
            's' => tts_stop(),
            'q' | '\n' => {
                save_text_position(conn, "ps", &book, chap, verse).ok();
                break;
            }
            _ => {}
        }
    }
}

// ─── Browse ───────────────────────────────────────────────────────────────────

fn browse_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  Enter book + chapter, or just a chapter number, e.g.:".dimmed());
    println!("{}", "    Book 1 5   ·   b2 70   ·   3 110   ·   42".dimmed());
    println!();

    loop {
        print!("{}", "  Book + chapter (blank to exit): ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or(0);
        let raw = input.trim();

        if raw.is_empty() {
            break;
        }

        match parse_book_chapter(raw) {
            Some((book, chap)) => {
                let max_chap = ps_chapter_count(conn, book);
                let min_chap = ps_chapter_min(conn, book);
                let mut cur  = chap;
                loop {
                    match read_chapter(conn, book, cur, max_chap, min_chap) {
                        ChapNav::Next if cur < max_chap => cur += 1,
                        ChapNav::Prev if cur > min_chap => cur -= 1,
                        _ => break,
                    }
                }
            }
            None => println!(
                "{}",
                "  Not recognised — try 'Book 1 5' or 'b2 70' or just '42'".yellow()
            ),
        }
    }
}

// ─── Chapter reader ───────────────────────────────────────────────────────────

enum ChapNav { Prev, Next, Done }

fn read_chapter(
    conn: &rusqlite::Connection,
    book: &str,
    chapter: u32,
    max_chap: u32,
    min_chap: u32,
) -> ChapNav {
    let verses = match get_ps_chapter(conn, book, chapter) {
        Ok(v) if !v.is_empty() => v,
        Ok(_) => {
            println!("{}", format!("  No content found for {book} ch.{chapter}.").yellow());
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
        "✨".bright_white(),
        book.bright_yellow().bold(),
        chapter.to_string().bright_cyan(),
        verses.len(),
    );
    println!(
        "{}",
        "  ──────────────────────────────────────────────────────".dimmed()
    );

    if tts_auto_read() {
        let texts: Vec<&str> = verses.iter().map(|v| v.text.as_str()).collect();
        tts_speak(&build_chapter_speech(book, chapter, &texts));
    }

    const PAGE: usize = 4;
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
        if chapter > min_chap { nav.push_str("p=prev  "); }
        if chapter < max_chap { nav.push_str("n=next  "); }
        nav.push_str(&tts_nav_hint("s"));
        nav.push_str("q=back");

        print!("  {}", nav.dimmed());

        match read_key() {
            'n' if chapter < max_chap => return ChapNav::Next,
            'p' if chapter > min_chap => return ChapNav::Prev,
            'r' => {
                let texts: Vec<&str> = verses.iter().map(|v| v.text.as_str()).collect();
                tts_speak(&build_chapter_speech(book, chapter, &texts));
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

// ─── Book list ────────────────────────────────────────────────────────────────

fn list_books() {
    println!();
    println!("{}", "  ── Pistis Sophia ────────────────────────────────────────────".bright_yellow());
    for (i, b) in BOOKS.iter().enumerate() {
        let line = format!("{}.  {:<8}  {:>3} ch.  {}", i + 1, b.name, b.ch_hi - b.ch_lo + 1, b.blurb);
        println!("  {}", line.bright_white());
    }
    println!();
    println!("{}", "  Short names: b1–b6  (use in Browse or Lookup)".dimmed());
    println!("{}", "  Translation: G.R.S. Mead [1921] — public domain".dimmed());
    println!();
}

// ─── Reference parsers ────────────────────────────────────────────────────────

/// Parse `"Book 1 5:2"`, `"b2 70:1"`, `"3 110:3"` → `Some((book_name, chapter, verse))`.
fn parse_verse_ref(input: &str) -> Option<(&'static str, u32, u32)> {
    let s = input.trim();
    let colon = s.rfind(':')?;
    let verse: u32 = s[colon + 1..].trim().parse().ok()?;
    let prefix = s[..colon].trim();
    let space = prefix.rfind(|c: char| c.is_ascii_whitespace())?;
    let chapter: u32 = prefix[space + 1..].trim().parse().ok()?;
    let book_str = &prefix[..space];
    let book = resolve_book(book_str)?;
    Some((book.name, chapter, verse))
}

/// Parse `"Book 1 5"`, `"b2 70"`, `"42"` → `Some((book_name, chapter))`.
fn parse_book_chapter(input: &str) -> Option<(&'static str, u32)> {
    let s = input.trim();

    // Pure chapter number — find the right book automatically
    if let Ok(ch) = s.parse::<u32>() {
        let book = book_for_chapter(ch)?;
        return Some((book.name, ch));
    }

    let space = s.rfind(|c: char| c.is_ascii_whitespace())?;
    let chapter: u32 = s[space + 1..].trim().parse().ok()?;
    let book_str = &s[..space];
    let book = resolve_book(book_str)?;
    Some((book.name, chapter))
}

// ─── Display helpers ──────────────────────────────────────────────────────────

fn print_verse_card(v: &PsVerse) {
    let sep = "  ──────────────────────────────────────────────────────";
    println!();
    println!("{}", sep.dimmed());
    println!(
        "  {} ch.{} ¶{}  {}",
        v.book.bright_yellow().bold(),
        v.chapter.to_string().bright_cyan(),
        v.verse.to_string().bright_cyan(),
        "G.R.S. Mead tr. · sacred-texts.com".dimmed(),
    );
    println!("{}", sep.dimmed());
    for line in word_wrap(&v.text, 68) {
        println!("  {}", line.bright_white());
    }
    println!("{}", sep.dimmed());
    println!();
}

fn print_verse_full(v: &PsVerse) {
    let ref_tag = format!("{} ch.{} ¶{}", v.book, v.chapter, v.verse);
    println!("  {}", ref_tag.bright_yellow().bold());
    for line in word_wrap(&v.text, 68) {
        println!("  {}", line.bright_white());
    }
    println!();
}

fn paginate_verses(verses: &[PsVerse]) {
    const PAGE: usize = 4;
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
