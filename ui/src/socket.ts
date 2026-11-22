// The one socket to the server. Page and chat frames come down, and what
// the student does goes up. It reconnects on its own, and the server sends
// every page and the chat again on each connect.
import { parseFrame, type ServerFrame, type StudentFrame } from "./protocol";

const FIRST_RETRY_MS = 250;
const LONGEST_RETRY_MS = 5000;

/** Returns the address of the socket on the server this page came from. */
export function socketUrl(): string {
  const scheme = location.protocol === "https:" ? "wss" : "ws";
  return `${scheme}://${location.host}/ws`;
}

export class Socket {
  readonly #url: string;
  readonly #onFrame: (frame: ServerFrame) => void;
  #socket: WebSocket | null = null;
  #retryMs = FIRST_RETRY_MS;

  constructor(url: string, onFrame: (frame: ServerFrame) => void) {
    this.#url = url;
    this.#onFrame = onFrame;
  }

  /** Opens the socket. A socket that closes is opened again after a wait. */
  connect(): void {
    const socket = new WebSocket(this.#url);
    this.#socket = socket;
    socket.addEventListener("message", (event) => {
      this.#receive(event.data);
    });
    socket.addEventListener("close", () => {
      if (this.#socket === socket) {
        this.#socket = null;
      }
      this.#retry();
    });
  }

  /**
   * Sends what the student did. Returns false when the socket is not open,
   * and then the frame is dropped: the server holds the state, so the page
   * shows what the server has once the socket is back.
   */
  send(frame: StudentFrame): boolean {
    if (this.#socket?.readyState !== WebSocket.OPEN) {
      console.warn(
        `the socket is closed, so a "${frame.type}" frame was dropped`,
      );
      return false;
    }
    this.#socket.send(JSON.stringify(frame));
    return true;
  }

  #receive(data: unknown): void {
    const parsed = parseFrame(data);
    if (!parsed.ok) {
      console.warn(`a frame from the server was dropped: ${parsed.problem}`);
      return;
    }
    // A frame means the server is well, so the next retry is quick again.
    this.#retryMs = FIRST_RETRY_MS;
    this.#onFrame(parsed.frame);
  }

  #retry(): void {
    const wait = this.#retryMs;
    this.#retryMs = Math.min(wait * 2, LONGEST_RETRY_MS);
    setTimeout(() => {
      this.connect();
    }, wait);
  }
}
