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

Prepare version changes through a pull request. Update `Cargo.toml`, run `cargo check` to refresh `Cargo.lock`, and commit both files. The first release can use the current `0.1.0` version.

After the pull request is merged and CI passes on `main`, push a matching version tag:

```sh
git switch main
git pull --ff-only
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

Replace `0.1.0` with the version being released. The Release workflow checks that the tag matches `Cargo.toml` and belongs to `main`, then rebuilds and checks the CLI on both Apple Silicon and Intel macOS runners. It publishes:

- `im-switch-X.Y.Z-aarch64-apple-darwin.tar.gz`
- `im-switch-X.Y.Z-x86_64-apple-darwin.tar.gz`
- `SHA256SUMS`

Each archive contains the executable, MIT license, and both README languages. All builds must succeed before GitHub Release publication. The workflow uses the built-in GitHub token and does not require a separate personal access token.

The Release workflow also runs a build-only rehearsal for pull requests that change the workflow, scripts, or Cargo files. It can be run manually from Actions for a build-only rehearsal. Only a pushed version tag publishes a release. Rehearsal assets are available in the Actions run.

Do not move published version tags. For a failed build before publication, fix the cause and rerun the failed workflow where appropriate; if the source must change, prepare a new version. Package definitions must refer to a fixed release and its checksum.

GitHub Release publication does not publish to Homebrew or crates.io. Homebrew distribution needs a Formula and a separate update step.

For the current `0.x` series, use a patch increment for compatible fixes and a minor increment for breaking CLI changes or larger feature releases. Keep changes to `--english` and `--status` backward compatible when possible.

If older versions eventually need parallel maintenance, create `release/X.Y` from the corresponding release tag. Otherwise, use `main` and short-lived branches.
