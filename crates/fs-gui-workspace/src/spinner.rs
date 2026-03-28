//! Loading spinner stub.
//!
//! Spinner rendering is done via iced built-in widgets in `shell.rs`.

//! Size hint for a loading spinner.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SpinnerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Stub types kept for backward-compatible imports.
pub struct LoadingSpinner;
pub struct LoadingOverlay;
