// The entry point: it builds the two panes, then opens the socket. This is
// the one file that knows both sides.
import "./tokens.css";
import "./layout.css";
import { ChatView } from "./chat/chat-view";
import { PageView } from "./page/page-view";
import type { SendFrame } from "./protocol";
import { Socket, socketUrl } from "./socket";

function start(): void {
  const pagePane = document.querySelector<HTMLElement>("main.textbook");
  const chatPane = document.querySelector<HTMLElement>("aside.tutor");
  if (pagePane === null || chatPane === null) {
    throw new Error("index.html is missing the page pane or the chat pane");
  }
  const send: SendFrame = (frame) => {
    socket.send(frame);
  };
  const chat = new ChatView(chatPane, {
    send,
    now: () => new Date(),
    copy: (text) => navigator.clipboard.writeText(text),
  });
  const page = new PageView(pagePane, send, (chapter, section) => {
    chat.setPlace(chapter, section);
  });
  const socket = new Socket(socketUrl(), (frame) => {
    if (frame.type === "page") {
      page.show(frame);
    } else {
      chat.show(frame);
    }
  });
  socket.connect();
}

start();
