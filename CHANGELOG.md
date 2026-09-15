# Changelog

## [Unreleased]

## [1.0.11] - 2026-09-15

- Linux: go back to the exact window that was in front before pasting, rather than the first window the process owns.
- Linux: read the address only from an entry or combo box. The lookup took the first text anywhere in the browser that had a dot in it, so a link or an address written on the page could be reported as the current site.

## [1.0.10] - 2026-09-15

- Windows: clicking inside the overlay no longer closes it. The outside-click check scaled the window bounds down to logical points while the cursor already comes in physical pixels, so on any display above 100% every click counted as outside. The same applied to X11.
- Windows: open the overlay on the monitor under the cursor rather than the primary one.
- Go back to the pack matching the current app every time the overlay opens, instead of keeping a tab that was picked by hand. The reset alone was not enough: the tab clicked last got focus back when the overlay reopened and reactivated itself, so tabs now change only on click. Clicking a tab also returns focus to the search box.
- Windows: keep only the copied fragment when capturing a mention from the clipboard. The stored HTML carried the whole CF_HTML document, which then got wrapped in another one on paste.
- Windows: return to the exact window that was in front before pasting, rather than the first visible window of that process.
- Windows: read the browser address from the window that was in front. The lookup took the first window owned by the browser process, which in Chrome is a hidden helper with no address bar, so Jira and GitHub pages fell back to the default pack. Firefox-style combo box address bars are accepted too.

## [1.0.9] - 2026-09-15

- Read the address again in Firefox. Its URL bar is a combo box, not a text field, so the lookup found nothing and every page fell back to the default pack.

## [1.0.8] - 2026-09-15

- Share the CI dependency cache across versions. The key included the manifest hash, so every release missed the cache it had just written.

## [1.0.7] - 2026-09-15

- Mark the tray icon with a dot when an update is available, checked once at startup.
- Retry enabling Launch at Login if the first attempt fails, instead of marking it done.

## [1.0.6] - 2026-09-15

- **Launch at Login** in the tray menu, on by default.
- Offer a restart after granting Accessibility, which macOS only reads at launch.

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
