pub mod app;

pub use app::AiManagerApp;

pub fn register_i18n() {
    const EN: &str = include_str!("../assets/i18n/en.toml");
    const DE: &str = include_str!("../assets/i18n/de.toml");
    let _ = fs_i18n::add_toml_lang("en", EN);
    let _ = fs_i18n::add_toml_lang("de", DE);
}

// ── AiStatus ─────────────────────────────────────────────────────────────────

use fs_manager_ai::{AiEngine, LlmConfig, LlmEngine, LlmModel};

pub struct AiStatus;

impl AiStatus {
    fn engine() -> LlmEngine {
        LlmEngine::new(
            LlmConfig { model: LlmModel::Qwen3_4B, ..LlmConfig::default() },
            LlmEngine::default_binary(),
            LlmEngine::default_data_dir(),
        )
    }

    /// Returns `true` if the LLM engine binary is installed.
    pub fn is_installed() -> bool {
        Self::engine().is_installed()
    }

    /// Returns the OpenAI-compatible API base URL if the engine is running,
    /// e.g. `"http://127.0.0.1:1234/v1"`.
    pub fn api_url() -> Option<String> {
        match Self::engine().status() {
            fs_manager_ai::EngineStatus::Running { port } =>
                Some(format!("http://127.0.0.1:{port}/v1")),
            _ => None,
        }
    }
}

// ── Public shims ──────────────────────────────────────────────────────────────

pub fn is_ai_installed() -> bool       { AiStatus::is_installed() }
pub fn ai_api_url()      -> Option<String> { AiStatus::api_url() }
