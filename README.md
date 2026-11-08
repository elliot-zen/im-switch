# im-switch

English | [简体中文](README.zh-CN.md)

A small macOS CLI that switches to English before starting terminal apps.

## Install

Homebrew installation is planned. The install command will be added when the formula is published.

Once a version is published, prebuilt archives are available on [GitHub Releases](https://github.com/elliot-zen/im-switch/releases): choose `aarch64-apple-darwin` for Apple Silicon or `x86_64-apple-darwin` for Intel. Extract the archive and install `im-switch` into a directory on your `PATH`.

<details>
<summary>Build from source</summary>

For local builds, use Rust 1.88+ and Xcode Command Line Tools. Run from the project directory and ensure `~/.local/bin` is in your `PATH`.

```sh
cargo build --release --locked
mkdir -p "$HOME/.local/bin"
install -m 755 target/release/im-switch "$HOME/.local/bin/im-switch"
```

</details>

## Usage

Enable ABC or U.S. in System Settings > Keyboard, then run:

```sh
im-switch --english  # Switch to ABC or U.S.
im-switch --status   # Print the current input source ID
im-switch --help
```

`--english` prefers ABC, then U.S., and confirms the selection before returning success. Repeated calls keep English selected. No Accessibility permission is required.

With no arguments, the tool shows help without changing the input source.

## Herdr: lazygit & yazi

Add these popup bindings to `~/.config/herdr/config.toml`, or update existing bindings with the same keys. Ensure all three commands are on Herdr's `PATH`.

```toml
[[keys.command]]
key = "prefix+t"
type = "popup"
command = "im-switch --english && exec lazygit"
description = "Open Lazygit in English"
width = "90%"
height = "90%"

[[keys.command]]
key = "prefix+y"
type = "popup"
command = "im-switch --english && exec yazi"
description = "Open Yazi in English"
width = "90%"
height = "90%"
```

Validate and reload:

```sh
herdr config check
herdr server reload-config
```

`&&` starts the app only after switching succeeds. The input source stays English after the popup closes.

## Notes

- This changes the system input source, not an IME's internal mode such as Squirrel's `ascii_mode`.
- Project code forbids `unsafe` and uses safe `cidre` APIs. Dependencies may use unsafe code internally.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the branch workflow and release process. Licensed under [MIT](LICENSE).
