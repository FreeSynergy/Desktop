//! `WebDesktop` — data types for the mobile-first web shell layout.
//!
//! Rendering is handled by iced in a future phase.

//! Visibility state for the bottom taskbar in the web layout.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum WebTaskbarState {
    /// Always visible at the bottom.
    #[default]
    Fixed,
    /// Slides up to reveal more content.
    SlideUp,
    /// Hidden — only drag handle is visible.
    Hidden,
}

impl WebTaskbarState {
    /// Cycle: Fixed → `SlideUp` → Hidden → Fixed.
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Fixed => Self::SlideUp,
            Self::SlideUp => Self::Hidden,
            Self::Hidden => Self::Fixed,
        }
    }
}

/// Stub type kept for backward-compatible imports.
pub struct WebDesktop;
