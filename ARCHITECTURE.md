# Architecture

## Bird's eye view

Pedagogue shows a textbook page beside a tutor chat. A host (a model, or the sample script in this build) fills the page through page commands and writes into the chat. The student reads, answers quizzes and types messages. Everything the student does goes back to the host as one stream of events.

```mermaid
flowchart LR
  Host["Host"] -->|"commands, chat"| Session["Session"]
  Session -->|"student events"| Host
  Session -->|"page and chat frames"| Server["Server"]
  Server -->|"student frames"| Session
  Server -->|"one socket"| Ui["UI"]
  Desktop["Desktop app"] -->|"window on loopback"| Ui
```

The session is the one place that holds state. A host changes it through a handle, the server sends each open page what changed, and the UI draws what it is sent.

## Code map

- `crates/pedagogue-session/` - the lesson model, the quiz rules, the chat transcript, the wire frames and the `Session` handle. It is the one crate a host depends on, and it stays free of Axum and Tauri.
  - `src/lesson.rs` holds the `Lesson`, its fold `Lesson::apply`, and the calls a student's actions go through. Its children are plain data and logic: `command.rs` (one argument struct per page command), `block.rs` (the blocks those arguments become), `quiz.rs` (quiz blocks and the part of them the UI receives), `quiz_rules.rs` (the quiz rules, as plain functions), `page.rs`, `id.rs` and `refusal.rs` (why a command or an action is refused).
  - `src/session.rs` holds the `Session` handle: one lock around the lesson and the chat, a version that rises with every change, and `changes_since`, which returns the pages and the chat changed since a reader's cursor. A student action first waits for room on the event channel and then takes the lock, so events reach the host in the order of the changes they report.
  - `src/chat.rs` holds the chat transcript, `src/frames.rs` the frames on the wire both ways, `src/changes.rs` the `Version` and what a reader gets back, and `src/error.rs` the two error types.
  - `src/event.rs` holds the events a host hears about the student.
  - `src/wording.rs` writes everything a model reads: receipts, refusals, and the line that reports each student event.
  - `fixtures/student-frames.json` holds one example of each frame the UI sends. The Rust tests and the UI tests both read it.
- `crates/pedagogue-demo/` - a sample script player and a stand-in tutor. It uses the session crate exactly as a real host would, down to reading a new block's id out of its receipt.
  - `src/script.rs` reads a script, a JSON list of steps, and checks each step's shape and names up front.
  - `src/player.rs` holds `play`, which plays a script to its end, and `run`, which plays it and then answers the live student until its caller drops it. Neither spawns a task.
  - `src/stand_in.rs` is the stand-in: the typing dots and one fixed reply to a message, and a chat line with the text a tutor would be sent for everything else.
  - `src/wording.rs` holds the stand-in's lines, and `src/pace.rs` the two speeds a script plays at.
- `samples/` - the sample scripts. `contact-sheet.json` draws the two design files state for state, and `contact-sheet.frames.json` holds the frames the app sends for it. A Rust test keeps that file current, and the UI tests read it.
- `crates/pedagogue-server/` - the Axum router: the UI files, the socket and a command route. Its build script runs `ui/build.mjs`, and the bundle is embedded into the binary.
  - `src/serve.rs` holds `bind`, which opens the loopback listener, and `serve`, one future that its caller runs. It stops on the caller's signal and ends when every socket has closed.
  - `src/app.rs` joins the routes of each feature and holds the state they share. `src/assets.rs` serves the three UI files by name, `src/socket.rs` runs one task per `/ws` connection with a cursor into the session, and `src/commands.rs` is `POST /commands`.
  - `src/cross_site.rs` guards the socket and the command route against other sites, `src/csp.rs` stamps the content policy on every response, and `src/error.rs` holds the refusals sent over HTTP.
  - `examples/serve.rs` runs the server and the demo together for a browser.
- `crates/pedagogue-desktop/` - the Tauri app. It owns the session and the one async runtime, starts the server and the demo on it, and opens a window on the server's address.
  - `src/run.rs` holds `run`: it reads the script and opens the listener before the window exists, spawns the server and the demo on Tauri's async runtime, and after the window closes it signals the server and waits for it for a few seconds at most.
  - `src/navigation.rs` is the navigation policy: the window loads only the app's own address, and any other link opens in the system browser. The page has no access to Tauri's API, so links are handled in Rust.
  - `src/main.rs` only calls `run` and returns its exit code. `app-icon.png` is the source of the files in `icons/`, made with `cargo tauri icon`.
- `ui/` - the TypeScript UI. `ui/src/main.ts` is the entry point. `ui/src/tokens.css` holds the layer order, the reset and the two palettes. `ui/src/page/` is the textbook side and `ui/src/chat/` is the tutor side, and each component keeps its CSS file beside it. `ui/build.mjs` bundles them into three files and copies the icon sprites out of the design files.
- `design/` - the two design reference files. They link the CSS files under `ui/src`, so the look has one home, and each keeps its icons in a sprite of its own.

## Invariants

- **Rust decides, the UI draws.** Page state and every quiz rule live in the session crate. The UI sends what the student did and draws the state it gets back.
- **The answer key stays in Rust.** The page the UI receives holds option states. Which option is right stays in the session until the rules release it.
- **One home for the look.** The CSS files under `ui/src` are read by the design files and bundled by the app.
- **Pages only grow.** A command adds to the newest page. Marking a quiz is the one change to an existing block.
- **HTML enters the page in one place.** `ui/src/markdown.ts` turns markdown into sanitized HTML. All other text goes in as plain text.

## Cross-cutting concerns

- **Wording.** Text the student reads lives in `ui/src/wording.ts`. Text a model reads (receipts, refusals and reports of student actions) lives in each Rust crate's `wording.rs`.
- **Errors.** Each kind of failure has its own opaque error type. An error is logged or returned, and the layer that handles it is the one that logs it.
- **Logging.** The libraries log with `tracing`. The binaries install the subscriber.
- **Tests.** Rust unit tests sit in the file under test, and tests of a crate's public API sit in one binary at `tests/it/main.rs`. UI tests load the built bundle into jsdom. Shared JSON fixtures pin the wire contract from both sides.
