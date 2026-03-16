/// Desktop widgets — standalone UI cards that can be placed on the desktop
/// or embedded in any layout.
///
/// - `ClockWidget` — analog/digital clock with second-accurate updates.
/// - `SystemInfoWidget` — hostname, uptime, memory and disk at a glance.
/// - `QuickNotesWidget` — simple in-memory textarea for quick notes.
/// - `PlaceholderWidget` — "coming soon" card for unimplemented widgets.
/// - `WidgetKind` — enum of all supported widget types.
/// - `WidgetSlot` — a widget instance in a layout (id + kind).
/// - `render_widget` — dispatches a `WidgetKind` to its component.
use chrono::Local;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// ── WidgetKind ─────────────────────────────────────────────────────────────

/// All widget types that can appear on the home layer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WidgetKind {
    Clock,
    SystemInfo,
    Messages,
    MyTasks,
    QuickNotes,
    Weather,
}

impl WidgetKind {
    /// Human-readable label shown in the widget picker.
    pub fn label(&self) -> &'static str {
        match self {
            WidgetKind::Clock      => "Clock",
            WidgetKind::SystemInfo => "System Info",
            WidgetKind::Messages   => "Messages",
            WidgetKind::MyTasks    => "My Tasks",
            WidgetKind::QuickNotes => "Quick Notes",
            WidgetKind::Weather    => "Weather",
        }
    }

    /// Emoji icon used in the widget picker panel.
    pub fn icon(&self) -> &'static str {
        match self {
            WidgetKind::Clock      => "🕐",
            WidgetKind::SystemInfo => "🖥",
            WidgetKind::Messages   => "📬",
            WidgetKind::MyTasks    => "✅",
            WidgetKind::QuickNotes => "📝",
            WidgetKind::Weather    => "🌤",
        }
    }

    /// All available widget kinds, in picker order.
    pub fn all() -> Vec<WidgetKind> {
        vec![
            WidgetKind::Clock,
            WidgetKind::SystemInfo,
            WidgetKind::Messages,
            WidgetKind::MyTasks,
            WidgetKind::QuickNotes,
            WidgetKind::Weather,
        ]
    }

    /// TOML key used for persistence.
    pub fn as_str(&self) -> &'static str {
        match self {
            WidgetKind::Clock      => "Clock",
            WidgetKind::SystemInfo => "SystemInfo",
            WidgetKind::Messages   => "Messages",
            WidgetKind::MyTasks    => "MyTasks",
            WidgetKind::QuickNotes => "QuickNotes",
            WidgetKind::Weather    => "Weather",
        }
    }

    /// Parse from TOML key string.
    pub fn from_str(s: &str) -> Option<WidgetKind> {
        match s {
            "Clock"      => Some(WidgetKind::Clock),
            "SystemInfo" => Some(WidgetKind::SystemInfo),
            "Messages"   => Some(WidgetKind::Messages),
            "MyTasks"    => Some(WidgetKind::MyTasks),
            "QuickNotes" => Some(WidgetKind::QuickNotes),
            "Weather"    => Some(WidgetKind::Weather),
            _            => None,
        }
    }
}

// ── WidgetSlot ─────────────────────────────────────────────────────────────

/// A widget instance placed in the home layer layout.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetSlot {
    /// Unique ID within this layout session.
    pub id: u32,
    /// Which widget to render.
    pub kind: WidgetKind,
}

// ── Layout persistence ─────────────────────────────────────────────────────

/// Path to the widget layout config file.
fn layout_path() -> std::path::PathBuf {
    let base = std::env::var("HOME").unwrap_or_else(|_| "/root".into());
    std::path::PathBuf::from(base).join(".config/fsn/widget_layout.toml")
}

/// Load widget layout from `~/.config/fsn/widget_layout.toml`.
/// Falls back to `[Clock, SystemInfo]` on any error.
pub fn load_widget_layout() -> Vec<WidgetSlot> {
    let path = layout_path();
    if let Ok(content) = std::fs::read_to_string(&path) {
        if let Ok(table) = content.parse::<toml::Table>() {
            if let Some(toml::Value::Array(arr)) = table.get("widgets") {
                let slots: Vec<WidgetSlot> = arr
                    .iter()
                    .enumerate()
                    .filter_map(|(i, v)| {
                        v.as_str()
                            .and_then(WidgetKind::from_str)
                            .map(|kind| WidgetSlot { id: i as u32, kind })
                    })
                    .collect();
                if !slots.is_empty() {
                    return slots;
                }
            }
        }
    }
    // Default layout
    vec![
        WidgetSlot { id: 0, kind: WidgetKind::Clock },
        WidgetSlot { id: 1, kind: WidgetKind::SystemInfo },
    ]
}

/// Persist the current widget layout to `~/.config/fsn/widget_layout.toml`.
pub fn save_widget_layout(slots: &[WidgetSlot]) {
    let path = layout_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let list: Vec<toml::Value> = slots
        .iter()
        .map(|s| toml::Value::String(s.kind.as_str().to_string()))
        .collect();
    let mut table = toml::Table::new();
    table.insert("widgets".to_string(), toml::Value::Array(list));
    let content = toml::to_string(&table).unwrap_or_default();
    let _ = std::fs::write(&path, content);
}

// ── render_widget dispatch ─────────────────────────────────────────────────

/// Dispatches a `WidgetKind` to its concrete Dioxus component.
pub fn render_widget(kind: &WidgetKind) -> Element {
    match kind {
        WidgetKind::Clock      => rsx! { ClockWidget {} },
        WidgetKind::SystemInfo => rsx! { SystemInfoWidget {} },
        WidgetKind::QuickNotes => rsx! { QuickNotesWidget {} },
        other => rsx! { PlaceholderWidget { kind: other.clone() } },
    }
}

// ── ClockWidget ───────────────────────────────────────────────────────────────

/// A clock widget that updates every second.
///
/// Displays the current time (HH:MM:SS) and date (Weekday, DD Month YYYY).
#[component]
pub fn ClockWidget() -> Element {
    let mut time_str = use_signal(|| Local::now().format("%H:%M:%S").to_string());
    let mut date_str = use_signal(|| Local::now().format("%A, %d %B %Y").to_string());

    use_future(move || async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            time_str.set(Local::now().format("%H:%M:%S").to_string());
            date_str.set(Local::now().format("%A, %d %B %Y").to_string());
        }
    });

    rsx! {
        div {
            class: "fsn-widget fsn-widget--clock",
            style: "background: var(--fsn-color-bg-surface); \
                    border: 1px solid var(--fsn-color-border-default); \
                    border-radius: var(--fsn-radius-lg); \
                    padding: 20px 24px; \
                    display: flex; flex-direction: column; align-items: center; gap: 6px; \
                    min-width: 200px;",

            span {
                style: "font-size: 36px; font-weight: 700; letter-spacing: 2px; \
                        font-variant-numeric: tabular-nums; \
                        color: var(--fsn-color-primary);",
                "{time_str}"
            }
            span {
                style: "font-size: 13px; color: var(--fsn-color-text-muted);",
                "{date_str}"
            }
        }
    }
}

// ── SystemInfoWidget ──────────────────────────────────────────────────────────

/// Snapshot of system information.
#[derive(Clone, Default)]
struct SysInfo {
    hostname: String,
    uptime:   String,
    mem_used: String,
    mem_total: String,
    disk_used: String,
    disk_total: String,
}

/// A system-info widget showing hostname, uptime, memory and disk.
///
/// Reads `/etc/hostname`, `/proc/uptime`, `/proc/meminfo` and uses `df -h /`
/// for disk information. Refreshes every 10 seconds.
#[component]
pub fn SystemInfoWidget() -> Element {
    let mut info = use_signal(SysInfo::default);

    use_future(move || async move {
        loop {
            let snapshot = tokio::task::spawn_blocking(read_sys_info).await.unwrap_or_default();
            info.set(snapshot);
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });

    let i = info.read();
    rsx! {
        div {
            class: "fsn-widget fsn-widget--sysinfo",
            style: "background: var(--fsn-color-bg-surface); \
                    border: 1px solid var(--fsn-color-border-default); \
                    border-radius: var(--fsn-radius-lg); \
                    padding: 16px 20px; \
                    display: flex; flex-direction: column; gap: 10px; \
                    min-width: 240px;",

            // Widget title
            div {
                style: "font-size: 12px; font-weight: 600; text-transform: uppercase; \
                        letter-spacing: 0.08em; color: var(--fsn-color-text-muted); \
                        border-bottom: 1px solid var(--fsn-color-border-default); \
                        padding-bottom: 8px;",
                "System Info"
            }

            SysRow { icon: "🖥",  label: "Host",   value: i.hostname.clone() }
            SysRow { icon: "⏱",  label: "Uptime", value: i.uptime.clone() }
            SysRow { icon: "🧠",  label: "Memory", value: format!("{} / {}", i.mem_used, i.mem_total) }
            SysRow { icon: "💾",  label: "Disk",   value: format!("{} / {}", i.disk_used, i.disk_total) }
        }
    }
}

// ── SysRow ────────────────────────────────────────────────────────────────────

#[component]
fn SysRow(icon: String, label: String, value: String) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 10px; font-size: 13px;",
            span { style: "font-size: 16px; min-width: 20px;", "{icon}" }
            span {
                style: "color: var(--fsn-color-text-muted); min-width: 56px;",
                "{label}"
            }
            span {
                style: "color: var(--fsn-color-text-primary); font-weight: 500; \
                        overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                if value.is_empty() { "—" } else { "{value}" }
            }
        }
    }
}

// ── QuickNotesWidget ──────────────────────────────────────────────────────────

/// A simple in-memory text area for quick notes.
///
/// No persistence — notes are cleared on restart. Use the clipboard.
#[component]
pub fn QuickNotesWidget() -> Element {
    let mut text = use_signal(|| String::new());

    rsx! {
        div {
            class: "fsn-widget fsn-widget--notes",
            style: "background: var(--fsn-color-bg-surface); \
                    border: 1px solid var(--fsn-color-border-default); \
                    border-radius: var(--fsn-radius-lg); \
                    padding: 16px 20px; \
                    display: flex; flex-direction: column; gap: 10px; \
                    min-width: 240px; width: 280px;",

            div {
                style: "font-size: 12px; font-weight: 600; text-transform: uppercase; \
                        letter-spacing: 0.08em; color: var(--fsn-color-text-muted); \
                        border-bottom: 1px solid var(--fsn-color-border-default); \
                        padding-bottom: 8px;",
                "Quick Notes"
            }

            textarea {
                style: "background: var(--fsn-color-bg-base, #0f172a); \
                        color: var(--fsn-color-text-primary); \
                        border: 1px solid var(--fsn-color-border-default); \
                        border-radius: 6px; \
                        padding: 8px 10px; \
                        font-size: 13px; font-family: inherit; \
                        resize: none; \
                        height: 120px; width: 100%; \
                        outline: none; box-sizing: border-box;",
                placeholder: "Type your notes here…",
                value: "{text}",
                oninput: move |e| text.set(e.value()),
            }
        }
    }
}

// ── PlaceholderWidget ─────────────────────────────────────────────────────────

/// Shows a "coming soon" card for widget kinds not yet implemented.
#[component]
pub fn PlaceholderWidget(kind: WidgetKind) -> Element {
    let label = kind.label();
    let icon  = kind.icon();

    rsx! {
        div {
            class: "fsn-widget fsn-widget--placeholder",
            style: "background: var(--fsn-color-bg-surface); \
                    border: 1px solid var(--fsn-color-border-default); \
                    border-radius: var(--fsn-radius-lg); \
                    padding: 20px 24px; \
                    display: flex; flex-direction: column; align-items: center; \
                    justify-content: center; gap: 8px; \
                    min-width: 180px; opacity: 0.7;",

            span { style: "font-size: 28px;", "{icon}" }
            span {
                style: "font-size: 13px; font-weight: 600; \
                        color: var(--fsn-color-text-primary);",
                "{label}"
            }
            span {
                style: "font-size: 11px; color: var(--fsn-color-text-muted);",
                "coming soon"
            }
        }
    }
}

// ── system reads ─────────────────────────────────────────────────────────────

fn read_sys_info() -> SysInfo {
    SysInfo {
        hostname:   read_hostname(),
        uptime:     read_uptime(),
        mem_used:   read_mem_used(),
        mem_total:  read_mem_total(),
        disk_used:  read_disk_used(),
        disk_total: read_disk_total(),
    }
}

fn read_hostname() -> String {
    std::fs::read_to_string("/etc/hostname")
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn read_uptime() -> String {
    let raw = std::fs::read_to_string("/proc/uptime").unwrap_or_default();
    let secs: f64 = raw
        .split_whitespace()
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);
    let secs = secs as u64;
    let days  = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins  = (secs % 3600) / 60;
    if days > 0 {
        format!("{days}d {hours}h {mins}m")
    } else if hours > 0 {
        format!("{hours}h {mins}m")
    } else {
        format!("{mins}m")
    }
}

fn parse_meminfo_kb(key: &str) -> u64 {
    let raw = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
    raw.lines()
        .find(|l| l.starts_with(key))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

fn kb_to_display(kb: u64) -> String {
    if kb >= 1_048_576 {
        format!("{:.1}G", kb as f64 / 1_048_576.0)
    } else if kb >= 1024 {
        format!("{:.0}M", kb as f64 / 1024.0)
    } else {
        format!("{kb}K")
    }
}

fn read_mem_total() -> String {
    kb_to_display(parse_meminfo_kb("MemTotal:"))
}

fn read_mem_used() -> String {
    let total     = parse_meminfo_kb("MemTotal:");
    let available = parse_meminfo_kb("MemAvailable:");
    kb_to_display(total.saturating_sub(available))
}

fn read_disk_used() -> String {
    disk_stat(true)
}

fn read_disk_total() -> String {
    disk_stat(false)
}

/// Returns used or total disk space for `/` via `df`.
fn disk_stat(used: bool) -> String {
    let out = std::process::Command::new("df")
        .args(["--output=used,size", "-k", "/"])
        .output();
    let Ok(out) = out else { return "?".into() };
    let text = String::from_utf8_lossy(&out.stdout);
    // second line: "used size"
    let mut lines = text.lines();
    let _ = lines.next(); // header
    let data = lines.next().unwrap_or("");
    let mut parts = data.split_whitespace();
    let used_kb:  u64 = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
    let total_kb: u64 = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
    if used { kb_to_display(used_kb) } else { kb_to_display(total_kb) }
}
