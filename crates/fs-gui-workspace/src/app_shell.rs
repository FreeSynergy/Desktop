//! `AppMode` — how an fs-* app is rendered.
//!
//! This enum is kept for compatibility with code that distinguishes rendering
//! contexts. All rendering is now done via iced in `shell.rs`.

//! How an fs-* app is rendered.
#[derive(Clone, PartialEq, Debug, Default)]
pub enum AppMode {
    /// Embedded inside a window frame in the desktop shell.
    #[default]
    Window,
    /// Running as its own top-level OS window.
    Standalone,
    /// Running inside a terminal.
    Tui,
}

/// Global CSS constants kept for reference / future use in iced themes.
pub const GLOBAL_CSS: &str = "";
