// The tutor side: the chat panel beside the page.
//
// The server sends the whole chat. A chat only grows, so this view draws
// the entries it has yet to draw and leaves the rest alone, which lets a
// screen reader announce just the new reply.
import "./chat.css";
import { button, el, icon } from "../dom";
import { renderInline, renderMarkdown } from "../markdown";
import type { ChatEntry, ChatFrame, SendFrame } from "../protocol";
import { wording } from "../wording";
import { Chips } from "./chips";
import { Composer } from "./composer";

// A time line shows before the first message and after a gap this long.
const GAP_MS = 15 * 60 * 1000;
// How long the copy button stays green, and how long read-aloud shows.
const COPIED_MS = 1200;
const SPEAKING_MS = 3000;

type Speaker = "assistant" | "user";

/** The run of bubbles that the next message of the same speaker joins. */
interface Run {
  speaker: Speaker;
  message: HTMLElement;
}

/** What the chat needs from outside: the socket, the clock, the clipboard. */
export interface ChatEdges {
  send: SendFrame;
  now: () => Date;
  copy: (text: string) => Promise<void>;
}

function twoDigits(value: number): string {
  return String(value).padStart(2, "0");
}

/** Makes the centered time line for an entry, such as "Today 9:41 AM". */
function timeLine(at: number, now: Date): HTMLElement {
  const date = new Date(at);
  const isToday = date.toDateString() === now.toDateString();
  const day = isToday
    ? wording.chat.today
    : date.toLocaleDateString("en-US", { month: "short", day: "numeric" });
  const hour = date.getHours() % 12 === 0 ? 12 : date.getHours() % 12;
  const half = date.getHours() < 12 ? "AM" : "PM";
  const line = el(
    "time",
    "stamp",
    `${day} ${hour}:${twoDigits(date.getMinutes())} ${half}`,
  );
  line.setAttribute(
    "datetime",
    `${date.getFullYear()}-${twoDigits(date.getMonth() + 1)}-${twoDigits(date.getDate())}` +
      `T${twoDigits(date.getHours())}:${twoDigits(date.getMinutes())}`,
  );
  return line;
}

function message(speaker: Speaker): HTMLElement {
  const isTutor = speaker === "assistant";
  const node = el("div", isTutor ? "msg in" : "msg out");
  node.append(
    el(
      "span",
      "visually-hidden",
      isTutor ? wording.chat.tutor : wording.chat.you,
    ),
  );
  return node;
}

function iconButton(
  action: string,
  name: string,
  iconName: string,
): HTMLButtonElement {
  const node = button("");
  node.dataset["act"] = action;
  node.setAttribute("aria-label", name);
  node.title = name;
  node.append(icon(iconName));
  return node;
}

export class ChatView {
  readonly #edges: ChatEdges;
  readonly #place: HTMLElement;
  readonly #log: HTMLElement;
  readonly #chips = new Chips();
  readonly #composer: Composer;
  readonly #actions: HTMLElement;
  readonly #copyButton: HTMLButtonElement;
  readonly #speakButton: HTMLButtonElement;
  // The data of each entry on screen, to tell which entries are new.
  #drawn: string[] = [];
  #run: Run | null = null;
  #lastAt: number | null = null;
  #newestReply: { message: HTMLElement; text: string } | null = null;
  #typing: HTMLElement | null = null;

  constructor(root: HTMLElement, edges: ChatEdges) {
    this.#edges = edges;
    root.classList.add("panel");
    root.setAttribute("aria-label", wording.chat.panel);

    const head = el("header", "panel-head");
    const avatar = el("div", "avatar", wording.chat.avatar);
    avatar.setAttribute("aria-hidden", "true");
    const who = el("div", "who");
    this.#place = el("span", "who-place");
    who.append(el("h2", "who-name", wording.chat.tutor), this.#place);
    head.append(avatar, who);

    // A live region, so a screen reader announces each new reply.
    this.#log = el("div", "messages");
    this.#log.setAttribute("role", "log");
    this.#log.setAttribute("aria-live", "polite");
    this.#log.setAttribute("aria-label", wording.chat.conversation);

    // One icon row, which sits under the newest tutor message.
    this.#actions = el("div", "actions");
    this.#actions.setAttribute("role", "group");
    this.#actions.setAttribute("aria-label", wording.chat.actions);
    this.#copyButton = iconButton("copy", wording.chat.copy, "copy");
    this.#speakButton = iconButton("speak", wording.chat.readAloud, "speaker");
    this.#speakButton.setAttribute("aria-pressed", "false");
    const wave = el("span", "wave");
    wave.setAttribute("aria-hidden", "true");
    wave.append(
      el("span", "wave-bar"),
      el("span", "wave-bar"),
      el("span", "wave-bar"),
    );
    this.#actions.append(
      this.#copyButton,
      this.#speakButton,
      wave,
      iconButton("regen", wording.chat.regenerate, "refresh"),
    );

    this.#composer = new Composer((text) => {
      edges.send({ type: "say", text });
    });
    root.append(head, this.#log, this.#chips.element, this.#composer.form);

    // One listener of each kind for the panel finds what the student used.
    root.addEventListener("click", (event) => {
      const pressed =
        event.target instanceof Element ? event.target.closest("button") : null;
      if (pressed !== null) {
        this.#press(pressed);
      }
    });
    root.addEventListener("submit", (event) => {
      event.preventDefault();
      this.#composer.submit();
    });
    root.addEventListener("input", () => {
      this.#composer.type();
    });
  }

  /** Takes the chat as the server holds it now. */
  show(frame: ChatFrame): void {
    const entries = frame.entries.map((entry) => JSON.stringify(entry));
    const isSameChat = this.#drawn.every(
      (data, index) => entries[index] === data,
    );
    if (!isSameChat) {
      // The server holds another chat, as after a restart. Start over.
      this.#log.replaceChildren();
      this.#drawn = [];
      this.#run = null;
      this.#lastAt = null;
      this.#newestReply = null;
    }
    this.#typing?.remove();
    this.#typing = null;
    const fresh = frame.entries.slice(this.#drawn.length);
    for (const entry of fresh) {
      this.#add(entry);
    }
    this.#drawn = entries;
    if (this.#newestReply !== null) {
      this.#newestReply.message.append(this.#actions);
    }
    if (frame.state === "working") {
      this.#typing = message("assistant");
      const dots = el("div", "bubble in tail typing");
      dots.append(
        el("span", "typing-dot"),
        el("span", "typing-dot"),
        el("span", "typing-dot"),
        el("span", "visually-hidden", wording.chat.typing),
      );
      this.#typing.append(dots);
      this.#log.append(this.#typing);
    }
    this.#chips.show(frame.suggestions);
    this.#composer.setDisabled(frame.state === "working");
    if (fresh.length > 0 || this.#typing !== null) {
      this.#log.scrollTop = this.#log.scrollHeight;
    }
  }

  /** Shows which chapter and section the page on screen belongs to. */
  setPlace(chapter: number | undefined, section: string | undefined): void {
    this.#place.textContent = wording.chat.place(chapter, section);
  }

  #add(entry: ChatEntry): void {
    if (this.#lastAt === null || entry.at - this.#lastAt >= GAP_MS) {
      this.#log.append(timeLine(entry.at, this.#edges.now()));
      this.#run = null;
    }
    this.#lastAt = entry.at;
    if (entry.kind === "notice") {
      const notice = el("p", "notice");
      notice.append(renderInline(entry.text));
      this.#log.append(notice);
      this.#run = null;
      return;
    }

    // A message joins the run of its speaker, and the tail moves to it.
    const speaker: Speaker = entry.kind;
    if (this.#run?.speaker !== speaker) {
      this.#run = { speaker, message: message(speaker) };
      this.#log.append(this.#run.message);
    }
    const run = this.#run.message;
    run.querySelector(".bubble.tail")?.classList.remove("tail");
    const bubble = el(
      "div",
      speaker === "assistant" ? "bubble in tail" : "bubble out tail",
    );
    if (speaker === "assistant") {
      bubble.append(...renderMarkdown(entry.text, true));
      this.#newestReply = { message: run, text: entry.text };
    } else {
      // What the student typed is shown as typed.
      bubble.append(el("p", "", entry.text));
    }
    // The icon row, when this run holds it, stays below the bubbles.
    if (this.#actions.parentElement === run) {
      this.#actions.before(bubble);
    } else {
      run.append(bubble);
    }
  }

  #press(pressed: HTMLButtonElement): void {
    const chip = this.#chips.textOf(pressed);
    if (chip !== null) {
      this.#edges.send({ type: "say", text: chip });
    } else if (pressed === this.#composer.mic) {
      this.#composer.toggleRecording();
    } else if (pressed === this.#copyButton) {
      this.#copy();
    } else if (pressed === this.#speakButton) {
      this.#toggleSpeaking();
    } else if (pressed.dataset["act"] === "regen") {
      this.#edges.send({ type: "regenerate" });
    }
  }

  /** Copies the newest reply, and turns the button green for a moment. */
  #copy(): void {
    if (this.#newestReply === null) {
      return;
    }
    void this.#edges.copy(this.#newestReply.text).then(
      () => {
        this.#copyButton.classList.add("done");
        setTimeout(() => {
          this.#copyButton.classList.remove("done");
        }, COPIED_MS);
      },
      (error: unknown) => {
        console.warn(`the reply could not be copied: ${String(error)}`);
      },
    );
  }

  /** Shows the read-aloud look. Speech itself comes later. */
  #toggleSpeaking(): void {
    const isOn = this.#speakButton.getAttribute("aria-pressed") !== "true";
    this.#speakButton.setAttribute("aria-pressed", String(isOn));
    if (isOn) {
      setTimeout(() => {
        this.#speakButton.setAttribute("aria-pressed", "false");
      }, SPEAKING_MS);
    }
  }
}
