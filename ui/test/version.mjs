// One version for the whole app. The Rust workspace states it, the
// installers are named by it, and the UI package states the same one, so a
// release bumps both in one change.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { describe, it } from "node:test";

const uiDir = path.join(import.meta.dirname, "..");

/** Reads the `version` key of the `[workspace.package]` table. */
function workspaceVersion(manifest) {
  const table = manifest
    .split(/^\[/m)
    .find((part) => part.startsWith("workspace.package]"));
  const version = table?.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
  if (version == null) {
    throw new Error("Cargo.toml states no version under [workspace.package]");
  }
  return version;
}

async function readJson(name) {
  return JSON.parse(await readFile(path.join(uiDir, name), "utf8"));
}

describe("the version", () => {
  it("is the same in the UI package as in the Rust workspace", async () => {
    const manifest = await readFile(
      path.join(uiDir, "..", "Cargo.toml"),
      "utf8",
    );
    const { version } = await readJson("package.json");
    assert.equal(version, workspaceVersion(manifest));
  });

  it("is the same in the lockfile as in the UI package", async () => {
    const { version } = await readJson("package.json");
    const lock = await readJson("package-lock.json");
    assert.equal(lock.version, version);
    assert.equal(lock.packages[""].version, version);
  });

  it("is read from the workspace table alone", () => {
    const manifest = [
      "[package]",
      'version = "9.9.9"',
      "",
      "[workspace.package]",
      'edition = "2024"',
      'version = "1.2.3"',
      "",
      "[workspace.dependencies]",
      'version = "8.8.8"',
    ].join("\n");
    assert.equal(workspaceVersion(manifest), "1.2.3");
    assert.throws(
      () => workspaceVersion('[package]\nversion = "9.9.9"\n'),
      /states no version under \[workspace\.package\]/,
    );
  });
});
