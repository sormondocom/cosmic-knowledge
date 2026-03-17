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
CREATE TABLE IF NOT EXISTS readings (
    id          TEXT    PRIMARY KEY,
    user_id     TEXT    NOT NULL,
    drawn_at    TEXT    NOT NULL,
    tradition   TEXT    NOT NULL,
    spread_type TEXT    NOT NULL,
    cards       TEXT    NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
";

// ─── Public types ─────────────────────────────────────────────────────────────

/// A registered user record.
pub struct UserRecord {
    pub id: String,
    pub name: String,
}

/// Aggregate statistics computed over all of a user's RNG sessions.
pub struct CumulativeStats {
    /// Total sessions recorded (both match and stopped).
    pub total_sessions: u32,
    /// Mean number of draws across all sessions.
    pub mean_draws: f64,
    /// Fewest draws to a confirmed match (`None` if no matches yet).
    pub best_match_draw: Option<u32>,
    /// Sessions where `draws < range_size` (beat chance expectation).
    pub beat_chance_count: u32,
    /// `mean(draws / range_size)` across all sessions.
    ///
    /// A ratio < 1.0 means the user tends to match earlier than chance;
    /// > 1.0 means later.  Exactly 1.0 is at chance expectation.
    pub tendency_ratio: f64,
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
            (n[0] as u64) << 40
                | (n[1] as u64) << 32
                | (n[2] as u64) << 24
                | (n[3] as u64) << 16
                | (n[4] as u64) << 8
                | n[5] as u64
        },
    )
}

fn now_utc() -> String {
    chrono::Utc::now()
        .format("%Y-%m-%d %H:%M:%S UTC")
        .to_string()
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
        Ok((id, canonical)) => Ok((
            UserRecord {
                id,
                name: canonical,
            },
            false,
        )),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            let id = new_uuid();
            conn.execute(
                "INSERT INTO users (id, name, created_at) VALUES (?1, ?2, ?3)",
                params![id, name, now_utc()],
            )?;
            Ok((
                UserRecord {
                    id,
                    name: name.to_string(),
                },
                true,
            ))
        }
        Err(e) => Err(e),
    }
}

// ─── Session recording ────────────────────────────────────────────────────────

/// Persist one completed RNG experiment session.
#[allow(clippy::too_many_arguments)]
pub fn record_session(
    conn: &Connection,
    user_id: &str,
    started_at: &str,
    range_min: u32,
    range_max: u32,
    delay_secs: f64,
    outcome: &str,
    draws: u32,
    beat_chance: bool,
) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO rng_sessions \
         (user_id, started_at, range_min, range_max, delay_secs, outcome, draws, beat_chance) \
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",
        params![
            user_id,
            started_at,
            range_min,
            range_max,
            delay_secs,
            outcome,
            draws,
            beat_chance as i32,
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
        total_sessions: row.0,
        mean_draws: row.1.unwrap_or(0.0),
        best_match_draw: row.2,
        beat_chance_count: row.3.unwrap_or(0),
        tendency_ratio: row.4.unwrap_or(1.0),
    })
}

// ─── Reading records ──────────────────────────────────────────────────────────

/// A stored reading record (Tarot, Runes, Oracle, etc.).
pub struct ReadingRecord {
    pub id: String,
    pub user_name: String,
    pub drawn_at: String,
    pub tradition: String,
    pub spread_type: String,
    pub cards: String,
}

/// Persist a divination reading for a named user.
///
/// `tradition`   — e.g. "Angelic Tarot", "Elder Futhark", "Lenormand"
/// `spread_type` — e.g. "Single Card", "Three-Card", "Nine-Rune Cast"
/// `cards`       — newline-separated list of drawn card/rune descriptions
pub fn record_reading(
    conn: &Connection,
    user_id: &str,
    tradition: &str,
    spread_type: &str,
    cards: &str,
) -> rusqlite::Result<String> {
    let id = new_uuid();
    conn.execute(
        "INSERT INTO readings (id, user_id, drawn_at, tradition, spread_type, cards) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, user_id, now_utc(), tradition, spread_type, cards],
    )?;
    Ok(id)
}

/// Retrieve all readings for a given user, ordered newest-first.
pub fn get_user_readings(
    conn: &Connection,
    user_id: &str,
) -> rusqlite::Result<Vec<ReadingRecord>> {
    let mut stmt = conn.prepare(
        "SELECT r.id, u.name, r.drawn_at, r.tradition, r.spread_type, r.cards
         FROM readings r JOIN users u ON r.user_id = u.id
         WHERE r.user_id = ?1
         ORDER BY r.drawn_at DESC",
    )?;
    let rows = stmt.query_map(params![user_id], |row| {
        Ok(ReadingRecord {
            id: row.get(0)?,
            user_name: row.get(1)?,
            drawn_at: row.get(2)?,
            tradition: row.get(3)?,
            spread_type: row.get(4)?,
            cards: row.get(5)?,
        })
    })?;
    rows.collect()
}

/// Retrieve all readings across all users, ordered newest-first.
pub fn get_all_readings(conn: &Connection) -> rusqlite::Result<Vec<ReadingRecord>> {
    let mut stmt = conn.prepare(
        "SELECT r.id, u.name, r.drawn_at, r.tradition, r.spread_type, r.cards
         FROM readings r JOIN users u ON r.user_id = u.id
         ORDER BY r.drawn_at DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(ReadingRecord {
            id: row.get(0)?,
            user_name: row.get(1)?,
            drawn_at: row.get(2)?,
            tradition: row.get(3)?,
            spread_type: row.get(4)?,
            cards: row.get(5)?,
        })
    })?;
    rows.collect()
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn in_memory_db() -> Connection {
        let conn = Connection::open_in_memory().expect("in-memory db");
        conn.execute_batch(SCHEMA).expect("schema");
        conn
    }

    #[test]
    fn creates_new_user_and_returns_is_new() {
        let conn = in_memory_db();
        let (user, is_new) = get_or_create_user(&conn, "Alice").unwrap();
        assert!(is_new);
        assert_eq!(user.name, "Alice");
    }

    #[test]
    fn returns_existing_user_case_insensitive() {
        let conn = in_memory_db();
        let (u1, _) = get_or_create_user(&conn, "Bob").unwrap();
        let (u2, is_new) = get_or_create_user(&conn, "bob").unwrap();
        assert!(!is_new);
        assert_eq!(u1.id, u2.id);
    }

    #[test]
    fn stats_empty_for_new_user() {
        let conn = in_memory_db();
        let (user, _) = get_or_create_user(&conn, "Carol").unwrap();
        let stats = get_stats(&conn, &user.id).unwrap();
        assert_eq!(stats.total_sessions, 0);
        assert!(stats.best_match_draw.is_none());
    }

    #[test]
    fn records_sessions_and_stats_are_accurate() {
        let conn = in_memory_db();
        let (user, _) = get_or_create_user(&conn, "Dave").unwrap();

        // Match on draw 4 in a 1-9 range — beats chance (mean = 9)
        record_session(&conn, &user.id, "2024-01-01", 1, 9, 3.0, "match", 4, true).unwrap();
        // Stopped after 12 draws — does not beat chance
        record_session(
            &conn,
            &user.id,
            "2024-01-02",
            1,
            9,
            3.0,
            "stopped",
            12,
            false,
        )
        .unwrap();

        let stats = get_stats(&conn, &user.id).unwrap();
        assert_eq!(stats.total_sessions, 2);
        assert_eq!(stats.beat_chance_count, 1);
        assert_eq!(stats.best_match_draw, Some(4));
        // mean draws = (4 + 12) / 2 = 8.0
        assert!((stats.mean_draws - 8.0).abs() < 1e-6);
    }

    #[test]
    fn tendency_ratio_below_one_when_consistently_early() {
        let conn = in_memory_db();
        let (user, _) = get_or_create_user(&conn, "Eve").unwrap();

        // Three very early matches in a 1-100 range (mean = 100)
        for draw in [5u32, 8, 3] {
            record_session(
                &conn,
                &user.id,
                "2024-01-01",
                1,
                100,
                3.0,
                "match",
                draw,
                true,
            )
            .unwrap();
        }
        let stats = get_stats(&conn, &user.id).unwrap();
        assert!(
            stats.tendency_ratio < 0.95,
            "ratio was {}",
            stats.tendency_ratio
        );
    }
}
