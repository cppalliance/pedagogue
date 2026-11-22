// Turns an element into its skeleton: tags, classes, the attributes that
// carry meaning, and text. Two elements that draw the same thing have the
// same skeleton, so the markup tests compare the app with the design files
// this way.
//
// In a design file, an element marked `data-sheet` belongs to the sheet
// around the design and is skipped whole. One marked `data-sheet-wrap` only
// wraps a sample, so its children stand in its place.

// Attributes compared by value.
const BY_VALUE = [
  "type",
  "role",
  "aria-label",
  "aria-hidden",
  "aria-live",
  "aria-pressed",
  "scope",
  "placeholder",
  "title",
  "data-act",
  "data-dir",
  "data-edge",
  "autocomplete",
  "spellcheck",
  "max",
  "target",
  "rel",
];

// Attributes that are on or off.
const ON_OR_OFF = ["disabled", "readonly", "hidden"];

// Attributes whose value differs from sample to sample, such as an id made
// from a block id. Only their presence is compared.
const BY_PRESENCE = ["id", "for", "name", "href", "datetime"];

/** Collapses every run of whitespace, no-break spaces included. */
function squeeze(text) {
  return text.replaceAll(/[\s\u00a0]+/g, " ").trim();
}

function nodeLine(element) {
  const tag = element.tagName.toLowerCase();
  const classes = (element.getAttribute("class") ?? "")
    .split(/\s+/)
    .filter((name) => name !== "")
    .toSorted();
  const parts = [[tag, ...classes].join(".")];
  for (const name of BY_VALUE) {
    if (element.hasAttribute(name)) {
      parts.push(`[${name}="${element.getAttribute(name)}"]`);
    }
  }
  for (const name of ON_OR_OFF) {
    if (element.hasAttribute(name)) {
      parts.push(`[${name}]`);
    }
  }
  for (const name of BY_PRESENCE) {
    if (element.hasAttribute(name)) {
      parts.push(`[${name}]`);
    }
  }
  // A field shows its current value, and a box its current tick, whether
  // the markup or a script put it there.
  if (tag === "textarea" || tag === "progress") {
    parts.push(`[value="${element.value}"]`);
  } else if (tag === "input") {
    const type = element.getAttribute("type");
    if (type === "radio" || type === "checkbox") {
      if (element.checked) {
        parts.push("[checked]");
      }
    } else {
      parts.push(`[value="${element.value}"]`);
    }
  }
  return parts.join("");
}

function walk(node, depth, lines) {
  const indent = "  ".repeat(depth);
  if (node.nodeType === 3) {
    const text = squeeze(node.nodeValue);
    if (text !== "") {
      lines.push(`${indent}"${text}"`);
    }
    return;
  }
  if (node.nodeType !== 1 || node.hasAttribute("data-sheet")) {
    return;
  }
  if (node.hasAttribute("data-sheet-wrap")) {
    for (const child of node.childNodes) {
      walk(child, depth, lines);
    }
    return;
  }
  lines.push(`${indent}${nodeLine(node)}`);
  // A text area shows its value, which nodeLine() already wrote.
  if (node.tagName.toLowerCase() === "textarea") {
    return;
  }
  for (const child of node.childNodes) {
    walk(child, depth + 1, lines);
  }
}

/** Returns the skeleton of one element, one line for each node. */
export function skeleton(element) {
  const lines = [];
  walk(element, 0, lines);
  return lines.join("\n");
}

/** Returns the skeletons of several elements, in order, as one text. */
export function skeletons(elements) {
  return [...elements].map((element) => skeleton(element)).join("\n");
}
