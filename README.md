# Pikmin Mention

Press one hotkey and drop a correctly-formatted mention into whatever app you are in.

Jira, Slack and GitHub each encode mentions differently, so typing one by hand does not produce a real mention. This app detects where you are and puts the right payload on the clipboard.

## Install

Download your platform's build from [Releases](https://github.com/trustarc/pikmin-mention/releases). There is no Dock icon — the app lives in the menu bar or tray.

**macOS** — builds are unsigned, so clear the quarantine flag after moving the app into `/Applications`, then allow Accessibility when asked:

```sh
xattr -dr com.apple.quarantine "/Applications/Pikmin Mention.app"
```

**Windows** — run the installer and choose **More info → Run anyway** on the SmartScreen warning.

**Linux** — `chmod +x` the AppImage and run it. Browser detection needs X11.

## Usage

On Windows and Linux use `Ctrl` wherever this says `Cmd`.

|                 |                    |
| --------------- | ------------------ |
| `Cmd+Shift+.`   | Open or close      |
| type            | Filter             |
| `↑` `↓` `Enter` | Select and insert  |
| `Cmd+1`–`Cmd+9` | Insert by position |
| `Tab`           | Switch pack        |
| drag            | Reorder            |

The pack matching your current app or site is selected automatically. Add your own entries with **Add custom**, including any mention captured from the clipboard.

## Development

```sh
mise install
bun install
mise start      # run
mise check      # typecheck, format, test, clippy
mise build      # bundle
```

## Releases

Bump `version` in `src-tauri/Cargo.toml`, commit to `main`, then:

```sh
mise publish
```

## Privacy

Browser URLs are reduced to a hostname on read and never stored. Custom entries stay on your machine.
