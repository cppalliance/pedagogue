// The build writes exactly the three named files the server delivers.
import assert from "node:assert/strict";
import { readFile, stat } from "node:fs/promises";
import path from "node:path";
import { describe, it } from "node:test";

const dist = path.join(import.meta.dirname, "..", "dist");

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
});
