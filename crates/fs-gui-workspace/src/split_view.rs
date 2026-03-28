//! `SplitView` — state for a resizable horizontal master/detail split panel.
//!
//! Rendering is handled by `DesktopShell::view()` via iced widgets.

//! State of the split panel.
#[derive(Clone, PartialEq, Debug, Default)]
pub enum SplitState {
    /// Master collapsed — only detail visible.
    Collapsed,
    /// Both panes visible, resizable via drag handle.
    #[default]
    Half,
    /// Master full-width — detail hidden.
    FullRight,
}

impl SplitState {
    /// Cycle: Collapsed → Half → `FullRight` → Collapsed.
    #[must_use]
    pub fn next(&self) -> Self {
        match self {
            Self::Collapsed => Self::Half,
            Self::Half => Self::FullRight,
            Self::FullRight => Self::Collapsed,
        }
    }
}
