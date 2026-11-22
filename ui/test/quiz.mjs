// The quiz boxes, drawn from frames and compared with the page design.
import assert from "node:assert/strict";
import { afterEach, describe, it } from "node:test";
import { boot, sampleFrames, studentFrames } from "./helpers/boot.mjs";
import {
  choiceQuiz,
  designDocument,
  exposition,
  pageFrame,
  responseQuiz,
  sample,
} from "./helpers/frames.mjs";
import { findHtmlProblems } from "./helpers/html-rules.mjs";
import { skeleton } from "./helpers/skeleton.mjs";

const DESIGN = "page-contact-sheet.html";
const ANSWER = "It casts to an rvalue.";

/** Types into a field the way a student does. */
function type(field, text) {
  field.value = text;
  field.dispatchEvent(
    new field.ownerDocument.defaultView.Event("input", { bubbles: true }),
  );
}

/** Ticks an option the way a student does. */
function tick(form, index) {
  form.querySelectorAll(".opt input")[index].click();
}

/** Presses the button of a form that has the given text. */
function press(form, text) {
  const found = [...form.querySelectorAll("button")].find(
    (button) => button.textContent === text,
  );
  assert.ok(found, `the form has a "${text}" button`);
  found.click();
}

/**
 * Builds what a free-text box looks like after Submit, from the design:
 * its own open sample with the field closed and the main button gone, and
 * the status area of the short-answer sample in the given state.
 */
function afterSubmit(design, openLabel, statusLabel, answer) {
  const box = sample(design, openLabel).cloneNode(true);
  const field = box.querySelector(".field");
  field.setAttribute("readonly", "");
  field.value = answer;
  box.querySelector(".q-foot")?.remove();
  box.querySelector(".row .btn")?.remove();
  box.append(
    sample(design, statusLabel).querySelector(".q-status").cloneNode(true),
  );
  return box;
}

describe("the quizzes", () => {
  let app;

  afterEach(() => {
    app?.close();
  });

  /** Boots the app, draws one page of blocks, and returns its quiz forms. */
  async function draw(...blocks) {
    app = await boot();
    app.deliver(pageFrame(1, 1, { blocks }));
    return [...app.document.querySelectorAll("form.quiz")];
  }

  it("draw every quiz sample of the contact sheet as the design file has it", async () => {
    const design = await designDocument(DESIGN);
    app = await boot();
    for (const frame of await sampleFrames()) {
      app.deliver(frame);
    }

    const drawn = [...app.document.querySelectorAll("form.quiz")];
    const designed = [...design.querySelectorAll("article.page > form.quiz")];
    assert.equal(drawn.length, designed.length);
    for (const [index, form] of designed.entries()) {
      assert.equal(skeleton(drawn[index]), skeleton(form));
    }
    assert.deepEqual(findHtmlProblems(app.document), []);
    assert.deepEqual(app.warnings, []);
  });

  it("draw the essay and the Try It box after Submit as their open sample with the status area", async () => {
    const design = await designDocument(DESIGN);
    const essay = { question: "Describe a problem.", words: 100 };
    const tryIt = { question: "Double it.", code: "co_return x + 1;" };
    const feedback = "Nothing has moved yet.";
    const [sentEssay, wrongTryIt] = await draw(
      responseQuiz("b1", "short_essay", {
        ...essay,
        status: "submitted",
        answer: "Shared state.",
      }),
      responseQuiz("b2", "try_it", {
        ...tryIt,
        status: "wrong",
        answer: "co_return x;",
        feedback,
      }),
    );

    const expectedEssay = afterSubmit(
      design,
      ".callout.quiz (free text)",
      ".callout.quiz (short answer, sent)",
      "Shared state.",
    );
    expectedEssay.querySelector(".q").textContent = essay.question;
    assert.equal(skeleton(sentEssay), skeleton(expectedEssay));

    const expectedTryIt = afterSubmit(
      design,
      ".callout.quiz (try it, code)",
      ".callout.quiz (short answer, wrong)",
      "co_return x;",
    );
    expectedTryIt.querySelector(".q").textContent = tryIt.question;
    const line = expectedTryIt.querySelector(".feedback");
    line.replaceChildren(line.querySelector(".tag"), feedback);
    assert.equal(skeleton(wrongTryIt), skeleton(expectedTryIt));
  });

  it("send the fixture's frames from Check answer, Submit and Run", async () => {
    const fixture = await studentFrames();
    const [single, select, answer] = await draw(
      choiceQuiz("b1", ["This", "That"]),
      choiceQuiz("b2", ["One", "Two", "Three"], { multiple: true }),
      responseQuiz("b3", "short_answer"),
    );

    tick(single, 1);
    press(single, "Check answer");
    tick(select, 0);
    tick(select, 2);
    press(select, "Check answer");
    type(answer.querySelector(".field"), ` ${ANSWER} `);
    press(answer, "Submit");

    assert.deepEqual(app.sent(), [
      fixture.answer_picks,
      fixture.answer_select,
      fixture.answer_text,
    ]);
    app.close();

    const [tryIt] = await draw(
      exposition("b1", "One."),
      exposition("b2", "Two."),
      responseQuiz("b3", "try_it", { code: "x + 1" }),
    );
    type(tryIt.querySelector(".field"), ANSWER);
    press(tryIt, "Run");
    assert.deepEqual(app.sent(), [fixture.answer_text]);
  });

  it("send the same frame for Enter in a one-line field as for its button", async () => {
    const fixture = await studentFrames();
    const [answer] = await draw(
      exposition("b1", "One."),
      exposition("b2", "Two."),
      responseQuiz("b3", "short_answer"),
    );
    type(answer.querySelector(".field"), ANSWER);

    // Enter in a one-line field submits its form, which is what this does.
    answer.requestSubmit();

    assert.deepEqual(app.sent(), [fixture.answer_text]);
  });

  it("reopen a wrong answer on Try again, and send the next one", async () => {
    const fixture = await studentFrames();
    const [answer] = await draw(
      exposition("b1", "One."),
      exposition("b2", "Two."),
      responseQuiz("b3", "short_answer", {
        status: "wrong",
        attempts: 1,
        answer: "It moves the object.",
        feedback: "Nothing has moved yet.",
      }),
    );
    const field = answer.querySelector(".field");
    assert.equal(field.readOnly, true);
    // A closed box sends nothing, whatever submits it.
    answer.requestSubmit();
    assert.deepEqual(app.sent(), []);

    press(answer, "Try again");
    assert.equal(field.readOnly, false);
    assert.equal(field.value, "It moves the object.");
    assert.equal(answer.querySelector(".q-status"), null);
    type(field, ANSWER);
    press(answer, "Submit");

    assert.deepEqual(app.sent(), [fixture.answer_text]);
  });

  it("send nothing for a quiz with no pick and for an empty answer", async () => {
    const [single, answer] = await draw(
      choiceQuiz("b1", ["This", "That"]),
      responseQuiz("b2", "short_answer"),
    );
    press(single, "Check answer");
    type(answer.querySelector(".field"), "   ");
    press(answer, "Submit");
    assert.deepEqual(app.sent(), []);
  });

  it("keep a half-typed answer and a tick when the page gains a block", async () => {
    const blocks = [
      choiceQuiz("b1", ["This", "That"]),
      responseQuiz("b2", "short_essay", { words: 100 }),
    ];
    const [single, essay] = await draw(...blocks);
    tick(single, 1);
    const field = essay.querySelector(".field");
    type(field, "A half-typed thought");
    assert.equal(
      essay.querySelector(".word-count").textContent,
      "3 / 100 words",
    );

    app.deliver(
      pageFrame(1, 1, { blocks: [...blocks, exposition("b3", "More.")] }),
    );

    assert.equal(app.document.querySelector("form.quiz .field"), field);
    assert.equal(field.value, "A half-typed thought");
    assert.equal(single.querySelectorAll(".opt input")[1].checked, true);
  });

  it("put four or more short options in two columns", async () => {
    const [short, long, few] = await draw(
      choiceQuiz("b1", ["`int`", "`std::mutex`", "`void`", "`std::thread`"]),
      choiceQuiz("b2", ["Restate the question", "Count", "Close", "All"]),
      choiceQuiz("b3", ["Yes", "No", "Maybe"]),
    );
    assert.equal(short.querySelector(".options").className, "options grid");
    assert.equal(long.querySelector(".options").className, "options");
    assert.equal(few.querySelector(".options").className, "options");
  });
});
