// The quiz boxes: multiple choice, and the three free-text boxes.
//
// A quiz is a form, and its submit sends what the student answered. The
// rules live on the server: the page that comes back says which options
// are open, disabled or right and which feedback shows, and this file
// draws that state.
import "./quiz.css";
import { button, el } from "../../dom";
import { renderInline } from "../../markdown";
import type {
  ChoiceQuizBlock,
  ResponseQuizBlock,
  SendFrame,
} from "../../protocol";
import { wording } from "../../wording";

// Options this short, four or more of them, sit in two columns.
const SHORT_OPTION = 12;
const FEWEST_FOR_COLUMNS = 4;

/** A quiz as drawn, with what the page does when the student acts on it. */
export interface Quiz {
  form: HTMLFormElement;
  /** The student pressed the main button, or Enter in a one-line field. */
  submit: () => void;
  /** The student pressed a button of the form that does not submit. */
  press: (pressed: Element) => void;
  /** The student typed into the form. */
  type: () => void;
}

function quizForm(title: string): HTMLFormElement {
  const form = el("form", "callout quiz");
  form.append(el("p", "callout-title", title));
  return form;
}

function tag(text: string): HTMLElement {
  return el("span", "tag", text);
}

function nothing(): void {
  // This quiz has no such control.
}

export function renderChoiceQuiz(
  block: ChoiceQuizBlock,
  send: SendFrame,
): Quiz {
  const form = quizForm(wording.quiz.popQuiz);
  const isSolved = block.status === "solved";
  const choices = el("fieldset", "choices");
  const question = el("legend", "q");
  question.append(renderInline(block.question));
  choices.append(question);
  if (block.multiple) {
    choices.append(el("p", "q-note", wording.quiz.selectAll));
  }

  const inputs: HTMLInputElement[] = [];
  const options = el("div", "options");
  let isShort = block.options.length >= FEWEST_FOR_COLUMNS;
  for (const [index, option] of block.options.entries()) {
    const input = el("input");
    input.type = block.multiple ? "checkbox" : "radio";
    input.name = block.id;
    input.checked = option.checked;
    input.disabled = isSolved || option.state !== "open";
    inputs.push(input);

    const text = el("span", "opt-text", `${wording.quiz.letter(index)} `);
    const content = renderInline(option.text);
    isShort &&= (content.textContent ?? "").length <= SHORT_OPTION;
    text.append(content);
    // The letter and the words that follow it read as one run of text.
    text.normalize();

    const label = el("label", "opt");
    label.append(input, text);
    if (option.state === "right") {
      label.classList.add("correct");
      label.append(tag(wording.quiz.correct));
    } else if (option.state === "disabled") {
      label.classList.add("wrong");
      label.append(tag(wording.quiz.notQuite));
    }
    options.append(label);
  }
  options.classList.toggle("grid", isShort);
  choices.append(options);
  form.append(choices);

  for (const line of block.feedback) {
    const feedback = el("p", isSolved ? "feedback good" : "feedback");
    feedback.append(renderInline(line));
    form.append(feedback);
  }
  if (block.missing) {
    form.append(el("p", "feedback", wording.quiz.moreToSelect));
  }
  if (!isSolved) {
    const check = el("button", "btn quiz", wording.quiz.check);
    check.type = "submit";
    form.append(check);
  }

  return {
    form,
    submit: () => {
      const picks = inputs.flatMap((input, index) =>
        input.checked ? [index] : [],
      );
      if (picks.length > 0) {
        send({ type: "answer", quiz: block.id, picks });
      }
    },
    press: nothing,
    type: nothing,
  };
}

const RESPONSE_TITLE = {
  short_answer: wording.quiz.shortAnswer,
  short_essay: wording.quiz.shortEssay,
  try_it: wording.quiz.tryIt,
} as const;

function countWords(text: string): number {
  return text.split(/\s+/).filter((word) => word !== "").length;
}

export function renderResponseQuiz(
  block: ResponseQuizBlock,
  send: SendFrame,
): Quiz {
  const form = quizForm(RESPONSE_TITLE[block.variant]);
  const fieldId = `${block.id}-field`;
  const question = el("label", "q");
  question.htmlFor = fieldId;
  question.append(renderInline(block.question));

  const field =
    block.variant === "short_essay"
      ? el("textarea", "field")
      : el("input", "field");
  field.id = fieldId;
  if (field instanceof HTMLInputElement) {
    field.type = "text";
    field.autocomplete = "off";
  }
  if (block.variant === "try_it") {
    field.classList.add("mono");
    field.setAttribute("spellcheck", "false");
  } else {
    field.placeholder =
      block.variant === "short_essay"
        ? wording.quiz.essayPlaceholder
        : wording.quiz.answerPlaceholder;
  }
  field.value = block.answer ?? block.code ?? "";

  const submit = el(
    "button",
    "btn quiz",
    block.variant === "try_it" ? wording.quiz.run : wording.quiz.submit,
  );
  submit.type = "submit";
  const words = block.words;
  const wordCount = el("span", "word-count");
  wordCount.id = `${block.id}-words`;
  const showWordCount = (): void => {
    if (words !== undefined) {
      wordCount.textContent = wording.quiz.wordCount(
        countWords(field.value),
        words,
      );
    }
  };
  showWordCount();

  // The foot holds the main button while the box takes an answer. Try It
  // keeps its button on the line of its field.
  const foot = el("div", "q-foot");
  const row = el("div", "row");
  if (block.variant === "try_it") {
    row.append(field, submit);
    form.append(question, row);
  } else {
    if (words !== undefined) {
      foot.append(wordCount);
    }
    foot.append(submit);
    form.append(question, field, foot);
  }

  // The status area takes the place of the button once an answer is sent.
  const status = el("div", "q-status");
  const tryAgain = button("btn quiz", wording.quiz.tryAgain);
  if (block.status === "submitted") {
    status.append(el("p", "q-note", wording.quiz.sent));
  } else if (block.status !== "open") {
    const isRight = block.status === "right";
    const feedback = el("p", isRight ? "feedback good" : "feedback");
    feedback.append(
      tag(isRight ? wording.quiz.correct : wording.quiz.notQuite),
      renderInline(block.feedback ?? ""),
    );
    status.append(feedback);
    if (!isRight) {
      status.append(tryAgain);
    }
  }
  const close = (): void => {
    field.readOnly = true;
    submit.remove();
    foot.remove();
    form.append(status);
  };
  const reopen = (): void => {
    field.readOnly = false;
    status.remove();
    if (block.variant === "try_it") {
      row.append(submit);
    } else {
      form.append(foot);
      foot.append(submit);
    }
    field.focus();
  };
  if (block.status !== "open") {
    close();
  }

  return {
    form,
    submit: () => {
      const text = field.value.trim();
      if (text !== "" && !field.readOnly) {
        send({ type: "answer", quiz: block.id, text });
      }
    },
    press: (pressed) => {
      if (pressed === tryAgain) {
        reopen();
      }
    },
    type: showWordCount,
  };
}
