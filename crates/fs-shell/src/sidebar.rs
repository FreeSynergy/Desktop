/// ShellSidebar — left-side navigation panel for the desktop shell.
/// Uses the FsSidebar CSS class (icons-only 48px, expands to 220px on hover).
use dioxus::prelude::*;
use fs_db_desktop::package_registry::{InstalledPackage, PackageKind, PackageRegistry};
use fs_components::{FsSidebar, FsSidebarItem};
use fs_i18n;

use crate::icons::{ICON_MANAGERS, ICON_SETTINGS};

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
    fn is_pinned(&self) -> bool { false }
}

/// An installed app or manager package provides its own nav item.
impl SidebarEntry for InstalledPackage {
    fn nav_item(&self) -> SidebarNavItem {
        let key   = format!("shell.nav.{}", self.id);
        let label = fs_i18n::t(&key);
        let label = if label == key { self.name.clone() } else { label.into() };
        SidebarNavItem {
            id:       self.id.clone(),
            label,
            icon:     self.icon.clone(),
            children: vec![],
        }
    }

    fn is_pinned(&self) -> bool { self.pinned }
}

/// A bundle groups several installed packages under a single folder entry.
/// The bundle itself exposes its own id, icon, and label — OOP, no hard-coding.
pub struct ManagerBundle(pub Vec<InstalledPackage>);

impl SidebarEntry for ManagerBundle {
    fn nav_item(&self) -> SidebarNavItem {
        SidebarNavItem {
            id:       "managers-folder".into(),
            label:    fs_i18n::t("shell.nav.managers").into(),
            icon:     ICON_MANAGERS.into(),
            children: self.0.iter().map(|m| m.nav_item()).collect(),
        }
    }
}

// ── Dynamic registry reads ───────────────────────────────────────────────────

/// All non-pinned installed apps (`kind = "app"`) as nav items.
/// `fs-desktop` is excluded — it is the shell itself, not an openable app.
fn installed_app_items() -> Vec<SidebarNavItem> {
    PackageRegistry::by_kind(PackageKind::App)
        .iter()
        .filter(|pkg| pkg.id != "fs-desktop" && !pkg.pinned)
        .map(|pkg| pkg.nav_item())
        .collect()
}

/// All pinned installed apps (`kind = "app"`) as sidebar items.
/// These appear in the pinned section, above the always-pinned Settings entry.
fn pinned_app_items() -> Vec<FsSidebarItem> {
    PackageRegistry::by_kind(PackageKind::App)
        .iter()
        .filter(|pkg| pkg.id != "fs-desktop" && pkg.pinned)
        .map(|pkg| FsSidebarItem::new(pkg.id.clone(), pkg.icon.clone(), pkg.name.clone()))
        .collect()
}

/// Managers bundle — only returned when at least one manager is installed.
fn installed_manager_bundle() -> Option<SidebarNavItem> {
    let managers = PackageRegistry::by_kind(PackageKind::Manager);
    if managers.is_empty() {
        None
    } else {
        Some(ManagerBundle(managers).nav_item())
    }
}

// ── Sidebar sections ─────────────────────────────────────────────────────────

/// Default sidebar sections for the shell.
///
/// Only shows installed apps from PackageRegistry (`kind = "app"`) and,
/// if present, the managers bundle (`kind = "manager"`).
/// No hardcoded system items — everything must be installed first.
pub fn default_sidebar_sections() -> Vec<SidebarSection> {
    let mut items = installed_app_items();

    // Managers folder — only shows when at least one manager is installed.
    if let Some(bundle) = installed_manager_bundle() {
        items.push(bundle);
    }

    vec![SidebarSection { label: "Apps", items }]
}

// ── Component ────────────────────────────────────────────────────────────────

/// Converts a `SidebarNavItem` into a `FsSidebarItem`, recursively for folders.
fn nav_item_to_fsn(item: &SidebarNavItem) -> FsSidebarItem {
    if item.children.is_empty() {
        FsSidebarItem::new(item.id.clone(), item.icon.clone(), item.label.clone())
    } else {
        let children = item.children.iter().map(nav_item_to_fsn).collect();
        FsSidebarItem::folder(item.id.clone(), item.icon.clone(), item.label.clone(), children)
    }
}

/// Shell sidebar navigation — collapsible (48px → 220px on hover), FsSidebar style.
///
/// - Main section (scrollable): non-pinned installed apps + managers bundle.
/// - Pinned section: user-pinned apps + Settings always at the very bottom.
/// - Right-click on any leaf item → toggles its pinned state.
#[component]
pub fn ShellSidebar(
    sections:  Vec<SidebarSection>,
    active_id: String,
    on_select: EventHandler<String>,
) -> Element {
    // Access the desktop-level sidebar refresh signal so pin changes re-render.
    let mut sidebar_refresh = use_context::<Signal<u32>>();

    let items: Vec<FsSidebarItem> = sections.iter()
        .flat_map(|s| s.items.iter().map(nav_item_to_fsn))
        .collect();

    // Pinned apps from registry, followed by Settings (always at bottom).
    let mut pinned_items = pinned_app_items();
    if PackageRegistry::is_installed("fs-desktop") {
        pinned_items.push(FsSidebarItem::new(
            "settings",
            ICON_SETTINGS,
            fs_i18n::t("shell.nav.settings"),
        ));
    }

    // Right-click on any item toggles its pinned state.
    // "settings" and other non-registry IDs are silently ignored.
    let on_context_menu = move |id: String| {
        let pkgs = PackageRegistry::load();
        if let Some(pkg) = pkgs.iter().find(|p| p.id == id) {
            let _ = PackageRegistry::set_pinned(&id, !pkg.pinned);
            *sidebar_refresh.write() += 1;
        }
    };

    rsx! {
        FsSidebar {
            items,
            pinned_items,
            active_id,
            on_select,
            on_context_menu,
        }
    }
}
