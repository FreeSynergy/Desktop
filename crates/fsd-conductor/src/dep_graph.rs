/// Dependency graph — SVG visualisation of container relationships.
///
/// Reads `fsn.requires` labels from running containers and renders
/// directed edges between them as an SVG diagram.
///
/// Label convention (set by fsn-deploy):
///   `fsn.requires = "container-a,container-b"`
use dioxus::prelude::*;
use fsn_container::{ContainerInfo, PodmanClient};
use std::collections::HashMap;

// ── Layout constants ──────────────────────────────────────────────────────────

const NODE_W: f64 = 140.0;
const NODE_H: f64 = 40.0;
const H_GAP: f64  = 60.0;
const V_GAP: f64  = 80.0;
const COLS: usize  = 4;

// ── GraphNode ─────────────────────────────────────────────────────────────────

/// A positioned node in the graph.
#[derive(Clone, Debug)]
struct GraphNode {
    name:   String,
    health: String, // "ok" | "warn" | "err" | "unknown"
    x: f64,
    y: f64,
}

// ── DependencyGraph ───────────────────────────────────────────────────────────

/// SVG dependency-graph component.
///
/// Fetches containers via Podman, lays them out in a grid, and draws
/// arrows for `fsn.requires` edges.
#[component]
pub fn DependencyGraph() -> Element {
    let mut containers: Signal<Vec<ContainerInfo>> = use_signal(Vec::new);
    let mut error: Signal<Option<String>>          = use_signal(|| None);

    // Poll every 5 seconds
    use_future(move || async move {
        loop {
            match PodmanClient::new() {
                Ok(client) => match client.list(true).await {
                    Ok(list) => { containers.set(list); error.set(None); }
                    Err(e)   => error.set(Some(format!("List error: {e}"))),
                },
                Err(e) => error.set(Some(format!("Cannot connect to Podman: {e}"))),
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    let list = containers.read().clone();
    let (nodes, edges) = build_graph(&list);
    let svg_w = (COLS as f64) * (NODE_W + H_GAP) + H_GAP;
    let svg_h = if nodes.is_empty() { 120.0 } else {
        let rows = (nodes.len() as f64 / COLS as f64).ceil();
        rows * (NODE_H + V_GAP) + V_GAP
    };

    rsx! {
        div {
            class: "fsd-dep-graph",

            // Header
            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px;",
                h2 { style: "margin: 0; font-size: 18px;", "Dependency Graph" }
                span {
                    style: "font-size: 12px; color: var(--fsn-color-text-muted);",
                    "Edges from fsn.requires labels"
                }
            }

            // Error
            if let Some(err) = error.read().as_deref() {
                div {
                    style: "color: var(--fsn-color-error); font-size: 13px; margin-bottom: 12px;",
                    "{err}"
                }
            }

            if list.is_empty() {
                div {
                    style: "text-align: center; color: var(--fsn-color-text-muted); padding: 48px;",
                    "No containers found."
                }
            } else {
                // SVG canvas
                div {
                    style: "overflow: auto;",
                    dangerous_inner_html: "{build_svg(&nodes, &edges, svg_w, svg_h)}"
                }

                // Legend
                div {
                    style: "display: flex; gap: 16px; margin-top: 12px; font-size: 12px; color: var(--fsn-color-text-muted);",
                    LegendDot { color: "#22c55e", label: "Healthy" }
                    LegendDot { color: "#f59e0b", label: "Starting" }
                    LegendDot { color: "#ef4444", label: "Unhealthy" }
                    LegendDot { color: "#6b7280", label: "Unknown" }
                }
            }
        }
    }
}

#[component]
fn LegendDot(color: &'static str, label: &'static str) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 4px;",
            div { style: "width: 10px; height: 10px; border-radius: 50%; background: {color};" }
            span { "{label}" }
        }
    }
}

// ── Graph builder ─────────────────────────────────────────────────────────────

/// Build positioned nodes and (from, to) name edges from a container list.
fn build_graph(containers: &[ContainerInfo]) -> (Vec<GraphNode>, Vec<(String, String)>) {
    let mut nodes: Vec<GraphNode> = containers
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let col = i % COLS;
            let row = i / COLS;
            let x = H_GAP + (col as f64) * (NODE_W + H_GAP);
            let y = V_GAP + (row as f64) * (NODE_H + V_GAP);
            let health = match c.health {
                fsn_container::HealthStatus::Healthy   => "ok",
                fsn_container::HealthStatus::Starting  => "warn",
                fsn_container::HealthStatus::Unhealthy => "err",
                fsn_container::HealthStatus::None      => "unknown",
            };
            GraphNode { name: c.name.clone(), health: health.to_string(), x, y }
        })
        .collect();

    // Position map: name → (cx, cy)
    let pos: HashMap<String, (f64, f64)> = nodes
        .iter()
        .map(|n| (n.name.clone(), (n.x + NODE_W / 2.0, n.y + NODE_H / 2.0)))
        .collect();

    // Edges from fsn.requires label
    let mut edges: Vec<(String, String)> = Vec::new();
    for c in containers {
        if let Some(req) = c.labels.get("fsn.requires") {
            for dep in req.split(',').map(str::trim).filter(|s| !s.is_empty()) {
                if pos.contains_key(dep) {
                    edges.push((c.name.clone(), dep.to_string()));
                }
            }
        }
    }

    // Sort nodes alphabetically for a stable layout
    nodes.sort_by(|a, b| a.name.cmp(&b.name));
    // Re-apply positions after sort
    for (i, node) in nodes.iter_mut().enumerate() {
        let col = i % COLS;
        let row = i / COLS;
        node.x = H_GAP + (col as f64) * (NODE_W + H_GAP);
        node.y = V_GAP + (row as f64) * (NODE_H + V_GAP);
    }

    (nodes, edges)
}

// ── SVG renderer ──────────────────────────────────────────────────────────────

fn health_color(h: &str) -> &'static str {
    match h {
        "ok"      => "#22c55e",
        "warn"    => "#f59e0b",
        "err"     => "#ef4444",
        _         => "#6b7280",
    }
}

fn build_svg(nodes: &[GraphNode], edges: &[(String, String)], w: f64, h: f64) -> String {
    let pos: HashMap<&str, (f64, f64)> = nodes
        .iter()
        .map(|n| (n.name.as_str(), (n.x + NODE_W / 2.0, n.y + NODE_H / 2.0)))
        .collect();

    let mut svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" style="display:block;">"##,
    );

    // Arrow marker
    svg.push_str(concat!(
        r#"<defs><marker id="arr" markerWidth="8" markerHeight="8" refX="7" refY="3" orient="auto">"#,
        r##"<path d="M0,0 L0,6 L8,3 z" fill="#6b7280"/></marker></defs>"##,
    ));

    // Edges
    for (from, to) in edges {
        if let (Some(&(fx, fy)), Some(&(tx, ty))) = (pos.get(from.as_str()), pos.get(to.as_str())) {
            svg.push_str(&format!(
                r##"<line x1="{fx:.0}" y1="{fy:.0}" x2="{tx:.0}" y2="{ty:.0}" stroke="#6b7280" stroke-width="1.5" marker-end="url(#arr)"/>"##,
            ));
        }
    }

    // Nodes
    for node in nodes {
        let x  = node.x;
        let y  = node.y;
        let cx = x + NODE_W / 2.0;
        let cy = y + NODE_H / 2.0;
        let dot_cx = x + 14.0;
        let text_y = cy + 4.5;
        let color = health_color(&node.health);
        let label: String = if node.name.len() > 18 {
            format!("{}…", &node.name[..17])
        } else {
            node.name.clone()
        };

        svg.push_str(&format!(
            r##"<rect x="{x:.0}" y="{y:.0}" width="{NODE_W}" height="{NODE_H}" rx="6" fill="#1e293b" stroke="{color}" stroke-width="2"/>"##
        ));
        svg.push_str(&format!(
            r##"<circle cx="{dot_cx:.0}" cy="{cy:.0}" r="5" fill="{color}"/>"##
        ));
        svg.push_str(&format!(
            r##"<text x="{cx:.0}" y="{text_y:.0}" text-anchor="middle" fill="#e2e8f0" font-size="12" font-family="monospace">{label}</text>"##
        ));
    }

    svg.push_str("</svg>");
    svg
}