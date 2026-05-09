//! Interactive session for the Apocrypha collection:
//! 1 Enoch, 2 Enoch (Secrets of Enoch), and Jubilees.

use std::io::{self, Write};

use colored::*;

use crate::menu::{Menu, MenuColor, MenuItem};
use crate::persistence::{
    apocr_chapter_count, apocr_verse_count, apocrypha_is_loaded, apocrypha_stats,
    get_apocr_chapter, lookup_apocr_verse, open_db, search_apocrypha, seed_apocrypha_from_static,
    ApocrVerse,
};

use super::{resolve_book, BOOKS};

// ─── Menu ─────────────────────────────────────────────────────────────────────

static APOCR_ITEMS: &[MenuItem] = &[
    MenuItem {
        key:   "1",
        icon:  "🔍",
        label: "Search all texts",
        hint:  "Keyword, phrase, or boolean — FTS5 full-text search",
    },
    MenuItem {
        key:   "2",
        icon:  "📖",
        label: "Look up a verse",
        hint:  "e.g. 1 Enoch 6:1  ·  Jubilees 1:5  ·  2 Enoch 3:2",
    },
    MenuItem {
        key:   "3",
        icon:  "📜",
        label: "Browse a chapter",
        hint:  "Enter book + chapter: e.g. 1en 6  ·  jub 4  ·  2en 1",
    },
    MenuItem {
        key:   "4",
        icon:  "📚",
        label: "List books",
        hint:  "Show available books and chapter counts",
    },
];

static APOCR_MENU: Menu = Menu {
    title:       "✦  APOCRYPHA  (1 Enoch · 2 Enoch · Jubilees)  ✦",
    border_color: MenuColor::BrightMagenta,
    items:       APOCR_ITEMS,
    back_key:    "0",
    back_label:  "Back to main menu",
};

// ─── Entry point ──────────────────────────────────────────────────────────────

pub fn run_apocrypha_session() {
    let conn = match open_db() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", format!("  Database error: {e}").red());
            return;
        }
    };

    if !apocrypha_is_loaded(&conn) {
        println!();
        print!("{}", "  📜 Seeding Apocrypha (one-time, ~2 600 verses) …".dimmed());
        io::stdout().flush().unwrap_or(());
        match seed_apocrypha_from_static(&conn) {
            Ok(()) => println!("{}", "  done.".bright_green()),
            Err(e) => {
                println!();
                println!("{}", format!("  Seed error: {e}").red());
                return;
            }
        }
    }

    let (verse_total, book_count) = apocrypha_stats(&conn);
    println!();
    println!(
        "  {} Apocrypha — {} books · {} verses",
        "📜".bright_white(),
        book_count,
        verse_total,
    );

    loop {
        let choice = APOCR_MENU.show_and_read();
        match choice.trim() {
            "1" => search_session(&conn),
            "2" => lookup_session(&conn),
            "3" => browse_session(&conn),
            "4" => list_books(),
            "0" | "" => break,
            _ => println!("{}", "  Please enter 1–4 or 0.".yellow()),
        }
    }
}

// ─── Search ───────────────────────────────────────────────────────────────────

fn search_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  FTS5 search syntax:".bold().bright_cyan());
    println!("{}", "    watchers angels      — verses containing both words".dimmed());
    println!("{}", "    \"sons of God\"        — exact phrase".dimmed());
    println!("{}", "    enoch*               — prefix wildcard".dimmed());
    println!("{}", "    light AND NOT dark   — boolean".dimmed());
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

        match search_apocrypha(conn, query, 50) {
            Ok(results) if results.is_empty() => {
                println!("{}", "  No verses matched.".yellow());
            }
            Ok(results) => {
                let total = results.len();
                println!();
                println!(
                    "  {} {} result{}:",
                    "📜".bright_white(),
                    total.to_string().bright_yellow().bold(),
                    if total == 1 { "" } else { "s" }
                );
                paginate_verses(&results);
            }
            Err(e) => println!("{}", format!("  Search error: {e}  (check FTS5 syntax)").red()),
        }
    }
}

// ─── Verse lookup with navigation ────────────────────────────────────────────

fn lookup_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  Enter a verse reference, e.g.:".dimmed());
    println!("{}", "    1 Enoch 6:1   ·   Jubilees 1:5   ·   2 Enoch 3:2   ·   jub 10:1".dimmed());
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
            Some((book, chap, verse)) => match lookup_apocr_verse(conn, book, chap, verse) {
                Ok(Some(v)) => navigate_verses(conn, v),
                Ok(None) => println!(
                    "{}",
                    format!("  Not found: {book} {chap}:{verse}").yellow()
                ),
                Err(e) => println!("{}", format!("  DB error: {e}").red()),
            },
            None => println!(
                "{}",
                "  Could not parse — try '1 Enoch 6:1' or 'jub 1:5'".yellow()
            ),
        }
    }
}

fn navigate_verses(conn: &rusqlite::Connection, start: ApocrVerse) {
    let book      = start.book.clone();
    let mut chap  = start.chapter;
    let mut verse = start.verse;
    print_verse_card(&start);

    loop {
        let max_chap  = apocr_chapter_count(conn, &book);
        let max_verse = apocr_verse_count(conn, &book, chap);
        let at_start  = chap == 1 && verse <= 1;
        let at_end    = chap >= max_chap && verse >= max_verse;

        let mut hint = String::from("  ── ");
        if !at_start { hint.push_str("p=prev  "); }
        if !at_end   { hint.push_str("n=next  "); }
        hint.push_str("c=chapter  q=back: ");

        print!("{}", hint.dimmed());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap_or(0);

        match buf.trim().to_lowercase().as_str() {
            "n" | "" if !at_end => {
                let (nc, nv) = if verse < max_verse {
                    (chap, verse + 1)
                } else {
                    (chap + 1, 1)
                };
                match lookup_apocr_verse(conn, &book, nc, nv) {
                    Ok(Some(v)) => { chap = nc; verse = nv; print_verse_card(&v); }
                    Ok(None)    => println!("{}", "  (no next verse found)".yellow()),
                    Err(e)      => println!("{}", format!("  DB error: {e}").red()),
                }
            }
            "p" if !at_start => {
                let (nc, nv) = if verse > 1 {
                    (chap, verse - 1)
                } else if chap > 1 {
                    let pc = chap - 1;
                    let pv = apocr_verse_count(conn, &book, pc).max(1);
                    (pc, pv)
                } else {
                    (chap, verse)
                };
                match lookup_apocr_verse(conn, &book, nc, nv) {
                    Ok(Some(v)) => { chap = nc; verse = nv; print_verse_card(&v); }
                    Ok(None)    => println!("{}", "  (no prev verse found)".yellow()),
                    Err(e)      => println!("{}", format!("  DB error: {e}").red()),
                }
            }
            "c" => {
                let book_clone = book.clone();
                read_chapter(conn, &book_clone, chap, max_chap);
            }
            "q" | "" => break,
            _ => {}
        }
    }
}

// ─── Browse ───────────────────────────────────────────────────────────────────

fn browse_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  Enter book + chapter, e.g.:".dimmed());
    println!("{}", "    1 Enoch 6   ·   Jubilees 1   ·   2 Enoch 3   ·   jub 10".dimmed());
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
                let max_chap = apocr_chapter_count(conn, book);
                let mut cur  = chap;
                loop {
                    match read_chapter(conn, book, cur, max_chap) {
                        ChapNav::Next if cur < max_chap => cur += 1,
                        ChapNav::Prev if cur > 1        => cur -= 1,
                        _ => break,
                    }
                }
            }
            None => println!(
                "{}",
                "  Not recognised — try '1 Enoch 6' or 'jub 4'".yellow()
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
) -> ChapNav {
    let verses = match get_apocr_chapter(conn, book, chapter) {
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
        "  {} {}  Chapter {}  ·  {} verses",
        "📜".bright_white(),
        book.bright_yellow().bold(),
        chapter.to_string().bright_cyan(),
        verses.len(),
    );
    println!(
        "{}",
        "  ──────────────────────────────────────────────────────".dimmed()
    );

    const PAGE: usize = 5;
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
        nav.push_str("q=back: ");

        print!("  {}", nav.dimmed());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap_or(0);

        match buf.trim().to_lowercase().as_str() {
            "n" if chapter < max_chap => return ChapNav::Next,
            "p" if chapter > 1        => return ChapNav::Prev,
            "q"                       => return ChapNav::Done,
            "" if at_end              => return ChapNav::Done,
            _ if at_end               => return ChapNav::Done,
            _                         => {}
        }
    }
}

// ─── Book list ────────────────────────────────────────────────────────────────

fn list_books() {
    println!();
    println!("{}", "  ── Available Apocrypha ─────────────────────────────────────".bright_yellow());
    for (i, b) in BOOKS.iter().enumerate() {
        let line = format!("{}.  {:<12}  {}", i + 1, b.name, b.blurb);
        println!("  {}", line.bright_white());
    }
    println!();
    println!("{}", "  Short names: 1en, 2en, jub  (use in Browse or Lookup)".dimmed());
    println!();
}

// ─── Reference parsers ────────────────────────────────────────────────────────

/// Parse `"1 Enoch 6:1"` or `"jub 1:5"` → `Some((book_name, chapter, verse))`.
fn parse_verse_ref(input: &str) -> Option<(&'static str, u32, u32)> {
    let s = input.trim();

    // Find the last colon for chapter:verse split
    let colon = s.rfind(':')?;
    let verse: u32 = s[colon + 1..].trim().parse().ok()?;
    let prefix = s[..colon].trim();

    // Now split off the chapter number from the end of prefix
    let space = prefix.rfind(|c: char| c.is_ascii_whitespace())?;
    let chapter: u32 = prefix[space + 1..].trim().parse().ok()?;
    let book_str      = &prefix[..space];

    let book = resolve_book(book_str)?;
    Some((book.name, chapter, verse))
}

/// Parse `"1 Enoch 6"` or `"jub 4"` → `Some((book_name, chapter))`.
fn parse_book_chapter(input: &str) -> Option<(&'static str, u32)> {
    let s = input.trim();
    let space = s.rfind(|c: char| c.is_ascii_whitespace())?;
    let chapter: u32 = s[space + 1..].trim().parse().ok()?;
    let book_str      = &s[..space];
    let book = resolve_book(book_str)?;
    Some((book.name, chapter))
}

// ─── Display helpers ──────────────────────────────────────────────────────────

fn print_verse_card(v: &ApocrVerse) {
    let sep = "  ──────────────────────────────────────────────────────";
    println!();
    println!("{}", sep.dimmed());
    let vnum = if v.verse == 0 {
        "prologue".to_string()
    } else {
        v.verse.to_string()
    };
    println!(
        "  {} {}.{}  {}",
        v.book.bright_yellow().bold(),
        v.chapter.to_string().bright_cyan(),
        vnum.bright_cyan(),
        "R.H. Charles / sacred-texts.com".dimmed(),
    );
    println!("{}", sep.dimmed());
    for line in word_wrap(&v.text, 68) {
        println!("  {}", line.bright_white());
    }
    println!("{}", sep.dimmed());
    println!();
}

fn print_verse_full(v: &ApocrVerse) {
    let vnum = if v.verse == 0 {
        "Prol.".to_string()
    } else {
        v.verse.to_string()
    };
    let ref_tag = format!("{} {}.{}", v.book, v.chapter, vnum);
    println!("  {}", ref_tag.bright_yellow().bold());
    for line in word_wrap(&v.text, 68) {
        println!("  {}", line.bright_white());
    }
    println!();
}

fn paginate_verses(verses: &[ApocrVerse]) {
    const PAGE: usize = 5;
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
