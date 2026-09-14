# Pikmin Mention

Press one hotkey and drop a correctly-formatted mention into whatever app you are in.

Jira, Slack and GitHub each encode mentions differently — a Jira mention is an editor node keyed by account id, a Slack mention is a link to a user id, a GitHub mention is plain `@handle` markdown. Typing them by hand does not produce a real mention. This app detects the app or site you are in and puts the right payload on the clipboard.

Ships with packs for invoking [Pikmin](https://github.com/trustarc/pikmin), and you can capture any other mention straight from your clipboard.

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

## Permissions

macOS Accessibility permission is needed to paste automatically and to read the address bar in Firefox. Without it the snippet still reaches the clipboard and you paste with `Cmd+V`.

## Privacy

The app reads the foreground application and, for supported browsers, the active tab URL. The URL is reduced to a hostname immediately and the full URL is discarded. Nothing is uploaded, no account is required, and custom entries never leave the machine.
