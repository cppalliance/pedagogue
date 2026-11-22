// The tutor side, drawn from frames and compared with the chat design.
import assert from "node:assert/strict";
import { afterEach, describe, it, mock } from "node:test";
import { boot, studentFrames } from "./helpers/boot.mjs";
import { chatFrame, designDocument, pageFrame } from "./helpers/frames.mjs";
import { findHtmlProblems } from "./helpers/html-rules.mjs";
import { skeleton } from "./helpers/skeleton.mjs";

const DESIGN = "chat-panel.html";
// The time on the design's time line, on the day the tests pretend it is.
const AT = new Date(2026, 10, 21, 9, 41).getTime();
const NOW = new Date(2026, 10, 21, 14, 5).getTime();
const MINUTE = 60 * 1000;

const tutor = (text, at = AT) => ({ kind: "assistant", text, at });
const student = (text, at = AT) => ({ kind: "user", text, at });
const notice = (text, at = AT) => ({ kind: "notice", text, at });

/** Finds a labeled sample of the chat design by the text of its label. */
function stage(design, label) {
  const found = [...design.querySelectorAll(".spec")].find(
    (spec) => spec.querySelector(".label").textContent === label,
  );
  assert.ok(found, `the design has a sample labeled ${label}`);
  return found.querySelector(".stage");
}

/** The design's live panel without the ids its own script looks up. */
function livePanel(design) {
  const panel = design.querySelector("aside.panel").cloneNode(true);
  for (const marked of panel.querySelectorAll(
    "#messages, #composer, #rectime, #mic, #send",
  )) {
    marked.removeAttribute("id");
  }
  return panel;
}

describe("the chat panel", () => {
  let app;

  afterEach(() => {
    app?.close();
    mock.timers.reset();
  });

  /** Boots the app on the pretend day and delivers the frames. */
  async function draw(...frames) {
    mock.timers.enable({
      apis: ["Date", "setTimeout", "setInterval"],
      now: NOW,
    });
    app = await boot();
    for (const frame of frames) {
      app.deliver(frame);
    }
    return app.document.querySelector("aside.tutor");
  }

  it("draws the live panel of the design file", async () => {
    const design = await designDocument(DESIGN);
    const panel = await draw(
      pageFrame(1, 1, { chapter_number: 3, section_number: "3.2" }),
      chatFrame({
        entries: [
          tutor("Read this section, then try the quiz."),
          student(
            "Do I restate the question before or after closing the queue?",
          ),
          tutor("After. Close the queue, restate the question, then poll."),
        ],
        suggestions: ["Take the quiz", "Explain this page", "Show an example"],
      }),
    );

    // The app's panel is also the chat pane, which the design shows alone.
    const drawn = panel.cloneNode(true);
    drawn.classList.remove("tutor");
    assert.equal(skeleton(drawn), skeleton(livePanel(design)));
    assert.deepEqual(findHtmlProblems(app.document), []);
  });

  it("draws runs, tails, paragraphs and code as the labeled samples have them", async () => {
    const design = await designDocument(DESIGN);
    const panel = await draw(
      chatFrame({
        entries: [
          tutor("Nice work on the quiz."),
          tutor("Ready for section 3.3 when you are."),
          student("Can I combine restating and polling in one step?"),
          tutor(
            "`std::move` only casts to an rvalue reference.\n\nThe move happens when a move constructor or move assignment runs.",
          ),
          student("Thanks."),
          tutor("After. Close the queue, restate the question, then poll."),
        ],
      }),
    );
    const [run, out, code, , last] = panel.querySelectorAll(".msg");
    const only = (label) => stage(design, label).querySelector(".msg");

    assert.equal(
      skeleton(run),
      skeleton(only(".bubble.in (run of two, tail on last)")),
    );
    assert.equal(skeleton(out), skeleton(only(".bubble.out")));
    assert.equal(
      skeleton(code),
      skeleton(only(".bubble.in (paragraphs and inline code)")),
    );
    // The icon row sits under the newest tutor message alone.
    assert.equal(skeleton(last), skeleton(only(".actions (default)")));
    assert.equal(panel.querySelectorAll(".actions").length, 1);
  });

  it("draws a time line before the first message and after a 15 minute gap, and a notice", async () => {
    const design = await designDocument(DESIGN);
    const panel = await draw(
      chatFrame({
        entries: [
          notice("**Milestone 2 unlocked** - Section 2.2 is ready"),
          tutor("One.", AT + 14 * MINUTE),
          tutor("Two.", AT + 29 * MINUTE),
          tutor("Three.", AT + 29 * MINUTE),
        ],
      }),
    );
    const log = panel.querySelector(".messages");
    const lines = [...log.querySelectorAll("time.stamp")].map(
      (line) => line.textContent,
    );

    assert.deepEqual(lines, ["Today 9:41 AM", "Today 10:10 AM"]);
    const [stamp, milestone] = log.children;
    const designed = stage(design, ".stamp and .notice");
    assert.equal(skeleton(stamp), skeleton(designed.querySelector(".stamp")));
    assert.equal(
      skeleton(milestone),
      skeleton(designed.querySelector(".notice")),
    );
    // The gap broke the run, so each side of it has a tail of its own.
    assert.equal(log.querySelectorAll(".bubble.tail").length, 2);
  });

  it("names the day of a message from another day", async () => {
    const dayBefore = new Date(2026, 10, 20, 16, 5).getTime();
    const panel = await draw(
      chatFrame({ entries: [tutor("Earlier.", dayBefore)] }),
    );
    assert.equal(
      panel.querySelector("time.stamp").textContent,
      "Nov 20 4:05 PM",
    );
  });

  it("shows the typing dots and locks the composer while the tutor replies", async () => {
    const design = await designDocument(DESIGN);
    const entries = [student("Why close the queue first?")];
    const panel = await draw(chatFrame({ state: "working", entries }));
    const typing = panel.querySelector(".messages").lastElementChild;

    assert.equal(
      skeleton(typing),
      skeleton(stage(design, ".typing").querySelector(".msg")),
    );
    assert.equal(
      skeleton(panel.querySelector(".composer")),
      skeleton(
        composerOf(stage(design, ".composer.disabled (tutor is replying)")),
      ),
    );

    // The reply takes the place of the dots, and the composer opens again.
    app.deliver(
      chatFrame({ entries: [...entries, tutor("So the list is final.")] }),
    );
    assert.equal(panel.querySelector(".typing"), null);
    assert.equal(panel.querySelector(".composer").className, "composer");
    assert.equal(panel.querySelector(".field-area").disabled, false);
  });

  it("draws only the new entries of a longer chat", async () => {
    const first = tutor("One.");
    const panel = await draw(chatFrame({ entries: [first] }));
    const kept = panel.querySelector(".bubble");

    app.deliver(chatFrame({ entries: [first, tutor("Two.")] }));

    const bubbles = [...panel.querySelectorAll(".bubble")];
    assert.equal(bubbles[0], kept);
    assert.deepEqual(
      bubbles.map((bubble) => bubble.className),
      ["bubble in", "bubble in tail"],
    );
    // The icon row stays last in the run, under the newest bubble.
    assert.equal(
      panel.querySelector(".msg").lastElementChild.className,
      "actions",
    );
  });

  it("starts over when the server holds another chat", async () => {
    const panel = await draw(
      chatFrame({ entries: [tutor("One."), tutor("Two.")] }),
    );
    app.deliver(chatFrame({ entries: [tutor("Another lesson.")] }));
    const texts = [...panel.querySelectorAll(".bubble")].map(
      (bubble) => bubble.textContent,
    );
    assert.deepEqual(texts, ["Another lesson."]);
  });

  describe("the composer", () => {
    it("matches the design when empty, with text, and while recording", async () => {
      const design = await designDocument(DESIGN);
      const panel = await draw(chatFrame());
      const composer = panel.querySelector(".composer");
      const field = composer.querySelector(".field-area");
      const look = (label) => skeleton(composerOf(stage(design, label)));

      assert.equal(skeleton(composer), look(".composer (empty, send dimmed)"));

      type(field, "Why close the queue first?");
      assert.equal(skeleton(composer), look(".composer (typing, send ready)"));

      type(field, "");
      composer.querySelector(".mic").click();
      mock.timers.tick(7000);
      assert.equal(skeleton(composer), look(".composer.recording"));
      assert.equal(
        composer.querySelector(".mic use").getAttribute("href"),
        "#i-stop",
      );

      composer.querySelector(".mic").click();
      assert.equal(skeleton(composer), look(".composer (empty, send dimmed)"));
      assert.equal(
        composer.querySelector(".mic use").getAttribute("href"),
        "#i-mic",
      );
    });

    it("sends the fixture's frame on Send and on Enter, and clears the edit box", async () => {
      const fixture = await studentFrames();
      const panel = await draw(chatFrame());
      const composer = panel.querySelector(".composer");
      const field = composer.querySelector(".field-area");

      type(field, ` ${fixture.say.text} `);
      composer.querySelector(".send").click();
      assert.equal(field.value, "");
      assert.equal(composer.querySelector(".send").disabled, true);

      type(field, fixture.say.text);
      composer.requestSubmit();
      composer.requestSubmit();

      assert.deepEqual(app.sent(), [fixture.say, fixture.say]);
    });
  });

  it("sends a chip's text as a message from the student", async () => {
    const design = await designDocument(DESIGN);
    const fixture = await studentFrames();
    const panel = await draw(
      chatFrame({
        suggestions: ["Take the quiz", fixture.say.text, "Show an example"],
      }),
    );
    const chips = panel.querySelector(".chips");
    const designed = stage(design, ".chip")
      .querySelector(".chips")
      .cloneNode(true);
    designed.children[1].textContent = fixture.say.text;
    assert.equal(skeleton(chips), skeleton(designed));

    chips.children[1].click();
    assert.deepEqual(app.sent(), [fixture.say]);

    app.deliver(chatFrame());
    assert.equal(chips.hidden, true);
  });

  describe("the icon row", () => {
    const REPLY = "After. Close the queue, restate the question, then poll.";

    it("sends the fixture's frame on Regenerate", async () => {
      const fixture = await studentFrames();
      const panel = await draw(chatFrame({ entries: [tutor(REPLY)] }));
      panel.querySelector('[data-act="regen"]').click();
      assert.deepEqual(app.sent(), [fixture.regenerate]);
    });

    it("copies the newest reply and turns the button green for a moment", async () => {
      const design = await designDocument(DESIGN);
      const panel = await draw(
        chatFrame({
          entries: [tutor("An earlier reply."), student("Why?"), tutor(REPLY)],
        }),
      );
      const row = panel.querySelector(".actions");

      row.querySelector('[data-act="copy"]').click();
      await Promise.resolve();

      assert.deepEqual(app.clipboard.texts, [REPLY]);
      const copied = stage(design, ".actions (copied)").querySelector(
        ".actions",
      );
      assert.equal(skeleton(row), skeleton(copied));
      mock.timers.tick(1200);
      assert.equal(row.querySelector(".done"), null);
    });

    it("shows the read-aloud look for a moment", async () => {
      const design = await designDocument(DESIGN);
      const panel = await draw(chatFrame({ entries: [tutor(REPLY)] }));
      const row = panel.querySelector(".actions");

      row.querySelector('[data-act="speak"]').click();

      const speaking = stage(design, ".actions (speaking)").querySelector(
        ".actions",
      );
      assert.equal(skeleton(row), skeleton(speaking));
      mock.timers.tick(3000);
      assert.equal(
        row.querySelector('[data-act="speak"]').getAttribute("aria-pressed"),
        "false",
      );
    });
  });
});

/** Types into a field the way a student does. */
function type(field, text) {
  field.value = text;
  field.dispatchEvent(
    new field.ownerDocument.defaultView.Event("input", { bubbles: true }),
  );
}

/** The composer of a labeled sample of the chat design. */
function composerOf(sampleStage) {
  return sampleStage.querySelector(".composer");
}
