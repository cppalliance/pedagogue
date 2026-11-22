// Contrast of the color tokens, by the WCAG 2.2 AA thresholds: text meets
// 4.5:1 on its background, and field edges, focus rings and icons meet 3:1.
// A palette change is an edit to tokens.css, and this test keeps it legible.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { describe, it } from "node:test";

const TEXT = 4.5;
const EDGE = 3;
// Inline code sits on a tint: its own ink at this strength over the box.
const CODE_TINT = 0.13;

const tokensPath = path.join(import.meta.dirname, "..", "src", "tokens.css");
const css = await readFile(tokensPath, "utf8");

/** Reads the custom properties of one scope, such as `.textbook`. */
function readPalette(scope) {
  const start = css.indexOf(`${scope} {`);
  assert.notEqual(start, -1, `tokens.css has a ${scope} block`);
  const block = css.slice(start, css.indexOf("}", start));
  const palette = new Map();
  for (const [, name, value] of block.matchAll(/(--[\w-]+):\s*([^;]+);/g)) {
    palette.set(name, value.trim());
  }
  return palette;
}

/** Parses a hex color into its three channels, each from 0 to 255. */
function parseHex(value) {
  const digits = /^#([0-9a-f]{3}|[0-9a-f]{6})$/i.exec(value)?.[1];
  assert.ok(digits !== undefined, `${value} is a hex color`);
  const full = digits.length === 3 ? digits.replace(/./g, "$&$&") : digits;
  return [0, 2, 4].map((at) => Number.parseInt(full.slice(at, at + 2), 16));
}

/** Lays a color over another at the given strength. */
function mix(over, under, strength) {
  return over.map((channel, at) =>
    Math.round(channel * strength + under[at] * (1 - strength)),
  );
}

/** Relative luminance of a color, as WCAG defines it. */
function luminance(color) {
  const [r, g, b] = color.map((channel) => {
    const share = channel / 255;
    return share <= 0.04045 ? share / 12.92 : ((share + 0.055) / 1.055) ** 2.4;
  });
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

/** Contrast ratio of two colors, from 1 to 21. */
function contrast(a, b) {
  const [light, dark] = [luminance(a), luminance(b)].toSorted((x, y) => y - x);
  return (light + 0.05) / (dark + 0.05);
}

function checkPairs(palette, pairs) {
  for (const [front, back, minimum] of pairs) {
    const ratio = contrast(
      parseHex(palette.get(front)),
      parseHex(palette.get(back)),
    );
    assert.ok(
      ratio >= minimum,
      `${front} on ${back} is ${ratio.toFixed(2)}:1, under ${minimum}:1`,
    );
  }
}

describe("the textbook palette", () => {
  const palette = readPalette(".textbook");

  it("keeps text at 4.5:1 on its background", () => {
    checkPairs(palette, [
      ["--ink", "--paper", TEXT],
      ["--ink-soft", "--paper", TEXT],
      ["--link", "--paper", TEXT],
      ["--teal", "--paper", TEXT],
      ["--chart-ink", "--paper", TEXT],
      ["--tree-ink", "--paper", TEXT],
      ["--ink", "--info-bg", TEXT],
      ["--info-ink", "--info-bg", TEXT],
      ["--ink", "--note-bg", TEXT],
      ["--note-ink", "--note-bg", TEXT],
      ["--ink", "--quiz-bg", TEXT],
      ["--ink-soft", "--quiz-bg", TEXT],
      ["--quiz-ink", "--quiz-bg", TEXT],
      ["--ok-ink", "--quiz-bg", TEXT],
      ["--ink", "--ok-bg", TEXT],
      ["--ok-ink", "--ok-bg", TEXT],
      ["--on-accent", "--quiz-bd", TEXT],
      ["--on-accent", "--quiz-bd-hover", TEXT],
      ["--on-accent", "--link", TEXT],
      ["--ink", "--neutral-bg", TEXT],
      ["--ink", "--code-bg", TEXT],
      ["--code-kw", "--code-bg", TEXT],
      ["--code-ty", "--code-bg", TEXT],
      ["--code-st", "--code-bg", TEXT],
      ["--code-cm", "--code-bg", TEXT],
      ["--code-fn", "--code-bg", TEXT],
    ]);
  });

  it("keeps inline code at 4.5:1 on its tint", () => {
    const boxes = [
      ["--ink", "--code-tint", "--paper"],
      ["--info-ink", "--info-ink", "--info-bg"],
      ["--note-ink", "--note-ink", "--note-bg"],
      ["--quiz-ink", "--quiz-ink", "--quiz-bg"],
      ["--ok-ink", "--ok-ink", "--ok-bg"],
    ];
    for (const [ink, tint, box] of boxes) {
      const background = mix(
        parseHex(palette.get(tint)),
        parseHex(palette.get(box)),
        CODE_TINT,
      );
      const ratio = contrast(parseHex(palette.get(ink)), background);
      assert.ok(
        ratio >= TEXT,
        `${ink} code on ${box} is ${ratio.toFixed(2)}:1, under ${TEXT}:1`,
      );
    }
  });

  it("keeps field edges, focus rings and icons at 3:1", () => {
    checkPairs(palette, [
      ["--field-bd", "--paper", EDGE],
      ["--field-bd", "--quiz-bg", EDGE],
      ["--focus-ring", "--paper", EDGE],
      ["--focus-ring", "--quiz-bg", EDGE],
      ["--quiz-bd", "--paper", EDGE],
      ["--quiz-bd", "--quiz-bg", EDGE],
      ["--cue", "--paper", EDGE],
    ]);
  });
});

describe("the tutor palette", () => {
  const palette = readPalette(".tutor");

  it("keeps text at 4.5:1 on its background", () => {
    checkPairs(palette, [
      ["--text", "--bg", TEXT],
      ["--text", "--surface", TEXT],
      ["--text", "--in", TEXT],
      ["--on-accent", "--out", TEXT],
      ["--text-soft", "--bg", TEXT],
      ["--text-soft", "--surface", TEXT],
      ["--muted", "--bg", TEXT],
      ["--ok", "--surface", TEXT],
      ["--placeholder", "--surface", TEXT],
    ]);
  });

  it("keeps field edges, focus rings and icons at 3:1", () => {
    checkPairs(palette, [
      ["--composer-bd", "--bg", EDGE],
      ["--composer-bd", "--surface", EDGE],
      ["--accent", "--bg", EDGE],
      ["--accent", "--surface", EDGE],
      ["--muted", "--bg", EDGE],
      ["--ok", "--bg", EDGE],
      ["--text-soft", "--surface", EDGE],
      ["--rec", "--surface", EDGE],
      ["--on-accent", "--out-hover", EDGE],
      ["--on-accent", "--rec", EDGE],
      ["--on-accent", "--rec-hover", EDGE],
    ]);
  });
});
