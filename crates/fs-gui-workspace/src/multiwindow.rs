//! Multiwindow support — stub for iced migration.
//!
//! In the iced architecture, multiple windows are managed via `iced::window`
//! APIs inside `DesktopShell`. This module keeps the `MultiwindowHandle` type
//! for backward compatibility.

//! Handle for opening app windows.
//!
//! In the iced shell, window opening is triggered via `DesktopMessage::OpenApp(AppId)`.
//! This stub keeps the type name available for existing imports.
#[derive(Clone)]
pub struct MultiwindowHandle;

impl MultiwindowHandle {
    pub fn open_managers(&self) {}
    pub fn open_settings(&self) {}
    pub fn open_profile(&self) {}
    pub fn open_store(&self) {}
    pub fn open_builder(&self) {}
}

/// Returns a `MultiwindowHandle`.
#[must_use]
pub fn use_multiwindow() -> MultiwindowHandle {
    MultiwindowHandle
}
