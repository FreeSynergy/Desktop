// bot_management/mod.rs — Bot management tab: list, add, toggle, remove.

mod bot_row;
mod form;
mod model;

pub use model::{Bot, BotAction, BotTrigger};

use bot_row::BotRow;
use dioxus::prelude::*;
use form::{AddBotForm, AddBotFormView};
use fsn_i18n;
use model::BotsConfig;

// ── persistence helper ────────────────────────────────────────────────────────

pub(super) fn save_bots(bots: Signal<Vec<Bot>>, mut status_msg: Signal<Option<String>>) {
    match BotsConfig::save(&*bots.read()) {
        Ok(()) => status_msg.set(None),
        Err(e) => status_msg.set(Some(format!("Save error: {e}"))),
    }
}

// ── BotManagement ─────────────────────────────────────────────────────────────

/// Bot management tab — list, add, toggle, and remove bots.
#[component]
pub fn BotManagement() -> Element {
    let bots          = use_signal(BotsConfig::load);
    let mut show_add  = use_signal(|| false);
    let mut form      = use_signal(AddBotForm::default);
    let status_msg    = use_signal(|| Option::<String>::None);

    let showing_add   = *show_add.read();
    let is_empty      = bots.read().is_empty();
    let add_btn_label = if showing_add { fsn_i18n::t("actions.cancel") } else { fsn_i18n::t("conductor.bots.btn_add") };
    let save_err      = status_msg.read().clone();

    // Snapshot to avoid holding read guard across rsx
    let bot_list: Vec<Bot> = bots.read().clone();

    rsx! {
        div {
            class: "fsd-bots",
            style: "padding: 0;",

            // ── Header ───────────────────────────────────────────────────────
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;",
                div {
                    h3 { style: "margin: 0;", {fsn_i18n::t("conductor.section.bots")} }
                    p {
                        style: "margin: 4px 0 0; font-size: 13px; color: var(--fsn-color-text-muted);",
                        {fsn_i18n::t("conductor.bots.description")}
                    }
                }
                button {
                    style: "padding: 8px 16px; background: var(--fsn-color-primary); color: white; \
                            border: none; border-radius: var(--fsn-radius-md); cursor: pointer;",
                    onclick: move |_| {
                        show_add.set(!showing_add);
                        form.set(AddBotForm::default());
                    },
                    "{add_btn_label}"
                }
            }

            // ── Add-bot form ─────────────────────────────────────────────────
            if showing_add {
                AddBotFormView { form, bots, show_add, status_msg }
            }

            // ── Empty state ──────────────────────────────────────────────────
            if is_empty {
                div {
                    style: "text-align: center; padding: 40px; background: var(--fsn-color-bg-surface); \
                            border-radius: var(--fsn-radius-md); border: 1px dashed var(--fsn-color-border-default);",
                    p { style: "color: var(--fsn-color-text-muted); margin: 0;", {fsn_i18n::t("conductor.bots.empty")} }
                    p { style: "font-size: 12px; color: var(--fsn-color-text-muted); margin: 8px 0 0;",
                        {fsn_i18n::t("conductor.bots.empty_hint")}
                    }
                }
            }

            // ── Bot rows ─────────────────────────────────────────────────────
            for (idx, bot) in bot_list.into_iter().enumerate() {
                BotRow { key: "{idx}", idx, bot, bots, status_msg }
            }

            // ── Save error ───────────────────────────────────────────────────
            if let Some(msg) = save_err {
                p { style: "font-size: 12px; color: var(--fsn-color-error); margin-top: 8px;", "{msg}" }
            }
        }
    }
}
