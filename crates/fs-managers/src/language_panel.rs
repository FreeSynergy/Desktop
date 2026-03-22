/// Language Manager panel — shows active language, lists available, allows switching.
use dioxus::prelude::*;
use fs_i18n;
use fs_manager_language::LanguageManager;

use crate::picker_panel::{PickerItem, PickerPanel};

#[component]
pub fn LanguageManagerPanel() -> Element {
    let mgr   = LanguageManager::new();
    let items: Vec<PickerItem> = mgr.available()
        .into_iter()
        .map(|l| {
            let flag   = l.flag_svg().to_string();
            let locale = l.locale.clone();
            PickerItem::new(l.id, l.display_name)
                .with_icon_html(flag)
                .with_badge(locale)
        })
        .collect();
    let active_id = mgr.active().id;

    rsx! {
        PickerPanel {
            title: fs_i18n::t("managers.language.title").to_string(),
            description: fs_i18n::t("managers.language.description").to_string(),
            items,
            active_id,
            on_apply: move |id: String| {
                let _ = LanguageManager::new().set_active(&id);
            },
        }
    }
}
