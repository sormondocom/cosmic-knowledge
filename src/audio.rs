//! Audio engine module — sacred frequency generation, WAV export, and binaural beats.
//!
//! Contains:
//!  - `AudioSystem` wrapper around `rodio`'s `OutputStream` + `Sink`
//!  - `SineWave` iterator for pure-tone synthesis
//!  - Ambient / per-word frequency control
//!  - WAV file export (pure tone and binaural beat)
//!  - Solfeggio frequency table and root → frequency mapping
//!  - Interactive export-all and custom-binaural-beat helpers

use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

use colored::*;
use rodio::{OutputStream, Sink, Source};

// ─── Audio system ─────────────────────────────────────────────────────────────

/// Owns the `rodio` output stream (must stay alive for audio to play) and a
/// shared `Sink` that can be swapped out when the frequency changes.
pub struct AudioSystem {
    /// The stream handle must be kept alive — drop = silence.
    pub _stream: OutputStream,
    pub sink: Arc<Mutex<Sink>>,
}

// ─── Sine wave source ─────────────────────────────────────────────────────────

/// Infinite iterator that yields single-channel `f32` PCM samples for a pure sine tone.
///
/// Uses a normalised phase accumulator (0.0..1.0) rather than a raw sample counter.
/// This avoids precision loss over long runs (a u64 counter would eventually lose
/// floating-point resolution; a fractional phase wraps cleanly every cycle).
pub struct SineWave {
    pub frequency: f32,
    pub sample_rate: f32,
    /// Normalised phase in [0.0, 1.0).  Advances by `frequency / sample_rate` each sample.
    pub phase: f64,
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        // Amplitude kept low (0.1) to protect hearing when speakers are loud.
        let sample = (2.0 * std::f64::consts::PI * self.phase).sin() as f32 * 0.1;
        self.phase = (self.phase + self.frequency as f64 / self.sample_rate as f64) % 1.0;
        Some(sample)
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self)       -> u16 { 1 }
    fn sample_rate(&self)    -> u32 { self.sample_rate as u32 }
    fn total_duration(&self) -> Option<Duration> { None }
}

// ─── Intro chord ──────────────────────────────────────────────────────────────

/// A 5-second three-note chord with a decaying echo envelope.
///
/// Amplitude envelope: four overlapping strikes at t = 0.0 s, 0.4 s, 0.8 s,
/// and 1.2 s, each decaying at rate 1.8 s⁻¹.  The combined envelope fades to
/// near-silence by ~4.5 s before the source terminates at 5 s.
///
/// The three frequencies are supplied by the caller; see [`play_intro_chord`].
struct IntroChord {
    freqs:         [f32; 3],
    sample_rate:   u32,
    position:      u32,
    total_samples: u32,
}

impl IntroChord {
    fn new(freqs: [f32; 3]) -> Self {
        Self { freqs, sample_rate: 44100, position: 0, total_samples: 5 * 44100 }
    }
}

impl Iterator for IntroChord {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.position >= self.total_samples { return None; }

        let t   = self.position as f32 / self.sample_rate as f32;
        let tau = self.position as f64 / self.sample_rate as f64;

        // Four echoes, each an exponentially-decaying strike.
        const DECAY: f32 = 1.8;
        const STRIKES: &[(f32, f32)] = &[(0.0, 1.00), (0.4, 0.55), (0.8, 0.30), (1.2, 0.15)];
        let envelope: f32 = STRIKES.iter()
            .filter(|&&(onset, _)| t >= onset)
            .map(|&(onset, amp)| amp * (-(t - onset) * DECAY).exp())
            .sum();

        let wave: f32 = self.freqs.iter()
            .map(|&f| (2.0 * std::f64::consts::PI * f as f64 * tau).sin() as f32)
            .sum();

        self.position += 1;
        // 0.12 scale keeps peak amplitude (envelope ~1.0 × three in-phase waves)
        // well below clipping on typical consumer hardware.
        Some(wave * envelope * 0.12)
    }
}

impl Source for IntroChord {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self)       -> u16 { 1 }
    fn sample_rate(&self)    -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<Duration> { Some(Duration::from_secs(5)) }
}

/// Play a three-note intro chord at the given frequencies and block until done.
///
/// Clears the sink before appending so no stale audio bleeds in.
pub fn play_intro_chord(system: &AudioSystem, freqs: [f32; 3]) {
    if let Ok(sink) = system.sink.lock() {
        sink.stop();
        sink.append(IntroChord::new(freqs));
        sink.play();
    }
    thread::sleep(Duration::from_secs(5));
}

// ─── Lifecycle helpers ────────────────────────────────────────────────────────

/// Attempt to open the default audio output device.
///
/// Returns `Err` if no output device is available (headless servers, CI, etc.).
pub fn initialize_audio() -> Result<AudioSystem, Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle)?));
    Ok(AudioSystem { _stream, sink })
}

/// Replace the current tone with a new frequency.
///
/// Stops the current sink, waits 100 ms for a clean transition, then starts the
/// new frequency.  The brief silence prevents audible clicks.
pub fn change_frequency(system: &AudioSystem, frequency: f32) {
    if let Ok(sink) = system.sink.lock() {
        sink.stop();
    }
    thread::sleep(Duration::from_millis(100));
    let wave = SineWave { frequency, sample_rate: 44100.0, phase: 0.0 };
    if let Ok(sink) = system.sink.lock() {
        sink.append(wave);
        sink.play();
    }
}

/// Stop all audio output.
pub fn stop_audio(system: &AudioSystem) {
    if let Ok(sink) = system.sink.lock() {
        sink.stop();
    }
}

// ─── Frequency tables ─────────────────────────────────────────────────────────

/// Map a digital root (1-9) to its corresponding Solfeggio frequency in Hz.
pub fn get_frequency_for_number(root: u32) -> f32 {
    match root {
        1 => 396.0,  // Liberation from fear and guilt
        2 => 417.0,  // Undoing situations and facilitating change
        3 => 528.0,  // Love frequency / DNA repair
        4 => 639.0,  // Connecting relationships
        5 => 741.0,  // Awakening intuition
        6 => 852.0,  // Returning to spiritual order
        7 => 963.0,  // God consciousness / pineal gland activation
        8 => 432.0,  // Universal harmony / natural tuning
        9 => 285.0,  // Healing and regeneration
        _ => 432.0,  // Default
    }
}

/// Human-readable name for a Solfeggio frequency value.
pub fn get_frequency_name(frequency: f32) -> &'static str {
    match frequency as u32 {
        285 => "Healing & Regeneration",
        396 => "Liberation from Fear",
        417 => "Facilitating Change",
        432 => "Universal Harmony",
        528 => "Love & DNA Repair",
        639 => "Connecting Relationships",
        741 => "Awakening Intuition",
        852 => "Returning to Spiritual Order",
        963 => "God Consciousness",
        _   => "Sacred Frequency",
    }
}

// ─── WAV export ───────────────────────────────────────────────────────────────

/// Write a WAV file to `exports/<filename>`.
///
/// * `binaural = false` → mono pure tone (left only); `beat_hz` is ignored.
/// * `binaural = true`  → stereo with right channel at `frequency + beat_hz`.
///
/// The traditional theta-wave preset is 6 Hz; pass 6.0 when no specific beat is required.
pub fn generate_audio_file(
    frequency: f32,
    duration_seconds: u32,
    filename: &str,
    binaural: bool,
    beat_hz: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("exports")?;
    let filepath = format!("exports/{}", filename);
    let spec = hound::WavSpec {
        channels:        if binaural { 2 } else { 1 },
        sample_rate:     44100,
        bits_per_sample: 16,
        sample_format:   hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(&filepath, spec)?;
    let sample_rate   = 44100.0_f32;
    let total_samples = duration_seconds * 44100;

    for i in 0..total_samples {
        let t = i as f32 / sample_rate;
        if binaural {
            let left  = (2.0 * PI * frequency               * t).sin();
            let right = (2.0 * PI * (frequency + beat_hz)   * t).sin();
            writer.write_sample((left  * 0.3 * i16::MAX as f32) as i16)?;
            writer.write_sample((right * 0.3 * i16::MAX as f32) as i16)?;
        } else {
            let sample = (2.0 * PI * frequency * t).sin();
            writer.write_sample((sample * 0.3 * i16::MAX as f32) as i16)?;
        }
    }
    writer.finalize()?;
    Ok(())
}

/// Write a custom stereo binaural beat WAV file.
///
/// Left channel: `base_freq` Hz; right channel: `base_freq + beat_freq` Hz.
/// 1-second fade-in / fade-out is applied to prevent audible clicks.
pub fn generate_binaural_beat(
    base_freq: f32,
    beat_freq: f32,
    duration_seconds: u32,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("exports")?;
    let filepath = format!("exports/{}", filename);
    let spec = hound::WavSpec {
        channels:        2,
        sample_rate:     44100,
        bits_per_sample: 16,
        sample_format:   hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(&filepath, spec)?;
    let sample_rate   = 44100.0_f32;
    let total_samples = duration_seconds * 44100;
    let fade_samples  = sample_rate as u32; // 1-second fade

    for i in 0..total_samples {
        let t = i as f32 / sample_rate;

        // Raised-cosine (Hann) envelope: smoother than linear, avoids click artefacts.
        // envelope(t) = 0.5 * (1 − cos(π·t))  where t ∈ [0,1] → 0…1 fade-in
        let fade_factor = if i < fade_samples {
            let t = i as f32 / fade_samples as f32;
            0.5 * (1.0 - (PI * t).cos())
        } else if i > total_samples - fade_samples {
            let t = (total_samples - i) as f32 / fade_samples as f32;
            0.5 * (1.0 - (PI * t).cos())
        } else {
            1.0_f32
        };

        let left  = (2.0 * PI * base_freq               * t).sin();
        let right = (2.0 * PI * (base_freq + beat_freq) * t).sin();

        writer.write_sample((left  * 0.3 * fade_factor * i16::MAX as f32) as i16)?;
        writer.write_sample((right * 0.3 * fade_factor * i16::MAX as f32) as i16)?;
    }
    writer.finalize()?;
    Ok(())
}

// ─── Export helpers ───────────────────────────────────────────────────────────

/// The full Solfeggio set used throughout export menus.
pub const SOLFEGGIO_FREQUENCIES: &[(f32, &str, &str)] = &[
    (285.0, "Healing_Regeneration",       "🌿"),
    (396.0, "Liberation_Fear",            "🛡️"),
    (417.0, "Facilitating_Change",        "🦋"),
    (432.0, "Universal_Harmony",          "🌌"),
    (528.0, "Love_DNA_Repair",            "💖"),
    (639.0, "Connecting_Relationships",   "🤝"),
    (741.0, "Awakening_Intuition",        "🔮"),
    (852.0, "Spiritual_Order",            "👼"),
    (963.0, "God_Consciousness",          "🌟"),
];

/// Export a single frequency with the user's choice of duration and mono/binaural type.
pub fn export_frequency(frequency: f32, name: &str) {
    println!("\n{}", "🎵 Export Options:".bold().bright_cyan());
    println!("{}", "1. Pure Tone (5 minutes)".bright_white());
    println!("{}", "2. Binaural Beat (10 minutes)".bright_white());
    println!("{}", "3. Extended Session (30 minutes)".bright_white());

    print!("\n{}", "Choose export type (1-3): ".cyan());
    io::stdout().flush().unwrap_or(());

    let mut export_type = String::new();
    if io::stdin().read_line(&mut export_type).is_err() { return; }

    let (duration, suffix, binaural) = match export_type.trim() {
        "1" => (300,  "_pure_5min",       false),
        "2" => (600,  "_binaural_10min",  true),
        "3" => (1800, "_extended_30min",  true),
        _   => {
            println!("{}", "Invalid choice. Using default 10-minute binaural.".yellow());
            (600, "_binaural_10min", true)
        }
    };

    let filename = format!("{}Hz_{}{}.wav", frequency as u32, name, suffix);
    println!("\n{}", format!("🎶 Generating {} Hz frequency…", frequency).bright_magenta());

    match generate_audio_file(frequency, duration, &filename, binaural, 6.0) {
        Ok(_) => {
            println!("{}", format!("✅ Successfully exported: {}", filename).bright_green());
            println!("{}", format!("   Duration: {} minutes", duration / 60).dimmed());
            println!("{}", format!("   Location: ./exports/{}", filename).dimmed());
        }
        Err(e) => println!("{}", format!("❌ Export failed: {}", e).bright_red()),
    }
}

/// Export all nine Solfeggio frequencies as 10-minute binaural WAV files.
pub fn export_all_frequencies() {
    println!("\n{}", "🌟 Exporting Complete Solfeggio Set…".bold().bright_magenta());
    let (mut ok, mut fail) = (0u32, 0u32);
    for (freq, name, _) in SOLFEGGIO_FREQUENCIES {
        let filename = format!("{}Hz_{}_binaural_10min.wav", *freq as u32, name);
        print!("{}", format!("Generating {} Hz… ", freq).dimmed());
        io::stdout().flush().unwrap_or(());
        match generate_audio_file(*freq, 600, &filename, true, 6.0) {
            Ok(_)  => { println!("{}", "✅".bright_green());  ok   += 1; }
            Err(_) => { println!("{}", "❌".bright_red());    fail += 1; }
        }
    }
    println!("\n{}", "📊 Export Summary:".bold());
    println!("{}", format!("   ✅ Successful: {}", ok).bright_green());
    if fail > 0 {
        println!("{}", format!("   ❌ Failed: {}", fail).bright_red());
    }
    println!("{}", "   📁 Location: ./exports/".dimmed());
}

/// CLI-only variant of `export_all_frequencies` (no menu prompts).
pub fn export_all_frequencies_cli() {
    println!("\n{}", "Creating exports directory…".dimmed());
    if let Err(e) = std::fs::create_dir_all("exports") {
        println!("{}", format!("❌ Failed to create exports directory: {}", e).bright_red());
        return;
    }
    let (mut ok, mut fail) = (0u32, 0u32);
    for (freq, name, _) in SOLFEGGIO_FREQUENCIES {
        let filename = format!("{}Hz_{}_binaural_10min.wav", *freq as u32, name);
        print!("{}", format!("🎶 Generating {} Hz ({})… ", freq, name.replace('_', " ")).bright_cyan());
        io::stdout().flush().unwrap_or(());
        match generate_audio_file(*freq, 600, &filename, true, 6.0) {
            Ok(_)  => { println!("{}", "✅".bright_green()); ok   += 1; }
            Err(e) => { println!("{}", format!("❌ Error: {}", e).bright_red()); fail += 1; }
        }
    }
    println!("\n{}", "🎉 Export Complete!".bold().bright_green());
    println!("{}", format!("   ✅ Successful: {} files", ok).bright_green());
    if fail > 0 {
        println!("{}", format!("   ❌ Failed: {} files", fail).bright_red());
    }
    println!("{}", "   📁 Location: ./exports/".bright_blue());
    println!("{}", "   🎧 Ready for meditation, sleep, or spiritual practice!".italic());
}

/// Interactive custom binaural-beat creator (prompts for base freq, beat, duration).
pub fn create_custom_binaural() {
    println!("\n{}", "🎨 Custom Binaural Beat Creator".bold().bright_yellow());
    println!();

    let base_freq = prompt_f32("Enter base frequency (Hz, e.g., 432): ", 20.0, 20000.0, 432.0);
    let beat_freq = prompt_f32("Enter beat frequency (Hz, e.g., 6 for theta waves): ", 0.1, 50.0, 6.0);
    let duration_minutes = prompt_u32("Enter duration in minutes (e.g., 15): ", 1, 120, 15);

    let filename = format!(
        "custom_{}Hz_{}beat_{}min.wav",
        base_freq as u32, beat_freq, duration_minutes
    );

    println!("\n{}", "🎶 Creating custom binaural beat…".bright_magenta());
    println!("{}", format!("   Base: {} Hz",     base_freq).dimmed());
    println!("{}", format!("   Beat: {} Hz",     beat_freq).dimmed());
    println!("{}", format!("   Duration: {} minutes", duration_minutes).dimmed());

    match generate_binaural_beat(base_freq, beat_freq, duration_minutes * 60, &filename) {
        Ok(_) => {
            println!("{}", format!("✅ Custom binaural beat created: {}", filename).bright_green());
            println!("{}", format!("   Location: ./exports/{}", filename).dimmed());
        }
        Err(e) => println!("{}", format!("❌ Creation failed: {}", e).bright_red()),
    }
}

// ─── Private prompt helpers ───────────────────────────────────────────────────

fn prompt_f32(prompt: &str, min: f32, max: f32, default: f32) -> f32 {
    print!("{}", prompt.cyan());
    io::stdout().flush().unwrap_or(());
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_err() { return default; }
    buf.trim().parse::<f32>()
        .ok()
        .filter(|&v| v >= min && v <= max)
        .unwrap_or_else(|| {
            println!("{}", format!("Invalid value. Using {} Hz.", default).yellow());
            default
        })
}

fn prompt_u32(prompt: &str, min: u32, max: u32, default: u32) -> u32 {
    print!("{}", prompt.cyan());
    io::stdout().flush().unwrap_or(());
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_err() { return default; }
    buf.trim().parse::<u32>()
        .ok()
        .filter(|&v| v >= min && v <= max)
        .unwrap_or_else(|| {
            println!("{}", format!("Invalid value. Using {}.", default).yellow());
            default
        })
}
