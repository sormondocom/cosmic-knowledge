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
CREATE TABLE IF NOT EXISTS meta (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS verses (
    id      INTEGER PRIMARY KEY,
    book    TEXT    NOT NULL,
    chapter INTEGER NOT NULL,
    verse   INTEGER NOT NULL,
    text    TEXT    NOT NULL
);
CREATE VIRTUAL TABLE IF NOT EXISTS verses_fts USING fts5(
    text,
    content='verses',
    content_rowid='id'
);
CREATE TRIGGER IF NOT EXISTS verses_ai AFTER INSERT ON verses BEGIN
    INSERT INTO verses_fts(rowid, text) VALUES (new.id, new.text);
END;
CREATE TRIGGER IF NOT EXISTS verses_ad AFTER DELETE ON verses BEGIN
    INSERT INTO verses_fts(verses_fts, rowid, text) VALUES ('delete', old.id, old.text);
END;
CREATE TRIGGER IF NOT EXISTS verses_au AFTER UPDATE ON verses BEGIN
    INSERT INTO verses_fts(verses_fts, rowid, text) VALUES ('delete', old.id, old.text);
    INSERT INTO verses_fts(rowid, text) VALUES (new.id, new.text);
END;
CREATE TABLE IF NOT EXISTS quran_verses (
    id         INTEGER PRIMARY KEY,
    surah      INTEGER NOT NULL,
    ayah       INTEGER NOT NULL,
    surah_name TEXT    NOT NULL,
    text       TEXT    NOT NULL
);
CREATE VIRTUAL TABLE IF NOT EXISTS quran_fts USING fts5(
    text,
    content='quran_verses',
    content_rowid='id'
);
CREATE TRIGGER IF NOT EXISTS quran_ai AFTER INSERT ON quran_verses BEGIN
    INSERT INTO quran_fts(rowid, text) VALUES (new.id, new.text);
END;
CREATE TRIGGER IF NOT EXISTS quran_ad AFTER DELETE ON quran_verses BEGIN
    INSERT INTO quran_fts(quran_fts, rowid, text) VALUES ('delete', old.id, old.text);
END;
CREATE TRIGGER IF NOT EXISTS quran_au AFTER UPDATE ON quran_verses BEGIN
    INSERT INTO quran_fts(quran_fts, rowid, text) VALUES ('delete', old.id, old.text);
    INSERT INTO quran_fts(rowid, text) VALUES (new.id, new.text);
END;
CREATE TABLE IF NOT EXISTS apocrypha_verses (
    id      INTEGER PRIMARY KEY,
    book    TEXT    NOT NULL,
    chapter INTEGER NOT NULL,
    verse   INTEGER NOT NULL,
    text    TEXT    NOT NULL
);
CREATE INDEX IF NOT EXISTS apocrypha_book_chap ON apocrypha_verses(book, chapter, verse);
CREATE VIRTUAL TABLE IF NOT EXISTS apocrypha_fts USING fts5(
    book UNINDEXED,
    chapter UNINDEXED,
    verse UNINDEXED,
    text,
    content='apocrypha_verses',
    content_rowid='id'
);
CREATE TRIGGER IF NOT EXISTS apocrypha_ai AFTER INSERT ON apocrypha_verses BEGIN
    INSERT INTO apocrypha_fts(rowid, book, chapter, verse, text)
    VALUES (new.id, new.book, new.chapter, new.verse, new.text);
END;
CREATE TRIGGER IF NOT EXISTS apocrypha_ad AFTER DELETE ON apocrypha_verses BEGIN
    INSERT INTO apocrypha_fts(apocrypha_fts, rowid, book, chapter, verse, text)
    VALUES ('delete', old.id, old.book, old.chapter, old.verse, old.text);
END;
CREATE TRIGGER IF NOT EXISTS apocrypha_au AFTER UPDATE ON apocrypha_verses BEGIN
    INSERT INTO apocrypha_fts(apocrypha_fts, rowid, book, chapter, verse, text)
    VALUES ('delete', old.id, old.book, old.chapter, old.verse, old.text);
    INSERT INTO apocrypha_fts(rowid, book, chapter, verse, text)
    VALUES (new.id, new.book, new.chapter, new.verse, new.text);
END;
CREATE TABLE IF NOT EXISTS text_positions (
    module  TEXT PRIMARY KEY,
    book    TEXT NOT NULL,
    chapter INTEGER NOT NULL,
    verse   INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS trimorphic_verses (
    id      INTEGER PRIMARY KEY,
    book    TEXT    NOT NULL,
    chapter INTEGER NOT NULL,
    verse   INTEGER NOT NULL,
    text    TEXT    NOT NULL
);
CREATE INDEX IF NOT EXISTS trim_chap ON trimorphic_verses(book, chapter, verse);
CREATE VIRTUAL TABLE IF NOT EXISTS trimorphic_fts USING fts5(
    book UNINDEXED,
    chapter UNINDEXED,
    verse UNINDEXED,
    text,
    content='trimorphic_verses',
    content_rowid='id'
);
CREATE TRIGGER IF NOT EXISTS trim_ai AFTER INSERT ON trimorphic_verses BEGIN
    INSERT INTO trimorphic_fts(rowid, book, chapter, verse, text)
    VALUES (new.id, new.book, new.chapter, new.verse, new.text);
END;
CREATE TRIGGER IF NOT EXISTS trim_ad AFTER DELETE ON trimorphic_verses BEGIN
    INSERT INTO trimorphic_fts(trimorphic_fts, rowid, book, chapter, verse, text)
    VALUES ('delete', old.id, old.book, old.chapter, old.verse, old.text);
END;
CREATE TRIGGER IF NOT EXISTS trim_au AFTER UPDATE ON trimorphic_verses BEGIN
    INSERT INTO trimorphic_fts(trimorphic_fts, rowid, book, chapter, verse, text)
    VALUES ('delete', old.id, old.book, old.chapter, old.verse, old.text);
    INSERT INTO trimorphic_fts(rowid, book, chapter, verse, text)
    VALUES (new.id, new.book, new.chapter, new.verse, new.text);
END;
CREATE TABLE IF NOT EXISTS pistis_sophia_verses (
    id      INTEGER PRIMARY KEY,
    book    TEXT    NOT NULL,
    chapter INTEGER NOT NULL,
    verse   INTEGER NOT NULL,
    text    TEXT    NOT NULL
);
CREATE INDEX IF NOT EXISTS ps_book_chap ON pistis_sophia_verses(book, chapter, verse);
CREATE VIRTUAL TABLE IF NOT EXISTS pistis_sophia_fts USING fts5(
    book UNINDEXED,
    chapter UNINDEXED,
    verse UNINDEXED,
    text,
    content='pistis_sophia_verses',
    content_rowid='id'
);
CREATE TRIGGER IF NOT EXISTS ps_ai AFTER INSERT ON pistis_sophia_verses BEGIN
    INSERT INTO pistis_sophia_fts(rowid, book, chapter, verse, text)
    VALUES (new.id, new.book, new.chapter, new.verse, new.text);
END;
CREATE TRIGGER IF NOT EXISTS ps_ad AFTER DELETE ON pistis_sophia_verses BEGIN
    INSERT INTO pistis_sophia_fts(pistis_sophia_fts, rowid, book, chapter, verse, text)
    VALUES ('delete', old.id, old.book, old.chapter, old.verse, old.text);
END;
CREATE TRIGGER IF NOT EXISTS ps_au AFTER UPDATE ON pistis_sophia_verses BEGIN
    INSERT INTO pistis_sophia_fts(pistis_sophia_fts, rowid, book, chapter, verse, text)
    VALUES ('delete', old.id, old.book, old.chapter, old.verse, old.text);
    INSERT INTO pistis_sophia_fts(rowid, book, chapter, verse, text)
    VALUES (new.id, new.book, new.chapter, new.verse, new.text);
END;
CREATE TABLE IF NOT EXISTS zohar_verses (
    id      INTEGER PRIMARY KEY,
    book    TEXT    NOT NULL,
    chapter INTEGER NOT NULL,
    verse   INTEGER NOT NULL,
    text    TEXT    NOT NULL
);
CREATE INDEX IF NOT EXISTS zohar_book_chap ON zohar_verses(book, chapter, verse);
CREATE VIRTUAL TABLE IF NOT EXISTS zohar_fts USING fts5(
    book UNINDEXED,
    chapter UNINDEXED,
    verse UNINDEXED,
    text,
    content='zohar_verses',
    content_rowid='id'
);
CREATE TRIGGER IF NOT EXISTS zohar_ai AFTER INSERT ON zohar_verses BEGIN
    INSERT INTO zohar_fts(rowid, book, chapter, verse, text)
    VALUES (new.id, new.book, new.chapter, new.verse, new.text);
END;
CREATE TRIGGER IF NOT EXISTS zohar_ad AFTER DELETE ON zohar_verses BEGIN
    INSERT INTO zohar_fts(zohar_fts, rowid, book, chapter, verse, text)
    VALUES ('delete', old.id, old.book, old.chapter, old.verse, old.text);
END;
CREATE TRIGGER IF NOT EXISTS zohar_au AFTER UPDATE ON zohar_verses BEGIN
    INSERT INTO zohar_fts(zohar_fts, rowid, book, chapter, verse, text)
    VALUES ('delete', old.id, old.book, old.chapter, old.verse, old.text);
    INSERT INTO zohar_fts(rowid, book, chapter, verse, text)
    VALUES (new.id, new.book, new.chapter, new.verse, new.text);
END;
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

// ─── Bible / KJV queries ─────────────────────────────────────────────────────

/// One verse returned from the Bible database.
pub struct BibleVerse {
    pub id: i64,
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
    pub text: String,
}

/// True when the `verses` table has been populated by the import script.
pub fn bible_is_loaded(conn: &Connection) -> bool {
    conn.query_row("SELECT COUNT(*) FROM verses", [], |r| r.get::<_, i64>(0))
        .map(|n| n > 0)
        .unwrap_or(false)
}

/// Return (verse_count, book_count) from the `meta` table (or 0 if absent).
pub fn bible_stats(conn: &Connection) -> (u32, u32) {
    let vc: u32 = conn
        .query_row(
            "SELECT value FROM meta WHERE key='kjv_verse_count'",
            [],
            |r| r.get::<_, String>(0),
        )
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let bc: u32 = conn
        .query_row(
            "SELECT value FROM meta WHERE key='kjv_book_count'",
            [],
            |r| r.get::<_, String>(0),
        )
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    (vc, bc)
}

/// Full-text search across all verses using FTS5.
///
/// `query` follows SQLite FTS5 syntax:
///   - plain words: `love mercy`  — any verse containing both words
///   - phrase:      `"love mercy"` — exact phrase
///   - prefix:      `mercif*`      — prefix wildcard
///   - boolean:     `love AND NOT hate`
///
/// Results are returned in relevance order (BM25 rank).
pub fn search_verses(
    conn: &Connection,
    query: &str,
    limit: usize,
) -> rusqlite::Result<Vec<BibleVerse>> {
    let mut stmt = conn.prepare(
        "SELECT v.id, v.book, v.chapter, v.verse, v.text
         FROM verses_fts f
         JOIN verses v ON f.rowid = v.id
         WHERE verses_fts MATCH ?1
         ORDER BY rank
         LIMIT ?2",
    )?;
    let rows = stmt.query_map(params![query, limit as i64], |row| {
        Ok(BibleVerse {
            id: row.get(0)?,
            book: row.get(1)?,
            chapter: row.get(2)?,
            verse: row.get(3)?,
            text: row.get(4)?,
        })
    })?;
    rows.collect()
}

/// Look up a single verse by canonical book name, chapter, and verse number.
pub fn lookup_verse(
    conn: &Connection,
    book: &str,
    chapter: u32,
    verse: u32,
) -> rusqlite::Result<Option<BibleVerse>> {
    let result = conn.query_row(
        "SELECT id, book, chapter, verse, text FROM verses
         WHERE lower(book) = lower(?1) AND chapter = ?2 AND verse = ?3",
        params![book, chapter, verse],
        |row| {
            Ok(BibleVerse {
                id: row.get(0)?,
                book: row.get(1)?,
                chapter: row.get(2)?,
                verse: row.get(3)?,
                text: row.get(4)?,
            })
        },
    );
    match result {
        Ok(v) => Ok(Some(v)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Return all verses in a chapter, ordered by verse number.
pub fn get_chapter(
    conn: &Connection,
    book: &str,
    chapter: u32,
) -> rusqlite::Result<Vec<BibleVerse>> {
    let mut stmt = conn.prepare(
        "SELECT id, book, chapter, verse, text FROM verses
         WHERE lower(book) = lower(?1) AND chapter = ?2
         ORDER BY verse",
    )?;
    let rows = stmt.query_map(params![book, chapter], |row| {
        Ok(BibleVerse {
            id: row.get(0)?,
            book: row.get(1)?,
            chapter: row.get(2)?,
            verse: row.get(3)?,
            text: row.get(4)?,
        })
    })?;
    rows.collect()
}

/// Seed the `verses` table from the embedded static data in `bible::verses_data`.
///
/// Called automatically by [`run_bible_session`] when the table is empty.
/// Uses a single transaction for speed; rebuilds the FTS index afterwards.
pub fn seed_bible_from_static(conn: &Connection) -> rusqlite::Result<()> {
    use crate::bible::verses_data::KJV_VERSES;

    conn.execute_batch("BEGIN")?;
    {
        let mut stmt =
            conn.prepare("INSERT INTO verses (book, chapter, verse, text) VALUES (?1,?2,?3,?4)")?;
        for &(book, chapter, verse, text) in KJV_VERSES {
            stmt.execute(params![book, chapter, verse, text])?;
        }
    }
    conn.execute_batch("COMMIT")?;

    // Rebuild FTS index in one shot (much faster than per-row triggers)
    conn.execute_batch("INSERT INTO verses_fts(verses_fts) VALUES ('rebuild')")?;

    // Persist meta counts
    let book_count = KJV_VERSES
        .iter()
        .map(|&(b, _, _, _)| b)
        .collect::<std::collections::HashSet<_>>()
        .len();
    conn.execute(
        "INSERT OR REPLACE INTO meta VALUES ('kjv_source', ?1)",
        params!["King James Version — embedded static data (Project Gutenberg EBook #10)"],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO meta VALUES ('kjv_verse_count', ?1)",
        params![KJV_VERSES.len().to_string()],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO meta VALUES ('kjv_book_count', ?1)",
        params![book_count.to_string()],
    )?;

    Ok(())
}

/// Return the number of verses in a specific chapter (0 if not found).
pub fn verse_count(conn: &Connection, book: &str, chapter: u32) -> u32 {
    conn.query_row(
        "SELECT MAX(verse) FROM verses WHERE lower(book) = lower(?1) AND chapter = ?2",
        params![book, chapter],
        |r| r.get::<_, Option<u32>>(0),
    )
    .ok()
    .flatten()
    .unwrap_or(0)
}

/// Return the number of chapters in a book (0 if book not found).
pub fn chapter_count(conn: &Connection, book: &str) -> u32 {
    conn.query_row(
        "SELECT MAX(chapter) FROM verses WHERE lower(book) = lower(?1)",
        params![book],
        |r| r.get::<_, Option<u32>>(0),
    )
    .ok()
    .flatten()
    .unwrap_or(0)
}

// ─── Quran / Pickthall queries ────────────────────────────────────────────────

/// One ayah returned from the Quran table.
pub struct QuranVerse {
    pub surah:      u32,
    pub ayah:       u32,
    pub surah_name: String,
    pub text:       String,
}

/// True when the `quran_verses` table has been populated.
pub fn quran_is_loaded(conn: &Connection) -> bool {
    conn.query_row("SELECT COUNT(*) FROM quran_verses", [], |r| r.get::<_, i64>(0))
        .map(|n| n > 0)
        .unwrap_or(false)
}

/// Return (ayah_count, surah_count) from `meta` (or 0 if absent).
pub fn quran_stats(conn: &Connection) -> (u32, u32) {
    let ac: u32 = conn
        .query_row("SELECT value FROM meta WHERE key='quran_ayah_count'", [], |r| {
            r.get::<_, String>(0)
        })
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let sc: u32 = conn
        .query_row("SELECT value FROM meta WHERE key='quran_surah_count'", [], |r| {
            r.get::<_, String>(0)
        })
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    (ac, sc)
}

/// Full-text search across all ayahs using FTS5.
pub fn search_quran(
    conn: &Connection,
    query: &str,
    limit: usize,
) -> rusqlite::Result<Vec<QuranVerse>> {
    let mut stmt = conn.prepare(
        "SELECT q.surah, q.ayah, q.surah_name, q.text
         FROM quran_fts f
         JOIN quran_verses q ON f.rowid = q.id
         WHERE quran_fts MATCH ?1
         ORDER BY rank
         LIMIT ?2",
    )?;
    let rows = stmt.query_map(params![query, limit as i64], |row| {
        Ok(QuranVerse {
            surah:      row.get(0)?,
            ayah:       row.get(1)?,
            surah_name: row.get(2)?,
            text:       row.get(3)?,
        })
    })?;
    rows.collect()
}

/// Look up a single ayah by surah and ayah number.
pub fn lookup_ayah(
    conn: &Connection,
    surah: u32,
    ayah: u32,
) -> rusqlite::Result<Option<QuranVerse>> {
    let result = conn.query_row(
        "SELECT surah, ayah, surah_name, text FROM quran_verses
         WHERE surah = ?1 AND ayah = ?2",
        params![surah, ayah],
        |row| {
            Ok(QuranVerse {
                surah:      row.get(0)?,
                ayah:       row.get(1)?,
                surah_name: row.get(2)?,
                text:       row.get(3)?,
            })
        },
    );
    match result {
        Ok(v)                                       => Ok(Some(v)),
        Err(rusqlite::Error::QueryReturnedNoRows)   => Ok(None),
        Err(e)                                      => Err(e),
    }
}

/// Return all ayahs in a surah, ordered by ayah number.
pub fn get_surah(conn: &Connection, surah: u32) -> rusqlite::Result<Vec<QuranVerse>> {
    let mut stmt = conn.prepare(
        "SELECT surah, ayah, surah_name, text FROM quran_verses
         WHERE surah = ?1
         ORDER BY ayah",
    )?;
    let rows = stmt.query_map(params![surah], |row| {
        Ok(QuranVerse {
            surah:      row.get(0)?,
            ayah:       row.get(1)?,
            surah_name: row.get(2)?,
            text:       row.get(3)?,
        })
    })?;
    rows.collect()
}

/// Return the highest ayah number in a surah (0 if not found).
pub fn ayah_count(conn: &Connection, surah: u32) -> u32 {
    conn.query_row(
        "SELECT MAX(ayah) FROM quran_verses WHERE surah = ?1",
        params![surah],
        |r| r.get::<_, Option<u32>>(0),
    )
    .ok()
    .flatten()
    .unwrap_or(0)
}

/// Seed the `quran_verses` table from the embedded static data.
pub fn seed_quran_from_static(conn: &Connection) -> rusqlite::Result<()> {
    use crate::quran::verses_data::QURAN_VERSES;

    conn.execute_batch("BEGIN")?;
    {
        let mut stmt = conn.prepare(
            "INSERT INTO quran_verses (surah, ayah, surah_name, text) VALUES (?1,?2,?3,?4)",
        )?;
        for &(surah, ayah, surah_name, text) in QURAN_VERSES {
            stmt.execute(params![surah, ayah, surah_name, text])?;
        }
    }
    conn.execute_batch("COMMIT")?;

    conn.execute_batch("INSERT INTO quran_fts(quran_fts) VALUES ('rebuild')")?;

    conn.execute(
        "INSERT OR REPLACE INTO meta VALUES ('quran_source', ?1)",
        params!["Pickthall English translation — Project Gutenberg EBook #16955"],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO meta VALUES ('quran_ayah_count', ?1)",
        params![QURAN_VERSES.len().to_string()],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO meta VALUES ('quran_surah_count', '114')",
        [],
    )?;

    Ok(())
}

// ─── Apocrypha queries ────────────────────────────────────────────────────────

/// One verse from the apocrypha table.
pub struct ApocrVerse {
    pub id:      i64,
    pub book:    String,
    pub chapter: u32,
    pub verse:   u32,
    pub text:    String,
}

/// True when the `apocrypha_verses` table has been populated.
pub fn apocrypha_is_loaded(conn: &Connection) -> bool {
    conn.query_row("SELECT COUNT(*) FROM apocrypha_verses", [], |r| r.get::<_, i64>(0))
        .map(|n| n > 0)
        .unwrap_or(false)
}

/// Return (verse_count, book_count) for the apocrypha.
pub fn apocrypha_stats(conn: &Connection) -> (u32, u32) {
    let vc: u32 = conn
        .query_row("SELECT COUNT(*) FROM apocrypha_verses", [], |r| r.get::<_, u32>(0))
        .unwrap_or(0);
    let bc: u32 = conn
        .query_row("SELECT COUNT(DISTINCT book) FROM apocrypha_verses", [], |r| r.get::<_, u32>(0))
        .unwrap_or(0);
    (vc, bc)
}

/// Full-text search across all apocrypha verses using FTS5.
pub fn search_apocrypha(
    conn: &Connection,
    query: &str,
    limit: usize,
) -> rusqlite::Result<Vec<ApocrVerse>> {
    let mut stmt = conn.prepare(
        "SELECT a.id, a.book, a.chapter, a.verse, a.text
         FROM apocrypha_fts f
         JOIN apocrypha_verses a ON f.rowid = a.id
         WHERE apocrypha_fts MATCH ?1
         ORDER BY rank
         LIMIT ?2",
    )?;
    let rows = stmt.query_map(params![query, limit as i64], |row| {
        Ok(ApocrVerse {
            id:      row.get(0)?,
            book:    row.get(1)?,
            chapter: row.get(2)?,
            verse:   row.get(3)?,
            text:    row.get(4)?,
        })
    })?;
    rows.collect()
}

/// Look up a single apocrypha verse by book, chapter, and verse number.
pub fn lookup_apocr_verse(
    conn: &Connection,
    book: &str,
    chapter: u32,
    verse: u32,
) -> rusqlite::Result<Option<ApocrVerse>> {
    let result = conn.query_row(
        "SELECT id, book, chapter, verse, text FROM apocrypha_verses
         WHERE lower(book) = lower(?1) AND chapter = ?2 AND verse = ?3",
        params![book, chapter, verse],
        |row| {
            Ok(ApocrVerse {
                id:      row.get(0)?,
                book:    row.get(1)?,
                chapter: row.get(2)?,
                verse:   row.get(3)?,
                text:    row.get(4)?,
            })
        },
    );
    match result {
        Ok(v)                                     => Ok(Some(v)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e)                                    => Err(e),
    }
}

/// Return all verses in an apocrypha chapter, ordered by verse number.
pub fn get_apocr_chapter(
    conn: &Connection,
    book: &str,
    chapter: u32,
) -> rusqlite::Result<Vec<ApocrVerse>> {
    let mut stmt = conn.prepare(
        "SELECT id, book, chapter, verse, text FROM apocrypha_verses
         WHERE lower(book) = lower(?1) AND chapter = ?2
         ORDER BY verse",
    )?;
    let rows = stmt.query_map(params![book, chapter], |row| {
        Ok(ApocrVerse {
            id:      row.get(0)?,
            book:    row.get(1)?,
            chapter: row.get(2)?,
            verse:   row.get(3)?,
            text:    row.get(4)?,
        })
    })?;
    rows.collect()
}

/// Return the highest verse number in a chapter (0 if not found).
pub fn apocr_verse_count(conn: &Connection, book: &str, chapter: u32) -> u32 {
    conn.query_row(
        "SELECT MAX(verse) FROM apocrypha_verses WHERE lower(book) = lower(?1) AND chapter = ?2",
        params![book, chapter],
        |r| r.get::<_, Option<u32>>(0),
    )
    .ok()
    .flatten()
    .unwrap_or(0)
}

/// Return the highest chapter number for a book (0 if not found).
pub fn apocr_chapter_count(conn: &Connection, book: &str) -> u32 {
    conn.query_row(
        "SELECT MAX(chapter) FROM apocrypha_verses WHERE lower(book) = lower(?1)",
        params![book],
        |r| r.get::<_, Option<u32>>(0),
    )
    .ok()
    .flatten()
    .unwrap_or(0)
}

// ─── Text position (reading bookmark) ────────────────────────────────────────

/// Save or overwrite the reading bookmark for `module` ("kjv", "quran", "apocr").
pub fn save_text_position(
    conn: &Connection,
    module: &str,
    book: &str,
    chapter: u32,
    verse: u32,
) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO text_positions (module, book, chapter, verse) \
         VALUES (?1, ?2, ?3, ?4)",
        params![module, book, chapter, verse],
    )?;
    Ok(())
}

/// Load the saved reading bookmark for `module`.
/// Returns `Some((book, chapter, verse))` or `None` if not set.
pub fn load_text_position(
    conn: &Connection,
    module: &str,
) -> Option<(String, u32, u32)> {
    conn.query_row(
        "SELECT book, chapter, verse FROM text_positions WHERE module = ?1",
        params![module],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, u32>(1)?, row.get::<_, u32>(2)?)),
    )
    .ok()
}

// ─── Trimorphic Protennoia ────────────────────────────────────────────────────

/// A single Trimorphic Protennoia paragraph record.
pub struct TrimorphicVerse {
    pub id:      i64,
    pub book:    String,
    pub chapter: u32,   // 1=Thought, 2=Voice, 3=Word
    pub verse:   u32,
    pub text:    String,
}

pub fn trimorphic_is_loaded(conn: &Connection) -> bool {
    conn.query_row("SELECT COUNT(*) FROM trimorphic_verses", [], |r| r.get::<_, i64>(0))
        .map(|n| n > 0)
        .unwrap_or(false)
}

pub fn trimorphic_stats(conn: &Connection) -> (u32, u32) {
    let vc: u32 = conn
        .query_row("SELECT COUNT(*) FROM trimorphic_verses", [], |r| r.get::<_, u32>(0))
        .unwrap_or(0);
    let dc: u32 = conn
        .query_row("SELECT COUNT(DISTINCT chapter) FROM trimorphic_verses", [], |r| r.get::<_, u32>(0))
        .unwrap_or(0);
    (vc, dc)
}

pub fn search_trimorphic(
    conn: &Connection,
    query: &str,
    limit: usize,
) -> rusqlite::Result<Vec<TrimorphicVerse>> {
    let mut stmt = conn.prepare(
        "SELECT a.id, a.book, a.chapter, a.verse, a.text
         FROM trimorphic_fts f
         JOIN trimorphic_verses a ON f.rowid = a.id
         WHERE trimorphic_fts MATCH ?1
         ORDER BY rank
         LIMIT ?2",
    )?;
    let rows = stmt.query_map(params![query, limit as i64], |row| {
        Ok(TrimorphicVerse {
            id:      row.get(0)?,
            book:    row.get(1)?,
            chapter: row.get(2)?,
            verse:   row.get(3)?,
            text:    row.get(4)?,
        })
    })?;
    rows.collect()
}

pub fn lookup_trimorphic_verse(
    conn: &Connection,
    chapter: u32,
    verse: u32,
) -> rusqlite::Result<Option<TrimorphicVerse>> {
    let result = conn.query_row(
        "SELECT id, book, chapter, verse, text FROM trimorphic_verses
         WHERE chapter = ?1 AND verse = ?2",
        params![chapter, verse],
        |row| Ok(TrimorphicVerse {
            id:      row.get(0)?,
            book:    row.get(1)?,
            chapter: row.get(2)?,
            verse:   row.get(3)?,
            text:    row.get(4)?,
        }),
    );
    match result {
        Ok(v)                                     => Ok(Some(v)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e)                                    => Err(e),
    }
}

pub fn get_trimorphic_discourse(
    conn: &Connection,
    chapter: u32,
) -> rusqlite::Result<Vec<TrimorphicVerse>> {
    let mut stmt = conn.prepare(
        "SELECT id, book, chapter, verse, text FROM trimorphic_verses
         WHERE chapter = ?1 ORDER BY verse",
    )?;
    let rows = stmt.query_map(params![chapter], |row| {
        Ok(TrimorphicVerse {
            id:      row.get(0)?,
            book:    row.get(1)?,
            chapter: row.get(2)?,
            verse:   row.get(3)?,
            text:    row.get(4)?,
        })
    })?;
    rows.collect()
}

pub fn trimorphic_verse_count(conn: &Connection, chapter: u32) -> u32 {
    conn.query_row(
        "SELECT MAX(verse) FROM trimorphic_verses WHERE chapter = ?1",
        params![chapter],
        |r| r.get::<_, Option<u32>>(0),
    )
    .ok()
    .flatten()
    .unwrap_or(0)
}

pub fn seed_trimorphic_from_static(conn: &Connection) -> rusqlite::Result<()> {
    use crate::trimorphic::verses_data::TRIMORPHIC_VERSES;

    conn.execute_batch("BEGIN")?;
    {
        let mut stmt = conn.prepare(
            "INSERT INTO trimorphic_verses (book, chapter, verse, text) VALUES (?1,?2,?3,?4)",
        )?;
        for &(book, chapter, verse, text) in TRIMORPHIC_VERSES {
            stmt.execute(params![book, chapter, verse, text])?;
        }
    }
    conn.execute_batch("COMMIT")?;
    conn.execute_batch("INSERT INTO trimorphic_fts(trimorphic_fts) VALUES ('rebuild')")?;
    conn.execute(
        "INSERT OR REPLACE INTO meta VALUES ('trimorphic_source', ?1)",
        params!["Trimorphic Protennoia, tr. John D. Turner (NHC XIII,1)"],
    )?;
    Ok(())
}

// ─── Pistis Sophia ───────────────────────────────────────────────────────────

/// A single Pistis Sophia paragraph record.
pub struct PsVerse {
    pub id:      i64,
    pub book:    String,
    pub chapter: u32,
    pub verse:   u32,
    pub text:    String,
}

/// True when the `pistis_sophia_verses` table has been populated.
pub fn ps_is_loaded(conn: &Connection) -> bool {
    conn.query_row("SELECT COUNT(*) FROM pistis_sophia_verses", [], |r| r.get::<_, i64>(0))
        .map(|n| n > 0)
        .unwrap_or(false)
}

/// Return (paragraph_count, book_count) for the Pistis Sophia.
pub fn ps_stats(conn: &Connection) -> (u32, u32) {
    let vc: u32 = conn
        .query_row("SELECT COUNT(*) FROM pistis_sophia_verses", [], |r| r.get::<_, u32>(0))
        .unwrap_or(0);
    let bc: u32 = conn
        .query_row("SELECT COUNT(DISTINCT book) FROM pistis_sophia_verses", [], |r| r.get::<_, u32>(0))
        .unwrap_or(0);
    (vc, bc)
}

/// Full-text search across all Pistis Sophia paragraphs using FTS5.
pub fn search_ps(
    conn: &Connection,
    query: &str,
    limit: usize,
) -> rusqlite::Result<Vec<PsVerse>> {
    let mut stmt = conn.prepare(
        "SELECT a.id, a.book, a.chapter, a.verse, a.text
         FROM pistis_sophia_fts f
         JOIN pistis_sophia_verses a ON f.rowid = a.id
         WHERE pistis_sophia_fts MATCH ?1
         ORDER BY rank
         LIMIT ?2",
    )?;
    let rows = stmt.query_map(params![query, limit as i64], |row| {
        Ok(PsVerse {
            id:      row.get(0)?,
            book:    row.get(1)?,
            chapter: row.get(2)?,
            verse:   row.get(3)?,
            text:    row.get(4)?,
        })
    })?;
    rows.collect()
}

/// Look up a single Pistis Sophia paragraph by book, chapter, and paragraph number.
pub fn lookup_ps_verse(
    conn: &Connection,
    book: &str,
    chapter: u32,
    verse: u32,
) -> rusqlite::Result<Option<PsVerse>> {
    let result = conn.query_row(
        "SELECT id, book, chapter, verse, text FROM pistis_sophia_verses
         WHERE lower(book) = lower(?1) AND chapter = ?2 AND verse = ?3",
        params![book, chapter, verse],
        |row| {
            Ok(PsVerse {
                id:      row.get(0)?,
                book:    row.get(1)?,
                chapter: row.get(2)?,
                verse:   row.get(3)?,
                text:    row.get(4)?,
            })
        },
    );
    match result {
        Ok(v)                                     => Ok(Some(v)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e)                                    => Err(e),
    }
}

/// Return all paragraphs in a Pistis Sophia chapter.
pub fn get_ps_chapter(
    conn: &Connection,
    book: &str,
    chapter: u32,
) -> rusqlite::Result<Vec<PsVerse>> {
    let mut stmt = conn.prepare(
        "SELECT id, book, chapter, verse, text FROM pistis_sophia_verses
         WHERE lower(book) = lower(?1) AND chapter = ?2
         ORDER BY verse",
    )?;
    let rows = stmt.query_map(params![book, chapter], |row| {
        Ok(PsVerse {
            id:      row.get(0)?,
            book:    row.get(1)?,
            chapter: row.get(2)?,
            verse:   row.get(3)?,
            text:    row.get(4)?,
        })
    })?;
    rows.collect()
}

/// Return the highest paragraph number in a chapter (0 if not found).
pub fn ps_verse_count(conn: &Connection, book: &str, chapter: u32) -> u32 {
    conn.query_row(
        "SELECT MAX(verse) FROM pistis_sophia_verses WHERE lower(book) = lower(?1) AND chapter = ?2",
        params![book, chapter],
        |r| r.get::<_, Option<u32>>(0),
    )
    .ok()
    .flatten()
    .unwrap_or(0)
}

/// Return the highest chapter number for a book (0 if not found).
pub fn ps_chapter_count(conn: &Connection, book: &str) -> u32 {
    conn.query_row(
        "SELECT MAX(chapter) FROM pistis_sophia_verses WHERE lower(book) = lower(?1)",
        params![book],
        |r| r.get::<_, Option<u32>>(0),
    )
    .ok()
    .flatten()
    .unwrap_or(0)
}

/// Return the lowest chapter number for a book (0 if not found).
pub fn ps_chapter_min(conn: &Connection, book: &str) -> u32 {
    conn.query_row(
        "SELECT MIN(chapter) FROM pistis_sophia_verses WHERE lower(book) = lower(?1)",
        params![book],
        |r| r.get::<_, Option<u32>>(0),
    )
    .ok()
    .flatten()
    .unwrap_or(1)
}

/// Seed the `pistis_sophia_verses` table from the embedded static data.
pub fn seed_ps_from_static(conn: &Connection) -> rusqlite::Result<()> {
    use crate::pistis_sophia::verses_data::PISTIS_SOPHIA_VERSES;

    conn.execute_batch("BEGIN")?;
    {
        let mut stmt = conn.prepare(
            "INSERT INTO pistis_sophia_verses (book, chapter, verse, text) VALUES (?1,?2,?3,?4)",
        )?;
        for &(book, chapter, verse, text) in PISTIS_SOPHIA_VERSES {
            stmt.execute(params![book, chapter, verse, text])?;
        }
    }
    conn.execute_batch("COMMIT")?;

    conn.execute_batch("INSERT INTO pistis_sophia_fts(pistis_sophia_fts) VALUES ('rebuild')")?;

    conn.execute(
        "INSERT OR REPLACE INTO meta VALUES ('ps_source', ?1)",
        params!["Pistis Sophia, tr. G.R.S. Mead [1921], public domain"],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO meta VALUES ('ps_para_count', ?1)",
        params![PISTIS_SOPHIA_VERSES.len().to_string()],
    )?;

    Ok(())
}

// ─── Zohar ────────────────────────────────────────────────────────────────────

/// A single Zohar paragraph record.
pub struct ZoharVerse {
    pub id:      i64,
    pub book:    String,
    pub chapter: u32,
    pub verse:   u32,
    pub text:    String,
}

/// True when the `zohar_verses` table has been populated.
pub fn zohar_is_loaded(conn: &Connection) -> bool {
    conn.query_row("SELECT COUNT(*) FROM zohar_verses", [], |r| r.get::<_, i64>(0))
        .map(|n| n > 0)
        .unwrap_or(false)
}

/// Return (paragraph_count, section_count) for the Zohar.
pub fn zohar_stats(conn: &Connection) -> (u32, u32) {
    let vc: u32 = conn
        .query_row("SELECT COUNT(*) FROM zohar_verses", [], |r| r.get::<_, u32>(0))
        .unwrap_or(0);
    let sc: u32 = conn
        .query_row("SELECT COUNT(DISTINCT book) FROM zohar_verses", [], |r| r.get::<_, u32>(0))
        .unwrap_or(0);
    (vc, sc)
}

/// Full-text search across all Zohar paragraphs using FTS5.
pub fn search_zohar(
    conn: &Connection,
    query: &str,
    limit: usize,
) -> rusqlite::Result<Vec<ZoharVerse>> {
    let mut stmt = conn.prepare(
        "SELECT a.id, a.book, a.chapter, a.verse, a.text
         FROM zohar_fts f
         JOIN zohar_verses a ON f.rowid = a.id
         WHERE zohar_fts MATCH ?1
         ORDER BY rank
         LIMIT ?2",
    )?;
    let rows = stmt.query_map(params![query, limit as i64], |row| {
        Ok(ZoharVerse {
            id:      row.get(0)?,
            book:    row.get(1)?,
            chapter: row.get(2)?,
            verse:   row.get(3)?,
            text:    row.get(4)?,
        })
    })?;
    rows.collect()
}

/// Look up a single Zohar paragraph by section, chapter, and paragraph number.
pub fn lookup_zohar_verse(
    conn: &Connection,
    book: &str,
    chapter: u32,
    verse: u32,
) -> rusqlite::Result<Option<ZoharVerse>> {
    let result = conn.query_row(
        "SELECT id, book, chapter, verse, text FROM zohar_verses
         WHERE lower(book) = lower(?1) AND chapter = ?2 AND verse = ?3",
        params![book, chapter, verse],
        |row| {
            Ok(ZoharVerse {
                id:      row.get(0)?,
                book:    row.get(1)?,
                chapter: row.get(2)?,
                verse:   row.get(3)?,
                text:    row.get(4)?,
            })
        },
    );
    match result {
        Ok(v)                                     => Ok(Some(v)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e)                                    => Err(e),
    }
}

/// Return all paragraphs in a Zohar chapter, ordered by paragraph number.
pub fn get_zohar_chapter(
    conn: &Connection,
    book: &str,
    chapter: u32,
) -> rusqlite::Result<Vec<ZoharVerse>> {
    let mut stmt = conn.prepare(
        "SELECT id, book, chapter, verse, text FROM zohar_verses
         WHERE lower(book) = lower(?1) AND chapter = ?2
         ORDER BY verse",
    )?;
    let rows = stmt.query_map(params![book, chapter], |row| {
        Ok(ZoharVerse {
            id:      row.get(0)?,
            book:    row.get(1)?,
            chapter: row.get(2)?,
            verse:   row.get(3)?,
            text:    row.get(4)?,
        })
    })?;
    rows.collect()
}

/// Return the highest paragraph number in a chapter (0 if not found).
pub fn zohar_verse_count(conn: &Connection, book: &str, chapter: u32) -> u32 {
    conn.query_row(
        "SELECT MAX(verse) FROM zohar_verses WHERE lower(book) = lower(?1) AND chapter = ?2",
        params![book, chapter],
        |r| r.get::<_, Option<u32>>(0),
    )
    .ok()
    .flatten()
    .unwrap_or(0)
}

/// Return the highest chapter number for a section (0 if not found).
pub fn zohar_chapter_count(conn: &Connection, book: &str) -> u32 {
    conn.query_row(
        "SELECT MAX(chapter) FROM zohar_verses WHERE lower(book) = lower(?1)",
        params![book],
        |r| r.get::<_, Option<u32>>(0),
    )
    .ok()
    .flatten()
    .unwrap_or(0)
}

/// Seed the `zohar_verses` table from the embedded static data.
pub fn seed_zohar_from_static(conn: &Connection) -> rusqlite::Result<()> {
    use crate::zohar::verses_data::ZOHAR_VERSES;

    conn.execute_batch("BEGIN")?;
    {
        let mut stmt = conn.prepare(
            "INSERT INTO zohar_verses (book, chapter, verse, text) VALUES (?1,?2,?3,?4)",
        )?;
        for &(book, chapter, verse, text) in ZOHAR_VERSES {
            stmt.execute(params![book, chapter, verse, text])?;
        }
    }
    conn.execute_batch("COMMIT")?;

    conn.execute_batch("INSERT INTO zohar_fts(zohar_fts) VALUES ('rebuild')")?;

    conn.execute(
        "INSERT OR REPLACE INTO meta VALUES ('zohar_source', ?1)",
        params!["Zohar: Bereshith to Lekh Lekha, tr. Nurho de Manhar [1900-14]"],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO meta VALUES ('zohar_para_count', ?1)",
        params![ZOHAR_VERSES.len().to_string()],
    )?;

    Ok(())
}

/// Seed the `apocrypha_verses` table from the embedded static data.
pub fn seed_apocrypha_from_static(conn: &Connection) -> rusqlite::Result<()> {
    use crate::apocrypha::verses_data::APOCRYPHA_VERSES;

    conn.execute_batch("BEGIN")?;
    {
        let mut stmt = conn.prepare(
            "INSERT INTO apocrypha_verses (book, chapter, verse, text) VALUES (?1,?2,?3,?4)",
        )?;
        for &(book, chapter, verse, text) in APOCRYPHA_VERSES {
            stmt.execute(params![book, chapter, verse, text])?;
        }
    }
    conn.execute_batch("COMMIT")?;

    conn.execute_batch("INSERT INTO apocrypha_fts(apocrypha_fts) VALUES ('rebuild')")?;

    conn.execute(
        "INSERT OR REPLACE INTO meta VALUES ('apocr_source', ?1)",
        params!["1 Enoch (R.H. Charles 1917), 2 Enoch & Jubilees (sacred-texts.com)"],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO meta VALUES ('apocr_verse_count', ?1)",
        params![APOCRYPHA_VERSES.len().to_string()],
    )?;

    Ok(())
}

// ─── App settings (meta key-value store) ─────────────────────────────────────

/// Read one value from the `meta` key-value store.  Returns `None` if the key
/// has never been set.
pub fn get_setting(conn: &Connection, key: &str) -> Option<String> {
    conn.query_row(
        "SELECT value FROM meta WHERE key = ?1",
        params![key],
        |row| row.get(0),
    )
    .ok()
}

/// Write (or overwrite) one value in the `meta` key-value store.
pub fn set_setting(conn: &Connection, key: &str, value: &str) {
    conn.execute(
        "INSERT INTO meta(key, value) VALUES(?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )
    .ok();
}

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
