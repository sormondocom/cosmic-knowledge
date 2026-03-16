//! Export module — render content as Unicode text, HTML, or PDF.
//!
//! OS-printer and browser-based PDF dispatch have been removed in favour of pure-Rust output.
//! Three output formats are supported:
//!  - **Text**  — plain UTF-8 saved to `exports/<stem>.txt`
//!  - **HTML**  — styled single-page HTML saved to `exports/<stem>.html`
//!  - **PDF**   — pure-Rust PDF via `printpdf` saved to `exports/<stem>.pdf`
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

use std::io::{self, Write};
use std::fs;
use colored::*;

use crate::enochian::{
    aethyr_lookup, enochian_angelic_message, enochian_meaning, AETHYRS, ENOCHIAN_LETTERS,
};
use crate::numerology::{
    numerology, meaning_of, isopsephy_meaning, abjad_meaning,
    angelic_message, master_numbers_message, check_special_sequences, get_calculation_breakdown,
};
use crate::reports::{strip_leading_emoji, chrono_now};

// ─── Export choice ────────────────────────────────────────────────────────────

/// The three supported export formats plus skip.
#[derive(Debug, PartialEq)]
pub enum ExportChoice {
    Text,
    Html,
    Pdf,
    Skip,
}

/// Print the format prompt and read the user's choice.
pub fn prompt_export_format() -> ExportChoice {
    print!("{}", "  \u{25b8} Save as text (t), HTML (h), PDF (p), or skip (Enter): ".cyan());
    io::stdout().flush().unwrap_or(());
    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_err() {
        return ExportChoice::Skip;
    }
    match line.trim().to_lowercase().as_str() {
        "t" | "text" => ExportChoice::Text,
        "h" | "html" => ExportChoice::Html,
        "p" | "pdf"  => ExportChoice::Pdf,
        _            => ExportChoice::Skip,
    }
}

// ─── Save functions ───────────────────────────────────────────────────────────

/// Save `content` as UTF-8 to `exports/<stem>.txt`.
pub fn export_text(stem: &str, content: &str) {
    fs::create_dir_all("exports").ok();
    let path = format!("exports/{}.txt", stem);
    match fs::write(&path, content) {
        Ok(_)  => println!("  {}", format!("Saved to: {}", path).bright_green()),
        Err(e) => println!("  {}", format!("Could not write file: {}", e).bright_red()),
    }
}

/// Save `html` to `exports/<stem>.html`.
pub fn export_html(stem: &str, html: &str) {
    fs::create_dir_all("exports").ok();
    let path = format!("exports/{}.html", stem);
    match fs::write(&path, html) {
        Ok(_)  => println!("  {}", format!("HTML saved to: {}", path).bright_green()),
        Err(e) => println!("  {}", format!("Could not write HTML file: {}", e).bright_red()),
    }
}

/// Save `text` to `exports/<stem>.pdf` using pure-Rust printpdf (no browser required).
pub fn export_pdf(stem: &str, text: &str) {
    use printpdf::{BuiltinFont, Mm, PdfDocument};
    use std::io::BufWriter;

    fs::create_dir_all("exports").ok();
    let path = format!("exports/{}.pdf", stem);

    let sanitized = sanitize_for_pdf(text);
    let lines: Vec<&str> = sanitized.lines().collect();

    const FONT_SIZE:     f32 = 9.5;
    const HEADER_SIZE:   f32 = 10.5;
    const LINE_H:        f32 = 4.2;   // mm between baselines
    const MARGIN_LEFT:   f32 = 20.0;
    const MARGIN_TOP:    f32 = 277.0; // 297 - 20 mm
    const MARGIN_BOTTOM: f32 = 20.0;
    const PW:            f32 = 210.0;
    const PH:            f32 = 297.0;

    let (doc, first_page, first_layer) =
        PdfDocument::new(stem, Mm(PW), Mm(PH), "Content");
    let font_r = doc.add_builtin_font(BuiltinFont::Courier).unwrap();
    let font_b = doc.add_builtin_font(BuiltinFont::CourierBold).unwrap();

    let mut layer = doc.get_page(first_page).get_layer(first_layer);
    let mut y = MARGIN_TOP;

    for line in &lines {
        if y < MARGIN_BOTTOM + LINE_H {
            let (new_page, new_layer) = doc.add_page(Mm(PW), Mm(PH), "Content");
            layer = doc.get_page(new_page).get_layer(new_layer);
            y = MARGIN_TOP;
        }

        if !line.is_empty() {
            // Detect all-caps header lines
            let alpha: Vec<char> = line.chars().filter(|c| c.is_alphabetic()).collect();
            let is_header = !alpha.is_empty() && alpha.iter().all(|c| c.is_uppercase());
            let (font, size) = if is_header { (&font_b, HEADER_SIZE) } else { (&font_r, FONT_SIZE) };
            layer.use_text(*line, size, Mm(MARGIN_LEFT), Mm(y), font);
        }

        y -= LINE_H;
    }

    let file = match fs::File::create(&path) {
        Ok(f)  => f,
        Err(e) => { println!("  {}", format!("Could not create PDF: {}", e).bright_red()); return; }
    };
    match doc.save(&mut BufWriter::new(file)) {
        Ok(_)  => println!("  {}", format!("PDF saved to: {}", path).bright_green()),
        Err(e) => println!("  {}", format!("Could not write PDF: {}", e).bright_red()),
    }
}

/// Replace box-drawing and non-Latin-1 characters so printpdf's built-in Courier font renders them.
fn sanitize_for_pdf(s: &str) -> String {
    s.chars().map(|c| match c {
        '═' | '━'                       => '=',
        '─' | '\u{2014}' | '\u{2013}'   => '-',
        '║' | '│'                       => '|',
        '╔'|'╗'|'╚'|'╝'|'╠'|'╣'|'╦'|'╩'|'╬' => '+',
        '\u{201C}' | '\u{201D}'         => '"',
        '\u{2018}' | '\u{2019}'         => '\'',
        '\u{25b8}' | '\u{27c1}' | '\u{2726}' | '\u{2192}' => '>',
        c if (c as u32) <= 0xFF         => c,
        _                               => ' ',
    }).collect()
}

// ─── Generic export helper ────────────────────────────────────────────────────

/// Prompt for a format then call the appropriate builder and saver.
///
/// Each `FnOnce` builder is called at most once — only the chosen branch runs.
pub fn handle_export<FT, FH>(stem: &str, build_text: FT, build_html: FH)
where
    FT: FnOnce() -> String,
    FH: FnOnce() -> String,
{
    match prompt_export_format() {
        ExportChoice::Text => export_text(stem, &build_text()),
        ExportChoice::Html => export_html(stem, &build_html()),
        ExportChoice::Pdf  => export_pdf(stem, &build_text()),
        ExportChoice::Skip => {}
    }
}

// ─── HTML wrapper ─────────────────────────────────────────────────────────────

/// Wrap arbitrary pre-built HTML body content with full page HTML + cultural theme CSS.
/// `tradition_key` selects the theme (one of "hebrew", "pythagorean", "chaldean",
/// "greek", "agrippan", "ordinal", "abjad", "enochian", "chinese", "african", or "").
pub fn wrap_html(title: &str, body_html: &str, tradition_key: &str) -> String {
    let theme = theme_for_key(tradition_key);
    let font_imports = theme.font_imports;
    let css_vars     = theme.css_vars;
    let tradition    = theme.tradition;
    let timestamp    = chrono_now();

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>{title}</title>
  {font_imports}
  <style>
    /* Cultural theme variables */
    {css_vars}

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
    .page-header {{
      border-bottom: 2px solid var(--accent);
      padding-bottom: 6pt;
      margin-bottom: 8pt;
    }}
    .page-header h1 {{
      font-size: 14pt;
      color: var(--accent);
      letter-spacing: 0.03em;
    }}
    .meta {{
      font-size: 7.5pt;
      color: var(--calc-color);
    }}
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
    .calc    {{ font-size: 7.5pt; color: var(--calc-color); font-family: monospace; margin-top: 2pt; }}
    .master  {{ font-size: 8pt; color: var(--master-color); font-weight: bold; margin-top: 2pt; }}
    .aethyr  {{ font-size: 7.5pt; color: var(--aethyr-color); font-style: italic; margin-top: 2pt; }}
    .special {{ font-size: 7.5pt; color: var(--special-color); margin-top: 2pt; }}
    .block-note {{
      margin-top: 8pt;
      padding: 6pt 10pt;
      border-left: 3px solid var(--accent);
      background: var(--row-even);
      font-size: 8.5pt;
      page-break-inside: avoid;
    }}
    .page-footer {{
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

  <div class="page-header">
    <h1>{title}</h1>
    <div class="meta">Generated: {timestamp}</div>
  </div>

  {body_html}

  <div class="page-footer">
    <span>Celestial Numerology Analyzer &mdash; {tradition}</span>
    <span>{timestamp2}</span>
  </div>

</body>
</html>"#,
        title      = html_escape(title),
        font_imports = font_imports,
        css_vars     = css_vars,
        body_html    = body_html,
        tradition    = tradition,
        timestamp    = timestamp,
        timestamp2   = timestamp,
    )
}

// ─── Numerology HTML builder ──────────────────────────────────────────────────

/// Build a styled HTML report for a numerology word analysis.
pub fn build_numerology_html(word: &str, active_systems: &[&str]) -> String {
    let theme     = theme_for_systems(active_systems);
    let html_body = build_numerology_html_body(word, active_systems, &theme);
    let all_results = numerology(word);

    // Angelic message block
    let first_root = all_results
        .iter()
        .filter(|(name, _)| active_systems.iter().any(|s| s == name))
        .next()
        .map(|(_, (_, r))| *r);

    let angelic_html = if let Some(root) = first_root {
        format!(
            r#"<div class="block-note">
               <strong>Angelic Message (root {root}):</strong><br>
               <em>{msg}</em>
             </div>"#,
            root = root,
            msg  = html_escape(angelic_message(root)),
        )
    } else {
        String::new()
    };

    let enochian_call_html = if active_systems.iter().any(|s| *s == "Enochian Ordinal") {
        if let Some((_, (_, er))) = all_results.iter().find(|(s, _)| *s == "Enochian Ordinal") {
            format!(
                r#"<div class="block-note" style="border-left-color:var(--aethyr-color);margin-top:5pt;">
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

    let systems_joined = active_systems.join(" \u{00b7} ");
    let full_body = format!(
        r#"<div style="margin-bottom:6pt;">
             <span style="font-size:22pt;font-weight:bold;">&ldquo;{word}&rdquo;</span>
             <span class="meta" style="margin-left:10pt;">Systems: {systems}</span>
           </div>
           {table}
           {angelic}
           {enochian_call}"#,
        word         = html_escape(word),
        systems      = html_escape(&systems_joined),
        table        = html_body,
        angelic      = angelic_html,
        enochian_call = enochian_call_html,
    );

    let title = format!("Celestial Numerology \u{2014} {}", word);
    wrap_html(&title, &full_body, "")
}

fn build_numerology_html_body(word: &str, active_systems: &[&str], theme: &CulturalTheme) -> String {
    let all_results = numerology(word);
    let results: Vec<_> = all_results
        .iter()
        .filter(|(name, _)| active_systems.iter().any(|s| s == name))
        .collect();

    let _ = theme; // theme applied by caller via wrap_html / build_numerology_html

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
                "<div class=\"aethyr\">\u{27c1} Aethyr {} ({}) \u{2014} {}</div>",
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
            .map(|msg| format!("<div class=\"special\">\u{2726} {}</div>", html_escape(msg)))
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

    format!(
        r#"<table>
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
  </table>"#,
        rows = rows,
    )
}

// ─── Enochian HTML builders ───────────────────────────────────────────────────

/// Build a styled HTML page for the Enochian alphabet table.
pub fn build_enochian_alphabet_html() -> String {
    let mut rows = String::new();
    for (name, chars, ordinal, gd) in ENOCHIAN_LETTERS {
        let chars_str: String = chars.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", ");
        rows.push_str(&format!(
            r#"<tr>
              <td class="sys">{name}</td>
              <td>{chars}</td>
              <td class="num">{ordinal}</td>
              <td class="num">{gd}</td>
            </tr>"#,
            name    = html_escape(name),
            chars   = html_escape(&chars_str),
            ordinal = ordinal,
            gd      = gd,
        ));
    }

    let body = format!(
        r#"<p style="font-size:8.5pt;margin-bottom:6pt;">
     The 21 letters of the Enochian alphabet as received by John Dee &amp; Edward Kelley (1582&ndash;1587).
     J, K, W have no Enochian equivalents and are substituted: J&rarr;I, K&rarr;C, W&rarr;V.
   </p>
   <table>
     <thead>
       <tr>
         <th>Letter Name</th>
         <th>English Chars</th>
         <th class="num">Ordinal</th>
         <th class="num">GD Value</th>
       </tr>
     </thead>
     <tbody>{rows}</tbody>
   </table>"#,
        rows = rows,
    );

    wrap_html("Enochian Alphabet & Gematria Table", &body, "enochian")
}

/// Build a styled HTML page for all 30 Aethyrs.
pub fn build_aethyr_table_html() -> String {
    let mut rows = String::new();
    for (num, name, desc) in AETHYRS {
        rows.push_str(&format!(
            r#"<tr>
              <td class="num">{num}</td>
              <td class="sys">{name}</td>
              <td class="meaning">{desc}</td>
            </tr>"#,
            num  = num,
            name = html_escape(name),
            desc = html_escape(desc),
        ));
    }

    let body = format!(
        r#"<p style="font-size:8.5pt;margin-bottom:6pt;">
     The 30 Aethyrs (Heavens) of the Enochian system, from outermost (TEX/30) to innermost (LIL/1).
     Sources: Dee&rsquo;s diaries; Crowley, <em>The Vision and the Voice</em> (1909).
   </p>
   <table>
     <thead>
       <tr>
         <th class="num">#</th>
         <th>Name</th>
         <th>Description</th>
       </tr>
     </thead>
     <tbody>{rows}</tbody>
   </table>"#,
        rows = rows,
    );

    wrap_html("The 30 Enochian Aethyrs", &body, "enochian")
}

/// Build a styled HTML page for a single Aethyr (looked up by number or 3-letter name).
pub fn build_aethyr_info_html(query: &str) -> String {
    let query_up = query.to_uppercase();
    let found = AETHYRS.iter().find(|(num, name, _)| {
        query_up == name.to_string() ||
        query.parse::<u32>().map(|n| n == *num).unwrap_or(false)
    });

    let body = match found {
        None => format!("<p>No Aethyr found for query: <em>{}</em></p>", html_escape(query)),
        Some((num, name, desc)) => format!(
            r#"<table>
             <tbody>
               <tr><td class="sys">Number</td><td>{num}</td></tr>
               <tr><td class="sys">Name</td><td>{name}</td></tr>
               <tr><td class="sys">Description</td><td class="meaning">{desc}</td></tr>
             </tbody>
           </table>"#,
            num  = num,
            name = html_escape(name),
            desc = html_escape(desc),
        ),
    };

    let title = format!("Aethyr: {}", query.to_uppercase());
    wrap_html(&title, &body, "enochian")
}

/// Build a styled HTML page for a single Enochian Key.
pub fn build_enochian_key_html(num: u32, title: &str, opening: &str, body: &str) -> String {
    let root = ((num - 1) % 9 + 1) as u32;
    let em = enochian_meaning(root);

    let html_body = format!(
        r#"<table>
         <tbody>
           <tr>
             <td class="sys">Key Number</td>
             <td>{num}</td>
           </tr>
           <tr>
             <td class="sys">Opening (Enochian)</td>
             <td class="meaning" style="font-style:italic;">{opening}</td>
           </tr>
           <tr>
             <td class="sys">Dee's Rendering</td>
             <td class="meaning">{body}</td>
           </tr>
           <tr>
             <td class="sys">Enochian Meaning (root {root})</td>
             <td class="meaning">{meaning}</td>
           </tr>
         </tbody>
       </table>"#,
        num     = num,
        opening = html_escape(opening),
        body    = html_escape(body),
        root    = root,
        meaning = html_escape(strip_leading_emoji(em)),
    );

    let page_title = format!("Enochian Key {} \u{2014} {}", num, title);
    wrap_html(&page_title, &html_body, "enochian")
}

// ─── Cultural theme ───────────────────────────────────────────────────────────

struct CulturalTheme {
    font_imports: &'static str,
    css_vars:     &'static str,
    tradition:    &'static str,
}

/// Select a theme by an explicit tradition key string.
fn theme_for_key(key: &str) -> CulturalTheme {
    match key {
        "hebrew"      => theme_hebrew(),
        "pythagorean" => theme_pythagorean(),
        "chaldean"    => theme_chaldean(),
        "greek"       => theme_greek(),
        "agrippan"    => theme_agrippan(),
        "ordinal"     => theme_ordinal(),
        "abjad"       => theme_abjad(),
        "enochian"    => theme_enochian(),
        "chinese"     => theme_chinese(),
        "african"     => theme_african(),
        _             => theme_default(),
    }
}

/// Select the most culturally-appropriate theme for a set of active systems.
fn theme_for_systems(systems: &[&str]) -> CulturalTheme {
    let mut groups: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for s in systems {
        let g = match *s {
            "Hebrew Gematria"                          => "hebrew",
            "Pythagorean"                              => "pythagorean",
            "Chaldean"                                 => "chaldean",
            "Greek Isopsephy"                          => "greek",
            "Agrippan"                                 => "agrippan",
            "Simple Ordinal" | "Reverse Ordinal"       => "ordinal",
            "Abjad"                                    => "abjad",
            s if s.starts_with("Enochian")             => "enochian",
            s if s.starts_with("Nine Star") || s.starts_with("Wu Xing")
              || s.starts_with("Ba Gua") || s.starts_with("Chinese") => "chinese",
            s if s.starts_with("Ifa") || s.starts_with("Yoruba")
              || s.starts_with("Akan") || s.starts_with("Kemetic")
              || s.starts_with("African") => "african",
            _ => "default",
        };
        groups.insert(g);
    }

    if groups.len() == 1 {
        let key = *groups.iter().next().unwrap();
        if key != "default" {
            return theme_for_key(key);
        }
    }
    theme_default()
}

fn theme_hebrew() -> CulturalTheme {
    CulturalTheme {
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
    .page-header h1 { font-family: 'Rubik', 'Frank Ruhl Libre', serif; letter-spacing: 0.1em; }"#,
        tradition: "Hebrew Kabbalistic Tradition",
    }
}

fn theme_pythagorean() -> CulturalTheme {
    CulturalTheme {
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
    }
}

fn theme_chaldean() -> CulturalTheme {
    CulturalTheme {
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
    }
}

fn theme_greek() -> CulturalTheme {
    CulturalTheme {
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
        tradition: "Greek Isopsephy \u{2014} Neoplatonic Tradition",
    }
}

fn theme_agrippan() -> CulturalTheme {
    CulturalTheme {
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
    }
}

fn theme_ordinal() -> CulturalTheme {
    CulturalTheme {
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
    }
}

fn theme_abjad() -> CulturalTheme {
    CulturalTheme {
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
        tradition: "Abjad \u{2014} Arabic/Islamic Tradition",
    }
}

fn theme_enochian() -> CulturalTheme {
    CulturalTheme {
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
        tradition: "Enochian \u{2014} John Dee's Angelic System (1582\u{2013}1587)",
    }
}

fn theme_chinese() -> CulturalTheme {
    CulturalTheme {
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
    }
}

fn theme_african() -> CulturalTheme {
    CulturalTheme {
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
        tradition: "African Traditions \u{2014} Yoruba If\u{00e1} \u{00b7} Akan \u{00b7} Kemetic",
    }
}

fn theme_default() -> CulturalTheme {
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

// ─── HTML helpers ─────────────────────────────────────────────────────────────

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}
