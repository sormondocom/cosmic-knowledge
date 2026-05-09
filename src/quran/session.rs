//! Interactive Quran session — Pickthall English translation.

use std::io::{self, Write};

use colored::*;

use crate::menu::{Menu, MenuColor, MenuItem};
use crate::persistence::{
    get_surah, lookup_ayah, open_db, quran_is_loaded, quran_stats, search_quran,
    seed_quran_from_static, QuranVerse,
};

use super::{resolve_surah, SURAHS};

// ─── Menu ─────────────────────────────────────────────────────────────────────

static QURAN_ITEMS: &[MenuItem] = &[
    MenuItem {
        key: "1",
        icon: "🔍",
        label: "Search ayahs",
        hint: "Keyword, phrase, or boolean — FTS5 full-text search",
    },
    MenuItem {
        key: "2",
        icon: "📖",
        label: "Look up an ayah",
        hint: "Enter a reference: e.g. 2:255 or Al-Baqara 255",
    },
    MenuItem {
        key: "3",
        icon: "📜",
        label: "Browse a surah",
        hint: "Enter a number or name: e.g. 36 or Ya-Sin",
    },
    MenuItem {
        key: "4",
        icon: "📚",
        label: "List all surahs",
        hint: "All 114 surahs with English meanings",
    },
];

static QURAN_MENU: Menu = Menu {
    title: "✦  QURAN SEARCH  (Pickthall)  ✦",
    border_color: MenuColor::Green,
    items: QURAN_ITEMS,
    back_key: "0",
    back_label: "Back to main menu",
};

// ─── Entry point ──────────────────────────────────────────────────────────────

pub fn run_quran_session() {
    let conn = match open_db() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", format!("  Database error: {e}").red());
            return;
        }
    };

    if !quran_is_loaded(&conn) {
        println!();
        print!("{}", "  📿 Seeding Quran (one-time, ~6 236 ayahs) …".dimmed());
        io::stdout().flush().unwrap_or(());
        match seed_quran_from_static(&conn) {
            Ok(()) => println!("{}", "  done.".bright_green()),
            Err(e) => {
                println!();
                println!("{}", format!("  Seed error: {e}").red());
                return;
            }
        }
    }

    let (ayah_count_total, surah_count) = quran_stats(&conn);
    println!();
    println!(
        "  {} Quran (Pickthall) — {} surahs · {} ayahs",
        "📿".bright_white(),
        surah_count,
        ayah_count_total,
    );

    loop {
        let choice = QURAN_MENU.show_and_read();
        match choice.trim() {
            "1" => search_session(&conn),
            "2" => lookup_session(&conn),
            "3" => surah_browse_session(&conn),
            "4" => list_surahs(),
            "0" | "" => break,
            _ => println!("{}", "  Please enter 1–4 or 0.".yellow()),
        }
    }
}

// ─── Search ───────────────────────────────────────────────────────────────────

fn search_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  FTS5 search syntax:".bold().bright_cyan());
    println!("{}", "    mercy guidance       — ayahs containing both words".dimmed());
    println!("{}", "    \"straight path\"      — exact phrase".dimmed());
    println!("{}", "    mercif*              — prefix wildcard".dimmed());
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

        match search_quran(conn, query, 50) {
            Ok(results) if results.is_empty() => {
                println!("{}", "  No ayahs matched.".yellow());
            }
            Ok(results) => {
                let total = results.len();
                println!();
                println!(
                    "  {} {} result{}:",
                    "📿".bright_white(),
                    total.to_string().bright_yellow().bold(),
                    if total == 1 { "" } else { "s" }
                );
                paginate_ayahs(&results);
            }
            Err(e) => {
                println!("{}", format!("  Search error: {e}  (check FTS5 syntax)").red());
            }
        }
    }
}

// ─── Ayah lookup with verse-by-verse navigation ───────────────────────────────

fn lookup_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  Enter an ayah reference, e.g.:".dimmed());
    println!("{}", "    2:255   · Al-Fatiha 1   · 36:1   · 112:1".dimmed());
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

        match parse_ayah_ref(raw) {
            Some((surah, ayah)) => match lookup_ayah(conn, surah, ayah) {
                Ok(Some(v)) => navigate_ayahs(conn, v),
                Ok(None) => println!(
                    "{}",
                    format!("  Not found: {surah}:{ayah}").yellow()
                ),
                Err(e) => println!("{}", format!("  DB error: {e}").red()),
            },
            None => println!(
                "{}",
                "  Could not parse — try '2:255' or 'Al-Baqara 255'".yellow()
            ),
        }
    }
}

fn navigate_ayahs(conn: &rusqlite::Connection, start: QuranVerse) {
    let mut surah = start.surah;
    let mut ayah  = start.ayah;
    print_ayah_card(&start);

    loop {
        let surah_info  = SURAHS.iter().find(|s| s.number == surah);
        let max_ayah    = surah_info.map_or(0, |s| s.ayahs);
        let at_start    = surah == 1 && ayah == 1;
        let at_end      = surah == 114 && ayah == max_ayah;

        let mut hint = String::from("  ── ");
        if !at_start { hint.push_str("p=prev  "); }
        if !at_end   { hint.push_str("n=next  "); }
        hint.push_str("s=surah  q=back: ");

        print!("{}", hint.dimmed());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap_or(0);

        match buf.trim().to_lowercase().as_str() {
            "n" | "" if !at_end => {
                let (ns, na) = if ayah < max_ayah {
                    (surah, ayah + 1)
                } else {
                    (surah + 1, 1)
                };
                match lookup_ayah(conn, ns, na) {
                    Ok(Some(v)) => { surah = ns; ayah = na; print_ayah_card(&v); }
                    Ok(None)    => println!("{}", "  (no next ayah found)".yellow()),
                    Err(e)      => println!("{}", format!("  DB error: {e}").red()),
                }
            }
            "p" if !at_start => {
                let (ns, na) = if ayah > 1 {
                    (surah, ayah - 1)
                } else {
                    let ps = surah - 1;
                    let pa = SURAHS.iter().find(|s| s.number == ps).map_or(1, |s| s.ayahs);
                    (ps, pa)
                };
                match lookup_ayah(conn, ns, na) {
                    Ok(Some(v)) => { surah = ns; ayah = na; print_ayah_card(&v); }
                    Ok(None)    => println!("{}", "  (no prev ayah found)".yellow()),
                    Err(e)      => println!("{}", format!("  DB error: {e}").red()),
                }
            }
            "s" => {
                let max_surah = SURAHS.last().map_or(114, |s| s.number);
                read_surah(conn, surah, max_surah);
            }
            "q" | "" => break,
            _ => {}
        }
    }
}

// ─── Surah browse ─────────────────────────────────────────────────────────────

fn surah_browse_session(conn: &rusqlite::Connection) {
    println!();
    println!("{}", "  Enter a surah number or name, e.g.:".dimmed());
    println!("{}", "    36   · Al-Fatiha   · rahman   · 2".dimmed());
    println!();

    loop {
        print!("{}", "  Surah (blank to exit): ".bold().cyan());
        io::stdout().flush().unwrap_or(());
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or(0);
        let raw = input.trim();

        if raw.is_empty() {
            break;
        }

        match resolve_surah(raw) {
            Some(su) => {
                let mut cur = su.number;
                loop {
                    match read_surah(conn, cur, 114) {
                        SurahNav::Next if cur < 114 => cur += 1,
                        SurahNav::Prev if cur > 1   => cur -= 1,
                        _ => break,
                    }
                }
            }
            None => println!(
                "{}",
                "  Surah not found — try a number (1–114) or name.".yellow()
            ),
        }
    }
}

// ─── Surah reader ─────────────────────────────────────────────────────────────

enum SurahNav { Prev, Next, Done }

fn read_surah(conn: &rusqlite::Connection, surah: u32, max_surah: u32) -> SurahNav {
    let ayahs = match get_surah(conn, surah) {
        Ok(v) if !v.is_empty() => v,
        Ok(_) => {
            println!("{}", format!("  No ayahs found for surah {surah}.").yellow());
            return SurahNav::Done;
        }
        Err(e) => {
            println!("{}", format!("  DB error: {e}").red());
            return SurahNav::Done;
        }
    };

    let su_info = SURAHS.iter().find(|s| s.number == surah);
    let name    = su_info.map_or("", |s| s.name);
    let english = su_info.map_or("", |s| s.english);

    println!();
    println!(
        "  {} Surah {}  ·  {}  ({})  ·  {} ayahs",
        "📿".bright_white(),
        surah.to_string().bright_cyan(),
        name.bright_yellow().bold(),
        english.dimmed(),
        ayahs.len()
    );
    println!(
        "{}",
        "  ──────────────────────────────────────────────────────".dimmed()
    );

    const PAGE: usize = 5;
    let total   = ayahs.len();
    let mut start = 0;

    loop {
        let end = (start + PAGE).min(total);
        for v in &ayahs[start..end] {
            print_ayah_full(v);
        }
        start = end;

        let at_end = start >= total;
        let mut nav = String::new();
        if at_end { nav.push_str("── End of surah ── "); } else { nav.push_str(&format!("── {start}/{total} ── Enter=more  ")); }
        if surah > 1        { nav.push_str("p=prev  "); }
        if surah < max_surah{ nav.push_str("n=next  "); }
        nav.push_str("q=back: ");

        print!("  {}", nav.dimmed());
        io::stdout().flush().unwrap_or(());
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap_or(0);

        match buf.trim().to_lowercase().as_str() {
            "n" if surah < max_surah => return SurahNav::Next,
            "p" if surah > 1        => return SurahNav::Prev,
            "q"                     => return SurahNav::Done,
            "" if at_end            => return SurahNav::Done,
            _ if at_end             => return SurahNav::Done,
            _                       => {}
        }
    }
}

// ─── Surah list ───────────────────────────────────────────────────────────────

fn list_surahs() {
    println!();
    println!("{}", "  ── Surahs 1–57  (Meccan origins) ──────────────────────────".bright_yellow());
    for su in &SURAHS[..57] {
        let line = format!("{:>3}. {:<18} ({})", su.number, su.name, su.english);
        println!("  {}", line.bright_white());
    }
    println!();
    println!("{}", "  ── Surahs 58–114  (Medinan origins) ───────────────────────".bright_cyan());
    for su in &SURAHS[57..] {
        let line = format!("{:>3}. {:<18} ({})", su.number, su.name, su.english);
        println!("  {}", line.bright_white());
    }
    println!();
    println!("{}", "  (Use numbers or names in Browse mode — e.g. 36, Ya-Sin, rahman)".dimmed());
    println!();
}

// ─── Reference parsers ────────────────────────────────────────────────────────

/// Parse `"2:255"` or `"Al-Baqara 255"` → `Some((surah, ayah))`.
fn parse_ayah_ref(input: &str) -> Option<(u32, u32)> {
    let s = input.trim();

    // Numeric surah:ayah
    if let Some(colon) = s.find(':') {
        let surah: u32 = s[..colon].trim().parse().ok()?;
        let ayah: u32  = s[colon + 1..].trim().parse().ok()?;
        if surah >= 1 && surah <= 114 {
            return Some((surah, ayah));
        }
    }

    // "Name number"
    let parts: Vec<&str> = s.rsplitn(2, |c: char| c.is_ascii_whitespace()).collect();
    if parts.len() == 2 {
        let ayah: u32 = parts[0].trim().parse().ok()?;
        let su = resolve_surah(parts[1].trim())?;
        return Some((su.number, ayah));
    }

    None
}

// ─── Display helpers ──────────────────────────────────────────────────────────

fn print_ayah_card(v: &QuranVerse) {
    let sep = "  ──────────────────────────────────────────────────────";
    println!();
    println!("{}", sep.dimmed());
    println!(
        "  {} {}:{}  {}  {}",
        v.surah_name.bright_yellow().bold(),
        v.surah.to_string().bright_cyan(),
        v.ayah.to_string().bright_cyan(),
        "·".dimmed(),
        "Pickthall".dimmed(),
    );
    println!("{}", sep.dimmed());
    for line in word_wrap(&v.text, 68) {
        println!("  {}", line.bright_white());
    }
    println!("{}", sep.dimmed());
    println!();
}

fn print_ayah_full(v: &QuranVerse) {
    let ref_tag = format!("{}  {}:{}", v.surah_name, v.surah, v.ayah);
    println!("  {}  {}", ref_tag.bright_yellow().bold(), "Pickthall".dimmed());
    for line in word_wrap(&v.text, 68) {
        println!("  {}", line.bright_white());
    }
    println!();
}

fn paginate_ayahs(ayahs: &[QuranVerse]) {
    const PAGE: usize = 5;
    let mut start = 0;
    let total     = ayahs.len();
    loop {
        let end = (start + PAGE).min(total);
        for v in &ayahs[start..end] {
            print_ayah_full(v);
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
