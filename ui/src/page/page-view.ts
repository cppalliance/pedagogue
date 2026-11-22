// The textbook side: the pages of the lesson, one on screen at a time.
import "./page.css";
import "./scroll-cues.css";
import "./blocks/exposition.css";
import "./blocks/callout.css";
import "./blocks/quiz.css";
import "./blocks/figures.css";
import "./blocks/progress.css";
import { el } from "../dom";
import type { Page, PageFrame, PageNumber } from "../protocol";

export class PageView {
  readonly #viewport: HTMLElement;
  readonly #pages = new Map<PageNumber, Page>();

  constructor(root: HTMLElement) {
    this.#viewport = el("div", "viewport");
    root.append(this.#viewport);
  }

  /** Takes one page as the server holds it now. */
  show(frame: PageFrame): void {
    this.#pages.set(frame.page.number, frame.page);
    this.#viewport.dataset["pages"] = String(frame.page_count);
  }
}
