/// Desktop — root layout: header + content area with auto-hide overlay sidebar.
use std::time::Duration;
use dioxus::prelude::*;

use fsd_conductor::ConductorApp;
use fsd_profile::ProfileApp;
use fsd_settings::SettingsApp;
use fsd_store::StoreApp;
use fsd_studio::StudioApp;

use crate::ai_view::AiApp;
use crate::app_shell::{AppMode, AppShell, GLOBAL_CSS, LayoutA, LayoutC};
use crate::help_view::HelpApp;
use crate::header::{Breadcrumb, ShellHeader};
use crate::launcher::{AppLauncher, LauncherState};
use crate::notification::{NotificationManager, NotificationStack};
use crate::sidebar::{ShellSidebar, SidebarSection, default_sidebar_sections};
use crate::taskbar::{AppEntry, default_apps};
use crate::wallpaper::Wallpaper;
use crate::window::{Window, WindowId, WindowManager};
use crate::window_frame::WindowFrame;

/// Root desktop component.
#[component]
pub fn Desktop() -> Element {
    let wallpaper           = use_signal(Wallpaper::default);
    let mut wm              = use_signal(WindowManager::default);
    let mut apps            = use_signal(default_apps);
    let mut launcher        = use_signal(LauncherState::default);
    let mut notifs          = use_signal(NotificationManager::default);
    let sidebar_sections: Signal<Vec<SidebarSection>> = use_signal(default_sidebar_sections);
    let mut theme: Signal<String> = use_context_provider(|| Signal::new("midnight-blue".to_string()));

    // Sidebar auto-hide: visible = shown (translateX(0)), hidden = off-screen (translateX(-240px))
    let mut sidebar_visible  = use_signal(|| true);
    let mut sidebar_hide_gen = use_signal(|| 0u32);

    let bg = wallpaper.read().to_css_background();

    // ── Theme + menu action handler ────────────────────────────────────────
    let menu_action_handler = move |id: String| {
        match id.as_str() {
            "theme-midnight-blue" => theme.set("midnight-blue".to_string()),
            "theme-cloud-white"   => theme.set("cloud-white".to_string()),
            "theme-cupertino"     => theme.set("cupertino".to_string()),
            "theme-nordic"        => theme.set("nordic".to_string()),
            "theme-rose-pine"     => theme.set("rose-pine".to_string()),
            "launcher"            => launcher.write().toggle(),
            _ => {}
        }
    };

    // ── Sidebar auto-hide logic ────────────────────────────────────────────
    let on_sidebar_enter = move |_: MouseEvent| {
        let gen = *sidebar_hide_gen.read() + 1;
        *sidebar_hide_gen.write() = gen;
        *sidebar_visible.write() = true;
    };
    let on_sidebar_leave = move |_: MouseEvent| {
        let gen = *sidebar_hide_gen.read() + 1;
        *sidebar_hide_gen.write() = gen;
        spawn(async move {
            tokio::time::sleep(Duration::from_secs(2)).await;
            if *sidebar_hide_gen.read() == gen {
                *sidebar_visible.write() = false;
            }
        });
    };
    let on_edge_enter = move |_: MouseEvent| {
        let gen = *sidebar_hide_gen.read() + 1;
        *sidebar_hide_gen.write() = gen;
        *sidebar_visible.write() = true;
    };

    // ── Sidebar app select ─────────────────────────────────────────────────
    let on_sidebar_select = move |app_id: String| {
        open_app(&mut wm, &mut apps, &app_id);
        launcher.write().close();
    };

    // ── Launcher callbacks ──────────────────────────────────────────────────
    let on_launcher_launch = move |app_id: String| {
        open_app(&mut wm, &mut apps, &app_id);
        launcher.write().close();
    };
    let on_launcher_query = move |q: String| { launcher.write().query = q; };
    let on_launcher_close = move |_: ()| { launcher.write().close(); };

    // ── Window manager callbacks ────────────────────────────────────────────
    let on_close_window = move |id: WindowId| {
        wm.write().close(id);
        for app in apps.write().iter_mut() {
            app.windows.retain(|&wid| wid != id);
        }
    };
    let on_focus_window    = move |id: WindowId| { wm.write().focus(id); };
    let on_minimize_window = move |id: WindowId| { wm.write().minimize(id); };
    let on_maximize_window = move |id: WindowId| { wm.write().maximize(id); };

    // ── Notification dismiss ────────────────────────────────────────────────
    let on_dismiss_notif = move |id: u64| { notifs.write().dismiss(id); };

    // ── Derived state ───────────────────────────────────────────────────────
    let launcher_state = launcher.read().clone();
    let notif_items    = notifs.read().items().to_vec();
    let app_list       = apps.read().clone();
    let visible        = *sidebar_visible.read();
    let sidebar_transform = if visible { "translateX(0)" } else { "translateX(-240px)" };

    let active_app_id = wm.read()
        .windows()
        .iter()
        .filter(|w| !w.minimized)
        .max_by_key(|w| w.z_index)
        .and_then(|w| w.title_key.strip_prefix("app-").map(String::from))
        .unwrap_or_default();

    let breadcrumbs = wm.read()
        .windows()
        .iter()
        .filter(|w| !w.minimized)
        .max_by_key(|w| w.z_index)
        .map(|w| {
            let label = w.title_key.trim_start_matches("app-");
            let label = match label {
                "conductor" => "Conductor",
                "store"     => "Store",
                "studio"    => "Studio",
                "settings"  => "Settings",
                "profile"   => "Profile",
                "ai"        => "AI Assistant",
                "help"      => "Help",
                other       => other,
            };
            vec![Breadcrumb::new(label)]
        })
        .unwrap_or_else(|| vec![Breadcrumb::new("Desktop")]);

    rsx! {
        style { "{GLOBAL_CSS}" }

        div {
            id: "fsd-desktop",
            "data-theme": "{theme}",
            style: "
                width: 100vw; height: 100vh; overflow: hidden;
                display: flex; flex-direction: column;
                background: var(--fsn-bg-base);
                {bg}
            ",

            // ── Header ─────────────────────────────────────────────────────
            div { style: "flex-shrink: 0;",
                ShellHeader {
                    breadcrumbs,
                    user_name: "Admin".to_string(),
                    user_avatar: None,
                    on_menu_action: Some(EventHandler::new(menu_action_handler)),
                }
            }

            // ── Content area (sidebar overlay + window area) ────────────────
            div {
                style: "flex: 1; position: relative; overflow: hidden;",

                // Window area (full size — sidebar overlays on top)
                div {
                    id: "fsd-window-area",
                    style: "position: absolute; inset: 0; overflow: hidden;",
                    for window in wm.read().windows().iter().filter(|w| !w.minimized).cloned().collect::<Vec<_>>() {
                        WindowFrame {
                            key: "{window.id.0}",
                            window: window.clone(),
                            on_close: on_close_window,
                            on_focus: on_focus_window,
                            on_minimize: on_minimize_window,
                            on_maximize: on_maximize_window,
                            AppWindowContent { title_key: window.title_key.clone() }
                        }
                    }
                }

                // Sidebar — absolute overlay with auto-hide animation
                div {
                    style: "position: absolute; top: 0; left: 0; height: 100%; z-index: 50; \
                            width: 240px; \
                            transform: {sidebar_transform}; \
                            transition: transform 300ms ease;",
                    onmouseenter: on_sidebar_enter,
                    onmouseleave: on_sidebar_leave,
                    ShellSidebar {
                        sections: sidebar_sections.read().clone(),
                        active_id: active_app_id,
                        on_select: on_sidebar_select,
                    }
                }

                // Edge trigger strip — shows sidebar when mouse approaches the left edge
                if !visible {
                    div {
                        style: "position: absolute; top: 0; left: 0; width: 8px; height: 100%; \
                                z-index: 49; cursor: default;",
                        onmouseenter: on_edge_enter,
                    }
                }

                // App Launcher overlay
                if launcher_state.open {
                    AppLauncher {
                        apps: app_list,
                        query: launcher_state.query.clone(),
                        on_query_change: on_launcher_query,
                        on_launch: on_launcher_launch,
                        on_close: on_launcher_close,
                    }
                }

                // Notification stack
                NotificationStack {
                    notifications: notif_items,
                    on_dismiss: on_dismiss_notif,
                }
            }
        }
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn open_app(wm: &mut Signal<WindowManager>, apps: &mut Signal<Vec<AppEntry>>, app_id: &str) {
    let existing_id = apps
        .read()
        .iter()
        .find(|a| a.id == app_id)
        .and_then(|a| a.windows.first().copied());

    if let Some(win_id) = existing_id {
        wm.write().focus(win_id);
        return;
    }

    let title_key = format!("app-{}", app_id);
    let window = Window::new(title_key);
    let win_id = window.id;
    wm.write().open(window);

    if let Some(app) = apps.write().iter_mut().find(|a| a.id == app_id) {
        app.windows.push(win_id);
    }
    tracing::info!("Opened app: {}", app_id);
}

/// Wraps each app in the appropriate layout (A / B / C).
#[component]
fn AppWindowContent(title_key: String) -> Element {
    match title_key.as_str() {
        "app-conductor" => rsx! {
            AppShell { mode: AppMode::Window,
                ConductorApp {}
            }
        },
        "app-store" => rsx! {
            AppShell { mode: AppMode::Window,
                LayoutA { StoreApp {} }
            }
        },
        "app-studio" => rsx! {
            AppShell { mode: AppMode::Window,
                LayoutA { StudioApp {} }
            }
        },
        "app-settings" => rsx! {
            AppShell { mode: AppMode::Window,
                SettingsApp {}
            }
        },
        "app-profile" => rsx! {
            AppShell { mode: AppMode::Window,
                LayoutC { ProfileApp {} }
            }
        },
        "app-ai" => rsx! {
            AppShell { mode: AppMode::Window,
                LayoutA { AiApp {} }
            }
        },
        "app-help" => rsx! {
            AppShell { mode: AppMode::Window,
                LayoutA { HelpApp {} }
            }
        },
        _ => rsx! {
            div {
                style: "color: var(--fsn-color-text-muted, #94a3b8); font-size: 13px; \
                        display: flex; align-items: center; justify-content: center; height: 200px;",
                "Unknown app: {title_key}"
            }
        },
    }
}
