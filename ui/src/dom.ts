// Small helpers for building DOM nodes. Text always goes in as text.

const SVG_NAMESPACE = "http://www.w3.org/2000/svg";

/** Makes an element with a class and plain text. */
export function el<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  className = "",
  text?: string,
): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag);
  if (className !== "") {
    node.className = className;
  }
  if (text !== undefined) {
    node.textContent = text;
  }
  return node;
}

/** Makes a button that never submits a form. */
export function button(className: string, text?: string): HTMLButtonElement {
  const node = el("button", className, text);
  node.type = "button";
  return node;
}

/** Makes an SVG element with attributes and plain text. */
export function svg(
  tag: string,
  attributes: Record<string, string | number> = {},
  text?: string,
): SVGElement {
  const node = document.createElementNS(SVG_NAMESPACE, tag);
  for (const [name, value] of Object.entries(attributes)) {
    node.setAttribute(name, String(value));
  }
  if (text !== undefined) {
    node.textContent = text;
  }
  return node;
}

/** Makes a decorative icon from a symbol of the page's sprite. */
export function icon(name: string): SVGElement {
  const node = svg("svg", { "aria-hidden": "true" });
  node.append(svg("use", { href: `#i-${name}` }));
  return node;
}
