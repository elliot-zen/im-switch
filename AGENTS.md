# Agent instructions

These instructions apply to the entire repository. See `CONTRIBUTING.md` for the contributor-facing workflow.

## Branch management

- Keep `main` ready to release. Do not commit or push changes directly to `main`.
- Start each independent change on a short-lived branch from the latest `origin/main`. Continue the existing branch when updating the same pull request.
- Use `feat/<name>` for features, `fix/<name>` for fixes, `docs/<name>` for documentation, and `chore/<name>` for maintenance, CI, or packaging.
- Open pull requests against `main`. Wait for all relevant checks to pass and resolve review conversations before merging.
- Use squash merge and delete the merged working branch. A separate approving reviewer is not required while there is one maintainer.
- Do not force-push or delete `main`, bypass its protection, or disable checks to make a change mergeable.
- The required checks are `macOS (macos-15)` and `macOS (macos-15-intel)`. Keep these job names aligned with GitHub branch protection when changing CI.
- A permanent `develop` branch is not part of the current workflow. Add maintenance branches only when older release lines need separate support.

## Validation

Before pushing code or workflow changes, run the applicable checks:

```sh
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
python3 scripts/check_cli.py
git diff --check
```

For documentation-only changes, check content, links, and consistency with the workflows; do not repeat builds solely for prose changes.

CI checks compilation and CLI behavior on Apple Silicon and Intel macOS. Actual input source changes require a logged-in macOS GUI session; do not claim that CI validates desktop switching.

## Versions

- `Cargo.toml` is the source of the application version. Commit `Cargo.lock` and keep it in sync.
- Make version changes through a pull request. After editing the version, run `cargo check` to refresh `Cargo.lock` and commit both files.
- During `0.x`, use patch increments for compatible fixes and minor increments for breaking CLI changes or larger feature releases.
- Stable release tags must be exactly `vX.Y.Z`, matching the package version. Prerelease versions are not supported by the current release workflow.

## Release procedure

1. Merge the release preparation pull request and confirm that CI passes on the intended `main` commit.
2. Update the local `main` with `git pull --ff-only` and confirm the working tree is clean.
3. Create an annotated version tag on that tested commit and push that tag to `origin`.
4. Watch the Release workflow and verify its result and uploaded assets before reporting publication as complete.

Example for the first release; replace `0.1.0` for later releases:

```sh
git switch main
git pull --ff-only
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

`.github/workflows/release.yml` validates the version and requires the tagged commit to be contained in `main`. It builds and checks both macOS targets, packages the binaries and documentation, generates checksums, and creates the GitHub Release only after all preceding jobs succeed.

Expected assets:

- `im-switch-X.Y.Z-aarch64-apple-darwin.tar.gz`
- `im-switch-X.Y.Z-x86_64-apple-darwin.tar.gz`
- `SHA256SUMS`

Each archive contains `im-switch`, `LICENSE`, `README.md`, and `README.zh-CN.md`.

## Release safeguards and distribution

- Use the pull request rehearsal or a manual Actions run to test release automation. These runs build artifacts without publishing; do not push a real version tag merely to test CI.
- Do not move, delete, or reuse published version tags, or replace published release assets. Publish a new version when the source or released binaries must change.
- If a workflow fails before publication, inspect the failure and rerun the failed jobs when no source change is needed. Prepare a new version if the source must change.
- The current workflow supports releases from `main` history only. Adapt and validate the release workflow before publishing from a separate maintenance branch.
- GitHub Release publication does not publish to Homebrew, crates.io, AUR, or APT. Treat each distribution channel as a separate release step and use fixed versions and checksums.
- Keep these instructions, `CONTRIBUTING.md`, and the workflows consistent when changing branch or release policy.
