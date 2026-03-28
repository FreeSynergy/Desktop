//! Notification / Toast system — data types and manager.
//!
//! All rendering is done by `DesktopShell::view()` via iced widgets.
//! This module only owns the domain model.

// ── NotificationKind ──────────────────────────────────────────────────────────

/// Severity level of a notification.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum NotificationKind {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// Trait that gives a notification kind its visual identity.
pub trait NotificationStyle {
    fn accent_color(&self) -> &'static str;
}

impl NotificationStyle for NotificationKind {
    fn accent_color(&self) -> &'static str {
        match self {
            Self::Info => "#06b6d4",
            Self::Success => "#22c55e",
            Self::Warning => "#f59e0b",
            Self::Error => "#ef4444",
        }
    }
}

// ── Notification ──────────────────────────────────────────────────────────────

/// A single notification entry.
#[derive(Clone, Debug, PartialEq)]
pub struct Notification {
    pub id: u64,
    pub kind: NotificationKind,
    pub title: String,
    pub body: Option<String>,
}

// ── NotificationManager ───────────────────────────────────────────────────────

/// Manages the stack of active notifications.
#[derive(Clone, Default)]
pub struct NotificationManager {
    next_id: u64,
    items: Vec<Notification>,
}

impl NotificationManager {
    /// Push a new notification. Returns its ID.
    pub fn push(
        &mut self,
        kind: NotificationKind,
        title: impl Into<String>,
        body: Option<String>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.items.push(Notification {
            id,
            kind,
            title: title.into(),
            body,
        });
        // Keep at most 5 toasts visible
        if self.items.len() > 5 {
            self.items.remove(0);
        }
        id
    }

    pub fn dismiss(&mut self, id: u64) {
        self.items.retain(|n| n.id != id);
    }

    #[must_use]
    pub fn items(&self) -> &[Notification] {
        &self.items
    }
}

// ── NotificationHistory ───────────────────────────────────────────────────────

/// A persistent notification history entry.
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryEntry {
    pub id: u64,
    pub kind: NotificationKind,
    pub title: String,
    pub body: Option<String>,
    pub read: bool,
}

/// Manages the notification history (bell panel).
#[derive(Clone, Default, PartialEq)]
pub struct NotificationHistory {
    next_id: u64,
    entries: Vec<HistoryEntry>,
}

impl NotificationHistory {
    pub fn push(&mut self, kind: NotificationKind, title: impl Into<String>, body: Option<String>) {
        let id = self.next_id;
        self.next_id += 1;
        self.entries.insert(
            0,
            HistoryEntry {
                id,
                kind,
                title: title.into(),
                body,
                read: false,
            },
        );
        if self.entries.len() > 50 {
            self.entries.truncate(50);
        }
    }

    pub fn mark_all_read(&mut self) {
        for e in &mut self.entries {
            e.read = true;
        }
    }

    #[must_use]
    pub fn unread_count(&self) -> usize {
        self.entries.iter().filter(|e| !e.read).count()
    }

    #[must_use]
    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }
}
