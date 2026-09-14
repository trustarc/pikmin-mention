# Pikmin Shortcut

Context-aware keyboard shortcut cheat sheet for macOS and Windows.

Press one hotkey and see the shortcuts for whatever app or site you are currently in — Jira in Chrome shows Jira shortcuts, Slack shows Slack shortcuts, no manual switching.

## Requirements

Toolchain versions are pinned in `mise.toml`:

```sh
mise install
```

This provides Bun and Rust. Node is not required — `bunfig.toml` sets `[run] bun = true`, so every script runs on the Bun runtime.

## Development

```sh
bun install
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

The app reads the foreground application and, for supported browsers, the active tab URL. The URL is reduced to a hostname immediately and the full URL is discarded. Nothing is uploaded, no account is required, and custom shortcuts never leave the machine.
