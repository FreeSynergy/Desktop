/// Desktop settings — window behavior, click, animations, icons, workspace, display.
use std::path::PathBuf;

use dioxus::prelude::*;
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
            Self::Window => fs_i18n::t("settings.desktop.mode_window").into(),
            Self::Web => fs_i18n::t("settings.desktop.mode_web").into(),
            Self::Tui => fs_i18n::t("settings.desktop.mode_tui").into(),
        }
    }

    #[must_use]
    pub fn icon(&self) -> &str {
        match self {
            Self::Window => "🖥",
            Self::Web => "🌐",
            Self::Tui => "⬛",
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

#[derive(Clone, PartialEq, Debug)]
enum DesktopTab {
    General,
    Window,
    Click,
    Animations,
    Icons,
    Workspace,
}

impl DesktopTab {
    fn all() -> &'static [Self] {
        &[
            Self::General,
            Self::Window,
            Self::Click,
            Self::Animations,
            Self::Icons,
            Self::Workspace,
        ]
    }

    fn label(&self) -> String {
        fs_i18n::t(match self {
            Self::General => "settings.desktop.tab_general",
            Self::Window => "settings.desktop.tab_window",
            Self::Click => "settings.desktop.tab_click",
            Self::Animations => "settings.desktop.tab_animations",
            Self::Icons => "settings.desktop.tab_icons",
            Self::Workspace => "settings.desktop.tab_workspace",
        })
        .into()
    }
}

// ── DesktopSettings component ─────────────────────────────────────────────────

/// Desktop behavior settings component.
#[component]
pub fn DesktopSettings() -> Element {
    let config = use_signal(DesktopConfig::load);
    let mut active_tab = use_signal(|| DesktopTab::General);

    rsx! {
        div {
            class: "fs-desktop-settings fs-scrollable",
            style: "padding: 24px; max-width: 560px; height: 100%;",

            h3 { style: "margin-top: 0; margin-bottom: 16px;",
                {fs_i18n::t("settings.desktop.title")}
            }

            // Tab bar
            div {
                style: "display: flex; gap: 4px; margin-bottom: 24px; flex-wrap: wrap; \
                        border-bottom: 1px solid var(--fs-color-border-default); padding-bottom: 8px;",
                for tab in DesktopTab::all() {
                    {
                        let is_active = *active_tab.read() == *tab;
                        let tab_clone = tab.clone();
                        let label = tab.label();
                        let style = if is_active {
                            "padding: 6px 14px; font-size: 13px; font-weight: 600; cursor: pointer; \
                             background: var(--fs-color-primary); color: white; border: none; \
                             border-radius: var(--fs-radius-md);"
                        } else {
                            "padding: 6px 14px; font-size: 13px; cursor: pointer; \
                             background: transparent; color: var(--fs-text-primary); \
                             border: 1px solid var(--fs-color-border-default); \
                             border-radius: var(--fs-radius-md);"
                        };
                        rsx! {
                            button {
                                key: "{label}",
                                style: "{style}",
                                onclick: move |_| active_tab.set(tab_clone.clone()),
                                "{label}"
                            }
                        }
                    }
                }
            }

            // Tab content
            match active_tab.read().clone() {
                DesktopTab::General    => rsx! { GeneralTab    { config } },
                DesktopTab::Window     => rsx! { WindowTab     { config } },
                DesktopTab::Click      => rsx! { ClickTab      { config } },
                DesktopTab::Animations => rsx! { AnimationsTab { config } },
                DesktopTab::Icons      => rsx! { IconsTab      { config } },
                DesktopTab::Workspace  => rsx! { WorkspaceTab  { config } },
            }

            // Save
            div { style: "margin-top: 28px;",
                button {
                    style: "padding: 8px 24px; background: var(--fs-color-primary); color: white; \
                            border: none; border-radius: var(--fs-radius-md); cursor: pointer; font-weight: 600;",
                    onclick: move |_| config.read().save(),
                    {fs_i18n::t("actions.save")}
                }
            }
        }
    }
}

// ── GeneralTab ────────────────────────────────────────────────────────────────

#[component]
fn GeneralTab(config: Signal<DesktopConfig>) -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 24px;",

            // Display Mode
            div {
                SectionLabel { label_key: "settings.desktop.display_mode" }
                p { style: "font-size: 13px; color: var(--fs-color-text-muted); margin: 0 0 8px;",
                    {fs_i18n::t("settings.desktop.next_launch_hint")}
                }
                div { style: "display: flex; flex-direction: column; gap: 6px;",
                    for mode in [DisplayMode::Window, DisplayMode::Web, DisplayMode::Tui] {
                        DisplayModeBtn { mode: mode.clone(), config }
                    }
                }
            }

            // Taskbar Position
            div {
                SectionLabel { label_key: "settings.desktop.taskbar_position" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px;",
                    TaskbarPosBtn { pos: TaskbarPosition::Bottom, config }
                    TaskbarPosBtn { pos: TaskbarPosition::Top,    config }
                    TaskbarPosBtn { pos: TaskbarPosition::Left,   config }
                    TaskbarPosBtn { pos: TaskbarPosition::Right,  config }
                }
            }

            // Sidebar Position
            div {
                SectionLabel { label_key: "settings.desktop.sidebar_position" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px;",
                    SidebarPosBtn { pos: SidebarPosition::Left,   config }
                    SidebarPosBtn { pos: SidebarPosition::Right,  config }
                    SidebarPosBtn { pos: SidebarPosition::Top,    config }
                    SidebarPosBtn { pos: SidebarPosition::Bottom, config }
                }
                div { style: "margin-top: 10px; display: flex; align-items: center; gap: 10px;",
                    input {
                        r#type: "checkbox",
                        checked: config.read().sidebar.default_collapsed,
                        onchange: move |e| config.write().sidebar.default_collapsed = e.checked(),
                    }
                    label { style: "font-size: 14px;",
                        {fs_i18n::t("settings.desktop.sidebar_collapsed")}
                    }
                }
            }
        }
    }
}

// ── WindowTab ─────────────────────────────────────────────────────────────────

#[component]
fn WindowTab(config: Signal<DesktopConfig>) -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 24px;",

            // Title bar style
            div {
                SectionLabel { label_key: "settings.desktop.titlebar_style" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px;",
                    for style in [TitleBarStyle::Full, TitleBarStyle::Compact, TitleBarStyle::Minimal, TitleBarStyle::Hidden] {
                        {
                            let is_active = config.read().window.title_bar_style == style;
                            let label = style.label();
                            let s = style.clone();
                            rsx! {
                                button {
                                    key: "{label}",
                                    style: option_btn_style(is_active),
                                    onclick: move |_| config.write().window.title_bar_style = s.clone(),
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }

            // Resize edge size
            div {
                SectionLabel { label_key: "settings.desktop.resize_edge" }
                div { style: "display: flex; gap: 8px;",
                    for size in [ResizeEdgeSize::Narrow, ResizeEdgeSize::Normal, ResizeEdgeSize::Wide] {
                        {
                            let is_active = config.read().window.resize_edge_size == size;
                            let label = size.label();
                            let s = size.clone();
                            rsx! {
                                button {
                                    key: "{label}",
                                    style: option_btn_style(is_active),
                                    onclick: move |_| config.write().window.resize_edge_size = s.clone(),
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }

            // Double-click action
            div {
                SectionLabel { label_key: "settings.desktop.dblclick_action" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px;",
                    for action in [DoubleClickAction::Maximize, DoubleClickAction::Minimize, DoubleClickAction::Shade, DoubleClickAction::Close] {
                        {
                            let is_active = config.read().window.double_click_action == action;
                            let label = action.label();
                            let a = action.clone();
                            rsx! {
                                button {
                                    key: "{label}",
                                    style: option_btn_style(is_active),
                                    onclick: move |_| config.write().window.double_click_action = a.clone(),
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }

            // Focus policy
            div {
                SectionLabel { label_key: "settings.desktop.focus_policy" }
                div { style: "display: flex; flex-direction: column; gap: 6px;",
                    for policy in [FocusPolicy::Click, FocusPolicy::FocusFollowsMouse, FocusPolicy::StrictFollowsMouse] {
                        {
                            let is_active = config.read().window.focus_policy == policy;
                            let label = policy.label();
                            let p = policy.clone();
                            rsx! {
                                button {
                                    key: "{label}",
                                    style: option_btn_style(is_active),
                                    onclick: move |_| config.write().window.focus_policy = p.clone(),
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }

            // Snap zones
            div { style: "display: flex; align-items: center; gap: 10px;",
                input {
                    r#type: "checkbox",
                    checked: config.read().window.snap_zones_enabled,
                    onchange: move |e| config.write().window.snap_zones_enabled = e.checked(),
                }
                label { style: "font-size: 14px;",
                    {fs_i18n::t("settings.desktop.snap_zones")}
                }
            }
        }
    }
}

// ── ClickTab ──────────────────────────────────────────────────────────────────

#[component]
fn ClickTab(config: Signal<DesktopConfig>) -> Element {
    let threshold = config.read().click.drag_threshold;
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 24px;",

            // Click style
            div {
                SectionLabel { label_key: "settings.desktop.click_style" }
                p { style: "font-size: 13px; color: var(--fs-color-text-muted); margin: 0 0 8px;",
                    {fs_i18n::t("settings.desktop.click_style_hint")}
                }
                div { style: "display: flex; gap: 8px;",
                    for style in [ClickStyle::Single, ClickStyle::Double] {
                        {
                            let is_active = config.read().click.icon_click == style;
                            let label = style.label();
                            let s = style.clone();
                            rsx! {
                                button {
                                    key: "{label}",
                                    style: option_btn_style(is_active),
                                    onclick: move |_| config.write().click.icon_click = s.clone(),
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }

            // Drag threshold
            div {
                div { style: "display: flex; justify-content: space-between; margin-bottom: 6px;",
                    SectionLabel { label_key: "settings.desktop.drag_threshold" }
                    span { style: "font-size: 13px; color: var(--fs-color-text-muted);",
                        "{threshold}px"
                    }
                }
                input {
                    r#type: "range",
                    min: "1",
                    max: "16",
                    value: "{threshold}",
                    style: "width: 100%; accent-color: var(--fs-color-primary);",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<u32>() {
                            config.write().click.drag_threshold = v;
                        }
                    },
                }
                div { style: "display: flex; justify-content: space-between; font-size: 11px; color: var(--fs-color-text-muted); margin-top: 4px;",
                    span { "1px" }
                    span { "16px" }
                }
            }
        }
    }
}

// ── AnimationsTab ─────────────────────────────────────────────────────────────

#[component]
fn AnimationsTab(config: Signal<DesktopConfig>) -> Element {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let speed_pct = (config.read().animation.speed_factor * 100.0) as u32;
    let disabled = config.read().animation.disabled;
    let set_id = config.read().animation.set_id.clone();

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 24px;",

            // Enable / disable
            div { style: "display: flex; align-items: center; gap: 10px;",
                input {
                    r#type: "checkbox",
                    checked: !disabled,
                    onchange: move |e| config.write().animation.disabled = !e.checked(),
                }
                label { style: "font-size: 14px; font-weight: 500;",
                    {fs_i18n::t("settings.desktop.animations_enabled")}
                }
            }

            // Speed factor (only when enabled)
            if !disabled {
                div {
                    div { style: "display: flex; justify-content: space-between; margin-bottom: 6px;",
                        SectionLabel { label_key: "settings.desktop.animation_speed" }
                        span { style: "font-size: 13px; color: var(--fs-color-text-muted);",
                            "{speed_pct}%"
                        }
                    }
                    input {
                        r#type: "range",
                        min: "25",
                        max: "200",
                        value: "{speed_pct}",
                        style: "width: 100%; accent-color: var(--fs-color-primary);",
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<f32>() {
                                config.write().animation.speed_factor = v / 100.0;
                            }
                        },
                    }
                    div { style: "display: flex; justify-content: space-between; font-size: 11px; color: var(--fs-color-text-muted); margin-top: 4px;",
                        span { "0.25×" }
                        span { {fs_i18n::t("settings.desktop.animation_normal")} }
                        span { "2.0×" }
                    }
                }

                // Animation set (future: populated from Store)
                div {
                    SectionLabel { label_key: "settings.desktop.animation_set" }
                    p { style: "font-size: 12px; color: var(--fs-color-text-muted); margin: 0 0 8px;",
                        {fs_i18n::t("settings.desktop.animation_set_hint")}
                    }
                    div { style: "display: flex; gap: 8px;",
                        {
                            let is_active = set_id == "default";
                            rsx! {
                                button {
                                    style: option_btn_style(is_active),
                                    onclick: move |_| config.write().animation.set_id = "default".to_string(),
                                    {fs_i18n::t("settings.desktop.animation_set_default")}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── IconsTab ──────────────────────────────────────────────────────────────────

#[component]
fn IconsTab(config: Signal<DesktopConfig>) -> Element {
    let icon_set = config.read().icons.icon_set_id.clone();
    let cursor_set = config.read().icons.cursor_set_id.clone();

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 24px;",
            p { style: "font-size: 13px; color: var(--fs-color-text-muted); margin: 0;",
                {fs_i18n::t("settings.desktop.icons_store_hint")}
            }

            // Icon set
            div {
                SectionLabel { label_key: "settings.desktop.icon_set" }
                div { style: "display: flex; gap: 8px; align-items: center;",
                    {
                        let is_active = icon_set == "fs-default";
                        rsx! {
                            button {
                                style: option_btn_style(is_active),
                                onclick: move |_| config.write().icons.icon_set_id = "fs-default".to_string(),
                                "FS Default"
                            }
                        }
                    }
                }
            }

            // Cursor set
            div {
                SectionLabel { label_key: "settings.desktop.cursor_set" }
                div { style: "display: flex; gap: 8px; align-items: center;",
                    {
                        let is_active = cursor_set == "fs-default";
                        rsx! {
                            button {
                                style: option_btn_style(is_active),
                                onclick: move |_| config.write().icons.cursor_set_id = "fs-default".to_string(),
                                "FS Default"
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── WorkspaceTab ──────────────────────────────────────────────────────────────

#[component]
fn WorkspaceTab(config: Signal<DesktopConfig>) -> Element {
    let columns = config.read().workspace.columns;

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 24px;",

            // Panel arrangement
            div {
                SectionLabel { label_key: "settings.desktop.panel_arrangement" }
                div { style: "display: flex; gap: 8px;",
                    for arr in [PanelArrangement::Default, PanelArrangement::Compact, PanelArrangement::Wide] {
                        {
                            let is_active = config.read().workspace.panel_arrangement == arr;
                            let label = arr.label();
                            let a = arr.clone();
                            rsx! {
                                button {
                                    key: "{label}",
                                    style: option_btn_style(is_active),
                                    onclick: move |_| config.write().workspace.panel_arrangement = a.clone(),
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }

            // Columns
            div {
                div { style: "display: flex; justify-content: space-between; margin-bottom: 6px;",
                    SectionLabel { label_key: "settings.desktop.workspace_columns" }
                    span { style: "font-size: 13px; color: var(--fs-color-text-muted);",
                        "{columns}"
                    }
                }
                input {
                    r#type: "range",
                    min: "1",
                    max: "6",
                    value: "{columns}",
                    style: "width: 100%; accent-color: var(--fs-color-primary);",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<u32>() {
                            config.write().workspace.columns = v;
                        }
                    },
                }
                div { style: "display: flex; justify-content: space-between; font-size: 11px; color: var(--fs-color-text-muted); margin-top: 4px;",
                    span { "1" }
                    span { "6" }
                }
            }
        }
    }
}

// ── Shared helper components ──────────────────────────────────────────────────

#[component]
fn SectionLabel(label_key: &'static str) -> Element {
    rsx! {
        label { style: "display: block; font-weight: 500; font-size: 14px; margin-bottom: 8px;",
            {fs_i18n::t(label_key)}
        }
    }
}

fn option_btn_style(active: bool) -> &'static str {
    if active {
        "padding: 8px 16px; border-radius: var(--fs-radius-md); border: 2px solid var(--fs-color-primary); \
         cursor: pointer; background: var(--fs-color-bg-surface); color: var(--fs-text-primary); font-weight: 600;"
    } else {
        "padding: 8px 16px; border-radius: var(--fs-radius-md); border: 2px solid var(--fs-color-border-default); \
         cursor: pointer; background: var(--fs-color-bg-surface); color: var(--fs-text-primary);"
    }
}

// ── Re-used button sub-components ─────────────────────────────────────────────

#[component]
fn DisplayModeBtn(mode: DisplayMode, config: Signal<DesktopConfig>) -> Element {
    let is_active = config.read().display_mode == mode;
    let border = if is_active {
        "var(--fs-color-primary)"
    } else {
        "var(--fs-color-border-default)"
    };
    let weight = if is_active { "600" } else { "400" };
    rsx! {
        button {
            style: "display: flex; align-items: center; gap: 10px; padding: 10px 14px; \
                    border-radius: var(--fs-radius-md); border: 2px solid {border}; \
                    cursor: pointer; background: var(--fs-color-bg-surface); \
                    text-align: left; font-weight: {weight};",
            onclick: move |_| config.write().display_mode = mode.clone(),
            span { style: "font-size: 18px;", "{mode.icon()}" }
            div {
                span { style: "display: block; font-size: 14px;", "{mode.label()}" }
                span { style: "display: block; font-size: 12px; color: var(--fs-color-text-muted);",
                    "{mode.description()}"
                }
            }
        }
    }
}

#[component]
fn TaskbarPosBtn(pos: TaskbarPosition, config: Signal<DesktopConfig>) -> Element {
    let is_active = config.read().taskbar_pos == pos;
    let label = pos.label();
    rsx! {
        button {
            style: option_btn_style(is_active),
            onclick: move |_| config.write().taskbar_pos = pos.clone(),
            "{label}"
        }
    }
}

#[component]
fn SidebarPosBtn(pos: SidebarPosition, config: Signal<DesktopConfig>) -> Element {
    let is_active = config.read().sidebar.position == pos;
    let label = pos.label();
    rsx! {
        button {
            style: option_btn_style(is_active),
            onclick: move |_| config.write().sidebar.position = pos.clone(),
            "{label}"
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_config_default_roundtrip() {
        let cfg = DesktopConfig::default();
        let toml = toml::to_string_pretty(&cfg).expect("serialize");
        let back: DesktopConfig = toml::from_str(&toml).expect("deserialize");
        assert_eq!(back.taskbar_pos, cfg.taskbar_pos);
        assert_eq!(back.display_mode, cfg.display_mode);
        assert_eq!(back.window.title_bar_style, cfg.window.title_bar_style);
        assert_eq!(
            back.window.snap_zones_enabled,
            cfg.window.snap_zones_enabled
        );
        assert_eq!(back.click.drag_threshold, cfg.click.drag_threshold);
        assert!((back.animation.speed_factor - cfg.animation.speed_factor).abs() < f32::EPSILON);
        assert_eq!(back.icons.icon_set_id, cfg.icons.icon_set_id);
        assert_eq!(back.workspace.columns, cfg.workspace.columns);
    }

    #[test]
    fn window_config_defaults() {
        let w = WindowConfig::default();
        assert_eq!(w.title_bar_style, TitleBarStyle::Full);
        assert_eq!(w.resize_edge_size, ResizeEdgeSize::Normal);
        assert_eq!(w.double_click_action, DoubleClickAction::Maximize);
        assert_eq!(w.focus_policy, FocusPolicy::Click);
        assert!(w.snap_zones_enabled);
    }

    #[test]
    fn click_config_defaults() {
        let c = ClickConfig::default();
        assert_eq!(c.icon_click, ClickStyle::Double);
        assert_eq!(c.drag_threshold, 4);
    }

    #[test]
    fn animation_config_defaults() {
        let a = AnimationConfig::default();
        assert_eq!(a.set_id, "default");
        assert!((a.speed_factor - 1.0).abs() < f32::EPSILON);
        assert!(!a.disabled);
    }

    #[test]
    fn icon_config_defaults() {
        let i = IconConfig::default();
        assert_eq!(i.icon_set_id, "fs-default");
        assert_eq!(i.cursor_set_id, "fs-default");
    }

    #[test]
    fn workspace_config_defaults() {
        let w = WorkspaceConfig::default();
        assert_eq!(w.columns, 3);
        assert_eq!(w.panel_arrangement, PanelArrangement::Default);
    }

    #[test]
    fn resize_edge_pixels() {
        assert_eq!(ResizeEdgeSize::Narrow.pixels(), 2);
        assert_eq!(ResizeEdgeSize::Normal.pixels(), 4);
        assert_eq!(ResizeEdgeSize::Wide.pixels(), 8);
    }

    #[test]
    fn partial_toml_deserializes_with_defaults() {
        let partial = r"
[window]
snap_zones_enabled = false

[workspace]
columns = 5
";
        let cfg: DesktopConfig = toml::from_str(partial).expect("deserialize partial");
        assert!(!cfg.window.snap_zones_enabled);
        assert_eq!(cfg.workspace.columns, 5);
        // Fields not in TOML should use defaults
        assert_eq!(cfg.window.title_bar_style, TitleBarStyle::Full);
        assert_eq!(cfg.click.drag_threshold, 4);
    }
}
