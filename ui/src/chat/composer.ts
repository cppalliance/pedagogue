// The composer at the foot of the chat: an edit box, a record button and a
// send button. It has four looks: empty, with text, recording, and
// disabled while the tutor replies.
import "./composer.css";
import { button, el, icon } from "../dom";
import { wording } from "../wording";

const SECOND_MS = 1000;

function clockText(seconds: number): string {
  return `${Math.floor(seconds / 60)}:${String(seconds % 60).padStart(2, "0")}`;
}

export class Composer {
  readonly form: HTMLFormElement;
  readonly mic: HTMLButtonElement;
  readonly #field: HTMLInputElement;
  readonly #send: HTMLButtonElement;
  readonly #recordTime: HTMLElement;
  readonly #onSend: (text: string) => void;
  #isDisabled = false;
  #recording: ReturnType<typeof setInterval> | null = null;

  constructor(onSend: (text: string) => void) {
    this.#onSend = onSend;
    this.form = el("form", "composer");
    this.form.setAttribute("autocomplete", "off");

    const label = el("label", "visually-hidden", wording.chat.composer);
    this.#field = el("input", "field-area");
    this.#field.id = "draft";
    this.#field.type = "text";
    this.#field.placeholder = wording.chat.composer;
    label.htmlFor = this.#field.id;

    // The recording look: a red dot, the time and a row of level bars.
    const recording = el("div", "rec-area");
    this.#recordTime = el("span", "rec-time", clockText(0));
    const bars = el("span", "rec-bars");
    bars.setAttribute("aria-hidden", "true");
    recording.append(el("span", "rec-dot"), this.#recordTime, bars);

    this.mic = button("mic");
    this.mic.append(icon("mic"));
    this.#send = el("button", "send");
    this.#send.type = "submit";
    this.#send.setAttribute("aria-label", wording.chat.send);
    this.#send.title = wording.chat.send;
    this.#send.append(icon("send"));
    this.form.append(label, this.#field, recording, this.mic, this.#send);
    this.#draw();
  }

  /** The student typed into the edit box. */
  type(): void {
    this.#draw();
  }

  /** The student pressed Send, or Enter in the edit box. */
  submit(): void {
    const text = this.#field.value.trim();
    if (text === "" || this.#isDisabled) {
      return;
    }
    this.#field.value = "";
    this.#draw();
    this.#onSend(text);
  }

  /**
   * The student pressed the record button. Speech comes later, so this
   * shows the recording look and its running time, and a second press
   * goes back to the edit box.
   */
  toggleRecording(): void {
    if (this.#recording === null) {
      let seconds = 0;
      this.#recordTime.textContent = clockText(seconds);
      this.#recording = setInterval(() => {
        seconds += 1;
        this.#recordTime.textContent = clockText(seconds);
      }, SECOND_MS);
    } else {
      this.#stopRecording();
      this.#field.focus();
    }
    this.#draw();
  }

  /** Locks the composer while the tutor replies. */
  setDisabled(isDisabled: boolean): void {
    this.#isDisabled = isDisabled;
    if (isDisabled) {
      this.#stopRecording();
    }
    this.#draw();
  }

  #stopRecording(): void {
    if (this.#recording !== null) {
      clearInterval(this.#recording);
      this.#recording = null;
      this.#recordTime.textContent = clockText(0);
    }
  }

  #draw(): void {
    const isRecording = this.#recording !== null;
    this.form.classList.toggle("recording", isRecording);
    this.form.classList.toggle("disabled", this.#isDisabled);
    this.#field.disabled = this.#isDisabled;
    this.mic.disabled = this.#isDisabled;
    this.#send.disabled =
      this.#isDisabled || isRecording || this.#field.value.trim() === "";

    const name = isRecording ? wording.chat.stopRecording : wording.chat.record;
    this.mic.setAttribute("aria-label", name);
    this.mic.title = name;
    this.mic.replaceChildren(icon(isRecording ? "stop" : "mic"));
  }
}
