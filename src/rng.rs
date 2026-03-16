//! Psi–RNG Experiment — mind-matter interaction session.
//!
//! Tests the hypothesis that focused human intention can influence the output
//! of a hardware random number generator.
//!
//! **Randomness source priority:**
//!  1. RDRAND CPU instruction (Intel/AMD hardware TRNG) — `rdrand` crate
//!  2. OS entropy fallback (`getrandom`) when RDRAND is unsupported
//!
//! **Session flow:**
//!  1. Select number range and inter-draw delay (seconds)
//!  2. Think of a number — do not reveal it until a match occurs
//!  3. Numbers are drawn at the configured interval
//!  4. Type `Y` + Enter when the displayed number matches your thought
//!  5. Type `Q` + Enter to end the session without confirming a match
//!  6. Session statistics compare your result against chance expectation
//!
//! **Statistical note:**
//! For a range of N values, the geometric distribution gives a mean of N
//! draws until the first match by pure chance.  A match significantly earlier
//! than N is consistent with (but does not prove) intentional influence.

use std::io::{self, Write};
use std::sync::mpsc;
use std::sync::mpsc::RecvTimeoutError;
use std::thread;
use std::time::Duration;

use colored::*;
use rdrand::RdRand;

// ─── Randomness source ────────────────────────────────────────────────────────

enum RngSource {
    Hardware(RdRand),
    Software,
}

impl RngSource {
    /// Draw one raw `u32`.  Falls back to OS entropy if RDRAND retries exhaust.
    fn next_u32(&self) -> u32 {
        match self {
            RngSource::Hardware(rng) => rng.try_next_u32().unwrap_or_else(|_| os_u32()),
            RngSource::Software      => os_u32(),
        }
    }

    fn label(&self) -> &'static str {
        match self {
            RngSource::Hardware(_) => "RDRAND  (Intel/AMD hardware TRNG)",
            RngSource::Software    => "OS entropy  (getrandom / BCryptGenRandom)",
        }
    }
}

fn os_u32() -> u32 {
    let mut buf = [0u8; 4];
    getrandom::getrandom(&mut buf).expect("getrandom failed — no OS entropy source");
    u32::from_le_bytes(buf)
}

fn init_rng() -> RngSource {
    match RdRand::new() {
        Ok(r)  => RngSource::Hardware(r),
        Err(_) => RngSource::Software,
    }
}

// ─── Session configuration ────────────────────────────────────────────────────

struct Config {
    min:        u32,
    max:        u32,
    delay_secs: f64,
}

impl Config {
    fn range_size(&self) -> u32 { self.max - self.min + 1 }
}

/// Map a raw `u32` uniformly into `[min, max]`.
/// Modulo bias is negligible at these range sizes.
fn clamp(raw: u32, min: u32, max: u32) -> u32 {
    min + raw % (max - min + 1)
}

fn configure() -> Config {
    // ── Range ────────────────────────────────────────────────────────────────
    println!();
    println!("  {}", "─ Select number range ────────────────────────────────────".dimmed());
    println!("    {}  Single digit   (1 – 9)", "1.".cyan());
    println!("    {}  Low range      (1 – 10)", "2.".cyan());
    println!("    {}  Wide range     (1 – 100)", "3.".cyan());
    println!("    {}  Broad range    (1 – 1,000)", "4.".cyan());
    println!("    {}  Custom", "5.".cyan());
    println!();
    print!("  {} ", "▸ Choice [default 1]:".bold().cyan());
    io::stdout().flush().unwrap_or(());

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap_or(0);

    let (min, max) = match buf.trim() {
        "2"            => (1, 10),
        "3"            => (1, 100),
        "4"            => (1, 1_000),
        "5"            => read_custom_range(),
        _ /* "1" | "" */ => (1, 9),
    };

    // ── Delay ────────────────────────────────────────────────────────────────
    println!();
    println!("  {}", "─ Set draw interval ──────────────────────────────────────".dimmed());
    print!("  {} ", "▸ Seconds between each draw (1–60) [default 3]:".bold().cyan());
    io::stdout().flush().unwrap_or(());

    let mut dbuf = String::new();
    io::stdin().read_line(&mut dbuf).unwrap_or(0);

    let delay_secs = dbuf.trim()
        .parse::<f64>()
        .unwrap_or(3.0)
        .clamp(1.0, 60.0);

    Config { min, max, delay_secs }
}

fn read_custom_range() -> (u32, u32) {
    let read_u32 = |prompt: &str| -> u32 {
        loop {
            print!("    {} ", prompt);
            io::stdout().flush().unwrap_or(());
            let mut b = String::new();
            io::stdin().read_line(&mut b).unwrap_or(0);
            if let Ok(n) = b.trim().parse::<u32>() { return n; }
            println!("{}", "    Please enter a whole number.".yellow());
        }
    };

    loop {
        let lo = read_u32("Minimum value:");
        let hi = read_u32("Maximum value:");
        if hi > lo {
            return (lo, hi);
        }
        println!("{}", "    Maximum must be greater than minimum.".yellow());
    }
}

// ─── Session outcome ──────────────────────────────────────────────────────────

enum Outcome {
    Match   { on_draw: u32 },
    Stopped { total:   u32 },
}

// ─── Public entry point ───────────────────────────────────────────────────────

/// Run the Psi–RNG interactive session.
pub fn run_rng_session() {
    println!();
    println!("{}", "╔════════════════════════════════════════════════════════════╗".bright_magenta());
    println!("{}", "║            🎲  PSI – RNG EXPERIMENT  🎲                    ║".bold().bright_magenta());
    println!("{}", "╠════════════════════════════════════════════════════════════╣".bright_magenta());
    println!("{}", "║  Can focused intention influence a hardware random number  ║".dimmed());
    println!("{}", "║  generator?  Think of a number — watch for your match.    ║".dimmed());
    println!("{}", "╚════════════════════════════════════════════════════════════╝".bright_magenta());

    let src = init_rng();
    println!();
    println!("  {}  {}", "RNG source:".bold(), src.label().bright_cyan());

    let cfg = configure();

    println!();
    println!("  {}", "─ How to participate ─────────────────────────────────────".dimmed());
    println!("  1. Choose a number between {} and {} — hold it clearly in your mind.",
        cfg.min.to_string().bright_white(), cfg.max.to_string().bright_white());
    println!("  2. Numbers will draw every {} second(s).",
        format!("{:.1}", cfg.delay_secs).bright_white());
    println!("  3. When the number shown matches yours: type {} and press Enter.",
        "Y".bold().bright_green());
    println!("  4. To end without a confirmed match:   type {} and press Enter.",
        "Q".bold().bright_red());
    println!();
    print!("  {} ", "Ready? Press Enter to begin…".italic().bright_yellow());
    io::stdout().flush().unwrap_or(());

    // Spawn a dedicated stdin-reader thread so draws aren't gated on user input.
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        loop {
            let mut line = String::new();
            if io::stdin().read_line(&mut line).is_err() { break; }
            if tx.send(line.trim().to_lowercase().to_string()).is_err() { break; }
        }
    });

    // Consume the "begin" Enter.
    let _ = rx.recv();

    println!();
    println!("  {}", "─ Session running ────────────────────────────────────────".dimmed());
    println!("  {}", "[Y = match · Q = stop · or just watch and wait]".dimmed());
    println!();

    let delay   = Duration::from_secs_f64(cfg.delay_secs);
    let mut draw = 0u32;

    let outcome = loop {
        let n = clamp(src.next_u32(), cfg.min, cfg.max);
        draw += 1;

        println!("  {}  {}  →  {}",
            "⟁".bright_magenta(),
            format!("Draw {:>5}", draw).dimmed(),
            format!("{:>6}", n).bold().bright_white(),
        );

        match rx.recv_timeout(delay) {
            Ok(s) if s.starts_with('y') => break Outcome::Match   { on_draw: draw },
            Ok(s) if s.starts_with('q') => break Outcome::Stopped { total:   draw },
            Err(RecvTimeoutError::Disconnected) => break Outcome::Stopped { total: draw },
            _ => {} // timeout or unrecognised input → continue
        }
    };

    show_results(&cfg, &outcome);
}

// ─── Results display ──────────────────────────────────────────────────────────

fn show_results(cfg: &Config, outcome: &Outcome) {
    let n = cfg.range_size();

    println!();
    println!("{}", "  ╔══════════════════════════════════════════════════════════╗".bright_magenta());
    println!("{}", "  ║                   SESSION RESULTS                        ║".bold().bright_magenta());
    println!("{}", "  ╠══════════════════════════════════════════════════════════╣".bright_magenta());

    println!("  ║  {}  {}",
        format!("{:<20}", "Range:").bold(),
        format!("{} – {}  ({} values)", cfg.min, cfg.max, n).bright_white(),
    );
    println!("  ║  {}  {}",
        format!("{:<20}", "Draw interval:").bold(),
        format!("{:.1} second(s)", cfg.delay_secs).bright_white(),
    );
    println!("  ║  {}  {}",
        format!("{:<20}", "Chance expectation:").bold(),
        format!("{} draws on average", n).bright_white(),
    );

    println!("{}", "  ╠══════════════════════════════════════════════════════════╣".bright_magenta());

    match outcome {
        Outcome::Match { on_draw } => {
            println!("  ║  {}  {}",
                format!("{:<20}", "Match on draw:").bold(),
                format!("#{}", on_draw).bold().bright_green(),
            );

            let interpretation = if *on_draw < n {
                let early = n - on_draw;
                format!(
                    "{} draw(s) EARLIER than chance expectation  ✦",
                    early
                ).bright_green().to_string()
            } else if *on_draw > n {
                let late = on_draw - n;
                format!(
                    "{} draw(s) later than chance expectation",
                    late
                ).yellow().to_string()
            } else {
                "Exactly at chance expectation".bright_white().to_string()
            };

            println!("  ║  {}  {}", format!("{:<20}", "vs. expectation:").bold(), interpretation);

            // Probability of matching on or before this draw by chance alone
            // P(X <= k) = 1 - ((N-1)/N)^k
            let p_by_chance = 1.0 - ((n as f64 - 1.0) / n as f64).powi(*on_draw as i32);
            println!("  ║  {}  {}",
                format!("{:<20}", "Prob. by chance:").bold(),
                format!("{:.1}% chance of matching by draw {} by luck alone",
                    p_by_chance * 100.0, on_draw).dimmed(),
            );
        }

        Outcome::Stopped { total } => {
            println!("  ║  {}  {}",
                format!("{:<20}", "Total draws:").bold(),
                total.to_string().bright_white(),
            );
            println!("  ║  {}",
                "No match confirmed — session ended by user.".yellow(),
            );
            let p_no_match = ((n as f64 - 1.0) / n as f64).powi(*total as i32);
            println!("  ║  {}  {}",
                format!("{:<20}", "Prob. no match:").bold(),
                format!("{:.1}% chance of 0 hits in {} draws by luck alone",
                    p_no_match * 100.0, total).dimmed(),
            );
        }
    }

    println!("{}", "  ╠══════════════════════════════════════════════════════════╣".bright_magenta());
    println!("{}", "  ║  Note: a single trial is not statistically conclusive.   ║".dimmed());
    println!("{}", "  ║  Repeat sessions accumulate evidence over time.          ║".dimmed());
    println!("{}", "  ╚══════════════════════════════════════════════════════════╝".bright_magenta());
    println!();
}
