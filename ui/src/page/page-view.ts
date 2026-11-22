// The textbook side: the pages of the lesson, one on screen at a time.
//
// The server sends whole pages. This view matches blocks by id, draws the
// new and the changed ones, and leaves the rest alone, which keeps the
// scroll position and whatever the student is typing.
import "./page.css";
import { button, el } from "../dom";
import { renderInline } from "../markdown";
import type {
  Block,
  BlockId,
  Page,
  PageFrame,
  PageNumber,
  SendFrame,
} from "../protocol";
import { wording } from "../wording";
import { renderCallout, renderProgress, renderSuccess } from "./blocks/callout";
import { renderExposition } from "./blocks/exposition";
import {
  renderBarChart,
  renderTimeline,
  renderTreeDiagram,
} from "./blocks/figures";
import { type Quiz, renderChoiceQuiz, renderResponseQuiz } from "./blocks/quiz";
import { ScrollCues } from "./scroll-cues";

/** Tells the other side which chapter and section are on screen. */
export type ShowPlace = (
  chapter: number | undefined,
  section: string | undefined,
) => void;

/** One block as drawn: the data it was drawn from, and its elements. */
interface DrawnBlock {
  data: string;
  elements: Element[];
}

/** One page as drawn. Each page keeps its own elements for good. */
interface DrawnPage {
  page: Page;
  scroller: HTMLElement;
  article: HTMLElement;
  headTitle: HTMLElement;
  headNumber: HTMLElement;
  chapterNumber: HTMLElement;
  title: HTMLElement;
  section: HTMLElement;
  blocks: Map<BlockId, DrawnBlock>;
  footnotes: HTMLElement;
  previous: HTMLButtonElement;
  forward: HTMLButtonElement;
}

export class PageView {
  readonly #root: HTMLElement;
  readonly #send: SendFrame;
  readonly #showPlace: ShowPlace;
  readonly #cues: ScrollCues;
  readonly #pages = new Map<PageNumber, DrawnPage>();
  // Each quiz form on a page, with what to do when the student acts on it.
  readonly #quizzes = new WeakMap<Element, Quiz>();
  #pageCount = 0;
  #shown: PageNumber | null = null;

  constructor(root: HTMLElement, send: SendFrame, showPlace: ShowPlace) {
    this.#root = root;
    this.#send = send;
    this.#showPlace = showPlace;
    const viewport = el("div", "viewport");
    this.#cues = new ScrollCues(viewport);
    root.append(viewport);
  }

  /** Takes one page as the server holds it now. */
  show(frame: PageFrame): void {
    const number = frame.page.number;
    const newest = this.#newest();
    let drawn = this.#pages.get(number);
    const isNew = drawn === undefined;
    if (drawn === undefined) {
      drawn = this.#addPage(frame.page);
      this.#pages.set(number, drawn);
    }
    drawn.page = frame.page;
    this.#pageCount = Math.max(
      this.#pageCount,
      frame.page_count,
      this.#pages.size,
    );
    drawHead(drawn);
    this.#drawBlocks(drawn);
    drawFootnotes(drawn);
    for (const each of this.#pages.values()) {
      this.#drawButtons(each);
    }
    // A new page comes on screen when the student is on the newest page.
    // A student who reads an older page gets there with the forward button.
    if (this.#shown === null || (isNew && this.#shown === newest)) {
      this.#turnTo(number);
    } else if (this.#shown === number) {
      this.#showPlace(frame.page.chapter_number, frame.page.section_number);
    }
  }

  #newest(): PageNumber | null {
    let newest: PageNumber | null = null;
    for (const number of this.#pages.keys()) {
      if (newest === null || number > newest) {
        newest = number;
      }
    }
    return newest;
  }

  #addPage(page: Page): DrawnPage {
    const article = el("article", "page");
    const headTitle = el("span", "rh-title");
    const headNumber = el("span", "rh-page");
    const head = el("header", "running-head");
    head.append(headTitle, headNumber);
    const chapterNumber = el("p", "chapter-num");
    const title = el("h1", "chapter-title");
    const section = el("h2");
    const footnotes = el("footer", "footnotes");
    const previous = button("btn neutral", wording.page.previous);
    const forward = button("btn continue");
    const end = el("nav", "page-end");
    end.setAttribute("aria-label", wording.page.pages);
    end.append(previous, forward);
    article.append(head, chapterNumber, title, section, footnotes, end);

    // One listener of each kind for the page finds what the student used.
    article.addEventListener("click", (event) => {
      const pressed =
        event.target instanceof Element ? event.target.closest("button") : null;
      if (pressed === previous) {
        this.#turnBy(page.number, -1);
      } else if (pressed === forward) {
        this.#goForward(page.number);
      } else if (pressed !== null) {
        this.#quizOf(pressed)?.press(pressed);
      }
    });
    // A quiz is a form, so its button and the Enter key do the same thing.
    article.addEventListener("submit", (event) => {
      event.preventDefault();
      this.#quizOf(event.target)?.submit();
    });
    article.addEventListener("input", (event) => {
      this.#quizOf(event.target)?.type();
    });

    // The first page takes the place of the heading the shell shows alone.
    const placeholder = this.#root.querySelector<HTMLElement>(":scope > h1");
    if (placeholder !== null) {
      placeholder.hidden = true;
    }
    return {
      page,
      scroller: this.#cues.addScroller(article),
      article,
      headTitle,
      headNumber,
      chapterNumber,
      title,
      section,
      blocks: new Map(),
      footnotes,
      previous,
      forward,
    };
  }

  #drawBlocks(drawn: DrawnPage): void {
    let hasHeadingAbove = !drawn.section.hidden;
    for (const block of drawn.page.blocks) {
      const data = JSON.stringify(block);
      const before = drawn.blocks.get(block.id);
      if (before?.data !== data) {
        const elements = this.#render(block, hasHeadingAbove);
        if (before === undefined) {
          // Pages only grow, so a new block goes last, above the foot.
          drawn.footnotes.before(...elements);
        } else {
          replace(before.elements, elements, drawn.footnotes);
        }
        drawn.blocks.set(block.id, { data, elements });
      }
      const elements = drawn.blocks.get(block.id)?.elements ?? [];
      hasHeadingAbove ||= elements.some(
        (element) =>
          element.tagName === "H2" || element.querySelector("h2") !== null,
      );
    }
  }

  #render(block: Block, hasHeadingAbove: boolean): Element[] {
    switch (block.kind) {
      case "exposition":
        return renderExposition(block, hasHeadingAbove);
      case "callout":
        return renderCallout(block);
      case "bar_chart":
        return renderBarChart(block);
      case "tree_diagram":
        return renderTreeDiagram(block);
      case "timeline":
        return renderTimeline(block);
      case "progress":
        return renderProgress(block);
      case "success":
        return renderSuccess(block);
      case "choice_quiz":
        return this.#keep(renderChoiceQuiz(block, this.#send));
      case "response_quiz":
        return this.#keep(renderResponseQuiz(block, this.#send));
      default: {
        // Every kind has a case above, and the compiler checks that here.
        const unknown: never = block;
        return unknown;
      }
    }
  }

  #keep(quiz: Quiz): Element[] {
    this.#quizzes.set(quiz.form, quiz);
    return [quiz.form];
  }

  #quizOf(target: EventTarget | null): Quiz | undefined {
    const form = target instanceof Element ? target.closest("form") : null;
    return form === null ? undefined : this.#quizzes.get(form);
  }

  /**
   * Draws the two buttons at the foot of a page. Previous goes back a
   * page. The second button goes forward a page, and on the newest page it
   * is the tutor's Continue button. A button shows only when it has
   * somewhere to go.
   */
  #drawButtons(drawn: DrawnPage): void {
    const { page, previous, forward } = drawn;
    const isNewest = page.number >= this.#pageCount;
    previous.hidden = page.number <= 1;
    if (isNewest) {
      forward.textContent = page.continue_label ?? "";
      forward.hidden = page.continue_label === undefined;
      forward.disabled = page.continue_pressed;
    } else {
      forward.textContent = wording.page.next;
      forward.hidden = false;
      forward.disabled = false;
    }
  }

  #goForward(number: PageNumber): void {
    if (number < this.#pageCount) {
      this.#turnBy(number, 1);
    } else {
      this.#send({ type: "continue", page: number });
    }
  }

  #turnBy(from: PageNumber, step: number): void {
    for (const number of this.#pages.keys()) {
      if (number === from + step) {
        this.#turnTo(number);
      }
    }
  }

  #turnTo(number: PageNumber): void {
    const drawn = this.#pages.get(number);
    if (drawn === undefined) {
      return;
    }
    this.#shown = number;
    this.#cues.show(drawn.scroller);
    this.#showPlace(drawn.page.chapter_number, drawn.page.section_number);
  }
}

/** Draws the running head, the chapter title and the section heading. */
function drawHead(drawn: DrawnPage): void {
  const { page } = drawn;
  const chapter =
    page.chapter_number === undefined
      ? ""
      : wording.page.chapter(page.chapter_number);
  const headParts = [chapter, page.chapter_title ?? ""].filter(
    (part) => part !== "",
  );
  drawn.headTitle.textContent =
    headParts.length === 0 ? wording.page.untitled : headParts.join(" \u00a0 ");
  drawn.headNumber.textContent = String(page.number);

  // Every page opens with one h1. It is drawn large where a chapter opens
  // and kept for screen readers alone on the pages after.
  const opensChapter = page.chapter_opener && page.chapter_title !== undefined;
  drawn.chapterNumber.textContent = chapter;
  drawn.chapterNumber.hidden = !opensChapter || chapter === "";
  drawn.title.textContent = page.chapter_title ?? wording.page.untitled;
  drawn.title.className = opensChapter
    ? "chapter-title"
    : "chapter-title visually-hidden";

  const sectionParts = [
    page.section_number ?? "",
    page.section_title ?? "",
  ].filter((part) => part !== "");
  drawn.section.replaceChildren(renderInline(sectionParts.join(" ")));
  drawn.section.hidden = sectionParts.length === 0;
}

function drawFootnotes(drawn: DrawnPage): void {
  const notes = drawn.page.footnotes.map((note, index) => {
    const line = el("p");
    line.append(
      el("sup", "fn-ref", String(index + 1)),
      " ",
      renderInline(note),
    );
    return line;
  });
  drawn.footnotes.replaceChildren(...notes);
  drawn.footnotes.hidden = notes.length === 0;
}

/** Puts new elements where old ones stand, or above `fallback`. */
function replace(old: Element[], next: Element[], fallback: Element): void {
  const first = old[0];
  if (first === undefined) {
    fallback.before(...next);
  } else {
    first.before(...next);
  }
  for (const element of old) {
    element.remove();
  }
}
