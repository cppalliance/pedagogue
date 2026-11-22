// The tutor side: the chat panel beside the page.
import "./chat.css";
import "./composer.css";
import "./chips.css";
import { el } from "../dom";
import type { ChatFrame } from "../protocol";
import { wording } from "../wording";

export class ChatView {
  readonly #place: HTMLElement;
  #chat: ChatFrame | null = null;

  constructor(root: HTMLElement) {
    root.classList.add("panel");
    root.setAttribute("aria-label", wording.chat.panel);

    const head = el("header", "panel-head");
    const avatar = el("div", "avatar", wording.chat.avatar);
    avatar.setAttribute("aria-hidden", "true");
    const who = el("div", "who");
    this.#place = el("span", "who-place");
    who.append(el("h2", "who-name", wording.chat.tutor), this.#place);
    head.append(avatar, who);
    root.append(head);
  }

  /** Takes the chat as the server holds it now. */
  show(frame: ChatFrame): void {
    this.#chat = frame;
  }

  /** Shows which chapter and section the page on screen belongs to. */
  setPlace(chapter: number | undefined, section: string | undefined): void {
    this.#place.textContent = wording.chat.place(chapter, section);
  }

  /** Returns the chat as last shown. */
  get chat(): ChatFrame | null {
    return this.#chat;
  }
}
