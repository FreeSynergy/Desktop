//! Database integration for the Desktop shell.
//!
//! Opens `fsn-desktop.db` and `fsn-shared.db` at startup and exposes
//! helper functions for the Desktop component to read/write persistent state.
//!
//! All functions are synchronous wrappers that block on a background Tokio task,
//! since Dioxus component code cannot directly await.

use fsd_db::{DesktopDb, SharedDb};
use fsd_db::desktop::DbWidgetSlot;

use crate::widgets::{WidgetKind, WidgetSlot};

// ── Sync wrappers (called from Dioxus component init) ────────────────────────

/// Loads the active theme from `fsn-desktop.db`.
/// Falls back to `"midnight-blue"` on any error.
pub fn load_theme_from_db() -> String {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            match DesktopDb::open().await {
                Ok(db) => db.active_theme().await.unwrap_or_else(|_| "midnight-blue".to_string()),
                Err(_) => "midnight-blue".to_string(),
            }
        })
    })
}

/// Saves the active theme to `fsn-desktop.db` (fire-and-forget via spawn).
pub fn save_theme_to_db(name: String) {
    tokio::spawn(async move {
        if let Ok(db) = DesktopDb::open().await {
            let _ = db.set_active_theme(&name).await;
        }
    });
}

/// Loads widget slots from `fsn-desktop.db`.
/// Falls back to an empty vec on error (caller should use default layout then).
pub fn load_widgets_from_db() -> Vec<WidgetSlot> {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            match DesktopDb::open().await {
                Ok(db) => db.widget_slots().await
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(db_slot_to_widget)
                    .collect(),
                Err(_) => vec![],
            }
        })
    })
}

/// Saves widget slots to `fsn-desktop.db` (fire-and-forget via spawn).
pub fn save_widgets_to_db(slots: Vec<WidgetSlot>) {
    tokio::spawn(async move {
        if let Ok(db) = DesktopDb::open().await {
            let db_slots: Vec<DbWidgetSlot> = slots.iter().enumerate().map(|(i, s)| DbWidgetSlot {
                id:         s.id,
                kind:       s.kind.as_str().to_string(),
                x:          s.x,
                y:          s.y,
                w:          s.w,
                h:          s.h,
                sort_order: i as u32,
            }).collect();
            let _ = db.save_widget_slots(&db_slots).await;
        }
    });
}

/// Loads the i18n language selection from `fsn-shared.db`.
/// Falls back to `"de"` on error.
pub fn load_language_from_db() -> String {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            match SharedDb::open().await {
                Ok(db) => db.get_setting_or("language", "de").await.unwrap_or_else(|_| "de".to_string()),
                Err(_) => "de".to_string(),
            }
        })
    })
}

/// Saves the language selection to `fsn-shared.db` (fire-and-forget via spawn).
pub fn save_language_to_db(lang: String) {
    tokio::spawn(async move {
        if let Ok(db) = SharedDb::open().await {
            let _ = db.set_setting("language", &lang).await;
        }
    });
}

// ── Conversions ───────────────────────────────────────────────────────────────

fn db_slot_to_widget(db: DbWidgetSlot) -> Option<WidgetSlot> {
    let kind = WidgetKind::from_str(&db.kind)?;
    Some(WidgetSlot { id: db.id, kind, x: db.x, y: db.y, w: db.w, h: db.h })
}
