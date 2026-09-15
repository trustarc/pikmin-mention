# Pikmin Mention

Press one hotkey and drop a correctly-formatted mention into whatever app you are in.

Jira, Slack and GitHub each encode mentions differently, so typing one by hand does not produce a real mention. This app detects where you are and puts the right payload on the clipboard.

## Install

Download your platform's build from [Releases](https://github.com/trustarc/pikmin-mention/releases).

On macOS, right-click the app the first time and choose **Open** (unsigned), then allow Accessibility.

There is no Dock icon — it lives in the menu bar or tray.

## Usage

`Mod` is `Cmd` on macOS and `Ctrl` elsewhere.

|                 |                    |
| --------------- | ------------------ |
| `Mod+Shift+.`   | Open or close      |
| type            | Filter             |
| `↑` `↓` `Enter` | Select and insert  |
| `Mod+1`–`Mod+9` | Insert by position |
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
