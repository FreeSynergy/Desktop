// fs-settings/src/shortcuts.rs — Keyboard shortcuts editor (iced).
//
// Action registry, persistence, and settings UI.

use std::collections::HashMap;

use fs_gui_engine_iced::iced::{
    widget::{button, column, row, scrollable, text, text_input},
    Alignment, Element, Length,
};
use fs_i18n;
use serde::{Deserialize, Serialize};

use crate::app::{Message, SettingsApp};
use crate::config_path;

// ── ActionDef ─────────────────────────────────────────────────────────────────

/// A registered desktop action with an optional keyboard shortcut.
#[derive(Clone, PartialEq, Debug)]
pub struct ActionDef {
    pub id: &'static str,
    pub label: String,
    pub category: &'static str,
    pub default_shortcut: Option<&'static str>,
}

/// All desktop actions. The source of truth for shortcut defaults.
#[must_use]
pub fn register_actions() -> Vec<ActionDef> {
    vec![
        ActionDef {
            id: "app.settings",
            label: fs_i18n::t("settings-shortcuts-action-open-settings").into(),
            category: "App",
            default_shortcut: Some("Ctrl+,"),
        },
        ActionDef {
            id: "app.launcher",
            label: fs_i18n::t("settings-shortcuts-action-launcher").into(),
            category: "App",
            default_shortcut: Some("Ctrl+Space"),
        },
        ActionDef {
            id: "app.quit",
            label: fs_i18n::t("settings-shortcuts-action-quit").into(),
            category: "App",
            default_shortcut: Some("Ctrl+Q"),
        },
        ActionDef {
            id: "view.fullscreen",
            label: fs_i18n::t("settings-shortcuts-action-fullscreen").into(),
            category: "View",
            default_shortcut: Some("F11"),
        },
        ActionDef {
            id: "view.sidebar.show",
            label: fs_i18n::t("settings-shortcuts-action-sidebar").into(),
            category: "View",
            default_shortcut: None,
        },
        ActionDef {
            id: "store.open",
            label: fs_i18n::t("settings-shortcuts-action-store").into(),
            category: "Tools",
            default_shortcut: Some("Ctrl+S"),
        },
        ActionDef {
            id: "store.install",
            label: fs_i18n::t("settings-shortcuts-action-install").into(),
            category: "Tools",
            default_shortcut: Some("Ctrl+I"),
        },
        ActionDef {
            id: "tasks.open",
            label: fs_i18n::t("settings-shortcuts-action-tasks").into(),
            category: "Tools",
            default_shortcut: Some("Ctrl+T"),
        },
        ActionDef {
            id: "container-app.open",
            label: fs_i18n::t("settings-shortcuts-action-container").into(),
            category: "Tools",
            default_shortcut: None,
        },
        ActionDef {
            id: "studio.open",
            label: fs_i18n::t("settings-shortcuts-action-studio").into(),
            category: "Tools",
            default_shortcut: None,
        },
        ActionDef {
            id: "bots.open",
            label: fs_i18n::t("settings-shortcuts-action-bots").into(),
            category: "Tools",
            default_shortcut: None,
        },
        ActionDef {
            id: "help.open",
            label: fs_i18n::t("settings-shortcuts-action-help").into(),
            category: "Help",
            default_shortcut: Some("F1"),
        },
        ActionDef {
            id: "help.shortcuts",
            label: fs_i18n::t("settings-shortcuts-action-shortcuts").into(),
            category: "Help",
            default_shortcut: None,
        },
        ActionDef {
            id: "window.close",
            label: fs_i18n::t("settings-shortcuts-action-close").into(),
            category: "Window",
            default_shortcut: Some("Escape"),
        },
    ]
}

/// Returns the current shortcut for an action (custom > default).
#[must_use]
pub fn resolve_shortcut<'a>(action: &'a ActionDef, config: &'a ShortcutsConfig) -> Option<&'a str> {
    config
        .custom
        .get(action.id)
        .map(std::string::String::as_str)
        .or(action.default_shortcut)
}

// ── ShortcutsConfig ───────────────────────────────────────────────────────────

/// Persisted custom shortcut overrides.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ShortcutsConfig {
    #[serde(default)]
    pub custom: HashMap<String, String>,
}

impl ShortcutsConfig {
    #[must_use]
    pub fn load() -> Self {
        let path = config_path("shortcuts.toml");
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    }

    pub fn save(&self) {
        if let Ok(content) = toml::to_string(self) {
            let path = config_path("shortcuts.toml");
            let _ = std::fs::write(path, content);
        }
    }
}

// ── ShortcutsState ────────────────────────────────────────────────────────────

/// Runtime state for the Shortcuts settings section.
#[derive(Debug, Clone)]
pub struct ShortcutsState {
    pub config: ShortcutsConfig,
    pub search: String,
    /// Action ID currently being recorded (waiting for key input).
    pub recording: Option<String>,
}

impl ShortcutsState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: ShortcutsConfig::load(),
            search: String::new(),
            recording: None,
        }
    }
}

impl Default for ShortcutsState {
    fn default() -> Self {
        Self::new()
    }
}

// ── view_shortcuts ────────────────────────────────────────────────────────────

/// Render the Shortcuts settings section.
pub fn view_shortcuts(app: &SettingsApp) -> Element<'_, Message> {
    let state = &app.shortcuts;
    let actions = register_actions();

    let placeholder = fs_i18n::t("settings-shortcuts-search-placeholder").to_string();
    let search_input = text_input(placeholder.as_str(), &state.search)
        .on_input(Message::ShortcutsSearchChanged)
        .padding([6, 10])
        .width(Length::Fill);

    let q = state.search.to_lowercase();
    let filtered: Vec<&ActionDef> = actions
        .iter()
        .filter(|a| {
            q.is_empty()
                || a.label.to_lowercase().contains(&q)
                || a.category.to_lowercase().contains(&q)
        })
        .collect();

    // Build sorted, deduplicated category list
    let mut categories: Vec<&str> = filtered.iter().map(|a| a.category).collect();
    categories.sort_unstable();
    categories.dedup();

    let mut rows: Vec<Element<Message>> = vec![
        text(fs_i18n::t("settings-shortcuts-title").to_string())
            .size(16)
            .into(),
        search_input.into(),
    ];

    for cat in categories {
        let cat_actions: Vec<&&ActionDef> = filtered.iter().filter(|a| a.category == cat).collect();
        if cat_actions.is_empty() {
            continue;
        }

        rows.push(text(cat).size(11).into());

        for action in cat_actions {
            let current = resolve_shortcut(action, &state.config)
                .map_or_else(|| "—".to_string(), ToString::to_string);
            let is_recording = state.recording.as_deref() == Some(action.id);
            let is_default = !state.config.custom.contains_key(action.id);
            let action_id = action.id.to_string();
            let action_id2 = action.id.to_string();

            let shortcut_label = if is_recording {
                fs_i18n::t("settings-shortcuts-press-keys").to_string()
            } else {
                current
            };

            let record_btn = button(text(shortcut_label).size(12))
                .padding([3, 10])
                .style(if is_recording {
                    fs_gui_engine_iced::iced::widget::button::primary
                } else {
                    fs_gui_engine_iced::iced::widget::button::secondary
                })
                .on_press(if is_recording {
                    Message::ShortcutsStopRecording
                } else {
                    Message::ShortcutsStartRecording(action_id)
                });

            let mut row_items: Vec<Element<Message>> = vec![
                text(action.label.clone())
                    .size(13)
                    .width(Length::Fill)
                    .into(),
                record_btn.into(),
            ];

            if !is_default {
                row_items.push(
                    button(text(fs_i18n::t("actions.reset").to_string()).size(11))
                        .padding([3, 8])
                        .on_press(Message::ShortcutsResetAction(action_id2))
                        .into(),
                );
            }

            rows.push(
                row(row_items)
                    .align_y(Alignment::Center)
                    .spacing(6)
                    .padding([4, 0])
                    .into(),
            );
        }
    }

    let recording_hint: Element<Message> = if state.recording.is_some() {
        text(fs_i18n::t("settings-shortcuts-recording-hint").to_string())
            .size(12)
            .into()
    } else {
        fs_gui_engine_iced::iced::widget::Space::with_height(0).into()
    };

    rows.push(recording_hint);

    scrollable(column(rows).spacing(6).width(Length::Fill)).into()
}
