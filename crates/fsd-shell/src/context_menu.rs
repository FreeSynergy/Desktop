/// Generic right-click context menu system.
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct ContextMenuItem {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: Option<&'static str>,
    pub danger: bool,
}

impl ContextMenuItem {
    pub fn new(id: &'static str, label: &'static str) -> Self {
        Self { id, label, icon: None, danger: false }
    }

    pub fn with_icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }

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
    pub fn open_at(x: f64, y: f64, items: Vec<ContextMenuItem>) -> Self {
        Self { open: true, x, y, items }
    }
}

#[component]
pub fn ContextMenu(
    state: ContextMenuState,
    on_action: EventHandler<String>,
    on_close: EventHandler<()>,
) -> Element {
    if !state.open {
        return rsx! {};
    }

    let x = state.x;
    let y = state.y;

    rsx! {
        div {
            style: "position: fixed; inset: 0; z-index: 899;",
            onclick: move |_| on_close.call(()),
        }
        div {
            style: "position: fixed; left: {x}px; top: {y}px; \
                    background: var(--fsn-color-bg-surface, #0f172a); \
                    border: 1px solid var(--fsn-color-border-default); \
                    border-radius: var(--fsn-radius-md); \
                    min-width: 180px; z-index: 900; padding: 4px 0; \
                    box-shadow: 0 8px 32px rgba(0,0,0,0.6);",
            for item in &state.items {
                ContextMenuRow { item: item.clone(), on_action, on_close }
            }
        }
    }
}

#[component]
fn ContextMenuRow(
    item: ContextMenuItem,
    on_action: EventHandler<String>,
    on_close: EventHandler<()>,
) -> Element {
    let color = if item.danger {
        "#ef4444"
    } else {
        "var(--fsn-color-text-primary, #e2e8f0)"
    };
    let id = item.id.to_string();

    rsx! {
        button {
            style: "display: flex; align-items: center; gap: 10px; width: 100%; \
                    padding: 7px 16px; background: none; border: none; cursor: pointer; \
                    font-size: 13px; text-align: left; color: {color}; font-family: inherit;",
            onclick: move |_| {
                on_action.call(id.clone());
                on_close.call(());
            },
            if let Some(icon) = item.icon {
                span { style: "min-width: 18px; display: flex; align-items: center;", dangerous_inner_html: icon }
            }
            span { "{item.label}" }
        }
    }
}
