// The boxes: blue for information, yellow for a personal note, green for
// progress, and the green strip after a right answer.
import "./callout.css";
import "./progress.css";
import { el, icon } from "../../dom";
import { renderInline, renderMarkdown } from "../../markdown";
import type { CalloutBlock, ProgressBlock, SuccessBlock } from "../../protocol";
import { wording } from "../../wording";

type Variant = CalloutBlock["variant"];

// The color, the icon and the title of each kind of box.
const KINDS: Record<Variant, { tone: string; icon: string; title: string }> = {
  tip: { tone: "info", icon: "info", title: wording.callout.tip },
  key_idea: { tone: "info", icon: "info", title: wording.callout.keyIdea },
  remember: { tone: "info", icon: "info", title: wording.callout.remember },
  tailored_for_you: {
    tone: "note",
    icon: "star",
    title: wording.callout.tailoredForYou,
  },
  hint: { tone: "note", icon: "star", title: wording.callout.hint },
  milestone: { tone: "note", icon: "clock", title: "" },
  section_complete: {
    tone: "ok",
    icon: "check",
    title: wording.callout.sectionComplete,
  },
};

function box(tone: string, iconName: string, title: string): HTMLElement {
  const node = el("aside", `callout ${tone}`);
  const heading = el("p", "callout-title");
  heading.append(icon(iconName), renderInline(title));
  node.append(heading);
  return node;
}

function titleOf(block: CalloutBlock): string {
  if (block.title !== undefined) {
    return block.title;
  }
  if (block.variant === "milestone" && block.number !== undefined) {
    return wording.callout.milestone(block.number);
  }
  return KINDS[block.variant].title;
}

export function renderCallout(block: CalloutBlock): Element[] {
  const kind = KINDS[block.variant];
  const node = box(kind.tone, kind.icon, titleOf(block));
  node.append(...renderMarkdown(block.markdown, true));
  return [node];
}

export function renderProgress(block: ProgressBlock): Element[] {
  const node = box("ok", "book", wording.callout.progress);
  const bar = el(
    "progress",
    "progress-bar",
    wording.callout.progressCount(block.done, block.total),
  );
  bar.max = block.total;
  bar.value = block.done;
  bar.setAttribute("aria-label", wording.callout.progressBar);
  const label = el("p");
  label.append(renderInline(block.label));
  node.append(bar, label);
  return [node];
}

export function renderSuccess(block: SuccessBlock): Element[] {
  const node = el("p", "callout success");
  node.append(renderInline(block.text));
  return [node];
}
