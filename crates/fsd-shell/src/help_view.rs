/// Help View — context-sensitive help and keyboard shortcuts reference.
/// Also exports HelpSidebarPanel: the collapsible right-side help panel for the Desktop.
use dioxus::prelude::*;
use fsn_components::{FsnSidebar, FsnSidebarItem, FSN_SIDEBAR_CSS};
use fsd_settings::{ShortcutsConfig, register_actions, resolve_shortcut};
use serde_json;

#[derive(Clone, PartialEq, Debug)]
enum HelpSection {
    Topics,
    Shortcuts,
}

impl HelpSection {
    fn id(&self) -> &'static str {
        match self {
            Self::Topics    => "topics",
            Self::Shortcuts => "shortcuts",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Topics    => "Topics",
            Self::Shortcuts => "Shortcuts",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Self::Topics    => "📚",
            Self::Shortcuts => "⌨",
        }
    }

    fn from_id(id: &str) -> Option<Self> {
        match id {
            "topics"    => Some(Self::Topics),
            "shortcuts" => Some(Self::Shortcuts),
            _           => None,
        }
    }
}

#[derive(Clone, PartialEq)]
struct HelpTopic {
    id: &'static str,
    title: &'static str,
    summary: &'static str,
}

const TOPICS: &[HelpTopic] = &[
    HelpTopic { id: "getting-started", title: "Getting Started",    summary: "Learn how to set up your first FreeSynergy.Node deployment." },
    HelpTopic { id: "container-app",   title: "Container",      summary: "Manage services, bots, and containers from the Container App view." },
    HelpTopic { id: "store",           title: "Module Store",       summary: "Browse, install, and update service modules from the store." },
    HelpTopic { id: "studio",          title: "Studio",             summary: "Create custom modules, plugins, and language packs." },
    HelpTopic { id: "settings",        title: "Settings",           summary: "Configure appearance, language, service roles, and AI connections." },
    HelpTopic { id: "ai-assistant",    title: "AI Assistant",       summary: "Use your local Ollama instance as an integrated AI helper." },
    HelpTopic { id: "troubleshooting", title: "Troubleshooting",    summary: "Common issues and how to resolve them." },
];

const ALL_SECTIONS: &[HelpSection] = &[HelpSection::Topics, HelpSection::Shortcuts];

/// Root component for the Help view.
#[component]
pub fn HelpApp() -> Element {
    let mut active = use_signal(|| HelpSection::Topics);

    let sidebar_items: Vec<FsnSidebarItem> = ALL_SECTIONS.iter()
        .map(|s| FsnSidebarItem::new(s.id(), s.icon(), s.label()))
        .collect();

    rsx! {
        style { "{FSN_SIDEBAR_CSS}" }
        div {
            class: "fsd-help-view",
            style: "display: flex; flex-direction: column; height: 100%; \
                    background: var(--fsn-color-bg-base);",

            // App title bar
            div {
                style: "padding: 10px 16px; border-bottom: 1px solid var(--fsn-border); \
                        flex-shrink: 0; background: var(--fsn-bg-surface);",
                h2 {
                    style: "margin: 0; font-size: 16px; font-weight: 600; color: var(--fsn-text-primary);",
                    "Help & Documentation"
                }
            }

            // Sidebar + Content row
            div {
                style: "display: flex; flex: 1; overflow: hidden;",

                FsnSidebar {
                    items: sidebar_items,
                    active_id: active.read().id().to_string(),
                    on_select: move |id: String| {
                        if let Some(section) = HelpSection::from_id(&id) {
                            active.set(section);
                        }
                    },
                }

                div { style: "flex: 1; overflow: hidden;",
                    match *active.read() {
                        HelpSection::Topics    => rsx! { TopicsView {} },
                        HelpSection::Shortcuts => rsx! { ShortcutsReference {} },
                    }
                }
            }
        }
    }
}

#[component]
fn TopicsView() -> Element {
    let mut query = use_signal(String::new);

    let q = query.read().to_lowercase();
    let filtered: Vec<&HelpTopic> = TOPICS
        .iter()
        .filter(|t| q.is_empty() || t.title.to_lowercase().contains(&q) || t.summary.to_lowercase().contains(&q))
        .collect();

    rsx! {
        div { style: "display: flex; flex-direction: column; height: 100%;",
            div { style: "padding: 12px 24px;",
                input {
                    r#type: "text",
                    placeholder: "Search help topics…",
                    style: "width: 100%; max-width: 480px; padding: 8px 12px; border-radius: 6px; \
                            background: var(--fsn-color-bg-input, #0f172a); \
                            border: 1px solid var(--fsn-color-border-default, #334155); \
                            color: var(--fsn-color-text-primary, #e2e8f0); font-size: 14px; \
                            outline: none; box-sizing: border-box;",
                    oninput: move |evt| query.set(evt.value()),
                }
            }
            div {
                class: "fsn-scrollable", style: "flex: 1; overflow-y: auto; padding: 0 24px 16px;",
                if filtered.is_empty() {
                    p { style: "color: var(--fsn-color-text-muted); font-size: 14px;",
                        "No topics found."
                    }
                } else {
                    div { style: "display: flex; flex-direction: column; gap: 8px;",
                        for topic in filtered {
                            HelpTopicCard { topic: topic.clone() }
                        }
                    }
                }
            }
        }
    }
}

/// Read-only shortcuts reference — auto-generated from the action registry.
#[component]
fn ShortcutsReference() -> Element {
    let config = ShortcutsConfig::load();
    let actions = register_actions();

    let mut categories: Vec<&str> = actions.iter().map(|a| a.category).collect();
    categories.sort();
    categories.dedup();

    rsx! {
        div {
            class: "fsn-scrollable", style: "overflow-y: auto; padding: 16px 24px; height: 100%;",
            p {
                style: "font-size: 12px; color: var(--fsn-text-muted); margin-bottom: 16px;",
                "Shortcuts can be customized in Settings → Shortcuts."
            }
            for cat in &categories {
                {
                    let cat_actions: Vec<&fsd_settings::ActionDef> = actions.iter().filter(|a| a.category == *cat).collect();
                    rsx! {
                        div { style: "margin-bottom: 20px;",
                            div {
                                style: "font-size: 11px; font-weight: 600; text-transform: uppercase; \
                                        letter-spacing: 0.08em; color: var(--fsn-text-muted); \
                                        margin-bottom: 6px; padding-bottom: 4px; \
                                        border-bottom: 1px solid var(--fsn-border);",
                                "{cat}"
                            }
                            for action in cat_actions {
                                {
                                    let shortcut = resolve_shortcut(action, &config)
                                        .unwrap_or("—");
                                    rsx! {
                                        div {
                                            key: "{action.id}",
                                            style: "display: flex; align-items: center; justify-content: space-between; \
                                                    padding: 5px 0; font-size: 13px;",
                                            span { style: "color: var(--fsn-text-primary);", "{action.label}" }
                                            span {
                                                style: "font-family: var(--fsn-font-mono); font-size: 12px; \
                                                        color: var(--fsn-text-secondary); \
                                                        background: var(--fsn-bg-elevated); \
                                                        padding: 2px 8px; border-radius: var(--fsn-radius-sm); \
                                                        border: 1px solid var(--fsn-border);",
                                                "{shortcut}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn HelpTopicCard(topic: HelpTopic) -> Element {
    rsx! {
        div {
            class: "fsd-help-topic",
            style: "padding: 14px 16px; border-radius: 8px; \
                    background: var(--fsn-color-bg-surface, #1e293b); \
                    border: 1px solid var(--fsn-color-border-default, #334155); \
                    cursor: pointer;",
            h3 {
                style: "margin: 0 0 4px; font-size: 15px; \
                        color: var(--fsn-color-primary, #06b6d4);",
                "{topic.title}"
            }
            p {
                style: "margin: 0; font-size: 13px; \
                        color: var(--fsn-color-text-muted, #94a3b8);",
                "{topic.summary}"
            }
        }
    }
}

// ── HelpSidebarPanel ──────────────────────────────────────────────────────────
// Collapsible right-side panel embedded in the Desktop shell.
// Layout when open (280 px):
//   ┌──────────────────┐
//   │ Header: Help [×] │
//   ├──────────────────┤
//   │ [Topics][Kbd]    │  ← tab strip
//   ├──────────────────┤
//   │ topic list /     │
//   │ shortcuts        │  ← scrollable content
//   ├──────────────────┤  (only when AI is running)
//   │ AI Chat          │  ← flex: 1
//   │ [input ↵]        │
//   └──────────────────┘

#[derive(Clone, PartialEq, Debug)]
enum SidebarTab {
    Topics,
    Shortcuts,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ChatMsg {
    pub role:    &'static str,
    pub content: String,
}

/// Collapsed toggle button — a small tab on the right edge of the desktop.
#[component]
pub fn HelpSidebarToggle(on_open: EventHandler<()>) -> Element {
    rsx! {
        div {
            style: "position: absolute; right: 0; top: 50%; transform: translateY(-50%); \
                    z-index: 50;",
            button {
                onclick: move |_| on_open.call(()),
                style: "writing-mode: vertical-rl; text-orientation: mixed; \
                        background: var(--fsn-bg-elevated); \
                        border: 1px solid var(--fsn-border); \
                        border-right: none; \
                        border-radius: 8px 0 0 8px; \
                        padding: 12px 6px; \
                        color: var(--fsn-text-secondary); \
                        font-size: 11px; font-family: inherit; \
                        cursor: pointer; \
                        display: flex; align-items: center; gap: 6px;",
                "❓ Help"
            }
        }
    }
}

/// Collapsible right-side help panel.
/// When `open` is false the panel has zero width and is invisible.
/// When an AI engine is running the panel splits into help (top) + chat (bottom).
#[component]
pub fn HelpSidebarPanel(open: bool, on_close: EventHandler<()>) -> Element {
    let mut tab = use_signal(|| SidebarTab::Topics);

    // Check AI status once per render — lightweight PID-file check.
    let ai_url = fsd_ai::ai_api_url();
    let ai_active = ai_url.is_some();

    // Chat state
    let mut messages: Signal<Vec<ChatMsg>> = use_signal(Vec::new);
    let mut input   = use_signal(String::new);
    let mut thinking = use_signal(|| false);

    if !open {
        return rsx! {};
    }

    const PANEL_CSS: &str = r#"
.fsn-help-sidebar {
    width: 280px; flex-shrink: 0;
    display: flex; flex-direction: column;
    background: var(--fsn-bg-surface);
    border-left: 1px solid var(--fsn-border);
    overflow: hidden;
}
.fsn-help-sidebar__header {
    display: flex; align-items: center; justify-content: space-between;
    padding: 10px 14px; border-bottom: 1px solid var(--fsn-border);
    flex-shrink: 0; background: var(--fsn-bg-elevated);
}
.fsn-help-sidebar__tabs {
    display: flex; border-bottom: 1px solid var(--fsn-border); flex-shrink: 0;
}
.fsn-help-sidebar__tab {
    flex: 1; padding: 7px 0; text-align: center;
    font-size: 12px; cursor: pointer;
    background: none; border: none; border-bottom: 2px solid transparent;
    color: var(--fsn-text-muted); font-family: inherit;
    transition: color 120ms, border-color 120ms;
}
.fsn-help-sidebar__tab--active {
    color: var(--fsn-primary);
    border-bottom-color: var(--fsn-primary);
}
.fsn-help-sidebar__content { flex: 1; overflow-y: auto; min-height: 0; }
.fsn-help-sidebar__ai {
    flex-shrink: 0; border-top: 1px solid var(--fsn-border);
    display: flex; flex-direction: column;
    height: 260px; /* fixed chat height */
}
.fsn-help-sidebar__ai-title {
    padding: 6px 12px; font-size: 11px; font-weight: 600;
    text-transform: uppercase; letter-spacing: 0.08em;
    color: var(--fsn-accent); border-bottom: 1px solid var(--fsn-border);
    flex-shrink: 0;
}
.fsn-help-sidebar__chat-msgs {
    flex: 1; overflow-y: auto; padding: 8px 12px; display: flex;
    flex-direction: column; gap: 6px; min-height: 0;
}
.fsn-help-sidebar__chat-input-row {
    display: flex; gap: 6px; padding: 8px 10px;
    border-top: 1px solid var(--fsn-border); flex-shrink: 0;
}
.fsn-help-sidebar__chat-input {
    flex: 1; padding: 7px 10px; border-radius: 6px;
    background: var(--fsn-bg-input); border: 1px solid var(--fsn-border);
    color: var(--fsn-text-primary); font-size: 13px; font-family: inherit;
    outline: none; resize: none; height: 34px; line-height: 1.4;
}
.fsn-help-sidebar__chat-send {
    padding: 0 12px; border-radius: 6px;
    background: var(--fsn-primary); border: none;
    color: #fff; font-size: 13px; cursor: pointer; flex-shrink: 0;
}
.fsn-help-sidebar__chat-send:disabled { opacity: 0.4; cursor: not-allowed; }
"#;

    rsx! {
        style { "{PANEL_CSS}" }
        div { class: "fsn-help-sidebar",

            // ── Header ──────────────────────────────────────────────────────
            div { class: "fsn-help-sidebar__header",
                span {
                    style: "font-size: 14px; font-weight: 600; color: var(--fsn-text-primary);",
                    "Help"
                }
                if ai_active {
                    span {
                        style: "font-size: 10px; background: var(--fsn-success-bg); \
                                color: var(--fsn-success); border-radius: 4px; padding: 2px 6px; \
                                border: 1px solid var(--fsn-success);",
                        "AI"
                    }
                }
                button {
                    onclick: move |_| on_close.call(()),
                    style: "background: none; border: none; cursor: pointer; padding: 4px; \
                            color: var(--fsn-text-muted); font-size: 16px; line-height: 1;",
                    "×"
                }
            }

            // ── Tab strip ───────────────────────────────────────────────────
            div { class: "fsn-help-sidebar__tabs",
                button {
                    class: if *tab.read() == SidebarTab::Topics {
                        "fsn-help-sidebar__tab fsn-help-sidebar__tab--active"
                    } else { "fsn-help-sidebar__tab" },
                    onclick: move |_| tab.set(SidebarTab::Topics),
                    "📚 Topics"
                }
                button {
                    class: if *tab.read() == SidebarTab::Shortcuts {
                        "fsn-help-sidebar__tab fsn-help-sidebar__tab--active"
                    } else { "fsn-help-sidebar__tab" },
                    onclick: move |_| tab.set(SidebarTab::Shortcuts),
                    "⌨ Shortcuts"
                }
            }

            // ── Content ─────────────────────────────────────────────────────
            div { class: "fsn-help-sidebar__content fsn-scrollable",
                match *tab.read() {
                    SidebarTab::Topics    => rsx! { SidebarTopicsView {} },
                    SidebarTab::Shortcuts => rsx! { SidebarShortcutsView {} },
                }
            }

            // ── AI Chat (only when AI engine is running) ─────────────────────
            if ai_active {
                div { class: "fsn-help-sidebar__ai",
                    div { class: "fsn-help-sidebar__ai-title", "AI Assistant" }

                    // Message list
                    div { class: "fsn-help-sidebar__chat-msgs fsn-scrollable",
                        if messages.read().is_empty() {
                            p {
                                style: "color: var(--fsn-text-muted); font-size: 12px; \
                                        text-align: center; margin: 12px 0;",
                                "Ask me anything about FreeSynergy…"
                            }
                        }
                        for msg in messages.read().iter() {
                            {
                                let is_user = msg.role == "user";
                                let (bg, align, color) = if is_user {
                                    ("var(--fsn-primary)", "flex-end", "#fff")
                                } else {
                                    ("var(--fsn-bg-elevated)", "flex-start", "var(--fsn-text-primary)")
                                };
                                rsx! {
                                    div {
                                        style: "display: flex; justify-content: {align};",
                                        div {
                                            style: "max-width: 90%; padding: 6px 10px; border-radius: 8px; \
                                                    background: {bg}; color: {color}; \
                                                    font-size: 12px; line-height: 1.5; white-space: pre-wrap;",
                                            "{msg.content}"
                                        }
                                    }
                                }
                            }
                        }
                        if *thinking.read() {
                            div {
                                style: "display: flex; align-items: center; gap: 4px; \
                                        color: var(--fsn-text-muted); font-size: 11px;",
                                "AI is thinking…"
                            }
                        }
                    }

                    // Input row
                    div { class: "fsn-help-sidebar__chat-input-row",
                        input {
                            r#type: "text",
                            class: "fsn-help-sidebar__chat-input",
                            placeholder: "Ask a question…",
                            value: "{input.read()}",
                            oninput: move |e| input.set(e.value()),
                            onkeydown: {
                                let api = ai_url.clone();
                                move |e: KeyboardEvent| {
                                    if e.key() == Key::Enter && !input.read().is_empty() && !*thinking.read() {
                                        let text = input.read().clone();
                                        input.set(String::new());
                                        thinking.set(true);
                                        messages.write().push(ChatMsg { role: "user", content: text.clone() });
                                        let url = api.clone().unwrap_or_default();
                                        let msgs_clone = messages.read().clone();
                                        spawn(async move {
                                            let reply = chat_request(&url, &msgs_clone).await;
                                            messages.write().push(ChatMsg { role: "assistant", content: reply });
                                            thinking.set(false);
                                        });
                                    }
                                }
                            },
                        }
                        button {
                            class: "fsn-help-sidebar__chat-send",
                            disabled: *thinking.read() || input.read().is_empty(),
                            onclick: {
                                let api = ai_url.clone();
                                move |_| {
                                    if input.read().is_empty() || *thinking.read() { return; }
                                    let text = input.read().clone();
                                    input.set(String::new());
                                    thinking.set(true);
                                    messages.write().push(ChatMsg { role: "user", content: text.clone() });
                                    let url = api.clone().unwrap_or_default();
                                    let msgs_clone = messages.read().clone();
                                    spawn(async move {
                                        let reply = chat_request(&url, &msgs_clone).await;
                                        messages.write().push(ChatMsg { role: "assistant", content: reply });
                                        thinking.set(false);
                                    });
                                }
                            },
                            "↵"
                        }
                    }
                }
            }
        }
    }
}

// ── Sidebar-specific compact views ───────────────────────────────────────────

/// Compact topic list for the sidebar (no search box to save space).
#[component]
fn SidebarTopicsView() -> Element {
    rsx! {
        div { style: "padding: 8px;",
            for topic in TOPICS {
                div {
                    style: "padding: 8px 10px; border-radius: 6px; margin-bottom: 4px; \
                            cursor: pointer; \
                            border: 1px solid var(--fsn-border);",
                    p {
                        style: "margin: 0 0 2px; font-size: 13px; font-weight: 500; \
                                color: var(--fsn-primary);",
                        "{topic.title}"
                    }
                    p {
                        style: "margin: 0; font-size: 11px; color: var(--fsn-text-muted); \
                                line-height: 1.4;",
                        "{topic.summary}"
                    }
                }
            }
        }
    }
}

/// Compact shortcuts list for the sidebar.
#[component]
fn SidebarShortcutsView() -> Element {
    let config  = ShortcutsConfig::load();
    let actions = register_actions();

    rsx! {
        div { style: "padding: 8px 12px;",
            for action in &actions {
                {
                    let shortcut = resolve_shortcut(action, &config).unwrap_or("—");
                    rsx! {
                        div {
                            key: "{action.id}",
                            style: "display: flex; align-items: center; justify-content: space-between; \
                                    padding: 4px 0; font-size: 12px; \
                                    border-bottom: 1px solid var(--fsn-border);",
                            span { style: "color: var(--fsn-text-primary);", "{action.label}" }
                            span {
                                style: "font-family: var(--fsn-font-mono); font-size: 11px; \
                                        color: var(--fsn-text-secondary); \
                                        background: var(--fsn-bg-elevated); \
                                        padding: 1px 6px; border-radius: 3px; \
                                        border: 1px solid var(--fsn-border);",
                                "{shortcut}"
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── AI chat HTTP helper ───────────────────────────────────────────────────────

/// Sends all messages to the OpenAI-compatible API and returns the assistant reply.
/// Errors are returned as a user-visible error string instead of panicking.
async fn chat_request(api_base: &str, messages: &[ChatMsg]) -> String {
    let url = format!("{api_base}/chat/completions");

    let body = serde_json::json!({
        "model": "default",
        "messages": messages.iter().map(|m| serde_json::json!({
            "role": m.role,
            "content": m.content,
        })).collect::<Vec<_>>(),
        "max_tokens": 512,
        "temperature": 0.4,
    });

    let result = reqwest::Client::new()
        .post(&url)
        .json(&body)
        .send()
        .await;

    match result {
        Err(e) => format!("Request failed: {e}"),
        Ok(resp) => {
            match resp.json::<serde_json::Value>().await {
                Err(e) => format!("Parse error: {e}"),
                Ok(json) => {
                    json["choices"][0]["message"]["content"]
                        .as_str()
                        .unwrap_or("(no response)")
                        .to_string()
                }
            }
        }
    }
}
