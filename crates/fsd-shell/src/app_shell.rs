/// AppShell — unified app wrapper with mode context and layout primitives.
use dioxus::prelude::*;

/// How an fsd-* app is rendered.
#[derive(Clone, PartialEq, Debug, Default)]
pub enum AppMode {
    /// Embedded inside a WindowFrame in the desktop shell.
    #[default]
    Window,
    /// Running as its own top-level OS window.
    Standalone,
    /// Running inside a terminal (dioxus-terminal).
    Tui,
}

/// Global CSS: Midnight Blue theme variables + page-transition animations.
/// Injected at the root and within every AppShell so variables are always available.
pub const GLOBAL_CSS: &str = r#"
:root, [data-theme="midnight-blue"] {
    /* ── Midnight Blue – backgrounds ──────────────────────────────── */
    --fsn-bg-base:     #0c1222;
    --fsn-bg-surface:  #162032;
    --fsn-bg-elevated: #1e2d45;
    --fsn-bg-sidebar:  #0a0f1a;
    --fsn-bg-card:     #1a2538;
    --fsn-bg-input:    #0f1a2e;
    --fsn-bg-hover:    #243352;

    /* ── Text (WCAG AAA on #0c1222) ───────────────────────────────── */
    --fsn-text-primary:   #e8edf5;
    --fsn-text-secondary: #a0b0c8;
    --fsn-text-muted:     #5a6e88;
    --fsn-text-bright:    #ffffff;

    /* ── Primary – luminous blue ──────────────────────────────────── */
    --fsn-primary:       #4d8bf5;
    --fsn-primary-hover: #3a78e8;
    --fsn-primary-text:  #ffffff;
    --fsn-primary-glow:  rgba(77, 139, 245, 0.35);

    /* ── Accent – cyan ────────────────────────────────────────────── */
    --fsn-accent:       #22d3ee;
    --fsn-accent-hover: #06b6d4;

    /* ── Status ───────────────────────────────────────────────────── */
    --fsn-success:    #34d399;
    --fsn-success-bg: rgba(52, 211, 153, 0.12);
    --fsn-warning:    #fbbf24;
    --fsn-warning-bg: rgba(251, 191, 36, 0.12);
    --fsn-error:      #f87171;
    --fsn-error-bg:   rgba(248, 113, 113, 0.12);
    --fsn-info:       #60a5fa;

    /* ── Borders ──────────────────────────────────────────────────── */
    --fsn-border:       rgba(148, 170, 200, 0.18);
    --fsn-border-focus: #4d8bf5;
    --fsn-border-hover: rgba(148, 170, 200, 0.3);

    /* ── Sidebar ──────────────────────────────────────────────────── */
    --fsn-sidebar-text:      #a0b0c8;
    --fsn-sidebar-active:    #4d8bf5;
    --fsn-sidebar-active-bg: rgba(77, 139, 245, 0.15);
    --fsn-sidebar-hover-bg:  rgba(255, 255, 255, 0.05);

    /* ── Glassmorphism ────────────────────────────────────────────── */
    --fsn-glass-bg:   rgba(22, 32, 50, 0.75);
    --fsn-glass-border: rgba(148, 170, 200, 0.12);
    --fsn-glass-blur: 16px;

    /* ── Shadows ──────────────────────────────────────────────────── */
    --fsn-shadow:      0 4px 16px rgba(0, 0, 0, 0.4);
    --fsn-shadow-glow: 0 0 24px rgba(77, 139, 245, 0.2);

    /* ── Motion ───────────────────────────────────────────────────── */
    --fsn-transition: all 180ms ease;

    /* ── Geometry ─────────────────────────────────────────────────── */
    --fsn-radius-sm: 6px;
    --fsn-radius-md: 10px;
    --fsn-radius-lg: 14px;

    /* ── Typography ───────────────────────────────────────────────── */
    --fsn-font:      'Inter', system-ui, sans-serif;
    --fsn-font-mono: 'JetBrains Mono', monospace;
    --fsn-font-size: 15px;

    /* ── Window frame (glassmorphism) ─────────────────────────────── */
    --fsn-window-bg:     rgba(15, 23, 42, 0.80);
    --fsn-window-border: rgba(255, 255, 255, 0.10);
    --fsn-window-shadow: 0 8px 32px rgba(0, 0, 0, 0.6);

    /* ── Compat aliases for existing --fsn-color-* usage ─────────── */
    --fsn-color-primary:       var(--fsn-primary);
    --fsn-color-bg-base:       var(--fsn-bg-base);
    --fsn-color-bg-surface:    var(--fsn-bg-surface);
    --fsn-color-bg-sidebar:    var(--fsn-bg-sidebar);
    --fsn-color-bg-panel:      var(--fsn-bg-card);
    --fsn-color-bg-card:       var(--fsn-bg-card);
    --fsn-color-bg-overlay:    var(--fsn-bg-elevated);
    --fsn-color-bg-active:     var(--fsn-bg-elevated);
    --fsn-color-bg-input:      var(--fsn-bg-input);
    --fsn-color-text-primary:  var(--fsn-text-primary);
    --fsn-color-text-secondary: var(--fsn-text-secondary);
    --fsn-color-text-muted:    var(--fsn-text-muted);
    --fsn-color-text-inverse:  var(--fsn-text-primary);
    --fsn-color-border-default: var(--fsn-border);
    --fsn-color-success:       var(--fsn-success);
    --fsn-color-warning:       var(--fsn-warning);
    --fsn-color-error:         var(--fsn-error);
    --fsn-color-info:          var(--fsn-info);
}

/* ── Cloud White — Light Theme ────────────────────────────────────── */
[data-theme="light"], [data-theme="cloud-white"] {
    /* backgrounds */
    --fsn-bg-base:     #f8fafc;
    --fsn-bg-surface:  #ffffff;
    --fsn-bg-elevated: #f1f5f9;
    --fsn-bg-sidebar:  #1e293b;
    --fsn-bg-card:     #ffffff;
    --fsn-bg-input:    #f1f5f9;
    --fsn-bg-hover:    #e2e8f0;

    /* text */
    --fsn-text-primary:   #0f172a;
    --fsn-text-secondary: #475569;
    --fsn-text-muted:     #94a3b8;
    --fsn-text-bright:    #0f172a;

    /* primary */
    --fsn-primary:       #2563eb;
    --fsn-primary-hover: #1d4ed8;
    --fsn-primary-text:  #ffffff;
    --fsn-primary-glow:  rgba(37, 99, 235, 0.2);

    /* accent */
    --fsn-accent:       #0891b2;
    --fsn-accent-hover: #0e7490;

    /* status */
    --fsn-success:    #16a34a;
    --fsn-success-bg: rgba(22, 163, 74, 0.12);
    --fsn-warning:    #d97706;
    --fsn-warning-bg: rgba(217, 119, 6, 0.12);
    --fsn-error:      #dc2626;
    --fsn-error-bg:   rgba(220, 38, 38, 0.12);
    --fsn-info:       #2563eb;

    /* borders */
    --fsn-border:       #e2e8f0;
    --fsn-border-focus: #2563eb;
    --fsn-border-hover: #cbd5e1;

    /* sidebar */
    --fsn-sidebar-text:      #cbd5e1;
    --fsn-sidebar-active:    #60a5fa;
    --fsn-sidebar-active-bg: rgba(96, 165, 250, 0.15);
    --fsn-sidebar-hover-bg:  rgba(255, 255, 255, 0.08);

    /* glassmorphism */
    --fsn-glass-bg:     rgba(255, 255, 255, 0.85);
    --fsn-glass-border: rgba(0, 0, 0, 0.08);

    /* shadows */
    --fsn-shadow:      0 1px 8px rgba(0, 0, 0, 0.08);
    --fsn-shadow-glow: 0 0 24px rgba(37, 99, 235, 0.12);

    /* window frame */
    --fsn-window-bg:     rgba(255, 255, 255, 0.90);
    --fsn-window-border: rgba(0, 0, 0, 0.08);
    --fsn-window-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
}

/* ── Cupertino — macOS-inspired Light Theme ───────────────────────── */
[data-theme="cupertino"] {
    --fsn-bg-base:     #f5f5f7;
    --fsn-bg-surface:  #ffffff;
    --fsn-bg-elevated: #f0f0f2;
    --fsn-bg-sidebar:  #1c1c1e;
    --fsn-bg-card:     #ffffff;
    --fsn-bg-input:    #ffffff;
    --fsn-bg-hover:    #e8e8ed;
    --fsn-text-primary:   #1d1d1f;
    --fsn-text-secondary: #6e6e73;
    --fsn-text-muted:     #aeaeb2;
    --fsn-text-bright:    #1d1d1f;
    --fsn-primary:       #007AFF;
    --fsn-primary-hover: #0071e3;
    --fsn-primary-text:  #ffffff;
    --fsn-primary-glow:  rgba(0, 122, 255, 0.2);
    --fsn-accent:       #30D158;
    --fsn-accent-hover: #28c44e;
    --fsn-success:    #34c759;
    --fsn-success-bg: rgba(52, 199, 89, 0.12);
    --fsn-warning:    #ff9f0a;
    --fsn-warning-bg: rgba(255, 159, 10, 0.12);
    --fsn-error:      #ff3b30;
    --fsn-error-bg:   rgba(255, 59, 48, 0.12);
    --fsn-info:       #007AFF;
    --fsn-border:       rgba(0, 0, 0, 0.1);
    --fsn-border-focus: #007AFF;
    --fsn-border-hover: rgba(0, 0, 0, 0.2);
    --fsn-sidebar-text:      #e5e5ea;
    --fsn-sidebar-active:    #007AFF;
    --fsn-sidebar-active-bg: rgba(0, 122, 255, 0.2);
    --fsn-sidebar-hover-bg:  rgba(255, 255, 255, 0.08);
    --fsn-glass-bg:     rgba(255, 255, 255, 0.85);
    --fsn-glass-border: rgba(0, 0, 0, 0.06);
    --fsn-glass-blur:   12px;
    --fsn-shadow:      0 1px 8px rgba(0, 0, 0, 0.1);
    --fsn-shadow-glow: 0 0 24px rgba(0, 122, 255, 0.1);
    --fsn-window-bg:     rgba(255, 255, 255, 0.92);
    --fsn-window-border: rgba(0, 0, 0, 0.1);
    --fsn-window-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
    --fsn-color-primary:        var(--fsn-primary);
    --fsn-color-bg-base:        var(--fsn-bg-base);
    --fsn-color-bg-surface:     var(--fsn-bg-surface);
    --fsn-color-bg-sidebar:     var(--fsn-bg-sidebar);
    --fsn-color-bg-panel:       var(--fsn-bg-card);
    --fsn-color-bg-card:        var(--fsn-bg-card);
    --fsn-color-bg-overlay:     var(--fsn-bg-elevated);
    --fsn-color-bg-active:      var(--fsn-bg-elevated);
    --fsn-color-bg-input:       var(--fsn-bg-input);
    --fsn-color-text-primary:   var(--fsn-text-primary);
    --fsn-color-text-secondary: var(--fsn-text-secondary);
    --fsn-color-text-muted:     var(--fsn-text-muted);
    --fsn-color-text-inverse:   var(--fsn-text-primary);
    --fsn-color-border-default: var(--fsn-border);
    --fsn-color-success:        var(--fsn-success);
    --fsn-color-warning:        var(--fsn-warning);
    --fsn-color-error:          var(--fsn-error);
    --fsn-color-info:           var(--fsn-info);
}

/* ── Nordic — Nord color palette Dark Theme ────────────────────────── */
[data-theme="nordic"] {
    --fsn-bg-base:     #2E3440;
    --fsn-bg-surface:  #3B4252;
    --fsn-bg-elevated: #434C5E;
    --fsn-bg-sidebar:  #2E3440;
    --fsn-bg-card:     #3B4252;
    --fsn-bg-input:    #2E3440;
    --fsn-bg-hover:    #4C566A;
    --fsn-text-primary:   #ECEFF4;
    --fsn-text-secondary: #D8DEE9;
    --fsn-text-muted:     #4C566A;
    --fsn-text-bright:    #ffffff;
    --fsn-primary:       #88C0D0;
    --fsn-primary-hover: #81b9c9;
    --fsn-primary-text:  #2E3440;
    --fsn-primary-glow:  rgba(136, 192, 208, 0.25);
    --fsn-accent:       #81A1C1;
    --fsn-accent-hover: #7a9ab8;
    --fsn-success:    #A3BE8C;
    --fsn-success-bg: rgba(163, 190, 140, 0.12);
    --fsn-warning:    #EBCB8B;
    --fsn-warning-bg: rgba(235, 203, 139, 0.12);
    --fsn-error:      #BF616A;
    --fsn-error-bg:   rgba(191, 97, 106, 0.12);
    --fsn-info:       #5E81AC;
    --fsn-border:       rgba(76, 86, 106, 0.5);
    --fsn-border-focus: #88C0D0;
    --fsn-border-hover: rgba(76, 86, 106, 0.8);
    --fsn-sidebar-text:      #D8DEE9;
    --fsn-sidebar-active:    #88C0D0;
    --fsn-sidebar-active-bg: rgba(136, 192, 208, 0.15);
    --fsn-sidebar-hover-bg:  rgba(76, 86, 106, 0.3);
    --fsn-glass-bg:     rgba(59, 66, 82, 0.85);
    --fsn-glass-border: rgba(76, 86, 106, 0.3);
    --fsn-glass-blur:   16px;
    --fsn-shadow:      0 4px 16px rgba(0, 0, 0, 0.35);
    --fsn-shadow-glow: 0 0 24px rgba(136, 192, 208, 0.15);
    --fsn-window-bg:     rgba(46, 52, 64, 0.90);
    --fsn-window-border: rgba(76, 86, 106, 0.5);
    --fsn-window-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    --fsn-color-primary:        var(--fsn-primary);
    --fsn-color-bg-base:        var(--fsn-bg-base);
    --fsn-color-bg-surface:     var(--fsn-bg-surface);
    --fsn-color-bg-sidebar:     var(--fsn-bg-sidebar);
    --fsn-color-bg-panel:       var(--fsn-bg-card);
    --fsn-color-bg-card:        var(--fsn-bg-card);
    --fsn-color-bg-overlay:     var(--fsn-bg-elevated);
    --fsn-color-bg-active:      var(--fsn-bg-elevated);
    --fsn-color-bg-input:       var(--fsn-bg-input);
    --fsn-color-text-primary:   var(--fsn-text-primary);
    --fsn-color-text-secondary: var(--fsn-text-secondary);
    --fsn-color-text-muted:     var(--fsn-text-muted);
    --fsn-color-text-inverse:   var(--fsn-text-primary);
    --fsn-color-border-default: var(--fsn-border);
    --fsn-color-success:        var(--fsn-success);
    --fsn-color-warning:        var(--fsn-warning);
    --fsn-color-error:          var(--fsn-error);
    --fsn-color-info:           var(--fsn-info);
}

/* ── Rosé Pine — Dark Rose Pink Theme ─────────────────────────────── */
[data-theme="rose-pine"] {
    --fsn-bg-base:     #191724;
    --fsn-bg-surface:  #1f1d2e;
    --fsn-bg-elevated: #26233a;
    --fsn-bg-sidebar:  #191724;
    --fsn-bg-card:     #1f1d2e;
    --fsn-bg-input:    #191724;
    --fsn-bg-hover:    #26233a;
    --fsn-text-primary:   #e0def4;
    --fsn-text-secondary: #908caa;
    --fsn-text-muted:     #6e6a86;
    --fsn-text-bright:    #ffffff;
    --fsn-primary:       #ebbcba;
    --fsn-primary-hover: #e8b3b1;
    --fsn-primary-text:  #191724;
    --fsn-primary-glow:  rgba(235, 188, 186, 0.25);
    --fsn-accent:       #31748f;
    --fsn-accent-hover: #286980;
    --fsn-success:    #9ccfd8;
    --fsn-success-bg: rgba(156, 207, 216, 0.12);
    --fsn-warning:    #f6c177;
    --fsn-warning-bg: rgba(246, 193, 119, 0.12);
    --fsn-error:      #eb6f92;
    --fsn-error-bg:   rgba(235, 111, 146, 0.12);
    --fsn-info:       #c4a7e7;
    --fsn-border:       rgba(110, 106, 134, 0.25);
    --fsn-border-focus: #ebbcba;
    --fsn-border-hover: rgba(110, 106, 134, 0.5);
    --fsn-sidebar-text:      #908caa;
    --fsn-sidebar-active:    #ebbcba;
    --fsn-sidebar-active-bg: rgba(235, 188, 186, 0.12);
    --fsn-sidebar-hover-bg:  rgba(110, 106, 134, 0.1);
    --fsn-glass-bg:     rgba(31, 29, 46, 0.85);
    --fsn-glass-border: rgba(110, 106, 134, 0.15);
    --fsn-glass-blur:   16px;
    --fsn-shadow:      0 4px 16px rgba(0, 0, 0, 0.4);
    --fsn-shadow-glow: 0 0 24px rgba(235, 188, 186, 0.15);
    --fsn-window-bg:     rgba(25, 23, 36, 0.90);
    --fsn-window-border: rgba(110, 106, 134, 0.25);
    --fsn-window-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
    --fsn-color-primary:        var(--fsn-primary);
    --fsn-color-bg-base:        var(--fsn-bg-base);
    --fsn-color-bg-surface:     var(--fsn-bg-surface);
    --fsn-color-bg-sidebar:     var(--fsn-bg-sidebar);
    --fsn-color-bg-panel:       var(--fsn-bg-card);
    --fsn-color-bg-card:        var(--fsn-bg-card);
    --fsn-color-bg-overlay:     var(--fsn-bg-elevated);
    --fsn-color-bg-active:      var(--fsn-bg-elevated);
    --fsn-color-bg-input:       var(--fsn-bg-input);
    --fsn-color-text-primary:   var(--fsn-text-primary);
    --fsn-color-text-secondary: var(--fsn-text-secondary);
    --fsn-color-text-muted:     var(--fsn-text-muted);
    --fsn-color-text-inverse:   var(--fsn-text-primary);
    --fsn-color-border-default: var(--fsn-border);
    --fsn-color-success:        var(--fsn-success);
    --fsn-color-warning:        var(--fsn-warning);
    --fsn-color-error:          var(--fsn-error);
    --fsn-color-info:           var(--fsn-info);
}

* { box-sizing: border-box; margin: 0; padding: 0; }

html, body { height: 100%; overflow: hidden; }

body {
    background: var(--fsn-bg-base);
    color: var(--fsn-text-primary);
    font-family: var(--fsn-font);
    font-size: var(--fsn-font-size);
}

/* ── Page-transition animations ───────────────────────────────────── */
@keyframes slideInRight {
    from { opacity: 0; transform: translateX(12px); }
    to   { opacity: 1; transform: translateX(0); }
}
@keyframes fadeInUp {
    from { opacity: 0; transform: translateY(6px); }
    to   { opacity: 1; transform: translateY(0); }
}
.fsd-page-enter { animation: slideInRight 180ms ease forwards; }
.fsd-page-fade  { animation: fadeInUp 140ms ease forwards; }
@media (prefers-reduced-motion: reduce) {
    .fsd-page-enter, .fsd-page-fade { animation: none; }
}

/* ── Window control buttons (KDE/Breeze style) ─────────────────── */
.fsd-window-btn {
    width: 22px; height: 20px;
    border-radius: var(--fsn-radius-sm);
    background: transparent;
    border: 1px solid transparent;
    cursor: pointer; padding: 0;
    display: inline-flex; align-items: center; justify-content: center;
    color: var(--fsn-text-secondary);
    transition: background 120ms, border-color 120ms, color 120ms;
    flex-shrink: 0;
}
.fsd-window-btn:hover {
    background: var(--fsn-bg-hover);
    border-color: var(--fsn-border);
}
.fsd-window-btn--close:hover {
    background: var(--fsn-error-bg);
    border-color: var(--fsn-error);
    color: var(--fsn-error);
}

/* ── Disabled button state ─────────────────────────────────────── */
button:disabled,
.fsd-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
    pointer-events: none;
}

/* ── Scrollable container ──────────────────────────────────────── */
.fsn-scrollable {
    overflow-y: auto;
    overflow-x: hidden;
    scrollbar-width: thin;
    scrollbar-color: var(--fsn-border) transparent;
}
.fsn-scrollable::-webkit-scrollbar { width: 6px; }
.fsn-scrollable::-webkit-scrollbar-track { background: transparent; }
.fsn-scrollable::-webkit-scrollbar-thumb {
    background: var(--fsn-border);
    border-radius: 3px;
}
.fsn-scrollable::-webkit-scrollbar-thumb:hover {
    background: var(--fsn-border-hover);
}

/* ── Content area ──────────────────────────────────────────────── */
.fsn-content { flex: 1; overflow-y: auto; }

/* ── Sidebar collapsed: CSS hover-tooltip via ::after ─────────── */
.fsd-sidebar--collapsed .fsd-sidebar__item {
    position: relative;
}
.fsd-sidebar--collapsed .fsd-sidebar__item::after {
    content: attr(data-label);
    position: absolute;
    left: calc(100% + 10px);
    top: 50%;
    transform: translateY(-50%);
    background: var(--fsn-bg-elevated);
    color: var(--fsn-text-primary);
    border: 1px solid var(--fsn-border);
    border-radius: var(--fsn-radius-md);
    padding: 4px 10px;
    font-size: 12px;
    white-space: nowrap;
    pointer-events: none;
    opacity: 0;
    transition: opacity 120ms ease;
    z-index: 9999;
    box-shadow: var(--fsn-shadow);
}
.fsd-sidebar--collapsed .fsd-sidebar__item:hover::after {
    opacity: 1;
}
"#;

/// Root app wrapper. Injects global CSS and applies mode-specific root styles.
#[component]
pub fn AppShell(mode: AppMode, children: Element) -> Element {
    let height = match mode {
        AppMode::Window     => "height: 100%; width: 100%;",
        AppMode::Standalone => "height: 100vh; width: 100vw;",
        AppMode::Tui        => "height: 100%; width: 100%;",
    };
    rsx! {
        style { "{GLOBAL_CSS}" }
        div {
            class: "fsd-app-shell",
            style: "display: flex; flex-direction: column; {height} overflow: hidden;",
            {children}
        }
    }
}

/// Consistent content wrapper: max-width, padding, and scroll behavior.
#[component]
pub fn ScreenWrapper(
    max_width: Option<String>,
    #[props(default = true)]
    scroll: bool,
    #[props(default = "24px".to_string())]
    padding: String,
    children: Element,
) -> Element {
    let overflow = if scroll { "auto" } else { "hidden" };
    let max_w    = max_width.as_deref().unwrap_or("none");
    rsx! {
        div {
            class: "fsd-screen-wrapper",
            style: "flex: 1; overflow: {overflow}; padding: {padding}; max-width: {max_w}; \
                    width: 100%; box-sizing: border-box;",
            {children}
        }
    }
}

// ── Standard Layouts ──────────────────────────────────────────────────────────

/// Layout A — full-width scrollable column (fsd-store, fsd-studio).
#[component]
pub fn LayoutA(children: Element) -> Element {
    rsx! {
        div {
            class: "fsd-layout-a fsd-page-enter",
            style: "display: flex; flex-direction: column; height: 100%; width: 100%; overflow: hidden;",
            {children}
        }
    }
}

/// Layout B — fixed sidebar (master) + scrollable detail pane.
/// Used for: fsd-conductor, fsd-settings.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutBProps {
    #[props(default = 240)]
    pub sidebar_width: u32,
    pub master: Element,
    pub children: Element,
}

#[component]
pub fn LayoutB(props: LayoutBProps) -> Element {
    rsx! {
        div {
            class: "fsd-layout-b fsd-page-enter",
            style: "display: flex; height: 100%; width: 100%; overflow: hidden;",
            div {
                class: "fsd-layout-b__master",
                style: "width: {props.sidebar_width}px; flex-shrink: 0; overflow-y: auto; \
                        background: var(--fsn-color-bg-surface, #0f172a); \
                        border-right: 1px solid var(--fsn-color-border-default, #334155);",
                {props.master}
            }
            div {
                class: "fsd-layout-b__detail fsn-scrollable",
                style: "flex: 1; overflow: auto;",
                {props.children}
            }
        }
    }
}

/// Layout C — centered card (fsd-profile, login screens).
#[component]
pub fn LayoutC(
    #[props(default = 640)]
    max_width: u32,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "fsd-layout-c fsd-page-fade",
            style: "display: flex; justify-content: center; overflow: auto; \
                    height: 100%; width: 100%; padding: 32px 24px; box-sizing: border-box;",
            div {
                style: "width: 100%; max-width: {max_width}px;",
                {children}
            }
        }
    }
}
