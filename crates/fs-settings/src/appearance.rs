// fs-settings/src/appearance.rs — Appearance settings section (iced).
//
// State: AppearanceState
// View:  view_appearance(&SettingsApp) -> Element<Message>

use fs_gui_engine_iced::iced::{
    widget::{button, checkbox, column, container, row, scrollable, text},
    Alignment, Element, Length,
};
use fs_i18n;

use crate::app::{Message, SettingsApp};

// ── AppearanceState ───────────────────────────────────────────────────────────

/// State for the Appearance settings section.
#[derive(Debug, Clone)]
pub struct AppearanceState {
    pub selected_theme: String,
    pub animations_enabled: bool,
}

impl AppearanceState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            selected_theme: "midnight-blue".to_string(),
            animations_enabled: true,
        }
    }

    pub fn save(&self) {
        // Persist to ~/.config/fsn/appearance.toml (best-effort).
        let path = crate::config_path("appearance.toml");
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let content = format!(
            "theme = \"{}\"\nanimations_enabled = {}\n",
            self.selected_theme, self.animations_enabled
        );
        let _ = std::fs::write(&path, content);
    }
}

impl Default for AppearanceState {
    fn default() -> Self {
        Self::new()
    }
}

// ── Built-in themes ───────────────────────────────────────────────────────────

/// Built-in theme entries: (id, label).
const BUILTIN_THEMES: &[(&str, &str)] = &[
    ("midnight-blue", "Midnight Blue"),
    ("ocean", "Ocean"),
    ("forest", "Forest"),
    ("sunset", "Sunset"),
    ("light", "Light"),
];

// ── view_appearance ───────────────────────────────────────────────────────────

/// Render the Appearance settings section.
pub fn view_appearance(app: &SettingsApp) -> Element<'_, Message> {
    let state = &app.appearance;

    // Theme selection buttons
    let theme_buttons: Vec<Element<Message>> = BUILTIN_THEMES
        .iter()
        .map(|(id, label)| {
            let is_active = state.selected_theme == *id;
            button(text(*label).size(13))
                .width(Length::Fill)
                .padding([8, 12])
                .style(if is_active {
                    fs_gui_engine_iced::iced::widget::button::primary
                } else {
                    fs_gui_engine_iced::iced::widget::button::secondary
                })
                .on_press(Message::ThemeSelected((*id).to_string()))
                .into()
        })
        .collect();

    let theme_col = column(theme_buttons).spacing(6);

    let anim_row = row![
        text(fs_i18n::t("settings-appearance-animations").to_string())
            .size(13)
            .width(Length::Fill),
        checkbox("", state.animations_enabled).on_toggle(Message::AnimationsToggled),
    ]
    .align_y(Alignment::Center)
    .spacing(8);

    let save_btn = button(text(fs_i18n::t("actions.save").to_string()).size(13))
        .padding([8, 20])
        .on_press(Message::SaveAppearance);

    let content = column![
        text(fs_i18n::t("settings-appearance-color-theme").to_string()).size(16),
        theme_col,
        text(fs_i18n::t("settings-appearance-animations").to_string()).size(14),
        anim_row,
        save_btn,
    ]
    .spacing(16)
    .width(Length::Fill);

    scrollable(container(content).width(Length::Fill)).into()
}
