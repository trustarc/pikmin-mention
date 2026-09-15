# Changelog

## [Unreleased]

## [1.0.6] - 2026-09-15

- **Launch at Login** in the tray menu starts the app with macOS, Windows or your Linux session.
- Offer a restart after granting Accessibility. macOS reads the permission only when a process launches, so the prompt stayed up until the app was restarted.

## [1.0.5] - 2026-09-15

- Fix the update feed. macOS was missing from it because the build only produced a `.dmg`, the platform keys did not match what the app looks for, and the asset URLs used the local filenames rather than the names GitHub assigns.

## [1.0.4] - 2026-09-15

- Attach release assets as files. The previous run uploaded artifacts that kept their directory structure, so the release step tried to upload a directory and failed.

## [1.0.3] - 2026-09-15

- Sign the macOS app while it is bundled, so the `.dmg` carries the signature too. The previous release signed the app after the `.dmg` was already built, which had no effect.

## [1.0.2] - 2026-09-15

- Update from inside the app: **Check for Updates…** in the tray downloads and installs a signed release, then restarts.
- Show the version in the overlay header.

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
