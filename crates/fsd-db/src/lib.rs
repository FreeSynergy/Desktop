//! FreeSynergy.Desktop — SQLite storage layer (SeaORM-based).
//!
//! Manages two databases:
//! - `fsn-desktop.db`: widget positions, active theme, shortcuts
//! - `fsn-shared.db`: cross-program settings, i18n selection, audit log
//!
//! Also holds SQL schema definitions for the other FSN program databases.
//!
//! # Usage
//! ```rust,ignore
//! let db = FsdDb::open().await?;
//! db.desktop().set_active_theme("midnight-blue").await?;
//! db.shared().set_setting("language", "de").await?;
//! ```

pub mod desktop;
pub mod entities;
pub mod migration;
pub mod package_registry;
pub mod schemas;
pub mod shared;

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

/// Returns `~/.local/share/fsn/<name>` as the path for an FSN database.
pub fn db_path(filename: &str) -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".local/share/fsn").join(filename)
}

// ── Error type ────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("SeaORM error: {0}")]
    SeaOrm(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
