# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-04-07

### Added
- `CHANGELOG.md` to track project history going forward

### Changed
- Version bumped to 0.4.0

---

## [0.3.1] - 2026-01-17

### Fixed
- Conditional compilation guard for `itertools` import (cross-platform correctness)
- CI matrix strategy syntax errors
- Windows build compatibility

### Changed
- CI workflow now tests on Linux, macOS, and Windows via matrix strategy
- Added `workflow_dispatch` trigger to CI workflow

---

## [0.3.0] - 2026-01-11

### Added
- **F7 history popup** — opens a scrollable, selectable list of previous commands; navigate with Up/Down, select with Enter or mouse click, dismiss with Escape or F7 again
- `ConsoleBuilder::history_popup_key` — configure which key opens the popup, or pass `None` to disable it entirely
- `ConsoleBuilder::scrollback_size` — limit the number of lines retained in the scrollback buffer (default 1000)
- `ConsoleWindow::clear` — programmatically clear the console output
- `ConsoleWindow::clear_history` — programmatically clear the command history
- Support for multiple simultaneous `ConsoleWindow` instances (each gets a unique egui `Id`)

### Changed
- Updated to **egui / eframe 0.33.3**
- Switched to **Rust edition 2024**

---

## [0.2.0] - 2024-09-27

### Added
- **Tab completion for filesystem paths** — complete file and directory names inline
- **Tab completion for commands** — complete the first token against a user-supplied command table (`command_table_mut`)
- `ConsoleBuilder::tab_quote_character` — choose the quote character wrapped around completed paths that contain spaces (default `'`)
- Cycling through multiple completions by pressing Tab repeatedly
- Windows-compatible filesystem tab completion

---

## [0.1.0] - 2024-08-06

### Added
- Initial public release
- `ConsoleWindow` — an egui/eframe widget that provides an interactive console pane
- `ConsoleBuilder` — fluent builder for `ConsoleWindow` (prompt text, history size)
- **Command history** — Up/Down arrow navigation through previous commands
- **Ctrl-R incremental reverse search** — live substring search through history
- `ConsoleEvent::Command` — returned from `draw()` when the user submits a command
- `ConsoleWindow::write` — write a line of output to the console
- `ConsoleWindow::prompt` — re-issue the prompt after handling a command
- `ConsoleWindow::load_history` / `ConsoleWindow::get_history` — manual history save/restore
- Optional **`persistence` feature** — automatically persists command history via eframe storage between sessions

[0.4.0]: https://github.com/pm100/egui_console/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/pm100/egui_console/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/pm100/egui_console/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/pm100/egui_console/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/pm100/egui_console/releases/tag/v0.1.0
