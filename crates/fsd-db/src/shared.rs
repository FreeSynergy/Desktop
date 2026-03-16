//! `fsn-shared.db` — cross-program shared storage.
//!
//! Tables:
//! - `settings`   — generic key-value store (all program-wide settings)
//! - `audit_log`  — append-only event log

use sqlx::{SqlitePool, Row};

use crate::{DbError, fsn_data_dir};

const MIGRATIONS: &str = r#"
CREATE TABLE IF NOT EXISTS settings (
    key        TEXT PRIMARY KEY,
    value      TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS audit_log (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    actor      TEXT NOT NULL,
    action     TEXT NOT NULL,
    target     TEXT,
    outcome    TEXT NOT NULL DEFAULT 'ok',
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
"#;

/// Database handle for `fsn-shared.db`.
pub struct SharedDb {
    pool: SqlitePool,
}

impl SharedDb {
    /// Open (or create) `~/.local/share/fsn/fsn-shared.db`, running migrations.
    pub async fn open() -> Result<Self, DbError> {
        let dir = fsn_data_dir();
        std::fs::create_dir_all(&dir)?;
        let path = format!("sqlite://{}?mode=rwc", dir.join("fsn-shared.db").display());
        let pool = SqlitePool::connect(&path).await?;
        sqlx::query(MIGRATIONS).execute(&pool).await?;
        Ok(Self { pool })
    }

    // ── Settings ──────────────────────────────────────────────────────────────

    /// Gets a setting value by key. Returns `None` if not set.
    pub async fn get_setting(&self, key: &str) -> Result<Option<String>, DbError> {
        let row = sqlx::query("SELECT value FROM settings WHERE key = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| r.get::<String, _>("value")))
    }

    /// Gets a setting value, returning `default` if not set.
    pub async fn get_setting_or(&self, key: &str, default: &str) -> Result<String, DbError> {
        Ok(self.get_setting(key).await?.unwrap_or_else(|| default.to_string()))
    }

    /// Upserts a setting.
    pub async fn set_setting(&self, key: &str, value: &str) -> Result<(), DbError> {
        sqlx::query(
            "INSERT INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at"
        )
        .bind(key)
        .bind(value)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Deletes a setting (resets to default behavior).
    pub async fn delete_setting(&self, key: &str) -> Result<(), DbError> {
        sqlx::query("DELETE FROM settings WHERE key = ?")
            .bind(key)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // ── Audit log ─────────────────────────────────────────────────────────────

    /// Appends an audit log entry.
    pub async fn audit(&self, actor: &str, action: &str, target: Option<&str>, outcome: &str) -> Result<(), DbError> {
        sqlx::query(
            "INSERT INTO audit_log (actor, action, target, outcome) VALUES (?, ?, ?, ?)"
        )
        .bind(actor)
        .bind(action)
        .bind(target)
        .bind(outcome)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Returns the most recent N audit log entries.
    pub async fn recent_audit(&self, limit: u32) -> Result<Vec<AuditEntry>, DbError> {
        let rows = sqlx::query(
            "SELECT actor, action, target, outcome, created_at FROM audit_log
             ORDER BY id DESC LIMIT ?"
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| AuditEntry {
            actor:      r.get("actor"),
            action:     r.get("action"),
            target:     r.get("target"),
            outcome:    r.get("outcome"),
            created_at: r.get("created_at"),
        }).collect())
    }
}

// ── Data types ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub actor:      String,
    pub action:     String,
    pub target:     Option<String>,
    pub outcome:    String,
    pub created_at: String,
}
