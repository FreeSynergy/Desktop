/// sidebar.rs — Shell sidebar domain helpers.
///
/// Public API:
///   `SidebarItem`             — a single nav entry (leaf or folder).
///   `SidebarSection`          — a named group of items (may be untitled).
///   `SidebarEntry`            — trait: any type that can be shown as a sidebar item.
///   `ManagerBundle`           — groups manager packages into a single folder item.
///   `default_sidebar_sections()` — main (scrollable) sections for the shell.
///   `default_pinned_items()`     — pinned (bottom) items for the shell.
use fs_db_desktop::package_registry::{InstalledPackage, PackageKind, PackageRegistry};
use fs_i18n;

use crate::icons::{ICON_MANAGERS, ICON_SETTINGS};

// ── SidebarItem ───────────────────────────────────────────────────────────────

/// A single navigation entry in the sidebar.
///
/// Can be a leaf (navigates to an app) or a folder (shows child items).
#[derive(Clone, Debug, PartialEq)]
pub struct SidebarItem {
    pub id: String,
    pub icon: String,
    pub label: String,
    /// Child items if this is a folder. Empty = leaf.
    pub children: Vec<SidebarItem>,
}

impl SidebarItem {
    /// Create a leaf navigation item.
    pub fn new(id: impl Into<String>, icon: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            icon: icon.into(),
            label: label.into(),
            children: vec![],
        }
    }

    /// Create a folder item containing child items.
    pub fn folder(
        id: impl Into<String>,
        icon: impl Into<String>,
        label: impl Into<String>,
        children: Vec<SidebarItem>,
    ) -> Self {
        Self {
            id: id.into(),
            icon: icon.into(),
            label: label.into(),
            children,
        }
    }

    /// Returns `true` if this item is a folder (has children).
    #[must_use]
    pub fn is_folder(&self) -> bool {
        !self.children.is_empty()
    }
}

// ── SidebarSection ────────────────────────────────────────────────────────────

/// A named (or untitled) group of sidebar items.
#[derive(Clone, Debug, PartialEq)]
pub struct SidebarSection {
    /// Optional heading shown above the group. `None` = no heading rendered.
    pub title: Option<String>,
    pub items: Vec<SidebarItem>,
}

impl SidebarSection {
    /// Create a section with a visible title.
    pub fn titled(title: impl Into<String>, items: Vec<SidebarItem>) -> Self {
        Self {
            title: Some(title.into()),
            items,
        }
    }

    /// Create a section without a title heading.
    #[must_use]
    pub fn untitled(items: Vec<SidebarItem>) -> Self {
        Self { title: None, items }
    }
}

// ── SidebarEntry ─────────────────────────────────────────────────────────────

/// Any type that can present itself as a sidebar item.
///
/// The sidebar knows nothing about packages, bots, or any domain type.
/// Domain types implement this trait and supply their own id, icon, and label.
pub trait SidebarEntry {
    fn sidebar_item(&self) -> SidebarItem;
    fn is_pinned(&self) -> bool {
        false
    }
}

/// An installed package knows how to render itself as a sidebar item.
impl SidebarEntry for InstalledPackage {
    fn sidebar_item(&self) -> SidebarItem {
        let key = format!("shell-nav-{}", self.id);
        let label = fs_i18n::t(&key);
        let label = if label == key {
            self.name.clone()
        } else {
            label.into()
        };
        SidebarItem::new(self.id.clone(), self.icon.clone(), label)
    }

    fn is_pinned(&self) -> bool {
        self.pinned
    }
}

// ── ManagerBundle ─────────────────────────────────────────────────────────────

/// Groups manager packages into a single folder item.
///
/// When exactly one manager is installed, the single-item folder rule in
/// the sidebar renders the manager directly instead of the folder.
pub struct ManagerBundle(pub Vec<InstalledPackage>);

impl SidebarEntry for ManagerBundle {
    fn sidebar_item(&self) -> SidebarItem {
        SidebarItem::folder(
            "managers-folder",
            ICON_MANAGERS,
            fs_i18n::t("shell-nav-managers"),
            self.0.iter().map(SidebarEntry::sidebar_item).collect(),
        )
    }
}

// ── Registry helpers ──────────────────────────────────────────────────────────

/// All non-pinned installed apps (`kind = "app"`) as sidebar items.
/// `fs-desktop` is excluded — it is the shell itself.
fn installed_app_items() -> Vec<SidebarItem> {
    PackageRegistry::by_kind(PackageKind::App)
        .iter()
        .filter(|pkg| pkg.id != "fs-desktop" && !pkg.pinned)
        .map(SidebarEntry::sidebar_item)
        .collect()
}

/// The managers folder — `None` when no managers are installed.
fn installed_manager_bundle() -> Option<SidebarItem> {
    let managers = PackageRegistry::by_kind(PackageKind::Manager);
    if managers.is_empty() {
        None
    } else {
        Some(ManagerBundle(managers).sidebar_item())
    }
}

// ── Public section builders ───────────────────────────────────────────────────

/// Main (scrollable) sidebar sections for the desktop shell.
///
/// Contains all non-pinned installed apps and, if present, the managers folder.
/// Everything comes from the `PackageRegistry` — nothing is hard-coded here.
#[must_use]
pub fn default_sidebar_sections() -> Vec<SidebarSection> {
    let mut items = installed_app_items();

    if let Some(bundle) = installed_manager_bundle() {
        items.push(bundle);
    }

    vec![SidebarSection::untitled(items)]
}

/// Pinned (bottom) sidebar items for the desktop shell.
///
/// Includes all user-pinned apps and, when `fs-desktop` is registered,
/// the Settings entry (always at the very bottom).
#[must_use]
pub fn default_pinned_items() -> Vec<SidebarItem> {
    let mut pinned: Vec<SidebarItem> = PackageRegistry::by_kind(PackageKind::App)
        .iter()
        .filter(|pkg| pkg.id != "fs-desktop" && pkg.pinned)
        .map(SidebarEntry::sidebar_item)
        .collect();

    if PackageRegistry::is_installed("fs-desktop") {
        pinned.push(SidebarItem::new(
            "settings",
            ICON_SETTINGS,
            fs_i18n::t("shell-nav-settings"),
        ));
    }

    pinned
}
