// The build writes exactly the three named files the server delivers, and
// it copies the design files' icon sprites into the page.
import assert from "node:assert/strict";
import { readFile, stat } from "node:fs/promises";
import path from "node:path";
import { describe, it } from "node:test";
import { collectSprites } from "../sprites.mjs";

const dist = path.join(import.meta.dirname, "..", "dist");

function sprite(...ids) {
  const symbols = ids.map((id) => `<symbol id="${id}"></symbol>`).join("");
  return `<body><svg aria-hidden="true">${symbols}</svg></body>`;
}

describe("the build", () => {
  for (const name of ["index.html", "app.js", "app.css"]) {
    it(`writes ${name}`, async () => {
      const info = await stat(path.join(dist, name));
      assert.ok(info.isFile());
    });
  }

  it("links the bundle from the page", async () => {
    const html = await readFile(path.join(dist, "index.html"), "utf8");
    assert.ok(html.includes('href="/app.css"'));
    assert.ok(html.includes('src="/app.js"'));
  });

  it("puts the icons of both design files into the page", async () => {
    const html = await readFile(path.join(dist, "index.html"), "utf8");
    assert.ok(html.includes('<symbol id="i-info"'));
    assert.ok(html.includes('<symbol id="i-send"'));
    assert.ok(!html.includes("<!-- icons -->"));
  });
});

describe("collectSprites", () => {
  it("joins the sprite of each file", () => {
    const sprites = collectSprites([
      { name: "a.html", html: sprite("i-one") },
      { name: "b.html", html: sprite("i-two") },
    ]);
    assert.ok(sprites.includes('id="i-one"'));
    assert.ok(sprites.includes('id="i-two"'));
  });

  it("stops when two icons share an id", () => {
    assert.throws(
      () =>
        collectSprites([
          { name: "a.html", html: sprite("i-one", "i-same") },
          { name: "b.html", html: sprite("i-same") },
        ]),
      /icon id "i-same" is in both a\.html and b\.html/,
    );
  });

  it("stops when a file holds no sprite", () => {
    assert.throws(
      () => collectSprites([{ name: "a.html", html: "<body></body>" }]),
      /a\.html holds no icon sprite/,
    );
  });
});
