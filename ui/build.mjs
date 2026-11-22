// Bundles src/main.ts into app.js and app.css and writes index.html beside
// them: the three files the server delivers. `--out <dir>` picks the output
// folder (default dist/) and `--watch` rebuilds on save. The server crate's
// build script runs this same file into OUT_DIR, so the jsdom tests and the
// app load the same bundle.
import { copyFile, mkdir } from "node:fs/promises";
import path from "node:path";
import { parseArgs } from "node:util";
import * as esbuild from "esbuild";

const uiDir = import.meta.dirname;

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

const writeIndexHtml = {
  name: "write-index-html",
  setup(build) {
    build.onEnd(async (result) => {
      if (result.errors.length > 0) {
        return;
      }
      await mkdir(outDir, { recursive: true });
      await copyFile(
        path.join(uiDir, "index.html"),
        path.join(outDir, "index.html"),
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
