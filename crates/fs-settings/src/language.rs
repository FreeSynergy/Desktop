// fs-settings/src/language.rs — Language settings section (iced).
//
// State: LanguageState
// View:  view_language(&SettingsApp) -> Element<Message>

use fs_db_desktop::package_registry::{PackageKind, PackageRegistry};
use fs_gui_engine_iced::iced::{
    widget::{button, column, row, scrollable, text},
    Element, Length,
};
use fs_i18n;
use fs_manager_language::LanguageManager;
use serde::Deserialize;

use crate::app::{Message, SettingsApp};

// ── Public types ─────────────────────────────────────────────────────────────

/// A single locale entry from the Store's locale catalog.
#[derive(Debug, Clone, Deserialize)]
pub struct LocaleEntry {
    pub code: String,
    pub name: String,
    pub version: String,
    pub completeness: u8,
    pub direction: String,
    pub path: Option<String>,
}

/// Built-in (always available, cannot be removed) languages.
pub const BUILTIN_LANGUAGES: &[(&str, &str)] = &[("en", "English")];

/// Returns the currently active language code, read from the user's locale inventory.
/// Falls back to "en" when no preference is saved.
#[must_use]
pub fn load_active_language() -> String {
    LanguageManager::new().effective_settings().language
}

// ── LangEntry ────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Debug)]
struct LangEntry {
    code: String,
    name: String,
    builtin: bool,
}

fn load_installed() -> Vec<LangEntry> {
    let mut entries: Vec<LangEntry> = BUILTIN_LANGUAGES
        .iter()
        .map(|(c, n)| LangEntry {
            code: c.to_string(),
            name: n.to_string(),
            builtin: true,
        })
        .collect();
    let builtin_codes: Vec<&str> = BUILTIN_LANGUAGES.iter().map(|(c, _)| *c).collect();
    for pkg in PackageRegistry::by_kind(PackageKind::Language) {
        if !builtin_codes.contains(&pkg.id.as_str()) {
            entries.push(LangEntry {
                code: pkg.id,
                name: pkg.name,
                builtin: false,
            });
        }
    }
    entries
}

// ── LanguageState ─────────────────────────────────────────────────────────────

/// State for the Language settings section.
#[derive(Debug, Clone)]
pub struct LanguageState {
    pub selected: String,
    installed: Vec<LangEntry>,
}

impl LanguageState {
    #[must_use]
    pub fn new() -> Self {
        let installed = load_installed();
        let selected = load_active_language();
        Self {
            selected,
            installed,
        }
    }

    pub fn save(&self) {
        let _ = fs_manager_language::LanguageManager::new().set_active(&self.selected);
    }
}

impl Default for LanguageState {
    fn default() -> Self {
        Self::new()
    }
}

// ── LanguageSettings (public re-export type) ──────────────────────────────────

/// Public type alias kept for backwards compatibility with `lib.rs` re-exports.
pub struct LanguageSettings;

// ── view_language ─────────────────────────────────────────────────────────────

/// Render the Language settings section.
#[must_use]
pub fn view_language(app: &SettingsApp) -> Element<'_, Message> {
    let state = &app.language;

    let lang_btns: Vec<Element<Message>> = state
        .installed
        .iter()
        .map(|entry| {
            let is_active = state.selected == entry.code;
            let label = if entry.builtin {
                format!("{} (built-in)", entry.name)
            } else {
                entry.name.clone()
            };
            let code = entry.code.clone();
            button(text(label).size(13))
                .width(Length::Fill)
                .padding([8, 12])
                .style(if is_active {
                    fs_gui_engine_iced::iced::widget::button::primary
                } else {
                    fs_gui_engine_iced::iced::widget::button::secondary
                })
                .on_press(Message::LanguageSelected(code))
                .into()
        })
        .collect();

    let save_btn = button(text(fs_i18n::t("actions.save").to_string()).size(13))
        .padding([8, 20])
        .on_press(Message::SaveLanguage);

    let content = column![
        text(fs_i18n::t("settings-language-default-label").to_string()).size(16),
        row![
            text(fs_i18n::t("settings-language-active").to_string())
                .size(13)
                .width(Length::Fill),
            text(state.selected.as_str()).size(13),
        ]
        .spacing(8),
        column(lang_btns).spacing(6),
        save_btn,
    ]
    .spacing(12)
    .width(Length::Fill);

    scrollable(content).into()
}
