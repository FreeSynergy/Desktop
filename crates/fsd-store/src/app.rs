/// Store — root component with package-type tabs.
use dioxus::prelude::*;
use fsn_components::TabBtn;
use fsn_store::StoreClient;

use crate::browser::PackageBrowser;
use crate::installed_list::InstalledList;
use crate::node_package::{NodePackage, PackageKind};
use crate::package_card::PackageEntry;
use crate::package_detail::PackageDetail;

#[derive(Clone, PartialEq, Debug)]
pub enum StoreTab {
    All,
    Plugins,
    Languages,
    Themes,
    Widgets,
    Bots,
    Bridges,
    Tasks,
    Installed,
    Updates,
}

impl StoreTab {
    /// Returns the PackageKind filter for this tab (None = show all).
    pub fn kind_filter(&self) -> Option<PackageKind> {
        match self {
            Self::Plugins   => Some(PackageKind::Plugin),
            Self::Languages => Some(PackageKind::Language),
            Self::Themes    => Some(PackageKind::Theme),
            Self::Widgets   => Some(PackageKind::Widget),
            Self::Bots      => Some(PackageKind::BotCommand),
            Self::Bridges   => Some(PackageKind::Bridge),
            Self::Tasks     => Some(PackageKind::Task),
            Self::All | Self::Installed | Self::Updates => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::All       => "All",
            Self::Plugins   => "Plugins",
            Self::Languages => "Languages",
            Self::Themes    => "Themes",
            Self::Widgets   => "Widgets",
            Self::Bots      => "Bots",
            Self::Bridges   => "Bridges",
            Self::Tasks     => "Tasks",
            Self::Installed => "Installed",
            Self::Updates   => "Updates",
        }
    }
}

const BROWSE_TABS: &[StoreTab] = &[
    StoreTab::All,
    StoreTab::Plugins,
    StoreTab::Languages,
    StoreTab::Themes,
    StoreTab::Widgets,
    StoreTab::Bots,
    StoreTab::Bridges,
    StoreTab::Tasks,
];

/// Root Store component.
#[component]
pub fn StoreApp() -> Element {
    let mut active_tab = use_signal(|| StoreTab::All);
    let mut search = use_signal(String::new);
    let mut detail: Signal<Option<PackageEntry>> = use_signal(|| None);

    let catalog_versions: Signal<Vec<(String, String)>> = use_signal(Vec::new);
    {
        let catalog_versions = catalog_versions.clone();
        use_future(move || {
            let mut catalog_versions = catalog_versions.clone();
            async move {
                if let Ok(catalog) = StoreClient::node_store()
                    .fetch_catalog::<NodePackage>("Node", false)
                    .await
                {
                    catalog_versions.set(
                        catalog.packages.into_iter()
                            .map(|p| (p.id, p.version))
                            .collect(),
                    );
                }
            }
        });
    }

    if let Some(pkg) = detail.read().clone() {
        return rsx! {
            PackageDetail {
                package: pkg,
                on_back: move |_| detail.set(None),
            }
        };
    }

    let tab = active_tab.read().clone();
    let kind_filter = tab.kind_filter();

    rsx! {
        div {
            class: "fsd-store",
            style: "display: flex; flex-direction: column; height: 100%; background: var(--fsn-color-bg-base);",

            // Header
            div {
                style: "padding: 16px; background: var(--fsn-color-bg-surface); border-bottom: 1px solid var(--fsn-color-border-default);",
                h2 { style: "margin: 0 0 12px 0; font-size: 20px;", "Store" }
                input {
                    r#type: "search",
                    placeholder: "Search packages…",
                    style: "width: 100%; padding: 8px 12px; border: 1px solid var(--fsn-color-border-default); \
                            border-radius: var(--fsn-radius-md); font-size: 14px; \
                            background: var(--fsn-bg-input); color: var(--fsn-text-primary);",
                    oninput: move |e| *search.write() = e.value(),
                }
            }

            // Tab bar (scrollable for small windows)
            div {
                style: "display: flex; overflow-x: auto; border-bottom: 1px solid var(--fsn-color-border-default); \
                        scrollbar-width: none;",
                for store_tab in BROWSE_TABS {
                    TabBtn {
                        key: "{store_tab.label()}",
                        label: store_tab.label(),
                        is_active: *active_tab.read() == *store_tab,
                        on_click: {
                            let t = (*store_tab).clone();
                            move |_| active_tab.set(t.clone())
                        }
                    }
                }
                // Separator
                div { style: "width: 1px; height: 24px; margin: auto 4px; background: var(--fsn-border);" }
                TabBtn {
                    label: "Installed",
                    is_active: *active_tab.read() == StoreTab::Installed,
                    on_click: move |_| active_tab.set(StoreTab::Installed)
                }
                TabBtn {
                    label: "Updates",
                    is_active: *active_tab.read() == StoreTab::Updates,
                    on_click: move |_| active_tab.set(StoreTab::Updates)
                }
            }

            // Content
            div {
                style: "flex: 1; overflow: auto; padding: 16px;",
                match *active_tab.read() {
                    StoreTab::Installed => rsx! {
                        InstalledList {
                            catalog_versions: catalog_versions.read().clone(),
                        }
                    },
                    StoreTab::Updates => rsx! {
                        UpdatesList {
                            catalog_versions: catalog_versions.read().clone(),
                        }
                    },
                    _ => rsx! {
                        PackageBrowser {
                            search: search.read().clone(),
                            kind: kind_filter,
                            on_select: move |pkg| detail.set(Some(pkg)),
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn UpdatesList(catalog_versions: Vec<(String, String)>) -> Element {
    rsx! {
        div {
            style: "text-align: center; color: var(--fsn-text-muted); padding: 48px;",
            p { style: "font-size: 24px; margin-bottom: 12px;", "↑" }
            p { style: "margin-bottom: 8px;", "Update detection requires deployment metadata." }
            p { style: "font-size: 13px;",
                "Run "
                code { style: "background: var(--fsn-bg-elevated); padding: 2px 6px; border-radius: 4px;",
                    "fsn deploy"
                }
                " to check and apply updates for all deployed services."
            }
            if !catalog_versions.is_empty() {
                p { style: "margin-top: 16px; font-size: 13px;",
                    "{catalog_versions.len()} package(s) available in the catalog."
                }
            }
        }
    }
}
