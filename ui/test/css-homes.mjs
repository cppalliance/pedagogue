// One home for the look. The CSS files under src/ hold every design rule,
// the design files link them and the app imports them, so an edit shows in
// both. Colors live in tokens.css alone, and every file states the layer
// order, so the files give the same result in any load order.
import assert from "node:assert/strict";
import { readdir, readFile } from "node:fs/promises";
import path from "node:path";
import { describe, it } from "node:test";

const uiDir = path.join(import.meta.dirname, "..");
const srcDir = path.join(uiDir, "src");
const designDir = path.join(uiDir, "..", "design");

const LAYER_LINE = "@layer reset, base, components, utilities;";
const TOKENS = "tokens.css";
// The app's own split between the two sides. Each design file shows one
// side alone, so it leaves this file out.
const APP_ONLY = ["layout.css"];
const SHEETS = [
  { file: "page-contact-sheet.html", side: "page/" },
  { file: "chat-panel.html", side: "chat/" },
];

const HEX_COLOR = /#[0-9a-f]{3,8}\b/i;
const COLOR_FUNCTION = /\b(?:rgba?|hsla?|hwb|lab|lch|oklab|oklch|color)\(/i;

/** Orders two names by code unit, the same way on every machine. */
function byName(a, b) {
  if (a === b) {
    return 0;
  }
  return a < b ? -1 : 1;
}

/** Whether a stylesheet is the tokens, the app layout or part of a side. */
function hasKnownHome(name) {
  return (
    name === TOKENS ||
    APP_ONLY.includes(name) ||
    SHEETS.some(({ side }) => name.startsWith(side))
  );
}

/** Lists the files under `dir` with the given extension, as posix paths. */
async function listFiles(dir, extension) {
  const entries = await readdir(dir, { recursive: true, withFileTypes: true });
  return entries
    .filter((entry) => entry.isFile() && entry.name.endsWith(extension))
    .map((entry) =>
      path
        .relative(dir, path.join(entry.parentPath, entry.name))
        .replaceAll(path.sep, "/"),
    )
    .toSorted(byName);
}

/** The stylesheets a design file links, as paths relative to src/. */
async function linkedStyles(sheet) {
  const html = await readFile(path.join(designDir, sheet), "utf8");
  const prefix = "../ui/src/";
  return [...html.matchAll(/<link rel="stylesheet" href="([^"]+)">/g)].map(
    ([, href]) => {
      assert.ok(href.startsWith(prefix), `${sheet} links ${href}`);
      return href.slice(prefix.length);
    },
  );
}

/** A stylesheet's text with its comments and url() contents removed. */
function stripNonColorText(css) {
  return css.replace(/\/\*[\s\S]*?\*\//g, "").replace(/url\([^)]*\)/g, "url()");
}

const cssFiles = await listFiles(srcDir, ".css");

describe("the design files", () => {
  for (const { file, side } of SHEETS) {
    it(`${file} links every stylesheet on its side, tokens first`, async () => {
      const linked = await linkedStyles(file);
      const expected = cssFiles.filter((name) => name.startsWith(side));
      assert.equal(linked[0], TOKENS);
      assert.deepEqual(linked.slice(1).toSorted(byName), expected);
    });
  }
});

describe("the stylesheets", () => {
  it("are each imported by a TypeScript file", async () => {
    const imported = new Set();
    for (const source of await listFiles(srcDir, ".ts")) {
      const text = await readFile(path.join(srcDir, source), "utf8");
      for (const [, target] of text.matchAll(/^import "([^"]+\.css)";$/gm)) {
        imported.add(path.posix.join(path.posix.dirname(source), target));
      }
    }
    assert.deepEqual([...imported].toSorted(byName), cssFiles);
  });

  it("each open with the layer order", async () => {
    for (const name of cssFiles) {
      const css = await readFile(path.join(srcDir, name), "utf8");
      assert.ok(css.startsWith(LAYER_LINE), `${name} opens with ${LAYER_LINE}`);
    }
  });

  it("keep every color value in tokens.css", async () => {
    for (const name of cssFiles.filter((file) => file !== TOKENS)) {
      const css = stripNonColorText(
        await readFile(path.join(srcDir, name), "utf8"),
      );
      const found = HEX_COLOR.exec(css) ?? COLOR_FUNCTION.exec(css);
      assert.equal(found, null, `${name} holds the color value ${found?.[0]}`);
    }
  });

  it("split into the shared tokens, the two sides and the app layout", () => {
    assert.deepEqual(
      cssFiles.filter((name) => !hasKnownHome(name)),
      [],
    );
  });
});
