/// Profile — user profile, avatar, SSH keys, linked OIDC accounts, and personal capabilities.
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Convenience: return translation as owned `String` for use in iced widgets.
fn tr(key: &str) -> String {
    fs_i18n::t(key).to_string()
}

#[cfg(feature = "iced")]
use fs_gui_engine_iced::iced::{
    self,
    widget::{button, column, container, row, scrollable, text, text_input, Space},
    Alignment, Element, Length, Task,
};

// ── PersonalCapability ────────────────────────────────────────────────────────

/// Static metadata for a capability variant.
pub struct CapabilityMeta {
    pub icon: &'static str,
    /// Short, untranslated kind name shown in UI badges.
    pub kind: &'static str,
}

/// A personal resource this user has connected to the system.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PersonalCapability {
    /// User has a Telegram account → can receive personal bot messages.
    MessengerAccount {
        platform: String,
        username: String,
        #[serde(default)]
        verified: bool,
    },
    /// User has personal tasks in a task service (Vikunja).
    TaskManager { service_id: String },
    /// User has a personal mail inbox.
    Mailbox { service_id: String, address: String },
    /// User has a personal LLM assistant configured.
    LlmAssistant { provider: String, model: String },
}

impl PersonalCapability {
    /// Static metadata (icon, kind label) — single source of truth per variant.
    #[must_use]
    pub fn meta(&self) -> CapabilityMeta {
        match self {
            Self::MessengerAccount { .. } => CapabilityMeta {
                icon: "💬",
                kind: "Messenger",
            },
            Self::TaskManager { .. } => CapabilityMeta {
                icon: "✅",
                kind: "Task Manager",
            },
            Self::Mailbox { .. } => CapabilityMeta {
                icon: "📬",
                kind: "Mailbox",
            },
            Self::LlmAssistant { .. } => CapabilityMeta {
                icon: "🤖",
                kind: "LLM Assistant",
            },
        }
    }

    #[must_use]
    pub fn icon(&self) -> &'static str {
        self.meta().icon
    }
    #[must_use]
    pub fn kind_label(&self) -> &'static str {
        self.meta().kind
    }

    #[must_use]
    pub fn label(&self) -> String {
        match self {
            Self::MessengerAccount {
                platform, username, ..
            } => {
                format!("{} (@{})", capitalize(platform), username)
            }
            Self::TaskManager { service_id } => format!("{} ({})", self.kind_label(), service_id),
            Self::Mailbox { address, .. } => format!("{} ({})", self.kind_label(), address),
            Self::LlmAssistant { provider, model } => format!("LLM: {provider} / {model}"),
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// ── Structs ───────────────────────────────────────────────────────────────────

/// A linked OIDC identity from an external provider.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkedAccount {
    pub provider: String,
    pub subject: String,
    pub username: String,
}

/// User profile data.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UserProfile {
    pub display_name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub bio: String,
    pub ssh_keys: Vec<SshKey>,
    pub timezone: String,
    #[serde(default)]
    pub linked_accounts: Vec<LinkedAccount>,
    #[serde(default)]
    pub personal_capabilities: Vec<PersonalCapability>,
}

/// An SSH public key entry.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SshKey {
    pub label: String,
    pub public_key: String,
    pub added_at: String,
}

impl UserProfile {
    fn path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        PathBuf::from(home)
            .join(".config")
            .join("fsn")
            .join("profile.toml")
    }

    /// Load profile from `~/.config/fsn/profile.toml`. Returns default if absent.
    #[must_use]
    pub fn load() -> Self {
        let path = Self::path();
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    }

    /// Save profile to `~/.config/fsn/profile.toml`.
    ///
    /// # Errors
    /// Returns an error string if the directory cannot be created or the file cannot be written.
    pub fn save(&self) -> Result<(), String> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let content = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(&path, content).map_err(|e| e.to_string())
    }
}

// ── ProfileMessage ─────────────────────────────────────────────────────────────

/// All messages for the profile application.
#[derive(Debug, Clone)]
pub enum ProfileMessage {
    // ── Form fields ───────────────────────────────────────────────────────────
    DisplayNameChanged(String),
    EmailChanged(String),
    BioChanged(String),
    TimezoneChanged(String),
    AvatarRemove,

    // ── SSH keys ──────────────────────────────────────────────────────────────
    ShowAddKey,
    HideAddKey,
    NewKeyLabelChanged(String),
    NewKeyValueChanged(String),
    AddKey,
    RemoveKey(usize),

    // ── Linked accounts ───────────────────────────────────────────────────────
    ShowLinkAccount,
    HideLinkAccount,
    LinkProviderChanged(String),
    LinkSubjectChanged(String),
    LinkUsernameChanged(String),
    LinkAdd,
    LinkRemove(usize),

    // ── Persistence ───────────────────────────────────────────────────────────
    Save,
    SaveResult(Result<(), String>),
}

// ── ProfileApp ────────────────────────────────────────────────────────────────

/// Profile application state (iced-based MVU).
#[derive(Debug)]
pub struct ProfileApp {
    pub profile: UserProfile,
    pub save_msg: Option<String>,
    pub save_error: bool,

    // ── SSH key form ─────────────────────────────────────────────────────────
    pub show_add_key: bool,
    pub new_key_label: String,
    pub new_key_value: String,

    // ── Linked account form ───────────────────────────────────────────────────
    pub show_link: bool,
    pub link_provider: String,
    pub link_subject: String,
    pub link_username: String,
}

impl Default for ProfileApp {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfileApp {
    /// Create a new profile app, loading the profile from disk.
    #[must_use]
    pub fn new() -> Self {
        Self {
            profile: UserProfile::load(),
            save_msg: None,
            save_error: false,
            show_add_key: false,
            new_key_label: String::new(),
            new_key_value: String::new(),
            show_link: false,
            link_provider: String::new(),
            link_subject: String::new(),
            link_username: String::new(),
        }
    }
}

// ── update() ─────────────────────────────────────────────────────────────────

#[cfg(feature = "iced")]
impl ProfileApp {
    pub fn update(&mut self, msg: ProfileMessage) -> Task<ProfileMessage> {
        match msg {
            ProfileMessage::DisplayNameChanged(v) => self.profile.display_name = v,
            ProfileMessage::EmailChanged(v) => self.profile.email = v,
            ProfileMessage::BioChanged(v) => self.profile.bio = v,
            ProfileMessage::TimezoneChanged(v) => self.profile.timezone = v,
            ProfileMessage::AvatarRemove => self.profile.avatar_url = None,

            ProfileMessage::ShowAddKey => self.show_add_key = true,
            ProfileMessage::HideAddKey => {
                self.show_add_key = false;
                self.new_key_label.clear();
                self.new_key_value.clear();
            }
            ProfileMessage::NewKeyLabelChanged(v) => self.new_key_label = v,
            ProfileMessage::NewKeyValueChanged(v) => self.new_key_value = v,
            ProfileMessage::AddKey => {
                if !self.new_key_label.is_empty() && !self.new_key_value.is_empty() {
                    self.profile.ssh_keys.push(SshKey {
                        label: self.new_key_label.clone(),
                        public_key: self.new_key_value.clone(),
                        added_at: chrono::Local::now().format("%Y-%m-%d").to_string(),
                    });
                    self.new_key_label.clear();
                    self.new_key_value.clear();
                    self.show_add_key = false;
                }
            }
            ProfileMessage::RemoveKey(idx) => {
                if idx < self.profile.ssh_keys.len() {
                    self.profile.ssh_keys.remove(idx);
                }
            }

            ProfileMessage::ShowLinkAccount => self.show_link = true,
            ProfileMessage::HideLinkAccount => {
                self.show_link = false;
                self.link_provider.clear();
                self.link_subject.clear();
                self.link_username.clear();
            }
            ProfileMessage::LinkProviderChanged(v) => self.link_provider = v,
            ProfileMessage::LinkSubjectChanged(v) => self.link_subject = v,
            ProfileMessage::LinkUsernameChanged(v) => self.link_username = v,
            ProfileMessage::LinkAdd => {
                if !self.link_provider.is_empty() && !self.link_subject.is_empty() {
                    self.profile.linked_accounts.push(LinkedAccount {
                        provider: self.link_provider.clone(),
                        subject: self.link_subject.clone(),
                        username: self.link_username.clone(),
                    });
                    self.show_link = false;
                    self.link_provider.clear();
                    self.link_subject.clear();
                    self.link_username.clear();
                }
            }
            ProfileMessage::LinkRemove(idx) => {
                if idx < self.profile.linked_accounts.len() {
                    self.profile.linked_accounts.remove(idx);
                }
            }

            ProfileMessage::Save => {
                let profile = self.profile.clone();
                return Task::perform(async move { profile.save() }, ProfileMessage::SaveResult);
            }
            ProfileMessage::SaveResult(result) => match result {
                Ok(()) => {
                    self.save_msg = Some(fs_i18n::t("profile.saved").into());
                    self.save_error = false;
                }
                Err(e) => {
                    self.save_msg = Some(e);
                    self.save_error = true;
                }
            },
        }
        Task::none()
    }
}

// ── view() ───────────────────────────────────────────────────────────────────

#[cfg(feature = "iced")]
impl ProfileApp {
    pub fn view(&self) -> Element<'_, ProfileMessage> {
        let title = text(tr("profile.title")).size(24);

        // ── Avatar row ────────────────────────────────────────────────────────
        let avatar_icon = text("👤").size(40);
        let remove_avatar_btn = button(text(tr("actions.remove")).size(12))
            .on_press(ProfileMessage::AvatarRemove)
            .padding([4, 10]);
        let avatar_row = row![avatar_icon, Space::with_width(16), remove_avatar_btn]
            .align_y(Alignment::Center)
            .spacing(8);

        // ── Display name + email ──────────────────────────────────────────────
        let name_placeholder = tr("profile.label.display_name");
        let name_label = text(name_placeholder.clone()).size(12);
        let name_input = text_input(&name_placeholder, &self.profile.display_name)
            .on_input(ProfileMessage::DisplayNameChanged)
            .padding([8, 12])
            .size(14)
            .width(Length::Fill);

        let email_placeholder = tr("profile.label.email");
        let email_label = text(email_placeholder.clone()).size(12);
        let email_input = text_input(&email_placeholder, &self.profile.email)
            .on_input(ProfileMessage::EmailChanged)
            .padding([8, 12])
            .size(14)
            .width(Length::Fill);

        let name_col = column![name_label, Space::with_height(4), name_input].spacing(0);
        let email_col = column![email_label, Space::with_height(4), email_input].spacing(0);
        let name_email_row = row![name_col, Space::with_width(12), email_col].spacing(0);

        // ── Bio ───────────────────────────────────────────────────────────────
        let bio_label = text(tr("profile.label.bio")).size(12);
        let bio_input = text_input("A short description...", &self.profile.bio)
            .on_input(ProfileMessage::BioChanged)
            .padding([8, 12])
            .size(14)
            .width(Length::Fill);

        // ── SSH Keys ─────────────────────────────────────────────────────────
        let ssh_title = text(tr("profile.section.ssh_keys")).size(16);
        let add_key_btn = button(text(tr("profile.btn.add_key")).size(13))
            .on_press(ProfileMessage::ShowAddKey)
            .padding([6, 14]);

        let mut ssh_items: Vec<Element<'_, ProfileMessage>> = self
            .profile
            .ssh_keys
            .iter()
            .enumerate()
            .map(|(idx, key)| {
                let remove_btn = button(text("✕").size(11))
                    .on_press(ProfileMessage::RemoveKey(idx))
                    .padding([2, 6]);
                row![
                    column![
                        text(&key.label).size(13),
                        text(key.public_key.chars().take(48).collect::<String>() + "…")
                            .size(10)
                            .color(iced::Color::from_rgb(0.5, 0.5, 0.6)),
                    ]
                    .spacing(2)
                    .width(Length::Fill),
                    remove_btn,
                ]
                .align_y(Alignment::Center)
                .spacing(8)
                .into()
            })
            .collect();

        if self.show_add_key {
            let key_label_input = text_input("Key label", &self.new_key_label)
                .on_input(ProfileMessage::NewKeyLabelChanged)
                .padding([6, 10])
                .size(13)
                .width(Length::Fill);
            let key_value_input = text_input("ssh-ed25519 AAAA...", &self.new_key_value)
                .on_input(ProfileMessage::NewKeyValueChanged)
                .padding([6, 10])
                .size(13)
                .width(Length::Fill);
            let confirm_btn = button(text(tr("actions.add")).size(13))
                .on_press(ProfileMessage::AddKey)
                .padding([6, 14]);
            let cancel_btn = button(text(tr("actions.cancel")).size(13))
                .on_press(ProfileMessage::HideAddKey)
                .padding([6, 14]);
            ssh_items.push(
                column![
                    key_label_input,
                    Space::with_height(4),
                    key_value_input,
                    Space::with_height(8),
                    row![confirm_btn, Space::with_width(8), cancel_btn].spacing(0),
                ]
                .spacing(0)
                .into(),
            );
        }

        // ── Linked Accounts ───────────────────────────────────────────────────
        let accounts_title = text(tr("profile.section.linked_accounts")).size(16);
        let add_account_btn = button(text(tr("profile.btn.link_account")).size(13))
            .on_press(ProfileMessage::ShowLinkAccount)
            .padding([6, 14]);

        let account_items: Vec<Element<'_, ProfileMessage>> = self
            .profile
            .linked_accounts
            .iter()
            .enumerate()
            .map(|(idx, acc)| {
                let remove_btn = button(text("✕").size(11))
                    .on_press(ProfileMessage::LinkRemove(idx))
                    .padding([2, 6]);
                row![
                    column![
                        text(&acc.provider).size(13),
                        text(&acc.username)
                            .size(11)
                            .color(iced::Color::from_rgb(0.5, 0.5, 0.6)),
                    ]
                    .spacing(2)
                    .width(Length::Fill),
                    remove_btn,
                ]
                .align_y(Alignment::Center)
                .spacing(8)
                .into()
            })
            .collect();

        // ── Save button + status ──────────────────────────────────────────────
        let save_btn = button(text(tr("actions.save")).size(14))
            .on_press(ProfileMessage::Save)
            .padding([8, 24]);

        let status: Element<'_, ProfileMessage> = if let Some(msg) = &self.save_msg {
            let color = if self.save_error {
                iced::Color::from_rgb(0.87, 0.27, 0.27)
            } else {
                iced::Color::from_rgb(0.13, 0.76, 0.37)
            };
            text(msg).size(13).color(color).into()
        } else {
            Space::with_height(0).into()
        };

        let content = column![
            title,
            Space::with_height(24),
            avatar_row,
            Space::with_height(24),
            name_email_row,
            Space::with_height(16),
            bio_label,
            Space::with_height(4),
            bio_input,
            Space::with_height(32),
            ssh_title,
            Space::with_height(8),
            column(ssh_items).spacing(8),
            Space::with_height(8),
            add_key_btn,
            Space::with_height(32),
            accounts_title,
            Space::with_height(8),
            column(account_items).spacing(8),
            Space::with_height(8),
            add_account_btn,
            Space::with_height(32),
            row![save_btn, Space::with_width(16), status]
                .align_y(Alignment::Center)
                .spacing(0),
        ]
        .spacing(0)
        .padding([24, 32])
        .max_width(640);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

// ── Non-iced stubs ────────────────────────────────────────────────────────────

#[cfg(not(feature = "iced"))]
impl ProfileApp {
    pub fn update(&mut self, _msg: ProfileMessage) {}
}
