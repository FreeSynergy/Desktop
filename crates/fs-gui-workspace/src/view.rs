// view.rs — ProgramViewProvider for the FreeSynergy Desktop shell.
//
// This is the ONLY file in this crate that adds ProgramViewProvider.
// The Desktop exposes only SettingsConfig — no Start (it is always running)
// and no SettingsContainer (it is not a container service).

use fs_render::{ProgramView, ProgramViewProvider};

use crate::shell::DesktopShell;

impl ProgramViewProvider for DesktopShell {
    fn available_views(&self) -> Vec<ProgramView> {
        vec![ProgramView::SettingsConfig]
    }
}
