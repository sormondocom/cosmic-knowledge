//! Gregorian chant synthesis for the angelic hymn experience.
//!
//! Generates monophonic Gregorian-style chant melodies and meditative modal
//! tones using additive synthesis (harmonic series with three detuned voice
//! layers) and a cathedral reverb tail.  Entirely cross-platform — all
//! synthesis is pure Rust arithmetic via `rodio`.
//!
//! Both public functions are **non-blocking**: the chant is synthesised and
//! played on a background thread; the caller returns immediately so the hymn
//! text remains visible while the music plays.

#[cfg(not(target_os = "android"))]
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};
#[cfg(not(target_os = "android"))]
use std::f32::consts::PI;

const SR: u32 = 44_100;
const SRF: f32 = SR as f32;

// ─── Note frequencies (Hz) ───────────────────────────────────────────────────

const C3: f32 = 130.813;
const D3: f32 = 146.832;
const E3: f32 = 164.814;
const F3: f32 = 174.614;
const G3: f32 = 195.998;
const A3: f32 = 220.000;
const Bb3: f32 = 233.082;
const C4: f32 = 261.626;
const D4: f32 = 293.665;
const R: f32 = 0.0; // rest / silence

// ─── Melody encoding ──────────────────────────────────────────────────────────
//
// Each step is (frequency_hz, duration_seconds).
// R (0.0 Hz) produces silence for the given duration.

type Step = (f32, f32);

// ─────────────────────────────────────────────────────────────────────────────
// GREGORIAN MELODIES
// Indexed to match ANGELIC_HYMNS in tarot/session.rs:
//   0  Sanctus            1  Gloria in Excelsis
//   2  Trisagion          3  Te Deum Laudamus
//   4  Agnus Dei          5  Veni Creator Spiritus
//   6  Alma Redemptoris   7  O Lux Beata Trinitas
// ─────────────────────────────────────────────────────────────────────────────

// [0] Sanctus — Mode VII (Mixolydian), finalis G
//     Simplified ferial Sanctus tone, ~22 s
static SANCTUS: &[Step] = &[
    (G3,1.1),(A3,0.5),(G3,1.5),(R,0.4),
    (G3,1.1),(A3,0.5),(G3,1.5),(R,0.4),
    (G3,0.5),(A3,0.5),(Bb3,0.5),(A3,0.5),(G3,0.5),(F3,0.5),(E3,0.5),(D3,0.5),
    (E3,0.5),(D3,2.0),(R,0.5),
    (D3,0.5),(E3,0.5),(D3,0.5),(C3,0.5),(D3,1.8),(R,0.3),
    (E3,0.5),(F3,0.5),(G3,1.0),(R,0.3),
    (G3,0.5),(A3,0.5),(G3,0.5),(F3,0.5),(E3,0.5),(D3,0.5),(E3,0.5),(D3,3.0),
];

// [1] Gloria in Excelsis — Mode VI (Hypolydian), finalis F
//     Ferial tone flowing from reciting note down to finalis, ~20 s
static GLORIA: &[Step] = &[
    (C4,1.0),(C4,0.5),(C4,0.5),(Bb3,0.5),(C4,0.5),(R,0.3),
    (C4,0.5),(D4,0.5),(C4,0.5),(Bb3,0.5),(A3,0.5),(G3,0.5),(F3,2.0),(R,0.4),
    (A3,0.5),(Bb3,0.5),(A3,0.5),(G3,0.5),(F3,0.5),(E3,0.5),(F3,2.0),(R,0.4),
    (G3,0.5),(A3,0.5),(Bb3,0.5),(A3,0.5),(G3,0.5),(F3,0.5),(G3,0.5),(F3,0.5),
    (E3,0.5),(F3,3.0),
];

// [2] Trisagion — Mode III (Phrygian), finalis E
//     Characteristic half-step Phrygian opening  E – F, ~18 s
static TRISAGION: &[Step] = &[
    (E3,1.2),(F3,0.5),(E3,1.0),(D3,0.5),(E3,0.5),(R,0.4),
    (E3,0.5),(G3,0.5),(A3,0.5),(G3,0.5),(F3,0.5),(E3,2.0),(R,0.4),
    (G3,0.5),(A3,0.5),(Bb3,0.5),(A3,0.5),(G3,0.5),(F3,0.5),(E3,0.5),(D3,0.5),
    (E3,3.0),
];

// [3] Te Deum Laudamus — Mode IV (Hypophrygian), finalis E
//     Solemn chant opening, broad arching phrase, ~20 s
static TE_DEUM: &[Step] = &[
    (C4,0.5),(Bb3,0.5),(A3,0.5),(G3,0.5),(F3,0.5),(G3,0.5),(A3,1.2),(R,0.3),
    (A3,0.5),(G3,0.5),(F3,0.5),(G3,0.5),(A3,0.5),(Bb3,0.5),(A3,0.5),(G3,2.0),(R,0.4),
    (G3,0.5),(A3,0.5),(G3,0.5),(F3,0.5),(E3,0.5),(F3,0.5),(G3,0.5),(F3,0.5),(E3,3.2),
];

// [4] Agnus Dei — Mode VII (Mixolydian), finalis G
//     Gently arching phrases across three petitions, ~22 s
static AGNUS_DEI: &[Step] = &[
    (G3,1.2),(A3,0.5),(Bb3,0.5),(A3,0.5),(G3,1.5),(R,0.3),
    (G3,0.5),(F3,0.5),(G3,0.5),(A3,0.5),(G3,1.0),(F3,0.5),(E3,1.2),(R,0.3),
    (G3,0.5),(A3,0.5),(G3,0.5),(F3,0.5),(E3,0.5),(D3,0.5),(E3,0.5),(D3,0.5),(R,0.3),
    (D3,0.5),(E3,0.5),(F3,0.5),(G3,0.5),(A3,0.5),(G3,0.5),(F3,0.5),(G3,3.2),
];

// [5] Veni Creator Spiritus — Mode VIII (Hypomixolydian), finalis G
//     One of the most-sung Latin hymn melodies, ~22 s
static VENI_CREATOR: &[Step] = &[
    (G3,1.2),(A3,0.5),(G3,0.5),(F3,0.5),(G3,1.5),(R,0.3),
    (D4,0.5),(D4,0.5),(C4,0.5),(Bb3,0.5),(A3,1.2),(R,0.3),
    (G3,0.5),(A3,0.5),(Bb3,0.5),(A3,0.5),(G3,0.5),(F3,0.5),(E3,0.5),(D3,1.5),(R,0.3),
    (D3,0.5),(E3,0.5),(F3,0.5),(G3,0.5),(A3,0.5),(Bb3,0.5),(A3,0.5),(G3,3.0),
];

// [6] Alma Redemptoris Mater — Mode V (Lydian), finalis F
//     Beautiful ascending opening, then descending arch, ~22 s
static ALMA_REDEMPTORIS: &[Step] = &[
    (F3,0.5),(G3,0.5),(A3,0.5),(Bb3,0.5),(C4,0.5),(Bb3,0.5),(A3,1.2),(R,0.3),
    (G3,0.5),(A3,0.5),(G3,0.5),(F3,0.5),(E3,0.5),(F3,1.5),(R,0.3),
    (A3,0.5),(Bb3,0.5),(C4,0.5),(D4,0.5),(C4,0.5),(Bb3,0.5),(A3,0.5),(G3,0.5),
    (F3,2.0),(R,0.4),
    (F3,0.5),(G3,0.5),(A3,0.5),(Bb3,0.5),(A3,0.5),(G3,0.5),(F3,3.0),
];

// [7] O Lux Beata Trinitas — Mode II (Hypodorian), finalis D
//     St. Ambrose (~4th c.) — one of the oldest surviving hymn tunes, ~22 s
static O_LUX_BEATA: &[Step] = &[
    (G3,1.0),(G3,0.5),(A3,0.5),(G3,0.5),(F3,0.5),(G3,1.2),(R,0.3),
    (D4,0.5),(C4,0.5),(Bb3,0.5),(A3,0.5),(G3,0.5),(F3,0.5),(G3,2.0),(R,0.4),
    (E3,0.5),(F3,0.5),(G3,0.5),(A3,0.5),(G3,0.5),(F3,0.5),(E3,0.5),(D3,0.5),
    (E3,0.5),(F3,0.5),(E3,0.5),(D3,3.0),
];

static GREGORIAN_MELODIES: &[&[Step]] = &[
    SANCTUS, GLORIA, TRISAGION, TE_DEUM,
    AGNUS_DEI, VENI_CREATOR, ALMA_REDEMPTORIS, O_LUX_BEATA,
];

// ─────────────────────────────────────────────────────────────────────────────
// CONTEMPLATIVE TONES
// Indexed to match CONTEMPLATIVE_OPENINGS in tarot/session.rs:
//   0 Jung (Your vision)    1 Rilke    2 Jung (Privilege)
//   3 Weil                  4 Buber    5 Thich Nhất Hạnh
//   6 Winnicott             7 Eckhart
// D Dorian palette: D3 E3 F3 G3 A3 C4 D4 — modal, meditative, non-denominational
// ─────────────────────────────────────────────────────────────────────────────

static CONTEM_0: &[Step] = &[  // Flowing ascent
    (D3,1.5),(E3,0.8),(G3,0.8),(A3,1.2),(R,0.4),
    (A3,0.8),(C4,0.8),(D4,1.5),(R,0.4),
    (D4,0.8),(C4,0.8),(A3,0.8),(G3,0.8),(E3,0.8),(D3,3.5),
];

static CONTEM_1: &[Step] = &[  // Gentle descent — Rilke
    (A3,1.2),(G3,0.8),(E3,0.8),(D3,1.5),(R,0.4),
    (D3,0.8),(F3,0.8),(G3,0.8),(A3,0.8),(C4,1.5),(R,0.4),
    (A3,0.8),(G3,0.8),(F3,0.8),(E3,0.8),(D3,3.5),
];

static CONTEM_2: &[Step] = &[  // Modal arc — Jung (Privilege)
    (D3,1.0),(E3,0.8),(F3,0.8),(G3,0.8),(A3,1.2),(R,0.4),
    (C4,0.8),(A3,0.8),(G3,0.8),(F3,0.8),(E3,1.5),(R,0.4),
    (E3,0.8),(G3,0.8),(A3,0.8),(G3,0.8),(E3,0.8),(D3,3.5),
];

static CONTEM_3: &[Step] = &[  // Still presence — Weil
    (G3,2.0),(R,0.3),(A3,1.5),(R,0.3),(G3,1.0),(E3,1.0),(D3,2.0),(R,0.4),
    (D3,0.8),(E3,0.8),(G3,1.2),(A3,1.5),(G3,0.8),(E3,0.8),(D3,3.5),
];

static CONTEM_4: &[Step] = &[  // Call and response — Buber
    (A3,1.0),(C4,0.8),(D4,1.2),(C4,0.8),(A3,1.5),(R,0.4),
    (G3,0.8),(A3,0.8),(G3,0.8),(E3,0.8),(D3,2.0),(R,0.4),
    (D3,0.8),(E3,0.8),(G3,0.8),(A3,0.8),(C4,0.8),(A3,0.8),(G3,0.8),(D3,3.0),
];

static CONTEM_5: &[Step] = &[  // Breath — Thich Nhất Hạnh
    (D4,1.5),(C4,0.8),(A3,1.2),(G3,0.8),(E3,1.5),(R,0.4),
    (D3,0.8),(G3,1.0),(A3,1.0),(G3,0.8),(E3,0.8),(D3,2.0),(R,0.4),
    (E3,0.8),(G3,0.8),(A3,0.8),(G3,0.8),(E3,1.0),(D3,3.0),
];

static CONTEM_6: &[Step] = &[  // Play — Winnicott
    (E3,0.8),(G3,0.8),(A3,0.8),(C4,0.8),(A3,0.8),(G3,1.5),(R,0.4),
    (G3,0.8),(A3,0.8),(G3,0.8),(E3,0.8),(D3,2.0),(R,0.4),
    (D3,0.8),(E3,0.8),(G3,0.8),(A3,0.8),(C4,0.8),(D4,1.5),(A3,0.8),(G3,0.8),
    (E3,0.8),(D3,3.0),
];

static CONTEM_7: &[Step] = &[  // The eye — Eckhart
    (D4,2.0),(R,0.3),(C4,1.5),(R,0.3),(A3,1.0),(G3,1.0),(E3,1.5),(R,0.4),
    (D3,0.8),(E3,0.8),(G3,0.8),(A3,0.8),(C4,0.8),(A3,0.8),(G3,1.5),(D3,2.5),
];

static CONTEMPLATIVE_MELODIES: &[&[Step]] = &[
    CONTEM_0, CONTEM_1, CONTEM_2, CONTEM_3,
    CONTEM_4, CONTEM_5, CONTEM_6, CONTEM_7,
];

// ─── Synthesis ────────────────────────────────────────────────────────────────

/// Render a melody to a buffer of mono f32 PCM samples at 44 100 Hz.
#[cfg(not(target_os = "android"))]
fn render_melody(steps: &[Step]) -> Vec<f32> {
    // Timbre constants — choral organ quality
    // Three voice layers detuned at −0.3 %, 0 %, +0.3 % for ensemble warmth.
    const DETUNE: &[f32] = &[0.997, 1.000, 1.003];
    // Six harmonics with naturally falling amplitudes (shaped for warmth).
    const HARM_AMPS: &[f32] = &[1.00, 0.50, 0.28, 0.16, 0.08, 0.04];
    // Peak of one sample: 3 layers × Σ amps = 3 × 2.06 ≈ 6.18
    // Scale so peak ≈ 0.22 (conservative, headroom for reverb).
    const SCALE: f32 = 0.036;

    const ATTACK_S: f32 = 0.055; // raised-cosine attack
    const RELEASE_S: f32 = 0.085; // linear release
    const GAP_S: f32 = 0.035; // inter-note articulation gap

    let attack_samp = (ATTACK_S * SRF) as usize;
    let release_samp = (RELEASE_S * SRF) as usize;
    let gap_samp = (GAP_S * SRF) as usize;

    let mut out: Vec<f32> = Vec::new();

    for &(freq, dur) in steps {
        let n_total = (dur * SRF) as usize;

        if freq < 1.0 {
            // Rest — pure silence
            out.extend(std::iter::repeat(0.0f32).take(n_total));
            continue;
        }

        let n_note = n_total.saturating_sub(gap_samp);

        for s in 0..n_note {
            // Amplitude envelope
            let env = if s < attack_samp {
                0.5 * (1.0 - (PI * s as f32 / attack_samp as f32).cos())
            } else if s + release_samp >= n_note {
                (n_note - s) as f32 / release_samp as f32
            } else {
                1.0_f32
            };

            // Additive synthesis: three detuned layers × six harmonics.
            // Phase computed as fractional part to stay numerically stable.
            let mut sample = 0.0_f32;
            for &det in DETUNE {
                for (h, &ha) in HARM_AMPS.iter().enumerate() {
                    let hf = freq * det * (h + 1) as f32;
                    let phi = (hf * s as f32 / SRF).fract();
                    sample += (2.0 * PI * phi).sin() * ha;
                }
            }
            out.push(sample * env * SCALE);
        }

        // Silent gap between notes for clean articulation
        out.extend(std::iter::repeat(0.0f32).take(gap_samp));
    }

    apply_cathedral_reverb(&mut out);
    out
}

/// Four-tap feedback delay approximating a large stone cathedral (~2.5 s RT60).
///
/// Delays are irrational multiples of each other (Schroeder criterion) to
/// avoid comb-filter coloration.  The result is mixed 50 % wet.
fn apply_cathedral_reverb(buf: &mut Vec<f32>) {
    let orig = buf.clone();
    let taps: &[(usize, f32)] = &[
        ((0.0297 * SRF) as usize, 0.40),
        ((0.0371 * SRF) as usize, 0.32),
        ((0.0537 * SRF) as usize, 0.22),
        ((0.0891 * SRF) as usize, 0.14),
        ((0.1270 * SRF) as usize, 0.08),
        ((0.1830 * SRF) as usize, 0.04),
    ];
    for &(delay, gain) in taps {
        for i in delay..buf.len() {
            buf[i] += orig[i - delay] * gain;
        }
    }
    // Peak-normalize to 0.88 to prevent any clipping.
    let peak = buf.iter().map(|s| s.abs()).fold(0.0_f32, f32::max);
    if peak > 0.88 {
        let scale = 0.88 / peak;
        for s in buf.iter_mut() {
            *s *= scale;
        }
    }
}

// ─── Public API ───────────────────────────────────────────────────────────────

/// Synthesise and play the Gregorian chant for the given hymn index.
///
/// **Non-blocking** — returns immediately.  The chant plays on a background
/// thread and stops naturally when the melody ends.  `hymn_index` matches
/// the position of the hymn in `ANGELIC_HYMNS` in `tarot/session.rs`.
#[cfg(not(target_os = "android"))]
pub fn play_gregorian_chant(hymn_index: usize) {
    let steps = GREGORIAN_MELODIES[hymn_index % GREGORIAN_MELODIES.len()];
    // Collect into an owned Vec so it can cross the thread boundary.
    let steps: Vec<Step> = steps.to_vec();
    std::thread::spawn(move || {
        let samples = render_melody(&steps);
        let Ok((_stream, handle)) = OutputStream::try_default() else {
            return;
        };
        let Ok(sink) = Sink::try_new(&handle) else { return };
        sink.append(SamplesBuffer::new(1, SR, samples));
        sink.sleep_until_end();
    });
}

/// Synthesise and play the meditative modal tone for the given opening index.
///
/// **Non-blocking** — returns immediately.  `opening_index` matches the
/// position of the text in `CONTEMPLATIVE_OPENINGS` in `tarot/session.rs`.
#[cfg(not(target_os = "android"))]
pub fn play_contemplative_tone(opening_index: usize) {
    let steps = CONTEMPLATIVE_MELODIES[opening_index % CONTEMPLATIVE_MELODIES.len()];
    let steps: Vec<Step> = steps.to_vec();
    std::thread::spawn(move || {
        let samples = render_melody(&steps);
        let Ok((_stream, handle)) = OutputStream::try_default() else {
            return;
        };
        let Ok(sink) = Sink::try_new(&handle) else { return };
        sink.append(SamplesBuffer::new(1, SR, samples));
        sink.sleep_until_end();
    });
}

// ─── Android stubs ────────────────────────────────────────────────────────────
// rodio is not compiled on Android (requires the NDK C++ runtime).
// Both functions are no-ops; the hymn text is still displayed by the caller.

#[cfg(target_os = "android")]
pub fn play_gregorian_chant(_hymn_index: usize) {}

#[cfg(target_os = "android")]
pub fn play_contemplative_tone(_opening_index: usize) {}
