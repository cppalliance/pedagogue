// Both design files follow the HTML rules, and the checker itself catches
// each kind of break.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { describe, it } from "node:test";
import { JSDOM } from "jsdom";
import { findHtmlProblems } from "./helpers/html-rules.mjs";

const designDir = path.join(import.meta.dirname, "..", "..", "design");

const BROKEN_PAGE = `<!DOCTYPE html>
<html>
  <body>
    <h1>One</h1>
    <h1>Two</h1>
    <h3>Skipped</h3>
    <input type="text" id="twice">
    <button id="twice"></button>
    <progress max="2" value="1"></progress>
    <a>Nowhere</a>
    <img src="x.png">
    <svg></svg>
    <div role="button" tabindex="3" style="color: red" onclick="go()">Go</div>
  </body>
</html>`;

describe("the design files follow the HTML rules", () => {
  for (const file of ["page-contact-sheet.html", "chat-panel.html"]) {
    it(file, async () => {
      const html = await readFile(path.join(designDir, file), "utf8");
      const { window } = new JSDOM(html);
      assert.deepEqual(findHtmlProblems(window.document), []);
    });
  }
});

describe("findHtmlProblems", () => {
  const { window } = new JSDOM(BROKEN_PAGE);
  const problems = findHtmlProblems(window.document);

  const expected = [
    "<html> has no lang",
    "expected one <main>, found 0",
    "expected one exposed <h1>, found 2",
    "<h3> skips from level 1 to level 3",
    "<input#twice> has no <label>",
    "<button#twice> has no type",
    "<button#twice> has no name",
    "<progress> has no name",
    "<a> has no href",
    "<img> has no alt",
    "<img> has no width",
    "<img> has no height",
    "<svg> is neither a named image nor aria-hidden",
    'id "twice" is used twice',
    "<div> has an inline onclick handler",
    "<div> has an inline style",
    "<div> has a positive tabindex",
    "<div> plays a button without being one",
  ];

  it("catches each kind of break", () => {
    assert.deepEqual(problems, expected);
  });

  it("looks past content under the hidden attribute", () => {
    const page = new JSDOM(
      `<!DOCTYPE html><html lang="en"><body><main><h1>One</h1>
       <div hidden><h1>Two</h1><input type="text"></div></main></body></html>`,
    );
    assert.deepEqual(findHtmlProblems(page.window.document), []);
  });
});
