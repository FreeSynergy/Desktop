/// Resource view — live CPU / RAM / PID stats for all running containers.
///
/// Polls `PodmanClient::stats_all()` every 3 seconds and renders a table.
use dioxus::prelude::*;
use fsn_container::{ContainerStats, PodmanClient};

// ── ResourceView ──────────────────────────────────────────────────────────────

/// Live resource usage table for all running containers.
#[component]
pub fn ResourceView() -> Element {
    let mut stats: Signal<Vec<ContainerStats>> = use_signal(Vec::new);
    let mut error: Signal<Option<String>>      = use_signal(|| None);

    // Poll every 3 seconds
    use_future(move || async move {
        loop {
            match PodmanClient::new() {
                Ok(client) => match client.stats_all().await {
                    Ok(s) => {
                        stats.set(s);
                        error.set(None);
                    }
                    Err(e) => error.set(Some(format!("Stats error: {e}"))),
                },
                Err(e) => error.set(Some(format!("Cannot connect to Podman: {e}"))),
            }
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });

    rsx! {
        div {
            class: "fsd-resource-view",

            // Header
            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px;",
                h2 { style: "margin: 0; font-size: 18px;", "Resources" }
                span {
                    style: "font-size: 12px; color: var(--fsn-color-text-muted);",
                    "Refreshes every 3 s"
                }
            }

            // Error
            if let Some(err) = error.read().as_deref() {
                div {
                    style: "color: var(--fsn-color-error); background: rgba(239,68,68,0.1); border: 1px solid var(--fsn-color-error); border-radius: 6px; padding: 12px; margin-bottom: 16px; font-size: 13px;",
                    "{err}"
                }
            }

            if stats.read().is_empty() {
                div {
                    style: "text-align: center; color: var(--fsn-color-text-muted); padding: 48px;",
                    "No running containers."
                }
            } else {
                table {
                    style: "width: 100%; border-collapse: collapse;",

                    thead {
                        tr {
                            style: "border-bottom: 1px solid var(--fsn-color-border-default); font-size: 12px; color: var(--fsn-color-text-muted);",
                            th { style: "text-align: left; padding: 8px;",           "NAME" }
                            th { style: "text-align: right; padding: 8px; width: 120px;", "CPU" }
                            th { style: "text-align: right; padding: 8px; width: 180px;", "MEMORY" }
                            th { style: "text-align: right; padding: 8px; width: 80px;",  "PIDs" }
                        }
                    }

                    tbody {
                        for s in stats.read().iter().cloned().collect::<Vec<_>>() {
                            ResourceRow { stats: s }
                        }
                    }
                }
            }
        }
    }
}

// ── ResourceRow ───────────────────────────────────────────────────────────────

#[component]
fn ResourceRow(stats: ContainerStats) -> Element {
    let cpu_color = cpu_color(stats.cpu_percent);
    let mem_pct   = mem_percent(&stats);
    let mem_bar   = mem_bar_width(mem_pct);
    let mem_color = mem_color(mem_pct);

    let mem_label = if stats.memory_limit_mib > 0 {
        format!("{} / {} MiB  ({:.0}%)", stats.memory_mib, stats.memory_limit_mib, mem_pct)
    } else {
        format!("{} MiB", stats.memory_mib)
    };

    rsx! {
        tr {
            style: "border-bottom: 1px solid var(--fsn-color-border-default);",

            // Name
            td { style: "padding: 10px 8px; font-weight: 500;", "{stats.name}" }

            // CPU
            td { style: "padding: 10px 8px; text-align: right;",
                span {
                    style: "font-size: 13px; font-weight: 600; color: {cpu_color};",
                    "{stats.cpu_percent:.1}%"
                }
            }

            // Memory — value + mini progress bar
            td { style: "padding: 10px 8px; text-align: right;",
                div {
                    style: "display: flex; flex-direction: column; align-items: flex-end; gap: 3px;",
                    span { style: "font-size: 12px;", "{mem_label}" }
                    if stats.memory_limit_mib > 0 {
                        div {
                            style: "height: 4px; width: 120px; background: var(--fsn-color-bg-overlay); border-radius: 2px; overflow: hidden;",
                            div {
                                style: "height: 100%; width: {mem_bar}%; background: {mem_color}; border-radius: 2px; transition: width 0.5s;",
                            }
                        }
                    }
                }
            }

            // PIDs
            td { style: "padding: 10px 8px; text-align: right; color: var(--fsn-color-text-muted); font-size: 13px;",
                "{stats.pids}"
            }
        }
    }
}

// ── Colour helpers ────────────────────────────────────────────────────────────

fn cpu_color(pct: f64) -> &'static str {
    if pct >= 80.0 { "var(--fsn-color-error)" }
    else if pct >= 40.0 { "var(--fsn-color-warning)" }
    else { "var(--fsn-color-success)" }
}

fn mem_percent(s: &ContainerStats) -> f64 {
    if s.memory_limit_mib == 0 { return 0.0; }
    (s.memory_mib as f64 / s.memory_limit_mib as f64) * 100.0
}

fn mem_bar_width(pct: f64) -> f64 {
    pct.clamp(0.0, 100.0)
}

fn mem_color(pct: f64) -> &'static str {
    if pct >= 90.0 { "var(--fsn-color-error)" }
    else if pct >= 70.0 { "var(--fsn-color-warning)" }
    else { "var(--fsn-color-primary)" }
}
