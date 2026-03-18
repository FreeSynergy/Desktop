/// Built-in app registry — pre-registers only the Store at startup.
///
/// The Store is the only app that exists before anything is installed.
/// All other apps (Browser, Lenses, Tasks, managers, …) must be installed
/// through the Store and are registered dynamically when installed.
///
/// Built-in apps still need to be registered as `kind = "app"` entries in the
/// PackageRegistry so that:
///  - they appear in the Store's "Installed" section with proper metadata
///  - the sidebar reads them dynamically (only registered apps are shown)
///  - users can uninstall (hide) or reinstall (re-show) them via the Store
///
/// Call `ensure_registered()` once at startup — it is idempotent (skips
/// apps that are already in the registry).

use fsd_db::package_registry::{InstalledPackage, PackageRegistry};

/// Metadata for one built-in app.
struct BuiltinApp {
    id:      &'static str,
    name:    &'static str,
    icon:    &'static str,
    version: &'static str,
}

const ICON_STORE: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z"/><line x1="3" y1="6" x2="21" y2="6"/><path d="M16 10a4 4 0 0 1-8 0"/></svg>"#;

/// Only the Store is pre-installed. Everything else is installed via the Store.
const BUILTIN_APPS: &[BuiltinApp] = &[
    BuiltinApp { id: "store", name: "Store", icon: ICON_STORE, version: env!("CARGO_PKG_VERSION") },
];

/// Pre-registers all built-in apps in the PackageRegistry (idempotent).
/// Should be called once at Desktop startup before the sidebar is rendered.
pub fn ensure_registered() {
    for app in BUILTIN_APPS {
        if !PackageRegistry::is_installed(app.id) {
            let pkg = InstalledPackage {
                id:        app.id.to_string(),
                name:      app.name.to_string(),
                kind:      "app".to_string(),
                version:   app.version.to_string(),
                icon:      app.icon.to_string(),
                file_path: None,
            };
            // Ignore write errors at startup — the app still runs, just won't persist.
            let _ = PackageRegistry::install(pkg);
        }
    }
}
