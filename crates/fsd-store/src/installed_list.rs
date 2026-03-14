/// Installed list — shows running containers that carry `fsn.module.id` labels.
///
/// Each entry maps to a store package. "Remove" stops the container and
/// disables the systemd unit via SystemdManager.
use dioxus::prelude::*;
use fsn_container::{ContainerInfo, PodmanClient, RunState, SystemdManager};

// ── InstalledEntry ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub struct InstalledEntry {
    pub name:    String,
    pub module:  String, // fsn.module.id label value
    pub version: String, // fsn.module.version label value
    pub running: bool,
}

impl From<&ContainerInfo> for InstalledEntry {
    fn from(c: &ContainerInfo) -> Self {
        Self {
            name:    c.name.clone(),
            module:  c.labels.get("fsn.module.id").cloned().unwrap_or_else(|| c.name.clone()),
            version: c.labels.get("fsn.module.version").cloned().unwrap_or_else(|| "?".into()),
            running: c.state == RunState::Running,
        }
    }
}

// ── InstalledList ─────────────────────────────────────────────────────────────

/// Component that lists installed (label-tagged) containers with a Remove button.
#[component]
pub fn InstalledList(catalog_versions: Vec<(String, String)>) -> Element {
    let mut entries: Signal<Vec<InstalledEntry>>  = use_signal(Vec::new);
    let mut error:   Signal<Option<String>>        = use_signal(|| None);
    let mut confirm: Signal<Option<InstalledEntry>> = use_signal(|| None);

    // Fetch containers on mount
    use_future(move || async move {
        loop {
            match PodmanClient::new() {
                Ok(client) => match client.list(true).await {
                    Ok(list) => {
                        // Keep only containers with fsn.module.id label
                        let tagged: Vec<InstalledEntry> = list
                            .iter()
                            .filter(|c| c.labels.contains_key("fsn.module.id"))
                            .map(InstalledEntry::from)
                            .collect();
                        entries.set(tagged);
                        error.set(None);
                    }
                    Err(e) => error.set(Some(format!("List error: {e}"))),
                },
                Err(e) => error.set(Some(format!("Cannot connect to Podman: {e}"))),
            }
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });

    rsx! {
        div {
            // Confirm dialog overlay
            if let Some(entry) = confirm.read().clone() {
                RemoveConfirmDialog {
                    entry: entry.clone(),
                    on_confirm: move |_| {
                        let entry = entry.clone();
                        spawn(async move {
                            if let Ok(client) = PodmanClient::new() {
                                let _ = client.stop(&entry.name, Some(5)).await;
                            }
                            // Disable systemd unit
                            let unit = format!("{}.service", entry.name);
                            let _ = SystemdManager::new().disable(&unit).await;
                        });
                        *confirm.write() = None;
                    },
                    on_cancel: move |_| *confirm.write() = None,
                }
            }

            // Error
            if let Some(err) = error.read().as_deref() {
                div {
                    style: "color: var(--fsn-color-error); font-size: 13px; margin-bottom: 12px;",
                    "{err}"
                }
            }

            if entries.read().is_empty() {
                div {
                    style: "text-align: center; color: var(--fsn-color-text-muted); padding: 48px;",
                    p { "No installed FreeSynergy modules found." }
                    p { style: "font-size: 12px;",
                        "Containers must carry the label "
                        code { "fsn.module.id" }
                        " to appear here."
                    }
                }
            } else {
                table {
                    style: "width: 100%; border-collapse: collapse;",
                    thead {
                        tr {
                            style: "border-bottom: 1px solid var(--fsn-color-border-default); font-size: 12px; color: var(--fsn-color-text-muted);",
                            th { style: "text-align: left; padding: 8px;",           "NAME" }
                            th { style: "text-align: left; padding: 8px;",           "MODULE" }
                            th { style: "text-align: left; padding: 8px;",           "VERSION" }
                            th { style: "text-align: left; padding: 8px;",           "STATUS" }
                            th { style: "text-align: right; padding: 8px;",          "ACTIONS" }
                        }
                    }
                    tbody {
                        for entry in entries.read().iter().cloned().collect::<Vec<_>>() {
                            InstalledRow {
                                entry: entry.clone(),
                                catalog_versions: catalog_versions.clone(),
                                on_remove: move |e: InstalledEntry| {
                                    *confirm.write() = Some(e);
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── InstalledRow ──────────────────────────────────────────────────────────────

#[component]
fn InstalledRow(
    entry: InstalledEntry,
    catalog_versions: Vec<(String, String)>,
    on_remove: EventHandler<InstalledEntry>,
) -> Element {
    let status_color = if entry.running {
        "var(--fsn-color-success)"
    } else {
        "var(--fsn-color-text-muted)"
    };
    let status_label = if entry.running { "Running" } else { "Stopped" };

    // Check for update
    let catalog_ver = catalog_versions
        .iter()
        .find(|(id, _)| id == &entry.module)
        .map(|(_, v)| v.as_str());
    let has_update = catalog_ver.map(|cv| cv != entry.version.as_str()).unwrap_or(false);

    rsx! {
        tr {
            style: "border-bottom: 1px solid var(--fsn-color-border-default);",

            td { style: "padding: 10px 8px; font-weight: 500;", "{entry.name}" }
            td { style: "padding: 10px 8px; font-size: 13px; color: var(--fsn-color-text-muted);", "{entry.module}" }
            td { style: "padding: 10px 8px; font-size: 13px;",
                "{entry.version}"
                if has_update {
                    if let Some(cv) = catalog_ver {
                        span {
                            style: "margin-left: 6px; font-size: 11px; background: var(--fsn-color-warning); color: black; padding: 1px 5px; border-radius: 4px;",
                            "→ {cv}"
                        }
                    }
                }
            }
            td { style: "padding: 10px 8px;",
                span { style: "font-size: 13px; color: {status_color};", "{status_label}" }
            }
            td { style: "padding: 10px 8px; text-align: right;",
                button {
                    style: "padding: 4px 10px; background: var(--fsn-color-error); color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;",
                    onclick: {
                        let e = entry.clone();
                        move |_| on_remove.call(e.clone())
                    },
                    "Remove"
                }
            }
        }
    }
}

// ── RemoveConfirmDialog ───────────────────────────────────────────────────────

#[component]
fn RemoveConfirmDialog(
    entry: InstalledEntry,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            style: "position: fixed; inset: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;",
            div {
                style: "background: var(--fsn-color-bg-surface); border: 1px solid var(--fsn-color-border-default); border-radius: var(--fsn-radius-lg); padding: 24px; max-width: 400px; width: 100%;",
                h3 { style: "margin: 0 0 12px 0;", "Remove {entry.name}?" }
                p {
                    style: "color: var(--fsn-color-text-muted); font-size: 14px; margin-bottom: 20px;",
                    "This will stop the container and disable its systemd unit. "
                    "Data volumes will not be deleted."
                }
                div {
                    style: "display: flex; gap: 8px; justify-content: flex-end;",
                    button {
                        style: "padding: 8px 16px; background: var(--fsn-color-bg-overlay); border: 1px solid var(--fsn-color-border-default); border-radius: var(--fsn-radius-md); cursor: pointer;",
                        onclick: move |_| on_cancel.call(()),
                        "Cancel"
                    }
                    button {
                        style: "padding: 8px 16px; background: var(--fsn-color-error); color: white; border: none; border-radius: var(--fsn-radius-md); cursor: pointer;",
                        onclick: move |_| on_confirm.call(()),
                        "Remove"
                    }
                }
            }
        }
    }
}
