// The textbook side, drawn from frames and compared with the page design.
import assert from "node:assert/strict";
import { afterEach, describe, it } from "node:test";
import { boot, sampleFrames, studentFrames } from "./helpers/boot.mjs";
import {
  chatFrame,
  designDocument,
  exposition,
  pageFrame,
  sample,
} from "./helpers/frames.mjs";
import { findHtmlProblems } from "./helpers/html-rules.mjs";
import { skeleton } from "./helpers/skeleton.mjs";

const DESIGN = "page-contact-sheet.html";
// A paragraph of running text, as opposed to the chapter line over the title.
const TEXT = "p:not([class])";

/** The parts of a page that the page step draws: everything but quizzes. */
function withoutQuizzes(article) {
  return [...article.children].filter(
    (child) => !child.matches("form.quiz, nav.page-end, [data-sheet]"),
  );
}

describe("the page", () => {
  let app;

  afterEach(() => {
    app?.close();
  });

  /** Boots the app and returns the page that is on screen. */
  async function draw(...frames) {
    app = await boot();
    for (const frame of frames) {
      app.deliver(frame);
    }
    return app.document.querySelector(".scroller:not([hidden]) article.page");
  }

  it("draws the contact-sheet sample as the design file has it, quizzes aside", async () => {
    const design = await designDocument(DESIGN);
    const article = await draw(...(await sampleFrames()));

    const drawn = withoutQuizzes(article);
    const designed = withoutQuizzes(design.querySelector("article.page"));
    assert.equal(drawn.length, designed.length);
    for (const [index, element] of designed.entries()) {
      assert.equal(skeleton(drawn[index]), skeleton(element));
    }
    assert.deepEqual(app.warnings, []);
  });

  it("follows the HTML rules with the whole sample drawn", async () => {
    await draw(...(await sampleFrames()));
    assert.deepEqual(findHtmlProblems(app.document), []);
  });

  it("puts each page in a scroller between two markers, under the cues of the design", async () => {
    const design = await designDocument(DESIGN);
    await draw(pageFrame(1, 1));
    const viewport = app.document.querySelector(".viewport");

    assert.equal(
      skeleton(emptied(viewport)),
      skeleton(emptied(sample(design, ".viewport (scroll cues)"))),
    );
  });

  it("lights a cue while more of the page lies that way, and scrolls on a press", async () => {
    await draw(pageFrame(1, 1));
    const viewport = app.document.querySelector(".viewport");
    const scroller = viewport.querySelector(".scroller");
    assert.equal(viewport.dataset.bottom, "");

    app.showEdge(scroller, "bottom", false);
    assert.equal(viewport.dataset.bottom, "more");
    assert.equal(viewport.dataset.top, "");
    viewport.querySelector(".cue.bottom button").click();
    assert.equal(app.scrolls.at(-1).element, scroller);

    app.showEdge(scroller, "bottom", true);
    app.showEdge(scroller, "top", false);
    assert.equal(viewport.dataset.bottom, "");
    assert.equal(viewport.dataset.top, "more");
  });

  it("draws a new block and leaves the others as they are", async () => {
    const first = exposition("b1", "One.");
    const article = await draw(pageFrame(1, 1, { blocks: [first] }));
    const kept = article.querySelector(TEXT);

    app.deliver(pageFrame(1, 1, { blocks: [first, exposition("b2", "Two.")] }));

    const texts = [...article.querySelectorAll(TEXT)].map((p) => p.textContent);
    assert.deepEqual(texts, ["One.", "Two."]);
    assert.equal(article.querySelector(TEXT), kept);
  });

  describe("the buttons at the foot", () => {
    it("match the design on the first page, with Previous out of sight", async () => {
      const design = await designDocument(DESIGN);
      const article = await draw(pageFrame(1, 1, labeled()));
      assert.equal(
        skeleton(article.querySelector(".page-end")),
        skeleton(strip(sample(design, ".page-end (first page)"))),
      );
    });

    it("match the design on the newest page, after Continue, and on an earlier page", async () => {
      const design = await designDocument(DESIGN);
      const end = () =>
        skeleton(
          app.document.querySelector(".scroller:not([hidden]) .page-end"),
        );

      await draw(pageFrame(1, 1), pageFrame(2, 2, labeled()));
      assert.equal(end(), skeleton(sample(design, ".page-end (newest page)")));

      app.deliver(pageFrame(2, 2, labeled({ continue_pressed: true })));
      assert.equal(
        end(),
        skeleton(strip(sample(design, ".page-end (after Continue)"))),
      );

      app.deliver(pageFrame(3, 3));
      app.document
        .querySelector(".scroller:not([hidden]) .page-end .btn.neutral")
        .click();
      assert.equal(
        end(),
        skeleton(strip(sample(design, ".page-end (earlier page)"))),
      );
    });

    it("show nothing on a lone page with no Continue", async () => {
      const article = await draw(pageFrame(1, 1));
      const buttons = [...article.querySelectorAll(".page-end button")];
      assert.deepEqual(
        buttons.map((button) => button.hidden),
        [true, true],
      );
    });

    it("send the Continue frame of the fixture", async () => {
      const fixture = await studentFrames();
      const article = await draw(pageFrame(1, 1, labeled()));

      article.querySelector(".page-end .btn.continue").click();

      assert.deepEqual(app.sent(), [fixture.continue]);
    });

    it("page back and forward, and a new page comes up only for a student on the newest", async () => {
      await draw(
        pageFrame(1, 1, { blocks: [exposition("b1", "Page one.")] }),
        pageFrame(2, 2, { blocks: [exposition("b2", "Page two.")] }),
      );
      const shown = () =>
        app.document.querySelector(`.scroller:not([hidden]) article ${TEXT}`)
          .textContent;
      const press = (selector) =>
        app.document
          .querySelector(`.scroller:not([hidden]) .page-end ${selector}`)
          .click();
      assert.equal(shown(), "Page two.");

      press(".btn.neutral");
      assert.equal(shown(), "Page one.");
      // A third page arrives while the student reads page one.
      app.deliver(
        pageFrame(3, 3, { blocks: [exposition("b3", "Page three.")] }),
      );
      assert.equal(shown(), "Page one.");

      press(".btn.continue");
      assert.equal(shown(), "Page two.");
      press(".btn.continue");
      assert.equal(shown(), "Page three.");
      assert.deepEqual(app.sent(), []);
    });
  });

  describe("headings", () => {
    it("expose one h1, large where a chapter opens and hidden from view after", async () => {
      const chapter = {
        chapter_number: 3,
        chapter_title: "Chairing a WG21 Room",
      };
      await draw(
        pageFrame(1, 1, { ...chapter, chapter_opener: true }),
        pageFrame(2, 2, { ...chapter, chapter_opener: false }),
      );
      const [first, second] = app.document.querySelectorAll("article.page");

      assert.equal(first.querySelector("h1").className, "chapter-title");
      assert.equal(first.querySelector(".chapter-num").hidden, false);
      assert.equal(
        second.querySelector("h1").className,
        "chapter-title visually-hidden",
      );
      assert.equal(
        second.querySelector("h1").textContent,
        "Chairing a WG21 Room",
      );
      assert.equal(second.querySelector(".chapter-num").hidden, true);
      assert.deepEqual(findHtmlProblems(app.document), []);
    });

    it("title a page that names no chapter, for screen readers alone", async () => {
      const article = await draw(pageFrame(1, 1));
      const title = article.querySelector("h1");
      assert.equal(title.textContent, "Lesson");
      assert.equal(title.className, "chapter-title visually-hidden");
      assert.equal(article.querySelector(".rh-title").textContent, "Lesson");
    });

    it("draw markdown headings on two levels that step down by one", async () => {
      const markdown = "### Deep first\n\n# Top\n\n#### Deeper\n\n## Second";
      const bare = await draw(
        pageFrame(1, 1, { blocks: [exposition("b1", markdown)] }),
      );
      // With only the h1 above, the first deep heading is drawn as h2.
      assert.deepEqual(levels(bare), [
        "H2 Deep first",
        "H2 Top",
        "H3 Deeper",
        "H2 Second",
      ]);
      app.close();

      const sectioned = await draw(
        pageFrame(1, 1, {
          section_title: "Taking a Straw Poll",
          blocks: [exposition("b1", "### Deep first")],
        }),
      );
      assert.deepEqual(levels(sectioned), [
        "H2 Taking a Straw Poll",
        "H3 Deep first",
      ]);
    });
  });

  describe("markdown", () => {
    it("draws a table with the design's class, column headers and alignment classes", async () => {
      const markdown = "| Name | Count |\n| :--- | ---: |\n| SF | 9 |";
      const article = await draw(
        pageFrame(1, 1, { blocks: [exposition("b1", markdown)] }),
      );
      assert.equal(
        skeleton(article.querySelector("table")),
        [
          "table.data",
          "  thead",
          "    tr",
          '      th.align-left[scope="col"]',
          '        "Name"',
          '      th.align-right[scope="col"]',
          '        "Count"',
          "  tbody",
          "    tr",
          "      td.align-left",
          '        "SF"',
          "      td.align-right",
          '        "9"',
        ].join("\n"),
      );
    });

    it("sends a link outside the app and says so", async () => {
      const article = await draw(
        pageFrame(1, 1, {
          blocks: [
            exposition("b1", "Read [the guide](https://example.com/a)."),
          ],
        }),
      );
      const link = article.querySelector("a");
      assert.equal(link.getAttribute("href"), "https://example.com/a");
      assert.equal(link.getAttribute("target"), "_blank");
      assert.equal(link.getAttribute("rel"), "noopener");
      assert.equal(
        link.querySelector(".visually-hidden").textContent,
        " (opens in a new window)",
      );
    });

    it("leaves out raw HTML, images and script addresses", async () => {
      const markdown = [
        'Plain <b onclick="x()">bold</b> text.',
        "",
        "<script>window.hacked = true</script>",
        "",
        "![a chart](https://example.com/chart.png)",
        "",
        "[press](javascript:alert(1))",
      ].join("\n");
      const article = await draw(
        pageFrame(1, 1, { blocks: [exposition("b1", markdown)] }),
      );
      assert.equal(article.querySelector("b, script, img"), null);
      assert.equal(article.querySelector(TEXT).textContent, "Plain bold text.");
      assert.equal(
        article.querySelector("a")?.hasAttribute("href") ?? false,
        false,
      );
      assert.equal(app.window.hacked, undefined);
    });

    it("colors a fenced block in the five code colors of the design", async () => {
      const markdown = '```cpp\n// note\nint x = f("s");\n```';
      const article = await draw(
        pageFrame(1, 1, { blocks: [exposition("b1", markdown)] }),
      );
      const classes = new Set(
        [...article.querySelectorAll("pre.code > code span")].map(
          (token) => token.className,
        ),
      );
      assert.ok(classes.has("cm") && classes.has("st"));
      for (const name of classes) {
        assert.ok(["kw", "ty", "st", "cm", "fn"].includes(name), name);
      }
    });
  });

  it("tells the chat side which chapter and section are on screen", async () => {
    await draw(
      pageFrame(1, 1, { chapter_number: 3, section_number: "3.2" }),
      pageFrame(2, 2, { chapter_number: 3, section_number: "3.3" }),
      chatFrame(),
    );
    const place = app.document.querySelector(".who-place");
    assert.equal(place.textContent, "Chapter 3 - Section 3.3");

    app.document
      .querySelector(".scroller:not([hidden]) .page-end .btn.neutral")
      .click();
    assert.equal(place.textContent, "Chapter 3 - Section 3.2");
  });
});

/** A design sample without the mark that keeps it out of the page. */
function strip(node) {
  const copy = node.cloneNode(true);
  copy.removeAttribute("data-sheet");
  return copy;
}

/**
 * A scroll viewport with nothing between the markers of its scrollers. The
 * design's sample holds sample text there and the app holds a page, so the
 * two are compared without that content.
 */
function emptied(viewport) {
  const copy = strip(viewport);
  copy.removeAttribute("id");
  for (const inner of copy.querySelectorAll(".scroller > :not(.edge)")) {
    inner.remove();
  }
  return copy;
}

/** The fields of a page that has a Continue button. */
function labeled(page) {
  return { continue_label: "Continue to 3.3", ...page };
}

/** The headings a student sees on a page, each with its level. */
function levels(article) {
  return [...article.querySelectorAll("h2:not([hidden]), h3")].map(
    (heading) => `${heading.tagName} ${heading.textContent}`,
  );
}
