pub mod app;
pub mod language_panel;
pub mod theme_panel;
pub mod icons_panel;
pub mod container_app_panel;
pub mod bots_panel;

pub use app::ManagersApp;

/// Register app-specific i18n strings for fsd-managers (`managers.*` keys).
/// Called once at desktop startup before any component renders.
pub fn register_i18n() {
    const EN: &str = include_str!("../assets/i18n/en.toml");
    const DE: &str = include_str!("../assets/i18n/de.toml");
    let _ = fsn_i18n::add_toml_lang("en", EN);
    let _ = fsn_i18n::add_toml_lang("de", DE);
}
