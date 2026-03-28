#![deny(clippy::all, clippy::pedantic, warnings)]
pub mod ai_view;
pub mod app_shell;
pub mod builtin_apps;
pub mod context_menu;
pub mod db;
pub mod header;
pub mod help_view;
pub mod icons;
pub mod launcher;
pub mod multiwindow;
pub mod notification;
pub mod shell;
pub mod sidebar;
pub mod spinner;
pub mod split_view;
pub mod system_info;
pub mod taskbar;
pub mod theme_loader;
pub mod wallpaper;
pub mod web_desktop;
pub mod widgets;
pub mod window;
pub mod window_frame;

// ── i18n plugin for shell strings (shell.*, profile.*) ───────────────────────

const I18N_SNIPPETS: &[(&str, &str)] = &[
    ("en", include_str!("../assets/i18n/en.toml")),
    ("de", include_str!("../assets/i18n/de.toml")),
];

struct ShellI18nPlugin;

impl fs_i18n::SnippetPlugin for ShellI18nPlugin {
    fn name(&self) -> &'static str {
        "fs-gui-workspace"
    }
    fn snippets(&self) -> &[(&str, &str)] {
        I18N_SNIPPETS
    }
}

// ── App-level i18n init ───────────────────────────────────────────────────────

/// Initialize global i18n at app startup.
///
/// Call **once from `main()`** — before any iced rendering — so that all
/// translation keys are resolved before the first frame is painted.
pub fn init_i18n() {
    let lang = fs_settings::load_active_language();

    let plugins: &[&dyn fs_i18n::SnippetPlugin] = &[&ShellI18nPlugin];

    if let Err(e) = fs_i18n::init_with_plugins(&lang, plugins) {
        if !e.to_string().contains("already initialized") {
            tracing::error!("i18n init failed: {e}");
        }
    }

    // Overlay user-installed language pack from disk.
    if lang != "en" {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        let pack = std::path::PathBuf::from(home)
            .join(".local/share/fsn/i18n")
            .join(&lang)
            .join("ui.toml");
        if let Ok(content) = std::fs::read_to_string(&pack) {
            let _ = fs_i18n::add_toml_lang(&lang, &content);
        }
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

pub use app_shell::AppMode;
pub use context_menu::{ContextMenuItem, ContextMenuState};
pub use header::{Breadcrumb, HeaderState};
pub use help_view::{HelpApp, HelpSidebarPanel};
pub use launcher::LauncherState;
pub use multiwindow::{use_multiwindow, MultiwindowHandle};
pub use notification::{Notification, NotificationHistory, NotificationKind, NotificationManager};
pub use shell::{DesktopMessage, DesktopShell};
pub use sidebar::{
    default_pinned_items, default_sidebar_sections, ManagerBundle, SidebarEntry, SidebarItem,
    SidebarSection,
};
pub use spinner::{LoadingOverlay, LoadingSpinner, SpinnerSize};
pub use split_view::SplitState;
pub use system_info::{Architecture, Platform, RunMode, SystemInfo};
pub use taskbar::AppEntry;
pub use web_desktop::WebTaskbarState;
pub use widgets::{load_widget_layout, save_widget_layout, WidgetKind, WidgetSlot};
pub use window::{
    AppId, FsWindow, OpenWindow, Window, WindowButton, WindowContent, WindowHost, WindowId,
    WindowManager, WindowRenderFn, WindowSidebarItem, WindowSize,
};
pub use window_frame::{MinimizedWindowIcon, WindowFrame};
