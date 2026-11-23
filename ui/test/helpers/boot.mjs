// Boots the built bundle in jsdom, the same three files the app ships.
// The fakes sit at the edges: a scripted socket in place of the server, a
// clipboard that records, and an observer that the test fires by hand.
// Each boot is a fresh page. Run after `node build.mjs`.
import { readFile } from "node:fs/promises";
import path from "node:path";
import { pathToFileURL } from "node:url";
import { JSDOM } from "jsdom";

const uiDir = path.join(import.meta.dirname, "..", "..");
const distDir = path.join(uiDir, "dist");
const repoDir = path.join(uiDir, "..");

// Constructors the bundle names as globals, for `instanceof` checks.
const DOM_GLOBALS = [
  "Node",
  "Element",
  "HTMLElement",
  "HTMLButtonElement",
  "HTMLFormElement",
  "HTMLInputElement",
  "HTMLTextAreaElement",
  "SVGElement",
  "DocumentFragment",
  "NodeFilter",
  "DOMParser",
  "MutationObserver",
];

let boots = 0;

/** Sets a global that the bundle reads, even one Node defines read-only. */
function setGlobal(name, value) {
  Object.defineProperty(globalThis, name, {
    value,
    configurable: true,
    writable: true,
  });
}

/** Reads a JSON file by its path from the repository root. */
export async function readJson(...parts) {
  return JSON.parse(await readFile(path.join(repoDir, ...parts), "utf8"));
}

/** The frames the app sends for the contact-sheet sample. */
export function sampleFrames() {
  return readJson("samples", "contact-sheet.frames.json");
}

/** One named example of each frame the UI sends. */
export function studentFrames() {
  return readJson(
    "crates",
    "teachpad-session",
    "fixtures",
    "student-frames.json",
  );
}

/**
 * Loads dist/index.html into jsdom and runs the bundle in it. Returns the
 * page with its fakes:
 *
 * - `deliver(frame)` sends one frame from the server, as JSON.
 * - `sent()` lists the frames the app sent, parsed.
 * - `socket` is the newest socket, and `socket.drop()` closes it.
 * - `warnings` lists what the app wrote with console.warn.
 * - `clipboard.texts` lists what the app copied.
 * - `showEdge(page, edge, visible)` fires the scroll observer for one end.
 * - `close()` puts console.warn back.
 */
export async function boot() {
  const html = await readFile(path.join(distDir, "index.html"), "utf8");
  const { window } = new JSDOM(html, {
    url: "http://127.0.0.1:7910/",
    pretendToBeVisual: true,
  });

  const sockets = [];
  class FakeWebSocket {
    static CONNECTING = 0;
    static OPEN = 1;
    static CLOSING = 2;
    static CLOSED = 3;

    constructor(url) {
      this.url = url;
      this.readyState = FakeWebSocket.OPEN;
      this.sent = [];
      this.listeners = { message: [], close: [], open: [], error: [] };
      sockets.push(this);
    }

    addEventListener(type, listener) {
      this.listeners[type].push(listener);
    }

    send(data) {
      this.sent.push(data);
    }

    close() {
      this.drop();
    }

    /** Sends text from the server as it is. */
    deliverText(data) {
      for (const listener of this.listeners.message) {
        listener({ data });
      }
    }

    /** Closes the socket from the server's side. */
    drop() {
      this.readyState = FakeWebSocket.CLOSED;
      for (const listener of this.listeners.close) {
        listener({});
      }
    }
  }

  const observers = [];
  class FakeIntersectionObserver {
    constructor(callback, options) {
      this.callback = callback;
      this.options = options;
      this.targets = [];
      observers.push(this);
    }

    observe(target) {
      this.targets.push(target);
    }

    unobserve(target) {
      this.targets = this.targets.filter((held) => held !== target);
    }

    disconnect() {
      this.targets = [];
    }

    takeRecords() {
      return [];
    }
  }

  const clipboard = {
    texts: [],
    writeText(text) {
      this.texts.push(text);
      return Promise.resolve();
    },
  };
  Object.defineProperty(window.navigator, "clipboard", {
    value: clipboard,
    configurable: true,
  });

  // jsdom has no layout, so scrolling only records where it was asked to go.
  const scrolls = [];
  window.Element.prototype.scrollBy = function scrollBy(options) {
    scrolls.push({ element: this, by: options });
  };
  window.Element.prototype.scrollTo = function scrollTo(options) {
    scrolls.push({ element: this, to: options });
  };
  window.Element.prototype.scrollIntoView = function scrollIntoView() {
    scrolls.push({ element: this, intoView: true });
  };

  const warnings = [];
  const realWarn = console.warn;
  console.warn = (...parts) => {
    warnings.push(parts.join(" "));
  };

  setGlobal("window", window);
  setGlobal("document", window.document);
  setGlobal("navigator", window.navigator);
  setGlobal("location", window.location);
  setGlobal("getComputedStyle", window.getComputedStyle.bind(window));
  setGlobal("WebSocket", FakeWebSocket);
  setGlobal("IntersectionObserver", FakeIntersectionObserver);
  for (const name of DOM_GLOBALS) {
    setGlobal(name, window[name]);
  }

  boots += 1;
  const bundle = pathToFileURL(path.join(distDir, "app.js")).href;
  await import(`${bundle}?boot=${boots}`);

  return {
    window,
    document: window.document,
    sockets,
    warnings,
    clipboard,
    scrolls,
    get socket() {
      return sockets.at(-1);
    },
    deliver(frame) {
      sockets.at(-1).deliverText(JSON.stringify(frame));
    },
    sent() {
      return sockets.flatMap((socket) =>
        socket.sent.map((data) => JSON.parse(data)),
      );
    },
    showEdge(scroller, edge, isVisible) {
      const target = scroller.querySelector(`.edge[data-edge="${edge}"]`);
      for (const observer of observers) {
        if (observer.targets.includes(target)) {
          observer.callback([{ target, isIntersecting: isVisible }], observer);
        }
      }
    },
    close() {
      console.warn = realWarn;
    },
  };
}
