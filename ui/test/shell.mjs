// The app shell: two panes, one socket, and frames that pass a schema
// before any view sees them.
import assert from "node:assert/strict";
import { afterEach, describe, it, mock } from "node:test";
import { boot, sampleFrames } from "./helpers/boot.mjs";
import { findHtmlProblems } from "./helpers/html-rules.mjs";

describe("the app shell", () => {
  let app;

  afterEach(() => {
    app?.close();
    mock.timers.reset();
  });

  it("builds both panes, then opens one socket on the server it came from", async () => {
    app = await boot();
    const { document } = app;

    assert.ok(document.querySelector("main.textbook > .viewport"));
    const panel = document.querySelector("aside.tutor.panel");
    assert.equal(panel.getAttribute("aria-label"), "Tutor chat");
    assert.equal(panel.querySelector("h2.who-name").textContent, "Tutor");
    assert.deepEqual(
      app.sockets.map((socket) => socket.url),
      ["ws://127.0.0.1:7910/ws"],
    );
  });

  it("follows the HTML rules before its first page arrives", async () => {
    app = await boot();
    assert.deepEqual(findHtmlProblems(app.document), []);
    // One heading is exposed, so a screen reader has a title to read.
    assert.equal(app.document.querySelector("h1").textContent, "TeachPad");
  });

  it("takes every frame of the contact-sheet sample", async () => {
    app = await boot();
    const frames = await sampleFrames();
    assert.ok(frames.length >= 2);

    for (const frame of frames) {
      app.deliver(frame);
    }

    assert.deepEqual(app.warnings, []);
    assert.deepEqual(findHtmlProblems(app.document), []);
  });

  it("drops a frame that fails its schema, with one warning that names the field", async () => {
    app = await boot();
    const [page] = await sampleFrames();
    const { page_count: _, ...withoutCount } = page;
    const wrongState = {
      type: "chat",
      state: "asleep",
      suggestions: [],
      entries: [],
    };
    const extraField = {
      type: "chat",
      state: "working",
      suggestions: [],
      entries: [],
      mood: 1,
    };

    app.deliver(withoutCount);
    app.deliver(wrongState);
    app.deliver(extraField);
    app.socket.deliverText("this is not JSON");
    app.deliver({ type: "shout" });

    assert.equal(app.warnings.length, 5);
    assert.match(
      app.warnings[0],
      /^a frame from the server was dropped: page_count: /,
    );
    assert.match(app.warnings[1], /dropped: state: /);
    assert.match(app.warnings[2], /dropped: frame: .*mood/);
    assert.match(app.warnings[3], /dropped: the message is not JSON$/);
    assert.match(app.warnings[4], /dropped: type: /);
  });

  it("opens the socket again after a wait that doubles, and resets once a frame arrives", async () => {
    app = await boot();
    mock.timers.enable({ apis: ["setTimeout"] });
    const frames = await sampleFrames();

    app.socket.drop();
    mock.timers.tick(249);
    assert.equal(app.sockets.length, 1);
    mock.timers.tick(1);
    assert.equal(app.sockets.length, 2);

    // No frame arrived on the second socket, so the next wait is longer.
    app.socket.drop();
    mock.timers.tick(499);
    assert.equal(app.sockets.length, 2);
    mock.timers.tick(1);
    assert.equal(app.sockets.length, 3);

    app.deliver(frames.at(-1));
    app.socket.drop();
    mock.timers.tick(250);
    assert.equal(app.sockets.length, 4);
  });
});
