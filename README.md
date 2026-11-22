# Pedagogue

Pedagogue is a "living textbook": a textbook page on the left and a tutor chat on the right. A model fills the page through a small set of page commands and talks with the student in the chat. The app is a Tauri 2 desktop window on an in-process Axum server, which delivers a TypeScript UI over loopback.

## Check it

From the repository root:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --workspace --no-deps --all-features
```

Set `RUSTDOCFLAGS` to `-D warnings` for the docs build. A plain `cargo test` covers the two crates that need only Rust.

From `ui/`:

```text
npm install
npm run check
```

`npm run check` runs the type check, oxlint, Stylelint, Prettier and the tests. `npm test` builds the bundle first.

## Sample scripts

A sample script is a JSON list of steps: page commands, chat lines, scripted student answers and a wait for Continue. The doc comment on `Script` in `crates/pedagogue-demo/src/script.rs` lists the shapes.

`samples/contact-sheet.frames.json` is written from `samples/contact-sheet.json`, and a test fails when the two disagree. After a change to the sample or to the wire shapes, set the environment variable `PEDAGOGUE_UPDATE_FIXTURES` to `1`, run `cargo test -p pedagogue-demo`, and review the diff of the file.

## Layout

- `design/` - the two design reference files. The app must match them.
- `crates/pedagogue-session/` - the lesson model, the chat transcript and the session handle that a host drives.
- `crates/pedagogue-demo/` - a sample script player and a stand-in tutor.
- `crates/pedagogue-server/` - the loopback server that delivers the UI and its socket.
- `crates/pedagogue-desktop/` - the Tauri app. Its binary is named `pedagogue`.
- `ui/` - the TypeScript UI, bundled by esbuild.
- `samples/` - sample scripts, as JSON. `straw-poll.json` is the lesson the app plays by default. `contact-sheet.json` draws every element of the two design files on one page, and `contact-sheet.frames.json` holds the frames the app sends for it.

`ARCHITECTURE.md` explains how the parts fit together, and `AGENTS.md` holds the rules for working in this repository.

## Toolchain

- Rust 1.99, pinned in `rust-toolchain.toml`.
- Node 24.15 or newer, with npm.
- The Tauri 2 prerequisites for your platform, for the desktop crate.

## License

Boost Software License 1.0. See `LICENSE`.
