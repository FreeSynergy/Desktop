// fs-settings/src/accounts.rs — Account settings section (iced).
//
// Manages OIDC provider connections. Providers are stored in
// ~/.config/fsn/accounts.toml. No tokens are stored here.

use fs_gui_engine_iced::iced::{
    widget::{button, checkbox, column, row, scrollable, text, text_input},
    Alignment, Element, Length,
};
use fs_i18n;
use serde::{Deserialize, Serialize};

use crate::app::{Message, SettingsApp};

// ── Data model ────────────────────────────────────────────────────────────────

/// A configured OIDC provider entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OidcProvider {
    /// Short display name (e.g. "Kanidm", "Keycloak").
    pub name: String,
    /// OIDC discovery base URL (e.g. `https://auth.example.com`).
    pub discovery_url: String,
    /// `OAuth2` client ID registered with this provider.
    pub client_id: String,
    /// Scopes to request (space-separated, e.g. `"openid email profile"`).
    pub scopes: String,
    /// Whether this provider is currently active.
    pub enabled: bool,
}

#[derive(Default, Serialize, Deserialize)]
struct AccountsConfig {
    #[serde(default)]
    providers: Vec<OidcProvider>,
}

impl AccountsConfig {
    fn path() -> std::path::PathBuf {
        crate::config_path("accounts.toml")
    }

    fn load() -> Vec<OidcProvider> {
        let content = std::fs::read_to_string(Self::path()).unwrap_or_default();
        toml::from_str::<AccountsConfig>(&content)
            .unwrap_or_default()
            .providers
    }

    fn save(providers: &[OidcProvider]) -> Result<(), String> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let cfg = AccountsConfig {
            providers: providers.to_vec(),
        };
        let content = toml::to_string_pretty(&cfg).map_err(|e| e.to_string())?;
        std::fs::write(&path, content).map_err(|e| e.to_string())
    }
}

// ── AddProviderForm ────────────────────────────────────────────────────────────

/// Form state for adding a new OIDC provider.
#[derive(Clone, Default, Debug)]
pub struct AddProviderForm {
    pub name: String,
    pub discovery_url: String,
    pub client_id: String,
    pub scopes: String,
}

impl AddProviderForm {
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.name.trim().is_empty()
            && !self.discovery_url.trim().is_empty()
            && !self.client_id.trim().is_empty()
    }

    #[must_use]
    pub fn build(&self) -> OidcProvider {
        OidcProvider {
            name: self.name.trim().to_string(),
            discovery_url: self.discovery_url.trim().to_string(),
            client_id: self.client_id.trim().to_string(),
            scopes: if self.scopes.trim().is_empty() {
                "openid email profile".to_string()
            } else {
                self.scopes.trim().to_string()
            },
            enabled: true,
        }
    }
}

// ── AccountsState ─────────────────────────────────────────────────────────────

/// Runtime state for the Accounts settings section.
#[derive(Debug, Clone)]
pub struct AccountsState {
    pub providers: Vec<OidcProvider>,
    pub show_add: bool,
    pub form: AddProviderForm,
    pub error: Option<String>,
}

impl AccountsState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            providers: AccountsConfig::load(),
            show_add: false,
            form: AddProviderForm::default(),
            error: None,
        }
    }

    pub fn save(&mut self) {
        match AccountsConfig::save(&self.providers) {
            Ok(()) => self.error = None,
            Err(e) => self.error = Some(format!("Save error: {e}")),
        }
    }
}

impl Default for AccountsState {
    fn default() -> Self {
        Self::new()
    }
}

// ── view_accounts ─────────────────────────────────────────────────────────────

/// Render the Accounts settings section.
pub fn view_accounts(app: &SettingsApp) -> Element<'_, Message> {
    let state = &app.accounts;

    // Header row
    let toggle_label = if state.show_add {
        fs_i18n::t("actions.cancel").to_string()
    } else {
        fs_i18n::t("settings-accounts-btn-connect").to_string()
    };
    let header_row = row![
        text(fs_i18n::t("settings-accounts-title").to_string())
            .size(16)
            .width(Length::Fill),
        button(text(toggle_label).size(13))
            .padding([6, 14])
            .on_press(Message::AccountsToggleAddForm),
    ]
    .align_y(Alignment::Center)
    .spacing(8);

    // Add-provider form
    let add_form: Element<Message> = if state.show_add {
        let name_input = text_input("e.g. Kanidm", &state.form.name)
            .on_input(Message::AccountsFormNameChanged)
            .padding([6, 10]);
        let url_input = text_input("https://auth.example.com", &state.form.discovery_url)
            .on_input(Message::AccountsFormDiscoveryUrlChanged)
            .padding([6, 10]);
        let client_input = text_input("e.g. fs-desktop", &state.form.client_id)
            .on_input(Message::AccountsFormClientIdChanged)
            .padding([6, 10]);
        let scopes_input = text_input("openid email profile", &state.form.scopes)
            .on_input(Message::AccountsFormScopesChanged)
            .padding([6, 10]);

        let add_btn = {
            let b = button(text(fs_i18n::t("settings-accounts-btn-save").to_string()).size(13))
                .padding([7, 20]);
            if state.form.is_valid() {
                b.on_press(Message::AccountsAddProvider)
            } else {
                b
            }
        };

        column![
            text(fs_i18n::t("settings-accounts-form-title").to_string()).size(14),
            row![
                column![
                    text(fs_i18n::t("labels.name").to_string()).size(12),
                    name_input,
                ]
                .spacing(4)
                .width(Length::Fill),
                column![
                    text(fs_i18n::t("settings-accounts-label-client-id").to_string()).size(12),
                    client_input,
                ]
                .spacing(4)
                .width(Length::Fill),
            ]
            .spacing(12),
            text(fs_i18n::t("settings-accounts-label-discovery-url").to_string()).size(12),
            url_input,
            text(fs_i18n::t("settings-accounts-label-scopes").to_string()).size(12),
            scopes_input,
            add_btn,
        ]
        .spacing(8)
        .padding([12, 0])
        .into()
    } else {
        fs_gui_engine_iced::iced::widget::Space::new()
            .height(0)
            .into()
    };

    // Provider list
    let provider_rows: Vec<Element<Message>> = if state.providers.is_empty() {
        vec![text(fs_i18n::t("settings-accounts-empty").to_string())
            .size(13)
            .into()]
    } else {
        state
            .providers
            .iter()
            .enumerate()
            .map(|(idx, p)| {
                row![
                    checkbox(p.enabled).on_toggle(move |_| Message::AccountsToggleProvider(idx)),
                    column![
                        text(p.name.as_str()).size(13),
                        text(p.discovery_url.as_str()).size(11),
                        text(format!("client_id: {}  scopes: {}", p.client_id, p.scopes)).size(10),
                    ]
                    .spacing(2)
                    .width(Length::Fill),
                    button(text("x").size(12))
                        .padding([4, 8])
                        .on_press(Message::AccountsRemoveProvider(idx)),
                ]
                .align_y(Alignment::Center)
                .spacing(8)
                .padding([6, 0])
                .into()
            })
            .collect()
    };

    // Error message
    let error_row: Element<Message> = if let Some(err) = &state.error {
        text(err.as_str()).size(12).into()
    } else {
        fs_gui_engine_iced::iced::widget::Space::new()
            .height(0)
            .into()
    };

    let content = column![
        header_row,
        text(fs_i18n::t("settings-accounts-description").to_string()).size(12),
        add_form,
        column(provider_rows).spacing(4),
        error_row,
    ]
    .spacing(12)
    .width(Length::Fill);

    scrollable(content).into()
}
