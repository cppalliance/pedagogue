// Small frames for tests that need a page or a chat of their own.
import { readFile } from "node:fs/promises";
import path from "node:path";
import { JSDOM } from "jsdom";

const designDir = path.join(import.meta.dirname, "..", "..", "..", "design");

/** Makes a page frame. `page` overrides the fields of a bare page. */
export function pageFrame(number, pageCount, page = {}) {
  return {
    type: "page",
    page_count: pageCount,
    page: {
      number,
      chapter_opener: number === 1,
      blocks: [],
      footnotes: [],
      continue_pressed: false,
      ...page,
    },
  };
}

/** Makes an exposition block. */
export function exposition(id, markdown) {
  return { id, kind: "exposition", markdown };
}

/** Makes an open choice quiz whose options are the given texts. */
export function choiceQuiz(id, texts, quiz = {}) {
  return {
    id,
    kind: "choice_quiz",
    question: "Which?",
    multiple: false,
    status: "open",
    attempts: 0,
    missing: false,
    options: texts.map((text) => ({ text, state: "open", checked: false })),
    feedback: [],
    ...quiz,
  };
}

/** Makes an open free-text quiz of the given variant. */
export function responseQuiz(id, variant, quiz = {}) {
  return {
    id,
    kind: "response_quiz",
    variant,
    question: "Why?",
    status: "open",
    attempts: 0,
    ...quiz,
  };
}

/** Makes a chat frame. */
export function chatFrame(chat = {}) {
  return {
    type: "chat",
    state: "awaiting_input",
    suggestions: [],
    entries: [],
    ...chat,
  };
}

/** Loads one of the two design files as a document. */
export async function designDocument(file) {
  const html = await readFile(path.join(designDir, file), "utf8");
  return new JSDOM(html).window.document;
}

/** Finds the design sample with the given label in the left gutter. */
export function sample(document, label) {
  const found = document.querySelector(`[data-spec="${label}"]`);
  if (found === null) {
    throw new Error(`the design file has no sample labeled ${label}`);
  }
  return found;
}
