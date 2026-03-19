/// Built-in app and manager registry — pre-registers built-in packages at startup.
///
/// The Store is always present as the entry point for installing everything else.
/// The five built-in managers (Language, Theme, Icons, ContainerApp, Bots) are
/// desktop components that exist without installation.
///
/// Built-in items are registered as PackageRegistry entries so that:
///  - they appear in the sidebar (only registered packages are shown)
///  - they show up as "Installed" in the Store browser
///
/// Call `ensure_registered()` once at startup — it is idempotent.

use fsd_db::package_registry::{InstalledPackage, PackageRegistry};
use crate::icons::{ICON_STORE, ICON_LANGUAGE, ICON_THEME, ICON_ICONS, ICON_CONTAINER, ICON_BOTS};

/// Metadata for one built-in package.
struct BuiltinPkg {
    id:      &'static str,
    name:    &'static str,
    kind:    &'static str,
    icon:    &'static str,
    version: &'static str,
}

// ── Registry ──────────────────────────────────────────────────────────────────

const BUILTIN_PKGS: &[BuiltinPkg] = &[
    // Built-in app — always the entry point
    BuiltinPkg { id: "store",            name: "Store",             kind: "app",     icon: ICON_STORE,     version: env!("CARGO_PKG_VERSION") },
    // Built-in managers — IDs must match AppWindowContent cases ("app-{id}")
    BuiltinPkg { id: "language-manager", name: "Language Manager",  kind: "manager", icon: ICON_LANGUAGE,  version: env!("CARGO_PKG_VERSION") },
    BuiltinPkg { id: "theme-manager",    name: "Theme Manager",     kind: "manager", icon: ICON_THEME,     version: env!("CARGO_PKG_VERSION") },
    BuiltinPkg { id: "icons-manager",    name: "Icons Manager",     kind: "manager", icon: ICON_ICONS,     version: env!("CARGO_PKG_VERSION") },
    BuiltinPkg { id: "container",        name: "Container Manager", kind: "manager", icon: ICON_CONTAINER, version: env!("CARGO_PKG_VERSION") },
    BuiltinPkg { id: "bot-manager",      name: "Bots Manager",      kind: "manager", icon: ICON_BOTS,      version: env!("CARGO_PKG_VERSION") },
];

/// Old IDs that were renamed — remove these on startup to avoid stale sidebar entries.
const LEGACY_IDS: &[&str] = &[
    "manager-language",
    "manager-theme",
    "manager-icons",
    "manager-container-app",
    "manager-bots",
];

/// Pre-registers all built-in packages in the PackageRegistry (idempotent).
/// Should be called once at Desktop startup before the sidebar is rendered.
pub fn ensure_registered() {
    // Remove stale legacy entries so renamed IDs don't produce duplicates.
    for id in LEGACY_IDS {
        if PackageRegistry::is_installed(id) {
            let _ = PackageRegistry::remove(id);
        }
    }

    // Remove any non-builtin "app" entries whose binary no longer exists.
    // These are stale entries from previous auto-registration code. Keeps the
    // sidebar clean: only genuinely installed apps (with a file on disk) and
    // built-in apps (which are part of the Desktop binary) are shown.
    let builtin_ids: std::collections::HashSet<&str> =
        BUILTIN_PKGS.iter().map(|p| p.id).collect();
    for pkg in PackageRegistry::load() {
        if pkg.kind == "app" && !builtin_ids.contains(pkg.id.as_str()) {
            let has_binary = pkg.file_path.as_ref()
                .map_or(false, |p| std::path::Path::new(p).exists());
            if !has_binary {
                let _ = PackageRegistry::remove(&pkg.id);
            }
        }
    }

    for pkg in BUILTIN_PKGS {
        if !PackageRegistry::is_installed(pkg.id) {
            let entry = InstalledPackage {
                id:           pkg.id.to_string(),
                name:         pkg.name.to_string(),
                kind:         pkg.kind.to_string(),
                version:      pkg.version.to_string(),
                icon:         pkg.icon.to_string(),
                file_path:    None,
                installed_by: None,
            };
            let _ = PackageRegistry::install(entry);
        }
    }
}
