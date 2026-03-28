/// `AppLauncher` — domain types for the full-screen app launcher overlay.
///
/// All rendering is done in `shell.rs` via iced widgets.
/// This module only owns `LauncherState` and `AppGroup`.
use crate::taskbar::AppEntry;

/// How many app groups to show per page in the launcher.
pub const GROUPS_PER_PAGE: usize = 3;

/// State exposed to the desktop shell for open/close.
#[derive(Clone, Default, PartialEq)]
pub struct LauncherState {
    pub open: bool,
    pub query: String,
    pub page: usize,
}

impl LauncherState {
    pub fn toggle(&mut self) {
        self.open = !self.open;
        if !self.open {
            self.query.clear();
            self.page = 0;
        }
    }

    pub fn close(&mut self) {
        self.open = false;
        self.query.clear();
        self.page = 0;
    }

    pub fn set_query(&mut self, q: String) {
        self.query = q;
        self.page = 0;
    }

    pub fn prev_page(&mut self, total_pages: usize) {
        if self.page > 0 {
            self.page -= 1;
        }
        let _ = total_pages;
    }

    pub fn next_page(&mut self, total_pages: usize) {
        if self.page + 1 < total_pages {
            self.page += 1;
        }
    }

    pub fn goto_page(&mut self, idx: usize) {
        self.page = idx;
    }
}

/// A named group of apps displayed in the launcher.
#[derive(Clone, PartialEq, Debug)]
pub struct AppGroup {
    pub id: String,
    pub label: String,
    pub apps: Vec<AppEntry>,
}

impl AppGroup {
    /// Build groups from a flat app list using `AppEntry::group`.
    /// Apps without a group fall into "Other".
    /// Insertion order is preserved by tracking order in a separate Vec.
    #[must_use]
    pub fn from_entries(entries: &[AppEntry]) -> Vec<AppGroup> {
        let mut order: Vec<String> = Vec::new();
        let mut map: std::collections::HashMap<String, Vec<AppEntry>> =
            std::collections::HashMap::new();
        for app in entries {
            let key = app.group.clone().unwrap_or_else(|| "Other".into());
            if !map.contains_key(&key) {
                order.push(key.clone());
            }
            map.entry(key).or_default().push(app.clone());
        }
        order
            .into_iter()
            .map(|label| {
                let apps = map.remove(&label).unwrap_or_default();
                AppGroup {
                    id: label.to_lowercase().replace(' ', "-"),
                    label,
                    apps,
                }
            })
            .collect()
    }

    /// Filtered groups for the current query.
    #[must_use]
    pub fn filtered(entries: &[AppEntry], query: &str) -> Vec<AppGroup> {
        let q = query.to_lowercase();
        let filtered: Vec<AppEntry> = entries
            .iter()
            .filter(|a| {
                q.is_empty()
                    || a.id.to_lowercase().contains(&q)
                    || a.label_key.to_lowercase().contains(&q)
            })
            .cloned()
            .collect();
        Self::from_entries(&filtered)
    }

    /// Returns the page slice from all groups.
    #[must_use]
    pub fn page_slice(groups: &[AppGroup], page: usize) -> &[AppGroup] {
        let start = page * GROUPS_PER_PAGE;
        if start >= groups.len() {
            return &[];
        }
        let end = (start + GROUPS_PER_PAGE).min(groups.len());
        &groups[start..end]
    }

    /// Total number of pages.
    #[must_use]
    pub fn total_pages(groups: &[AppGroup]) -> usize {
        groups.len().div_ceil(GROUPS_PER_PAGE).max(1)
    }
}
