/// Desktop widgets — data types for standalone UI cards on the desktop.
///
/// Persistence: widget layout is stored in `fs-desktop.db` via `crate::db`.
/// Rendering is done in `shell.rs` via iced widgets.
use serde::{Deserialize, Serialize};

// ── WidgetKind ─────────────────────────────────────────────────────────────

/// All widget types that can appear on the home layer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WidgetKind {
    Clock,
    SystemInfo,
    Messages,
    MyTasks,
    QuickNotes,
    Weather,
    /// Status widget for one bot instance (id = bot name).
    BotStatus(String),
    /// A widget installed from the Store, identified by its package ID.
    Custom(String),
}

impl WidgetKind {
    /// Human-readable label shown in the widget picker.
    #[must_use]
    pub fn label(&self) -> String {
        match self {
            WidgetKind::Clock => "Clock".to_string(),
            WidgetKind::SystemInfo => "System Info".to_string(),
            WidgetKind::Messages => "Messages".to_string(),
            WidgetKind::MyTasks => "My Tasks".to_string(),
            WidgetKind::QuickNotes => "Quick Notes".to_string(),
            WidgetKind::Weather => "Weather".to_string(),
            WidgetKind::BotStatus(id) => format!("Bot: {id}"),
            WidgetKind::Custom(id) => id.clone(),
        }
    }

    /// Default (width, height) in pixels for a newly placed widget.
    #[must_use]
    pub fn default_size(&self) -> (f64, f64) {
        match self {
            WidgetKind::Clock => (220.0, 140.0),
            WidgetKind::SystemInfo => (280.0, 190.0),
            WidgetKind::QuickNotes => (300.0, 230.0),
            WidgetKind::Messages | WidgetKind::MyTasks => (320.0, 220.0),
            WidgetKind::Weather => (260.0, 160.0),
            WidgetKind::BotStatus(_) => (260.0, 140.0),
            WidgetKind::Custom(_) => (280.0, 180.0),
        }
    }

    /// Emoji icon used as fallback when no icon URL is available.
    #[must_use]
    pub fn icon(&self) -> &'static str {
        match self {
            WidgetKind::Clock => "🕐",
            WidgetKind::SystemInfo => "🖥",
            WidgetKind::Messages => "📬",
            WidgetKind::MyTasks => "✅",
            WidgetKind::QuickNotes => "📝",
            WidgetKind::Weather => "🌤",
            WidgetKind::BotStatus(_) => "🤖",
            WidgetKind::Custom(_) => "🧩",
        }
    }

    /// `We10X` SVG icon URL for the widget picker panel.
    #[must_use]
    pub fn icon_url(&self) -> &'static str {
        match self {
            WidgetKind::Clock      => "https://raw.githubusercontent.com/yeyushengfan258/We10X-icon-theme/master/src/apps/scalable/preferences-system-time.svg",
            WidgetKind::SystemInfo => "https://raw.githubusercontent.com/yeyushengfan258/We10X-icon-theme/master/src/apps/scalable/utilities-system-monitor.svg",
            WidgetKind::Messages   => "https://raw.githubusercontent.com/yeyushengfan258/We10X-icon-theme/master/src/apps/scalable/internet-mail.svg",
            WidgetKind::MyTasks    => "https://raw.githubusercontent.com/yeyushengfan258/We10X-icon-theme/master/src/apps/scalable/evolution-tasks.svg",
            WidgetKind::QuickNotes => "https://raw.githubusercontent.com/yeyushengfan258/We10X-icon-theme/master/src/apps/scalable/accessories-text-editor.svg",
            WidgetKind::Weather    => "https://raw.githubusercontent.com/yeyushengfan258/We10X-icon-theme/master/src/apps/scalable/indicator-weather.svg",
            WidgetKind::BotStatus(_) => "https://raw.githubusercontent.com/yeyushengfan258/We10X-icon-theme/master/src/apps/scalable/internet-chat.svg",
            WidgetKind::Custom(_)    => "https://raw.githubusercontent.com/yeyushengfan258/We10X-icon-theme/master/src/apps/scalable/preferences-plugin-script.svg",
        }
    }

    /// Built-in widget kinds, in picker order. Does not include Custom variants.
    #[must_use]
    pub fn all() -> Vec<WidgetKind> {
        vec![
            WidgetKind::Clock,
            WidgetKind::SystemInfo,
            WidgetKind::Messages,
            WidgetKind::MyTasks,
            WidgetKind::QuickNotes,
            WidgetKind::Weather,
        ]
    }

    /// All widget kinds including store-installed Custom widgets.
    #[must_use]
    pub fn all_with_custom() -> Vec<WidgetKind> {
        use fs_db_desktop::package_registry::{PackageKind, PackageRegistry};
        let mut kinds = Self::all();
        for pkg in PackageRegistry::by_kind(PackageKind::Widget) {
            kinds.push(WidgetKind::Custom(pkg.id));
        }
        kinds
    }

    /// Persistence key string.
    #[must_use]
    pub fn as_str(&self) -> String {
        match self {
            WidgetKind::Clock => "Clock".to_string(),
            WidgetKind::SystemInfo => "SystemInfo".to_string(),
            WidgetKind::Messages => "Messages".to_string(),
            WidgetKind::MyTasks => "MyTasks".to_string(),
            WidgetKind::QuickNotes => "QuickNotes".to_string(),
            WidgetKind::Weather => "Weather".to_string(),
            WidgetKind::BotStatus(id) => format!("bot:{id}"),
            WidgetKind::Custom(id) => format!("custom:{id}"),
        }
    }

    /// Parse from persistence key string.
    #[must_use]
    pub fn from_key(s: &str) -> Option<WidgetKind> {
        match s {
            "Clock" => Some(WidgetKind::Clock),
            "SystemInfo" => Some(WidgetKind::SystemInfo),
            "Messages" => Some(WidgetKind::Messages),
            "MyTasks" => Some(WidgetKind::MyTasks),
            "QuickNotes" => Some(WidgetKind::QuickNotes),
            "Weather" => Some(WidgetKind::Weather),
            s if s.starts_with("bot:") => {
                Some(WidgetKind::BotStatus(s["bot:".len()..].to_string()))
            }
            s if s.starts_with("custom:") => {
                Some(WidgetKind::Custom(s["custom:".len()..].to_string()))
            }
            _ => None,
        }
    }
}

// ── WidgetSlot ─────────────────────────────────────────────────────────────

/// A widget instance placed in the home layer layout.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetSlot {
    /// Unique ID within this layout session.
    pub id: u32,
    /// Which widget to render.
    pub kind: WidgetKind,
    /// X position on the desktop (pixels from left edge).
    #[serde(default)]
    pub x: f64,
    /// Y position on the desktop (pixels from top edge).
    #[serde(default)]
    pub y: f64,
    /// Width of the widget card in pixels.
    #[serde(default)]
    pub w: f64,
    /// Height of the widget card in pixels.
    #[serde(default)]
    pub h: f64,
}

// ── Layout persistence ─────────────────────────────────────────────────────

/// Default widget layout: Clock + `SystemInfo` side by side.
#[must_use]
pub fn default_widget_layout() -> Vec<WidgetSlot> {
    let kinds = [WidgetKind::Clock, WidgetKind::SystemInfo];
    kinds
        .iter()
        .enumerate()
        .map(|(i, kind)| {
            let (w, h) = kind.default_size();
            WidgetSlot {
                id: u32::try_from(i).unwrap_or(0),
                kind: kind.clone(),
                x: 24.0 + f64::from(u32::try_from(i).unwrap_or(0)) * 296.0,
                y: 24.0,
                w,
                h,
            }
        })
        .collect()
}

/// Loads widget layout from `fs-desktop.db`. Falls back to default if empty.
#[must_use]
pub fn load_widget_layout(db: &fs_db_desktop::FsdDb) -> Vec<WidgetSlot> {
    let slots = crate::db::load_widgets_from_db(db);
    if slots.is_empty() {
        default_widget_layout()
    } else {
        slots
    }
}

/// Persists the current widget layout to `fs-desktop.db` (async, fire-and-forget).
pub fn save_widget_layout(db: std::sync::Arc<fs_db_desktop::FsdDb>, slots: &[WidgetSlot]) {
    crate::db::save_widgets_to_db(db, slots.to_vec());
}

/// Returns all widgets for use in the picker (built-in + store-installed).
#[must_use]
pub fn all_picker_widgets() -> Vec<WidgetKind> {
    WidgetKind::all_with_custom()
}
