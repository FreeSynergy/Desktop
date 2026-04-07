// fs-settings/src/desktop_settings.rs — Desktop configuration types and iced view.
//
// All data structures (DesktopConfig, WindowConfig, etc.) are preserved.
// The Dioxus component is replaced with view_desktop(&SettingsApp).

use std::path::PathBuf;

use fs_i18n;
use serde::{Deserialize, Serialize};

use crate::config_path;

// ── TaskbarPosition ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TaskbarPosition {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
}

impl TaskbarPosition {
    #[must_use]
    pub fn label(&self) -> &str {
        match self {
            Self::Bottom => "Bottom",
            Self::Top => "Top",
            Self::Left => "Left",
            Self::Right => "Right",
        }
    }
}

// ── DisplayMode ────────────────────────────────────────────────────────────────

/// Preferred rendering mode for the desktop.
/// Saved to `~/.config/fsn/desktop.toml` and applied on the next launch.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DisplayMode {
    /// Native OS window (iced engine via fs-render).
    #[default]
    Window,
    /// Browser / web server.
    Web,
    /// Terminal UI.
    Tui,
}

impl DisplayMode {
    #[must_use]
    pub fn label(&self) -> &str {
        match self {
            Self::Window => "Window",
            Self::Web => "Web",
            Self::Tui => "TUI",
        }
    }

    #[must_use]
    pub fn description(&self) -> String {
        match self {
            Self::Window => fs_i18n::t("settings-desktop-mode-window").into(),
            Self::Web => fs_i18n::t("settings-desktop-mode-web").into(),
            Self::Tui => fs_i18n::t("settings-desktop-mode-tui").into(),
        }
    }
}

// ── SidebarPosition ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SidebarPosition {
    #[default]
    Left,
    Right,
    Top,
    Bottom,
}

impl SidebarPosition {
    #[must_use]
    pub fn label(&self) -> &str {
        match self {
            Self::Left => "Left",
            Self::Right => "Right",
            Self::Top => "Top",
            Self::Bottom => "Bottom",
        }
    }
}

// ── SidebarConfig ──────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SidebarConfig {
    #[serde(default)]
    pub position: SidebarPosition,
    #[serde(default = "default_true")]
    pub collapsible: bool,
    #[serde(default)]
    pub default_collapsed: bool,
    #[serde(default = "default_sidebar_width")]
    pub width: u32,
}

impl Default for SidebarConfig {
    fn default() -> Self {
        Self {
            position: SidebarPosition::Left,
            collapsible: true,
            default_collapsed: false,
            width: 240,
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_sidebar_width() -> u32 {
    240
}

// ── TitleBarStyle ──────────────────────────────────────────────────────────────

/// Window title bar decoration style (KDE-inspired).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TitleBarStyle {
    /// Full title bar with icon, title text, and all window controls.
    #[default]
    Full,
    /// Compact: reduced height, minimal decoration.
    Compact,
    /// Minimal: window controls only, no title text.
    Minimal,
    /// Hidden: no title bar — window dragged via content area.
    Hidden,
}

impl TitleBarStyle {
    #[must_use]
    pub fn label(&self) -> &str {
        match self {
            Self::Full => "Full",
            Self::Compact => "Compact",
            Self::Minimal => "Minimal",
            Self::Hidden => "Hidden",
        }
    }
}

// ── ResizeEdgeSize ─────────────────────────────────────────────────────────────

/// Width of the invisible resize border around windows.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ResizeEdgeSize {
    Narrow,
    #[default]
    Normal,
    Wide,
}

impl ResizeEdgeSize {
    #[must_use]
    pub fn label(&self) -> &str {
        match self {
            Self::Narrow => "Narrow",
            Self::Normal => "Normal",
            Self::Wide => "Wide",
        }
    }

    /// Returns the pixel width for this edge size.
    #[must_use]
    pub fn pixels(&self) -> u32 {
        match self {
            Self::Narrow => 2,
            Self::Normal => 4,
            Self::Wide => 8,
        }
    }
}

// ── DoubleClickAction ──────────────────────────────────────────────────────────

/// Action performed when the title bar is double-clicked.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DoubleClickAction {
    #[default]
    Maximize,
    Minimize,
    Shade,
    Close,
}

impl DoubleClickAction {
    #[must_use]
    pub fn label(&self) -> &str {
        match self {
            Self::Maximize => "Maximize",
            Self::Minimize => "Minimize",
            Self::Shade => "Shade",
            Self::Close => "Close",
        }
    }
}

// ── FocusPolicy ───────────────────────────────────────────────────────────────

/// Window focus acquisition policy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FocusPolicy {
    /// Window receives focus only on click (default).
    #[default]
    Click,
    /// Window receives focus when the pointer enters its area.
    FocusFollowsMouse,
    /// Focus follows mouse and the window also raises on focus.
    StrictFollowsMouse,
}

impl FocusPolicy {
    #[must_use]
    pub fn label(&self) -> &str {
        match self {
            Self::Click => "Click",
            Self::FocusFollowsMouse => "Focus Follows Mouse",
            Self::StrictFollowsMouse => "Strict Follows Mouse",
        }
    }
}

// ── WindowConfig ──────────────────────────────────────────────────────────────

/// Window decoration and interaction behavior.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindowConfig {
    #[serde(default)]
    pub title_bar_style: TitleBarStyle,
    #[serde(default)]
    pub resize_edge_size: ResizeEdgeSize,
    #[serde(default)]
    pub double_click_action: DoubleClickAction,
    #[serde(default)]
    pub focus_policy: FocusPolicy,
    #[serde(default = "default_true")]
    pub snap_zones_enabled: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title_bar_style: TitleBarStyle::Full,
            resize_edge_size: ResizeEdgeSize::Normal,
            double_click_action: DoubleClickAction::Maximize,
            focus_policy: FocusPolicy::Click,
            snap_zones_enabled: true,
        }
    }
}

// ── ClickStyle ─────────────────────────────────────────────────────────────────

/// Whether icons and items are opened on single or double click.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ClickStyle {
    /// Single click activates items (KDE default).
    Single,
    /// Double click activates items (classic Windows / macOS style).
    #[default]
    Double,
}

impl ClickStyle {
    #[must_use]
    pub fn label(&self) -> &str {
        match self {
            Self::Single => "Single Click",
            Self::Double => "Double Click",
        }
    }
}

// ── ClickConfig ───────────────────────────────────────────────────────────────

/// Mouse click and drag behavior.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClickConfig {
    /// Whether icons are opened on single or double click.
    #[serde(default)]
    pub icon_click: ClickStyle,
    /// Pixels the pointer must move before a drag gesture is initiated.
    #[serde(default = "default_drag_threshold")]
    pub drag_threshold: u32,
}

impl Default for ClickConfig {
    fn default() -> Self {
        Self {
            icon_click: ClickStyle::Double,
            drag_threshold: 4,
        }
    }
}

fn default_drag_threshold() -> u32 {
    4
}

// ── AnimationConfig ───────────────────────────────────────────────────────────

/// Desktop animation settings — integrates with `fs-render` `AnimationSet`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnimationConfig {
    /// ID of the active `AnimationSet` (e.g. `"default"` or a Store-installed set).
    #[serde(default = "default_animation_set")]
    pub set_id: String,
    /// Global speed multiplier: 0.5 = half speed, 1.0 = normal, 2.0 = double.
    #[serde(default = "default_speed_factor")]
    pub speed_factor: f32,
    /// When `true`, all animations are disabled (respects `prefers-reduced-motion`).
    #[serde(default)]
    pub disabled: bool,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            set_id: "default".to_string(),
            speed_factor: 1.0,
            disabled: false,
        }
    }
}

fn default_animation_set() -> String {
    "default".to_string()
}

fn default_speed_factor() -> f32 {
    1.0
}

// ── IconConfig ────────────────────────────────────────────────────────────────

/// Icon and cursor set selection (→ `fs-icons`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IconConfig {
    /// ID of the active icon set (e.g. `"fs-default"` or a Store-installed set).
    #[serde(default = "default_icon_set")]
    pub icon_set_id: String,
    /// ID of the active cursor theme (e.g. `"fs-default"`).
    #[serde(default = "default_cursor_set")]
    pub cursor_set_id: String,
}

impl Default for IconConfig {
    fn default() -> Self {
        Self {
            icon_set_id: "fs-default".to_string(),
            cursor_set_id: "fs-default".to_string(),
        }
    }
}

fn default_icon_set() -> String {
    "fs-default".to_string()
}

fn default_cursor_set() -> String {
    "fs-default".to_string()
}

// ── PanelArrangement ──────────────────────────────────────────────────────────

/// Workspace panel layout density.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PanelArrangement {
    #[default]
    Default,
    Compact,
    Wide,
}

impl PanelArrangement {
    #[must_use]
    pub fn label(&self) -> &str {
        match self {
            Self::Default => "Default",
            Self::Compact => "Compact",
            Self::Wide => "Wide",
        }
    }
}

// ── WorkspaceConfig ───────────────────────────────────────────────────────────

/// Workspace layout configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    /// Number of columns in the workspace grid (1–6).
    #[serde(default = "default_columns")]
    pub columns: u32,
    #[serde(default)]
    pub panel_arrangement: PanelArrangement,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            columns: 3,
            panel_arrangement: PanelArrangement::Default,
        }
    }
}

fn default_columns() -> u32 {
    3
}

// ── DesktopConfig ──────────────────────────────────────────────────────────────

/// Complete desktop configuration — persisted to `~/.config/fsn/desktop.toml`.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DesktopConfig {
    #[serde(default)]
    pub taskbar_pos: TaskbarPosition,
    #[serde(default)]
    pub display_mode: DisplayMode,
    #[serde(default)]
    pub sidebar: SidebarConfig,
    #[serde(default)]
    pub window: WindowConfig,
    #[serde(default)]
    pub click: ClickConfig,
    #[serde(default)]
    pub animation: AnimationConfig,
    #[serde(default)]
    pub icons: IconConfig,
    #[serde(default)]
    pub workspace: WorkspaceConfig,
}

impl DesktopConfig {
    /// Loads config from `~/.config/fsn/desktop.toml`, falling back to defaults.
    #[must_use]
    pub fn load() -> Self {
        let path = desktop_toml_path();
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    }

    /// Saves config to `~/.config/fsn/desktop.toml`.
    pub fn save(&self) {
        let path = desktop_toml_path();
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Ok(text) = toml::to_string_pretty(self) {
            let _ = std::fs::write(&path, text);
        }
    }
}

fn desktop_toml_path() -> PathBuf {
    config_path("desktop.toml")
}

// ── DesktopTab ────────────────────────────────────────────────────────────────

/// Tabs inside the Desktop settings section.
#[derive(Clone, PartialEq, Debug, Default)]
pub enum DesktopTab {
    #[default]
    General,
    Window,
    Click,
    Animations,
    Icons,
    Workspace,
}

impl DesktopTab {
    #[must_use]
    pub fn all() -> &'static [Self] {
        &[
            Self::General,
            Self::Window,
            Self::Click,
            Self::Animations,
            Self::Icons,
            Self::Workspace,
        ]
    }

    #[must_use]
    pub fn label(&self) -> String {
        fs_i18n::t(match self {
            Self::General => "settings-desktop-tab-general",
            Self::Window => "settings-desktop-tab-window",
            Self::Click => "settings-desktop-tab-click",
            Self::Animations => "settings-desktop-tab-animations",
            Self::Icons => "settings-desktop-tab-icons",
            Self::Workspace => "settings-desktop-tab-workspace",
        })
        .into()
    }
}

// ── view_desktop ──────────────────────────────────────────────────────────────

use fs_gui_engine_iced::iced::{
    widget::{button, checkbox, column, row, text, text_input},
    Alignment, Element, Length,
};

use crate::app::{Message, SettingsApp};

/// Render the Desktop settings section.
#[must_use]
pub fn view_desktop(app: &SettingsApp) -> Element<'_, Message> {
    let cfg = &app.desktop.config;
    let active_tab = &app.desktop.active_tab;

    // Tab bar
    let tabs: Vec<Element<Message>> = DesktopTab::all()
        .iter()
        .map(|tab| {
            let is_active = tab == active_tab;
            button(text(tab.label()).size(12))
                .padding([6, 12])
                .style(if is_active {
                    fs_gui_engine_iced::iced::widget::button::primary
                } else {
                    fs_gui_engine_iced::iced::widget::button::secondary
                })
                .on_press(Message::DesktopTabSelected(tab.clone()))
                .into()
        })
        .collect();
    let tab_row = row(tabs).spacing(4);

    // Tab content
    let tab_content: Element<Message> = match active_tab {
        DesktopTab::General => view_general_tab(cfg),
        DesktopTab::Window => view_window_tab(cfg),
        DesktopTab::Click => view_click_tab(cfg),
        DesktopTab::Animations => view_animations_tab(cfg),
        DesktopTab::Icons => view_icons_tab(cfg),
        DesktopTab::Workspace => view_workspace_tab(cfg),
    };

    let save_btn = button(text(fs_i18n::t("actions.save").to_string()).size(13))
        .padding([8, 20])
        .on_press(Message::SaveDesktop);

    column![
        text(fs_i18n::t("settings-desktop-title").to_string()).size(16),
        tab_row,
        tab_content,
        save_btn,
    ]
    .spacing(16)
    .width(Length::Fill)
    .into()
}

fn view_general_tab(cfg: &DesktopConfig) -> Element<'_, Message> {
    let taskbar_options = [
        ("bottom", "Bottom"),
        ("top", "Top"),
        ("left", "Left"),
        ("right", "Right"),
    ];
    let current_taskbar = match cfg.taskbar_pos {
        TaskbarPosition::Bottom => "bottom",
        TaskbarPosition::Top => "top",
        TaskbarPosition::Left => "left",
        TaskbarPosition::Right => "right",
    };

    let taskbar_btns: Vec<Element<Message>> = taskbar_options
        .iter()
        .map(|(id, label)| {
            let is_active = current_taskbar == *id;
            button(text(*label).size(12))
                .padding([6, 10])
                .style(if is_active {
                    fs_gui_engine_iced::iced::widget::button::primary
                } else {
                    fs_gui_engine_iced::iced::widget::button::secondary
                })
                .on_press(Message::DesktopTaskbarPositionChanged((*id).to_string()))
                .into()
        })
        .collect();

    column![
        text(fs_i18n::t("settings-desktop-taskbar-position").to_string()).size(14),
        row(taskbar_btns).spacing(6),
    ]
    .spacing(12)
    .into()
}

fn view_window_tab(cfg: &DesktopConfig) -> Element<'_, Message> {
    let focus_options = [
        ("click", "Click"),
        ("focus_follows_mouse", "Focus Follows Mouse"),
        ("strict_follows_mouse", "Strict Follows Mouse"),
    ];
    let current_focus = match cfg.window.focus_policy {
        FocusPolicy::Click => "click",
        FocusPolicy::FocusFollowsMouse => "focus_follows_mouse",
        FocusPolicy::StrictFollowsMouse => "strict_follows_mouse",
    };

    let focus_btns: Vec<Element<Message>> = focus_options
        .iter()
        .map(|(id, label)| {
            let is_active = current_focus == *id;
            button(text(*label).size(12))
                .padding([6, 10])
                .style(if is_active {
                    fs_gui_engine_iced::iced::widget::button::primary
                } else {
                    fs_gui_engine_iced::iced::widget::button::secondary
                })
                .on_press(Message::DesktopFocusPolicyChanged((*id).to_string()))
                .into()
        })
        .collect();

    let titlebar_options = [
        ("full", "Full"),
        ("compact", "Compact"),
        ("minimal", "Minimal"),
        ("hidden", "Hidden"),
    ];
    let current_titlebar = match cfg.window.title_bar_style {
        TitleBarStyle::Full => "full",
        TitleBarStyle::Compact => "compact",
        TitleBarStyle::Minimal => "minimal",
        TitleBarStyle::Hidden => "hidden",
    };
    let titlebar_btns: Vec<Element<Message>> = titlebar_options
        .iter()
        .map(|(id, label)| {
            let is_active = current_titlebar == *id;
            button(text(*label).size(12))
                .padding([6, 10])
                .style(if is_active {
                    fs_gui_engine_iced::iced::widget::button::primary
                } else {
                    fs_gui_engine_iced::iced::widget::button::secondary
                })
                .on_press(Message::DesktopTitleBarStyleChanged((*id).to_string()))
                .into()
        })
        .collect();

    column![
        text(fs_i18n::t("settings-desktop-focus-policy").to_string()).size(14),
        row(focus_btns).spacing(6),
        text(fs_i18n::t("settings-desktop-titlebar-style").to_string()).size(14),
        row(titlebar_btns).spacing(6),
    ]
    .spacing(12)
    .into()
}

fn view_click_tab(cfg: &DesktopConfig) -> Element<'_, Message> {
    let single_active = cfg.click.icon_click == ClickStyle::Single;
    let double_active = cfg.click.icon_click == ClickStyle::Double;

    let single_btn = button(text("Single Click").size(12))
        .padding([6, 10])
        .style(if single_active {
            fs_gui_engine_iced::iced::widget::button::primary
        } else {
            fs_gui_engine_iced::iced::widget::button::secondary
        })
        .on_press(Message::DesktopClickStyleChanged("single".to_string()));

    let double_btn = button(text("Double Click").size(12))
        .padding([6, 10])
        .style(if double_active {
            fs_gui_engine_iced::iced::widget::button::primary
        } else {
            fs_gui_engine_iced::iced::widget::button::secondary
        })
        .on_press(Message::DesktopClickStyleChanged("double".to_string()));

    column![
        text(fs_i18n::t("settings-desktop-click-style").to_string()).size(14),
        row![single_btn, double_btn].spacing(6),
    ]
    .spacing(12)
    .into()
}

fn view_animations_tab(cfg: &DesktopConfig) -> Element<'_, Message> {
    let disabled_row = row![
        text(fs_i18n::t("settings-desktop-animations-disabled").to_string())
            .size(13)
            .width(Length::Fill),
        checkbox(cfg.animation.disabled).on_toggle(Message::DesktopAnimationsDisabledToggled),
    ]
    .align_y(Alignment::Center)
    .spacing(8);

    column![
        text(fs_i18n::t("settings-desktop-tab-animations").to_string()).size(14),
        disabled_row,
        text(format!("Speed: {:.1}x", cfg.animation.speed_factor)).size(12),
    ]
    .spacing(12)
    .into()
}

fn view_icons_tab(cfg: &DesktopConfig) -> Element<'_, Message> {
    column![
        text(fs_i18n::t("settings-desktop-tab-icons").to_string()).size(14),
        row![
            text("Icon set:").size(12).width(120),
            text(cfg.icons.icon_set_id.as_str()).size(12),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
        row![
            text("Cursor:").size(12).width(120),
            text(cfg.icons.cursor_set_id.as_str()).size(12),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    ]
    .spacing(12)
    .into()
}

fn view_workspace_tab(cfg: &DesktopConfig) -> Element<'_, Message> {
    let cols_input = text_input("3", &cfg.workspace.columns.to_string())
        .on_input(Message::DesktopWorkspaceColumnsChanged)
        .padding([6, 10])
        .width(80);

    column![
        text(fs_i18n::t("settings-desktop-tab-workspace").to_string()).size(14),
        row![
            text(fs_i18n::t("settings-desktop-workspace-columns").to_string())
                .size(13)
                .width(Length::Fill),
            cols_input,
        ]
        .align_y(Alignment::Center)
        .spacing(8),
    ]
    .spacing(12)
    .into()
}
