//! Persistence layer — SQLite-backed user profiles and RNG session history.
//!
//! The database is stored at `data/cosmic_knowledge.db` relative to the working
//! directory (created alongside `exports/` on first use).
//!
//! ## Schema
//!
//! ```sql
//! users (
//!     id         TEXT PRIMARY KEY,   -- UUID v4
//!     name       TEXT NOT NULL,
//!     created_at TEXT NOT NULL       -- "YYYY-MM-DD HH:MM:SS UTC"
//! )
//!
//! rng_sessions (
//!     id           INTEGER PRIMARY KEY AUTOINCREMENT,
//!     user_id      TEXT    NOT NULL REFERENCES users(id),
//!     started_at   TEXT    NOT NULL,
//!     range_min    INTEGER NOT NULL,
//!     range_max    INTEGER NOT NULL,
//!     delay_secs   REAL    NOT NULL,
//!     outcome      TEXT    NOT NULL,   -- "match" | "stopped"
//!     draws        INTEGER NOT NULL,   -- draw # of match, or total draws if stopped
//!     beat_chance  INTEGER NOT NULL    -- 1 when draws < range_size, 0 otherwise
//! )
//! ```

use std::fs;

use colored::*;
use rusqlite::{params, Connection};

// ─── Schema ───────────────────────────────────────────────────────────────────

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS users (
    id         TEXT PRIMARY KEY,
    name       TEXT NOT NULL,
    created_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS rng_sessions (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id     TEXT    NOT NULL,
    started_at  TEXT    NOT NULL,
    range_min   INTEGER NOT NULL,
    range_max   INTEGER NOT NULL,
    delay_secs  REAL    NOT NULL,
    outcome     TEXT    NOT NULL,
    draws       INTEGER NOT NULL,
    beat_chance INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
";

// ─── Public types ─────────────────────────────────────────────────────────────

/// A registered user record.
pub struct UserRecord {
    pub id:   String,
    pub name: String,
}

/// Aggregate statistics computed over all of a user's RNG sessions.
pub struct CumulativeStats {
    /// Total sessions recorded (both match and stopped).
    pub total_sessions:    u32,
    /// Mean number of draws across all sessions.
    pub mean_draws:        f64,
    /// Fewest draws to a confirmed match (`None` if no matches yet).
    pub best_match_draw:   Option<u32>,
    /// Sessions where `draws < range_size` (beat chance expectation).
    pub beat_chance_count: u32,
    /// `mean(draws / range_size)` across all sessions.
    ///
    /// A ratio < 1.0 means the user tends to match earlier than chance;
    /// > 1.0 means later.  Exactly 1.0 is at chance expectation.
    pub tendency_ratio:    f64,
}

// ─── Database connection ──────────────────────────────────────────────────────

/// Open (or create) the application database at `data/cosmic_knowledge.db`.
///
/// Both tables are created if they do not yet exist.
pub fn open_db() -> rusqlite::Result<Connection> {
    fs::create_dir_all("data").ok();
    let conn = Connection::open("data/cosmic_knowledge.db")?;
    conn.execute_batch(SCHEMA)?;
    Ok(conn)
}

// ─── UUID v4 ──────────────────────────────────────────────────────────────────

/// Generate a random UUID v4 string using the existing `getrandom` dependency.
fn new_uuid() -> String {
    let mut b = [0u8; 16];
    getrandom::getrandom(&mut b).expect("getrandom unavailable");
    b[6] = (b[6] & 0x0F) | 0x40; // version 4
    b[8] = (b[8] & 0x3F) | 0x80; // variant 1 (RFC 4122)
    format!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        u32::from_be_bytes(b[0..4].try_into().unwrap()),
        u16::from_be_bytes(b[4..6].try_into().unwrap()),
        u16::from_be_bytes(b[6..8].try_into().unwrap()),
        u16::from_be_bytes(b[8..10].try_into().unwrap()),
        {
            let n = &b[10..16];
            (n[0] as u64) << 40 | (n[1] as u64) << 32 | (n[2] as u64) << 24
                | (n[3] as u64) << 16 | (n[4] as u64) << 8 | n[5] as u64
        },
    )
}

fn now_utc() -> String {
    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

// ─── User operations ──────────────────────────────────────────────────────────

/// Look up a user by name (case-insensitive) or create a new profile.
///
/// Returns `(record, is_new)`.
pub fn get_or_create_user(conn: &Connection, name: &str) -> rusqlite::Result<(UserRecord, bool)> {
    let name = name.trim();

    let existing: rusqlite::Result<(String, String)> = conn.query_row(
        "SELECT id, name FROM users WHERE lower(name) = lower(?1)",
        params![name],
        |row| Ok((row.get(0)?, row.get(1)?)),
    );

    match existing {
        Ok((id, canonical)) => Ok((UserRecord { id, name: canonical }, false)),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            let id = new_uuid();
            conn.execute(
                "INSERT INTO users (id, name, created_at) VALUES (?1, ?2, ?3)",
                params![id, name, now_utc()],
            )?;
            Ok((UserRecord { id, name: name.to_string() }, true))
        }
        Err(e) => Err(e),
    }
}

// ─── Session recording ────────────────────────────────────────────────────────

/// Persist one completed RNG experiment session.
pub fn record_session(
    conn:        &Connection,
    user_id:     &str,
    started_at:  &str,
    range_min:   u32,
    range_max:   u32,
    delay_secs:  f64,
    outcome:     &str,
    draws:       u32,
    beat_chance: bool,
) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO rng_sessions \
         (user_id, started_at, range_min, range_max, delay_secs, outcome, draws, beat_chance) \
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",
        params![
            user_id, started_at,
            range_min, range_max, delay_secs,
            outcome, draws, beat_chance as i32,
        ],
    )?;
    Ok(())
}

// ─── Statistics ───────────────────────────────────────────────────────────────

/// Compute aggregate statistics for a user across all their RNG sessions.
pub fn get_stats(conn: &Connection, user_id: &str) -> rusqlite::Result<CumulativeStats> {
    let row: (u32, Option<f64>, Option<u32>, Option<u32>, Option<f64>) = conn.query_row(
        "SELECT
             COUNT(*),
             AVG(CAST(draws AS REAL)),
             MIN(CASE WHEN outcome = 'match' THEN draws ELSE NULL END),
             SUM(beat_chance),
             AVG(CAST(draws AS REAL) / CAST(range_max - range_min + 1 AS REAL))
         FROM rng_sessions
         WHERE user_id = ?1",
        params![user_id],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
    )?;

    Ok(CumulativeStats {
        total_sessions:    row.0,
        mean_draws:        row.1.unwrap_or(0.0),
        best_match_draw:   row.2,
        beat_chance_count: row.3.unwrap_or(0),
        tendency_ratio:    row.4.unwrap_or(1.0),
    })
}

// ─── Display ─────────────────────────────────────────────────────────────────

/// Print a cumulative statistics panel for a user.
pub fn print_cumulative_stats(name: &str, stats: &CumulativeStats) {
    println!();
    println!("{}", "  ╔══════════════════════════════════════════════════════════╗".bright_cyan());
    println!(
        "{}",
        format!("  ║  📊  PSI HISTORY — {:<38}║", name.to_uppercase())
            .bold().bright_cyan()
    );
    println!("{}", "  ╠══════════════════════════════════════════════════════════╣".bright_cyan());

    println!(
        "  ║  {}  {}",
        format!("{:<26}", "Sessions recorded:").bold(),
        stats.total_sessions.to_string().bright_white(),
    );
    println!(
        "  ║  {}  {}",
        format!("{:<26}", "Mean draws per session:").bold(),
        format!("{:.2}", stats.mean_draws).bright_white(),
    );

    if let Some(best) = stats.best_match_draw {
        println!(
            "  ║  {}  {}",
            format!("{:<26}", "Personal best match:").bold(),
            format!("draw #{}", best).bold().bright_green(),
        );
    }

    println!(
        "  ║  {}  {}",
        format!("{:<26}", "Beat chance:").bold(),
        format!("{}/{} sessions", stats.beat_chance_count, stats.total_sessions)
            .bright_cyan(),
    );

    let tendency_str = if stats.tendency_ratio < 0.95 {
        format!("{:.2}× — tends earlier than chance  ✦", stats.tendency_ratio)
            .bright_green().to_string()
    } else if stats.tendency_ratio > 1.05 {
        format!("{:.2}× — tends later than chance", stats.tendency_ratio)
            .yellow().to_string()
    } else {
        format!("{:.2}× — near chance expectation", stats.tendency_ratio)
            .dimmed().to_string()
    };
    println!(
        "  ║  {}  {}",
        format!("{:<26}", "Overall tendency:").bold(),
        tendency_str,
    );

    if stats.total_sessions < 10 {
        println!("{}", "  ╠══════════════════════════════════════════════════════════╣".bright_cyan());
        println!("{}", "  ║  Trends emerge after ~10 sessions — keep experimenting. ║".italic().dimmed());
    }

    println!("{}", "  ╚══════════════════════════════════════════════════════════╝".bright_cyan());
    println!();
}
