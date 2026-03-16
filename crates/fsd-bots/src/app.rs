/// Bot Manager — usage interface for messaging bots.
use dioxus::prelude::*;
use fsn_components::SidebarNavBtn;

use crate::broadcast_view::BroadcastView;
use crate::gatekeeper_view::GatekeeperView;
use crate::model::{BotKind, MessagingBot, MessagingBotsConfig};

#[component]
pub fn BotManagerApp() -> Element {
    let mut bots = use_signal(MessagingBotsConfig::load);
    let mut selected_idx: Signal<Option<usize>> = use_signal(|| Some(0));

    let bot_list = bots.read().clone();
    let sel_idx = *selected_idx.read();
    let selected = sel_idx.and_then(|i| bot_list.get(i).cloned());

    rsx! {
        div {
            style: "display: flex; height: 100%; width: 100%; overflow: hidden; \
                    background: var(--fsn-color-bg-base);",

            nav {
                style: "width: 200px; flex-shrink: 0; overflow-y: auto; \
                        background: var(--fsn-color-bg-surface, #0f172a); \
                        border-right: 1px solid var(--fsn-color-border-default, #334155); \
                        padding: 12px 8px;",

                div {
                    style: "margin: 0 0 12px 8px; font-size: 11px; font-weight: 600; \
                            text-transform: uppercase; letter-spacing: 0.08em; \
                            color: var(--fsn-color-text-muted, #64748b);",
                    "Bot Manager"
                }

                for (idx, bot) in bot_list.iter().enumerate() {
                    SidebarNavBtn {
                        key: "{bot.id}",
                        label: bot.name.clone(),
                        icon: bot.kind.icon().to_string(),
                        is_active: sel_idx == Some(idx),
                        left_border: true,
                        on_click: move |_| selected_idx.set(Some(idx)),
                    }
                }
            }

            div {
                style: "flex: 1; overflow: auto; padding: 20px;",

                match selected {
                    None => rsx! {
                        div {
                            style: "display: flex; align-items: center; justify-content: center; \
                                    height: 200px; color: var(--fsn-color-text-muted); font-size: 13px;",
                            "Select a bot from the list"
                        }
                    },
                    Some(bot) => rsx! {
                        BotDetail {
                            bot,
                            on_update: move |updated: MessagingBot| {
                                if let Some(i) = sel_idx {
                                    bots.write()[i] = updated;
                                    let _ = MessagingBotsConfig::save(&*bots.read());
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn BotDetail(bot: MessagingBot, on_update: EventHandler<MessagingBot>) -> Element {
    let status_color = if bot.enabled { "#22c55e" } else { "#64748b" };
    let status_label = if bot.enabled { "● Running" } else { "○ Stopped" };
    let kind = bot.kind.clone();

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 20px;",

            div { style: "display: flex; align-items: center; gap: 12px;",
                span { style: "font-size: 28px;", "{bot.kind.icon()}" }
                div {
                    h2 { style: "margin: 0; font-size: 18px; color: var(--fsn-color-text-primary);",
                        "{bot.name}"
                    }
                    span { style: "font-size: 12px; color: {status_color};", "{status_label}" }
                }
            }

            match kind {
                BotKind::Broadcast => rsx! {
                    BroadcastView { bot, on_update }
                },
                BotKind::Gatekeeper => rsx! {
                    GatekeeperView { bot, on_update }
                },
                _ => rsx! {
                    div {
                        style: "background: var(--fsn-color-bg-overlay); \
                                border-radius: var(--fsn-radius-md); \
                                padding: 20px; color: var(--fsn-color-text-muted); font-size: 13px;",
                        "This bot type does not have a usage interface yet."
                    }
                },
            }
        }
    }
}
