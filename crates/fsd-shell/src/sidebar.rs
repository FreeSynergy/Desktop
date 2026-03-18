/// ShellSidebar — left-side navigation panel for the desktop shell.
/// Uses the FsnSidebar CSS class (icons-only 48px, expands to 220px on hover).
use dioxus::prelude::*;
use fsn_components::{FsnSidebarItem, FsnSidebar};
use fsn_i18n;

/// A single navigation item in the sidebar.
/// Items with non-empty `children` are rendered as folders.
#[derive(Clone, PartialEq, Debug)]
pub struct SidebarNavItem {
    pub id:       String,
    pub label:    String,
    pub icon:     String,
    pub children: Vec<SidebarNavItem>,
}

/// A section grouping navigation items.
#[derive(Clone, PartialEq, Debug)]
pub struct SidebarSection {
    pub label: &'static str,
    pub items: Vec<SidebarNavItem>,
}

/// Build the Apps section from the PackageRegistry.
///
/// Only packages with `kind = "app"` are shown. The display label is resolved
/// via i18n (`shell.nav.<id>`) with a fallback to the package name so that
/// third-party apps installed from the Store also get a label even if they
/// have no built-in translation key.
fn installed_app_items() -> Vec<SidebarNavItem> {
    fsd_db::package_registry::PackageRegistry::by_kind("app")
        .into_iter()
        .map(|pkg| {
            let key = format!("shell.nav.{}", pkg.id);
            let label = fsn_i18n::t(&key);
            // Fall back to the package name when no translation key exists.
            let label = if label == key { pkg.name.clone() } else { label };
            SidebarNavItem {
                id:       pkg.id,
                label,
                icon:     pkg.icon,
                children: vec![],
            }
        })
        .collect()
}

/// Default sidebar sections for the shell.
///
/// The **Apps** section is built dynamically from the PackageRegistry so that
/// only installed apps appear. The **System** section (Settings, Profile, AI,
/// Help, Managers) is always present — these are not user-installable.
pub fn default_sidebar_sections() -> Vec<SidebarSection> {
    vec![
        SidebarSection {
            label: "Apps",
            items: installed_app_items(),
        },
        SidebarSection {
            label: "System",
            items: vec![
                SidebarNavItem { id: "settings".into(), label: fsn_i18n::t("shell.nav.settings"),     icon: "⚙".into(),  children: vec![] },
                SidebarNavItem { id: "profile".into(),  label: fsn_i18n::t("shell.nav.profile"),      icon: "👤".into(), children: vec![] },
                SidebarNavItem { id: "ai".into(),       label: fsn_i18n::t("shell.nav.ai_assistant"), icon: "🤖".into(), children: vec![] },
                SidebarNavItem { id: "help".into(),     label: fsn_i18n::t("shell.nav.help"),         icon: "❓".into(), children: vec![] },
                // Managers folder — opens sub-level with all available managers
                SidebarNavItem {
                    id:    "managers-folder".into(),
                    label: fsn_i18n::t("shell.nav.managers"),
                    icon:  "🧩".into(),
                    children: vec![
                        SidebarNavItem { id: "managers".into(), label: fsn_i18n::t("shell.nav.manager_language"),      icon: "🌐".into(), children: vec![] },
                        SidebarNavItem { id: "managers".into(), label: fsn_i18n::t("shell.nav.manager_theme"),         icon: "🎨".into(), children: vec![] },
                        SidebarNavItem { id: "managers".into(), label: fsn_i18n::t("shell.nav.manager_icons"),         icon: "🖼".into(), children: vec![] },
                        SidebarNavItem { id: "managers".into(), label: fsn_i18n::t("shell.nav.manager_container_app"), icon: "📦".into(), children: vec![] },
                        SidebarNavItem { id: "managers".into(), label: fsn_i18n::t("shell.nav.manager_bots"),          icon: "🤖".into(), children: vec![] },
                    ],
                },
            ],
        },
    ]
}

/// Converts a `SidebarNavItem` (shell type) into a `FsnSidebarItem` (component type),
/// recursively converting children so that folder items work correctly.
fn nav_item_to_fsn(item: &SidebarNavItem) -> FsnSidebarItem {
    if item.children.is_empty() {
        FsnSidebarItem::new(item.id.clone(), item.icon.clone(), item.label.clone())
    } else {
        let children = item.children.iter().map(nav_item_to_fsn).collect();
        FsnSidebarItem::folder(item.id.clone(), item.icon.clone(), item.label.clone(), children)
    }
}

/// Shell sidebar navigation — collapsible (48px → 220px on hover), FsnSidebar style.
#[component]
pub fn ShellSidebar(
    sections:  Vec<SidebarSection>,
    active_id: String,
    on_select: EventHandler<String>,
) -> Element {
    let items: Vec<FsnSidebarItem> = sections.iter()
        .flat_map(|s| s.items.iter().map(nav_item_to_fsn))
        .collect();

    rsx! {
        FsnSidebar {
            items,
            active_id,
            on_select,
        }
    }
}
