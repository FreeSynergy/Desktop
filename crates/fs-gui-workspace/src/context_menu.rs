/// Generic right-click context menu — data types only.
///
/// Rendering is handled by `DesktopShell::view()` via iced widgets.

#[derive(Clone, PartialEq, Debug)]
pub struct ContextMenuItem {
    pub id: &'static str,
    pub label: String,
    pub icon: Option<&'static str>,
    pub danger: bool,
}

impl ContextMenuItem {
    pub fn new(id: &'static str, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            icon: None,
            danger: false,
        }
    }

    #[must_use]
    pub fn with_icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }

    #[must_use]
    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct ContextMenuState {
    pub open: bool,
    pub x: f64,
    pub y: f64,
    pub items: Vec<ContextMenuItem>,
}

impl ContextMenuState {
    #[must_use]
    pub fn open_at(x: f64, y: f64, items: Vec<ContextMenuItem>) -> Self {
        Self {
            open: true,
            x,
            y,
            items,
        }
    }
}
