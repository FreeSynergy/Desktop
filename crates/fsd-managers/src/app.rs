/// Managers app — shows all available managers in a sidebar layout.
use dioxus::prelude::*;
use fsn_components::{FsnSidebar, FsnSidebarItem, FSN_SIDEBAR_CSS};
use fsn_i18n;

use crate::language_panel::LanguageManagerPanel;
use crate::theme_panel::ThemeManagerPanel;
use crate::icons_panel::IconsManagerPanel;
use crate::container_app_panel::ContainerAppManagerPanel;
use crate::bots_panel::BotsManagerPanel;

#[derive(Clone, PartialEq, Debug)]
pub enum ManagerSection {
    Language,
    Theme,
    Icons,
    ContainerApps,
    Bots,
}

impl ManagerSection {
    pub fn id(&self) -> &str {
        match self {
            Self::Language      => "language",
            Self::Theme         => "theme",
            Self::Icons         => "icons",
            Self::ContainerApps => "container_apps",
            Self::Bots          => "bots",
        }
    }

    pub fn label(&self) -> String {
        match self {
            Self::Language      => fsn_i18n::t("managers.section.language"),
            Self::Theme         => fsn_i18n::t("managers.section.theme"),
            Self::Icons         => fsn_i18n::t("managers.section.icons"),
            Self::ContainerApps => fsn_i18n::t("managers.section.container_apps"),
            Self::Bots          => fsn_i18n::t("managers.section.bots"),
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Self::Language      => "🌐",
            Self::Theme         => "🎨",
            Self::Icons         => "🖼",
            Self::ContainerApps => "📦",
            Self::Bots          => "🤖",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "language"       => Some(Self::Language),
            "theme"          => Some(Self::Theme),
            "icons"          => Some(Self::Icons),
            "container_apps" => Some(Self::ContainerApps),
            "bots"           => Some(Self::Bots),
            _                => None,
        }
    }
}

const ALL_SECTIONS: &[ManagerSection] = &[
    ManagerSection::Language,
    ManagerSection::Theme,
    ManagerSection::Icons,
    ManagerSection::ContainerApps,
    ManagerSection::Bots,
];

/// Root Managers component — sidebar (manager list) + detail pane.
#[component]
pub fn ManagersApp() -> Element {
    let mut active = use_signal(|| ManagerSection::Language);

    let sidebar_items: Vec<FsnSidebarItem> = ALL_SECTIONS.iter()
        .map(|s| FsnSidebarItem::new(s.id(), s.icon(), s.label()))
        .collect();

    rsx! {
        style { "{FSN_SIDEBAR_CSS}" }
        div {
            class: "fsd-managers",
            style: "display: flex; flex-direction: column; height: 100%; \
                    background: var(--fsn-color-bg-base);",

            // Title bar
            div {
                style: "padding: 10px 16px; border-bottom: 1px solid var(--fsn-border); \
                        flex-shrink: 0; background: var(--fsn-bg-surface);",
                h2 {
                    style: "margin: 0; font-size: 16px; font-weight: 600; \
                            color: var(--fsn-text-primary);",
                    {fsn_i18n::t("managers.title")}
                }
            }

            // Sidebar + Content
            div {
                style: "display: flex; flex: 1; overflow: hidden;",

                FsnSidebar {
                    items:     sidebar_items,
                    active_id: active.read().id().to_string(),
                    on_select: move |id: String| {
                        if let Some(section) = ManagerSection::from_id(&id) {
                            active.set(section);
                        }
                    },
                }

                div {
                    style: "flex: 1; overflow: auto;",
                    match *active.read() {
                        ManagerSection::Language      => rsx! { LanguageManagerPanel {} },
                        ManagerSection::Theme         => rsx! { ThemeManagerPanel {} },
                        ManagerSection::Icons         => rsx! { IconsManagerPanel {} },
                        ManagerSection::ContainerApps => rsx! { ContainerAppManagerPanel {} },
                        ManagerSection::Bots          => rsx! { BotsManagerPanel {} },
                    }
                }
            }
        }
    }
}
