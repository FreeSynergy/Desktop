//! `DesktopShell` — iced MVU root application for `FreeSynergy` Desktop.
//!
//! Architecture: Elm (MVU) pattern via iced 0.13.
//!   - `DesktopShell`   — owns shell chrome state + active app routing
//!   - `DesktopMessage` — flat enum wrapping all sub-app messages + shell actions
//!   - `update()`       — state transitions
//!   - `view()`         — shell chrome + active app content placeholder

#[cfg(feature = "iced")]
use fs_gui_engine_iced::iced::{
    self,
    widget::{button, column, container, row, scrollable, text, text_input, Space},
    Alignment, Element, Length, Task,
};

use chrono::Local;

/// Convenience: translate key to owned `String` for use in iced widgets.
fn tr(key: &str) -> String {
    fs_i18n::t(key).to_string()
}

/// Convenience: translate key with variables to owned `String`.
fn tr_with(key: &str, args: &[(&str, &str)]) -> String {
    fs_i18n::t_with(key, args).to_string()
}

use crate::header::{default_menu, HeaderState};
use crate::launcher::{AppGroup, LauncherState};
use crate::notification::{NotificationHistory, NotificationManager};
use crate::sidebar::{default_pinned_items, default_sidebar_sections, SidebarItem, SidebarSection};
use crate::taskbar::{default_apps, AppEntry};
use crate::window::{AppId, Window, WindowHost, WindowId, WindowManager};

// ── DesktopMessage ────────────────────────────────────────────────────────────

/// All messages the desktop shell can process.
#[derive(Debug, Clone)]
pub enum DesktopMessage {
    // ── Window management ─────────────────────────────────────────────────────
    OpenApp(AppId),
    CloseWindow(WindowId),
    FocusWindow(WindowId),
    MinimizeWindow(WindowId),

    // ── Shell navigation ──────────────────────────────────────────────────────
    MenuAction(String),
    SidebarSelect(String),
    NotificationDismiss(u64),
    NotificationMarkRead,

    // ── Launcher ─────────────────────────────────────────────────────────────
    LauncherToggle,
    LauncherSearch(String),
    LauncherLaunch(String),
    LauncherClose,
    LauncherPrevPage,
    LauncherNextPage,
    LauncherGotoPage(usize),

    // ── Header ────────────────────────────────────────────────────────────────
    HeaderMenuToggle(usize),
    HeaderMenuClose,
    HeaderAvatarToggle,

    // ── Clock tick ────────────────────────────────────────────────────────────
    ClockTick,

    // ── No-op / async completion ──────────────────────────────────────────────
    Noop,
}

// ── DesktopShell ──────────────────────────────────────────────────────────────

/// Root desktop application state.
///
/// Owns all shell chrome state and routes to sub-app views.
/// Implements the iced MVU pattern via `update()` and `view()`.
pub struct DesktopShell {
    // ── Window management ─────────────────────────────────────────────────────
    pub windows: WindowManager,
    pub active_app: Option<AppId>,

    // ── Shell chrome ─────────────────────────────────────────────────────────
    pub header_state: HeaderState,
    pub taskbar_apps: Vec<AppEntry>,
    pub notifications: NotificationManager,
    pub notification_history: NotificationHistory,
    pub sidebar_sections: Vec<SidebarSection>,
    pub pinned_items: Vec<SidebarItem>,
    pub launcher_state: LauncherState,
    pub current_desktop: usize,

    // ── Clock ─────────────────────────────────────────────────────────────────
    pub clock_time: String,
    pub clock_date: String,
}

impl Default for DesktopShell {
    fn default() -> Self {
        crate::builtin_apps::ensure_registered();
        Self {
            windows: WindowManager::default(),
            active_app: None,
            header_state: HeaderState::new(std::env::var("USER").unwrap_or_else(|_| "User".into())),
            taskbar_apps: default_apps(),
            notifications: NotificationManager::default(),
            notification_history: NotificationHistory::default(),
            sidebar_sections: default_sidebar_sections(),
            pinned_items: default_pinned_items(),
            launcher_state: LauncherState::default(),
            current_desktop: 0,
            clock_time: Local::now().format("%H:%M").to_string(),
            clock_date: Local::now().format("%d.%m.%Y").to_string(),
        }
    }
}

// ── update() ─────────────────────────────────────────────────────────────────

#[cfg(feature = "iced")]
impl DesktopShell {
    /// Process a `DesktopMessage` and return any async tasks.
    pub fn update(&mut self, msg: DesktopMessage) -> Task<DesktopMessage> {
        match msg {
            // ── Window management ─────────────────────────────────────────────
            DesktopMessage::OpenApp(app_id) => {
                self.active_app = Some(app_id);
                let meta = Window::new(app_id.name()).with_icon(app_id.icon().to_string());
                let open = crate::window::OpenWindow::new(meta, app_id);
                self.windows.open_window(open);
            }
            DesktopMessage::CloseWindow(id) => {
                self.windows.close_window(id);
                if self.windows.open_windows().is_empty() {
                    self.active_app = None;
                }
            }
            DesktopMessage::FocusWindow(id) => {
                self.windows.focus_window(id);
            }
            DesktopMessage::MinimizeWindow(id) => {
                self.windows.minimize_window(id);
            }

            // ── Shell navigation ──────────────────────────────────────────────
            DesktopMessage::MenuAction(id) => {
                self.handle_menu_action(&id);
            }
            DesktopMessage::SidebarSelect(id) => {
                self.handle_sidebar_select(&id);
            }
            DesktopMessage::NotificationDismiss(id) => {
                self.notifications.dismiss(id);
            }
            DesktopMessage::NotificationMarkRead => {
                self.notification_history.mark_all_read();
            }

            // ── Launcher ─────────────────────────────────────────────────────
            DesktopMessage::LauncherToggle => {
                self.launcher_state.toggle();
            }
            DesktopMessage::LauncherSearch(q) => {
                self.launcher_state.set_query(q);
            }
            DesktopMessage::LauncherLaunch(id) => {
                self.launcher_state.close();
                self.handle_sidebar_select(&id);
            }
            DesktopMessage::LauncherClose => {
                self.launcher_state.close();
            }
            DesktopMessage::LauncherPrevPage => {
                let groups = AppGroup::filtered(&self.taskbar_apps, &self.launcher_state.query);
                let total = AppGroup::total_pages(&groups);
                self.launcher_state.prev_page(total);
            }
            DesktopMessage::LauncherNextPage => {
                let groups = AppGroup::filtered(&self.taskbar_apps, &self.launcher_state.query);
                let total = AppGroup::total_pages(&groups);
                self.launcher_state.next_page(total);
            }
            DesktopMessage::LauncherGotoPage(idx) => {
                self.launcher_state.goto_page(idx);
            }

            // ── Header ────────────────────────────────────────────────────────
            DesktopMessage::HeaderMenuToggle(idx) => {
                self.header_state.open_menu = if self.header_state.open_menu == Some(idx) {
                    None
                } else {
                    Some(idx)
                };
            }
            DesktopMessage::HeaderMenuClose => {
                self.header_state.open_menu = None;
            }
            DesktopMessage::HeaderAvatarToggle => {
                self.header_state.avatar_menu_open = !self.header_state.avatar_menu_open;
            }

            // ── Clock tick ────────────────────────────────────────────────────
            DesktopMessage::ClockTick => {
                self.clock_time = Local::now().format("%H:%M").to_string();
                self.clock_date = Local::now().format("%d.%m.%Y").to_string();
            }

            DesktopMessage::Noop => {}
        }
        Task::none()
    }

    fn handle_menu_action(&mut self, id: &str) {
        self.header_state.open_menu = None;
        match id {
            "launcher" => self.launcher_state.toggle(),
            "settings" => {
                self.active_app = Some(AppId::Settings);
            }
            _ => {}
        }
    }

    fn handle_sidebar_select(&mut self, id: &str) {
        let app_id = match id {
            "browser" => Some(AppId::Browser),
            "settings" => Some(AppId::Settings),
            "profile" => Some(AppId::Profile),
            "store" => Some(AppId::Store),
            "lenses" => Some(AppId::Lenses),
            "builder" => Some(AppId::Builder),
            "tasks" => Some(AppId::Tasks),
            "bots" => Some(AppId::Bots),
            "ai" => Some(AppId::Ai),
            "container-app" => Some(AppId::Container),
            "managers" | "managers-folder" => Some(AppId::Managers),
            "help" => Some(AppId::Help),
            _ => None,
        };
        if let Some(app) = app_id {
            self.active_app = Some(app);
        }
    }
}

// ── view() ────────────────────────────────────────────────────────────────────

#[cfg(feature = "iced")]
impl DesktopShell {
    /// Render the full desktop shell.
    #[must_use]
    pub fn view(&self) -> Element<'_, DesktopMessage> {
        if self.launcher_state.open {
            return self.view_launcher();
        }

        let header = self.view_header();
        let sidebar = self.view_sidebar();
        let content = self.view_content();
        let taskbar = self.view_taskbar();

        let main_row = row![sidebar, content].spacing(0).height(Length::Fill);

        let shell = column![header, main_row, taskbar]
            .spacing(0)
            .height(Length::Fill)
            .width(Length::Fill);

        container(shell)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_header(&self) -> Element<'_, DesktopMessage> {
        let brand = text("FreeSynergy")
            .size(15)
            .color(iced::Color::from_rgb(0.02, 0.74, 0.84));

        let by_kal = text(" by KalEl")
            .size(11)
            .color(iced::Color::from_rgb(0.6, 0.6, 0.7));

        let brand_row = row![brand, by_kal].align_y(Alignment::Center);

        let menu_bar = Self::view_menu_bar();

        let clock = column![
            text(&self.clock_time).size(13),
            text(&self.clock_date).size(10),
        ]
        .align_x(Alignment::Center);

        let notif_count = self.notification_history.unread_count();
        let bell_label = if notif_count > 0 {
            format!("🔔 {notif_count}")
        } else {
            "🔔".to_string()
        };
        let bell_btn = button(text(bell_label).size(13))
            .on_press(DesktopMessage::NotificationMarkRead)
            .padding([4, 8]);

        let avatar_initial = self
            .header_state
            .user_name
            .chars()
            .next()
            .map_or_else(|| "?".to_string(), |c| c.to_uppercase().to_string());
        let avatar_btn = button(text(avatar_initial).size(13))
            .on_press(DesktopMessage::HeaderAvatarToggle)
            .padding([4, 8]);

        let header_row = row![
            brand_row,
            Space::with_width(12),
            menu_bar,
            Space::with_width(Length::Fill),
            bell_btn,
            Space::with_width(8),
            avatar_btn,
            Space::with_width(12),
            clock,
            Space::with_width(8),
        ]
        .align_y(Alignment::Center)
        .spacing(4)
        .padding([0, 8])
        .height(60);

        container(header_row)
            .width(Length::Fill)
            .height(60)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.04, 0.06, 0.10,
                ))),
                border: iced::Border {
                    color: iced::Color::from_rgba(0.58, 0.67, 0.78, 0.18),
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..container::Style::default()
            })
            .into()
    }

    fn view_menu_bar() -> Element<'static, DesktopMessage> {
        let menus = default_menu();
        // Collect owned labels first to avoid borrow-of-temporary issues.
        let labels: Vec<(usize, String)> = menus
            .iter()
            .enumerate()
            .map(|(idx, m)| (idx, m.label.clone()))
            .collect();
        let buttons: Vec<Element<'_, DesktopMessage>> = labels
            .into_iter()
            .map(|(idx, label)| {
                button(text(label).size(13))
                    .on_press(DesktopMessage::HeaderMenuToggle(idx))
                    .padding([4, 10])
                    .into()
            })
            .collect();

        row(buttons).spacing(2).into()
    }

    fn view_sidebar(&self) -> Element<'_, DesktopMessage> {
        let launcher_btn = button(text(format!("⊞  {}", tr("shell.launcher.title"))).size(13))
            .on_press(DesktopMessage::LauncherToggle)
            .width(Length::Fill)
            .padding([6, 12]);

        let mut items_col: Vec<Element<'_, DesktopMessage>> = vec![launcher_btn.into()];

        for section in &self.sidebar_sections {
            if let Some(title) = &section.title {
                items_col.push(
                    text(title)
                        .size(10)
                        .color(iced::Color::from_rgb(0.5, 0.5, 0.6))
                        .into(),
                );
            }
            for item in &section.items {
                items_col.push(self.view_sidebar_item(item));
            }
        }

        if !self.pinned_items.is_empty() {
            items_col.push(Space::with_height(Length::Fill).into());
            for item in &self.pinned_items {
                items_col.push(self.view_sidebar_item(item));
            }
        }

        let sidebar_col =
            scrollable(column(items_col).spacing(2).padding([8, 4])).height(Length::Fill);

        container(sidebar_col)
            .width(220)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.04, 0.06, 0.10,
                ))),
                border: iced::Border {
                    color: iced::Color::from_rgba(0.58, 0.67, 0.78, 0.12),
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..container::Style::default()
            })
            .into()
    }

    fn view_sidebar_item<'a>(&'a self, item: &'a SidebarItem) -> Element<'a, DesktopMessage> {
        let is_active = self
            .active_app
            .is_some_and(|a| a.name().to_lowercase() == item.id);

        let label = format!("{} {}", item.icon, item.label);
        let id = item.id.clone();

        let btn = button(text(label).size(13))
            .on_press(DesktopMessage::SidebarSelect(id))
            .width(Length::Fill)
            .padding([6, 12]);

        if is_active {
            container(btn)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(
                        0.02, 0.74, 0.84, 0.15,
                    ))),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.02, 0.74, 0.84),
                        width: 2.0,
                        radius: 6.0.into(),
                    },
                    ..container::Style::default()
                })
                .into()
        } else {
            btn.into()
        }
    }

    fn view_content(&self) -> Element<'_, DesktopMessage> {
        let content: Element<'_, DesktopMessage> = match self.active_app {
            Some(app_id) => {
                // Content placeholder for each app — sub-apps will be embedded
                // once they are added as dependencies of fs-gui-workspace.
                container(
                    column![
                        text(format!("{} {}", app_id.icon(), app_id.name())).size(32),
                        Space::with_height(16),
                        text(app_id.name())
                            .size(20)
                            .color(iced::Color::from_rgb(0.02, 0.74, 0.84)),
                        Space::with_height(8),
                        text(tr("shell.app.opening"))
                            .size(14)
                            .color(iced::Color::from_rgb(0.6, 0.6, 0.7)),
                    ]
                    .align_x(Alignment::Center)
                    .spacing(8),
                )
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
            }
            None => {
                // Home screen
                container(
                    column![
                        text("FreeSynergy")
                            .size(36)
                            .color(iced::Color::from_rgb(0.02, 0.74, 0.84)),
                        text("by KalEl")
                            .size(14)
                            .color(iced::Color::from_rgb(0.5, 0.5, 0.6)),
                        Space::with_height(32),
                        text(tr("shell.home.hint"))
                            .size(14)
                            .color(iced::Color::from_rgb(0.6, 0.6, 0.7)),
                        Space::with_height(16),
                        button(text(format!("⊞  {}", tr("shell.launcher.open"))).size(14))
                            .on_press(DesktopMessage::LauncherToggle)
                            .padding([8, 20]),
                    ]
                    .align_x(Alignment::Center)
                    .spacing(4),
                )
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
            }
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.05, 0.07, 0.12,
                ))),
                ..container::Style::default()
            })
            .into()
    }

    fn view_taskbar(&self) -> Element<'_, DesktopMessage> {
        let launcher_btn = button(text("⊞").size(20))
            .on_press(DesktopMessage::LauncherToggle)
            .padding([4, 8]);

        let mut app_btns: Vec<Element<'_, DesktopMessage>> = vec![launcher_btn.into()];

        for app in &self.taskbar_apps {
            let id = app.id.clone();
            let icon_text = if app.icon.starts_with('<') {
                "●".to_string()
            } else {
                app.icon.clone()
            };
            let btn = button(text(icon_text).size(18))
                .on_press(DesktopMessage::SidebarSelect(id))
                .padding([4, 8]);
            app_btns.push(btn.into());
        }

        app_btns.push(Space::with_width(Length::Fill).into());

        let clock = column![
            text(&self.clock_time).size(13),
            text(&self.clock_date).size(10),
        ]
        .align_x(Alignment::Center)
        .padding([0, 12]);
        app_btns.push(clock.into());

        let taskbar_row = row(app_btns)
            .align_y(Alignment::Center)
            .spacing(4)
            .padding([0, 8])
            .height(48);

        container(taskbar_row)
            .width(Length::Fill)
            .height(48)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.04, 0.06, 0.10,
                ))),
                border: iced::Border {
                    color: iced::Color::from_rgba(0.58, 0.67, 0.78, 0.18),
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..container::Style::default()
            })
            .into()
    }

    fn view_launcher(&self) -> Element<'_, DesktopMessage> {
        let query = self.launcher_state.query.clone();
        let groups = AppGroup::filtered(&self.taskbar_apps, &query);
        let total_pages = AppGroup::total_pages(&groups);
        let cur_page = self.launcher_state.page.min(total_pages - 1);
        // Clone the page slice to avoid borrow-of-local issues when building elements.
        let page_groups: Vec<AppGroup> = AppGroup::page_slice(&groups, cur_page).to_vec();

        let search_placeholder = tr("shell.launcher.search_placeholder");
        let search = text_input(&search_placeholder, &query)
            .on_input(DesktopMessage::LauncherSearch)
            .padding([10, 14])
            .size(15)
            .width(Length::Fill);

        let mut group_items: Vec<Element<'_, DesktopMessage>> = vec![];

        if groups.is_empty() {
            group_items.push(
                container(
                    text(tr_with(
                        "shell.launcher.no_apps",
                        &[("query", query.as_str())],
                    ))
                    .size(14)
                    .color(iced::Color::from_rgb(0.6, 0.6, 0.7)),
                )
                .center_x(Length::Fill)
                .padding(48)
                .into(),
            );
        } else {
            for group in page_groups {
                // Move group fields to avoid borrow-of-local-variable issues.
                let group_label = group.label.clone();
                let label = text(group_label)
                    .size(11)
                    .color(iced::Color::from_rgb(0.5, 0.5, 0.6));

                let tiles: Vec<Element<'_, DesktopMessage>> = group
                    .apps
                    .into_iter()
                    .map(|app| {
                        let id = app.id.clone();
                        let icon = if app.icon.starts_with('<') {
                            "●".to_string()
                        } else {
                            app.icon.clone()
                        };
                        let label_key = app.label_key.clone();
                        button(
                            column![text(icon).size(28), text(label_key).size(11),]
                                .align_x(Alignment::Center)
                                .spacing(4),
                        )
                        .on_press(DesktopMessage::LauncherLaunch(id))
                        .padding([12, 8])
                        .width(100)
                        .into()
                    })
                    .collect();

                let tile_row = row(tiles).spacing(8).wrap();

                group_items.push(
                    column![label, Space::with_height(4), tile_row]
                        .spacing(0)
                        .padding([8, 12])
                        .into(),
                );
            }
        }

        let mut pagination: Vec<Element<'_, DesktopMessage>> = vec![];
        if total_pages > 1 {
            let prev_btn = button(text("◄").size(13))
                .on_press(DesktopMessage::LauncherPrevPage)
                .padding([2, 10]);
            let next_btn = button(text("►").size(13))
                .on_press(DesktopMessage::LauncherNextPage)
                .padding([2, 10]);
            let page_label = text(format!("{} / {}", cur_page + 1, total_pages)).size(12);

            pagination.push(prev_btn.into());
            pagination.push(page_label.into());
            for i in 0..total_pages {
                let dot = button(text(if i == cur_page { "●" } else { "○" }).size(8))
                    .on_press(DesktopMessage::LauncherGotoPage(i))
                    .padding([2, 4]);
                pagination.push(dot.into());
            }
            pagination.push(next_btn.into());
        }

        let close_btn = button(text(tr("actions.close")).size(13))
            .on_press(DesktopMessage::LauncherClose)
            .padding([6, 16]);

        let panel = column![
            text("FreeSynergy")
                .size(18)
                .color(iced::Color::from_rgb(0.02, 0.74, 0.84)),
            Space::with_height(16),
            search,
            Space::with_height(12),
            scrollable(column(group_items).spacing(8)).height(400),
            Space::with_height(8),
            row(pagination).spacing(4).align_y(Alignment::Center),
            Space::with_height(8),
            close_btn,
        ]
        .spacing(0)
        .padding(24)
        .width(700)
        .max_width(800);

        container(panel)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgba(
                    0.0, 0.0, 0.0, 0.85,
                ))),
                ..container::Style::default()
            })
            .into()
    }
}

// ── Non-iced stub ─────────────────────────────────────────────────────────────

#[cfg(not(feature = "iced"))]
impl DesktopShell {
    pub fn update(&mut self, _msg: DesktopMessage) {}
    pub fn view(&self) {}
}
