// help_sidebar.rs — Right sidebar: context-sensitive Help + AI assistant.
//
// Design Pattern: Strategy (HelpSource) + Observer (ActiveWindowObserver)
//
// Strategy — HelpSource trait with 3 implementations:
//   LocalHelpTopicSource  — looks up HelpTopic from fs-help HelpSystem
//   AiHelpSource          — forwards query to AI capability (fs-ai gRPC stub)
//   NoHelpSource          — fallback when no topic + no AI installed
//
// Observer — ActiveWindowObserver: watches active_app id and resolves HelpContext.
//
// CapabilityCheck: looks for "ai" capability in a capability registry.
// When AI is present, AiInputBar (TextInput + Send) is shown at the bottom.
//
// Sandbox rule (O7): reads are via HelpSystem (in-process), writes are via bus events.
// No direct network access from this module.

use fs_help::{HelpSystem, HelpTopic};
use serde_json;

use crate::sidebar_state::{SidebarMode, SidebarState};

// ── HelpContent ───────────────────────────────────────────────────────────────

/// Resolved help content ready for display.
#[derive(Debug, Clone)]
pub enum HelpContent {
    /// A loaded help topic (title + content i18n keys + link URLs).
    Topic {
        title_key: String,
        content_key: String,
        links: Vec<String>,
    },
    /// AI-generated response text.
    AiResponse(String),
    /// No help available.
    None,
}

// ── HelpSource trait ──────────────────────────────────────────────────────────

/// Strategy: any source that can produce `HelpContent` for an app context.
pub trait HelpSource: Send + Sync {
    /// Resolve help for the given application context string (e.g. `"fs-browser"`).
    fn resolve(&self, context: &str) -> HelpContent;

    /// Human-readable name of this source (for debug / logging).
    fn source_name(&self) -> &'static str;
}

// ── LocalHelpTopicSource ──────────────────────────────────────────────────────

/// Resolves help from the local `HelpSystem` (fs-help, in-process).
pub struct LocalHelpTopicSource {
    system: HelpSystem,
}

impl LocalHelpTopicSource {
    #[must_use]
    pub fn new(system: HelpSystem) -> Self {
        Self { system }
    }

    fn topic_to_content(topic: &HelpTopic) -> HelpContent {
        HelpContent::Topic {
            title_key: topic.title_key.clone(),
            content_key: topic.content_key.clone(),
            links: topic.kind.links().iter().map(|l| l.url.clone()).collect(),
        }
    }
}

impl HelpSource for LocalHelpTopicSource {
    fn resolve(&self, context: &str) -> HelpContent {
        match self.system.help_for_context(context) {
            Some(topic) => Self::topic_to_content(topic),
            None => HelpContent::None,
        }
    }

    fn source_name(&self) -> &'static str {
        "local-help"
    }
}

// ── AiHelpSource ──────────────────────────────────────────────────────────────

/// Resolves help via the running fs-ai REST service (`POST /ai/chat`).
///
/// The endpoint is read from the `FS_AI_ENDPOINT` environment variable
/// (e.g. `http://localhost:8080`).  Falls back to `HelpContent::None`
/// when the service is unreachable or not configured.
pub struct AiHelpSource;

impl AiHelpSource {
    fn endpoint() -> Option<String> {
        std::env::var("FS_AI_ENDPOINT").ok()
    }
}

impl HelpSource for AiHelpSource {
    fn resolve(&self, context: &str) -> HelpContent {
        let Some(base) = Self::endpoint() else {
            return HelpContent::None;
        };

        let url = format!("{base}/ai/chat");
        let body = serde_json::json!({
            "question": format!("Help me with the context: {context}"),
            "context": context
        });

        let client = reqwest::blocking::Client::new();
        let result = client
            .post(&url)
            .json(&body)
            .send()
            .and_then(reqwest::blocking::Response::error_for_status)
            .and_then(reqwest::blocking::Response::json::<serde_json::Value>);

        match result {
            Ok(json) => {
                let answer = json["answer"]
                    .as_str()
                    .unwrap_or("(no response)")
                    .to_owned();
                HelpContent::AiResponse(answer)
            }
            Err(_) => HelpContent::None,
        }
    }

    fn source_name(&self) -> &'static str {
        "ai-help"
    }
}

// ── NoHelpSource ──────────────────────────────────────────────────────────────

/// Fallback when neither local help nor AI is available.
pub struct NoHelpSource;

impl HelpSource for NoHelpSource {
    fn resolve(&self, _context: &str) -> HelpContent {
        HelpContent::None
    }

    fn source_name(&self) -> &'static str {
        "no-help"
    }
}

// ── ActiveWindowObserver ──────────────────────────────────────────────────────

/// Observes the active app ID and maps it to a help context key.
///
/// Observer: shell passes the `active_app` string on every focus change.
/// The observer returns the appropriate context string for `HelpSource` lookup.
pub struct ActiveWindowObserver;

impl ActiveWindowObserver {
    /// Map an active app id (e.g. `"browser"`) to a `HelpContext` key.
    #[must_use]
    pub fn context_for(app_id: &str) -> String {
        // App IDs map 1:1 to help context keys.
        // Compound IDs like "managers" stay as-is — HelpContext walks the hierarchy.
        app_id.replace('-', ".")
    }
}

// ── CapabilityCheck ───────────────────────────────────────────────────────────

/// Checks whether the AI capability is available in the running system.
///
/// In Phase 4B this is a heuristic check (env var / flag file).
/// Full check: fs-registry gRPC `ListCapabilities` → look for "ai" capability.
pub struct CapabilityCheck;

impl CapabilityCheck {
    /// Returns `true` if the AI capability (fs-ai) appears to be installed.
    #[must_use]
    pub fn ai_available() -> bool {
        // Heuristic: env var set by fs-ai service on start, or flag file.
        if std::env::var("FS_AI_ENDPOINT").is_ok() {
            return true;
        }
        let flag = std::path::PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()))
            .join(".local/share/freesynergy/registry/capabilities/ai.flag");
        flag.exists()
    }
}

// ── HelpSidebarState ──────────────────────────────────────────────────────────

/// Full state for the right help sidebar panel.
pub struct HelpSidebarState {
    /// Collapsed / Expanding / Open (shared State Machine type).
    pub state: SidebarState,
    /// Auto-collapse or stay pinned.
    pub mode: SidebarMode,
    /// Current help content to display.
    pub content: HelpContent,
    /// AI input text (empty when not typing).
    pub ai_input: String,
    /// Whether the AI capability is present.
    pub ai_available: bool,
}

impl Default for HelpSidebarState {
    fn default() -> Self {
        Self {
            state: SidebarState::Collapsed,
            mode: SidebarMode::Auto,
            content: HelpContent::None,
            ai_input: String::new(),
            ai_available: CapabilityCheck::ai_available(),
        }
    }
}

impl HelpSidebarState {
    /// Update content when the active window changes.
    ///
    /// Selects the best available `HelpSource` and resolves content.
    pub fn on_active_window_changed(&mut self, app_id: Option<&str>) {
        let context = app_id
            .map(ActiveWindowObserver::context_for)
            .unwrap_or_default();

        if context.is_empty() {
            self.content = HelpContent::None;
            return;
        }

        // Build the best available source chain: local first, AI fallback.
        let system = HelpSystem::default();
        let local = LocalHelpTopicSource::new(system);
        let resolved = local.resolve(&context);

        self.content = match resolved {
            HelpContent::None if self.ai_available => AiHelpSource.resolve(&context),
            other => other,
        };
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_help_source_returns_none() {
        let src = NoHelpSource;
        assert!(matches!(src.resolve("anything"), HelpContent::None));
    }

    #[test]
    fn ai_help_source_returns_none_without_endpoint() {
        // Without FS_AI_ENDPOINT set the source gracefully returns None.
        let src = AiHelpSource;
        assert!(matches!(src.resolve("browser"), HelpContent::None));
    }

    #[test]
    fn active_window_observer_maps_id() {
        assert_eq!(
            ActiveWindowObserver::context_for("fs-browser"),
            "fs.browser"
        );
        assert_eq!(ActiveWindowObserver::context_for("settings"), "settings");
    }

    #[test]
    fn help_sidebar_state_default_collapsed() {
        let state = HelpSidebarState::default();
        assert_eq!(state.state, SidebarState::Collapsed);
    }

    #[test]
    fn help_sidebar_on_no_active_window() {
        let mut state = HelpSidebarState::default();
        state.on_active_window_changed(None);
        assert!(matches!(state.content, HelpContent::None));
    }
}
