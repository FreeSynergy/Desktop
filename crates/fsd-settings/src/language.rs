/// Language settings — choose UI language, load language packs from store.
use dioxus::prelude::*;
use fsd_db::package_registry::PackageRegistry;

/// Built-in (always installed) languages.
pub const BUILTIN_LANGUAGES: &[(&str, &str)] = &[
    ("de", "Deutsch"),
    ("en", "English"),
    ("fr", "Français"),
    ("es", "Español"),
    ("it", "Italiano"),
    ("pt", "Português"),
];

/// A language entry (code + native name) with owned strings so store packs can be added.
#[derive(Clone, PartialEq)]
struct LangEntry {
    code: String,
    name: String,
}

/// Language settings component.
///
/// Built-in languages are always present. Language packs installed from the
/// Store are loaded from `PackageRegistry` and appended to the list.
/// When 8+ entries are shown a scrollbar appears.
#[component]
pub fn LanguageSettings() -> Element {
    let installed: Signal<Vec<LangEntry>> = use_signal(|| {
        let mut entries: Vec<LangEntry> = BUILTIN_LANGUAGES
            .iter()
            .map(|(code, name)| LangEntry {
                code: code.to_string(),
                name: name.to_string(),
            })
            .collect();

        // Append store-installed language packs (skip if id already in builtins)
        let builtin_codes: Vec<&str> = BUILTIN_LANGUAGES.iter().map(|(c, _)| *c).collect();
        for pkg in PackageRegistry::by_kind("language") {
            if !builtin_codes.contains(&pkg.id.as_str()) {
                entries.push(LangEntry { code: pkg.id, name: pkg.name });
            }
        }
        entries
    });

    let mut selected = use_signal(|| "de".to_string());
    let mut install_hint = use_signal(|| false);

    let count = installed.read().len();
    let list_style = if count >= 8 {
        "max-height: 240px; overflow-y: auto; border: 1px solid var(--fsn-color-border-default); \
         border-radius: var(--fsn-radius-md); scrollbar-width: thin;"
    } else {
        "border: 1px solid var(--fsn-color-border-default); border-radius: var(--fsn-radius-md);"
    };

    rsx! {
        div {
            class: "fsd-language",
            style: "padding: 24px; max-width: 500px;",

            h3 { style: "margin-top: 0;", "Language" }

            // Installed language list
            div { style: "margin-bottom: 16px;",
                label {
                    style: "display: block; font-weight: 500; margin-bottom: 8px;",
                    "Interface Language"
                    span {
                        style: "margin-left: 8px; font-size: 12px; font-weight: 400; \
                                color: var(--fsn-color-text-muted);",
                        "({count} installed)"
                    }
                }
                div { style: "{list_style}",
                    for entry in installed.read().clone() {
                        LangRow {
                            key: "{entry.code}",
                            code: entry.code.clone(),
                            name: entry.name.clone(),
                            selected: *selected.read() == entry.code,
                            on_select: {
                                let code = entry.code.clone();
                                move |_| *selected.write() = code.clone()
                            },
                        }
                    }
                }
            }

            // "Install more" button
            div { style: "margin-bottom: 24px;",
                button {
                    style: "display: flex; align-items: center; gap: 8px; padding: 8px 16px; \
                            background: var(--fsn-color-bg-surface); \
                            border: 1px solid var(--fsn-color-border-default); \
                            border-radius: var(--fsn-radius-md); font-size: 13px; \
                            cursor: pointer; color: var(--fsn-color-primary); width: 100%;",
                    onclick: move |_| {
                        let cur = *install_hint.read();
                        install_hint.set(!cur);
                    },
                    span { "🌐" }
                    span { "Install more languages…" }
                }
                if *install_hint.read() {
                    div {
                        style: "margin-top: 8px; padding: 10px 14px; \
                                background: var(--fsn-color-bg-surface); \
                                border: 1px solid var(--fsn-color-border-default); \
                                border-radius: var(--fsn-radius-md); font-size: 13px;",
                        "Open "
                        strong { "Store" }
                        " → filter by "
                        strong { "Language" }
                        " to find and install additional language packs."
                    }
                }
            }

            // Apply button
            button {
                style: "padding: 8px 24px; background: var(--fsn-color-primary); \
                        color: white; border: none; border-radius: var(--fsn-radius-md); \
                        cursor: pointer;",
                "Apply"
            }
        }
    }
}

// ── LangRow ───────────────────────────────────────────────────────────────────

#[component]
fn LangRow(
    code: String,
    name: String,
    selected: bool,
    on_select: EventHandler<MouseEvent>,
) -> Element {
    let bg = if selected {
        "background: var(--fsn-color-primary); color: white;"
    } else {
        "background: transparent; color: var(--fsn-color-text-primary);"
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; padding: 10px 14px; \
                    cursor: pointer; transition: background 0.1s; {bg}",
            onclick: on_select,
            span {
                style: "font-size: 16px;",
                if selected { "◉" } else { "○" }
            }
            span { style: "font-size: 14px;", "{name}" }
            span {
                style: "margin-left: auto; font-size: 12px; opacity: 0.6;",
                "{code}"
            }
        }
    }
}
