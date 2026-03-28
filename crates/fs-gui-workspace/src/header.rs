/// `ShellHeader` — data types for the 60px fixed header.
///
/// All rendering is done in `shell.rs` via iced widgets.
use fs_i18n;

/// A single breadcrumb entry.
#[derive(Clone, PartialEq, Debug)]
pub struct Breadcrumb {
    pub label: String,
    pub icon: Option<String>,
}

impl Breadcrumb {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
        }
    }
}

/// A menu item descriptor for the menu bar.
#[derive(Clone, PartialEq, Debug)]
pub struct MenuItem {
    pub label: String,
    pub items: Vec<MenuAction>,
}

/// A single action in a submenu (leaf item only).
#[derive(Clone, PartialEq, Debug)]
pub struct SubAction {
    pub label: String,
    pub id: &'static str,
}

/// A single action in a menu dropdown.
#[derive(Clone, PartialEq, Debug)]
pub enum MenuAction {
    Action {
        label: String,
        shortcut: Option<&'static str>,
        id: &'static str,
    },
    SubMenu {
        label: String,
        items: Vec<SubAction>,
    },
    Separator,
}

/// State for the header menu bar.
#[derive(Clone, Default, PartialEq, Debug)]
pub struct HeaderState {
    /// Index of the currently open top-level menu (None = closed).
    pub open_menu: Option<usize>,
    pub breadcrumbs: Vec<Breadcrumb>,
    pub user_name: String,
    pub user_avatar: Option<String>,
    pub avatar_menu_open: bool,
}

impl HeaderState {
    pub fn new(user_name: impl Into<String>) -> Self {
        Self {
            user_name: user_name.into(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_breadcrumb(mut self, label: impl Into<String>) -> Self {
        self.breadcrumbs.push(Breadcrumb::new(label));
        self
    }
}

/// Build the default menu structure.
#[must_use]
pub fn default_menu() -> Vec<MenuItem> {
    vec![
        MenuItem {
            label: "FreeSynergy".into(),
            items: vec![
                MenuAction::Action {
                    label: fs_i18n::t("shell.menu.about").into(),
                    shortcut: None,
                    id: "about",
                },
                MenuAction::Action {
                    label: fs_i18n::t("settings.title").into(),
                    shortcut: Some("Ctrl+,"),
                    id: "settings",
                },
                MenuAction::Action {
                    label: fs_i18n::t("shell.menu.launcher").into(),
                    shortcut: Some("Ctrl+Space"),
                    id: "launcher",
                },
                MenuAction::Separator,
                MenuAction::Action {
                    label: fs_i18n::t("shell.menu.quit").into(),
                    shortcut: Some("Ctrl+Q"),
                    id: "quit",
                },
            ],
        },
        MenuItem {
            label: fs_i18n::t("shell.menu.view").into(),
            items: vec![
                MenuAction::Action {
                    label: fs_i18n::t("shell.menu.fullscreen").into(),
                    shortcut: Some("F11"),
                    id: "fullscreen",
                },
                MenuAction::Separator,
                MenuAction::SubMenu {
                    label: fs_i18n::t("shell.menu.theme").into(),
                    items: vec![
                        SubAction {
                            label: "Midnight Blue".into(),
                            id: "theme-midnight-blue",
                        },
                        SubAction {
                            label: "Cloud White".into(),
                            id: "theme-cloud-white",
                        },
                        SubAction {
                            label: "Cupertino".into(),
                            id: "theme-cupertino",
                        },
                        SubAction {
                            label: "Nordic".into(),
                            id: "theme-nordic",
                        },
                        SubAction {
                            label: "Rose Pine".into(),
                            id: "theme-rose-pine",
                        },
                    ],
                },
            ],
        },
        MenuItem {
            label: fs_i18n::t("shell.menu.services").into(),
            items: vec![
                MenuAction::Action {
                    label: fs_i18n::t("shell.menu.open_container").into(),
                    shortcut: None,
                    id: "open-container-app",
                },
                MenuAction::Separator,
                MenuAction::Action {
                    label: fs_i18n::t("shell.menu.start_all").into(),
                    shortcut: None,
                    id: "start-all",
                },
                MenuAction::Action {
                    label: fs_i18n::t("shell.menu.stop_all").into(),
                    shortcut: None,
                    id: "stop-all",
                },
            ],
        },
        MenuItem {
            label: fs_i18n::t("shell.menu.tools").into(),
            items: vec![
                MenuAction::Action {
                    label: fs_i18n::t("shell.menu.open_store").into(),
                    shortcut: Some("Ctrl+S"),
                    id: "open-store",
                },
                MenuAction::Action {
                    label: fs_i18n::t("shell.menu.open_tasks").into(),
                    shortcut: Some("Ctrl+T"),
                    id: "open-tasks",
                },
                MenuAction::Action {
                    label: fs_i18n::t("shell.menu.open_bots").into(),
                    shortcut: None,
                    id: "open-bots",
                },
                MenuAction::Separator,
                MenuAction::Action {
                    label: fs_i18n::t("shell.menu.install_package").into(),
                    shortcut: Some("Ctrl+I"),
                    id: "install-package",
                },
            ],
        },
        MenuItem {
            label: fs_i18n::t("shell.menu.help").into(),
            items: vec![
                MenuAction::Action {
                    label: fs_i18n::t("shell.menu.help").into(),
                    shortcut: Some("F1"),
                    id: "help",
                },
                MenuAction::Action {
                    label: fs_i18n::t("shell.menu.keyboard_shortcuts").into(),
                    shortcut: None,
                    id: "shortcuts",
                },
                MenuAction::Separator,
                MenuAction::Action {
                    label: fs_i18n::t("shell.menu.report_bug").into(),
                    shortcut: None,
                    id: "report-bug",
                },
            ],
        },
    ]
}
