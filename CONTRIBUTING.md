# Contributing

## Branches

Keep `main` ready to release. Create a short-lived branch from the latest `main` for each change:

- `feat/<name>` for new functionality.
- `fix/<name>` for bug fixes.
- `docs/<name>` for documentation.
- `chore/<name>` for maintenance, CI, and packaging.

Open a pull request against `main`, wait for CI to pass, and squash merge the change. Delete the branch after merging. A separate approving reviewer is not required while the project has one maintainer.

## Validation

Run these checks before opening a pull request:

```sh
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
./target/release/im-switch --help
```

Verify input source changes in a logged-in macOS GUI session. CI checks builds and CLI behavior; it does not establish that switching works in a desktop session.

Keep source code, comments, and CLI output in English. Update both README languages when behavior changes. Project code must continue to forbid `unsafe`.

## Versions and releases

`Cargo.toml` is the source of the application version. Keep `Cargo.lock` committed and in sync.

Prepare a release through a pull request, then create an immutable `vX.Y.Z` tag on the tested `main` commit and publish the matching GitHub Release. Do not move published version tags. Package definitions must refer to a fixed release and its checksum.

For the current `0.x` series, use a patch increment for compatible fixes and a minor increment for breaking CLI changes or larger feature releases. Keep changes to `--english` and `--status` backward compatible when possible.

If older versions eventually need parallel maintenance, create `release/X.Y` from the corresponding release tag. Otherwise, use `main` and short-lived branches.
