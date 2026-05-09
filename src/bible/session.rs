//! Interactive KJV Bible search session.

use std::io::{self, Write};

use colored::*;

use crate::menu::{Menu, MenuColor, MenuItem};
use crate::persistence::{
    bible_is_loaded, bible_stats, chapter_count, get_chapter, load_text_position, lookup_verse,
    open_db, save_text_position, search_verses, seed_bible_from_static, verse_count, BibleVerse,
};
use crate::tts_reader::{
    build_chapter_speech, clean_for_tts, tts_auto_read, tts_nav_hint, tts_speak, tts_stop,
    tts_toggle_auto, tts_toggle_pause,
};

use crate::utils::read_key;

use super::resolve_book;

// ─── Menu ─────────────────────────────────────────────────────────────────────

static BIBLE_ITEMS: &[MenuItem] = &[
    MenuItem {
        key: "1",
        icon: "🔍",
        label: "Search verses",
        hint: "Keyword, phrase, or boolean — FTS5 full-text search",
    },
    MenuItem {
        key: "2",
        icon: "📖",
        label: "Look up a verse",
        hint: "Enter a reference: e.g. John 3:16 or Gen 1:1",
    },
    MenuItem {
        key: "3",
        icon: "📜",
        label: "Browse a chapter",
        hint: "Enter a reference: e.g. Psalm 23 or Rev 1",
    },
    MenuItem {
        key: "4",
        icon: "📚",
        label: "List all books",
        hint: "All 66 canonical books of the KJV",
    },
];

static BIBLE_MENU: Menu = Menu {
    title: "✦  KJV BIBLE SEARCH  (beta)  ✦",
    border_color: MenuColor::Yellow,
    items: BIBLE_ITEMS,
    back_key: "0",
    back_label: "Back to main menu",
};

// ─── Entry point ──────────────────────────────────────────────────────────────

pub fn run_bible_session() {
    let conn = match open_db() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", format!("  Database error: {e}").red());
            return;
        }
    };

    if !bible_is_loaded(&conn) {
        println!();
        print!("{}", "  📖 Seeding KJV Bible (one-time, ~30 k verses) …".dimmed());
        io::stdout().flush().unwrap_or(());
        match seed_bible_from_static(&conn) {
            Ok(()) => println!("{}", "  done.".bright_green()),
            Err(e) => {
                println!();
                println!("{}", format!("  Seed error: {e}").red());
                return;
            }
        }
    }

    let (verse_count, book_count) = bible_stats(&conn);
    println!();
    println!(
        "  {} King James Version — {} books · {} verses",
        "📖".bright_white(),
        book_count,
        verse_count,
    );

    // Offer to resume the last reading position
    if let Some((saved_book, saved_chap, saved_verse)) = load_text_position(&conn, "kjv") {
        println!(
            "  {}  Last read: {} {}:{}",
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
            if let Ok(Some(v)) = lookup_verse(&conn, &saved_book, saved_chap, saved_verse) {
                navigate_verses(&conn, v);
            }
        }
    }

    loop {
        let choice = BIBLE_MENU.show_and_read();
        match choice.trim() {
            "1" => search_session(&conn),
            "2" => lookup_session(&conn),
            "3" => chapter_session(&conn),
            "4" => list_books(),
            "0" | "" => { tts_stop(); break; }
            _ => println!("{}", "  Please enter 1–4 or 0.".yellow()),
        }
    }
}

// ─── Search session ───────────────────────────────────────────────────────────

fn search_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  FTS5 search syntax:".bold().bright_cyan());
    println!("{}", "    love mercy            — verses containing both words".dimmed());
    println!("{}", "    \"still small voice\"   — exact phrase".dimmed());
    println!("{}", "    mercif*               — prefix wildcard".dimmed());
    println!("{}", "    light AND NOT darkness — boolean".dimmed());
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

        match search_verses(conn, query, 50) {
            Ok(results) if results.is_empty() => {
                println!("{}", "  No verses matched.".yellow());
            }
            Ok(results) => {
                let total = results.len();
                println!();
                println!(
                    "  {} {} result{}:",
                    "📖".bright_white(),
                    total.to_string().bright_yellow().bold(),
                    if total == 1 { "" } else { "s" }
                );
                paginate_verses(&results);
            }
            Err(e) => {
                println!(
                    "{}",
                    format!("  Search error: {e}  (check FTS5 syntax)").red()
                );
            }
        }
    }
}

// ─── Verse lookup with verse-by-verse navigation ─────────────────────────────

fn lookup_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  Enter a verse reference, e.g.:".dimmed());
    println!("{}", "    John 3:16   · Gen 1:1   · Ps 23:1   · Rev 22:21".dimmed());
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
            Some((book, chapter, verse)) => {
                match lookup_verse(conn, book, chapter, verse) {
                    Ok(Some(v)) => navigate_verses(conn, v),
                    Ok(None) => println!(
                        "{}",
                        format!("  Not found: {book} {chapter}:{verse}").yellow()
                    ),
                    Err(e) => println!("{}", format!("  DB error: {e}").red()),
                }
            }
            None => println!(
                "{}",
                "  Could not parse reference — try 'John 3:16' or 'Gen 1:1'".yellow()
            ),
        }
    }
}

/// Show a verse and let the user step forward/back one verse at a time.
fn navigate_verses(conn: &rusqlite::Connection, start: BibleVerse) {
    // Resolve to a static book name so we can pass it to lookup functions.
    // (BibleVerse.book is a String; canonicalize it through the alias table.)
    let book: &'static str = match super::resolve_book(&start.book) {
        Some(b) => b,
        None => {
            print_verse_card(&start);
            return;
        }
    };

    let mut chap      = start.chapter;
    let mut verse     = start.verse;
    let mut cur_text  = start.text.clone();
    print_verse_card(&start);
    if tts_auto_read() { tts_speak(&clean_for_tts(&cur_text)); }

    loop {
        let max_chap  = chapter_count(conn, book);
        let max_verse = verse_count(conn, book, chap);
        let at_start  = chap == 1 && verse == 1;
        let at_end    = chap == max_chap && verse == max_verse;

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
                match lookup_verse(conn, book, nc, nv) {
                    Ok(Some(v)) => {
                        chap = nc; verse = nv;
                        cur_text = v.text.clone();
                        print_verse_card(&v);
                        if tts_auto_read() { tts_speak(&clean_for_tts(&cur_text)); }
                    }
                    Ok(None) => println!("{}", "  (no next verse found)".yellow()),
                    Err(e)   => println!("{}", format!("  DB error: {e}").red()),
                }
            }
            'p' if !at_start => {
                let (nc, nv) = if verse > 1 {
                    (chap, verse - 1)
                } else {
                    let pc = chap - 1;
                    let pv = verse_count(conn, book, pc);
                    (pc, pv)
                };
                match lookup_verse(conn, book, nc, nv) {
                    Ok(Some(v)) => {
                        chap = nc; verse = nv;
                        cur_text = v.text.clone();
                        print_verse_card(&v);
                        if tts_auto_read() { tts_speak(&clean_for_tts(&cur_text)); }
                    }
                    Ok(None) => println!("{}", "  (no prev verse found)".yellow()),
                    Err(e)   => println!("{}", format!("  DB error: {e}").red()),
                }
            }
            'c' => {
                read_chapter(conn, book, chap, max_chap);
            }
            'r' => tts_speak(&clean_for_tts(&cur_text)),
            'a' => {
                let on = tts_toggle_auto();
                println!("  {}", if on { "Auto-read ON".bright_green() } else { "Auto-read OFF".yellow() });
            }
            't' => { tts_toggle_pause(); }
            's' => tts_stop(),
            'q' | '\n' => {
                save_text_position(conn, "kjv", book, chap, verse).ok();
                break;
            }
            _ => {}
        }
    }
}

// ─── Chapter browse ───────────────────────────────────────────────────────────

fn chapter_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  Enter a chapter reference, e.g.:".dimmed());
    println!("{}", "    Psalm 23   · John 1   · Genesis 1   · Rev 22".dimmed());
    println!();

    loop {
        print!("{}", "  Chapter (blank to exit): ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or(0);
        let raw = input.trim();

        if raw.is_empty() {
            break;
        }

        match parse_chapter_ref(raw) {
            Some((book, chapter)) => {
                let max = chapter_count(conn, book);
                if max == 0 {
                    println!("{}", format!("  Book '{book}' not found.").yellow());
                    continue;
                }
                let start_chap = if chapter == 0 { 1 } else { chapter.min(max) };
                let mut cur = start_chap;
                loop {
                    match read_chapter(conn, book, cur, max) {
                        ChapterNav::Next if cur < max => cur += 1,
                        ChapterNav::Prev if cur > 1   => cur -= 1,
                        _ => break,
                    }
                }
            }
            None => println!(
                "{}",
                "  Could not parse — try 'Genesis 1' or 'John 3'".yellow()
            ),
        }
    }
}

/// Display all verses of one chapter with full text and prev/next navigation.
/// Returns the direction the user wants to move, or `Done` to exit.
enum ChapterNav { Prev, Next, Done }

fn read_chapter(conn: &rusqlite::Connection, book: &str, chap: u32, max_chap: u32) -> ChapterNav {
    let verses = match get_chapter(conn, book, chap) {
        Ok(v) if !v.is_empty() => v,
        Ok(_) => {
            println!("{}", format!("  No verses found for {book} {chap}.").yellow());
            return ChapterNav::Done;
        }
        Err(e) => {
            println!("{}", format!("  DB error: {e}").red());
            return ChapterNav::Done;
        }
    };

    println!();
    println!(
        "  {} {}  Chapter {}  ({} verses)",
        "📖".bright_white(),
        book.bright_yellow().bold(),
        chap.to_string().bright_cyan(),
        verses.len()
    );
    println!(
        "{}",
        "  ──────────────────────────────────────────────────────".dimmed()
    );

    // Auto-read the whole chapter if enabled
    if tts_auto_read() {
        let texts: Vec<&str> = verses.iter().map(|v| v.text.as_str()).collect();
        tts_speak(&build_chapter_speech(book, chap, &texts));
    }

    const PAGE: usize = 5;
    let total = verses.len();
    let mut start = 0;

    loop {
        let end = (start + PAGE).min(total);
        for v in &verses[start..end] {
            print_verse_full(v);
        }
        start = end;

        let at_end = start >= total;
        let mut nav_hint = String::new();
        if at_end {
            nav_hint.push_str("── End of chapter ── ");
        } else {
            nav_hint.push_str(&format!("── {start}/{total} shown ── Enter=more  "));
        }
        if chap > 1        { nav_hint.push_str("p=prev  "); }
        if chap < max_chap { nav_hint.push_str("n=next  "); }
        nav_hint.push_str(&tts_nav_hint("s"));
        nav_hint.push_str("q=back");

        print!("  {}", nav_hint.dimmed());

        match read_key() {
            'n' if chap < max_chap => return ChapterNav::Next,
            'p' if chap > 1       => return ChapterNav::Prev,
            'r' => {
                let texts: Vec<&str> = verses.iter().map(|v| v.text.as_str()).collect();
                tts_speak(&build_chapter_speech(book, chap, &texts));
            }
            'a' => {
                let on = tts_toggle_auto();
                println!("  {}", if on { "Auto-read ON".bright_green() } else { "Auto-read OFF".yellow() });
            }
            't' => { tts_toggle_pause(); }
            's' => tts_stop(),
            'q'            => return ChapterNav::Done,
            '\n' if at_end => return ChapterNav::Done,
            _ => {
                if at_end { return ChapterNav::Done; }
            }
        }
    }
}

// ─── Book list ────────────────────────────────────────────────────────────────

fn list_books() {
    let books: Vec<&str> = super::all_books().collect();
    println!();
    println!("{}", "  ── Old Testament (39 books) ──────────────────────────────".bright_yellow());
    for (i, b) in books[..39].iter().enumerate() {
        let num = format!("{:>2}. {:<20}", i + 1, b);
        if (i + 1) % 3 == 0 {
            println!("  {}", num.bright_white());
        } else {
            print!("  {}", num.bright_white());
        }
    }
    println!();
    println!("{}", "  ── New Testament (27 books) ──────────────────────────────".bright_cyan());
    for (i, b) in books[39..].iter().enumerate() {
        let num = format!("{:>2}. {:<20}", i + 40, b);
        if (i + 1) % 3 == 0 {
            println!("  {}", num.bright_white());
        } else {
            print!("  {}", num.bright_white());
        }
    }
    println!();
    println!("{}", "  (Use abbreviations like Gen, Exo, Ps, Matt, Rev in searches)".dimmed());
    println!();
}

// ─── Reference parsers ────────────────────────────────────────────────────────

/// Parse `"John 3:16"` → `Some(("John", 3, 16))`.
/// Handles multi-word book names and common abbreviations.
fn parse_verse_ref(input: &str) -> Option<(&'static str, u32, u32)> {
    // Look for trailing chapter:verse pattern
    let re_cv = regex_cv(input)?;
    let (book_part, chapter, verse) = re_cv;
    let book = resolve_book(book_part.trim())?;
    Some((book, chapter, verse))
}

/// Parse `"Psalm 23"` or `"Genesis 1"` → `Some(("Psalms", 23))`.
fn parse_chapter_ref(input: &str) -> Option<(&'static str, u32)> {
    // Split on last whitespace-separated number
    let parts: Vec<&str> = input.rsplitn(2, |c: char| c.is_ascii_whitespace()).collect();
    if parts.len() == 2 {
        let chapter: u32 = parts[0].trim().parse().ok()?;
        let book = resolve_book(parts[1].trim())?;
        return Some((book, chapter));
    }
    None
}

/// Extract (book_part_str, chapter, verse) from "Book ch:v".
/// Returns None if the pattern isn't found.
fn regex_cv(input: &str) -> Option<(&str, u32, u32)> {
    // Find last occurrence of digits:digits at the end (possibly after whitespace)
    let trimmed = input.trim();
    // Locate the colon separating chapter and verse
    let colon_pos = trimmed.rfind(':')?;
    let verse_str = trimmed[colon_pos + 1..].trim();
    let verse: u32 = verse_str.parse().ok()?;

    // Everything before the colon — find the last whitespace before the chapter number
    let before_colon = trimmed[..colon_pos].trim();
    let last_space = before_colon.rfind(|c: char| c.is_ascii_whitespace())?;
    let chapter_str = before_colon[last_space + 1..].trim();
    let chapter: u32 = chapter_str.parse().ok()?;

    let book_part = before_colon[..last_space].trim();
    Some((book_part, chapter, verse))
}

// ─── Display helpers ──────────────────────────────────────────────────────────

/// Full card display — used for single-verse lookup and verse-by-verse navigation.
fn print_verse_card(v: &BibleVerse) {
    let sep = "  ──────────────────────────────────────────────────────";
    println!();
    println!("{}", sep.dimmed());
    println!(
        "  {} {}:{} {}",
        v.book.bright_yellow().bold(),
        v.chapter.to_string().bright_cyan(),
        v.verse.to_string().bright_cyan(),
        "KJV".dimmed(),
    );
    println!("{}", sep.dimmed());
    for line in word_wrap(&v.text, 68) {
        println!("  {}", line.bright_white());
    }
    println!("{}", sep.dimmed());
    println!();
}

/// Compact full-text row — used in paginated lists (search results, chapter view).
fn print_verse_full(v: &BibleVerse) {
    let ref_tag = format!("{}  {}:{}", v.book, v.chapter, v.verse);
    println!("  {}  {}", ref_tag.bright_yellow().bold(), "KJV".dimmed());
    for line in word_wrap(&v.text, 68) {
        println!("  {}", line.bright_white());
    }
    println!();
}

/// Paginate a verse list with full text, 5 verses per page.
fn paginate_verses(verses: &[BibleVerse]) {
    const PAGE: usize = 5;
    let mut start = 0;
    let total = verses.len();
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
            format!(
                "  ── {} of {} shown — Enter=more, q=stop: ",
                start, total
            )
            .dimmed()
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

/// Naive word-wrapper: splits text into lines of at most `max_cols` chars.
fn word_wrap(text: &str, max_cols: usize) -> Vec<String> {
    let mut lines = Vec::new();
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
