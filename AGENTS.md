# Pedagogue

Pedagogue is a "living textbook" desktop app: a textbook page on the left and a tutor chat on the right. A model fills the page through a small set of page commands. This repository holds a Rust workspace (Tauri 2 and an in-process Axum server) and a TypeScript UI that the server delivers over loopback.

Runvil is the team's agent framework, spelled "Runvil". A Host is an app that runs prompts through it. The student is the person learning, and the tutor is the model.

## Standing rules

- **Built to change.** The quiz rules, the wording, the look and the command list will all change during user trials. Keep each kind of decision in one place: quiz rules in `crates/pedagogue-session/src/lesson/quiz_rules.rs`, text a model reads in each crate's `wording.rs`, text the student reads in `ui/src/wording.ts`, and the look in the CSS file beside each component.
- **Rust decides, the UI draws.** Page state and every quiz rule live in Rust. The UI sends what the student did and draws the state it gets back. The answer key stays in Rust.
- **Match the design.** `design/page-contact-sheet.html` and `design/chat-panel.html` are the reference, and the app must match them. A change to the look goes into the design file and the app in the same commit.
- **UI import rule.** `ui/src/page/` and `ui/src/chat/` import the shared files and never each other. Shared files never import a feature. `ui/src/main.ts` alone wires the two.
- **Comments** explain a non-obvious reason: a constraint, an ordering requirement or a workaround.
- **Formats change freely.** Lesson state lives in memory, so every run starts from the current shapes. Change frames, commands and APIs freely, as long as the Rust types, `ui/src/protocol.ts` and the fixtures move together.
- **Messages a model may read** (receipts, refusals and reports of student actions) are short and say what to fix.

## Rulebooks

Four rulebooks are the law for their languages. Their full text lives beside this repository, in the workspace folder `tools-public/rulebooks/`. Load a rulebook before writing or reviewing its language. Its binding rules are restated here so this file stands alone.

### Rust (`rust-rulebook.md`)

- Fix a borrow error by restructuring ownership, never with `unsafe`.
- Return `Result` for every expected failure. A panic means a bug.
- Set lint levels in the `[lints]` tables of `Cargo.toml`.
- Document every public item, with `# Errors`, `# Panics` and `# Safety` where they apply, and write each example as a doctest.
- Add a test in the same change as the code: unit tests in the file under test, and tests of the public API in one binary at `tests/it/main.rs`.
- Run `cargo fmt --all --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` before every commit.
- Confirm that a crate is the intended, existing package before adding it to `Cargo.toml`.

Departures: the license is BSL-1.0. `serde` is a plain dependency of the session crate. Public enums go without `#[non_exhaustive]` while every user lives in this workspace, so a new variant stops the build until each user handles it. Golden files are plain JSON with an update switch, because the UI tests read the same files.

### TypeScript (`typescript-rulebook.md`), for `ui/src`

- `strict`, `noUncheckedIndexedAccess` and `verbatimModuleSyntax` stay on in `tsconfig.json`.
- Keep `any` out. Data from outside arrives as `unknown` and passes a schema before use.
- Write a closed set as an `as const` object or a union, in place of `enum`.
- Import each file by its own path, in place of barrel files.
- Use `import type` for type-only imports.
- State the return type of every exported function.

Departures: oxlint with its type-aware mode is the linter, because the UI uses TypeScript 7 and typescript-eslint supports TypeScript below 6.1. Tests run on Node's own test runner against the built bundle. Imports are relative.

### JavaScript (`javascript-rulebook.md`), for the `.mjs` files and the scripts in the design files

- `const` by default, `let` when reassigned, never `var`.
- `===` and `!==`. The one exception is `x == null`.
- Throw `Error` objects.
- Await every promise, or detach it on purpose with `void` and a `catch`.
- Keep untrusted data out of `innerHTML`, `eval` and `new Function`. `ui/src/markdown.ts` is the one file that sets `innerHTML`, after DOMPurify.
- ESM with full file names and the `node:` prefix for built-ins.

### HTML and CSS (`html-css-rulebook.md`), for the design files, `ui/index.html`, the CSS files and the markup the app draws

- Semantic elements before `div` and `span`.
- One `h1`, heading levels that step by one, and a single `main`.
- `button` for actions and `a href` for navigation.
- Every image has `alt`, `width` and `height`, and every form control has a `label`.
- Every control shows a `:focus-visible` ring, and text meets 4.5:1 contrast.
- Native HTML before ARIA.

Departures: colors stay in hex, the form the palette was approved in. The app has one layout and one palette: two panes side by side, a white page and a dark chat. The design files stay hand-formatted, with their framing styles and demo scripts inline.

## Checks

- Rust, from the repository root: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --locked --workspace --all-features`, and `cargo doc --workspace --no-deps --all-features` with `RUSTDOCFLAGS` set to `-D warnings`.
- UI, from `ui/`: `npm run check`, which runs the type check, oxlint, Stylelint, Prettier and the tests.
- Dependencies, from the repository root: `cargo deny check` and `cargo audit`. `deny.toml` holds the allowed licenses and sources, and an accepted advisory goes into its `ignore` list with the reason.
- CI runs all of these on every pull request and on `master`, in `.github/workflows/ci.yml`. A job added there also goes into the `needs` list of `ci-green`, the one check to require.
