// The import rule, read from the source text. The page side and the chat
// side import the shared files and never each other, a shared file never
// imports a side, and main.ts alone wires the two. State lives in view
// objects, so no module keeps a variable of its own. Every file is imported
// by its own path, so no index.ts exists.
import assert from "node:assert/strict";
import { readdir, readFile } from "node:fs/promises";
import path from "node:path";
import { describe, it } from "node:test";

const srcDir = path.join(import.meta.dirname, "..", "src");
const SIDES = ["page", "chat"];
const ENTRY = "main.ts";

/** Lists the TypeScript files under src/, as posix paths. */
async function listSources() {
  const entries = await readdir(srcDir, {
    recursive: true,
    withFileTypes: true,
  });
  return entries
    .filter((entry) => entry.isFile() && entry.name.endsWith(".ts"))
    .map((entry) =>
      path
        .relative(srcDir, path.join(entry.parentPath, entry.name))
        .replaceAll(path.sep, "/"),
    );
}

/** Returns the side a source file belongs to, or null for a shared file. */
function sideOf(file) {
  return SIDES.find((side) => file.startsWith(`${side}/`)) ?? null;
}

/** Resolves each relative import of a file to a path under src/. */
async function importsOf(file) {
  const text = await readFile(path.join(srcDir, file), "utf8");
  const found = [];
  for (const [, target] of text.matchAll(
    /from\s+"(\.[^"]+)"|import\s+"(\.[^"]+)"/g,
  )) {
    if (target !== undefined) {
      found.push(path.posix.join(path.posix.dirname(file), target));
    }
  }
  return found;
}

describe("the import rule", async () => {
  const sources = await listSources();

  it("keeps each side out of the other", async () => {
    for (const file of sources) {
      const side = sideOf(file);
      if (side === null) {
        continue;
      }
      const other = SIDES.find((name) => name !== side);
      for (const target of await importsOf(file)) {
        assert.ok(
          !target.startsWith(`${other}/`),
          `${file} imports ${target}, which belongs to the other side`,
        );
      }
    }
  });

  it("keeps the shared files free of both sides", async () => {
    for (const file of sources) {
      if (sideOf(file) !== null || file === ENTRY) {
        continue;
      }
      for (const target of await importsOf(file)) {
        assert.equal(
          sideOf(target),
          null,
          `${file} is shared and imports ${target}`,
        );
      }
    }
  });

  it("lets main.ts alone wire the two sides", async () => {
    const targets = await importsOf(ENTRY);
    for (const side of SIDES) {
      assert.ok(
        targets.some((target) => target.startsWith(`${side}/`)),
        `main.ts imports the ${side} side`,
      );
    }
  });

  it("has no index.ts", () => {
    assert.deepEqual(
      sources.filter((file) => path.posix.basename(file) === "index.ts"),
      [],
    );
  });

  it("keeps state out of the modules", async () => {
    for (const file of sources) {
      const text = await readFile(path.join(srcDir, file), "utf8");
      assert.doesNotMatch(
        text,
        /^(?:export\s+)?(?:let|var)\s/m,
        `${file} keeps a variable at module level`,
      );
    }
  });
});
