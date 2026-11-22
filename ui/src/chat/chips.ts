// The suggestion chips above the composer. A chip sends its text as a
// message from the student.
import "./chips.css";
import { button, el } from "../dom";
import { wording } from "../wording";

export class Chips {
  readonly element: HTMLElement;
  #shown = "";

  constructor() {
    this.element = el("div", "chips");
    this.element.setAttribute("role", "group");
    this.element.setAttribute("aria-label", wording.chat.suggestions);
    this.element.hidden = true;
  }

  /** Shows the chips the tutor suggests. No chips, no row. */
  show(suggestions: readonly string[]): void {
    const next = JSON.stringify(suggestions);
    if (next === this.#shown) {
      return;
    }
    this.#shown = next;
    this.element.replaceChildren(
      ...suggestions.map((text) => button("chip", text)),
    );
    this.element.hidden = suggestions.length === 0;
  }

  /** Returns the text of a chip, or null for anything that is no chip. */
  textOf(pressed: Element): string | null {
    return pressed.classList.contains("chip") ? pressed.textContent : null;
  }
}
