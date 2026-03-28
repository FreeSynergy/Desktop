// fs-settings/src/browser_settings.rs — Browser settings section (iced).
//
// State: BrowserSettingsState
// View:  view_browser(&SettingsApp) -> Element<Message>

use fs_browser::{BrowserConfig, SearchEngineRegistry};
use fs_gui_engine_iced::iced::{
    widget::{button, column, row, scrollable, text},
    Element, Length,
};
use fs_i18n;

use crate::app::{Message, SettingsApp};

// ── BrowserSettingsState ──────────────────────────────────────────────────────

/// State for the Browser settings section.
#[derive(Debug, Clone)]
pub struct BrowserSettings;

/// Runtime state for the browser settings section.
#[derive(Debug, Clone)]
pub struct BrowserSettingsState {
    pub config: BrowserConfig,
}

impl BrowserSettingsState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: BrowserConfig::load(),
        }
    }
}

impl Default for BrowserSettingsState {
    fn default() -> Self {
        Self::new()
    }
}

// ── view_browser ──────────────────────────────────────────────────────────────

/// Render the Browser settings section.
#[must_use]
pub fn view_browser(app: &SettingsApp) -> Element<'_, Message> {
    let state = &app.browser;

    let engine_buttons: Vec<Element<Message>> = SearchEngineRegistry::all()
        .iter()
        .map(|engine| {
            let is_active = state.config.search_engine == engine.id;
            let lbl = format!("  {}  ", engine.name);
            let id = engine.id.clone();
            button(row![text(lbl).size(13)].padding([6, 4]))
                .width(Length::Fill)
                .padding([8, 12])
                .style(if is_active {
                    fs_gui_engine_iced::iced::widget::button::primary
                } else {
                    fs_gui_engine_iced::iced::widget::button::secondary
                })
                .on_press(Message::BrowserSearchEngineSelected(id))
                .into()
        })
        .collect();

    let engine_col = column(engine_buttons).spacing(6);

    let save_btn = button(text(fs_i18n::t("actions.save").to_string()).size(13))
        .padding([8, 20])
        .on_press(Message::SaveBrowser);

    let content = column![
        text(fs_i18n::t("settings-browser-title").to_string()).size(16),
        text(fs_i18n::t("settings-browser-search-engine").to_string()).size(14),
        text(fs_i18n::t("settings-browser-search-engine-hint").to_string()).size(12),
        engine_col,
        save_btn,
    ]
    .spacing(12)
    .width(Length::Fill);

    scrollable(content).into()
}
