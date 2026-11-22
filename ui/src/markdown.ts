// Markdown to DOM nodes. Every text field of a page command is markdown,
// and this is the one place where markup from outside becomes part of the
// page: marked writes HTML, DOMPurify reduces it to a short list of tags,
// and the nodes that come back are given the classes of the design. Raw
// HTML and images in the source are left out.
import DOMPurify from "dompurify";
import hljs from "highlight.js/lib/core";
import bash from "highlight.js/lib/languages/bash";
import cpp from "highlight.js/lib/languages/cpp";
import javascript from "highlight.js/lib/languages/javascript";
import json from "highlight.js/lib/languages/json";
import python from "highlight.js/lib/languages/python";
import rust from "highlight.js/lib/languages/rust";
import typescript from "highlight.js/lib/languages/typescript";
import {
  Marked,
  type TokenizerAndRendererExtension,
  type Tokens,
} from "marked";
import { el } from "./dom";
import { wording } from "./wording";

hljs.registerLanguage("bash", bash);
hljs.registerLanguage("cpp", cpp);
hljs.registerLanguage("javascript", javascript);
hljs.registerLanguage("json", json);
hljs.registerLanguage("python", python);
hljs.registerLanguage("rust", rust);
hljs.registerLanguage("typescript", typescript);

const ALLOWED_TAGS = [
  "a",
  "blockquote",
  "br",
  "code",
  "del",
  "em",
  "h1",
  "h2",
  "h3",
  "h4",
  "h5",
  "h6",
  "hr",
  "li",
  "ol",
  "p",
  "pre",
  "span",
  "strong",
  "sup",
  "table",
  "tbody",
  "td",
  "th",
  "thead",
  "tr",
  "ul",
];
const ALLOWED_ATTR = ["align", "class", "href", "start"];

// The highlighter's token kinds, mapped onto the five code colors of the
// design. A token of any other kind is drawn as plain code.
const TOKEN_CLASS = new Map([
  ["hljs-keyword", "kw"],
  ["hljs-literal", "kw"],
  ["hljs-meta", "kw"],
  ["hljs-type", "ty"],
  ["hljs-attr", "ty"],
  ["hljs-built_in", "fn"],
  ["hljs-string", "st"],
  ["hljs-number", "st"],
  ["hljs-regexp", "st"],
  ["hljs-symbol", "st"],
  ["hljs-comment", "cm"],
  ["hljs-doctag", "cm"],
  ["hljs-title", "fn"],
]);

function escapeHtml(text: string): string {
  return text
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

function highlight(code: string, language: string | undefined): string {
  if (language === undefined || hljs.getLanguage(language) === undefined) {
    return escapeHtml(code);
  }
  return hljs.highlight(code, { language, ignoreIllegals: true }).value;
}

// `[^1]` in the text is the marker of footnote 1 at the foot of the page.
const footnoteMarker: TokenizerAndRendererExtension = {
  name: "footnoteMarker",
  level: "inline",
  start(source) {
    const at = source.indexOf("[^");
    return at === -1 ? undefined : at;
  },
  tokenizer(source) {
    const match = /^\[\^(\d+)\]/.exec(source);
    if (match === null) {
      return undefined;
    }
    return { type: "footnoteMarker", raw: match[0], text: match[1] ?? "" };
  },
  renderer(token) {
    return `<sup class="fn-ref">${escapeHtml(String(token["text"]))}</sup>`;
  },
};

const marked = new Marked({
  gfm: true,
  extensions: [footnoteMarker],
  renderer: {
    html(): string {
      return "";
    },
    image({ text }: Tokens.Image): string {
      return escapeHtml(text);
    },
    code({ text, lang }: Tokens.Code): string {
      const language = lang?.trim().split(/\s+/)[0];
      return `<pre class="code"><code>${highlight(text, language)}</code></pre>\n`;
    },
  },
});

function sanitize(html: string): DocumentFragment {
  return DOMPurify.sanitize(html, {
    ALLOWED_TAGS,
    ALLOWED_ATTR,
    RETURN_DOM_FRAGMENT: true,
  });
}

/** Swaps an element for one of another tag with the same content. */
function retag(element: Element, tag: "h2" | "h3"): void {
  const next = el(tag);
  next.append(...element.childNodes);
  element.replaceWith(next);
}

/**
 * Brings headings onto the two sizes of the design. `#` and `##` are drawn
 * as h2 and deeper levels as h3. Until the page has an h2 above, a deeper
 * heading is drawn as h2 too, so levels step down by one from the page's
 * h1.
 */
function fitHeadings(
  fragment: DocumentFragment,
  hasHeadingAbove: boolean,
): void {
  let hasSecondLevel = hasHeadingAbove;
  for (const heading of fragment.querySelectorAll("h1, h2, h3, h4, h5, h6")) {
    const isDeep = !["H1", "H2"].includes(heading.tagName);
    if (isDeep && hasSecondLevel) {
      retag(heading, "h3");
    } else {
      retag(heading, "h2");
      hasSecondLevel = true;
    }
  }
}

function fitTables(fragment: DocumentFragment): void {
  for (const table of fragment.querySelectorAll("table")) {
    table.className = "data";
  }
  for (const cell of fragment.querySelectorAll("th, td")) {
    if (cell.tagName === "TH") {
      cell.setAttribute("scope", "col");
    }
    const align = cell.getAttribute("align");
    cell.removeAttribute("align");
    if (align === "left" || align === "right") {
      cell.className = `align-${align}`;
    }
  }
}

/** Sends each link outside the app, and says so to a screen reader. */
function fitLinks(fragment: DocumentFragment): void {
  for (const link of fragment.querySelectorAll("a")) {
    link.setAttribute("target", "_blank");
    link.setAttribute("rel", "noopener");
    link.append(el("span", "visually-hidden", wording.page.opensNewWindow));
  }
}

function fitCode(fragment: DocumentFragment): void {
  for (const token of fragment.querySelectorAll("pre span")) {
    const kind = [...token.classList].find((name) => TOKEN_CLASS.has(name));
    const color = token.classList.contains("class_")
      ? "ty"
      : TOKEN_CLASS.get(kind ?? "");
    if (color === undefined) {
      token.replaceWith(...token.childNodes);
    } else {
      token.className = color;
    }
  }
  // Unwrapping a token leaves its text beside its neighbors' text.
  for (const block of fragment.querySelectorAll("pre")) {
    block.normalize();
  }
}

/** Marks emphasis that opens a list item as the lead-in of that item. */
function fitLists(fragment: DocumentFragment): void {
  for (const item of fragment.querySelectorAll("li")) {
    const first = item.firstChild;
    if (first instanceof Element && first.tagName === "EM") {
      first.className = "lead";
    }
  }
}

function fit(fragment: DocumentFragment, hasHeadingAbove: boolean): void {
  fitHeadings(fragment, hasHeadingAbove);
  fitTables(fragment);
  fitLinks(fragment);
  fitCode(fragment);
  fitLists(fragment);
}

/**
 * Renders markdown as the elements of the page. `hasHeadingAbove` says
 * whether the page already holds an h2 above this text.
 */
export function renderMarkdown(
  source: string,
  hasHeadingAbove: boolean,
): Element[] {
  const fragment = sanitize(marked.parse(source, { async: false }));
  fit(fragment, hasHeadingAbove);
  return [...fragment.children];
}

/** Renders one line of markdown as inline content. */
export function renderInline(source: string): DocumentFragment {
  const fragment = sanitize(marked.parseInline(source, { async: false }));
  fit(fragment, true);
  return fragment;
}
