<img src="static/app-icon.svg" alt="FocusIsland logo" width="72" height="72">

# FocusIsland

FocusIsland is a lightweight, offline-first productivity island for Windows. It lives at the top center of a monitor as a compact timer pill and expands into a focused workspace for tasks, countdowns, stopwatch sessions, daily notes, reminders, insights, and appearance settings.

FocusIsland is free and open-source software released under the MIT License. It has no accounts, cloud dependency, analytics, or telemetry.

[Download the latest Windows release](https://github.com/vishvajeet2012/FocusIsland/releases/latest)

The product name and shared brand icon path shown in the frontend are isolated in [`src/lib/constants.ts`](src/lib/constants.ts). The original SVG source of truth is [`static/app-icon.svg`](static/app-icon.svg), with native icon sizes generated from it. Native runtime strings are grouped in `src-tauri/src/constants.rs`, while installer identity metadata lives in `src-tauri/tauri.conf.json` as required by Tauri.

## Features

- Top-center, borderless, transparent, always-on-top Windows utility window
- Per-monitor work-area positioning with DPI-aware sizing from 100% through 200%+
- Hybrid island animation: one native host resize per transition plus a CSS inner-shell morph
- Task create, edit, complete, restore, delete, duplicate, archive, reminder, focus, and drag reorder flows
- Drift-free countdown and stopwatch modes calculated from persisted timestamps
- Timer recovery after collapse, app restart, Windows sleep, and wake
- Daily notepad with 450 ms autosave and `Ctrl + Enter` line-to-task conversion
- Local reminder scheduler and native Windows toast notifications
- SQLite-backed focus sessions and real seven-day insights
- Custom colors, theme, island behavior, monitor policy, motion level, notifications, time format, and Windows startup settings
- Native tray menu, three global shortcuts, and compact quick-task island
- No analytics, telemetry, accounts, cloud calls, or normal-operation network dependency

## Download

Open the [latest release](https://github.com/vishvajeet2012/FocusIsland/releases/latest) and choose:

- `FocusIsland_0.1.0_x64-setup.exe` for the recommended per-user Windows installer.
- `FocusIsland.exe` for a portable launch without installation.
- `SHA256SUMS.txt` to verify either download.

FocusIsland currently targets 64-bit Windows 10/11 and requires the Microsoft Edge WebView2 Runtime. Release binaries are not yet code-signed, so Windows SmartScreen may ask for confirmation.

## Tech stack

- Tauri 2 and Rust
- Svelte 5, strict TypeScript, SvelteKit static adapter, and Vite
- Plain CSS and original SVG branding/icons
- SQLite through `rusqlite` with bundled SQLite
- Tauri notification, global-shortcut, and autostart plugins
- `windows-rs` for Windows monitor work areas and effective DPI

No Electron runtime, UI framework, animation library, chart package, date library, icon pack, or background server is used.

## Development setup

Requirements: Windows 10/11, the Edge WebView2 Runtime, Node.js 20+, Rust stable with the MSVC target, and Visual Studio 2022 Build Tools with Desktop development with C++ plus a Windows SDK.

```powershell
cd C:\path\to\FocusIsland
npm install
npm run tauri dev
```

Frontend-only validation:

```powershell
npm run check
npm run build
npm run preview
```

## Production build

```powershell
npm install
npm run check
npm run build
cargo test --manifest-path src-tauri\Cargo.toml
npm run tauri build
```

The configured target is a per-user NSIS installer. Tauri writes it to `src-tauri\target\release\bundle\nsis\`. If `CARGO_TARGET_DIR` is set, the same `release\bundle\nsis` path is created below that directory.

Validated release copies from the current build are available in `artifacts\`: the NSIS installer and a standalone executable. Production distribution should sign both with the publisher's Windows code-signing certificate.

## Keyboard shortcuts

| Action | Shortcut |
| --- | --- |
| Toggle workspace | `Ctrl + Alt + Space` |
| Quick add task | `Ctrl + Alt + T` |
| Start / pause / resume timer | `Ctrl + Alt + P` |
| Focus quick task input | `Ctrl + N` |
| Convert current note line to a task | `Ctrl + Enter` |
| Collapse / close transient UI | `Esc` |
| Toggle timer when its card is focused | `Space` |

Global shortcut registration failures are non-fatal and appear in Customize.

## Local data

The SQLite database is stored at `%APPDATA%\com.focusisland.desktop\focusisland.sqlite3`.

SQLite runs in WAL mode with foreign keys, normal synchronous durability, a busy timeout, and indexed deadline/date queries. A storage failure falls back to an in-memory database and surfaces a warning instead of producing a blank screen.

- `tasks` stores task text, status, ordering, due/reminder metadata, focus estimate, and archive timestamps.
- `daily_notes` stores one upserted note per local calendar date.
- `focus_sessions` stores completed countdown/stopwatch records used by Insights.
- `reminders` stores pending and fired local deadlines.
- `settings` stores validated JSON preferences under one key.
- `timer_state` stores one durable timer with start, pause, accumulated-pause, mode, target, and task context.
- `schema_migrations` tracks applied schema versions.

## Architecture

```text
src/
  components/        island, workspace, task, timer, note, event, and common UI
  stores/            small Svelte stores for predictable UI state
  lib/               typed native bridge, time math, formatting, constants, motion helpers
  types/             strict frontend domain types
  views/             Workspace, Insights, and Customize

src-tauri/src/
  commands/          narrow, validated Tauri command boundary
  db/                schema and domain repositories
  windows/           DPI-aware work-area positioning and window operations
  notifications/     native notification construction
  shortcuts/         registration, failure reporting, and event routing
  tray/              native menu and double-click handling
  scheduler.rs       condition-variable deadline scheduler
```

The frontend never executes SQL. Rust validates every command input before it reaches SQLite.

### Window animation strategy

Expansion does not resize the native window on every frame. Rust first enlarges and re-centers the transparent host once. On the next two animation frames, the inner shell morphs from the existing pill dimensions to the workspace with `width`, `height`, `border-radius`, `opacity`, and small translations. Collapse reverses the inner animation and shrinks the native host after it finishes. This avoids high-frequency IPC and Win32 resize jank while preserving the impression of one continuous object.

Motion variables are centralized in `src/app.css`; Customize and `prefers-reduced-motion` can reduce or disable transitions.

### Timer and scheduler behavior

The UI never treats an interval counter as the source of truth. Remaining or elapsed time is calculated from `reference_time - started_at - accumulated_pause_ms`. The one-second UI timeout is aligned to wall-clock boundaries and exists only while a timer is running. Native reminders and countdown completion use a condition variable that sleeps until the nearest persisted deadline and wakes only when a deadline changes. After suspend/resume it immediately reconciles against system time.

## Verification

Repository tests cover task persistence and completion, daily note restoration, settings round trips, reminder persistence, timer timestamp reconciliation, pause exclusion, focus-session creation, and Insights updates. The production checklist verifies task/note/settings persistence, completion-driven Insights, collapse/restart/sleep timer correctness, native reminders, shortcuts on a second monitor, motion-off behavior, and tray process exit.

## Privacy and security

FocusIsland is entirely local. It makes no analytics or telemetry calls and has no account or cloud subsystem. Tauri capabilities expose only core window/event functionality; all mutation goes through explicit commands with length, range, enum, date, and color validation. No shell execution API is exposed to the webview.

## Contributing

Issues, bug reports, and pull requests are welcome. Before opening a pull request, run:

```powershell
npm install
npm run check
npm run build
cargo test --manifest-path src-tauri\Cargo.toml
```

Please keep new dependencies small, preserve offline-first behavior, and avoid adding telemetry or cloud requirements.

## License

FocusIsland is licensed under the [MIT License](LICENSE).

Copyright (c) 2026 Vishvajeet Shukla.
