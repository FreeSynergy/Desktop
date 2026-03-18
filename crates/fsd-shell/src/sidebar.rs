/// ShellSidebar — left-side navigation panel for the desktop shell.
/// Uses the FsnSidebar CSS class (icons-only 48px, expands to 220px on hover).
use dioxus::prelude::*;
use fsd_db::package_registry::{InstalledPackage, PackageRegistry};
use fsn_components::{FsnSidebarItem, FsnSidebar};
use fsn_i18n;

/// A single navigation item in the sidebar.
/// Items with non-empty `children` are rendered as folders (bundles).
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

// ── OOP Trait ────────────────────────────────────────────────────────────────

/// Any type that can present itself as a sidebar navigation item.
/// Programs expose their own id, icon, and label — the sidebar just renders them.
pub trait SidebarEntry {
    fn nav_item(&self) -> SidebarNavItem;
}

/// An installed app or manager package provides its own nav item.
impl SidebarEntry for InstalledPackage {
    fn nav_item(&self) -> SidebarNavItem {
        let key   = format!("shell.nav.{}", self.id);
        let label = fsn_i18n::t(&key);
        let label = if label == key { self.name.clone() } else { label };
        SidebarNavItem {
            id:       self.id.clone(),
            label,
            icon:     self.icon.clone(),
            children: vec![],
        }
    }
}

/// A bundle groups several installed packages under a single folder entry.
/// The bundle itself exposes its own id, icon, and label — OOP, no hard-coding.
pub struct ManagerBundle(pub Vec<InstalledPackage>);

impl SidebarEntry for ManagerBundle {
    fn nav_item(&self) -> SidebarNavItem {
        SidebarNavItem {
            id:       "managers-folder".into(),
            label:    fsn_i18n::t("shell.nav.managers"),
            icon:     ICON_MANAGERS.into(),
            children: self.0.iter().map(|m| m.nav_item()).collect(),
        }
    }
}

// ── Dynamic registry reads ───────────────────────────────────────────────────

/// All installed apps (`kind = "app"`) as nav items.
fn installed_app_items() -> Vec<SidebarNavItem> {
    PackageRegistry::by_kind("app")
        .iter()
        .map(|pkg| pkg.nav_item())
        .collect()
}

/// Managers bundle — only returned when at least one manager is installed.
fn installed_manager_bundle() -> Option<SidebarNavItem> {
    let managers = PackageRegistry::by_kind("manager");
    if managers.is_empty() {
        None
    } else {
        Some(ManagerBundle(managers).nav_item())
    }
}

// ── Sidebar sections ─────────────────────────────────────────────────────────

// ── SVG icons for system nav items ───────────────────────────────────────────

const ICON_SETTINGS: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>"#;
const ICON_PROFILE: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>"#;
const ICON_AI: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2a4 4 0 0 1 4 4v1h1a3 3 0 0 1 3 3v6a3 3 0 0 1-3 3H7a3 3 0 0 1-3-3v-6a3 3 0 0 1 3-3h1V6a4 4 0 0 1 4-4z"/><line x1="9" y1="16" x2="9" y2="16" stroke-width="3"/><line x1="15" y1="16" x2="15" y2="16" stroke-width="3"/><path d="M9 12h6"/></svg>"#;
const ICON_HELP: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/><line x1="12" y1="17" x2="12.01" y2="17" stroke-width="3"/></svg>"#;
const ICON_MANAGERS: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>"#;

/// Default sidebar sections for the shell.
///
/// - **Apps**: built dynamically from PackageRegistry (`kind = "app"`).
/// - **System**: Settings, Profile, AI, Help — always present.
/// - The Managers bundle is appended to System when managers are installed.
pub fn default_sidebar_sections() -> Vec<SidebarSection> {
    let mut system_items = vec![
        SidebarNavItem { id: "settings".into(), label: fsn_i18n::t("shell.nav.settings"),     icon: ICON_SETTINGS.into(), children: vec![] },
        SidebarNavItem { id: "profile".into(),  label: fsn_i18n::t("shell.nav.profile"),      icon: ICON_PROFILE.into(),  children: vec![] },
        SidebarNavItem { id: "ai".into(),       label: fsn_i18n::t("shell.nav.ai_assistant"), icon: ICON_AI.into(),       children: vec![] },
        SidebarNavItem { id: "help".into(),     label: fsn_i18n::t("shell.nav.help"),         icon: ICON_HELP.into(),     children: vec![] },
    ];

    // Managers folder — only shows when at least one manager is installed.
    if let Some(bundle) = installed_manager_bundle() {
        system_items.push(bundle);
    }

    vec![
        SidebarSection {
            label: "Apps",
            items: installed_app_items(),
        },
        SidebarSection {
            label: "System",
            items: system_items,
        },
    ]
}

// ── Component ────────────────────────────────────────────────────────────────

/// Converts a `SidebarNavItem` into a `FsnSidebarItem`, recursively for folders.
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
