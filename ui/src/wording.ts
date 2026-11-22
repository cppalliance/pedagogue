// Every fixed line the student reads or hears: box titles, button labels,
// the names of icon-only controls and the text for screen readers. The
// words come from the two design files. Text the tutor writes arrives in
// frames, and text a model reads lives in the Rust crates.

/** Joins names as "a", "a and b" or "a, b and c". */
function listOf(names: readonly string[]): string {
  const last = names.at(-1) ?? "";
  return names.length < 2
    ? last
    : `${names.slice(0, -1).join(", ")} and ${last}`;
}

export const wording = {
  page: {
    /** The title of a page that names no chapter. */
    untitled: "Lesson",
    chapter: (number: number): string => `Chapter ${number}`,
    figure: (number: string): string => `Figure ${number}`,
    /** What a screen reader says for a bar chart: its caption and bars. */
    barChart: (caption: string, bars: readonly string[]): string =>
      `Bar chart, ${caption}: ${bars.join(", ")}`,
    /** What a screen reader says for a tree: its caption and each split. */
    treeDiagram: (
      caption: string,
      splits: readonly { parent: string; kids: readonly string[] }[],
    ): string => {
      const told = splits.map(
        ({ parent, kids }) => `${parent} splits into ${listOf(kids)}`,
      );
      return `Tree diagram, ${caption}: ${told.join("; ")}`;
    },
    previous: "Previous section",
    next: "Next section",
    pages: "Pages",
    scrollUp: "Scroll up",
    scrollDown: "Scroll down",
    opensNewWindow: " (opens in a new window)",
  },
  callout: {
    tip: "Tip",
    keyIdea: "Key Idea",
    remember: "Remember",
    tailoredForYou: "Tailored for you",
    hint: "Hint for you",
    milestone: (number: number): string => `Milestone ${number}`,
    sectionComplete: "Section complete",
    progress: "Your Progress",
    progressBar: "Your progress",
    progressCount: (done: number, total: number): string =>
      `${done} of ${total}`,
  },
  quiz: {
    popQuiz: "Pop Quiz",
    shortAnswer: "Check Your Understanding",
    shortEssay: "Short Essay",
    tryIt: "Try It",
    check: "Check answer",
    submit: "Submit",
    run: "Run",
    tryAgain: "Try again",
    selectAll: "Select all that apply.",
    moreToSelect: "There is more to select.",
    correct: "\u2713 Correct",
    notQuite: "Not quite",
    sent: "Sent to your tutor",
    answerPlaceholder: "One clear sentence is fine",
    essayPlaceholder: "Write your answer here",
    wordCount: (words: number, limit: number): string =>
      `${words} / ${limit} words`,
    letter: (index: number): string => `${String.fromCodePoint(65 + index)}.`,
  },
  chat: {
    panel: "Tutor chat",
    tutor: "Tutor",
    you: "You",
    avatar: "R",
    conversation: "Conversation",
    typing: "is typing",
    actions: "Message actions",
    copy: "Copy",
    readAloud: "Read aloud",
    regenerate: "Regenerate",
    suggestions: "Suggestions",
    composer: "Message the tutor",
    record: "Record",
    stopRecording: "Stop recording",
    send: "Send",
    today: "Today",
    place: (
      chapter: number | undefined,
      section: string | undefined,
    ): string => {
      const parts = [];
      if (chapter !== undefined) {
        parts.push(`Chapter ${chapter}`);
      }
      if (section !== undefined) {
        parts.push(`Section ${section}`);
      }
      return parts.join(" - ");
    },
  },
} as const;
