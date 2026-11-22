// The scroll viewport of the page pane. Each page scrolls in its own
// scroller, so its scroll position is still there when the student pages
// back. A faded edge and a chevron show at the top or the bottom only while
// more of the page lies that way.
import "./scroll-cues.css";
import { button, el, icon } from "../dom";
import { wording } from "../wording";

const MORE = "more";
const NONE = "";
// How far one press of a cue scrolls, as a share of the visible height.
const STEP = 0.8;

type End = "top" | "bottom";

export class ScrollCues {
  readonly #viewport: HTMLElement;
  readonly #bottomCue: HTMLElement;
  #shown: HTMLElement | null = null;

  constructor(viewport: HTMLElement) {
    this.#viewport = viewport;
    viewport.dataset["top"] = NONE;
    viewport.dataset["bottom"] = NONE;
    this.#bottomCue = cue("bottom", wording.page.scrollDown, "down", "1");
    viewport.append(
      cue("top", wording.page.scrollUp, "up", "-1"),
      this.#bottomCue,
    );
    viewport.addEventListener("click", (event) => {
      const pressed =
        event.target instanceof Element
          ? event.target.closest(".cue button")
          : null;
      if (pressed instanceof HTMLElement && this.#shown !== null) {
        const direction = Number(pressed.dataset["dir"]);
        this.#shown.scrollBy({
          top: direction * this.#shown.clientHeight * STEP,
        });
      }
    });
  }

  /**
   * Adds the scroller of one page, hidden, with a marker at each end. The
   * page's content goes between the two markers.
   */
  addScroller(content: HTMLElement): HTMLElement {
    const scroller = el("div", "scroller");
    scroller.hidden = true;
    const top = edge("top");
    const bottom = edge("bottom");
    scroller.append(top, content, bottom);
    this.#bottomCue.before(scroller);

    // A marker out of view means more content lies past that end. The
    // observer also fires when the page grows, so a new block lights the
    // bottom cue without moving the page.
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.target instanceof HTMLElement) {
            const end: End = entry.target === top ? "top" : "bottom";
            scroller.dataset[end] = entry.isIntersecting ? NONE : MORE;
          }
        }
        if (scroller === this.#shown) {
          this.#copyEnds(scroller);
        }
      },
      { root: scroller },
    );
    observer.observe(top);
    observer.observe(bottom);
    return scroller;
  }

  /** Shows one scroller and hides the one shown before. */
  show(scroller: HTMLElement): void {
    if (this.#shown !== null) {
      this.#shown.hidden = true;
    }
    scroller.hidden = false;
    this.#shown = scroller;
    this.#copyEnds(scroller);
  }

  #copyEnds(scroller: HTMLElement): void {
    this.#viewport.dataset["top"] = scroller.dataset["top"] ?? NONE;
    this.#viewport.dataset["bottom"] = scroller.dataset["bottom"] ?? NONE;
  }
}

function cue(
  end: End,
  name: string,
  chevron: string,
  direction: string,
): HTMLElement {
  const press = button("");
  press.setAttribute("aria-label", name);
  press.dataset["dir"] = direction;
  press.append(icon(chevron));
  const node = el("div", `cue ${end}`);
  node.append(press);
  return node;
}

function edge(end: End): HTMLElement {
  const node = el("div", "edge");
  node.dataset["edge"] = end;
  return node;
}
