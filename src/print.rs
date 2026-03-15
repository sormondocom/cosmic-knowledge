//! Print module — render a numerology report as HTML or PDF and send to output.
//!
//! Strategy:
//!  1. Determine a culturally-appropriate visual theme from the active systems.
//!  2. Build a styled, single-page HTML document using that theme.
//!  3. For **printing**: write to a temp file and invoke the OS default handler.
//!  4. For **PDF export**: attempt HTML→PDF via a detected browser binary
//!     (Chrome/Chromium headless, then Edge on Windows).  Falls back to saving
//!     the HTML and instructing the user to print-to-PDF manually.
//!
//! ## Cultural themes
//!
//! Each numerological tradition receives typographically and chromatically
//! appropriate styling:
//!
//! | System(s)           | Theme                             |
//! |---------------------|-----------------------------------|
//! | Hebrew Gematria     | Kabbalistic parchment (navy/gold) |
//! | Pythagorean         | Classical Greek marble            |
//! | Chaldean            | Babylonian lapis & gold           |
//! | Greek Isopsephy     | Neoplatonic dark parchment        |
//! | Agrippan            | Renaissance grimoire              |
//! | Ordinal (Simple/Rev)| Modern typographic                |
//! | Abjad               | Islamic manuscript (green/gold)   |
//! | Enochian            | Elizabethan angelic (navy/crimson)|
//! | Chinese cosmologies | Taoist lacquerware (red/gold)     |
//! | African traditions  | Earth-toned Yoruba/Akan           |
//! | Mixed / default     | Celestial purple (app default)    |

use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use colored::*;

use crate::enochian::{aethyr_lookup, enochian_meaning, enochian_angelic_message};
use crate::numerology::{
    numerology, meaning_of, isopsephy_meaning, abjad_meaning,
    angelic_message, master_numbers_message, check_special_sequences, get_calculation_breakdown,
};
use crate::reports::strip_leading_emoji;

// ─── Cultural theme ───────────────────────────────────────────────────────────

/// Visual theme applied to the HTML/PDF report.
struct CulturalTheme {
    /// CSS `@import` or `<link>` tags for web fonts (may be empty for offline safety).
    font_imports: &'static str,
    /// Inline `<style>` block: only CSS custom-property definitions + font rules.
    css_vars:     &'static str,
    /// Human-readable tradition name used in the report footer.
    tradition:    &'static str,
}

/// Select the most culturally-appropriate theme for a set of active systems.
///
/// When multiple traditions are mixed, returns the default "Celestial" theme.
fn theme_for_systems(systems: &[&str]) -> CulturalTheme {
    // Classify each system into a broad tradition group.
    let mut groups: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for s in systems {
        let g = match *s {
            "Hebrew Gematria"               => "hebrew",
            "Pythagorean"                   => "pythagorean",
            "Chaldean"                      => "chaldean",
            "Greek Isopsephy"               => "greek",
            "Agrippan"                      => "agrippan",
            "Simple Ordinal" | "Reverse Ordinal" => "ordinal",
            "Abjad"                         => "abjad",
            s if s.starts_with("Enochian") => "enochian",
            // Chinese/African cosmology systems come through cosmology module
            s if s.starts_with("Nine Star") || s.starts_with("Wu Xing")
                || s.starts_with("Ba Gua") || s.starts_with("Chinese") => "chinese",
            s if s.starts_with("Ifa") || s.starts_with("Yoruba")
                || s.starts_with("Akan") || s.starts_with("Kemetic")
                || s.starts_with("African") => "african",
            _ => "default",
        };
        groups.insert(g);
    }

    // Only apply a specialised theme when all active systems belong to the same tradition.
    if groups.len() == 1 {
        match *groups.iter().next().unwrap() {
            "hebrew" => return CulturalTheme {
                font_imports: r#"<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=Frank+Ruhl+Libre:wght@400;700&family=Rubik&display=swap" rel="stylesheet">"#,
                css_vars: r#"
    :root {
      --bg:          #0D0D2B;
      --text:        #F5F0E8;
      --accent:      #C9A84C;
      --header-bg:   #1B2A6B;
      --header-text: #F5F0E8;
      --row-even:    #14143A;
      --border:      #2A3A7A;
      --sys-color:   #C9A84C;
      --calc-color:  #9A9AB0;
      --master-color:#E8A020;
      --aethyr-color:#4A7FA5;
      --special-color:#CC8822;
    }
    body { font-family: 'Frank Ruhl Libre', 'Noto Serif Hebrew', 'Times New Roman', serif; }
    .header h1 { font-family: 'Rubik', 'Frank Ruhl Libre', serif; letter-spacing: 0.1em; }"#,
                tradition: "Hebrew Kabbalistic Tradition",
            },

            "pythagorean" => return CulturalTheme {
                font_imports: r#"<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=IM+Fell+English:ital@0;1&display=swap" rel="stylesheet">"#,
                css_vars: r#"
    :root {
      --bg:          #F2EDD7;
      --text:        #2C2C2C;
      --accent:      #B8960C;
      --header-bg:   #1A3A5C;
      --header-text: #F2EDD7;
      --row-even:    #E8E0C8;
      --border:      #9E9E8E;
      --sys-color:   #1A3A5C;
      --calc-color:  #666;
      --master-color:#8B0000;
      --aethyr-color:#005580;
      --special-color:#b8600a;
    }
    body { font-family: 'IM Fell English', 'Palatino Linotype', 'Book Antiqua', Georgia, serif; }"#,
                tradition: "Pythagorean / Western Tradition",
            },

            "chaldean" => return CulturalTheme {
                font_imports: r#"<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=Cinzel:wght@400;700&display=swap" rel="stylesheet">"#,
                css_vars: r#"
    :root {
      --bg:          #1C2E6B;
      --text:        #EDE5C8;
      --accent:      #D4A017;
      --header-bg:   #8B4513;
      --header-text: #EDE5C8;
      --row-even:    #243580;
      --border:      #C4A882;
      --sys-color:   #D4A017;
      --calc-color:  #AAA090;
      --master-color:#E8C060;
      --aethyr-color:#2E8B8B;
      --special-color:#D4A017;
    }
    body { font-family: 'Cinzel', 'Palatino Linotype', Georgia, serif; }"#,
                tradition: "Chaldean / Babylonian Tradition",
            },

            "greek" => return CulturalTheme {
                font_imports: r#"<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=Gentium+Plus:ital@0;1&display=swap" rel="stylesheet">"#,
                css_vars: r#"
    :root {
      --bg:          #2A1F0E;
      --text:        #E8D5A3;
      --accent:      #C8A84B;
      --header-bg:   #6B3FA0;
      --header-text: #E8D5A3;
      --row-even:    #362810;
      --border:      #A0752A;
      --sys-color:   #C8A84B;
      --calc-color:  #9A8A6A;
      --master-color:#E8C080;
      --aethyr-color:#2E7D7D;
      --special-color:#C8A84B;
    }
    body { font-family: 'Gentium Plus', 'GFS Neohellenic', 'Palatino Linotype', Georgia, serif; }"#,
                tradition: "Greek Isopsephy — Neoplatonic Tradition",
            },

            "agrippan" => return CulturalTheme {
                font_imports: r#"<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=IM+Fell+English:ital@0;1&display=swap" rel="stylesheet">"#,
                css_vars: r#"
    :root {
      --bg:          #1A1408;
      --text:        #EDE5C0;
      --accent:      #B8860B;
      --header-bg:   #3C0000;
      --header-text: #EDE5C0;
      --row-even:    #231C0A;
      --border:      #7A5C3A;
      --sys-color:   #CC8833;
      --calc-color:  #8A7A6A;
      --master-color:#CC2200;
      --aethyr-color:#2E6B3E;
      --special-color:#CC8822;
    }
    body { font-family: 'IM Fell English', 'Garamond', 'Palatino Linotype', Georgia, serif; }"#,
                tradition: "Agrippan / Renaissance Occult Tradition",
            },

            "ordinal" => return CulturalTheme {
                font_imports: r#"<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=Montserrat:wght@400;700&family=Source+Code+Pro&display=swap" rel="stylesheet">"#,
                css_vars: r#"
    :root {
      --bg:          #F8F8F6;
      --text:        #1A1A1A;
      --accent:      #3A7CA5;
      --header-bg:   #2C3E50;
      --header-text: #F8F8F6;
      --row-even:    #F0F0EE;
      --border:      #D5D5D5;
      --sys-color:   #2C3E50;
      --calc-color:  #666;
      --master-color:#8B0000;
      --aethyr-color:#005580;
      --special-color:#b8600a;
    }
    body { font-family: 'Montserrat', 'Segoe UI', Arial, sans-serif; }
    td.num, td.root { font-family: 'Source Code Pro', 'Courier New', monospace; }"#,
                tradition: "Modern English Ordinal",
            },

            "abjad" => return CulturalTheme {
                font_imports: r#"<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=Amiri:ital,wght@0,400;0,700;1,400&display=swap" rel="stylesheet">"#,
                css_vars: r#"
    :root {
      --bg:          #0D3B2E;
      --text:        #F0EAD6;
      --accent:      #C9A84C;
      --header-bg:   #1B3A8A;
      --header-text: #F0EAD6;
      --row-even:    #0F4A3A;
      --border:      #E8DCC8;
      --sys-color:   #C9A84C;
      --calc-color:  #A0906A;
      --master-color:#E8C060;
      --aethyr-color:#2E8B8B;
      --special-color:#C9A84C;
    }
    body { font-family: 'Amiri', 'Scheherazade New', 'Noto Naskh Arabic',
                         'Traditional Arabic', 'Times New Roman', serif; }"#,
                tradition: "Abjad — Arabic/Islamic Tradition",
            },

            "enochian" => return CulturalTheme {
                font_imports: r#"<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=Cardo:ital@0;1&display=swap" rel="stylesheet">"#,
                css_vars: r#"
    :root {
      --bg:          #0F1C3F;
      --text:        #E8ECF0;
      --accent:      #C8A84B;
      --header-bg:   #8B0000;
      --header-text: #E8ECF0;
      --row-even:    #162244;
      --border:      #1A2744;
      --sys-color:   #C8A84B;
      --calc-color:  #8A9AB0;
      --master-color:#E8A020;
      --aethyr-color:#4A8EB5;
      --special-color:#C8A84B;
    }
    body { font-family: 'Cardo', 'IM Fell English', 'Palatino Linotype', Georgia, serif; }"#,
                tradition: "Enochian — John Dee's Angelic System (1582–1587)",
            },

            "chinese" => return CulturalTheme {
                font_imports: r#"<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=Noto+Serif+SC:wght@400;700&display=swap" rel="stylesheet">"#,
                css_vars: r#"
    :root {
      --bg:          #0D0D0D;
      --text:        #F0E6D3;
      --accent:      #D4AF37;
      --header-bg:   #C0392B;
      --header-text: #F0E6D3;
      --row-even:    #141414;
      --border:      #8B6914;
      --sys-color:   #D4AF37;
      --calc-color:  #9A8A70;
      --master-color:#E8A020;
      --aethyr-color:#2E8B57;
      --special-color:#D4AF37;
    }
    body { font-family: 'Noto Serif SC', 'Source Han Serif SC', 'SimSun',
                         'STSong', Georgia, serif; }"#,
                tradition: "Chinese Cosmological Traditions",
            },

            "african" => return CulturalTheme {
                font_imports: r#"<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=Gentium+Plus:ital@0;1&display=swap" rel="stylesheet">"#,
                css_vars: r#"
    :root {
      --bg:          #2A1505;
      --text:        #F5E6C8;
      --accent:      #E8A020;
      --header-bg:   #1E5C2A;
      --header-text: #F5E6C8;
      --row-even:    #351A08;
      --border:      #A0522D;
      --sys-color:   #E8A020;
      --calc-color:  #9A7A5A;
      --master-color:#FFB300;
      --aethyr-color:#2A3D7A;
      --special-color:#E8A020;
    }
    body { font-family: 'Gentium Plus', 'Andika', 'Noto Serif',
                         Georgia, serif; }"#,
                tradition: "African Traditions — Yoruba Ifá · Akan · Kemetic",
            },

            _ => {}
        }
    }

    // Default: the app's existing celestial purple theme.
    CulturalTheme {
        font_imports: "",
        css_vars: r#"
    :root {
      --bg:          #fff;
      --text:        #1a1a2e;
      --accent:      #4a0e8f;
      --header-bg:   #4a0e8f;
      --header-text: #fff;
      --row-even:    #f7f3ff;
      --border:      #e0d7f0;
      --sys-color:   #4a0e8f;
      --calc-color:  #555;
      --master-color:#8b0000;
      --aethyr-color:#005580;
      --special-color:#b8600a;
    }
    body { font-family: 'Georgia', 'Times New Roman', serif; }"#,
        tradition: "Celestial Numerology",
    }
}

// ─── Public entry points ──────────────────────────────────────────────────────

/// Build an HTML report for `word`, apply a cultural theme, and send to the
/// system's default printer.
pub fn print_numerology_report(word: &str, active_systems: &[&str]) {
    println!("{}", "  Preparing print report…".bright_cyan());

    let theme = theme_for_systems(active_systems);
    let html  = build_html_report(word, active_systems, &theme);

    fs::create_dir_all("exports").ok();
    let tmp_path = format!("exports/print_{}.html", word.to_lowercase());
    if let Err(e) = fs::write(&tmp_path, &html) {
        println!("{}", format!("  Could not write temp file: {}", e).bright_red());
        return;
    }

    let abs_path = canonicalize_path(&tmp_path);

    match send_to_printer(&abs_path) {
        Ok(_)  => println!("{}", "  Report sent to default printer.".bright_green()),
        Err(e) => {
            println!("{}", format!("  Could not print automatically: {}", e).yellow());
            println!("{}", format!("      HTML saved to: {}", tmp_path).dimmed());
            println!("{}", "      Open that file in a browser and print from there.".dimmed());
        }
    }
}

/// Build an HTML report, then attempt to export it as a PDF via a detected
/// browser binary (Chrome/Chromium headless, then Edge on Windows).
///
/// The PDF is saved to `exports/<word>_<tradition>.pdf`.
/// Falls back to saving the HTML with instructions if no browser is found.
pub fn export_pdf_report(word: &str, active_systems: &[&str]) {
    println!("{}", "  Preparing PDF export…".bright_cyan());

    let theme     = theme_for_systems(active_systems);
    let html      = build_html_report(word, active_systems, &theme);
    let stem      = word.to_lowercase();

    fs::create_dir_all("exports").ok();
    let html_path = format!("exports/report_{}.html", stem);
    let pdf_path  = format!("exports/report_{}.pdf", stem);

    if let Err(e) = fs::write(&html_path, &html) {
        println!("{}", format!("  Could not write HTML file: {}", e).bright_red());
        return;
    }

    let abs_html = canonicalize_path(&html_path);
    let abs_pdf  = canonicalize_path(&pdf_path);

    match html_to_pdf(&abs_html, &abs_pdf) {
        Ok(_)  => {
            println!("{}", format!("  PDF saved to: {}", pdf_path).bright_green());
            // Clean up the intermediate HTML.
            fs::remove_file(&html_path).ok();
        }
        Err(e) => {
            println!("{}", format!("  PDF conversion unavailable: {}", e).yellow());
            println!("{}", format!("  HTML saved to: {}", html_path).dimmed());
            println!("{}", "  To create a PDF: open the HTML in Chrome/Edge and use".dimmed());
            println!("{}", "  File → Print → Save as PDF.".dimmed());
        }
    }
}

// ─── PDF conversion ───────────────────────────────────────────────────────────

/// Attempt to find a Chromium-based browser binary on the host system.
///
/// Search order: Chrome (Linux/macOS names), then Chrome Windows paths,
/// then Edge Windows path.  Returns `None` if nothing is found.
fn find_browser_binary() -> Option<PathBuf> {
    let candidates: &[&str] = &[
        // Linux
        "google-chrome",
        "google-chrome-stable",
        "chromium",
        "chromium-browser",
        // macOS
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
        // Windows — Chrome
        r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        // Windows — Edge (pre-installed on Windows 10+)
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
    ];

    for candidate in candidates {
        let p = Path::new(candidate);
        // For absolute paths check existence directly; for bare names try which/where.
        if p.is_absolute() {
            if p.exists() { return Some(p.to_path_buf()); }
        } else {
            // Try resolving via PATH (works on Unix; `where` on Windows handled below).
            if Command::new(candidate).arg("--version").output().is_ok() {
                return Some(PathBuf::from(candidate));
            }
        }
    }
    None
}

/// Convert an HTML file to PDF using a Chromium-based browser in headless mode.
fn html_to_pdf(html_abs: &str, pdf_abs: &str) -> io::Result<()> {
    let browser = find_browser_binary().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound,
            "No Chrome/Chromium/Edge binary found")
    })?;

    // Build the file URI from the absolute path.
    let file_uri = if html_abs.starts_with('/') {
        format!("file://{html_abs}")
    } else {
        // Windows absolute path — replace backslashes.
        format!("file:///{}", html_abs.replace('\\', "/"))
    };

    let status = Command::new(&browser)
        .args([
            "--headless",
            "--disable-gpu",
            "--no-sandbox",
            "--run-all-compositor-stages-before-draw",
            "--print-to-pdf-no-header",
            &format!("--print-to-pdf={pdf_abs}"),
            &file_uri,
        ])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other,
            "Browser headless PDF conversion returned non-zero exit code"))
    }
}

// ─── OS print dispatch ────────────────────────────────────────────────────────

fn send_to_printer(abs_path: &str) -> io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        let status = Command::new("rundll32.exe")
            .args(["mshtml.dll,PrintHTML", abs_path])
            .status()?;
        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "rundll32 PrintHTML returned non-zero exit code",
            ));
        }
        return Ok(());
    }

    #[cfg(target_os = "macos")]
    {
        let status = Command::new("lpr")
            .args(["-o", "media=A4", abs_path])
            .status()?;
        if !status.success() {
            return Err(io::Error::new(io::ErrorKind::Other, "lpr returned non-zero"));
        }
        return Ok(());
    }

    #[cfg(target_os = "linux")]
    {
        if Command::new("lpr").arg(abs_path).status().is_ok() {
            return Ok(());
        }
        let status = Command::new("xdg-open").arg(abs_path).status()?;
        if !status.success() {
            return Err(io::Error::new(io::ErrorKind::Other, "xdg-open failed"));
        }
        return Ok(());
    }

    #[allow(unreachable_code)]
    Err(io::Error::new(io::ErrorKind::Unsupported, "Printing not supported on this platform"))
}

// ─── Path helper ──────────────────────────────────────────────────────────────

fn canonicalize_path(path: &str) -> String {
    fs::canonicalize(path)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_owned())
}

// ─── HTML builder ─────────────────────────────────────────────────────────────

fn build_html_report(word: &str, active_systems: &[&str], theme: &CulturalTheme) -> String {
    let all_results = numerology(word);
    let results: Vec<_> = all_results
        .iter()
        .filter(|(name, _)| active_systems.iter().any(|s| s == name))
        .collect();

    let mut rows = String::new();
    for (system, (total, root)) in &results {
        let is_enochian = system.starts_with("Enochian");
        let meaning_raw = match *system {
            s if s.starts_with("Enochian") => enochian_meaning(*root),
            "Greek Isopsephy"              => isopsephy_meaning(*root),
            "Abjad"                        => abjad_meaning(*root),
            _                              => meaning_of(*root),
        };
        let meaning   = strip_leading_emoji(meaning_raw);
        let breakdown = get_calculation_breakdown(word, system);

        let master_html = if !is_enochian {
            master_numbers_message(*total)
                .map(|m| format!(
                    "<div class=\"master\">{}</div>",
                    html_escape(strip_leading_emoji(m))
                ))
                .unwrap_or_default()
        } else {
            String::new()
        };

        let aethyr_html = if is_enochian {
            let (an, aname, adesc) = aethyr_lookup(*total);
            format!(
                "<div class=\"aethyr\">⟁ Aethyr {} ({}) — {}</div>",
                an, aname, html_escape(adesc)
            )
        } else {
            String::new()
        };

        let breakdown_html = if !breakdown.is_empty() {
            format!("<div class=\"calc\">{}</div>", html_escape(&breakdown))
        } else {
            String::new()
        };

        let special_html = check_special_sequences(*total)
            .map(|msg| format!("<div class=\"special\">✦ {}</div>", html_escape(msg)))
            .unwrap_or_default();

        rows.push_str(&format!(
            r#"<tr>
              <td class="sys">{system}</td>
              <td class="num">{total}</td>
              <td class="root">{root}</td>
              <td class="meaning">{meaning}{breakdown_html}{master_html}{aethyr_html}{special_html}</td>
            </tr>"#,
            system         = html_escape(system),
            total          = total,
            root           = root,
            meaning        = html_escape(meaning),
            breakdown_html = breakdown_html,
            master_html    = master_html,
            aethyr_html    = aethyr_html,
            special_html   = special_html,
        ));
    }

    // Angelic message block
    let angelic_html = if let Some((_, (_, root))) = results.first() {
        format!(
            r#"<div class="angelic">
                 <strong>Angelic Message (root {root}):</strong><br>
                 <em>{msg}</em>
               </div>"#,
            root = root,
            msg  = html_escape(angelic_message(*root)),
        )
    } else {
        String::new()
    };

    // Enochian call block
    let enochian_call_html = if active_systems.iter().any(|s| *s == "Enochian Ordinal") {
        if let Some((_, (_, er))) = all_results.iter().find(|(s, _)| *s == "Enochian Ordinal") {
            format!(
                r#"<div class="enochian-call">
                     <strong>Enochian Call (root {root}):</strong><br>
                     <em>{msg}</em>
                   </div>"#,
                root = er,
                msg  = html_escape(enochian_angelic_message(*er)),
            )
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let systems_joined = active_systems.join(" · ");
    let timestamp      = chrono::Utc::now().format("%Y-%m-%d %H:%M UTC").to_string();
    let font_imports   = theme.font_imports;
    let css_vars       = theme.css_vars;
    let tradition      = theme.tradition;

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Celestial Numerology — {word}</title>
  {font_imports}
  <style>
    /* ── Cultural theme variables ── */
    {css_vars}

    /* ── Page setup ── */
    @page {{
      size: A4 portrait;
      margin: 14mm 16mm 14mm 16mm;
    }}
    * {{ box-sizing: border-box; margin: 0; padding: 0; }}
    body {{
      font-size: 9.5pt;
      color: var(--text);
      background: var(--bg);
      line-height: 1.45;
    }}

    /* ── Header ── */
    .header {{
      border-bottom: 2px solid var(--accent);
      padding-bottom: 6pt;
      margin-bottom: 8pt;
      display: flex;
      justify-content: space-between;
      align-items: flex-end;
    }}
    .header h1 {{
      font-size: 14pt;
      color: var(--accent);
      letter-spacing: 0.03em;
    }}
    .header .word {{
      font-size: 22pt;
      font-weight: bold;
      color: var(--text);
    }}
    .meta {{
      font-size: 7.5pt;
      color: var(--calc-color);
      text-align: right;
    }}

    /* ── Results table ── */
    table {{
      width: 100%;
      border-collapse: collapse;
      margin: 6pt 0;
      page-break-inside: avoid;
    }}
    thead tr {{
      background: var(--header-bg);
      color: var(--header-text);
    }}
    thead th {{
      padding: 4pt 6pt;
      font-size: 8pt;
      font-weight: bold;
      text-align: left;
      letter-spacing: 0.05em;
    }}
    th.num, th.root {{ text-align: right; width: 48pt; }}
    tbody tr {{ border-bottom: 1px solid var(--border); }}
    tbody tr:nth-child(even) {{ background: var(--row-even); }}
    td {{ padding: 4pt 6pt; vertical-align: top; }}
    td.sys  {{ font-weight: bold; font-size: 8.5pt; white-space: nowrap; color: var(--sys-color); }}
    td.num  {{ text-align: right; font-family: monospace; font-size: 9pt; }}
    td.root {{ text-align: right; font-family: monospace; font-size: 9pt; font-weight: bold; }}
    td.meaning {{ font-size: 8.5pt; }}

    /* ── Sub-cells within meaning ── */
    .calc    {{ font-size: 7.5pt; color: var(--calc-color); font-family: monospace; margin-top: 2pt; }}
    .master  {{ font-size: 8pt; color: var(--master-color); font-weight: bold; margin-top: 2pt; }}
    .aethyr  {{ font-size: 7.5pt; color: var(--aethyr-color); font-style: italic; margin-top: 2pt; }}
    .special {{ font-size: 7.5pt; color: var(--special-color); margin-top: 2pt; }}

    /* ── Footer blocks ── */
    .angelic, .enochian-call {{
      margin-top: 8pt;
      padding: 6pt 10pt;
      border-left: 3px solid var(--accent);
      background: var(--row-even);
      font-size: 8.5pt;
      page-break-inside: avoid;
    }}
    .enochian-call {{ border-left-color: var(--aethyr-color); margin-top: 5pt; }}

    /* ── Page footer ── */
    .footer {{
      margin-top: 10pt;
      border-top: 1px solid var(--border);
      padding-top: 4pt;
      font-size: 7pt;
      color: var(--calc-color);
      display: flex;
      justify-content: space-between;
    }}
  </style>
</head>
<body>

  <div class="header">
    <div>
      <div class="header h1">Celestial Numerology Analyzer</div>
      <div class="word">&#8220;{word}&#8221;</div>
    </div>
    <div class="meta">
      Systems: {systems}<br>
      Generated: {timestamp}
    </div>
  </div>

  <table>
    <thead>
      <tr>
        <th>System</th>
        <th class="num">Total</th>
        <th class="root">Root</th>
        <th>Meaning &amp; Breakdown</th>
      </tr>
    </thead>
    <tbody>
      {rows}
    </tbody>
  </table>

  {angelic}
  {enochian_call}

  <div class="footer">
    <span>Celestial Numerology Analyzer — {tradition}</span>
    <span>{timestamp2}</span>
  </div>

</body>
</html>"#,
        word          = html_escape(word),
        systems       = html_escape(&systems_joined),
        timestamp     = timestamp,
        timestamp2    = timestamp,
        rows          = rows,
        angelic       = angelic_html,
        enochian_call = enochian_call_html,
        tradition     = tradition,
        font_imports  = font_imports,
        css_vars      = css_vars,
    )
}

// ─── HTML helpers ─────────────────────────────────────────────────────────────

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}
