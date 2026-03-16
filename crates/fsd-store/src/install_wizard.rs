/// Install wizard — step-by-step configuration before first start.
use dioxus::prelude::*;
use fsd_db::package_registry::{InstalledPackage, PackageRegistry};
use fsn_store::StoreClient;

use crate::node_package::PackageKind;
use crate::package_card::PackageEntry;

#[derive(Clone, PartialEq, Debug)]
pub enum WizardStep {
    Overview,
    Configure,
    Confirm,
    Installing,
    Done,
    Error,
}

impl WizardStep {
    pub fn label(&self) -> &str {
        match self {
            Self::Overview   => "Overview",
            Self::Configure  => "Configure",
            Self::Confirm    => "Confirm",
            Self::Installing => "Installing",
            Self::Done       => "Done",
            Self::Error      => "Error",
        }
    }

    pub fn next(&self) -> Option<Self> {
        match self {
            Self::Overview   => Some(Self::Configure),
            Self::Configure  => Some(Self::Confirm),
            Self::Confirm    => Some(Self::Installing),
            // Installing transitions via async callback, not next()
            _ => None,
        }
    }
}

// ── async install logic ────────────────────────────────────────────────────────

/// Downloads and registers a package. Returns the display message on success.
async fn do_install(package: PackageEntry) -> Result<(), String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let fsn_dir = std::path::PathBuf::from(&home).join(".local/share/fsn");

    let store_path = package.store_path.as_deref()
        .map(|p| p.trim_end_matches('/').to_string());

    let file_path: Option<String> = match &package.kind {
        PackageKind::Language => {
            let base = store_path.unwrap_or_else(|| format!("shared/i18n/{}", package.id));
            let url  = format!("{base}/ui.toml");
            match StoreClient::node_store().fetch_raw(&url).await {
                Ok(content) => {
                    let dest_dir = fsn_dir.join("i18n").join(&package.id);
                    std::fs::create_dir_all(&dest_dir).map_err(|e| e.to_string())?;
                    let dest = dest_dir.join("ui.toml");
                    std::fs::write(&dest, content).map_err(|e| e.to_string())?;
                    Some(dest.to_string_lossy().into_owned())
                }
                Err(e) => {
                    tracing::warn!("Language pack download failed (registering anyway): {e}");
                    None
                }
            }
        }
        PackageKind::Theme => {
            let base = store_path.unwrap_or_else(|| format!("shared/themes/{}", package.id));
            let url  = format!("{base}/theme.css");
            match StoreClient::node_store().fetch_raw(&url).await {
                Ok(content) => {
                    let dest_dir = fsn_dir.join("themes");
                    std::fs::create_dir_all(&dest_dir).map_err(|e| e.to_string())?;
                    let dest = dest_dir.join(format!("{}.css", package.id));
                    std::fs::write(&dest, content).map_err(|e| e.to_string())?;
                    Some(dest.to_string_lossy().into_owned())
                }
                Err(e) => {
                    tracing::warn!("Theme download failed (registering anyway): {e}");
                    None
                }
            }
        }
        // Widget, Bot, Task, Bridge, Plugin — register without file download
        _ => None,
    };

    PackageRegistry::install(InstalledPackage {
        id:        package.id.clone(),
        name:      package.name.clone(),
        kind:      package.kind.kind_str(),
        version:   package.version.clone(),
        file_path,
    })
    .map_err(|e| format!("Registry error: {e}"))
}

// ── InstallWizard component ────────────────────────────────────────────────────

/// Install wizard — guides the user through pre-install configuration.
#[component]
pub fn InstallWizard(package: PackageEntry, on_cancel: EventHandler<()>) -> Element {
    let mut step          = use_signal(|| WizardStep::Overview);
    let mut install_error = use_signal(|| Option::<String>::None);

    let visible_steps = [
        WizardStep::Overview,
        WizardStep::Configure,
        WizardStep::Confirm,
    ];

    let current = step.read().clone();

    rsx! {
        div {
            class: "fsd-install-wizard",
            style: "display: flex; flex-direction: column; height: 100%;",

            // Step indicator
            div {
                style: "display: flex; align-items: center; padding: 16px; \
                        border-bottom: 1px solid var(--fsn-color-border-default);",
                for (i, s) in visible_steps.iter().enumerate() {
                    WizardStepDot {
                        key: "{i}",
                        index: i,
                        label: s.label().to_string(),
                        active: current == *s,
                        done: matches!((&current, s),
                            (WizardStep::Confirm, WizardStep::Overview)
                            | (WizardStep::Confirm, WizardStep::Configure)
                            | (WizardStep::Installing, _)
                            | (WizardStep::Done, _)),
                        last: i >= visible_steps.len() - 1,
                    }
                }
            }

            // Step content
            div {
                style: "flex: 1; overflow: auto; padding: 24px;",
                match &current {
                    WizardStep::Overview => rsx! {
                        h3 { style: "margin-top: 0;", "Install {package.name}" }
                        p { style: "color: var(--fsn-color-text-secondary);", "{package.description}" }
                        p { style: "color: var(--fsn-color-text-muted); font-size: 13px;",
                            "Version: {package.version} · Type: {package.kind.label()}"
                        }
                        if !package.tags.is_empty() {
                            div { style: "display: flex; flex-wrap: wrap; gap: 6px; margin-top: 12px;",
                                for tag in &package.tags {
                                    span {
                                        key: "{tag}",
                                        style: "font-size: 11px; padding: 2px 8px; border-radius: 999px; \
                                                background: var(--fsn-color-bg-overlay); \
                                                border: 1px solid var(--fsn-color-border-default); \
                                                color: var(--fsn-color-text-muted);",
                                        "{tag}"
                                    }
                                }
                            }
                        }
                    },
                    WizardStep::Configure => rsx! {
                        h3 { style: "margin-top: 0;", "Configure {package.name}" }
                        { match &package.kind {
                            PackageKind::Language => rsx! {
                                p { style: "color: var(--fsn-color-text-muted);",
                                    "The language pack will be downloaded and saved locally. \
                                     Select it in Settings → Language after installation."
                                }
                            },
                            PackageKind::Theme => rsx! {
                                p { style: "color: var(--fsn-color-text-muted);",
                                    "The theme CSS will be downloaded and saved locally. \
                                     Activate it in Settings → Appearance after installation."
                                }
                            },
                            PackageKind::Widget => rsx! {
                                p { style: "color: var(--fsn-color-text-muted);",
                                    "The widget will be registered. Add it to your desktop \
                                     via Edit Desktop → Add Widget."
                                }
                            },
                            _ => rsx! {
                                p { style: "color: var(--fsn-color-text-muted);",
                                    "No additional configuration required for this package type."
                                }
                            },
                        }}
                    },
                    WizardStep::Confirm => rsx! {
                        h3 { style: "margin-top: 0;", "Ready to install" }
                        p { "Click Install to download and register {package.name} v{package.version}." }
                        div {
                            style: "margin-top: 12px; padding: 12px 16px; \
                                    background: var(--fsn-color-bg-surface); \
                                    border: 1px solid var(--fsn-color-border-default); \
                                    border-radius: var(--fsn-radius-md); font-size: 13px;",
                            div { "Package: {package.name}" }
                            div { "Version: {package.version}" }
                            div { "Type: {package.kind.label()}" }
                        }
                    },
                    WizardStep::Installing => rsx! {
                        div { style: "text-align: center; padding: 48px;",
                            if let Some(err) = install_error.read().as_deref() {
                                div {
                                    style: "color: var(--fsn-color-error, #ef4444); \
                                            background: rgba(239,68,68,0.1); \
                                            border: 1px solid var(--fsn-color-error, #ef4444); \
                                            border-radius: var(--fsn-radius-md); \
                                            padding: 12px; font-size: 13px; text-align: left;",
                                    p { strong { "Installation failed" } }
                                    p { "{err}" }
                                }
                            } else {
                                p { style: "font-size: 32px; margin-bottom: 12px;", "⏳" }
                                p { "Installing {package.name}…" }
                            }
                        }
                    },
                    WizardStep::Done => rsx! {
                        div { style: "text-align: center; padding: 48px;",
                            p { style: "font-size: 48px; margin-bottom: 12px;", "✓" }
                            p { style: "font-size: 18px; font-weight: 600;",
                                "{package.name} installed"
                            }
                            p { style: "color: var(--fsn-color-text-muted); font-size: 13px; margin-top: 8px;",
                                { match &package.kind {
                                    PackageKind::Language => "Select it in Settings → Language.",
                                    PackageKind::Theme    => "Activate it in Settings → Appearance.",
                                    PackageKind::Widget   => "Add it via Edit Desktop → Add Widget.",
                                    _                     => "Package is ready to use.",
                                }}
                            }
                        }
                    },
                    WizardStep::Error => rsx! {
                        div { style: "text-align: center; padding: 48px; color: var(--fsn-color-error, #ef4444);",
                            p { style: "font-size: 32px;", "✗" }
                            p { "Installation failed." }
                        }
                    },
                }
            }

            // Navigation buttons
            div {
                style: "display: flex; justify-content: space-between; padding: 16px; \
                        border-top: 1px solid var(--fsn-color-border-default);",

                // Left: Cancel / Close
                button {
                    style: "padding: 8px 16px; background: var(--fsn-color-bg-surface); \
                            border: 1px solid var(--fsn-color-border-default); \
                            border-radius: var(--fsn-radius-md); cursor: pointer;",
                    onclick: move |_| on_cancel.call(()),
                    { if matches!(*step.read(), WizardStep::Done | WizardStep::Error) {
                        "Close"
                    } else {
                        "Cancel"
                    }}
                }

                // Right: Next / Install (hidden when Installing or Done)
                { match &current {
                    WizardStep::Installing => rsx! {
                        span {} // placeholder — buttons hidden while installing
                    },
                    WizardStep::Done | WizardStep::Error => rsx! {
                        span {}
                    },
                    WizardStep::Confirm => rsx! {
                        button {
                            style: "padding: 8px 20px; background: var(--fsn-color-primary); \
                                    color: white; border: none; \
                                    border-radius: var(--fsn-radius-md); cursor: pointer; \
                                    font-weight: 600;",
                            onclick: move |_| {
                                let pkg = package.clone();
                                step.set(WizardStep::Installing);
                                spawn(async move {
                                    match do_install(pkg).await {
                                        Ok(()) => step.set(WizardStep::Done),
                                        Err(e) => {
                                            install_error.set(Some(e));
                                            // Stay on Installing step so error is visible
                                        }
                                    }
                                });
                            },
                            "Install"
                        }
                    },
                    _ => rsx! {
                        button {
                            style: "padding: 8px 20px; background: var(--fsn-color-primary); \
                                    color: white; border: none; \
                                    border-radius: var(--fsn-radius-md); cursor: pointer;",
                            onclick: move |_| {
                                let next = step.read().next();
                                if let Some(n) = next {
                                    step.set(n);
                                }
                            },
                            "Next →"
                        }
                    },
                }}
            }
        }
    }
}

// ── WizardStepDot ─────────────────────────────────────────────────────────────

#[component]
fn WizardStepDot(
    index: usize,
    label: String,
    active: bool,
    done: bool,
    last: bool,
) -> Element {
    let bg    = if active { "var(--fsn-color-primary)" }
                else if done { "var(--fsn-color-success, #22c55e)" }
                else { "var(--fsn-color-bg-overlay)" };
    let color = if active || done { "white" } else { "var(--fsn-color-text-muted)" };
    let text  = if active { "var(--fsn-color-text-primary)" } else { "var(--fsn-color-text-muted)" };
    let num   = index + 1;
    let inner = if done { "✓".to_string() } else { num.to_string() };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 4px;",
            div {
                style: "width: 24px; height: 24px; border-radius: 50%; \
                        display: flex; align-items: center; justify-content: center; \
                        font-size: 12px; background: {bg}; color: {color};",
                "{inner}"
            }
            span {
                style: "font-size: 13px; color: {text};",
                "{label}"
            }
            if !last {
                span { style: "margin: 0 8px; color: var(--fsn-color-text-muted);", "›" }
            }
        }
    }
}
