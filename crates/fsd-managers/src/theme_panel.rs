/// Theme Manager panel — shows active theme, lists available, allows switching.
use dioxus::prelude::*;
use fsn_i18n;
use fsn_manager_theme::ThemeManager;

#[component]
pub fn ThemeManagerPanel() -> Element {
    let mgr       = ThemeManager::new();
    let available = mgr.available();
    let active    = mgr.active();

    let mut selected = use_signal(|| active.id.clone());
    let mut saved    = use_signal(|| false);

    rsx! {
        div {
            style: "padding: 24px; max-width: 480px;",

            h3 { style: "margin-top: 0; color: var(--fsn-text-primary);",
                {fsn_i18n::t("managers.theme.title")}
            }
            p { style: "font-size: 13px; color: var(--fsn-color-text-muted); margin-top: -8px;",
                {fsn_i18n::t("managers.theme.description")}
            }

            div {
                style: "border: 1px solid var(--fsn-color-border-default); \
                        border-radius: var(--fsn-radius-md); overflow: hidden; margin-bottom: 20px;",

                for theme in &available {
                    {
                        let is_active = theme.id == *selected.read();
                        let theme_id  = theme.id.clone();
                        let mode_label = if theme.is_dark { "Dark" } else { "Light" };
                        let bg = if is_active {
                            "background: var(--fsn-sidebar-active-bg, rgba(77,139,245,0.15)); \
                             color: var(--fsn-sidebar-active, #4d8bf5);"
                        } else {
                            "background: transparent; color: var(--fsn-color-text-primary);"
                        };
                        rsx! {
                            div {
                                style: "display: flex; align-items: center; gap: 12px; \
                                        padding: 11px 16px; cursor: pointer; \
                                        border-bottom: 1px solid var(--fsn-color-border-default); \
                                        transition: background 100ms; {bg}",
                                onclick: move |_| {
                                    selected.set(theme_id.clone());
                                    saved.set(false);
                                },
                                span { style: "font-size: 16px;",
                                    if is_active { "◉" } else { "○" }
                                }
                                span { style: "font-size: 20px;",
                                    if theme.is_dark { "🌙" } else { "☀" }
                                }
                                span { style: "font-size: 14px; flex: 1;", "{theme.display_name}" }
                                span {
                                    style: "font-size: 11px; padding: 2px 8px; \
                                            border-radius: 999px; opacity: 0.7; \
                                            background: var(--fsn-color-bg-overlay);",
                                    "{mode_label}"
                                }
                            }
                        }
                    }
                }
            }

            div { style: "display: flex; align-items: center; gap: 12px;",
                button {
                    style: "padding: 8px 24px; background: var(--fsn-color-primary, #06b6d4); \
                            color: white; border: none; border-radius: var(--fsn-radius-md, 6px); \
                            cursor: pointer; font-size: 13px;",
                    onclick: move |_| {
                        let id = selected.read().clone();
                        let mgr = ThemeManager::new();
                        let _ = mgr.set_active(&id);
                        saved.set(true);
                    },
                    {fsn_i18n::t("actions.apply")}
                }
                if *saved.read() {
                    span { style: "font-size: 12px; color: var(--fsn-color-text-muted);",
                        {fsn_i18n::t("managers.saved")}
                    }
                }
            }
        }
    }
}
