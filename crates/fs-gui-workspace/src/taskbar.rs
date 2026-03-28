/// Taskbar — data types for the bottom panel.
///
/// All rendering is done in `shell.rs` via iced widgets.
use crate::icons::ICON_STORE;
use crate::window::WindowId;

/// Homarr Dashboard Icons CDN base URL.
pub const DASHBOARD_ICONS_BASE: &str =
    "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg";

/// `We10X` icon theme raw base URL (scalable SVGs from the upstream repo).
pub const WE10X_ICONS_BASE: &str =
    "https://raw.githubusercontent.com/yeyushengfan258/We10X-icon-theme/master/src";

/// Returns the CDN URL for a Homarr Dashboard Icon.
/// `icon_name` must be the slug as it appears in the dashboard-icons repo (e.g. "kanidm").
#[must_use]
pub fn homarr_icon_url(icon_name: &str) -> String {
    format!("{DASHBOARD_ICONS_BASE}/{icon_name}.svg")
}

/// Returns the raw GitHub URL for a `We10X` icon.
/// `subdir` is the category (e.g. "apps/scalable", "places/scalable").
/// `icon_name` is the file stem without extension (e.g. "preferences-system").
#[must_use]
pub fn we10x_icon_url(subdir: &str, icon_name: &str) -> String {
    format!("{WE10X_ICONS_BASE}/{subdir}/{icon_name}.svg")
}

/// A registered application that can appear in the taskbar.
#[derive(Clone, Debug, PartialEq)]
pub struct AppEntry {
    /// Unique identifier (e.g. "container-app", "store").
    pub id: String,
    /// i18n key for the display name.
    pub label_key: String,
    /// Fallback emoji/text icon shown when no `icon_url` is available.
    pub icon: String,
    /// Optional icon URL (e.g. Homarr CDN SVG or local path).
    /// When set, `icon` is used as the `alt` text.
    pub icon_url: Option<String>,
    /// Optional group key for app launcher organisation.
    pub group: Option<String>,
    /// Whether this app is pinned to the taskbar permanently.
    pub pinned: bool,
    /// Open window IDs belonging to this app (empty = not running).
    pub windows: Vec<WindowId>,
}

impl AppEntry {
    #[must_use]
    pub fn is_running(&self) -> bool {
        !self.windows.is_empty()
    }

    /// Convenience constructor: no icon URL, no group.
    pub fn new(
        id: impl Into<String>,
        label_key: impl Into<String>,
        icon: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label_key: label_key.into(),
            icon: icon.into(),
            icon_url: None,
            group: None,
            pinned: false,
            windows: vec![],
        }
    }
}

/// Builds the default pinned apps list.
/// Only the Store is pinned by default; all other apps appear after installation.
#[must_use]
pub fn default_apps() -> Vec<AppEntry> {
    vec![AppEntry {
        id: "store".into(),
        label_key: "Store".into(),
        icon: ICON_STORE.into(),
        icon_url: Some(we10x_icon_url("apps/scalable", "system-software-install")),
        group: Some("System".into()),
        pinned: true,
        windows: vec![],
    }]
}
