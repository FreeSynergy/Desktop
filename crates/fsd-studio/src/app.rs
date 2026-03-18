/// Studio — root component: module builder, plugin builder, i18n editor, resource browser.
use dioxus::prelude::*;
use fsn_components::{FsnSidebar, FsnSidebarItem, FSN_SIDEBAR_CSS};

use crate::i18n_editor::I18nEditor;
use crate::module_builder::ModuleBuilder;
use crate::plugin_builder::PluginBuilder;
use crate::resource_browser::ResourceBrowser;

#[derive(Clone, PartialEq, Debug)]
pub enum StudioTab {
    Modules,
    Plugins,
    I18n,
    Resources,
}

impl StudioTab {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Modules   => "modules",
            Self::Plugins   => "plugins",
            Self::I18n      => "i18n",
            Self::Resources => "resources",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Modules   => "Module Builder",
            Self::Plugins   => "Plugin Builder",
            Self::I18n      => "i18n Editor",
            Self::Resources => "Resources",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Modules   => "📦",
            Self::Plugins   => "🔌",
            Self::I18n      => "🌐",
            Self::Resources => "📁",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "modules"   => Some(Self::Modules),
            "plugins"   => Some(Self::Plugins),
            "i18n"      => Some(Self::I18n),
            "resources" => Some(Self::Resources),
            _           => None,
        }
    }
}

const ALL_TABS: &[StudioTab] = &[
    StudioTab::Modules,
    StudioTab::Plugins,
    StudioTab::I18n,
    StudioTab::Resources,
];

/// Root Studio component.
#[component]
pub fn StudioApp() -> Element {
    let mut active_tab = use_signal(|| StudioTab::Modules);

    let sidebar_items: Vec<FsnSidebarItem> = ALL_TABS.iter()
        .map(|t| FsnSidebarItem::new(t.id(), t.icon(), t.label()))
        .collect();

    rsx! {
        style { "{FSN_SIDEBAR_CSS}" }
        div {
            class: "fsd-studio",
            style: "display: flex; flex-direction: column; height: 100%; background: var(--fsn-color-bg-base);",

            // App title bar
            div {
                style: "padding: 10px 16px; border-bottom: 1px solid var(--fsn-border); \
                        flex-shrink: 0; background: var(--fsn-bg-surface);",
                h2 {
                    style: "margin: 0; font-size: 16px; font-weight: 600; color: var(--fsn-text-primary);",
                    "Studio"
                }
            }

            // Sidebar + Content row
            div {
                style: "display: flex; flex: 1; overflow: hidden;",

                FsnSidebar {
                    items: sidebar_items,
                    active_id: active_tab.read().id().to_string(),
                    on_select: move |id: String| {
                        if let Some(tab) = StudioTab::from_id(&id) {
                            active_tab.set(tab);
                        }
                    },
                }

                // Content
                div {
                    style: "flex: 1; overflow: auto;",
                    match *active_tab.read() {
                        StudioTab::Modules   => rsx! { ModuleBuilder {} },
                        StudioTab::Plugins   => rsx! { PluginBuilder {} },
                        StudioTab::I18n      => rsx! { I18nEditor {} },
                        StudioTab::Resources => rsx! { ResourceBrowser {} },
                    }
                }
            }
        }
    }
}
