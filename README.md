# Pikmin Mention

Press one hotkey and drop a correctly-formatted mention into whatever app you are in.

Jira, Slack and GitHub each encode mentions differently, so typing one by hand does not produce a real mention. This app detects where you are and puts the right payload on the clipboard.

## Install

Download your platform's build from [Releases](https://github.com/trustarc/pikmin-mention/releases). There is no Dock icon — the app lives in the menu bar or tray.

### macOS

1. Open the `.dmg` and drag the app into Applications
2. Run this once — the build is unsigned, so macOS blocks it otherwise:

   ```sh
   xattr -dr com.apple.quarantine "/Applications/Pikmin Mention.app"
   ```

3. Open the app and allow Accessibility when asked

### Windows

Run the installer. On the SmartScreen warning choose **More info → Run anyway**.

### Linux

```sh
chmod +x Pikmin.Mention_*.AppImage
./Pikmin.Mention_*.AppImage
```

Browser detection needs X11.

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
mise format     # prettier + rustfmt
mise test       # typecheck, lint, tests
mise build      # bundle
```

## Releases

Bump `version` in `src-tauri/Cargo.toml`, commit to `main`, then:

```sh
mise publish
```

## Privacy

Browser URLs are reduced to a hostname on read and never stored. Custom entries stay on your machine.
