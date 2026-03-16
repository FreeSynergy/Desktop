//! `fsn-desktop.db` — Desktop-specific storage.
//!
//! Tables:
//! - `active_theme`   — current theme name (singleton)
//! - `widget_slots`   — widget positions + sizes on the home desktop
//! - `shortcuts`      — user-defined keyboard shortcut overrides
//! - `profile_data`   — user profile (name, avatar, links)

use sqlx::{SqlitePool, Row};

use crate::{DbError, fsn_data_dir};

/// Database handle for `fsn-desktop.db`.
pub struct DesktopDb {
    pool: SqlitePool,
}

// ── Migration SQL ─────────────────────────────────────────────────────────────

const MIGRATIONS: &str = r#"
CREATE TABLE IF NOT EXISTS active_theme (
    id   INTEGER PRIMARY KEY CHECK (id = 1),
    name TEXT    NOT NULL DEFAULT 'midnight-blue'
);
-- Ensure the singleton row always exists.
INSERT OR IGNORE INTO active_theme (id, name) VALUES (1, 'midnight-blue');

CREATE TABLE IF NOT EXISTS widget_slots (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    kind       TEXT    NOT NULL,
    pos_x      REAL    NOT NULL DEFAULT 0,
    pos_y      REAL    NOT NULL DEFAULT 0,
    width      REAL    NOT NULL DEFAULT 200,
    height     REAL    NOT NULL DEFAULT 150,
    sort_order INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS shortcuts (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    action_id TEXT    NOT NULL UNIQUE,
    key_combo TEXT    NOT NULL
);

CREATE TABLE IF NOT EXISTS profile_data (
    id         INTEGER PRIMARY KEY CHECK (id = 1),
    username   TEXT NOT NULL DEFAULT '',
    display_name TEXT NOT NULL DEFAULT '',
    avatar_url TEXT,
    bio        TEXT,
    links      TEXT  -- JSON array of {label, url}
);
INSERT OR IGNORE INTO profile_data (id) VALUES (1);
"#;

// ── DesktopDb implementation ──────────────────────────────────────────────────

impl DesktopDb {
    /// Open (or create) `~/.local/share/fsn/fsn-desktop.db`, running migrations.
    pub async fn open() -> Result<Self, DbError> {
        let dir = fsn_data_dir();
        std::fs::create_dir_all(&dir)?;
        let path = format!("sqlite://{}?mode=rwc", dir.join("fsn-desktop.db").display());
        let pool = SqlitePool::connect(&path).await?;
        // Run all migrations in one go — idempotent (CREATE TABLE IF NOT EXISTS).
        sqlx::query(MIGRATIONS).execute(&pool).await?;
        Ok(Self { pool })
    }

    // ── Theme ─────────────────────────────────────────────────────────────────

    /// Returns the active theme name (never empty — default is `midnight-blue`).
    pub async fn active_theme(&self) -> Result<String, DbError> {
        let row = sqlx::query("SELECT name FROM active_theme WHERE id = 1")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.get::<String, _>("name"))
    }

    /// Persists the active theme name.
    pub async fn set_active_theme(&self, name: &str) -> Result<(), DbError> {
        sqlx::query("UPDATE active_theme SET name = ? WHERE id = 1")
            .bind(name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // ── Widget slots ──────────────────────────────────────────────────────────

    /// Loads all widget slots ordered by `sort_order`.
    pub async fn widget_slots(&self) -> Result<Vec<DbWidgetSlot>, DbError> {
        let rows = sqlx::query(
            "SELECT id, kind, pos_x, pos_y, width, height, sort_order FROM widget_slots ORDER BY sort_order"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| DbWidgetSlot {
            id:         r.get::<i64, _>("id") as u32,
            kind:       r.get::<String, _>("kind"),
            x:          r.get::<f64, _>("pos_x"),
            y:          r.get::<f64, _>("pos_y"),
            w:          r.get::<f64, _>("width"),
            h:          r.get::<f64, _>("height"),
            sort_order: r.get::<i64, _>("sort_order") as u32,
        }).collect())
    }

    /// Replaces ALL widget slots with the given list (full replace on save).
    pub async fn save_widget_slots(&self, slots: &[DbWidgetSlot]) -> Result<(), DbError> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("DELETE FROM widget_slots").execute(&mut *tx).await?;
        for (i, s) in slots.iter().enumerate() {
            sqlx::query(
                "INSERT INTO widget_slots (kind, pos_x, pos_y, width, height, sort_order)
                 VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind(&s.kind)
            .bind(s.x)
            .bind(s.y)
            .bind(s.w)
            .bind(s.h)
            .bind(i as i64)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    // ── Shortcuts ─────────────────────────────────────────────────────────────

    /// Returns all custom shortcut overrides.
    pub async fn shortcuts(&self) -> Result<Vec<DbShortcut>, DbError> {
        let rows = sqlx::query("SELECT action_id, key_combo FROM shortcuts")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(|r| DbShortcut {
            action_id: r.get("action_id"),
            key_combo: r.get("key_combo"),
        }).collect())
    }

    /// Upserts a single shortcut override.
    pub async fn set_shortcut(&self, action_id: &str, key_combo: &str) -> Result<(), DbError> {
        sqlx::query(
            "INSERT INTO shortcuts (action_id, key_combo) VALUES (?, ?)
             ON CONFLICT(action_id) DO UPDATE SET key_combo = excluded.key_combo"
        )
        .bind(action_id)
        .bind(key_combo)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Removes a shortcut override (reverts to default).
    pub async fn delete_shortcut(&self, action_id: &str) -> Result<(), DbError> {
        sqlx::query("DELETE FROM shortcuts WHERE action_id = ?")
            .bind(action_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

// ── Data types ────────────────────────────────────────────────────────────────

/// A widget slot row as stored in `fsn-desktop.db`.
#[derive(Debug, Clone)]
pub struct DbWidgetSlot {
    pub id:         u32,
    pub kind:       String,
    pub x:          f64,
    pub y:          f64,
    pub w:          f64,
    pub h:          f64,
    pub sort_order: u32,
}

/// A keyboard shortcut override row.
#[derive(Debug, Clone)]
pub struct DbShortcut {
    pub action_id: String,
    pub key_combo: String,
}
