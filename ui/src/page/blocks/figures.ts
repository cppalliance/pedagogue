// Figures drawn from data: the bar chart, the tree diagram and the
// timeline. The geometry is the design's, written as arithmetic so it fits
// any count of bars or rows.
import "./figures.css";
import { el, svg } from "../../dom";
import { renderInline } from "../../markdown";
import type {
  BarChartBlock,
  TimelineBlock,
  TreeDiagramBlock,
} from "../../protocol";
import { wording } from "../../wording";

// Bar chart, in the units of its view box.
const CHART = {
  width: 460,
  height: 215,
  left: 30,
  right: 430,
  base: 170,
  tallest: 140,
};
// The share of its slot that a bar fills.
const BAR_SHARE = 0.7;

// Tree diagram, in the units of its view box.
const TREE = { width: 520, firstRow: 34, rowGap: 64, foot: 53 };

/** Rounds to one decimal place, so the markup stays short. */
function tidy(value: number): number {
  return Math.round(value * 10) / 10;
}

function figure(
  drawing: SVGElement,
  number: string,
  caption: string,
): Element[] {
  const node = el("figure");
  const text = el("span", "fig-text");
  text.append(renderInline(caption));
  const captionNode = el("figcaption");
  captionNode.append(el("span", "fig-no", wording.page.figure(number)), text);
  node.append(drawing, captionNode);
  return [node];
}

export function renderBarChart(block: BarChartBlock): Element[] {
  const drawing = svg("svg", {
    viewBox: `0 0 ${CHART.width} ${CHART.height}`,
    role: "img",
    "aria-label": wording.page.barChart(
      block.caption,
      block.bars.map((bar) => `${bar.label} ${bar.value}`),
    ),
  });
  drawing.append(
    svg("line", {
      class: "chart-axis",
      x1: CHART.left,
      y1: CHART.base,
      x2: CHART.right,
      y2: CHART.base,
    }),
  );
  const slot = (CHART.right - CHART.left) / Math.max(block.bars.length, 1);
  const top = Math.max(...block.bars.map((bar) => bar.value), 1);
  for (const [index, bar] of block.bars.entries()) {
    const width = tidy(slot * BAR_SHARE);
    const x = tidy(CHART.left + slot * index + (slot - width) / 2);
    const height = tidy((CHART.tallest * bar.value) / top);
    const y = tidy(CHART.base - height);
    const middle = tidy(x + width / 2);
    drawing.append(
      svg("rect", { class: `bar ${bar.tone}`, x, y, width, height }),
      svg(
        "text",
        { class: "chart-label", x: middle, y: y - 8 },
        String(bar.value),
      ),
      svg(
        "text",
        { class: "chart-label", x: middle, y: CHART.base + 22 },
        bar.label,
      ),
    );
  }
  return figure(drawing, block.figure, block.caption);
}

export function renderTreeDiagram(block: TreeDiagramBlock): Element[] {
  // Each row spreads its labels evenly across the width.
  const places = new Map<string, { x: number; y: number }>();
  for (const [row, labels] of block.rows.entries()) {
    for (const [index, label] of labels.entries()) {
      places.set(label, {
        x: tidy((TREE.width * (index + 0.5)) / labels.length),
        y: TREE.firstRow + row * TREE.rowGap,
      });
    }
  }
  const lastRow = block.rows.length - 1;
  const height = TREE.firstRow + Math.max(lastRow, 0) * TREE.rowGap + TREE.foot;
  const children = new Map<string, string[]>();
  for (const edge of block.edges) {
    children.set(edge.from, [...(children.get(edge.from) ?? []), edge.to]);
  }

  const drawing = svg("svg", {
    viewBox: `0 0 ${TREE.width} ${height}`,
    role: "img",
    "aria-label": wording.page.treeDiagram(
      block.caption,
      [...children].map(([parent, kids]) => ({ parent, kids })),
    ),
  });
  for (const edge of block.edges) {
    const from = places.get(edge.from);
    const to = places.get(edge.to);
    if (from !== undefined && to !== undefined) {
      drawing.append(
        svg("line", {
          class: "tree-edge",
          x1: from.x,
          y1: from.y + 6,
          x2: to.x,
          y2: to.y - 18,
        }),
      );
    }
  }
  // A lone label in the first row is the root, marked with a dot above it.
  const root =
    block.rows[0]?.length === 1
      ? places.get(block.rows[0][0] ?? "")
      : undefined;
  if (root !== undefined) {
    drawing.append(
      svg("circle", { class: "tree-root", cx: root.x, cy: root.y - 22, r: 6 }),
      svg("line", {
        class: "tree-edge",
        x1: root.x,
        y1: root.y - 16,
        x2: root.x,
        y2: root.y - 12,
      }),
    );
  }
  for (const [row, labels] of block.rows.entries()) {
    for (const label of labels) {
      const place = places.get(label);
      if (place !== undefined) {
        const kind =
          row === lastRow && row > 0 ? "tree-label leaf" : "tree-label";
        drawing.append(
          svg("text", { class: kind, x: place.x, y: place.y }, label),
        );
      }
    }
  }
  return figure(drawing, block.figure, block.caption);
}

export function renderTimeline(block: TimelineBlock): Element[] {
  const list = el("ol", "timeline");
  for (const stop of block.stops) {
    const item = el("li", "stop");
    const label = el("span", "stop-label");
    label.append(renderInline(stop.label));
    // The note is drawn even when empty, since it holds the stop's dot.
    const note = el("span", "stop-note");
    note.append(renderInline(stop.note));
    item.append(label, note);
    list.append(item);
  }
  return [list];
}
