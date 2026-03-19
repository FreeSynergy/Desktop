# CLAUDE.md – FreeSynergy.Desktop

## What is this?

FreeSynergy.Desktop – a KDE-like desktop environment for the FreeSynergy ecosystem.
Built with Dioxus (Desktop + Web + TUI from a single codebase).

Each `fsd-*` crate can run as a standalone window or embedded in the desktop shell.

## Rules

- Language in files: **English** (comments, code, variable names)
- Language in chat: **German**
- OOP everywhere: traits over match blocks, types carry their own behavior
- No CHANGELOG.md (removed for token savings)
- After every feature: commit directly

## Repository Structure

```
crates/
  fsd-shell/      → Desktop shell: taskbar, window manager, wallpaper
  fsd-container-app/ → Container/Service/Bot management (formerly "Conductor")
  fsd-store/      → Package manager (discovery, install, updates)
  fsd-studio/     → Plugin/Module/i18n creator (+AI optional)
  fsd-settings/   → System settings (appearance, language, service roles)
  fsd-profile/    → User profile
  fsd-app/        → Main launcher binary
```

## Library Dependencies (FreeSynergy.Lib)

All shared libraries live in `../FreeSynergy.Lib/`. Never duplicate their logic here.

| Library         | Purpose |
|---|---|
| `fsn-types`     | Resource/Capability traits, Meta, TypeRegistry |
| `fsn-error`     | FsnError, Repairable trait, ValidationIssue |
| `fsn-config`    | TOML loader/saver with backup + auto-repair |
| `fsn-i18n`      | Snippet-based i18n (t(), t_with()) |
| `fsn-theme`     | Theme system (theme.toml → TUI palette + CSS) |
| `fsn-help`      | Context-sensitive help topics |
| `fsn-health`    | Generic health check framework |
| `fsn-store`     | Universal store client |
| `fsn-container` | Container abstraction (Podman via bollard) |
| `fsn-plugin-sdk` | WASM plugin SDK |
| `fsn-auth`      | OAuth2 + JWT + Permissions |
| `fsn-core`      | FormAction, SelectionResult |

## Architecture

Desktop communicates with FreeSynergy.Node via:
- Direct library calls to `fsn-container` (Lib) for container operations
- `fsn` CLI subprocess for Node-specific operations
- Local SQLite database (shared, accessed via `fsn-db`)

Desktop does NOT import Node-internal crates directly.

## Window System

All dialogs and views are `Window` objects. See `fsd-shell/src/window.rs`.

## Layout System (E1 + E2)

### Shell Layout (fsd-shell)
CSS Grid desktop with four areas: `header | sidebar | main | taskbar`.

```
grid-template-areas: 'header header' 'sidebar main' 'taskbar taskbar';
grid-template-rows:  60px 1fr 48px;
grid-template-columns: {sidebar_width} 1fr;
```

- **ShellHeader** (`header.rs`): Brand + Breadcrumbs + AvatarMenu (60px)
- **ShellSidebar** (`sidebar.rs`): App navigation, collapsible 240px → 48px
- **Taskbar** (`taskbar.rs`): Slots via `TaskbarLauncherBtn`, `TaskbarSeparator`, `TaskbarApps`, `SystemTray`, `Clock`
- **WindowFrame** (`window_frame.rs`): Glassmorphism (backdrop-filter + rgba bg)

### App Layouts (app_shell.rs)
- `AppShell` — root wrapper, injects transition CSS, 3 modes: `Window | Standalone | Tui`
- `ScreenWrapper` — max-width + padding + scroll
- `LayoutA` — full-width column (fsd-store, fsd-studio)
- `LayoutB` — sidebar (master) + detail pane (fsd-container-app, fsd-settings)
- `LayoutC` — centered card (fsd-profile, login)

Page transitions: `.fsd-page-enter` (slideInRight), `.fsd-page-fade` (fadeInUp). Respects `prefers-reduced-motion`.

### SplitView (`split_view.rs`)
Resizable horizontal split: `SplitState::Collapsed | Half | FullRight`. Drag handle + double-click to cycle.

## Service Roles

Extended MIME-type system for functions (not files). See `fsd-settings/src/service_roles.rs`.
Example: `auth = "kanidm"`, `mail = "stalwart"`, `git = "forgejo"`.

## CSS Variables Prefix

Always `--fsn-` (e.g., `--fsn-color-primary`, `--fsn-font-family`).

## Branding

- "by KalEl" in header
- Cyan + White for FreeSynergy.Node colors
