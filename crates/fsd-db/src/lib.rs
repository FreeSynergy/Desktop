//! FreeSynergy.Desktop — SQLite storage layer.
//!
//! Manages two databases for the Desktop application:
//! - `fsn-desktop.db`: widget positions, active theme, shortcuts, profile, layout
//! - `fsn-shared.db`: cross-program settings, i18n selection, audit log
//!
//! Also provides schema definitions for the other program databases:
//! - `fsn-conductor.db`, `fsn-store.db`, `fsn-core.db`, `fsn-bus.db`
//!
//! # Usage
//! ```rust,ignore
//! let db = FsdDb::open().await?;
//! db.desktop().set_active_theme("midnight-blue").await?;
//! db.shared().set_setting("language", "de").await?;
//! ```

pub mod desktop;
pub mod shared;
pub mod schemas;

pub use desktop::DesktopDb;
pub use shared::SharedDb;

use std::path::PathBuf;

/// Combined handle for both Desktop databases.
pub struct FsdDb {
    desktop: DesktopDb,
    shared:  SharedDb,
}

impl FsdDb {
    /// Open (or create) both databases at their default paths.
    pub async fn open() -> Result<Self, DbError> {
        let desktop = DesktopDb::open().await?;
        let shared  = SharedDb::open().await?;
        Ok(Self { desktop, shared })
    }

    pub fn desktop(&self) -> &DesktopDb { &self.desktop }
    pub fn shared(&self)  -> &SharedDb  { &self.shared  }
}

/// Returns `~/.local/share/fsn/` as the base directory for all FSN databases.
pub fn fsn_data_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".local/share/fsn")
}

// ── Error type ────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("SQLite error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
