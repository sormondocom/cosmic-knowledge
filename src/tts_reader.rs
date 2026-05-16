//! Text-to-speech subsystem — thin wrapper around the `tts` crate.
//!
//! Cross-platform: Windows SAPI / WinRT, macOS AVSpeechSynthesizer,
//! Linux SpeechDispatcher.  Fails gracefully if no engine is available.
//!
//! On Android / Termux the `tts` crate is unavailable; all control
//! functions compile as no-ops so session files can import them unconditionally.
//!
//! State lives in a thread-local so sessions can access TTS without threading
//! it through every function signature.

// ─── Non-Android full implementation ─────────────────────────────────────────

#[cfg(not(target_os = "android"))]
use std::cell::RefCell;
#[cfg(not(target_os = "android"))]
use std::io::{self, Write};

#[cfg(not(target_os = "android"))]
use colored::*;
#[cfg(not(target_os = "android"))]
use tts::{Gender, Tts, Voice};

#[cfg(not(target_os = "android"))]
thread_local! {
    static TTS: RefCell<Option<TtsReader>> = const { RefCell::new(None) };
}

#[cfg(not(target_os = "android"))]
pub struct TtsReader {
    inner:         Tts,
    pub auto_read: bool,
    voice_name:    String,
    paused:        bool,
    last_speech:   String,  // stores text for pause→resume
}

#[cfg(not(target_os = "android"))]
impl TtsReader {
    fn new() -> Option<Self> {
        Tts::default().ok().map(|inner| Self {
            inner,
            auto_read:   false,
            voice_name:  String::new(),
            paused:      false,
            last_speech: String::new(),
        })
    }

    pub fn speak(&mut self, text: &str) {
        self.paused = false;
        self.last_speech = text.to_string();
        // interrupt=true: stop any current utterance before starting
        let _ = self.inner.speak(text, true);
    }

    pub fn stop(&mut self) {
        self.paused = false;
        let _ = self.inner.stop();
    }

    pub fn pause(&mut self) {
        if self.paused {
            // Resume: re-speak from the beginning of the last text
            self.paused = false;
            let text = self.last_speech.clone();
            if !text.is_empty() {
                let _ = self.inner.speak(&text, true);
            }
        } else {
            let _ = self.inner.stop();
            self.paused = true;
        }
    }

    pub fn is_speaking(&mut self) -> bool {
        self.inner.is_speaking().unwrap_or(false)
    }

    pub fn toggle_auto(&mut self) -> bool {
        self.auto_read = !self.auto_read;
        if !self.auto_read {
            self.stop();
        }
        self.auto_read
    }

    pub fn voice_label(&self) -> &str {
        if self.voice_name.is_empty() { "default" } else { &self.voice_name }
    }

    /// Return simplified voice list: up to 3 female + 3 male English voices.
    fn available_voices(&mut self) -> Vec<VoiceChoice> {
        let all = match self.inner.voices() {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };
        let mut female = Vec::new();
        let mut male   = Vec::new();
        let mut other  = Vec::new();

        for v in all {
            let lang = v.language().to_lowercase();
            if !lang.starts_with("en") {
                continue;
            }
            match v.gender() {
                Some(Gender::Female) => female.push(v),
                Some(Gender::Male)   => male.push(v),
                _                    => other.push(v),
            }
        }

        let mut choices = Vec::new();
        for v in female.into_iter().take(3) {
            choices.push(VoiceChoice { gender: "♀ female", voice: v });
        }
        for v in male.into_iter().take(3) {
            choices.push(VoiceChoice { gender: "♂ male  ", voice: v });
        }
        for v in other.into_iter().take(2) {
            choices.push(VoiceChoice { gender: "  other ", voice: v });
        }
        choices
    }

    pub fn get_rate(&mut self) -> f32 {
        self.inner.get_rate().unwrap_or_else(|_| self.inner.normal_rate())
    }

    pub fn rate_range(&self) -> (f32, f32) {
        (self.inner.min_rate(), self.inner.max_rate())
    }

    pub fn adjust_rate(&mut self, up: bool) -> f32 {
        let min = self.inner.min_rate();
        let max = self.inner.max_rate();
        let cur = self.inner.get_rate().unwrap_or_else(|_| self.inner.normal_rate());
        let step = (max - min) * 0.1;
        let new = if up { cur + step } else { cur - step }.clamp(min, max);
        let _ = self.inner.set_rate(new);
        new
    }

    pub fn get_volume(&mut self) -> f32 {
        self.inner.get_volume().unwrap_or_else(|_| self.inner.normal_volume())
    }

    pub fn volume_range(&self) -> (f32, f32) {
        (self.inner.min_volume(), self.inner.max_volume())
    }

    pub fn adjust_volume(&mut self, up: bool) -> f32 {
        let min = self.inner.min_volume();
        let max = self.inner.max_volume();
        let cur = self.inner.get_volume().unwrap_or_else(|_| self.inner.normal_volume());
        let step = (max - min) * 0.1;
        let new = if up { cur + step } else { cur - step }.clamp(min, max);
        let _ = self.inner.set_volume(new);
        new
    }

    pub fn set_rate_frac(&mut self, frac: f32) {
        let min = self.inner.min_rate();
        let max = self.inner.max_rate();
        let val = (min + frac.clamp(0.0, 1.0) * (max - min)).clamp(min, max);
        let _ = self.inner.set_rate(val);
    }

    pub fn get_rate_frac(&mut self) -> f32 {
        let min = self.inner.min_rate();
        let max = self.inner.max_rate();
        let cur = self.inner.get_rate().unwrap_or_else(|_| self.inner.normal_rate());
        if max <= min { return 0.5; }
        ((cur - min) / (max - min)).clamp(0.0, 1.0)
    }

    pub fn set_volume_frac(&mut self, frac: f32) {
        let min = self.inner.min_volume();
        let max = self.inner.max_volume();
        let val = (min + frac.clamp(0.0, 1.0) * (max - min)).clamp(min, max);
        let _ = self.inner.set_volume(val);
    }

    pub fn get_volume_frac(&mut self) -> f32 {
        let min = self.inner.min_volume();
        let max = self.inner.max_volume();
        let cur = self.inner.get_volume().unwrap_or_else(|_| self.inner.normal_volume());
        if max <= min { return 0.5; }
        ((cur - min) / (max - min)).clamp(0.0, 1.0)
    }

    pub fn set_voice_by_name(&mut self, name: &str) {
        if let Ok(voices) = self.inner.voices() {
            if let Some(v) = voices.iter().find(|v| v.name() == name) {
                self.voice_name = name.to_string();
                let _ = self.inner.set_voice(v);
            }
        }
    }

    /// Interactive voice selection. Returns true if a voice was set.
    pub fn select_voice_interactive(&mut self) -> bool {
        let voices = self.available_voices();
        if voices.is_empty() {
            println!("{}", "  No voices available — using system default.".yellow());
            return false;
        }

        println!();
        println!("{}", "  ── Available voices ────────────────────────────────────".bright_cyan());
        for (i, vc) in voices.iter().enumerate() {
            println!(
                "  {}. {} {}",
                (i + 1).to_string().bright_yellow(),
                vc.gender.dimmed(),
                vc.voice.name().bright_white(),
            );
        }
        println!("{}", "  Enter = keep current / 0 = cancel".dimmed());
        println!();
        print!("{}", "  Voice (1–{}): ".replace("{}", &voices.len().to_string()).bold().cyan());
        io::stdout().flush().unwrap_or(());

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap_or(0);
        let input = buf.trim();

        if input.is_empty() || input == "0" {
            return false;
        }
        if let Ok(n) = input.parse::<usize>() {
            if n >= 1 && n <= voices.len() {
                let chosen = &voices[n - 1].voice;
                self.voice_name = chosen.name().to_string();
                let _ = self.inner.set_voice(chosen);
                println!("{}", format!("  Voice set: {}", self.voice_name).bright_green());
                return true;
            }
        }
        false
    }
}

#[cfg(not(target_os = "android"))]
struct VoiceChoice {
    gender: &'static str,
    voice:  Voice,
}

// ─── Public thread-local API — non-Android ────────────────────────────────────

/// Initialise TTS once for this thread.  Silently does nothing if unavailable.
#[cfg(not(target_os = "android"))]
pub fn init_tts() {
    TTS.with(|cell| {
        if cell.borrow().is_none() {
            *cell.borrow_mut() = TtsReader::new();
        }
    });
}

/// True when TTS initialised successfully.
#[cfg(not(target_os = "android"))]
pub fn tts_available() -> bool {
    TTS.with(|cell| cell.borrow().is_some())
}

/// Speak `text` immediately, interrupting any current speech.
#[cfg(not(target_os = "android"))]
pub fn tts_speak(text: &str) {
    TTS.with(|cell| {
        if let Some(ref mut r) = *cell.borrow_mut() {
            r.speak(text);
        }
    });
}

/// Stop all speech.
#[cfg(not(target_os = "android"))]
pub fn tts_stop() {
    TTS.with(|cell| {
        if let Some(ref mut r) = *cell.borrow_mut() {
            r.stop();
        }
    });
}

/// Toggle pause / resume.  Returns `true` when now paused.
#[cfg(not(target_os = "android"))]
pub fn tts_toggle_pause() -> bool {
    TTS.with(|cell| {
        if let Some(ref mut r) = *cell.borrow_mut() {
            r.pause();
            return r.paused;
        }
        false
    })
}

/// Toggle auto-read mode.  Returns the new state (`true` = on).
#[cfg(not(target_os = "android"))]
pub fn tts_toggle_auto() -> bool {
    TTS.with(|cell| {
        if let Some(ref mut r) = *cell.borrow_mut() {
            return r.toggle_auto();
        }
        false
    })
}

/// Returns `true` when auto-read is currently on.
#[cfg(not(target_os = "android"))]
pub fn tts_auto_read() -> bool {
    TTS.with(|cell| {
        cell.borrow()
            .as_ref()
            .map(|r| r.auto_read)
            .unwrap_or(false)
    })
}

/// Returns `true` when speech is in progress.
#[cfg(not(target_os = "android"))]
pub fn tts_is_speaking() -> bool {
    TTS.with(|cell| {
        if let Some(ref mut r) = *cell.borrow_mut() {
            return r.is_speaking();
        }
        false
    })
}

/// Open the interactive voice selection dialog.
#[cfg(not(target_os = "android"))]
pub fn tts_select_voice() {
    TTS.with(|cell| {
        if let Some(ref mut r) = *cell.borrow_mut() {
            r.select_voice_interactive();
        }
    });
}

/// Current speech rate as a (value, min, max) tuple.
#[cfg(not(target_os = "android"))]
pub fn tts_get_rate() -> (f32, f32, f32) {
    TTS.with(|cell| {
        if let Some(ref mut r) = *cell.borrow_mut() {
            let (min, max) = r.rate_range();
            return (r.get_rate(), min, max);
        }
        (0.0, 0.0, 1.0)
    })
}

/// Step the speech rate up (faster) or down (slower) by 10 % of range.
/// Returns the new rate value.
#[cfg(not(target_os = "android"))]
pub fn tts_adjust_rate(up: bool) -> f32 {
    TTS.with(|cell| {
        if let Some(ref mut r) = *cell.borrow_mut() {
            return r.adjust_rate(up);
        }
        0.0
    })
}

/// Current speech volume as a (value, min, max) tuple.
#[cfg(not(target_os = "android"))]
pub fn tts_get_volume() -> (f32, f32, f32) {
    TTS.with(|cell| {
        if let Some(ref mut r) = *cell.borrow_mut() {
            let (min, max) = r.volume_range();
            return (r.get_volume(), min, max);
        }
        (0.0, 0.0, 1.0)
    })
}

/// Step the volume up (louder) or down (quieter) by 10 % of range.
/// Returns the new volume value.
#[cfg(not(target_os = "android"))]
pub fn tts_adjust_volume(up: bool) -> f32 {
    TTS.with(|cell| {
        if let Some(ref mut r) = *cell.borrow_mut() {
            return r.adjust_volume(up);
        }
        0.0
    })
}

/// Apply persisted TTS settings (rate, volume, voice, auto-read) from the database.
/// Call once at startup after `init_tts()`.
#[cfg(not(target_os = "android"))]
pub fn tts_apply_settings(conn: &rusqlite::Connection) {
    use crate::persistence::get_setting;
    TTS.with(|cell| {
        if let Some(ref mut r) = *cell.borrow_mut() {
            if let Some(s) = get_setting(conn, "tts_rate_frac") {
                if let Ok(f) = s.parse::<f32>() { r.set_rate_frac(f); }
            }
            if let Some(s) = get_setting(conn, "tts_volume_frac") {
                if let Ok(f) = s.parse::<f32>() { r.set_volume_frac(f); }
            }
            if let Some(name) = get_setting(conn, "tts_voice_name") {
                if !name.is_empty() { r.set_voice_by_name(&name); }
            }
            if let Some(s) = get_setting(conn, "tts_auto_read") {
                r.auto_read = s == "true";
            }
        }
    });
}

/// Persist current TTS settings (rate, volume, voice, auto-read) to the database.
#[cfg(not(target_os = "android"))]
pub fn tts_save_settings(conn: &rusqlite::Connection) {
    use crate::persistence::set_setting;
    TTS.with(|cell| {
        if let Some(ref mut r) = *cell.borrow_mut() {
            set_setting(conn, "tts_rate_frac",    &r.get_rate_frac().to_string());
            set_setting(conn, "tts_volume_frac",  &r.get_volume_frac().to_string());
            set_setting(conn, "tts_voice_name",   &r.voice_name.clone());
            set_setting(conn, "tts_auto_read",    if r.auto_read { "true" } else { "false" });
        }
    });
}

/// Label of the current voice for display.
#[cfg(not(target_os = "android"))]
pub fn tts_voice_label() -> String {
    TTS.with(|cell| {
        cell.borrow()
            .as_ref()
            .map(|r| r.voice_label().to_string())
            .unwrap_or_else(|| "unavailable".to_string())
    })
}

// ─── Android stubs (no-ops) ───────────────────────────────────────────────────

#[cfg(target_os = "android")]
pub fn init_tts() {}
#[cfg(target_os = "android")]
pub fn tts_available() -> bool { false }
#[cfg(target_os = "android")]
pub fn tts_speak(_text: &str) {}
#[cfg(target_os = "android")]
pub fn tts_stop() {}
#[cfg(target_os = "android")]
pub fn tts_toggle_pause() -> bool { false }
#[cfg(target_os = "android")]
pub fn tts_toggle_auto() -> bool { false }
#[cfg(target_os = "android")]
pub fn tts_auto_read() -> bool { false }
#[cfg(target_os = "android")]
pub fn tts_is_speaking() -> bool { false }
#[cfg(target_os = "android")]
pub fn tts_select_voice() {}
#[cfg(target_os = "android")]
pub fn tts_voice_label() -> String { "unavailable".to_string() }
#[cfg(target_os = "android")]
pub fn tts_get_rate() -> (f32, f32, f32) { (0.0, 0.0, 1.0) }
#[cfg(target_os = "android")]
pub fn tts_adjust_rate(_up: bool) -> f32 { 0.0 }
#[cfg(target_os = "android")]
pub fn tts_get_volume() -> (f32, f32, f32) { (0.0, 0.0, 1.0) }
#[cfg(target_os = "android")]
pub fn tts_adjust_volume(_up: bool) -> f32 { 0.0 }
#[cfg(target_os = "android")]
pub fn tts_apply_settings(_conn: &rusqlite::Connection) {}
#[cfg(target_os = "android")]
pub fn tts_save_settings(_conn: &rusqlite::Connection) {}

// ─── Text formatting for natural speech (all platforms) ──────────────────────

/// Book name as TTS would naturally pronounce it.
pub fn pronounce_book(book: &str) -> String {
    match book {
        "1 Kings"           => "First Kings".to_string(),
        "2 Kings"           => "Second Kings".to_string(),
        "1 Samuel"          => "First Samuel".to_string(),
        "2 Samuel"          => "Second Samuel".to_string(),
        "1 Chronicles"      => "First Chronicles".to_string(),
        "2 Chronicles"      => "Second Chronicles".to_string(),
        "1 Corinthians"     => "First Corinthians".to_string(),
        "2 Corinthians"     => "Second Corinthians".to_string(),
        "1 Thessalonians"   => "First Thessalonians".to_string(),
        "2 Thessalonians"   => "Second Thessalonians".to_string(),
        "1 Timothy"         => "First Timothy".to_string(),
        "2 Timothy"         => "Second Timothy".to_string(),
        "1 Peter"           => "First Peter".to_string(),
        "2 Peter"           => "Second Peter".to_string(),
        "1 John"            => "First John".to_string(),
        "2 John"            => "Second John".to_string(),
        "3 John"            => "Third John".to_string(),
        "1 Enoch"           => "First Enoch".to_string(),
        "2 Enoch"           => "Second Enoch".to_string(),
        other               => other.to_string(),
    }
}

/// Chapter announcement with a natural pause before the body text follows.
/// e.g. "Genesis, chapter 3.  " — the trailing spaces give a short breath.
pub fn chapter_intro(book: &str, chapter: u32) -> String {
    format!("{}. Chapter {}.  ", pronounce_book(book), chapter)
}

/// Prepare verse text for TTS: trim, ensure terminal punctuation.
pub fn clean_for_tts(text: &str) -> String {
    let t = text.trim();
    let ends_ok = matches!(t.chars().last(), Some('.' | '?' | '!' | '\'' | '"'));
    if ends_ok {
        t.to_string()
    } else {
        format!("{t}.")
    }
}

/// Build a full chapter speech string: header announcement + all verses joined
/// naturally, WITHOUT verse numbers (which would sound robotic).
pub fn build_chapter_speech(book: &str, chapter: u32, texts: &[&str]) -> String {
    let mut out = chapter_intro(book, chapter);
    for text in texts {
        out.push_str(&clean_for_tts(text));
        out.push(' ');
    }
    out
}

// ─── Status hints ─────────────────────────────────────────────────────────────

/// Short hint string to append to navigation prompts when TTS is available.
/// `stop_key` is the key bound to stop in the current context (e.g. "s" or "o").
pub fn tts_nav_hint(stop_key: &str) -> String {
    if !tts_available() {
        return String::new();
    }
    let auto_label = if tts_auto_read() { "a=auto-OFF" } else { "a=auto-ON" };
    format!("r=read  {}  t=pause  {}=stop  ", auto_label, stop_key)
}
