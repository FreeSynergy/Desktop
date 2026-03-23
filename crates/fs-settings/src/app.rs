/// Settings — root component: all settings sections in one place.
use dioxus::prelude::*;
use fs_components::{Sidebar, SidebarItem, FS_SIDEBAR_CSS};
use fs_i18n;

use crate::appearance::AppearanceSettings;
use crate::browser_settings::BrowserSettings;
use crate::language::LanguageSettings;
use crate::service_roles::ServiceRoles;
use crate::accounts::AccountSettings;
use crate::desktop_settings::DesktopSettings;
use crate::shortcuts::ShortcutsSettings;
use crate::package_settings::{PackageSettingsEntry, PackageSettingsView};

#[derive(Clone, PartialEq, Debug)]
pub enum SettingsSection {
    Appearance,
    Language,
    ServiceRoles,
    Accounts,
    Desktop,
    Browser,
    Shortcuts,
    Packages,
}

impl SettingsSection {
    /// Stable identifier used for routing — never translated.
    pub fn id(&self) -> &str {
        match self {
            Self::Appearance   => "appearance",
            Self::Language     => "language",
            Self::ServiceRoles => "service_roles",
            Self::Accounts     => "accounts",
            Self::Desktop      => "desktop",
            Self::Browser      => "browser",
            Self::Shortcuts    => "shortcuts",
            Self::Packages     => "packages",
        }
    }

    /// Translated display label.
    pub fn label(&self) -> String {
        match self {
            Self::Appearance   => fs_i18n::t("settings.section.appearance").into(),
            Self::Language     => fs_i18n::t("settings.section.language").into(),
            Self::ServiceRoles => fs_i18n::t("settings.section.roles").into(),
            Self::Accounts     => fs_i18n::t("settings.section.accounts").into(),
            Self::Desktop      => fs_i18n::t("settings.section.desktop").into(),
            Self::Browser      => fs_i18n::t("settings.section.browser").into(),
            Self::Shortcuts    => fs_i18n::t("settings.section.shortcuts").into(),
            Self::Packages     => fs_i18n::t("settings.section.packages").into(),
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Self::Appearance   => "🎨",
            Self::Language     => "🌐",
            Self::ServiceRoles => "🔗",
            Self::Accounts     => "👤",
            Self::Desktop      => "🖥",
            Self::Browser      => "🌍",
            Self::Shortcuts    => "⌨",
            Self::Packages     => "📦",
        }
    }

    /// Look up a section by its stable ID string.
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "appearance"    => Some(Self::Appearance),
            "language"      => Some(Self::Language),
            "service_roles" => Some(Self::ServiceRoles),
            "accounts"      => Some(Self::Accounts),
            "desktop"       => Some(Self::Desktop),
            "browser"       => Some(Self::Browser),
            "shortcuts"     => Some(Self::Shortcuts),
            "packages"      => Some(Self::Packages),
            _               => None,
        }
    }
}

const STANDARD_SECTIONS: &[SettingsSection] = &[
    SettingsSection::Appearance,
    SettingsSection::Language,
    SettingsSection::ServiceRoles,
    SettingsSection::Accounts,
    SettingsSection::Desktop,
    SettingsSection::Browser,
    SettingsSection::Shortcuts,
];

/// Props for the root Settings component.
///
/// All fields are optional — `SettingsApp` works standalone without any props.
/// When the Desktop provides `packages`, a "Packages" section appears in the sidebar.
#[derive(Props, Clone, PartialEq, Default)]
pub struct SettingsAppProps {
    /// Installed packages whose settings should be surfaced in the Packages section.
    /// When empty (default), the Packages section is hidden.
    #[props(default)]
    pub packages: Vec<PackageSettingsEntry>,

    /// Callback fired when the user saves a package setting.
    /// Receives `(package_id, field_key, new_value)`.
    #[props(default)]
    pub on_package_save: Option<EventHandler<(String, String, String)>>,
}

/// Root Settings component.
///
/// Pass `packages` + `on_package_save` props to enable the Packages section.
#[component]
pub fn SettingsApp(props: SettingsAppProps) -> Element {
    let has_packages = !props.packages.is_empty();
    let mut active = use_signal(|| SettingsSection::Appearance);

    let mut sidebar_items: Vec<SidebarItem> = STANDARD_SECTIONS.iter()
        .map(|s| SidebarItem::new(s.id(), s.icon(), s.label()))
        .collect();

    if has_packages {
        let s = SettingsSection::Packages;
        sidebar_items.push(SidebarItem::new(s.id(), s.icon(), s.label()));
    }

    rsx! {
        style { "{FS_SIDEBAR_CSS}" }
        div {
            class: "fs-settings",
            style: "display: flex; flex-direction: column; height: 100%; background: var(--fs-color-bg-base);",

            // App title bar
            div {
                style: "padding: 10px 16px; border-bottom: 1px solid var(--fs-border); \
                        flex-shrink: 0; background: var(--fs-bg-surface);",
                h2 {
                    style: "margin: 0; font-size: 16px; font-weight: 600; color: var(--fs-text-primary);",
                    {fs_i18n::t("settings.title")}
                }
            }

            // Sidebar + Content row
            div {
                style: "display: flex; flex: 1; overflow: hidden;",

                // Collapsible sidebar navigation
                Sidebar {
                    items:     sidebar_items,
                    active_id: active.read().id().to_string(),
                    on_select: move |id: String| {
                        if let Some(section) = SettingsSection::from_id(&id) {
                            active.set(section);
                        }
                    },
                }

                // Content
                div {
                    style: "flex: 1; overflow: auto;",
                    if *active.read() == SettingsSection::Packages {
                        if has_packages {
                            PackageSettingsView {
                                packages: props.packages.clone(),
                                on_save: props.on_package_save.clone()
                                    .unwrap_or_else(|| EventHandler::new(|_| {})),
                            }
                        }
                    } else {
                        { active.read().render_panel() }
                    }
                }
            } // end sidebar + content row
        }
    }
}

/// Trait that gives a settings section the ability to render itself.
///
/// Extend settings without touching `SettingsApp` — implement this trait.
pub trait SettingsPanel {
    fn render_panel(&self) -> Element;
}

impl SettingsPanel for SettingsSection {
    fn render_panel(&self) -> Element {
        match self {
            Self::Appearance   => rsx! { AppearanceSettings {} },
            Self::Language     => rsx! { LanguageSettings {} },
            Self::ServiceRoles => rsx! { ServiceRoles {} },
            Self::Accounts     => rsx! { AccountSettings {} },
            Self::Desktop      => rsx! { DesktopSettings {} },
            Self::Browser      => rsx! { BrowserSettings {} },
            Self::Shortcuts    => rsx! { ShortcutsSettings {} },
            // Packages is rendered inline in SettingsApp (needs props data).
            Self::Packages     => rsx! { div {} },
        }
    }
}
