# Contributing

## Setup

1. Install the toolchain that the README lists under [Toolchain](README.md#toolchain).
2. Run `npm ci` in `ui/`. The server crate bundles the UI when it builds, so the Rust workspace needs those packages too.
3. Turn on the git hooks, once per clone, with `git config core.hooksPath .githooks`. The pre-commit hook checks the formatting of the Rust and the UI sources. The pre-push hook runs clippy, and `cargo deny` when it is installed.

## The checks

The exact commands are in the README, under [Check it](README.md#check-it). CI runs them on every pull request, on Linux, Windows and macOS, and adds the Nu HTML Checker. `ci-green` is the one check a pull request has to pass.

A pull request that touches the installer tool, the launch check, the version or the changelog also builds all five installers and starts each app.

## The rules

- [AGENTS.md](AGENTS.md) holds the standing rules of the repository and the binding rules of its four rulebooks, for Rust, TypeScript, JavaScript, and HTML with CSS.
- [ARCHITECTURE.md](ARCHITECTURE.md) explains how the parts fit together and which invariants hold.
- A test goes into the same change as the code.
- A change to the look goes into the design file and into the app in the same commit.
