# Pikmin Mention

Press one hotkey and drop a correctly-formatted mention into whatever app you are in.

Jira, Slack and GitHub each encode mentions differently, so typing one by hand does not produce a real mention. This app detects where you are and puts the right payload on the clipboard.

## Install

Grab your platform's build from [Releases](https://github.com/trustarc/pikmin-mention/releases).

There is no Dock icon on any platform — the app lives in the menu bar or tray. Press `Mod+Shift+.` to open it.

### macOS

Builds are unsigned, so Gatekeeper blocks anything downloaded in a browser. Downloading from the command line avoids that entirely:

```sh
gh release download --repo trustarc/pikmin-mention --pattern '*.dmg' --clobber
MNT=$(hdiutil attach Pikmin.Mention_*.dmg -nobrowse | grep -o '/Volumes/.*')
ditto "$MNT/Pikmin Mention.app" "/Applications/Pikmin Mention.app"
hdiutil detach "$MNT"
open "/Applications/Pikmin Mention.app"
```

If you already downloaded it in a browser, clear the quarantine flag instead:

```sh
xattr -dr com.apple.quarantine "/Applications/Pikmin Mention.app"
```

Allow Accessibility when asked — it lets the app paste into whatever you were using. Without it snippets still reach the clipboard and you paste with `Cmd+V`.

### Windows

Run the `.exe` installer. SmartScreen will warn that the publisher is unknown; choose **More info → Run anyway**. No extra permissions are needed.

### Linux

Make the AppImage executable and run it:

```sh
chmod +x Pikmin.Mention_*.AppImage
./Pikmin.Mention_*.AppImage
```

Browser detection needs X11. On Wayland the focused window is not exposed, so pick a pack manually.

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
