// The binding rules of the HTML and CSS rulebook, as checks on a document.
// One set of checks runs over the two design files and over the page the
// app draws, so both stay in line with the rulebook.

const HEADINGS = "h1, h2, h3, h4, h5, h6";
const FIELDS = "input:not([type=hidden]), textarea, select";

/** Whether an element, or one of its ancestors, has the hidden attribute. */
function isHidden(element) {
  return element.closest("[hidden]") !== null;
}

/** A short name for an element in a problem line. */
function describe(element) {
  const id = element.id === "" ? "" : `#${element.id}`;
  const classes = [...element.classList].map((name) => `.${name}`).join("");
  return `<${element.localName}${id}${classes}>`;
}

/** Whether a control has a name a screen reader can say. */
function hasName(element) {
  const label = element.getAttribute("aria-label") ?? "";
  return label.trim() !== "" || element.textContent.trim() !== "";
}

/**
 * Lists every rule a document breaks.
 *
 * @param {Document} document The document to check.
 * @returns {string[]} One line per problem. An empty list means the document
 *   follows the rules.
 */
export function findHtmlProblems(document) {
  const problems = [];
  const exposed = (selector) =>
    [...document.querySelectorAll(selector)].filter(
      (element) => !isHidden(element),
    );

  if ((document.documentElement.getAttribute("lang") ?? "") === "") {
    problems.push("<html> has no lang");
  }

  const mains = exposed("main");
  if (mains.length !== 1) {
    problems.push(`expected one <main>, found ${mains.length}`);
  }

  const headings = exposed(HEADINGS);
  const firstLevels = headings.filter((h) => h.localName === "h1");
  if (firstLevels.length !== 1) {
    problems.push(`expected one exposed <h1>, found ${firstLevels.length}`);
  }
  let previous = 0;
  for (const heading of headings) {
    const level = Number(heading.localName.slice(1));
    if (level > previous + 1) {
      problems.push(
        `${describe(heading)} skips from level ${previous} to level ${level}`,
      );
    }
    previous = level;
  }

  for (const field of exposed(FIELDS)) {
    if (field.labels.length === 0) {
      problems.push(`${describe(field)} has no <label>`);
    }
  }

  for (const button of exposed("button")) {
    if (!button.hasAttribute("type")) {
      problems.push(`${describe(button)} has no type`);
    }
    if (!hasName(button)) {
      problems.push(`${describe(button)} has no name`);
    }
  }

  for (const bar of exposed("progress, meter")) {
    if (!hasName(bar) && bar.labels.length === 0) {
      problems.push(`${describe(bar)} has no name`);
    }
  }

  for (const link of exposed("a")) {
    if ((link.getAttribute("href") ?? "") === "") {
      problems.push(`${describe(link)} has no href`);
    }
  }

  for (const image of exposed("img")) {
    for (const attribute of ["alt", "width", "height"]) {
      if (!image.hasAttribute(attribute)) {
        problems.push(`${describe(image)} has no ${attribute}`);
      }
    }
  }

  for (const drawing of exposed("svg")) {
    const isNamed =
      drawing.getAttribute("role") === "img" &&
      (drawing.getAttribute("aria-label") ?? "") !== "";
    if (!isNamed && drawing.getAttribute("aria-hidden") !== "true") {
      problems.push(
        `${describe(drawing)} is neither a named image nor aria-hidden`,
      );
    }
  }

  const seen = new Set();
  for (const element of document.querySelectorAll("[id]")) {
    if (seen.has(element.id)) {
      problems.push(`id "${element.id}" is used twice`);
    }
    seen.add(element.id);
  }

  for (const element of document.querySelectorAll("*")) {
    for (const { name } of element.attributes) {
      if (name.startsWith("on")) {
        problems.push(`${describe(element)} has an inline ${name} handler`);
      }
    }
    if (element.hasAttribute("style")) {
      problems.push(`${describe(element)} has an inline style`);
    }
    if (Number(element.getAttribute("tabindex")) > 0) {
      problems.push(`${describe(element)} has a positive tabindex`);
    }
    if (
      element.getAttribute("role") === "button" &&
      element.localName !== "button"
    ) {
      problems.push(`${describe(element)} plays a button without being one`);
    }
  }

  return problems;
}
