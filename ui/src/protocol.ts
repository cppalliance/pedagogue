// The frames on the socket, both ways. The Rust side defines them in
// crates/teachpad-session (frames.rs and the page view beside the lesson
// model), and shared fixtures pin both sides: a field renamed there fails a
// test here.
import * as z from "zod";

// The content policy refuses eval, which Zod's compiled fast path needs.
z.config({ jitless: true });

const blockId = z
  .string()
  .regex(/^b[1-9]\d*$/)
  .brand<"BlockId">();
const pageNumber = z.int().positive().brand<"PageNumber">();
const count = z.int().nonnegative();

const exposition = z.strictObject({
  id: blockId,
  kind: z.literal("exposition"),
  markdown: z.string(),
});

const callout = z.strictObject({
  id: blockId,
  kind: z.literal("callout"),
  variant: z.enum([
    "tip",
    "key_idea",
    "remember",
    "tailored_for_you",
    "hint",
    "milestone",
    "section_complete",
  ]),
  title: z.string().optional(),
  number: z.int().positive().optional(),
  markdown: z.string(),
});

const choiceQuiz = z.strictObject({
  id: blockId,
  kind: z.literal("choice_quiz"),
  question: z.string(),
  multiple: z.boolean(),
  status: z.enum(["open", "solved"]),
  attempts: count,
  missing: z.boolean(),
  options: z.array(
    z.strictObject({
      text: z.string(),
      state: z.enum(["open", "disabled", "right"]),
      checked: z.boolean(),
    }),
  ),
  feedback: z.array(z.string()),
});

const responseQuiz = z.strictObject({
  id: blockId,
  kind: z.literal("response_quiz"),
  variant: z.enum(["short_answer", "short_essay", "try_it"]),
  question: z.string(),
  words: z.int().positive().optional(),
  code: z.string().optional(),
  language: z.string().optional(),
  status: z.enum(["open", "submitted", "right", "wrong"]),
  attempts: count,
  answer: z.string().optional(),
  feedback: z.string().optional(),
});

const barChart = z.strictObject({
  id: blockId,
  kind: z.literal("bar_chart"),
  figure: z.string(),
  caption: z.string(),
  bars: z.array(
    z.strictObject({
      label: z.string(),
      value: z.number().nonnegative(),
      tone: z.enum(["accent", "neutral", "negative"]),
    }),
  ),
});

const treeDiagram = z.strictObject({
  id: blockId,
  kind: z.literal("tree_diagram"),
  figure: z.string(),
  caption: z.string(),
  rows: z.array(z.array(z.string())),
  edges: z.array(z.strictObject({ from: z.string(), to: z.string() })),
});

const timeline = z.strictObject({
  id: blockId,
  kind: z.literal("timeline"),
  stops: z.array(z.strictObject({ label: z.string(), note: z.string() })),
});

const progress = z.strictObject({
  id: blockId,
  kind: z.literal("progress"),
  done: count,
  total: z.int().positive(),
  label: z.string(),
});

const success = z.strictObject({
  id: blockId,
  kind: z.literal("success"),
  text: z.string(),
});

const block = z.discriminatedUnion("kind", [
  exposition,
  callout,
  choiceQuiz,
  responseQuiz,
  barChart,
  treeDiagram,
  timeline,
  progress,
  success,
]);

const page = z.strictObject({
  number: pageNumber,
  chapter_number: z.int().nonnegative().optional(),
  chapter_title: z.string().optional(),
  section_number: z.string().optional(),
  section_title: z.string().optional(),
  chapter_opener: z.boolean(),
  blocks: z.array(block),
  footnotes: z.array(z.string()),
  continue_label: z.string().optional(),
  continue_pressed: z.boolean(),
});

const pageFrame = z.strictObject({
  type: z.literal("page"),
  page_count: count,
  page,
});

const chatEntry = z.strictObject({
  kind: z.enum(["assistant", "user", "notice"]),
  text: z.string(),
  at: count,
});

const chatFrame = z.strictObject({
  type: z.literal("chat"),
  state: z.enum(["working", "awaiting_input"]),
  suggestions: z.array(z.string()),
  entries: z.array(chatEntry),
});

const serverFrame = z.discriminatedUnion("type", [pageFrame, chatFrame]);

export type BlockId = z.infer<typeof blockId>;
export type PageNumber = z.infer<typeof pageNumber>;
export type Block = z.infer<typeof block>;
export type ExpositionBlock = z.infer<typeof exposition>;
export type CalloutBlock = z.infer<typeof callout>;
export type ChoiceQuizBlock = z.infer<typeof choiceQuiz>;
export type ResponseQuizBlock = z.infer<typeof responseQuiz>;
export type BarChartBlock = z.infer<typeof barChart>;
export type TreeDiagramBlock = z.infer<typeof treeDiagram>;
export type TimelineBlock = z.infer<typeof timeline>;
export type ProgressBlock = z.infer<typeof progress>;
export type SuccessBlock = z.infer<typeof success>;
export type Page = z.infer<typeof page>;
export type PageFrame = z.infer<typeof pageFrame>;
export type ChatEntry = z.infer<typeof chatEntry>;
export type ChatFrame = z.infer<typeof chatFrame>;
export type ServerFrame = z.infer<typeof serverFrame>;

/** Something the student did, as the frame the server takes. */
export type StudentFrame =
  | { type: "say"; text: string }
  | { type: "answer"; quiz: BlockId; picks: number[] }
  | { type: "answer"; quiz: BlockId; text: string }
  | { type: "continue"; page: PageNumber }
  | { type: "regenerate" };

/** Sends one student frame to the server. */
export type SendFrame = (frame: StudentFrame) => void;

/** A frame that passed its schema, or the reason it did not. */
export type ParsedFrame =
  { ok: true; frame: ServerFrame } | { ok: false; problem: string };

/**
 * Reads one message from the server. A message that is not JSON, or that
 * fails its schema, comes back as a problem that names the failing field.
 */
export function parseFrame(data: unknown): ParsedFrame {
  if (typeof data !== "string") {
    return { ok: false, problem: "the message is not text" };
  }
  let value: unknown;
  try {
    value = JSON.parse(data);
  } catch {
    return { ok: false, problem: "the message is not JSON" };
  }
  const parsed = serverFrame.safeParse(value);
  if (parsed.success) {
    return { ok: true, frame: parsed.data };
  }
  const issue = parsed.error.issues[0];
  if (issue === undefined) {
    return { ok: false, problem: "the frame fails its schema" };
  }
  const field = issue.path.length === 0 ? "frame" : issue.path.join(".");
  return { ok: false, problem: `${field}: ${issue.message}` };
}
