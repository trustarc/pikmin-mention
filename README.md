# Pikmin Mention

Press one hotkey and drop a correctly-formatted mention into whatever app you are in.

Jira, Slack and GitHub each encode mentions differently — a Jira mention is an editor node keyed by account id, a Slack mention is a link to a user id, a GitHub mention is plain `@handle` markdown. Typing them by hand does not produce a real mention. This app detects the app or site you are in and puts the right payload on the clipboard.

Ships with packs for invoking [Pikmin](https://github.com/trustarc/pikmin), and you can capture any other mention straight from your clipboard.

## Install

1. Download your platform's build from [Releases](https://github.com/trustarc/pikmin-mention/releases)
2. Install it — drag into `/Applications` on macOS, run the installer on Windows, make the AppImage executable on Linux
3. On macOS, right-click the app and choose **Open** the first time (unsigned build)
4. Allow Accessibility when asked (macOS only)

No Dock icon — look for it in the menu bar or tray. Press `Cmd+Shift+.` to open
(`Ctrl+Shift+.` on Windows and Linux).

Browser detection needs X11 on Linux; Wayland does not expose the focused window.

## Usage

|                 |                             |
| --------------- | --------------------------- |
| `Cmd+Shift+.`   | Open or close the overlay   |
| type            | Filter                      |
| `↑` `↓`         | Move                        |
| `Enter`         | Insert the selected snippet |
| `Cmd+1`–`Cmd+9` | Insert by position          |
| `Tab`           | Switch pack                 |
| drag            | Reorder (pins it)           |
| `Esc`           | Close                       |

The pack matching your current app or site is selected automatically and marked
with a dot. `Default` is used when nothing matches. Add your own entries with
**Add custom** — including any mention captured from the clipboard.

## Releases

See [CHANGELOG.md](CHANGELOG.md). The version lives in `src-tauri/Cargo.toml` only —
`tauri.conf.json` and `package.json` fall back to it.

## Requirements

Toolchain versions are pinned in `mise.toml`:

```sh
mise trust
mise install
bun install
```

Node is not required — `bunfig.toml` sets `[run] bun = true`, so every script runs on the Bun runtime.

## Development

```sh
mise start
```

## Build

```sh
mise build
```

## Checks

```sh
mise check
```

## Privacy

The app reads the foreground application and, for supported browsers, the active tab URL. The URL is reduced to a hostname immediately and the full URL is discarded. Nothing is uploaded, no account is required, and custom entries never leave the machine.
