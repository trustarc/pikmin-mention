# Changelog

## [Unreleased]

## [1.0.1] - 2026-09-15

- Re-check Accessibility permission each time the overlay opens, so the prompt clears once you grant it.
- Cache Rust dependencies in CI and relax LTO, cutting release builds from roughly nine minutes to a few.

## [1.0.0] - 2026-09-14

macOS, Windows and Linux.

- Global hotkey (`Cmd+Shift+.`) opens a context-aware overlay; configurable and resettable.
- Detects the foreground app and the browser hostname before showing.
- Packs for Jira, GitHub, Slack, plus `Default` when nothing matches.
- Mentions encoded per platform and placed on the clipboard as HTML.
- Pikmin snippets for 4 models × 5 effort levels.
- Custom entries, including capturing any mention from the clipboard.
- Pin and drag to reorder, `Cmd+1`–`Cmd+9`, search, keyboard navigation, last-used memory.
- Auto-paste into the previous app when Accessibility permission is granted.
- Tray icon.
- URLs are reduced to a hostname on read and never stored; custom entries stay local.
