// Bundles src/main.ts into app.js and app.css and writes index.html beside
// them: the three files the server delivers. `--out <dir>` picks the output
// folder (default dist/) and `--watch` rebuilds on save. The server crate's
// build script runs this same file into OUT_DIR, so the jsdom tests and the
// app load the same bundle.
import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { parseArgs } from "node:util";
import * as esbuild from "esbuild";
import { collectSprites } from "./sprites.mjs";

const uiDir = import.meta.dirname;
const designDir = path.join(uiDir, "..", "design");
const DESIGN_FILES = ["page-contact-sheet.html", "chat-panel.html"];
const ICON_MARKER = "<!-- icons -->";

const { values: flags } = parseArgs({
  options: {
    out: { type: "string" },
    watch: { type: "boolean", default: false },
  },
});
const outDir =
  flags.out === undefined
    ? path.join(uiDir, "dist")
    : path.resolve(uiDir, flags.out);

async function readDesignFiles() {
  return Promise.all(
    DESIGN_FILES.map(async (name) => ({
      name,
      html: await readFile(path.join(designDir, name), "utf8"),
    })),
  );
}

// Writes index.html with the design files' icon sprites in place of the
// marker. It runs after every build, watch rebuilds included.
const writeIndexHtml = {
  name: "write-index-html",
  setup(build) {
    build.onEnd(async (result) => {
      if (result.errors.length > 0) {
        return;
      }
      const page = await readFile(path.join(uiDir, "index.html"), "utf8");
      if (!page.includes(ICON_MARKER)) {
        throw new Error(`index.html has no ${ICON_MARKER} marker`);
      }
      const sprites = collectSprites(await readDesignFiles());
      await mkdir(outDir, { recursive: true });
      await writeFile(
        path.join(outDir, "index.html"),
        page.replace(ICON_MARKER, sprites),
      );
    });
  },
};

// The bundle stays unminified: it travels over loopback, where size costs
// nothing, and an error in the webview then points at a real line.
const options = {
  entryPoints: [path.join(uiDir, "src", "main.ts")],
  bundle: true,
  format: "esm",
  target: "es2022",
  entryNames: "app",
  outdir: outDir,
  logLevel: "info",
  plugins: [writeIndexHtml],
};

if (flags.watch) {
  const context = await esbuild.context(options);
  await context.watch();
  console.log("watching ui/src for changes...");
} else {
  await esbuild.build(options);
}
