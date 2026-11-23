# Adaptive learning products and AI tutors: who is popular, and who is closest to TeachPad

## Executive summary

No product that we found ships the whole design of TeachPad, the adaptive tutor our team is building, and the most popular places to learn with a model are general chatbots that cost the learner nothing. In TeachPad's design a language model teaches one student inside a learning companion: the tutor writes a textbook page during the lesson, a chat runs beside the page, and the student moves on from a skill after showing it, under mastery gates held in code.

**Every part of the design exists in some product, and no product joins expert interviews, a model tutor and gates held in code.** Closeness to the design is scored out of 12 on six parts: a tutor that holds back answers, a page written during the lesson, mastery gates, adaptation in both directions, any subject, and lessons for adults built from expert knowledge. Tutors that write a page or a whiteboard during the lesson are new: the first shipped in late 2025, and most appeared in 2026.

1. **The closest popular product is DeepTutor, an open-source project from the University of Hong Kong.** It has 61,585 GitHub stars less than eleven months after its code first appeared ([GitHub][s6-deeptutor-repo]). It generates a "living book" of text, quizzes, timelines and figures with a chat on each page. It also runs a separate tutor chat in which a mastery gate decides when the learner advances. Code alone decides the gate for facts and procedures, and the tutor model judges concepts. It scores 10 of 12. The book and the gated tutor are two separate activities, and its lessons come from documents the learner uploads.

2. **The closest product from a large company is Google's Gemini study notebooks, launched on 25 June 2026.** It gives a diagnostic quiz, tracks more than 100 learning objectives for each study goal, and writes short lessons with practice quizzes around the gaps ([Google][s2-g-studynotebooks]). It is free inside the Gemini app, which has a billion monthly users ([Google][s2-g-studynotebooksli]; [Google][s7-goog-1b]). The dashboard recommends the next lesson, and the student can still "filter and browse" the rest, so nothing blocks progress. It scores 9 of 12, and Google has not said whether its lesson chat holds back answers.

3. **The closest product for working adults that reports real use is the DataCamp AI Tutor.** It generates each lesson live from parts that DataCamp's experts prepared, gives hints as questions, and has served more than 300,000 learners ([Superintel][s5-dc3]; [Chain of Thought podcast][s5-dc4]). The model decides when the learner moves on, and the catalog covers data and AI skills. It also scores 9, and DataCamp has not described its student record. Uplimit, Mindmax and Novali AI score the same, and their use is small or unpublished.

4. **Small teams have already written down TeachPad's main idea, that the model teaches and code holds the gate.** An open-source project called Tutor MCP uses the slogan "Generative teaching. Deterministic pedagogy." ([GitHub][s6-tutormcp-repo]). Another, Mastery Tutor, updates one lesson page after each teaching turn and keeps its mastery rules in a script, and it has 3 GitHub stars ([GitHub][s6-masterytutor-arch]). A startup named Meridian, founded by two high school students, describes generated lesson pages with a gate at the end of each module ([Meridian][s3-mer1]; [LinkedIn][s3-mer2]). A small team can describe the same idea in months, so the idea alone will not set TeachPad apart from these projects.

5. **The most used products are general chatbots, and Duolingo leads the products built for learning.** ChatGPT reported 1.2 billion weekly users in September 2026, and about one message in ten from adult accounts asks for tutoring or teaching ([OpenAI][s7-oai-dev]; [NBER][s7-nber1]). Duolingo has 153.1 million monthly users ([Duolingo][s7-duol1]). Khan Academy says more than 100 million people used it in the 2025-26 school year, and its tax return for 2024-25 counts 8.5 million in an average month ([Khan Academy][s7-ka-li]; [tax return][s1-ka-990]). Quizlet claims more than 60 million monthly users ([Quizlet][s7-qz2]).

6. **Few of the people who are given a tutor take it up, and few who choose one come back.** Khan Academy says about 15% of students who have its tutor Khanmigo use it ([Khan Academy][s7-ka-blog]). Oboe, the best-funded startup we found that taught any topic to adults, replaced its tutor with a feed of puzzles on 3 November 2026 after raising $20 million ([TechCrunch][s3-oboe2]). Its chief executive wrote that "not many people were motivated to actively learn every day" ([LinkedIn][s3-oboe4]).

7. **Companies that teach from expert interviews describe checks and mastery tracking in place of a gate held in code.** Five do this work: BeOrigin, Novali AI, Napster Learn, Omnora and JoySuite. Each builds a course or an AI version of the expert. Our own pipeline from interview to lesson plan exists as prototypes and has yet to produce its first lesson plan.

8. **Other products use the names "Pedagogue" and "living textbook", which this project used until it became TeachPad.** A UK platform named Pedagogue trains school staff to use AI and includes an AI tutor ([pedagogue.io][s4-pg1]). Enki, a product for software engineers, calls its courses a "living textbook" ([Caffinix][s4-en1]).

**We recommend aiming TeachPad at teams that learn from their own experts.** Lessons built from a team's own experts and taught under gates held in code are what no product offers, and our own team learning Runvil is the first case. Compare our drafted gate rules with DeepTutor's gate code before they are written into Runvil, and try three of the closest products and three of the most used by hand. Measure what the model costs for each lesson hour as soon as the tutor runs on a live model. Check the names "TeachPad" and "learning companion" before the next release, for trademark conflicts and for confusion in search results.

**The evidence is public pages read on one day, and nobody has tried the products.** AI research agents searched public sources on 22 November 2026 and covered about 275 learning products and open-source projects. A few user counts, such as Duolingo's, come from company filings, and most are the makers' own claims. Every closeness score comes from makers' pages, source code, press and reviews. Small products are easy to miss, so a product that ships the whole design may exist unseen. We judge that unlikely for any product in English with more than a few thousand users. Our confidence in that judgment is medium: six separate searches, each covering a different kind of product, found the same gaps. For products in Chinese our confidence is low, because most of them were read through press summaries.

**The audience, the names, and whether the gate code serves other assistants are still the team's to decide.** The first choice is between teams and self-learners. The second is whether the names TeachPad and "learning companion" clash with other products. The third is whether the code that holds TeachPad's gates should also work behind other people's assistants, where learners already spend their time.

## Contents

- [Terms and products](#terms-and-products)
- [No product ships the whole design, and twelve score 9 or more out of 12](#no-product-ships-the-whole-design-and-twelve-score-9-or-more-out-of-12)
- [General chatbots have the most users, and Duolingo leads the products built for learning](#general-chatbots-have-the-most-users-and-duolingo-leads-the-products-built-for-learning)
- [Chatbots teach by conversation at the learner's pace, and older engines enforce gates on fixed screens](#chatbots-teach-by-conversation-at-the-learners-pace-and-older-engines-enforce-gates-on-fixed-screens)
- [Each part of the design ships somewhere, and one combination ships nowhere](#each-part-of-the-design-ships-somewhere-and-one-combination-ships-nowhere)
- [Other products gained the parts of the design between spring 2025 and autumn 2026](#other-products-gained-the-parts-of-the-design-between-spring-2025-and-autumn-2026)
- [Few students who get a tutor take it up, and venture money is scarce](#few-students-who-get-a-tutor-take-it-up-and-venture-money-is-scarce)
- [Recommendations: aim at expert-built lessons for teams, and learn from the nearest products](#recommendations-aim-at-expert-built-lessons-for-teams-and-learn-from-the-nearest-products)
- [What to watch: a large lab is likely to ship a tutor-written page within a year](#what-to-watch-a-large-lab-is-likely-to-ship-a-tutor-written-page-within-a-year)
- [Other products use the names "Pedagogue" and "living textbook"](#other-products-use-the-names-pedagogue-and-living-textbook)
- [The search read public pages on one day, and nobody tried the products](#the-search-read-public-pages-on-one-day-and-nobody-tried-the-products)
- [Sources](#sources)

## Terms and products

**Terms**

- **TeachPad** - our adaptive tutor. Its design is a desktop app with a textbook page on the left and a tutor chat on the right. Our own documents call it a "learning companion". Version 0.1.0, released on 22 November 2026, plays a sample lesson with a stand-in tutor. A tutor that runs on a live model is the next step.
- **The prototype prompt** - `tutor.md`, an earlier version of the idea. It is a prompt that builds one tutor file per topic, and each file runs inside an AI coding assistant.
- **Model** - a large language model, such as the ones behind ChatGPT, Claude and Gemini. Some sources write LLM.
- **Model tutor** - a tutor in which a model does the teaching.
- **General chatbot** - an assistant for any task, such as ChatGPT, Gemini, Claude or Copilot.
- **Large lab** - a maker of a general chatbot: OpenAI, Google, Anthropic or Microsoft.
- **Learning mode** - a setting that makes a general chatbot behave as a tutor. ChatGPT study mode, Claude learning mode and Gemini Guided Learning are learning modes.
- **Older engine** - an adaptive learning program built before language models, such as ALEKS. It teaches through fixed screens of questions and content.
- **Living page** - a lesson page that the tutor writes during the lesson, with text, diagrams and quizzes.
- **Board** - a whiteboard on the screen that a tutor writes on during a lesson.
- **Mastery gate** - a check that decides when the next step of a lesson opens. TeachPad's design has three kinds: a practice gate that blocks until the student gives two right answers in a row, a quiz gate with one question, and a read gate that the student opens at will.
- **Gate held in code** - a mastery gate where the product's own program decides whether the student passed, and the model cannot overrule it.
- **Student record** - what the product stores about one student's skills between sessions.
- **Two-way adaptation** - the lesson slows down or steps back for a stuck student, and skips ahead for a student who already knows the material.
- **Lesson plan** - the tutor's input for one lesson: ordered steps, key ideas, check tasks and common mistakes.
- **Expert capture** - drawing knowledge out of an expert, by interview or from the expert's documents, and turning it into lessons.
- **Skill file** - a file of prompts that turns an AI coding assistant, such as Claude Code or Cursor, into a tutor.
- **Closeness score** - a product's total on six parts of TeachPad's design, from 0 to 12. The parts are tutor dialogue, living page, mastery gates, two-way adaptation, open subject, and adults and expert knowledge, each worth 0 to 2 points. A question mark means the maker's public pages leave part of the score open.
- **Filing, vendor claim, tracker** - the three main kinds of source for a number. A filing is a document a company owes to a regulator or to investors. A vendor claim is the maker's own statement. A tracker is an outside service that counts or estimates, such as GitHub's star count.
- **Monthly users** - people who used a product in a month. Registered users and downloads count everyone who ever signed up or installed, so they run far higher.
- **SD** - standard deviation, the unit for the size of a learning gain. By the general convention 0.2 SD is small, 0.5 medium and 0.8 large. For school programs, one benchmark counts 0.2 SD or more as large ([Kraft][x-kraft]).
- **Runvil** - our open-source prompt engine. The tutor's rules are to run as code inside it, written in the Lua language.

**Products named in more than one place**

- **ALEKS** - McGraw Hill's adaptive math program, an older engine.
- **Area9 Rhapsode** - an older engine sold to employers and publishers.
- **Brilliant and Koji** - Brilliant is an app of interactive math and science lessons. Koji is its voiced model tutor.
- **DataCamp AI Tutor** - a model tutor that teaches data and AI courses at DataCamp, an online school.
- **DeepTutor** - an open-source learning app from the University of Hong Kong.
- **Engram** - an open-source skill file that adds a tutor, a grader and a review schedule to a coding assistant.
- **Gauth** - a homework app owned by ByteDance. Gauth AI Course is its lesson feature.
- **Gemini study notebooks** - a Google product that builds a study plan and lessons toward a goal. Gemini Notebook, formerly NotebookLM, is a separate Google tool for studying one's own documents.
- **Hyperknow** - a startup whose model tutor teaches on a board.
- **Khanmigo** - Khan Academy's model tutor.
- **Kudu** - a company that turns a professor's material into a textbook with a tutor.
- **Mastery Tutor** - an open-source skill file that keeps a lesson page and mastery rules.
- **Meridian** - a startup that generates courses with a gate at the end of each module.
- **Mindmax** - a startup with a voice tutor for professional exams.
- **Novali AI** - a startup that interviews experts and coaches learners from the interviews.
- **Oboe** - a startup that taught any topic to adults until November 2026.
- **Teacher-to-Skills** - an open-source project that turns one teacher's material into a tutor.
- **Tutor MCP** - an open-source server that holds mastery rules for any assistant that calls it.
- **Uplimit** - a corporate training company, owned by Handshake, whose AI Skills Instructor teaches from an employer's documents.

## No product ships the whole design, and twelve score 9 or more out of 12

**Closeness is scored on six parts of TeachPad's design.** Each product gets 0, 1 or 2 points on each part, for a closeness score out of 12.

- **Tutor dialogue.** A model tutor talks with one student, guides, and holds back answers.
- **Living page.** The tutor writes a structured lesson page during the lesson, with text, visuals and quizzes.
- **Mastery gates.** The product blocks the next step until the student passes a check, and the product's code enforces the block.
- **Two-way adaptation.** A student record for each skill slows the lesson for a stuck student and shortens it for a student who is ahead.
- **Open subject.** The product teaches any topic from a lesson plan, an upload or a prompt.
- **Adults and expert knowledge.** The product serves adults or professionals, and its lessons come from expert capture.

The six parts come from TeachPad's own design, so a high score means "resembles TeachPad" and is silent on quality. TeachPad's design would score 12. What exists today would score far lower: version 0.1.0 plays a scripted lesson with a stand-in tutor, and the prototype prompt teaches in a chat where the model itself keeps count of right answers. So the scores compare products that people can use today with a design that is still to be built. A project with a handful of users can outscore a product with millions, because a clear description in a project's documentation is enough to earn points.

Of the roughly 275 products that our search of 22 November 2026 covered, twelve score 9 or more and 24 score 8. Four of the 24 stand out: ChatGPT study mode for its reach, Brilliant with Koji for a tutor that acts on the page, Gauth AI Course for a tutor-written board in a widely used app, and Kudu for a textbook built from one expert's material. Table 1 lists them with the twelve.

| Product | Score | What it has | What it lacks or leaves unpublished | Reach or size |
| --- | --- | --- | --- | --- |
| Mastery Tutor (open source) | 11? | Lesson page updated every turn, staged hints, mastery rules in a script | Runs inside a coding assistant that may skip the rules | [3 GitHub stars][s6-masterytutor-repo] |
| DeepTutor (open source) | 10 | Generated book, tutor chat, code-held gate for facts, skips known material | Book and gated tutor are separate; lessons come from the learner's uploads | [61,585 GitHub stars][s6-deeptutor-repo] |
| Engram (open source) | 10? | Tutor, a separate grader, and a review scheduler in code | Lessons arrive as chat messages | [1,489 GitHub stars][s6-engram-repo] |
| Tutor MCP (open source) | 10? | Rules engine that any assistant can call, plus a desktop client | Lessons arrive as chat messages; rule on holding back answers unstated | [111 GitHub stars][s6-tutormcp-repo] |
| Meridian | 10? | Generated lesson pages, quizzes in the page, a gate per module | Who decides the gate is unpublished; aimed at students | [Team of 2][s3-mer4]; users unpublished |
| Gemini study notebooks (Google) | 9? | Diagnostic quiz, record of 100+ objectives, generated lessons with quizzes | The student can browse past the recommended lesson | [Free][s2-g-studynotebooksli], in an app with [1 billion monthly users][s7-goog-1b] |
| DataCamp AI Tutor | 9? | Lessons generated live from expert-made parts, hints as questions | The model decides when to move on; only data and AI topics | [300,000+ learners][s5-dc3] |
| Hyperknow | 9? | Generated course taught on a whiteboard that fills in live | A block on progress is unpublished | [100,000+ learners][s3-hyp2] |
| Uplimit AI Skills Instructor | 9? | Objective, mastery criteria and documents in; spoken, illustrated lesson out | Blocking rule and memory are unpublished | Corporate customers; [sold to Handshake][s5-up5] |
| Mindmax | 9? | Voice tutor for adult exams that draws diagrams as it talks | Who decides when to move on is unpublished; only professional exams | [Launched November 2026][s3-mmx2]; users unpublished |
| Teacher-to-Skills (open source) | 9? | Turns one teacher's recordings and books into a tutor | Chat only; the rules are prompt text | [488 GitHub stars][s6-t2s-repo] |
| Novali AI | 9? | Expert interviews, a Socratic coach, mastery tracking | Gate and tutor behavior unverified; known from one marketing page | [Team of 2, per a tracker][s5-n2] |
| ChatGPT study mode (OpenAI) | 8 | Socratic tutor with quizzes, flashcards and interactive visuals in the chat | The learner moves on at will; memory is general chat notes | [1.2 billion weekly ChatGPT users][s7-oai-dev] |
| Brilliant with Koji | 8 | Tutor holds back answers, marks up the page, follows a skill map | Lesson pages are made ahead of time; fixed catalog | [10 million+ learners][s4-br7] |
| Gauth AI Course (ByteDance) | 8? | Tutor writes on a shared lesson board with quick checks | Gate and student record are unpublished | [100 million+ Google Play downloads][s3-gau2] |
| Kudu | 8? | One professor's materials become a textbook and a tutor | Adaptation per skill is undescribed | [8 courses at UCLA][s4-ku1] |

*Table 1. The twelve products that score 9 or more out of 12 for closeness to TeachPad's design, and four chosen from the 24 that score 8. A question mark means the maker's public pages leave part of the score unsettled. Scores come from public pages and source code; no product was tried by hand.*

Twenty more products score 8, and they leave different parts open. Area9 Rhapsode, for example, teaches through fixed screens. The twenty include Gemini Guided Learning, Gemini Notebook, Microsoft's Study and Learn Agent, Perplexity Learn Mode, Khanmigo, Duolingo, Area9 Rhapsode, the coding school CodeSignal Learn, the corporate training engine Amplifire, and the tutor that Oboe has since retired.

**DeepTutor is the closest popular product, and its source code states TeachPad's main idea.** That idea is that the model teaches and code holds the gate. DeepTutor is a learning app that the learner hosts and feeds with documents. Its Book feature compiles each chapter into "editable typed blocks - text, callouts, quizzes, flash cards, timelines, code, figures, interactive HTML, animations, concept graphs, deep dives, and user notes - with its own Page Chat" ([GitHub][s6-deeptutor-repo]). Its Mastery Path feature runs a tutor chat behind a mastery gate. The source file for that feature says "the model decides what to teach and how to question - while the gate that decides whether the learner may advance is a deterministic engine call" ([source code][s6-deeptutor-cap]). For facts and procedures the gate asks for 90% accuracy, weighted toward recent answers, and code alone decides. For concepts the tutor model judges an answer in which the learner explains the idea back, and the code records the verdict, so the gate is held in code for facts and procedures only. The gate flags answers that used hints, and it skips objectives the learner has already proven ([source code][s6-deeptutor-policy]). The project adopted this design in June 2026, in place of a fixed sequence of stages ([release note][s6-deeptutor-rel145]). DeepTutor differs from TeachPad in three ways: its book is compiled before reading, its gated tutor is a separate chat, and its lessons come from the learner's own documents alone. Its published evaluation uses simulated students ([paper][s6-deeptutor-paper]). One user, running a modified copy, reported in October 2026 that the tutor "repeatedly stopped teaching even when the material was available" ([issue report][s6-deeptutor-issue1973]).

**Gemini study notebooks is the closest product from a large company.** Google calls it "a dedicated space that turns Gemini into an interactive, adaptive learning platform". The student states a goal or uploads class material and takes a diagnostic quiz. The product splits the goal into "more than 100 specific learning objectives", labels each one "Strengths", "Focus areas" or "Not started", and writes short lessons with practice quizzes for the gaps ([Google][s2-g-studynotebooks]). This student record lasts across sessions and updates after each quiz. The dashboard recommends the next lesson, and the student can still "filter and browse" the others, so the product keeps a record and leaves the gate out. Google's announcements give neither a usage figure nor a study. Google has also not said whether the chat inside a lesson holds back answers, or whether a lesson changes while the student works through it.

**The DataCamp AI Tutor is the closest product for working adults that reports real use.** More than 300,000 learners have used it ([Superintel][s5-dc3]). DataCamp bought Optima, the startup that built the tutor, in November 2025 and launched courses taught by a tutor beside its 700 older courses ([DataCamp][s5-dc1]). DataCamp says of the tutor: "This is not an AI chatbot bolted onto existing content." ([DataCamp][s5-dc2]). DataCamp's experts prepare the objectives, facts and figures of a course and a lesson plan, and the model generates the lesson from them for each learner. When a learner is stuck, the tutor asks the learner to explain their thinking, treats one error at a time, gives hints as questions, and offers to show the solution. The model is steered by "at least 30 core and 50 secondary teaching behaviors" ([Chain of Thought podcast][s5-dc4]). The model checks each answer against success criteria and decides when the learner moves on. DataCamp's pages and interviews give neither a mastery rule nor a description of its student record.

**Mastery Tutor, Tutor MCP and Engram are small projects in which a model teaches and code holds the rules.** All three run behind someone else's assistant: the assistant holds the conversation, and the project's code is meant to hold the rules. Mastery Tutor and Engram are skill files for AI coding assistants. Tutor MCP is a server that any assistant can call through the Model Context Protocol, a standard way for assistants to use outside tools. Mastery Tutor's design note says "Every substantive teaching turn updates one learner-facing local HTML classroom", and it keeps its mastery rules in a Python script ([design note][s6-masterytutor-arch]). It is the nearest match to a living page beside a chat, and one person built it in three months ([GitHub][s6-masterytutor-repo]). Tutor MCP keeps a map of skills and their prerequisites, a mastery estimate for each concept and review dates in its server. It ships a desktop client built with Tauri, the desktop framework TeachPad uses ([GitHub][s6-tutormcp-client]). Engram splits the tutor from a grader that never sees the lesson, and its author publishes a test of that grader against answers marked by hand ([GitHub][s6-engram-repo]). Whether the assistant can skip the rules in Mastery Tutor and Engram is unconfirmed. For Tutor MCP the open point is whether its tutor holds back answers.

**Meridian, Hyperknow, Mindmax and Gauth write a page or a board during the lesson.** Meridian's home page says "Nothing unlocks until you've actually shown it." and describes lessons that are "regenerated for the version of you on the second attempt" ([Meridian][s3-mer1]). It also says the product "refuses to advance you until an AI tutor agrees you actually understand it", so the model may have a say in the gate. Its launch on Product Hunt, a site where new products are voted on, drew one vote ([Product Hunt statistics][s3-mer3]). Hyperknow's founder describes an "AI professor" that teaches on an interactive whiteboard, and the company claims more than 100,000 learners and has raised $1 million ([LinkedIn][s3-hyp2]; [The GenAI Magazine][s3-hyp3]). Mindmax comes from Ed Cooke, a co-founder of the language app Memrise, and Daniel Zohar, Memrise's former chief technology officer. It announced a round of about $3.3 million on 5 November 2026 and aims a voice tutor at adults who study for professional exams ([Tech Funding News][s3-mmx2]; [LinkedIn][s3-mmx3]). Gauth launched a shared board on 1 October 2026 where the tutor "writes its explanation directly onto the board beneath the student's current step" ([PR Newswire][s2-prn-gauth]). Hyperknow and Gauth describe quizzes inside the lesson and publish no rule that blocks progress. Mindmax says it checks understanding before the learner moves on, and who decides is unpublished.

**Brilliant's Koji is the closest tutor with millions of users that acts on the page.** Brilliant sells interactive lessons in math, science and computing, and it added Koji to its math and coding courses on 29 May 2026. Koji "can also reach into the concept and change it - highlight part of a diagram, sketch on your screen, annotate a graph, or pose a smaller question", and Brilliant says he "never, ever just hands you the answer" ([Brilliant][s1-br-blog]; [Brilliant help][s4-br2]). A map of skills links each skill to the ideas it depends on, and it decides when the learner reviews and when the learner speeds up ([Brilliant][s4-br3]). People design the lesson pages ahead of time, the catalog is fixed, and paying users can open any lesson in any order ([Brilliant help][s4-br11]).

**Uplimit, Kudu, Teacher-to-Skills and Novali start from an expert's material.** Uplimit's AI Skills Instructor takes a learning objective, a description of mastery and existing documents, and teaches each employee with voice and generated visuals ([Uplimit][s5-up2]). Handshake, a job network that counts 25 million job seekers, bought Uplimit on 30 June 2026 and on 22 September 2026 launched a free "AI Skills Studio" for job seekers, built on Uplimit's technology ([Handshake][s5-up5]; [PR Newswire][s5-up8]). Kudu turns one professor's notes and sources into a $25 textbook with a tutor that "will not solve a homework question outright", and UCLA runs eight such courses ([Kudu][s4-ku2]; [Kudu][s4-ku1]). Teacher-to-Skills reads a teacher's videos, books, interviews and marked work and produces a tutor that teaches to that teacher's standard ([GitHub][s6-t2s-repo]). Novali AI describes guided expert interviews, a Socratic coach and mastery tracked "decision by decision", a phrase its page leaves unexplained ([Novali][s5-n1]). Three of the four read existing material and skip the interview. Novali interviews, and its product is known from one marketing page.

**The learning modes of the general chatbots score 7 or 8, and the learner sets the pace in every one.** Claude learning mode scores 7, and the modes of ChatGPT, Gemini, Microsoft and Perplexity score 8. ChatGPT study mode launched on 29 July 2025 and is "powered by custom system instructions" ([OpenAI][s2-oai-studylaunch]). During 2026 OpenAI added interactive math and science modules, quizzes, saved flashcard decks and pages built from a conversation ([OpenAI release notes][s2-oai-relnotes]). Claude learning mode launched on 2 April 2025 ([Anthropic][s2-anth-edulaunch]), and Gemini Guided Learning on 6 August 2025 ([Google][s2-g-guidedlaunch]). Each mode changes how a general chat model behaves. ChatGPT and Claude do it with a prompt that the maker writes, and Gemini does it with training of the model itself. Each keeps the chatbot's general memory in place of a student record. OpenAI's own help page says of study mode that "there may be times when it gives a direct answer" ([OpenAI help][s2-oai-studyhelp]).

**More than ten open-source projects resemble the prototype prompt.** Each is a skill file that teaches inside an AI coding assistant and saves the learner's state in local files. The most starred are rederive with 1,516 stars, Engram with 1,489, tutor-skills with 1,415 and learn-anything with 529 ([GitHub][s6-gh-api]). The two closest in form are StepsToGreat and a project named Tutor. Like the prototype, each keeps its teaching rules and the learner's progress in plain-text files that any assistant can read, and each has 2 or 3 stars ([GitHub][s6-stepstogreat-repo]; [GitHub][s6-yusentutor-repo]). Among these projects the prototype alone has three features: a famous-teacher persona for each topic, a fixed rubric for vetting web sources, and the learner's progress saved in a single line of the tutor file. The best-known tutor prompt, Mr. Ranedeer, has 29,527 stars, and its documentation now opens with the word "DISCONTINUED" ([GitHub][s6-ranedeer-repo]).

## General chatbots have the most users, and Duolingo leads the products built for learning

The largest numbers belong to general chatbots, where learning is one use among many. Among products built for learning, Duolingo has the most monthly users and Quizlet claims the second most, and Khan Academy reports the largest yearly count. Table 2 gives fourteen of the largest counts of recent use, by week, month or year, from ChatGPT's 1.2 billion weekly users down to i-Ready's 17 million students.

| Product | Users | Measure | Date | Kind of source |
| --- | --- | --- | --- | --- |
| ChatGPT | [1.2 billion][s7-oai-dev] | Weekly users | 29 September 2026 | Vendor claim |
| Gemini app | [1 billion+][s7-goog-1b] | Monthly users | 11 August 2026 | Vendor claim |
| Doubao, ByteDance's chatbot | [419 million][s2-qm-sep2026] | Monthly users in China | September 2026 | Tracker |
| Duolingo | [153.1 million][s7-duol1] | Monthly users | July to September 2026 | Filing |
| Microsoft Copilot | [150 million+][s7-rai1] | Monthly users, all Copilot apps | July to September 2025, the latest published | Vendor claim |
| Khan Academy | [100 million+][s7-ka-li] | People in the 2025-26 school year | October 2026 | Vendor claim |
| Quizlet | [60 million+][s7-qz2] | Monthly users | Undated page, read November 2026 | Vendor claim |
| Claude (Anthropic) | [56 million][s7-claude1] | Monthly users of the app | April to June 2026 | Tracker |
| Gemini Notebook, formerly NotebookLM | [30 million+][s7-nblm1] | Users, period unstated | 16 July 2026 | Vendor claim |
| McGraw Hill, including ALEKS | [26.7 million][s7-mh3] | Paying users of digital products | Year to March 2026 | Filing |
| Cambium Learning, maker of Lexia | [24 million][s1-cam-imp] | Students served | 2025-26 school year | Vendor claim |
| Prodigy | [20 million+][s7-prod1] | Students a year | 2026 | Vendor claim |
| IXL | [18 million][s7-ixl1] | Students | 2026 | Vendor claim |
| i-Ready | [17 million][s7-iready1] | Students | September 2026 | Vendor claim |

*Table 2. Fourteen large products with a count of current use, the measure behind each count, and the kind of source. A general chatbot is listed where it offers a learning mode; Doubao's is called Doubao Aixue, and Copilot's is Study and learn. A product's weekly count is smaller than its monthly count, and its monthly count smaller than its yearly count, so compare within one measure.*

Other general chatbots report more than 100 million monthly users, among them Meta AI, DeepSeek and Grok. We found no learning mode in any of the three ([Visibian][s2-visibian]; [QuestMobile][s2-qm-sep2026]).

**Headline counts overstate current use.** Some count everyone ever reached. Coursera, which merged with Udemy in May 2026, says it has reached more than 310 million learners, and it has 1.687 million paid subscribers ([Coursera][s7-cour4]; [Coursera][s7-cour3]). CK-12, a free textbook platform, counts 453 million users served ([CK-12][s4-ck4]). Others count a whole year. Khan Academy's annual report for 2024-25 counts 104.9 million learners active in the year, and its tax return for the same year says "an average of 8.5 million learners were active on our platform each month" ([School AI Atlas][s1-ka-atlas]; [tax return][s1-ka-990]). And some are disputed. One secondary source gives Gauth 86 million monthly users. Estimates built from app-store data come to about 9 million monthly users on each of Android and iOS ([Bittime][s7-gauth1]; [Outearn][s7-out3]; [Outearn][s7-out2]). The figure of 86 million is unconfirmed.

**The chatbot makers publish usage for the whole app and almost nothing for each learning mode.** One figure exists: OpenAI says more than 180,000 teens used study mode in one week in autumn 2026 ([OpenAI][s7-oai-teens]). The best measure of learning inside ChatGPT is a study by researchers at OpenAI, Duke and Harvard, published in 2025 as a working paper of the National Bureau of Economic Research. It classified messages from adult consumer accounts between May 2024 and June 2025 and found that 10.2% asked for tutoring or teaching, and a further 4.2% concerned computer programming, which the study counts apart from tutoring ([NBER][s7-nber1]). By early 2025, 26% of US adults had used ChatGPT "to learn something new" ([Pew][s7-pew3]). Among students the habit is wider: 54% of US teens had used chatbots for schoolwork by autumn 2025, and 95% of UK undergraduates used AI in at least one way by December 2025 ([Pew][s7-pew1]; [HEPI][s7-hepi1]). Asked how they had learned to code in the past year, 52.6% of developers in the Stack Overflow survey of 2026 named AI tools, second to documentation at 58.9% ([Stack Overflow][s7-so1]).

**Large school practice programs keep models away from students.** Zearn, a math program used by one in four US elementary students, states that it includes "no chatbots, no AI-generated lesson responses" ([Zearn][s7-zearn1]; [Zearn][s1-zearn-priv]). The strategy chief of IXL describes its suite of AI tools for teachers as "not AI for students per se" ([Getting Smart][s1-ixl-gs]). The AI work at i-Ready in 2026 is a pilot that listens to pupils read aloud ([NBC News][s1-ca-nbc]). These programs are large: IXL reports 18 million students and i-Ready 17 million.

**Among startups built on models, the users are in language practice and study tools.** ELSA Speak, an app for English pronunciation, told the press it has about 50 million users, and one of its own pages gives 100 million; neither count states a period ([Ynet][s3-els1]; [ELSA][s7-elsa2]). Praktika, a language app, claims more than 30 million learners. Speak, another language app, claims more than 15 million downloads, and Gizmo, a study app, 13 million users ([Praktika][s3-pra2]; [Speak][s3-spk1]; [TechCrunch][s3-giz1]). Turbolearn, a note-taking and study app, reports 10 million users with a full-time team of ten and $750,000 raised ([Turbolearn][s3-tur1]; [Inc.][s3-tur3]). The startups that teach any topic to adults are far smaller: Hyperknow claims more than 100,000 learners and Wondering, a course generator, 12,000 ([LinkedIn][s3-hyp2]; [Aiolum][s3-won3]).

**Open-source projects are counted in GitHub stars, and a star shows only that someone bookmarked the project.** DeepTutor has 61,585 stars. OpenMAIC, a project from Tsinghua University that generates a lesson with slides, quizzes, and AI teachers and classmates, has 44,142 ([GitHub][s6-deeptutor-repo]; [GitHub][s6-openmaic-repo]). GitHub lists 1,014 projects under the tag `ai-tutor`, and 23 of the 30 most starred were created in 2026 ([GitHub][s6-gh-topic-ai-tutor]). Neither DeepTutor nor OpenMAIC publishes a count of learners that we could find. Kolibri, an older open-source platform for offline lessons, says it has reached 13 million learners and educators ([Learning Equality][s6-le-gsoc]).

## Chatbots teach by conversation at the learner's pace, and older engines enforce gates on fixed screens

The products fall into six groups, listed in Table 3 with the largest product of each and the one closest to TeachPad's design.

| Group | Largest product | Its reach | Closest product | Score |
| --- | --- | --- | --- | --- |
| General chatbots with a learning mode | ChatGPT | [1.2 billion weekly users][s7-oai-dev] | Gemini study notebooks | 9? |
| Established adaptive platforms | Duolingo | [153.1 million monthly users][s7-duol1] | Brilliant with Koji, level with Duolingo and Area9 Rhapsode | 8 |
| Startups built on models | ELSA Speak | [About 50 million users][s3-els1] | Meridian | 10? |
| Textbooks with an AI helper | CK-12, a free textbook platform | [453 million+ users served, all time][s4-ck4] | Kudu, level with Enki and Hi Tuto | 8? |
| Learning for professionals and developers | Coursera with Udemy | [310 million+ learners reached][s7-cour4] | DataCamp AI Tutor, level with Uplimit and Novali AI | 9? |
| Open-source projects | DeepTutor | [61,585 GitHub stars][s6-deeptutor-repo] | Mastery Tutor | 11? |

*Table 3. The largest product and the product closest to TeachPad's design in each of six groups, with its closeness score out of 12. Reach is the largest headline figure available, with its measure, so the six figures are unlike each other. Enki is a course product for software engineers, and Hi Tuto is an open-source project that writes a lesson page live.*

**General chatbots with a learning mode reach the most people and leave the pace to the learner.** Every large lab ships a learning mode, and so does the search company Perplexity: ChatGPT study mode, Claude learning mode, Gemini Guided Learning, Microsoft's Study and Learn Agent and Perplexity Learn Mode ([Microsoft][s2-ms-slga]; [Perplexity][s2-pplx-sep2025]). OpenAI, Google, Anthropic and Microsoft offer theirs free or inside a plan the learner already has ([OpenAI help][s2-oai-studyhelp]; [Google][s2-g-studentoffer]; [Glasp][s2-glasp-claudegemini]; [Microsoft][s2-ms-slga]). During 2026 the chatbots began to produce pages and visuals as well as chat text. Gemini Notebook builds an interactive site with flash cards and quizzes from a request such as "make me a site" ([Engadget][s2-engadget-site]). Claude draws interactive visuals inside its answers ([Anthropic][s2-claude-visuals]). ChatGPT turns a conversation into a page with charts ([OpenAI release notes][s2-oai-relnotes]). By late 2026 schools had controls that hold a student inside the learning mode, while the student still sets the order and pace of the lesson ([OpenAI help][s2-oai-studyhelp]; [Microsoft][s2-ms-sllock]; [Google][s2-g-workspacegl]). One review, from a company that sells a study tool, found that Claude "resists the 'just give me the answer' pressure the longest" and that ChatGPT "Can fold under user pressure" ([Glasp][s2-glasp-modes]). The makers' own trials found mixed results. OpenAI reported that college students who prepared with study mode earned "roughly a 15% higher score relative" to students who prepared without AI, on a microeconomics exam in a trial of more than 300 students. On a neuroscience exam the result was "not distinguishable" from study with web resources ([OpenAI][s2-oai-outcomes]). Google and a partner reported that Gemini Guided Learning, used in class with teachers leading each session, produced a gain of 0.26 SD over comparison classrooms among 1,763 students in Sierra Leone ([Fab AI][s2-g-sierraleone]).

**Established adaptive platforms hold the gates and the student records, and the oldest of them still teach without dialogue.** ALEKS, MATHia, Math Academy and Knewton Alta are older engines that enforce mastery gates in code and keep a student record for each skill. Their closeness scores are 5 or 6, because they teach through fixed screens and earn nothing for dialogue or a living page. Duolingo scores 8: a model drafts its lessons ahead of time, its path is gated in code, and its chat is a practice partner ([Duolingo][s1-duo-148]). Math Academy's founder says its AI is "closer to an expert system" and "not an LLM chatbot" ([podcast episode summary][s1-ma-sunny]). College publishers added a tutor chat to their courseware, and Brilliant added Koji ([Cengage][s1-cg-2025]; [Macmillan][s1-mac-study]; [Brilliant][s1-br-blog]). Two companies have retired or repackaged the paid tutor features they launched in 2023. Quizlet ended its tutor chat, Q-Chat, on 30 June 2025 ([AI Tools of the Trade][s1-qz-rev]). Duolingo said on 3 November 2026 that it would stop selling its top tier, Duolingo Max, that month. It moved the tier's video calls, spoken practice with a model character, to a cheaper tier after cutting the model cost of a call from "about $0.30" to "well under $0.01" ([Duolingo][s2-duo-q3letter]; [earnings call][s2-duo-q3call]). Area9 Rhapsode is another older engine, and it scores 8 because its newer layer, Mindflow Rapid, builds a gated course on any topic from a prompt or a document ([Area9][s1-a9-genai]). The mastery rules of such systems are more often a simple count of right answers than a statistical estimate: ASSISTments, a homework tutoring system, asks for three right answers in a row. Our earlier research report gives the rules of eight deployed systems ([adaptive-learning-research.md](adaptive-learning-research.md)).

**Startups built on models have users in languages and study tools, and seven have left or paused general tutoring.** The largest, such as ELSA Speak and Praktika, teach languages. The seven are Oboe, Sizzle, Opennote, Learn.xyz, Eureka Labs, Miyagi Labs and Unriddle. Oboe moved to a feed of puzzles ([LinkedIn][s3-oboe4]). Sizzle, a tutoring app, was bought by Campus, an online college. Opennote, a study app, was bought by Reducto, and its app has since shut down ([Campus][s3-siz1]; [BetaKit][s3-opn3]; [Opennote][s3-opn2]). Learn.xyz was discontinued ([SaaSHub][s3-lxz2]). Eureka Labs, Andrej Karpathy's school, has released no course, and a profile on a tech news site describes it as paused since he joined Anthropic in May 2026 ([TECHi][s3-eur4]; [Das Numinous][s3-eur3]). Miyagi Labs moved to exam preparation, and Unriddle turned into Anara, a tool for researchers ([Y Combinator][s3-miy1]; [Anara][s3-ana1]). An eighth startup, Kinnu, kept its free general app and added paid exam preparation ([Fundraising Fox][s3-kin1]). Products for children hold their gates in code and limit what the model may do. Bloomy says "The language model does not choose the curriculum or decide whether a student has mastered a skill." ([Hacker News][s3-bmy1]). A review of Synthesis Tutor quotes an engineer on the project: "No user ever sees live/unchecked LLM output" ([Understand][s3-syn5]). Alpha School's online program says a student "moves forward only after reaching 90 percent mastery on the given topic", and a staff member told a reporter "We are anti-chatbot at Alpha." ([Alpha School][s3-alp9]; [GBH][s3-alp11]). Startups that teach any topic to adults, such as Hyperknow, do the reverse: the model teaches and also decides when to move on. Few of these startups have an independent study of learning. Medly, a UK exam tutor, is the exception. It reports a randomized trial of 644 students, run by an outside evaluator, that found a gain of 0.33 SD ([Medly][s3-med3]).

**Publishers' textbooks put a chat panel beside a fixed page, and a few small products generate the page.** Pearson, McGraw Hill, Cengage, Macmillan, VitalSource and Top Hat all work this way ([Pearson][s4-pe1]; [McGraw Hill][s4-mh2]; [Cengage][s4-ce3]; [Macmillan][s4-ma1]; [VitalSource][s4-vs2]; [University of Iowa][s4-th6]). McGraw Hill counts more than 7.5 million active users across eight AI tools, and Macmillan's AI Tutor served more than 253,000 students in spring 2026 ([McGraw Hill][s4-mh4]; [Macmillan][s4-ma3]). Interactive textbooks such as zyBooks, Runestone and OpenStax place questions inside the text, and course points from the instructor drive completion ([zyBooks study][s4-zy5]; [Runestone][s4-rs1]; [OpenStax][s4-os7]). South Korea's national program of AI digital textbooks lost most of its schools within 18 months. The program launched in March 2025, and about a third of schools signed up ([IAPQB][s4-kr4]). In the first semester of 2026, 5.2% of schools still subscribed ([Newsis][s7-kr1]). Critics, the state audit and a teacher who liked the books all say the rollout was rushed ([Rest of World][s4-kr3]). The small products that generate the page are Kudu, Enki and Hi Tuto. TeachPad reuses two ideas from this group: questions inside the text, as in zyBooks and Runestone, and lessons that open after a correct answer, as in Execute Program, a set of interactive programming courses. A third idea is on our research agenda and still to build: review prompts that return days after the lesson, which Quantum Country and Execute Program built into their texts ([Matuschak and Nielsen][s4-qc1]; [Execute Program][s4-ep1]).

**Products for professionals and developers add a chat helper to a catalog, with a few exceptions.** Coursera and Udemy merged on 11 May 2026, and their adaptive product for employers, Project Helix, is due in the first half of 2027 ([Coursera][s5-c1]; [Coursera][s5-c8]). Workday paid about $1.1 billion for Sana, whose tutor answers questions about the course it is attached to ([Workday][s5-w2]; [Sana help][s5-w6]). Amplifire and Area9 Rhapsode, both older engines, are the exceptions for working adults ([Amplifire][s5-am1]; [Area9][s5-ar5]). For software engineers, Boot.dev passes a lesson when the output of the submitted code matches the answer key, and CodeSignal's tutor Cosmo "checks learners' work" before the next lesson opens ([Boot.dev][s5-b7]; [CodeSignal][s5-csg1]). Two coding schools report shrinking business. Boot.dev's new paid subscriptions were 7.7% lower from May to October 2026 than in the same months of 2025, and CodeCrafters stopped adding new exercises ([Boot.dev][s5-b3]; [PracHub][s5-cc1]). The learning modes of AI coding assistants are prompts that ask the model to teach, and nothing in them enforces a check or keeps a record. Anthropic's documentation says such a mode "doesn't guarantee that something always happens", and a Cursor staff member wrote on 23 October 2026 that "a built-in Study mode doesn't exist yet" ([Anthropic][s5-a1]; [Cursor forum][s5-a7]). In a randomized study by Anthropic, 52 developers learned a Python library, and the group with AI help scored 17% lower on the quiz that followed ([arXiv][s5-a9]).

**Among products for professionals, a small set captures an expert's knowledge by interview.** BeOrigin reads documents, interviews the expert by voice, builds a course and tests the learner ([BeOrigin][s5-be1]). Napster Learn runs a setup in five steps, with an interview, and then a "Digital Twin" of the expert teaches courses ([Napster][s5-np1]). Omnora holds four one-hour AI interviews with each expert and produces playbooks and video training ([Omnora][s5-o2]). JoySuite interviews an expert and builds a virtual expert that answers questions ([Neovation][s5-j1]). None of the four publishes a gate held in code. Delphi and MasterClass On Call present an AI version of a named expert, and both require the expert's consent ([Delphi][s5-d6]; [Engadget][s5-m3]). TeachPad's rule is that the tutor may teach in an expert's manner but never claims to be the expert. One product matches that rule. Beside its Digital Twin, Napster offers a "Mentor", an invented persona that teaches a real expert's material without the expert's likeness ([Napster][s5-np1]).

**Most of the 30 most starred open-source tutors were created in 2026, and the top two come from universities.** Of the 30 most starred projects tagged `ai-tutor` on GitHub, 23 were created in 2026 ([GitHub][s6-gh-topic-ai-tutor]). DeepTutor comes from the University of Hong Kong and OpenMAIC from Tsinghua University ([GitHub][s6-deeptutor-repo]; [GitHub][s6-openmaic-repo]). Smaller projects run as skill files inside AI coding assistants, as the prototype prompt does. Two projects ship as desktop apps built with Tauri, the desktop framework TeachPad uses: Tutor ACP, which is the desktop client of Tutor MCP, and SelfSemester ([GitHub][s6-tutormcp-client]; [GitHub][s6-selfsemester-repo]). Learning results measured on real students are still to come for every open-source tutor released in 2026, and DeepTutor's paper tests on simulated students ([paper][s6-deeptutor-paper]). Useful parts exist under licenses that permit reuse, among them FSRS, a scheduler for review dates ([PyPI][s6-fsrs-pypi]).

## Each part of the design ships somewhere, and one combination ships nowhere

Each of the six parts of TeachPad's design ships in some product today. One combination ships in none: lessons from expert interviews, taught by a model tutor under gates held in code. Table 4 lists each part with the products that have it.

| Part of the design | How common | Who has it |
| --- | --- | --- |
| Tutor dialogue that holds back answers | Standard | Every learning mode, Koji, the publishers' chat tutors, DataCamp |
| Living page or board, written during the lesson | First shipped in late 2025, spread in 2026 | Copilot Learn Live, DataCamp; then Gauth, Hyperknow, Mindmax, Mastery Tutor |
| Mastery gates held in code | Common in older engines and children's products, rare in model tutors | ALEKS, Math Academy, Bloomy; DeepTutor, Tutor MCP, Mastery Tutor, trainingllm |
| Two-way adaptation from a student record | Common in older engines, rare in model tutors | ALEKS, Duolingo, Area9; Gemini study notebooks, DeepTutor, Tutor MCP |
| Open subject, from a prompt or an upload | Standard | Learning modes, course generators, most open-source tutors |
| Lessons for adults built from expert capture | Rare, and mostly small companies | Kudu, Teacher-to-Skills, BeOrigin, Omnora, Napster Learn |
| Expert interviews, a model tutor and gates held in code, together | Found in no product | Nearest: Kudu, Teacher-to-Skills, BeOrigin, Novali, Napster Learn |

*Table 4. The six parts of TeachPad's design, and the one combination found in no product, with how widely each ships as of 22 November 2026.*

**Holding back answers is standard, and few products publish a rule for hints.** Cengage says its assistant "does not provide answers", and Macmillan says its tutor "asks guiding questions rather than giving answers" ([Cengage][s1-cg-2025]; [Macmillan][s1-mac-study]). Two vendors publish an actual rule. Skye, the spoken tutor of Third Space Learning, talks the pupil through the answer after three wrong attempts, and zyBooks shows a hint option after two failed attempts ([Third Space Learning][s1-tsl-ai]; [zyBooks][s1-zy-new]). In TeachPad's design, a right answer that follows a hint earns no credit toward the practice gate. ASSISTments, an older tutoring system, applies the same rule ([vision.md](../vision.md)). Among model tutors the nearest equivalent is DeepTutor, which records which answers used hints ([source code][s6-deeptutor-policy]).

**Older engines enforce gates in code, and most model tutors leave the decision to the model.** Older engines teach without a model, and the children's products that use a model keep it out of the gate decision. Model tutors for adults let the model teach and also decide when to move on. In TeachPad's design the model teaches, code holds the gate, any subject can be taught, and the first students are working adults. Four open-source projects describe the same split between model and code, and all four took it up in 2026: DeepTutor, Tutor MCP, Mastery Tutor and trainingllm, a small project that tracks progress "in application state, not by the LLM" ([GitHub][s6-trainingllm-repo]). Research shows that rules held in code are followed more reliably than rules in a prompt. In a 2026 simulation study, a program that applied the rules to a model's output kept a rule of attempt before hint in all 2,400 runs. The same model, steered by a prompt alone, gave twelve times as many hints as the program did ([adaptive-learning-research.md](adaptive-learning-research.md) describes the study). Research has yet to show that tutors built this way teach better. Ruffle&Riley, a research tutor that holds its teaching plan in a script generated ahead of time, is one of the few tutors with fixed rules that has been tested on learners, and it showed no significant gain over reading in two studies with 200 learners in total ([GitHub][s6-ruffle-repo]). A gate in code also needs a way out that the student can see, or a stalled gate stops the lesson. DeepTutor lets the learner defer an objective, skip a bad question, or claim mastery, and it records such a claim as the learner's own statement ([source code][s6-deeptutor-policy]; [source code][s6-deeptutor-cap]).

**A tutor that writes a full lesson page turn by turn exists in two tiny open-source projects, and larger products write boards or fixed pages.** Pages and boards written for one learner come in three forms. The first form is a screen that fills in as the tutor talks. Microsoft's Copilot Learn Live, a voice tutor with interactive whiteboards, launched in October 2025 ([press report][s2-ppc-copilot]). The DataCamp AI Tutor has generated each lesson as it goes since November 2025, and DataCamp has not shown how the lesson is laid out on the screen. Gauth and Hyperknow write on a board, and Mindmax draws diagrams as it speaks. The second form is a page generated for one learner on request and then left fixed, a close relative of a living page. DeepTutor's book, the lessons in Gemini study notebooks, the sites that Gemini Notebook builds, and the pages ChatGPT makes from a conversation all work this way. Google Research's Learn Your Way belongs here too: it rewrites a textbook chapter for one learner before the learner reads it, and the published design includes no tutor dialogue ([Google Research][s2-g-lyw-blog]). The third form is a tutor that marks up a page made ahead of time, which is how Koji works. The two tiny projects are Mastery Tutor, with 3 stars, and Hi Tuto, which nobody had starred as of 22 November 2026 ([GitHub][s4-ht1]).

**No product joins expert interviews to gated lessons: some read existing material, and others interview and stop at a course.** Kudu, Uplimit and Teacher-to-Skills read material that already exists, which costs the expert no time. BeOrigin, Napster Learn and Omnora interview the expert, and then stop at a course, a video or an AI version of the expert. Novali also interviews, and it describes a coach that tracks mastery. Its product is known from one marketing page, so whether it gates in code is unknown. Two of the interviewing companies are tiny: BeOrigin has three people and Novali two ([LinkedIn][s5-be2]; [Exa][s5-n2]). Omnora has about forty people and more than 650 customer companies ([Omnora][s5-o3]). Larger sellers to employers are adding adaptive tutors of their own. Coursera's adaptive platform for employers, Project Helix, is due in 2027, and Workday has shipped a "personal AI tutor" inside its learning product ([Coursera][s5-c8]; [Workday][s5-w3]). Tutor MCP, an open-source project with 111 stars, already ships a desktop client that connects a learner to an organization's own programme of lessons ([GitHub][s6-tutormcp-client]). Teacher-to-Skills offers a ready checklist for turning an expert's material into teachable rules. It pulls six things from a teacher's material: the sources, what the teacher looks at when judging work, judgment rules, demonstrations, common mistakes with their corrections, and the limits of the method. Where the material is thin it says so in the output ([GitHub][s6-t2s-repo]).

## Other products gained the parts of the design between spring 2025 and autumn 2026

The parts that make up TeachPad's design appeared in other products one after another. The learning modes came first, in 2025. Whiteboards and lessons written by the tutor began in late 2025, with Copilot Learn Live and the DataCamp AI Tutor, and spread during 2026. A student record for each skill reached model tutors with Gemini study notebooks in June 2026, and a gate held in code with DeepTutor in the same month. Table 5 lists these launches with four deals and one withdrawal from tutoring. The deals moved three products for working adults to larger owners. Sana, a corporate learning platform, went to Workday. Optima, the startup that built DataCamp's tutor, went to DataCamp. Uplimit went to Handshake. Coursera and Udemy, two large course catalogs, merged.

| Date | Event |
| --- | --- |
| 2 April 2025 | [Anthropic launches Claude learning mode][s2-anth-edulaunch] |
| 29 July 2025 | [OpenAI launches ChatGPT study mode][s2-oai-studylaunch] |
| 6 August 2025 | [Google launches Gemini Guided Learning][s2-g-guidedlaunch] |
| 23 October 2025 | [Microsoft launches Learn Live, a voice tutor with whiteboards][s2-ppc-copilot] |
| 4 November 2025 | [Workday completes its purchase of Sana][s5-w1], agreed for [about $1.1 billion][s5-w2] |
| 12 November 2025 | [DataCamp buys Optima and launches its AI Tutor][s5-dc1] |
| 28 December 2025 | [DeepTutor's code first appears on GitHub][s6-deeptutor-repo] |
| 11 May 2026 | [Coursera and Udemy complete their merger][s5-c1] |
| 29 May 2026 | [Brilliant announces Koji, a tutor that acts on the lesson page][s1-br-blog] |
| 14 June 2026 | [DeepTutor replaces its fixed lesson sequence with a gated tutor chat][s6-deeptutor-rel145] |
| 25 June 2026 | [Google launches Gemini study notebooks][s2-g-studynotebooks] |
| 30 June 2026 | [Handshake buys Uplimit][s5-up5] |
| 29 September 2026 | [ChatGPT's Pro, Business and Enterprise plans gain pages built from a conversation][s2-oai-relnotes] |
| 1 October 2026 | [Gauth launches a lesson board that tutor and student both write on][s2-prn-gauth] |
| 3 November 2026 | [Oboe replaces its tutor with a feed of puzzles][s3-oboe4] |
| 5 November 2026 | [Mindmax announces about $3.3 million and a voice tutor that draws diagrams][s3-mmx2] |

*Table 5. Launches of the products discussed here, with four company deals and one withdrawal from tutoring, between April 2025 and November 2026.*

## Few students who get a tutor take it up, and venture money is scarce

**Few of the students who are given a tutor take it up.** Khan Academy writes that "only around 15% of students who have access to Khanmigo engage with it" ([Khan Academy][s2-khan-open]). Its founder, Sal Khan, called the tutor "a non-event" for many students ([Chalkbeat][s7-chalk1]). In a randomized trial over two years in Tennessee, 96% of students tried Khanmigo at least once, and the median student messaged it on 33% of the days they practiced ([NBER][s7-nber2]). Khan Academy's answer was to rebuild the tutor so that it steps in during practice ([Khan Academy][s2-khan-pilot]). The AI study app of Estonia's national program had 5,705 monthly users among 19,674 students invited ([AI Leap review][s7-ee1]). These students got the tutor through their schools, so a school's backing was not enough to make them use it.

**The best-funded tutor for curious adults gave up for lack of returning learners.** Oboe raised $20 million, including $16 million in a round led by Andreessen Horowitz in December 2025 ([TechCrunch][s3-oboe2]). Its chief executive gave three reasons for leaving tutoring. Strategic decisions had "left us undifferentiated in the age of LLMs", there was "little organic incentive for people to return to the platform", and "not many people were motivated to actively learn every day" ([LinkedIn][s3-oboe4]). Turbolearn, a study app, says it reached 10 million users, a count that states no period, after raising $750,000, about one twenty-seventh of Oboe's funding ([Turbolearn][s3-tur1]; [Inc.][s3-tur3]). Oboe's chief executive described "a small, loyal user base" ([LinkedIn][s3-oboe4]). The comparison suggests that funding did not decide which product found users.

**A tutor that writes each lesson as it goes costs dollars per hour in model fees.** DataCamp's chief executive says an hour of AI tutoring "can cost several dollars". Moving all 10 million yearly learning hours to the tutor would cost $10 million to $40 million, against yearly revenue running at about $100 million ([Superintel][s5-dc3]). He told a podcast that the cost per learning hour began at ten to twenty times that of DataCamp's earlier courses ([Chain of Thought podcast][s5-dc4]). Duolingo cut the cost of its video calls, spoken practice with a model character, by "moving to open-source models or open-weight models", which are models that a company can run on its own servers ([earnings call][s2-duo-q3call]).

**The largest tutors are free to the learner.** Study mode is part of every ChatGPT plan, the free plan included ([OpenAI help][s2-oai-studyhelp]). Gemini study notebooks are free ([Google][s2-g-studynotebooksli]). Khanmigo, at $4 a month, is among the few that charge the learner ([Khan Academy][s2-khanmigo-pricing]). University contracts for ChatGPT work out to between $1.67 and $3.23 per person per month ([Whizi][s2-whizi-edu]). A tutor that charges has to do something the free ones leave undone, such as holding a gate or teaching an expert's material.

**Venture money for education technology has fallen to a fraction of its peak.** Education technology raised $20.8 billion in 2021 and $2.4 billion in 2024, a fall of about 88%, and $1.28 billion in the first nine months of 2026 ([HolonIQ][s7-hiq-2022]; [HolonIQ][s7-hiq-2024]; [HolonIQ][s7-hiq-q3-26]). Workforce training, the segment a tutor for teams would sell into, took more than 70% of the money in the first three months of 2026, on a few large rounds, and about a quarter of venture funding from July to September 2026 ([Lingobright][s7-lb1]; [HolonIQ][s7-hiq-q3-26]). The largest round since 2023 that we could confirm for a company whose product is an AI tutor was $78 million in December 2024, raised by Speak, a language app ([Speak][s7-spk2]). Knewton, a pioneer of adaptive learning, raised more than $180 million and sold its assets in 2019 for under $17 million ([EdTech Update][s7-knew3]).

**Independent evidence of learning is rare, and for the chatbots' learning modes it is still missing.** A Stanford review of 52 causal studies in schools, updated in November 2026, concludes that AI tools used by students do not consistently improve learning ([Stanford review][s3-mag7]). The learning modes of the general chatbots have been tested only in trials that their makers ran or helped run ([OpenAI][s2-oai-outcomes]; [Fab AI][s2-g-sierraleone]). A small study of TeachPad with real students would add evidence of a kind that few tutors have.

**Developers use AI to learn, approve of it less than before, and in one trial learned less with it.** Asked how they had learned to code in the past year, 52.6% of developers in the Stack Overflow survey of 2026 named AI tools ([Stack Overflow][s7-so1]). Among people learning to code, favourable opinion of AI tools fell from 72.1% in 2024 to 52.8% in 2025 ([Stack Overflow][s7-so3]). In a randomized study by Anthropic of 52 developers, the group with AI help scored lower on the quiz that followed ([arXiv][s5-a9]). The Anthropic result supports a tutor that holds back answers, and the falling approval shows that fewer learners are satisfied with plain AI help.

## Recommendations: aim at expert-built lessons for teams, and learn from the nearest products

The audience comes first, because it decides which competitors TeachPad meets. Four options are open, and Table 6 gives the main point for and against each.

| Option | In its favor | Against it |
| --- | --- | --- |
| Expert-built lessons for teams (recommended) | No product joins expert interviews, a model tutor and code-held gates | Demand is untested; each customer needs its experts interviewed |
| Any topic for self-learners | Largest audience | Free learning modes cover it; Oboe left it |
| Gate code that runs behind other people's assistants | Reaches learners inside assistants they already use | The assistant, and its maker, own the lesson page |
| Keep the current plan: build for our team, choose the audience later | Needs no decision now | Nothing prompts an early look at model cost or the product's name |

*Table 6. Four options for TeachPad's first audience and form.*

1. **Aim the first release with a working tutor at teams that learn from their own experts.** Expert capture joined to a model tutor and a gate held in code is the one combination we found in no product. Several small companies already sell the capture step, and our own capture pipeline, an interviewer prototype and extraction prompts, has yet to produce its first lesson plan. Owner: the product lead. Next step: state this audience in our vision document, `vision.md`, and build the first lesson plan from captured knowledge about Runvil. Confidence: low - nobody has tested demand for it, and tools that a school or an employer assigns often go unused.

2. **Compare DeepTutor's gate code with our drafted gate rules before the rules are written in Lua.** DeepTutor made the same split between model and code and has shipped it in public releases since June 2026. Our earlier research report, [adaptive-learning-research.md](adaptive-learning-research.md), already drafts the gate rules as an algorithm. Owner: whoever writes the mastery rules. Next step: read the DeepTutor file that holds its gate rules, the file that runs its tutor, and its tutor prompt, and list what to adopt. There are four candidates: an accuracy threshold weighted toward recent answers; a rule for concepts in which the learner explains the idea back and the model judges; a record of which answers used hints; and visible ways out of a stalled gate. Treat its numbers as starting values, because DeepTutor has published no result with real students, and its 90% threshold copies Alpha School, whose results are disputed ([source code][s6-deeptutor-policy]; [Fortune][s3-alp4]). Confidence: high for the reading - the source is public under an open-source license that permits reuse, and it already shows one failure to design around, a gate that stalled a user's lesson.

3. **Try the closest and the most used products by hand, to learn what their public pages leave out.** Take three of the closest, DeepTutor, Mastery Tutor and Tutor MCP, and three with wide reach, Gemini study notebooks, the DataCamp AI Tutor and ChatGPT study mode. Owner: the design lead. Next step: take one lesson in each and record three things: whether the tutor gives up the answer under pressure, whether the learner can skip a check, and what the page does during the lesson. Confidence: high - a few hours of use answer these three questions, though use alone cannot show who decides a gate in a closed product.

4. **Show the difference from a chatbot in the first minutes of a lesson.** The student should see the page being written and meet a practice gate that holds. Once the student record exists, the student should see that too. Owner: the design lead. Next step: design the opening of the first lesson around those moments. Confidence: low - the evidence is one founder's account of why Oboe looked the same as a chatbot, and Oboe's larger problem was that learners did not return.

5. **Measure what the model costs for a lesson hour as soon as the live model is connected.** Owner: the Runvil lead. Next step: set a target cost for a lesson hour, and test cheaper models on simple replies. Confidence: high for measuring - DataCamp and Duolingo each describe the high cost of model time first-hand. For a team whose employer pays, DataCamp's figure of several dollars an hour is small, so the cost calls for a careful choice of models and leaves the tutor's design as it is.

6. **Check the names "TeachPad" and "learning companion" before the next release and before any promotion.** Version 0.1.0 was first public on GitHub under the name Pedagogue, and this project called itself a "living textbook". A UK company uses the same name for a product with an AI tutor for adults, and several products use "living textbook". Owner: the project lead. Next step: search trademarks in the markets we plan to enter, and look at what a web search for each name returns. Confidence: medium - the clashes are confirmed, and their legal weight is unknown.

7. **Plan a small study with real students from the first lesson.** Few model tutors have a study with real students. Owner: the research lead. Next step: write the study plan from the list of what to log and the small-group designs in Appendix B of the earlier research report, with our own team as the first group. Confidence: medium - a study is feasible, and a small group taught by the tool's own designer limits what it can show.

8. **Adopt DataCamp's way with mistakes, Engram's separate grader and Teacher-to-Skills's evidence labels.** DataCamp's tutor treats a learner's mistakes one at a time. Engram uses a separate grader that never sees the lesson, and tests it against answers marked by hand. Teacher-to-Skills marks where a teacher's material is too thin to support a rule. Owner: the authors of the tutor prompt and of the capture pipeline. Next step: add each practice to the backlog of the part it concerns. Confidence: low - each builder describes the practice first-hand, and outcome data is still to come.

The audience, the names, and whether the gate code should also serve other assistants are the team's to decide. On the third choice, Tutor MCP shows one answer: it offers its gate code through the Model Context Protocol, a standard way for assistants to call outside tools, so any chatbot can act as the tutor. Tutors already win use inside the general chatbots. Universal Primer, a tutor that one developer packaged inside ChatGPT as a custom GPT, shows more than 4 million conversations, and McGraw Hill and Quizlet now plug their content into general chatbots ([GPT Store ranking][s6-gptstore-rank]; [Investing.com][s1-mh-mcp]; [Quizlet][s1-qz-gpt]). TeachPad's design runs inside its own app, which lets the tutor control the page and asks the learner for one more install. We lean toward keeping TeachPad as its own app and revisiting the question after the first release with a working tutor. Confidence: low - neither approach has results yet.

## What to watch: a large lab is likely to ship a tutor-written page within a year

It is likely that within a year a large lab ships a lesson page that its tutor writes turn by turn during the lesson, beside a chat. Our confidence in that judgment is medium: the pieces, such as pages built from a conversation, quizzes and interactive visuals, shipped separately during 2026, and the labs have said little about their plans. It is unlikely that a large lab adds, within a year, a gate that the learner cannot skip. Our confidence there is low. Every learning mode so far leaves the pace to the learner, and the controls that OpenAI, Google and Microsoft gave schools in 2026 lock a student into the learning mode without setting the pace. Even so, a lab could add a gate for schools quickly.

Each event below would change how TeachPad compares with other products, and each is worth a check every quarter:

- DeepTutor joins its generated book and its gated tutor chat, which are separate today, in one page.
- Gemini study notebooks let a teacher or an expert assign a lesson plan to students.
- A company that interviews experts, such as Napster Learn or BeOrigin, adds gates held in code.
- Handshake's free AI Skills Studio, a set of short lessons for job seekers built on Uplimit's technology, grows into a general tutor.
- Coursera ships Project Helix, its adaptive platform for employers.
- Brilliant lets its tutor Koji write lesson content.
- An independent trial of a model tutor for adults is published.

## Other products use the names "Pedagogue" and "living textbook"

At least three other products are named Pedagogue. The closest clash is pedagogue.io, a UK platform that trains school staff to use AI and includes an AI tutor. A partnership signed in March 2026 gives it a route to more than 1,200 independent schools ([pedagogue.io][s4-pg1]). The second, pedagogue.app, is a social network for educators from the publisher of The Edvocate. The same owners offer Entelechy, which they call "an AI-powered personal tutor" ([pedagogue.app launch post][s4-pg4]; [Entelechy post][s4-pg5]). The third is a GitHub project created on 10 November 2026, described as "A tutor agent that walks you through a codebase in the browser" ([GitHub][x-gh-pedagogue]).

Until it became TeachPad on 23 November 2026, this project was called Pedagogue and our own documents described it as a "living textbook", and that phrase is in wide use. Enki, a product for software engineers from a firm named Caffinix, describes an editable "living textbook" ([Caffinix][s4-en1]). Lokalingo sells "The Living Textbook" for language teaching, and the University of Twente publishes a "Living Textbook" that joins a wiki to a concept map ([Lokalingo][s4-lk1]; [University of Twente][s4-lb1]). A hospital project in Bern, a hackathon project and several GitHub projects use the phrase too, and DeepTutor calls its generated books "living books" ([Inselspital Bern][s4-xm1]; [Devpost][s4-dv1]). The phrase also has an older meaning: a book written by people that keeps being updated.

## The search read public pages on one day, and nobody tried the products

- **AI research agents searched public sources on 22 November 2026.** Seven ran in parallel. Six covered product groups: established adaptive platforms, tutors from AI labs and large platforms, startups, textbooks, learning for professionals and developers, and open-source projects. The seventh gathered market data. Each wrote an evidence file with a source list that says what was read: the full page, an excerpt, or a secondary summary.
- **The six product searches covered about 275 products.** They wrote about 120 full records, each a page of facts on one product, and 180 one-line entries. About two dozen products appear in two searches.
- **Each search scored its own products against two shared examples.** All six searches were given two closeness scores to calibrate against: 5 of 12 for ALEKS, and 6 of 12 for ChatGPT study mode as it was at launch. Eight products have full records from two searches, and their two totals matched or differed by one point. Where two totals differ, the score in Table 1 comes from the search that covered the product's own group.
- **The scores reflect public descriptions.** They come from makers' pages, source code, press and reviews. No product was tried by hand, so a maker's description can flatter the product. Corporate tools earn easy points for accepting any uploaded document and for serving adults, so compare products part by part before trusting a total.
- **AI checkers reopened the sources and corrected the first draft.** Six of them compared about 540 statements with the evidence files and reopened about 325 of them on the live pages. They found the main counts of users and funding figures correct and all but two quotations word for word. They also marked about 140 statements as wrong, cited to the wrong page, unsupported or overstated, and each was corrected or cut. The corrections fixed three main errors. The draft had dated the first tutor-written boards to 2026, and they shipped in late 2025. It had treated the large share of venture funding that went to workforce training in one quarter as a trend. And it had described parts of TeachPad's plan as already built.
- **Most user counts are the makers' own.** Filings back the figures for Duolingo, Coursera and McGraw Hill. Some sources disagree, on counts that state no period: ELSA Speak appears with 50 million and 100 million users, Codecademy with 50 million and 85 million learners, and DataCamp with 18 million and 21 million learners.
- **Small products and products outside English are the likeliest gaps.** Products from China and South Korea were read mostly through press summaries. The field changed every month in 2026, so several facts here will be out of date within months.
- **Each evidence file also lists what its search could not confirm**, such as user counts, prices and launch dates that have a single weak source.

## Sources

Every source in this list was read on 22 November 2026, and each link in the text points to one of them. The list is grouped by the search that found the source.

**Established adaptive platforms**

- <https://siviq.co/nonprofits/organizations/261544963/khan-academy-inc/returns/202640439349300549/2025>
- <https://blog.brilliant.org/a-world-class-tutor-in-every-home/>
- <https://www.cambiumlearning.com/user_area/content_media/raw/CAM-Impact-Report-2025-2026.pdf>
- <https://schoolaiatlas.com/archive/khan-academy-founding-and-scale/>
- <https://about.zearn.org/privacy>
- <https://www.gettingsmart.com/podcast/richard-carranza-on-how-ai-and-arts-are-transforming-education/>
- <https://www.nbcnews.com/news/education/iready-school-software-faces-parent-teacher-student-fury-rcna342850>
- <https://www.globenewswire.com/news-release/2025/04/30/3071105/0/en/duolingo-launches-148-new-language-courses.html>
- <https://sunnyray.com/show/ep/r0gwwi3daps/>
- <https://www.cengagegroup.com/news/press-releases/2025/cengage-student-assistant-expansion-1m-students/>
- <https://www.macmillanlearning.com/us/explore/higher-ed/post/how-the-ai-tutor-drives-student-success>
- <https://aitoolsofthetrade.com/reviews/quizlet>
- <https://help.area9lyceum.com/area9-mindflow-rapid/does-area9-use-generative-ai/>
- <https://thirdspacelearning.com/maths-tutoring/ai-maths-tutor/>
- <https://www.zybooks.com/whats-new-for-fall-2026/>
- <https://ng.investing.com/news/company-news/mcgraw-hill-q1-fy27-slides-recurring-revenue-at-77-margins-expand-93CH-2658825>
- <https://www.prnewswire.com/news-releases/quizlet-launches-as-native-app-in-chatgpt-to-transform-ai-powered-learning-302710329.html>

**Tutors from AI labs and large platforms**

- <https://blog.google/innovation-and-ai/products/gemini-app/gemini-study-notebooks/>
- <https://www.linkedin.com/posts/google_today-were-introducing-study-notebooks-in-activity-7475958939583692800-BzUe>
- <https://www.prnewswire.com/news-releases/gauth-launches-unlimited-digital-canvas-gives-students-one-continuous-whiteboard-to-visualize-explore-and-understand-complex-topics-302891205.html>
- <https://openai.com/index/chatgpt-study-mode/>
- <https://help.openai.com/en/articles/6825453-chatgpt-release-notes>
- <https://www.anthropic.com/news/introducing-claude-for-education>
- <https://blog.google/products-and-platforms/products/education/guided-learning/>
- <https://help.openai.com/en/articles/11780217-using-study-mode-in-chatgpt>
- <https://news.qq.com/rain/a/20261020A06WHG00>
- <https://visibianhq.com/research/ai-users/>
- <https://techcommunity.microsoft.com/blog/educationblog/study-and-learn-agent-your-study-coach-built-for-learning/4520922>
- <https://www.perplexity.ai/changelog/what-we-shipped-september-5th>
- <https://blog.google/innovation-and-ai/products/gemini-app/student-offer-google-ai/>
- <https://glasp.co/articles/claude-vs-gemini-for-studying>
- <https://www.engadget.com/2321236/how-to-use-gemini-notebook-to-turn-your-research-into-an-interactive-site/>
- <https://claude.com/resources/articles/claude-builds-visuals>
- <https://support.microsoft.com/en-us/education/copilot/lock-copilot-chat-study-learn>
- <https://workspaceupdates.googleblog.com/2026/11/educators-can-now-add-guided-learning-to-assignments-in-Google-Classroom-to-help-students-get-extra-support.html>
- <https://glasp.co/articles/ai-study-modes-compared>
- <https://openai.com/index/understanding-ai-and-learning-outcomes/>
- <https://www.fab-ai.org/resources/case-study/evaluating-gemini-guided-learning-in-sierra-leone>
- <https://investors.duolingo.com/static-files/e1ee336e-b024-486f-875c-afd722653067>
- <https://exa.ai/library/markets/stocks/DUOL/earnings/FY2026/Q3/transcript>
- <https://ppc.land/microsoft-announces-12-features-for-copilot-in-ai-companion-strategy/>
- <https://research.google/blog/learn-your-way-reimagining-textbooks-with-generative-ai/>
- <https://blog.khanacademy.org/learning-in-the-open-what-ai-is-and-isnt-changing/>
- <https://blog.khanacademy.org/built-in-the-open-how-pilot-districts-shaped-the-reimagined-khan-academy/>
- <https://www.khanmigo.ai/pricing>
- <https://whizi.io/resources/chatgpt-edu/>

**Startups built on models**

- <https://www.meridianlearn.ai/>
- <https://www.linkedin.com/posts/meridian-learn_meridian-learn-activity-7449232243992137728-q-_S>
- <https://techcrunch.com/2025/12/10/oboe-raises-16-million-from-a16z-for-its-ai-powered-course-generation-platform/>
- <https://www.linkedin.com/posts/nir-zicherman-172a461a_weve-pivoted-oboe-the-first-version-which-activity-7523452301307457536-nHd6>
- <https://linkedin.com/company/meridian-learn>
- <https://www.linkedin.com/posts/leslie-jiang-05y117_introducing-hyperknow-a-new-way-to-learn-activity-7511605994272002049-uzd_>
- <https://techfundingnews.com/mindmax-3-3m-pre-seed-ai-tutor-memrise/>
- <https://www.forbes.com/sites/ianshepherd/2026/08/31/gauth-uses-ai-to-let-every-student-create-a-personalized-course/>
- <https://hunted.space/product/meridian-learn>
- <https://thegenaimagazine.com/hyperknow-ai-professor-funding-review>
- <https://linkedin.com/company/mindmaxlabs>
- <https://www.ynetnews.com/tech-and-digital/article/rkuxwd3cgx>
- <https://praktika.ai/>
- <https://www.speak.com/>
- <https://techcrunch.com/2026/04/15/ai-learning-app-gizmo-levels-up-with-13m-users-and-a-22m-investment/>
- <https://www.turbo.ai/blog/july-2026-announcement-10-million-users>
- <https://www.inc.com/diana-bocco/student-founders-entrepreneurs-ai-startup-venture-capital-funding/91380311>
- <https://aiolum.com/story/wondering/>
- <https://campus.edu/blog/press-release/campus-acquires-sizzle-ai>
- <https://betakit.com/canadian-founded-opennote-acquired-by-san-francisco-based-reducto/>
- <https://www.opennote.com/>
- <https://www.saashub.com/learn-xyz-alternatives>
- <https://www.techi.com/company/eureka-labs/>
- <https://dasnuminous.com/blog/education-after-agi-is-fun>
- <https://www.ycombinator.com/companies/miyagi-labs>
- <https://anara.com/blog/the-beginning-of-a-new-story>
- <https://fundraisingfox.com/companies/kinnu>
- <https://news.ycombinator.com/item?id=48981136>
- <https://understandtutor.com/blog/synthesis-tutor-review>
- <https://www.alphaanywhere.co/academics>
- <https://www.wgbh.org/news/local/2026-11-16/heres-how-a-typical-day-at-alpha-schools-an-ai-powered-institution-looks-for-students>
- <https://www.medlyai.com/uk/exams/blog/independent_rct_gcse_science>
- <https://scale.stanford.edu/sites/default/files/The%20Evidence%20Base%20on%20AI%20in%20K-12%20November%202026%20Review.pdf>
- <https://fortune.com/2026/11/14/alpha-school-ai-students-tuition-learning-goals/>

**Textbooks and their predecessors**

- <https://pedagogue.io/blog/pedagogue-partners-socs-ai-standard/>
- <https://www.caffinix.com/products/enki>
- <https://axiom.co/customers/brilliant>
- <https://kudu.com/>
- <https://brilliant.org/help/features/how-does-koji-work/>
- <https://brilliant.org/assessments/>
- <https://brilliant.org/help/pricing-and-plans/what-is-brilliant-premium-and-how-do-i-subscribe/>
- <https://kudu.com/ai>
- <https://www.ck12.org/pages/teacher-assistant/>
- <https://plc.pearson.com/sites/pearson-corp/files/2026-02/ai-active-reading-feb-2026.pdf>
- <https://stockanalysis.com/stocks/mh/transcripts/752312-investor-day-2026/>
- <https://help.cengage.com/mindtap/mt-student/common/student-assistant.html>
- <https://get.vitalsource.com/press/vitalsource-launches-bookshelf-plus>
- <https://teach.its.uiowa.edu/news/2026/02/top-hat-tools-spring-2026-icon-integration-and-ace-ai>
- <https://investors.mheducation.com/news/news-details/2026/McGraw-Hill-Inc--Reports-Strong-Fiscal-First-Quarter-2027-Results-Exceeding-Expectations/default.aspx>
- <https://www.linkedin.com/posts/macmillan-learning_achieve-aistudenttutor-aiineducation-activity-7501283694050168832-jc-4>
- <https://dl.acm.org/doi/pdf/10.1145/3017680.3017732>
- <https://proteus.aimath.org/?page_id=162>
- <https://openstax.org/blog/introducing-openstax-assignable>
- <https://iapqb.org/research/012-korea-ai-digital-textbooks.html>
- <https://restofworld.org/2025/south-korea-ai-textbook/>
- <https://numinous.productions/ttft/>
- <https://www.executeprogram.com/>
- <https://github.com/ayushozha/hituto>
- <https://pedagogue.app/introducing-pedagogue-a-new-social-media-site-for-educators/>
- <https://pedagogue.app/introducing-entelechy-an-ai-powered-personal-tutor/>
- <https://thelivingtextbook.lokalingo.com/en>
- <https://www.utwente.nl/en/itc/facilities/living-textbook/>
- <https://neurochirurgie.insel.ch/en/news/detail/from-knowledge-consumption-to-a-knowledge-twin-bern-based-ai-project-221>
- <https://devpost.com/software/aegis-mission-control>

**Learning for professionals and developers, and expert capture**

- <https://read.getsuperintel.com/p/datacamp-and-the-40-million-tutoring-bill>
- <https://chainofthought.show/podcast/84-this-is-the-worst-ai-tutor-well-ever-have-datacamp/>
- <https://joinhandshake.com/blog/our-team/handshake-acquires-uplimit-to-build-a-global-ai-learning-platform/>
- <https://exa.ai/library/organization/yt4jksfkfgn>
- <https://www.datacamp.com/blog/datacamp-has-acquired-optima>
- <https://support.datacamp.com/en-us/articles/17598794-introducing-the-ai-tutor-experience>
- <https://uplimit.com/>
- <https://www.prnewswire.com/news-releases/handshake-launches-first-ai-skills-studio-to-prepare-job-seekers-for-the-ai-economy-302885733.html>
- <https://www.novali.ai/>
- <https://investor.coursera.com/news/news-details/2026/Coursera-Completes-Combination-with-Udemy-to-Build-the-Worlds-Most-Comprehensive-Skills-Platform/>
- <https://blog.coursera.org/announcing-project-helix/>
- <https://newsroom.workday.com/2025-09-16-Workday-Signs-Definitive-Agreement-to-Acquire-Sana>
- <https://help.sana.ai/en/articles/316752-course-tutor-ai-support-within-courses>
- <https://amplifire.com/resource/meet-amplifires-ai-instructor/>
- <https://area9lyceum.com/>
- <https://www.boot.dev/lessons/142c8a73-5ede-49a6-9460-563890646023>
- <https://codesignal.com/cosmo/>
- <https://www.boot.dev/blog/jobs/job-data-on-learning-to-code-2026>
- <https://prachub.com/resources/codecrafters-review-2026>
- <https://code.claude.com/docs/en/output-styles>
- <https://forum.cursor.com/t/study-mode-in-chat-agent/175882>
- <https://arxiv.org/html/2601.20245v1>
- <https://beorigin.tech/>
- <https://www.napster.com/blog/napster-learn-digital-twins-consent>
- <https://www.omnora.com/en/retirement-knowledge-transfer>
- <https://www.neovation.com/platforms/joysuite>
- <https://www.delphi.co/blog/digital-identity-protection>
- <https://www.engadget.com/ai/masterclass-on-call-gives-you-on-demand-access-to-ai-facsimiles-of-its-experts-215022938.html>
- <https://linkedin.com/company/beorigintech>
- <https://www.omnora.com/en/company>
- <https://investor.workday.com/news-and-events/press-releases/news-details/2026/Workday-Learning-Powered-by-Sana-Now-Generally-Available-as-an-AI-Native-Learning-Experience-Built-on-Workdays-Trusted-Data/default.aspx>
- <https://newsroom.workday.com/2025-11-04-Workday-Completes-Acquisition-of-Sana>

**Open-source projects**

- <https://github.com/HKUDS/DeepTutor>
- <https://github.com/ArnaudGuiovanna/tutor-mcp>
- <https://github.com/fanfanfanfanfan626/mastery-tutor/blob/main/docs/architecture.md>
- <https://github.com/fanfanfanfanfan626/mastery-tutor>
- <https://github.com/nagisanzenin/engram>
- <https://github.com/JuneYaooo/teacher-to-skills>
- <https://github.com/HKUDS/DeepTutor/blob/main/deeptutor/capabilities/mastery/capability.py>
- <https://github.com/HKUDS/DeepTutor/blob/main/deeptutor/learning/policy.py>
- <https://github.com/HKUDS/DeepTutor/releases/tag/v1.4.5>
- <https://arxiv.org/abs/2604.26962>
- <https://github.com/HKUDS/DeepTutor/issues/1973>
- <https://github.com/ArnaudGuiovanna/tutor-mcp/tree/main/client>
- <https://github.com>
- <https://github.com/wildcat430524/StepsToGreat>
- <https://github.com/YusenZhang0601/tutor>
- <https://github.com/JushBJJ/Mr.-Ranedeer-AI-Tutor>
- <https://github.com/THU-MAIC/OpenMAIC>
- <https://github.com/topics/ai-tutor>
- <https://summerofcode.withgoogle.com/programs/2025/organizations/learning-equality>
- <https://github.com/munitkala/selfsemester>
- <https://pypi.org/project/fsrs/>
- <https://github.com/tianshuo/trainingllm>
- <https://github.com/rschmucker/ruffle-and-riley>
- <https://gptstore.ai/gpts/rank>

**Market data**

- <https://blog.google/innovation-and-ai/products/gemini-app/one-billion-monthly-users/>
- <https://openai.com/index/devday-2026-recap/>
- <https://www.nber.org/system/files/working_papers/w34255/w34255.pdf>
- <https://www.linkedin.com/posts/khan-academy_client-challenge-activity-7521601800999735297-hX09>
- <https://quizlet.com/ads/Audiences>
- <https://report-ai.org/indexes/state-of-ai/state-of-ai-usage/>
- <https://www.reuters.com/technology/chatgpt-app-hits-1-billion-monthly-active-users-record-time-data-shows-2026-06-02/>
- <https://blog.google/innovation-and-ai/products/gemini-notebook/notebooklm-gemini-notebook/>
- <https://www.sec.gov/Archives/edgar/data/1951070/000195107026000022/mh-20260331.htm>
- <https://linkedin.com/company/prodigyedu>
- <https://www.ixl.com/materials/us/press/Press_release__IXL_Hits_200B_Questions_Answered.pdf>
- <https://www.curriculumassociates.com/about/press-releases/2026/9/i-ready-2026-2027-updates>
- <https://www.sec.gov/Archives/edgar/data/1651562/000165156226000068/cour-20260930xshareholde.htm>
- <https://investor.coursera.com/news/news-details/2026/Coursera-Reports-Third-Quarter-2026-Financial-Results/default.aspx>
- <https://www.bittime.com/en/blog/apa-itu-gauth-ai>
- <https://outearn.app/app/google/com.education.android.h.intelligence>
- <https://outearn.app/app/apple/gauth-ai-study-companion-1542571008>
- <https://openai.com/index/teens-learn-and-plan/>
- <https://www.pewresearch.org/short-reads/2025/06/25/34-of-us-adults-have-used-chatgpt-about-double-the-share-in-2023/>
- <https://www.pewresearch.org/internet/2026/02/24/how-teens-use-and-view-ai/>
- <https://www.hepi.ac.uk/reports/student-generative-ai-survey-2026/>
- <https://survey.stackoverflow.co/2026/community/data/learn-code-how>
- <https://webassets.zearn.org/Implementation/Zearn_factsheet.pdf>
- <https://elsaspeak.com/en/enterprise/blog/elsa-s-ceo-vu-van-featured-in-exame-the-founder-story-behind-elsa-s-global-expansion>
- <https://www.newsis.com/view/NISX20261020_0003833773>
- <https://www.chalkbeat.org/2026/04/09/sal-khan-reflects-on-ai-in-schools-and-khanmigo/>
- <https://www.nber.org/system/files/working_papers/w35620/w35620.pdf>
- <https://tihupe.ee/wp-content/uploads/2026/08/AI-Leap-First-Annual-Review-202526.pdf>
- <https://www.holoniq.com/notes/2022-edtech-vc-funding-totals-10-6b-down-from-20-8b-in-2021>
- <https://www.holoniq.com/notes/edtech-vc-reached-2-4b-for-2024-representing-the-lowest-level-of-investment-in-a-decade>
- <https://www.holoniq.com/notes/education-funding-q3-2026-venture-pulls-back-buyers-lean-in>
- <https://www.lingobright.com/statistics/edtech-venture-funding-statistics-and-largest-rounds/>
- <https://www.speak.com/blog/series-c>
- <https://www.edtechupdate.com/knewton/pearson/>
- <https://stackoverflow.blog/2026/10/01/a-look-back-before-we-look-forward-a-developer-survey-retrospective/>

**Added during the fact check**

- <https://scholar.harvard.edu/files/mkraft/files/kraft_2019_effect_sizes.pdf>
- <https://github.com/aahoughton/pedagogue>

*2026-11-22 15:16 - claude-fable-5.2*

[s6-deeptutor-repo]: https://github.com/HKUDS/DeepTutor
[s2-g-studynotebooks]: https://blog.google/innovation-and-ai/products/gemini-app/gemini-study-notebooks/
[s2-g-studynotebooksli]: https://www.linkedin.com/posts/google_today-were-introducing-study-notebooks-in-activity-7475958939583692800-BzUe
[s7-goog-1b]: https://blog.google/innovation-and-ai/products/gemini-app/one-billion-monthly-users/
[s5-dc3]: https://read.getsuperintel.com/p/datacamp-and-the-40-million-tutoring-bill
[s5-dc4]: https://chainofthought.show/podcast/84-this-is-the-worst-ai-tutor-well-ever-have-datacamp/
[s6-tutormcp-repo]: https://github.com/ArnaudGuiovanna/tutor-mcp
[s6-masterytutor-arch]: https://github.com/fanfanfanfanfan626/mastery-tutor/blob/main/docs/architecture.md
[s3-mer1]: https://www.meridianlearn.ai/
[s3-mer2]: https://www.linkedin.com/posts/meridian-learn_meridian-learn-activity-7449232243992137728-q-_S
[s7-oai-dev]: https://openai.com/index/devday-2026-recap/
[s7-nber1]: https://www.nber.org/system/files/working_papers/w34255/w34255.pdf
[s7-duol1]: https://investors.duolingo.com/static-files/e1ee336e-b024-486f-875c-afd722653067
[s7-ka-li]: https://www.linkedin.com/posts/khan-academy_client-challenge-activity-7521601800999735297-hX09
[s1-ka-990]: https://siviq.co/nonprofits/organizations/261544963/khan-academy-inc/returns/202640439349300549/2025
[s7-qz2]: https://quizlet.com/ads/Audiences
[s7-ka-blog]: https://blog.khanacademy.org/learning-in-the-open-what-ai-is-and-isnt-changing/
[s3-oboe2]: https://techcrunch.com/2025/12/10/oboe-raises-16-million-from-a16z-for-its-ai-powered-course-generation-platform/
[s3-oboe4]: https://www.linkedin.com/posts/nir-zicherman-172a461a_weve-pivoted-oboe-the-first-version-which-activity-7523452301307457536-nHd6
[s4-pg1]: https://pedagogue.io/blog/pedagogue-partners-socs-ai-standard/
[s4-en1]: https://www.caffinix.com/products/enki
[x-kraft]: https://scholar.harvard.edu/files/mkraft/files/kraft_2019_effect_sizes.pdf
[s6-masterytutor-repo]: https://github.com/fanfanfanfanfan626/mastery-tutor
[s6-engram-repo]: https://github.com/nagisanzenin/engram
[s3-mer4]: https://linkedin.com/company/meridian-learn
[s3-hyp2]: https://www.linkedin.com/posts/leslie-jiang-05y117_introducing-hyperknow-a-new-way-to-learn-activity-7511605994272002049-uzd_
[s5-up5]: https://joinhandshake.com/blog/our-team/handshake-acquires-uplimit-to-build-a-global-ai-learning-platform/
[s3-mmx2]: https://techfundingnews.com/mindmax-3-3m-pre-seed-ai-tutor-memrise/
[s6-t2s-repo]: https://github.com/JuneYaooo/teacher-to-skills
[s5-n2]: https://exa.ai/library/organization/yt4jksfkfgn
[s4-br7]: https://axiom.co/customers/brilliant
[s3-gau2]: https://www.forbes.com/sites/ianshepherd/2026/08/31/gauth-uses-ai-to-let-every-student-create-a-personalized-course/
[s4-ku1]: https://kudu.com/
[s6-deeptutor-cap]: https://github.com/HKUDS/DeepTutor/blob/main/deeptutor/capabilities/mastery/capability.py
[s6-deeptutor-policy]: https://github.com/HKUDS/DeepTutor/blob/main/deeptutor/learning/policy.py
[s6-deeptutor-rel145]: https://github.com/HKUDS/DeepTutor/releases/tag/v1.4.5
[s6-deeptutor-paper]: https://arxiv.org/abs/2604.26962
[s6-deeptutor-issue1973]: https://github.com/HKUDS/DeepTutor/issues/1973
[s5-dc1]: https://www.datacamp.com/blog/datacamp-has-acquired-optima
[s5-dc2]: https://support.datacamp.com/en-us/articles/17598794-introducing-the-ai-tutor-experience
[s6-tutormcp-client]: https://github.com/ArnaudGuiovanna/tutor-mcp/tree/main/client
[s3-mer3]: https://hunted.space/product/meridian-learn
[s3-hyp3]: https://thegenaimagazine.com/hyperknow-ai-professor-funding-review
[s3-mmx3]: https://linkedin.com/company/mindmaxlabs
[s2-prn-gauth]: https://www.prnewswire.com/news-releases/gauth-launches-unlimited-digital-canvas-gives-students-one-continuous-whiteboard-to-visualize-explore-and-understand-complex-topics-302891205.html
[s1-br-blog]: https://blog.brilliant.org/a-world-class-tutor-in-every-home/
[s4-br2]: https://brilliant.org/help/features/how-does-koji-work/
[s4-br3]: https://brilliant.org/assessments/
[s4-br11]: https://brilliant.org/help/pricing-and-plans/what-is-brilliant-premium-and-how-do-i-subscribe/
[s5-up2]: https://uplimit.com/
[s5-up8]: https://www.prnewswire.com/news-releases/handshake-launches-first-ai-skills-studio-to-prepare-job-seekers-for-the-ai-economy-302885733.html
[s4-ku2]: https://kudu.com/ai
[s5-n1]: https://www.novali.ai/
[s2-oai-studylaunch]: https://openai.com/index/chatgpt-study-mode/
[s2-oai-relnotes]: https://help.openai.com/en/articles/6825453-chatgpt-release-notes
[s2-anth-edulaunch]: https://www.anthropic.com/news/introducing-claude-for-education
[s2-g-guidedlaunch]: https://blog.google/products-and-platforms/products/education/guided-learning/
[s2-oai-studyhelp]: https://help.openai.com/en/articles/11780217-using-study-mode-in-chatgpt
[s6-gh-api]: https://github.com
[s6-stepstogreat-repo]: https://github.com/wildcat430524/StepsToGreat
[s6-yusentutor-repo]: https://github.com/YusenZhang0601/tutor
[s6-ranedeer-repo]: https://github.com/JushBJJ/Mr.-Ranedeer-AI-Tutor
[s2-qm-sep2026]: https://news.qq.com/rain/a/20261020A06WHG00
[s7-rai1]: https://report-ai.org/indexes/state-of-ai/state-of-ai-usage/
[s7-claude1]: https://www.reuters.com/technology/chatgpt-app-hits-1-billion-monthly-active-users-record-time-data-shows-2026-06-02/
[s7-nblm1]: https://blog.google/innovation-and-ai/products/gemini-notebook/notebooklm-gemini-notebook/
[s7-mh3]: https://www.sec.gov/Archives/edgar/data/1951070/000195107026000022/mh-20260331.htm
[s1-cam-imp]: https://www.cambiumlearning.com/user_area/content_media/raw/CAM-Impact-Report-2025-2026.pdf
[s7-prod1]: https://linkedin.com/company/prodigyedu
[s7-ixl1]: https://www.ixl.com/materials/us/press/Press_release__IXL_Hits_200B_Questions_Answered.pdf
[s7-iready1]: https://www.curriculumassociates.com/about/press-releases/2026/9/i-ready-2026-2027-updates
[s2-visibian]: https://visibianhq.com/research/ai-users/
[s7-cour4]: https://www.sec.gov/Archives/edgar/data/1651562/000165156226000068/cour-20260930xshareholde.htm
[s7-cour3]: https://investor.coursera.com/news/news-details/2026/Coursera-Reports-Third-Quarter-2026-Financial-Results/default.aspx
[s4-ck4]: https://www.ck12.org/pages/teacher-assistant/
[s1-ka-atlas]: https://schoolaiatlas.com/archive/khan-academy-founding-and-scale/
[s7-gauth1]: https://www.bittime.com/en/blog/apa-itu-gauth-ai
[s7-out3]: https://outearn.app/app/google/com.education.android.h.intelligence
[s7-out2]: https://outearn.app/app/apple/gauth-ai-study-companion-1542571008
[s7-oai-teens]: https://openai.com/index/teens-learn-and-plan/
[s7-pew3]: https://www.pewresearch.org/short-reads/2025/06/25/34-of-us-adults-have-used-chatgpt-about-double-the-share-in-2023/
[s7-pew1]: https://www.pewresearch.org/internet/2026/02/24/how-teens-use-and-view-ai/
[s7-hepi1]: https://www.hepi.ac.uk/reports/student-generative-ai-survey-2026/
[s7-so1]: https://survey.stackoverflow.co/2026/community/data/learn-code-how
[s7-zearn1]: https://webassets.zearn.org/Implementation/Zearn_factsheet.pdf
[s1-zearn-priv]: https://about.zearn.org/privacy
[s1-ixl-gs]: https://www.gettingsmart.com/podcast/richard-carranza-on-how-ai-and-arts-are-transforming-education/
[s1-ca-nbc]: https://www.nbcnews.com/news/education/iready-school-software-faces-parent-teacher-student-fury-rcna342850
[s3-els1]: https://www.ynetnews.com/tech-and-digital/article/rkuxwd3cgx
[s7-elsa2]: https://elsaspeak.com/en/enterprise/blog/elsa-s-ceo-vu-van-featured-in-exame-the-founder-story-behind-elsa-s-global-expansion
[s3-pra2]: https://praktika.ai/
[s3-spk1]: https://www.speak.com/
[s3-giz1]: https://techcrunch.com/2026/04/15/ai-learning-app-gizmo-levels-up-with-13m-users-and-a-22m-investment/
[s3-tur1]: https://www.turbo.ai/blog/july-2026-announcement-10-million-users
[s3-tur3]: https://www.inc.com/diana-bocco/student-founders-entrepreneurs-ai-startup-venture-capital-funding/91380311
[s3-won3]: https://aiolum.com/story/wondering/
[s6-openmaic-repo]: https://github.com/THU-MAIC/OpenMAIC
[s6-gh-topic-ai-tutor]: https://github.com/topics/ai-tutor
[s6-le-gsoc]: https://summerofcode.withgoogle.com/programs/2025/organizations/learning-equality
[s2-ms-slga]: https://techcommunity.microsoft.com/blog/educationblog/study-and-learn-agent-your-study-coach-built-for-learning/4520922
[s2-pplx-sep2025]: https://www.perplexity.ai/changelog/what-we-shipped-september-5th
[s2-g-studentoffer]: https://blog.google/innovation-and-ai/products/gemini-app/student-offer-google-ai/
[s2-glasp-claudegemini]: https://glasp.co/articles/claude-vs-gemini-for-studying
[s2-engadget-site]: https://www.engadget.com/2321236/how-to-use-gemini-notebook-to-turn-your-research-into-an-interactive-site/
[s2-claude-visuals]: https://claude.com/resources/articles/claude-builds-visuals
[s2-ms-sllock]: https://support.microsoft.com/en-us/education/copilot/lock-copilot-chat-study-learn
[s2-g-workspacegl]: https://workspaceupdates.googleblog.com/2026/11/educators-can-now-add-guided-learning-to-assignments-in-Google-Classroom-to-help-students-get-extra-support.html
[s2-glasp-modes]: https://glasp.co/articles/ai-study-modes-compared
[s2-oai-outcomes]: https://openai.com/index/understanding-ai-and-learning-outcomes/
[s2-g-sierraleone]: https://www.fab-ai.org/resources/case-study/evaluating-gemini-guided-learning-in-sierra-leone
[s1-duo-148]: https://www.globenewswire.com/news-release/2025/04/30/3071105/0/en/duolingo-launches-148-new-language-courses.html
[s1-ma-sunny]: https://sunnyray.com/show/ep/r0gwwi3daps/
[s1-cg-2025]: https://www.cengagegroup.com/news/press-releases/2025/cengage-student-assistant-expansion-1m-students/
[s1-mac-study]: https://www.macmillanlearning.com/us/explore/higher-ed/post/how-the-ai-tutor-drives-student-success
[s1-qz-rev]: https://aitoolsofthetrade.com/reviews/quizlet
[s2-duo-q3letter]: https://investors.duolingo.com/static-files/e1ee336e-b024-486f-875c-afd722653067
[s2-duo-q3call]: https://exa.ai/library/markets/stocks/DUOL/earnings/FY2026/Q3/transcript
[s1-a9-genai]: https://help.area9lyceum.com/area9-mindflow-rapid/does-area9-use-generative-ai/
[s3-siz1]: https://campus.edu/blog/press-release/campus-acquires-sizzle-ai
[s3-opn3]: https://betakit.com/canadian-founded-opennote-acquired-by-san-francisco-based-reducto/
[s3-opn2]: https://www.opennote.com/
[s3-lxz2]: https://www.saashub.com/learn-xyz-alternatives
[s3-eur4]: https://www.techi.com/company/eureka-labs/
[s3-eur3]: https://dasnuminous.com/blog/education-after-agi-is-fun
[s3-miy1]: https://www.ycombinator.com/companies/miyagi-labs
[s3-ana1]: https://anara.com/blog/the-beginning-of-a-new-story
[s3-kin1]: https://fundraisingfox.com/companies/kinnu
[s3-bmy1]: https://news.ycombinator.com/item?id=48981136
[s3-syn5]: https://understandtutor.com/blog/synthesis-tutor-review
[s3-alp9]: https://www.alphaanywhere.co/academics
[s3-alp11]: https://www.wgbh.org/news/local/2026-11-16/heres-how-a-typical-day-at-alpha-schools-an-ai-powered-institution-looks-for-students
[s3-med3]: https://www.medlyai.com/uk/exams/blog/independent_rct_gcse_science
[s4-pe1]: https://plc.pearson.com/sites/pearson-corp/files/2026-02/ai-active-reading-feb-2026.pdf
[s4-mh2]: https://stockanalysis.com/stocks/mh/transcripts/752312-investor-day-2026/
[s4-ce3]: https://help.cengage.com/mindtap/mt-student/common/student-assistant.html
[s4-ma1]: https://www.macmillanlearning.com/us/explore/higher-ed/post/how-the-ai-tutor-drives-student-success
[s4-vs2]: https://get.vitalsource.com/press/vitalsource-launches-bookshelf-plus
[s4-th6]: https://teach.its.uiowa.edu/news/2026/02/top-hat-tools-spring-2026-icon-integration-and-ace-ai
[s4-mh4]: https://investors.mheducation.com/news/news-details/2026/McGraw-Hill-Inc--Reports-Strong-Fiscal-First-Quarter-2027-Results-Exceeding-Expectations/default.aspx
[s4-ma3]: https://www.linkedin.com/posts/macmillan-learning_achieve-aistudenttutor-aiineducation-activity-7501283694050168832-jc-4
[s4-zy5]: https://dl.acm.org/doi/pdf/10.1145/3017680.3017732
[s4-rs1]: https://proteus.aimath.org/?page_id=162
[s4-os7]: https://openstax.org/blog/introducing-openstax-assignable
[s4-kr4]: https://iapqb.org/research/012-korea-ai-digital-textbooks.html
[s7-kr1]: https://www.newsis.com/view/NISX20261020_0003833773
[s4-kr3]: https://restofworld.org/2025/south-korea-ai-textbook/
[s4-qc1]: https://numinous.productions/ttft/
[s4-ep1]: https://www.executeprogram.com/
[s5-c1]: https://investor.coursera.com/news/news-details/2026/Coursera-Completes-Combination-with-Udemy-to-Build-the-Worlds-Most-Comprehensive-Skills-Platform/
[s5-c8]: https://blog.coursera.org/announcing-project-helix/
[s5-w2]: https://newsroom.workday.com/2025-09-16-Workday-Signs-Definitive-Agreement-to-Acquire-Sana
[s5-w6]: https://help.sana.ai/en/articles/316752-course-tutor-ai-support-within-courses
[s5-am1]: https://amplifire.com/resource/meet-amplifires-ai-instructor/
[s5-ar5]: https://area9lyceum.com/
[s5-b7]: https://www.boot.dev/lessons/142c8a73-5ede-49a6-9460-563890646023
[s5-csg1]: https://codesignal.com/cosmo/
[s5-b3]: https://www.boot.dev/blog/jobs/job-data-on-learning-to-code-2026
[s5-cc1]: https://prachub.com/resources/codecrafters-review-2026
[s5-a1]: https://code.claude.com/docs/en/output-styles
[s5-a7]: https://forum.cursor.com/t/study-mode-in-chat-agent/175882
[s5-a9]: https://arxiv.org/html/2601.20245v1
[s5-be1]: https://beorigin.tech/
[s5-np1]: https://www.napster.com/blog/napster-learn-digital-twins-consent
[s5-o2]: https://www.omnora.com/en/retirement-knowledge-transfer
[s5-j1]: https://www.neovation.com/platforms/joysuite
[s5-d6]: https://www.delphi.co/blog/digital-identity-protection
[s5-m3]: https://www.engadget.com/ai/masterclass-on-call-gives-you-on-demand-access-to-ai-facsimiles-of-its-experts-215022938.html
[s6-selfsemester-repo]: https://github.com/munitkala/selfsemester
[s6-fsrs-pypi]: https://pypi.org/project/fsrs/
[s1-tsl-ai]: https://thirdspacelearning.com/maths-tutoring/ai-maths-tutor/
[s1-zy-new]: https://www.zybooks.com/whats-new-for-fall-2026/
[s6-trainingllm-repo]: https://github.com/tianshuo/trainingllm
[s6-ruffle-repo]: https://github.com/rschmucker/ruffle-and-riley
[s2-ppc-copilot]: https://ppc.land/microsoft-announces-12-features-for-copilot-in-ai-companion-strategy/
[s2-g-lyw-blog]: https://research.google/blog/learn-your-way-reimagining-textbooks-with-generative-ai/
[s4-ht1]: https://github.com/ayushozha/hituto
[s5-be2]: https://linkedin.com/company/beorigintech
[s5-o3]: https://www.omnora.com/en/company
[s5-w3]: https://investor.workday.com/news-and-events/press-releases/news-details/2026/Workday-Learning-Powered-by-Sana-Now-Generally-Available-as-an-AI-Native-Learning-Experience-Built-on-Workdays-Trusted-Data/default.aspx
[s5-w1]: https://newsroom.workday.com/2025-11-04-Workday-Completes-Acquisition-of-Sana
[s2-khan-open]: https://blog.khanacademy.org/learning-in-the-open-what-ai-is-and-isnt-changing/
[s7-chalk1]: https://www.chalkbeat.org/2026/04/09/sal-khan-reflects-on-ai-in-schools-and-khanmigo/
[s7-nber2]: https://www.nber.org/system/files/working_papers/w35620/w35620.pdf
[s2-khan-pilot]: https://blog.khanacademy.org/built-in-the-open-how-pilot-districts-shaped-the-reimagined-khan-academy/
[s7-ee1]: https://tihupe.ee/wp-content/uploads/2026/08/AI-Leap-First-Annual-Review-202526.pdf
[s2-khanmigo-pricing]: https://www.khanmigo.ai/pricing
[s2-whizi-edu]: https://whizi.io/resources/chatgpt-edu/
[s7-hiq-2022]: https://www.holoniq.com/notes/2022-edtech-vc-funding-totals-10-6b-down-from-20-8b-in-2021
[s7-hiq-2024]: https://www.holoniq.com/notes/edtech-vc-reached-2-4b-for-2024-representing-the-lowest-level-of-investment-in-a-decade
[s7-hiq-q3-26]: https://www.holoniq.com/notes/education-funding-q3-2026-venture-pulls-back-buyers-lean-in
[s7-lb1]: https://www.lingobright.com/statistics/edtech-venture-funding-statistics-and-largest-rounds/
[s7-spk2]: https://www.speak.com/blog/series-c
[s7-knew3]: https://www.edtechupdate.com/knewton/pearson/
[s3-mag7]: https://scale.stanford.edu/sites/default/files/The%20Evidence%20Base%20on%20AI%20in%20K-12%20November%202026%20Review.pdf
[s7-so3]: https://stackoverflow.blog/2026/10/01/a-look-back-before-we-look-forward-a-developer-survey-retrospective/
[s3-alp4]: https://fortune.com/2026/11/14/alpha-school-ai-students-tuition-learning-goals/
[s6-gptstore-rank]: https://gptstore.ai/gpts/rank
[s1-mh-mcp]: https://ng.investing.com/news/company-news/mcgraw-hill-q1-fy27-slides-recurring-revenue-at-77-margins-expand-93CH-2658825
[s1-qz-gpt]: https://www.prnewswire.com/news-releases/quizlet-launches-as-native-app-in-chatgpt-to-transform-ai-powered-learning-302710329.html
[s4-pg4]: https://pedagogue.app/introducing-pedagogue-a-new-social-media-site-for-educators/
[s4-pg5]: https://pedagogue.app/introducing-entelechy-an-ai-powered-personal-tutor/
[x-gh-pedagogue]: https://github.com/aahoughton/pedagogue
[s4-lk1]: https://thelivingtextbook.lokalingo.com/en
[s4-lb1]: https://www.utwente.nl/en/itc/facilities/living-textbook/
[s4-xm1]: https://neurochirurgie.insel.ch/en/news/detail/from-knowledge-consumption-to-a-knowledge-twin-bern-based-ai-project-221
[s4-dv1]: https://devpost.com/software/aegis-mission-control
