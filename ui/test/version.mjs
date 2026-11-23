// One version for the whole app. The Rust workspace states it, the
// installers are named by it, and the UI package states the same one, so a
// release bumps them in one change.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { describe, it } from "node:test";

const uiDir = path.join(import.meta.dirname, "..");

/** Returns the lines of one table of a manifest, without its heading. */
function tableOf(manifest, name) {
  return manifest.split(/^\[/m).find((part) => part.startsWith(`${name}]`));
}

/** Reads the `version` key of the `[workspace.package]` table. */
function workspaceVersion(manifest) {
  const version = tableOf(manifest, "workspace.package")?.match(
    /^version\s*=\s*"([^"]+)"/m,
  )?.[1];
  if (version == null) {
    throw new Error("Cargo.toml states no version under [workspace.package]");
  }
  return version;
}

/**
 * Lists the crates of this workspace that `[workspace.dependencies]` names,
 * each with the version its entry asks for, if it states one.
 */
function internalEntries(manifest) {
  const entries = tableOf(manifest, "workspace.dependencies") ?? "";
  return [...entries.matchAll(/^(teachpad-[\w-]+)\s*=\s*\{(.*)\}\s*$/gm)].map(
    ([, name, fields]) => ({
      name,
      version: fields.match(/\bversion\s*=\s*"([^"]+)"/)?.[1],
    }),
  );
}

async function readManifest() {
  return readFile(path.join(uiDir, "..", "Cargo.toml"), "utf8");
}

async function readJson(name) {
  return JSON.parse(await readFile(path.join(uiDir, name), "utf8"));
}

describe("the version", () => {
  it("is the same in the UI package as in the Rust workspace", async () => {
    const { version } = await readJson("package.json");
    assert.equal(version, workspaceVersion(await readManifest()));
  });

  it("is the same in the lockfile as in the UI package", async () => {
    const { version } = await readJson("package.json");
    const lock = await readJson("package-lock.json");
    assert.equal(lock.version, version);
    assert.equal(lock.packages[""].version, version);
  });

  // Cargo accepts 0.1.0 as a requirement for 0.1.1, so an entry that a
  // release left behind still builds. This test is what catches it.
  it("is the one each internal dependency entry asks for", async () => {
    const manifest = await readManifest();
    const entries = internalEntries(manifest);
    assert.ok(entries.length > 0, "Cargo.toml names no crate of the workspace");
    for (const { name, version } of entries) {
      if (version != null) {
        assert.equal(version, workspaceVersion(manifest), name);
      }
    }
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

  it("is read from each internal entry, whatever the order of its keys", () => {
    const manifest = [
      "[workspace.dependencies]",
      'teachpad-session = { path = "crates/teachpad-session", version = "1.2.3" }',
      'teachpad-demo = { version = "1.2.0", path = "crates/teachpad-demo" }',
      'teachpad-bare = { path = "crates/teachpad-bare" }',
      'serde = { version = "1", features = ["derive"] }',
      "",
      "[workspace.lints.rust]",
      'teachpad-late = { path = "elsewhere", version = "9.9.9" }',
    ].join("\n");
    assert.deepEqual(internalEntries(manifest), [
      { name: "teachpad-session", version: "1.2.3" },
      { name: "teachpad-demo", version: "1.2.0" },
      { name: "teachpad-bare", version: undefined },
    ]);
    assert.deepEqual(internalEntries('[package]\nname = "x"\n'), []);
  });
});
