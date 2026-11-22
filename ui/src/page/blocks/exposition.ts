// Running text: paragraphs, headings, lists, tables, quotes and code.
import "./exposition.css";
import { renderMarkdown } from "../../markdown";
import type { ExpositionBlock } from "../../protocol";

export function renderExposition(
  block: ExpositionBlock,
  hasHeadingAbove: boolean,
): Element[] {
  return renderMarkdown(block.markdown, hasHeadingAbove);
}
