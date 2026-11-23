---
produced: 2026-11-22
title: "Automated lesson-plan generation and the 80 percent rule evidence review - AI drafts and a human expert finishes: quality of LLM-drafted lesson plans and curricula (Oak National Academy Aila, EEF ChatGPT lesson preparation trial, NFER Aila trial, civics lesson plans and Bloom's taxonomy, LessonPlanner, EduPlanner, Instructional Agents, RAND and Gallup teacher AI surveys), model-drafted learning objectives, knowledge components, multiple-choice questions, distractors and misconceptions, worked examples, hints, prerequisite ordering, staged generation pipelines (STORM, Co-STORM, prompt chaining, self-refine, evaluator-optimizer, multi-agent review, LLM-as-judge rubrics), human gates, human-AI productivity and quality (Noy and Zhang, Brynjolfsson, Peng Copilot, Dell'Acqua jagged frontier, METR, Vaccaro meta-analysis), machine translation post-editing thresholds and post-editese, automation bias, complacency, detection of errors in AI drafts, anchoring and homogenization, confidence and uncertainty display, explanations and over-reliance, cognitive forcing functions, citations and provenance, small review units, checklists, alternatives, authoring cost per hour of instruction (ITS 200 to 300 hours, CTAT, Chapman Alliance, Kapp and Defelice), teacher adaptation of curriculum materials, educative curriculum materials, lethal mutations, expert interviews and cognitive task analysis turned into instructional content"
---

# Stream 6 - Automated lesson-plan generation and the 80% rule (AI drafts, a human finishes)

## How I checked things

- **Date of the work:** 2026-11-22.
- **Tools:** web search and web fetch. The search tool saved the full text of many result pages to disk, and I searched those saved pages for the numbers quoted here. Every URL in the source list is one I fetched or one a search result showed me.
- **What "Read" means in the source list:**
  - **full text** - the whole paper or report was on hand and I read or searched the sections I cite (abstract, method, results, limits).
  - **excerpts** - the search tool returned passages of the page (often the abstract plus method and results paragraphs) and I did not go through the whole document.
  - **abstract only** - I saw the abstract and nothing more.
  - **secondary summary at URL** - I did not see the original. The claim comes from the named page.
- **Rule on numbers:** every number in this file was on a fetched page or in a passage the search tool returned from that page. A few numbers appeared only in the search tool's own summary of a page and not in the page text I could check. Each of those was marked "search summary" at the point of use. The citation check of 2026-11-22 found every one of them on the primary source and removed the marks (see the Citation check section).
- **Source count:** 94 sources. 30 read as full text, 48 as excerpts, 9 as abstract only, 7 as secondary summaries.
- **Core sources read in depth (the 17 that matter most):** the EEF ChatGPT lesson-preparation trial report [EEF2024]; the Oak auto-evaluation paper [OakAutoEval2025]; Trust et al. on civics lesson plans [Trust2025]; LessonPlanner [Fan2024]; Instructional Agents [Yao2025]; Malik et al. on expert interviews feeding a model [Malik2024]; the Hong Kong exam-question study [HKMCQ2025]; Feng et al. and Scarlatos et al. on distractors [Feng2024] [Scarlatos2024]; STORM [Shao2024]; Noy and Zhang [Noy2023]; Vaccaro et al. [Vaccaro2024]; the jagged frontier paper [DellAcqua2023]; Dell'Acqua's recruiter experiment [DellAcqua2022]; the study of uncorrected errors in AI-drafted patient messages [Biro2025]; Parra Escartin and Arcedillo on post-editing thresholds [Parra2015]; Goddard et al. on automation bias [Goddard2012].
- **Overlap with earlier streams:** Stream 1 (lesson design) and Stream 5 (LLM tutors) already report Moore et al. 2024 on knowledge components, Hu et al. 2025, Feng et al. 2024, Doughty et al. 2024 and Pardos and Bhandari 2024. I re-opened the first four because the brief names them, and I add detail those streams did not have. I did not re-research hint validation (Pardos, Phung) or item difficulty estimation, which Streams 1 and 5 cover.
- **Grades:** A = meta-analysis or randomized controlled trial. B = controlled or quasi-experimental study, or an expert-rated evaluation with a stated method. C = practitioner or deployed-system report. D = expert opinion or theory with no direct test.
- **What stayed unconfirmed (first pass), with the result of the citation check of 2026-11-22:**
  - Author lists for sources marked "authors not captured". The fetched text did not show them and I did not fill them in from memory. Citation check: filled in from Crossref, arXiv, PubMed or the paper's first page.
  - Doyle et al. 2025: the four SMART percentages (measurable, relevant, specific, achievable) come from the search summary of the journal page. The page text I read confirms the direction and the significance, and the preference counts. Citation check: all four pairs of percentages are in the paper's Table 4.
  - Doughty et al. 2024: "81.7% of generated questions passed all criteria" is a search summary figure. Stream 5 flagged the same number. Citation check: the sentence is in the paper text.
  - Sun et al. 2024: the 53 wins for the single-prompt method is a search summary figure. The 77 wins for chaining is in the paper text. Citation check: both counts are in the paper's Table 1.
  - Kim et al. 2024: the sample size (404), the 74.8% agreement figure for first-person uncertainty, and the accuracy change when the AI was wrong (43.6% to 52.0%) are search summary figures. The 80.9% and 77.6% agreement rates and the confidence ratings are in the paper text. Citation check: all three are in the paper text.
  - Huang et al. 2024: the per-model accuracy drops come from a secondary summary. I read the abstract and introduction on arXiv, which state the direction. Citation check: the four figures are in the paper's Table 3 (GSM8K, standard prompting against two rounds of self-correction).
  - Lee et al. 2024: "four educators" and "70% of stems, keys and explanations valid" are search summary figures. The 37% figure and the 22 of 60 count are in the paper text. Citation check: both are in the paper's section 1.1, "Contributions".
  - Grossmann et al. 2025: the 3.3% score for addressing students' conceptions is a search summary figure. The 80.8% content-accuracy figure and the score distribution are in the paper text. Citation check: the 3.3% is in the paper's results text.
  - The stepped-wedge surgical checklist trial (complications 19.9% to 11.5%) is a search summary figure. I did not open the paper. Citation check: the figures are in the paper's PubMed abstract (Haugen et al. 2015).
  - Cemri et al. 2025: the split of failures across the three categories (44.2%, 32.3%, 23.5%) is a search summary figure. The overall failure range and the per-mode rates are in the paper text. Citation check: the split is in the paper's Figure 1.
  - Kapp and Defelice's survey, the Chapman Alliance study, the Cisco code-review study, Doshi and Hauser 2024 and Padmakumar and He 2024 were read through secondary pages or reposts, named in the source list. Citation check: the Cisco figures are in SmartBear's own case-study chapter, the Chapman ratios are on the slide deck, and the Doshi and Padmakumar figures are in the open-access papers. Kapp and Defelice was checked on the repost only.
  - Van den Bent et al. 2025 (AI-led expert interviews) is reported secondhand through a 2025 conference review [Johnson2025].
  - MagicSchool, Diffit and Khanmigo teacher tools: I found one independent assessment that covers MagicSchool and Khanmigo as a class of tools [CSM2025]. I found no independent quality study of Diffit's output.
- **Injection check:** none of the fetched pages contained instructions aimed at the reader agent.

## Top findings

1. **Two school-randomised trials put the lesson-planning time saved by AI at 24% to 31%, with no measured change in resource quality.** ChatGPT with a usage guide: 56.2 against 81.5 minutes a week in weeks 6 to 10 of the trial (69% of control, 95% CI 53% to 90%, p = 0.006; 68 schools, 259 science teachers). Oak's Aila: 2.5 against 3.3 hours a week (76% of control; 108 schools, 464 primary teachers). Teachers used the tool for a minority of lessons (median 34% and mean 7%), and use fell during both trials. Grade A. [EEF2024] [NFERAila2026]
2. **Self-reported savings are several times larger than measured ones.** Weekly AI users among 2,232 US teachers estimated 5.9 hours saved a week. The two trials measured 25 and 49 minutes a week. 53% of US core-subject teachers used AI for planning or teaching in 2024-25, up from 25% a year earlier. Grade C for the survey figure, A for the trial figures. [Gallup2025] [RAND2025] [Kaufman2025] [EEF2024] [NFERAila2026]
3. **Raw model lesson plans are accurate on content and thin on thinking demand and on the learner.** Of 2,230 activities in 310 generated civics plans, 90% sat at remember, understand or apply, and asking for a "highly interactive" plan left higher-order activities under 10%. Of 60 ChatGPT-4o biology plans, 22 scored under half marks and 5 scored 75% or more; content accuracy earned 80.8% of its points and attention to students' conceptions 3.3%. Grade B. [Trust2025] [Grossmann2025]
4. **Learning objectives are the part a model drafts closest to human level.** In an expert comparison on close to 2,000 generated objectives, generated objectives matched human ones on being measurable and relevant (rated blind), were significantly weaker on being specific and achievable, and were preferred for alignment to course goals in 79 of 98 decided comparisons. At unit level humans were slightly preferred (86 against 74). Grade B. [Doyle2025] [Sridhar2023]
5. **Skill tags from a model agree with the course authors' tags about half the time, and experts side with the model in about two thirds of the disagreements.** GPT-4 matched 56% (chemistry) and 35% (e-learning) of human knowledge-component labels; in 87 mismatches three experts preferred the model's label 57 times. Grade B. [Moore2024KC]
6. **Generated multiple-choice questions come out easier and lower-order than expert ones, with a small excess of hard errors.** In a licensing-exam study, AI items had 6% factual errors against 4%, 14% wrong difficulty against 1%, 37% item-writing flaws against 35%, and a mean difficulty index of 0.78 against 0.69. In programming, 4.9% of GPT-4 questions had more than one correct option against 1.1% of human ones, and generated questions were better aligned to their objective (12.0% misaligned against 20.5%). Grade B. [HKMCQ2025] [Doughty2024] [EMPsych2025]
7. **Distractors and misconceptions are the part a model drafts worst.** Four teachers in a pilot judged 37% (22 of 60) of GPT-4 distractors valid together with their stated misconception and feedback, and rated that output 2.5 out of 5. In a second study two teachers rated generated distractors 2.67 against 3.26 for human ones and preferred the human one in 62% of head-to-head pairs. All three generated distractors matched the human-authored set for about 11% of questions. Grade B. [Lee2024] [Scarlatos2024] [Feng2024]
8. **Worked examples and hints reach usable quality with a check step.** 13 of 14 generated programming worked examples had correct code and 93% had clear explanations; the usual flaw was assuming knowledge a novice lacks. Hints produced from prompts written by subject experts taught as well as hand-written hints (learning gain 8.13% against 7.47%, p = 0.688; 358 learners recruited, 225 analysed after exclusions) at about half the authoring workload. Grade B for worked examples, A for the hint comparison. [Jury2024] [Reza2025]
9. **An outline stage improved long generated articles, and separate calls per stage improved summaries of news articles.** Removing the outline stage from STORM cut ROUGE-1 against human articles from 45.82 to 26.77. Wikipedia editors rated 25 points more STORM articles as well organized than a retrieval baseline. Three chained calls (draft, critique, refine) beat a single prompt asking for all three, 77 wins in 100 against a one-step baseline. Instruction-following in single long outputs decays past about 4,000 tokens. Grade B. [Shao2024] [Sun2024] [LongGenBench2024] [Jury2024]
10. **Self-critique improves style and structure and does little for correctness unless something outside the model checks the facts.** Self-Refine gained about 20 points on average across seven tasks and 0.2 points on math (92.9 to 93.1). Without an external signal, self-correction lowered reasoning accuracy, and multi-agent debate did no better than sampling the same model several times. Seven open multi-agent frameworks failed 41% to 86.7% of tasks. Grade B. [Madaan2023] [Huang2024] [Cemri2025] [DebateOrVote2025]
11. **A model grading a lesson against a rubric is a coarse filter.** Oak recruited 20 teachers and excluded one teacher's scores as unreliable, and its judge agreed with the remaining teachers at a weighted kappa of 0.17, rising to 0.32 after the teachers' reasoning was written into the judge prompt. In another system model reviewers gave almost every artifact 2.9 to 3.1 out of 5 while human scores spread widely. A rule-based checker matched human flaw labels on 90.87% of criteria against 78.89% for GPT-4. Grade B. [OakAutoEval2025] [Yao2025] [Moore2023]
12. **Systems that pause for a human at the plan and again at the content score better than autonomous runs, at a cost of 30 to 45 minutes per course.** Instructional Agents' co-pilot mode (a pause after every subtask) scored 0.5 to 0.9 points higher on a 5-point scale than its autonomous mode. In Atlassian's deployed coding agent, engineers approved 82% of generated plans, raised a pull request for 25% of the code that followed, and merged 59% of those. Grade B and C. [Yao2025] [HULA2024]
13. **A human plus an AI is, on average, worse than the better of the two alone.** Across 106 experiments the pooled effect was g = -0.23. The pair gained when the human alone beat the AI (g = +0.46) and lost when the AI alone beat the human (g = -0.54). Content-creation tasks showed a positive, statistically uncertain effect (g = +0.19, 34 effect sizes). Grade A. [Vaccaro2024]
14. **In the best-known writing experiment the humans barely edited, and their editing added nothing.** ChatGPT cut task time 40% and raised graded quality 18% (N = 453), 33% submitted the output unedited, editors worked for a mean 3.3 minutes after pasting, and participants who used ChatGPT scored no higher than the raw ChatGPT output graded by the same evaluators. Grade A. [Noy2023]
15. **Gains from AI drafts are largest for novices and can be negative for experts on familiar, high-standard work.** Support agents gained 14% to 15% on average and 30% to 34% in the least-skilled group, with small quality declines for the most skilled. Consultants were 25.1% faster inside the model's competence and 19 points less often correct on a task outside it. Sixteen expert developers on their own repositories took 19% longer with AI (CI +2% to +39%) while believing they were 20% faster; a 2026 rerun pointed toward a speedup and was judged unreliable by its authors. Grade A. [Brynjolfsson2025] [DellAcqua2023] [METR2025] [METR2026]
16. **Reviewers miss most of the errors in a fluent AI draft.** Twenty physicians reviewing AI-drafted patient replies missed two thirds of four erroneous drafts, sent 35% to 45% of them unedited, and one physician caught all four. The four errors were the model's own, found beforehand by two physicians, and none was planted. People checking claims with a ChatGPT explanation were right 87% of the time when the explanation was right and 35% when it was wrong, below the 49% they reached with no help. Grade B. [Biro2025] [Si2024]
17. **Automation bias is robust, affects experts, and survives training and instructions.** Wrong advice from a decision-support system raised wrong decisions by 26% (risk ratio 1.26, CI 1.11 to 1.44), and in 6% to 11% of cases a correct answer was switched to a wrong one after consulting the system. Grade A for the pooled estimate (four studies), B for the review. [Goddard2012] [Parasuraman2010]
18. **A better draft can produce a worse review.** Recruiters given an 85%-accurate AI spent less time per case and performed worse than recruiters given a 75%-accurate AI (181 recruiters). Complacency is strongest when automation is highly and constantly reliable. Grade A, single experiment. [DellAcqua2022] [Parasuraman2010] [Goddard2012]
19. **Machine-translation post-editing, the mature analogue, pays above a quality floor and leaves fingerprints in the text.** Post-editing raised throughput 74% on average (range 20% to 131%, 12 translators) with equal or better quality. Gains vanished when the draft fell below about a 75% match to the final text (BLEU about 45). Post-edited text is lexically simpler and closer to the source than text translated from scratch. Grade B. [Plitt2010] [Green2013] [Parra2015] [Toral2019]
20. **No study measures an "80% complete" threshold for lesson plans. The nearest measured thresholds sit at 70% to 75%.** Translation shows productivity loss below a 75% match, and the translation industry pays full rate below that line. One automation study cited in a systematic review puts the reliability below which operators stop using an aid at 70%. Grade B for the translation threshold, D for any transfer to lesson plans. [Parra2015] [Goddard2012]
21. **Showing confidence changes trust, and explanations by themselves raise acceptance of wrong output.** Confidence scores made people follow high-confidence predictions more without raising accuracy. Explanations increased agreement with the AI whether it was right or wrong. Explanations cut over-reliance only when they made checking cheaper than doing the task. First-person uncertainty ("I'm not sure, but") lowered agreement from 80.9% to 74.8%. Cognitive forcing designs (for example showing the AI answer only on request) cut over-reliance and were the designs users liked least. Grade A (lab RCTs with crowd workers). [Zhang2020] [Bansal2021] [Vasconcelos2023] [Kim2024] [Bucinca2021]
22. **Citations beside claims help only if they are checked, because model citations are often wrong.** In four generative search engines, 51.5% of sentences were fully supported by their citations and 74.5% of citations supported their sentence. Showing retrieved passages produced less over-reliance than showing a model explanation (54% against 35% accuracy when the model was wrong). Grade B. [Liu2023] [Si2024]
23. **One draft invites approval, and several alternatives invite criticism.** Users rated a single design higher and criticized it less than the same design shown among three. AI-assisted writing makes outputs more alike (d = 0.33 across 19 studies). Grade A and B. [Tohidi2006] [HomogMeta2026] [Doshi2024]
24. **Review quality falls with the size and speed of the review, and checklists help only when they are used as intended.** A vendor study of 2,500 code reviews found defect detection collapsing past 200 to 400 lines and past 500 lines an hour. The WHO surgical checklist cut deaths from 1.5% to 0.8% in an eight-hospital before-and-after study and showed no significant effect when mandated across 101 Ontario hospitals. Grade C for the code-review limits, B for the checklist evidence. [CiscoSmartBear] [Haynes2009] [Urbach2014] [Goncalves2022]
25. **Teachers adapt whatever they are given, and structured adaptation can beat faithful delivery.** A 27-school trial found a literacy program with structured teacher adaptations scored 0.12 SD above the same program delivered as written, with core-component adherence still at 83% against 87%. Expert-interview findings fed to a model produced lesson warm-ups that 50 teachers preferred to expert-written ones (0.55 on a -2 to +2 scale). Grade A and B. [Kim2017] [Remillard2005] [Malik2024]

## Findings by topic

### 1. How good are LLM-drafted lesson plans and curricula?

#### 1.1 Trials of teachers planning with AI: time falls, measured quality holds

- **Finding:** Teachers who plan with a general chatbot or a purpose-built lesson assistant spend about a quarter to a third less time on preparation, and blind expert review finds their resources no better and no worse.
- **Evidence:**
  - EEF and NFER ChatGPT trial, England, summer term 2024. 34 schools (129 teachers) used ChatGPT 3.5 plus an online guide; 34 schools (130 teachers) used no generative AI. Year 7 and 8 science. Primary outcome: diary-reported preparation time in weeks 6 to 10. ChatGPT group 56.2 minutes a week, comparison 81.5, a saving of 25.3 minutes. The ChatGPT group's time was 69% of the comparison group's (95% CI 53% to 90%, p = 0.006). The report rates the finding "high security".
  - Quality in the same trial: lesson resources from 30 sampled teachers were ranked 1 to 30 by a panel of five experienced science teachers blind to group. Mann-Whitney U = 184, p = 0.84.
  - Use was modest: the median teacher used ChatGPT for 34% of lessons, falling from 39% of lessons in weeks 1 to 5 to 29% in weeks 6 to 10. Teachers "tended to use ChatGPT for one or two activities rather than across the lesson: most commonly for creating questions or quizzes or finding new ideas for activities."
  - NFER Aila trial, England, autumn term 2025. 108 schools, 464 Key Stage 2 teachers. Aila group 2.5 hours a week, comparison 3.3 hours, a saving of 48.6 minutes; the Aila group's time was 76% of the comparison group's. Resource quality in the Aila group "was no worse". Aila was used for an average of 7% of lessons and use declined over the ten weeks. Teachers with lower subject or pedagogical confidence and teachers new to the profession saved more.
- **Caveats:**
  - Time is self-reported in weekly diaries. 18.5% of teachers in the ChatGPT trial were missing from the primary analysis.
  - The quality result in the ChatGPT trial rests on 30 teachers who chose which resources to send. The report says teachers "were able to choose which lessons they sent us, so there is likely to be some self-selection bias".
  - Neither trial measured pupil learning.
  - In the Aila trial both groups could use other AI tools, so the contrast is Aila against whatever else teachers used. Many teachers "preferred alternative GenAI tools which provided more ease of use", and Aila's outputs "often required adaptation before being classroom ready". Aila is built to walk the teacher through the plan section by section, "requiring more involvement".
  - I read the Aila trial's publication summary, not the full report. The citation check found the same headline figures in the full evaluation report (October 2026).
- **Grade:** A.
- **Sources:** [EEF2024] [NFERAila2026]

#### 1.2 Unedited plans from general chatbots: accurate, lower-order, and blind to the learner

- **Finding:** When researchers prompt a chatbot once and rate what comes back, the plans are mostly correct on content and weak on cognitive demand, on anticipating what students think, and on staying faithful to the requested pedagogy.
- **Evidence:**
  - Trust, Maloy, Xu and Pelletier generated 310 plans (2,230 activities) from ChatGPT, Gemini and Copilot for 53 Massachusetts civics standards in August 2024. Bloom coding: remember 45%, apply 24%, understand 21%, analyze 4%, create 4%, evaluate 2%. So 90% lower-order and 10% higher-order. A second prompt asking for a "highly interactive" plan kept higher-order activities under 10% for all three chatbots. Higher-order work was pushed into homework: for Gemini, 85% of extension activities were higher-order against 9% of in-class activities. 94% of activities had no multicultural content on Banks's model.
  - Grossmann, Koberstein-Schwarz, Kruger and Krell rated 60 ChatGPT-4o biology plans for inquiry-based teaching on a ten-criterion rubric (two raters; kappa is .67 in the abstract and .77 in the method text). 22 plans scored under 50% of the maximum and 5 scored 75% or more. Content accuracy earned 80.8% of possible points and alignment of performance indicators 79.2%. Addressing students' inquiry-related conceptions earned 3.3%. Many plans "shifted inappropriately towards subject matter knowledge acquisition rather than scientific inquiry processes", against the prompt.
  - Powell and Courchesne built a first-grade heredity plan with ChatGPT in 30 minutes. It matched the curriculum framework and the 5E model, and it contained one invented book, an unsafe suggestion (live animals), and missing detail (no discussion prompts, no guidance on how to observe).
  - The authors of the Hong Kong exam study and of the civics study both note that their prompts did not ask for higher-order thinking.
- **Caveats:** These are single-shot or lightly iterated prompts to 2024 models with no curriculum documents attached. Bloom coding by verb is a blunt measure. The civics authors assumed teachers would not re-prompt, on the word of 10 teachers informally surveyed (2 said they would).
- **Grade:** B.
- **Sources:** [Trust2025] [Grossmann2025] [Powell2024]

#### 1.3 Plans from structured systems score higher, on small samples and often on their own rubric

- **Finding:** Wrapping the model in a lesson structure (objectives first, then a fixed sequence of lesson events, then materials) raises rated quality over plain chat. The evidence is from small studies run by the system builders.
- **Evidence:**
  - LessonPlanner (UIST 2024): objectives set against Bloom's taxonomy, then an outline on Gagne's nine events, then per-section materials with regenerate, edit and history. Within-subjects study, N = 12 (graduate and senior undergraduate students acting as teachers), 40 minutes per plan. Plan quality 32.25 against 26.33 for plain ChatGPT (p = 0.02). Alignment of content to objectives did not differ (5.42 against 4.92, p = 0.40). Mental demand 2.75 against 4.58 (p = 0.035). Six teachers interviewed said the remaining workload came from "the selection, refinement, and verification of LLMs' outputs", and asked for sources: "I would not count on an example without credible sources."
  - Hu and colleagues (npj Science of Learning 2025), high-school mathematics, 480 plans in eight sets of 60, rated on an 8-point scale. Plans generated directly by GPT-4 scored below pre-service teachers' plans on most dimensions. Plans generated with a "problem chain" framework scored slightly above pre-service teachers. One round of simulate-the-class, reflect, revise brought them slightly above exemplary teachers' plans. The highest-scoring set was pre-service teachers' own plans after one round of model revision (around 7 of 8). The earlier paper (Hu et al. 2024) found GPT-4 strong on objectives, priorities and problem chains and weak on geometry, algebra, mathematical culture and interdisciplinary assessment.
  - EduPlanner (2025): evaluator, optimizer and question-analyst agents in a loop. Overall score 88 against 49 for GPT-4 alone, on a five-dimension rubric applied by the system's own evaluator model.
  - Instructional Agents (2025): role-playing agents on an ADDIE workflow producing objectives, syllabus, assessments, slides and scripts for five university courses. Five human raters per course on a 1 to 5 scale that reflects revision effort. Autonomous mode averaged 2.73 to 3.24 across courses, co-pilot mode 3.55 to 3.98. A single-agent baseline averaged 2.33. The paper says objectives and slides rated highest and scripts slightly lower; in its table assessments have the lowest autonomous-mode scores (2.31 to 3.10).
  - Malik, Abdi, Wang and Demszky (2024): a model given the original curriculum warm-up plus a prompt built from expert-teacher interviews produced warm-ups that 50 teachers preferred to expert-written ones: alignment 0.636, accessibility 0.447, classroom readiness 0.371, overall 0.545, on a -2 to +2 scale (500 paired comparisons, all p < 0.001).
- **Caveats:** EduPlanner's scores come from its own judge. LessonPlanner's users were students, not working teachers. Hu's plans are rated on paper, never taught. In Hu's study the plans that started from a human draft kept more "practical value" than plans that started from a model draft: the authors call the model-origin plans "fabricated" in the sense of never having met a classroom. Malik's task is one slide per lesson.
- **Grade:** B.
- **Sources:** [Fan2024] [Hu2025] [Hu2024] [EduPlanner2025] [Yao2025] [Malik2024]

#### 1.4 Deployed teacher tools: grounded in a curated corpus, checked by a second model, finished by the teacher

- **Finding:** The one deployed lesson planner with a published design and evaluation (Oak's Aila) relies on retrieval from 13,000 expert-made resources, a fixed plan template walked through in steps, a separate moderation model, and a rubric-driven automatic evaluator. An independent assessment of commercial teacher assistants rates the class "moderate risk" and names frictionless export as the main hazard.
- **Evidence:**
  - Aila's plan template has ten parts in order: lesson details, learning outcome, learning-cycle outcomes, prior knowledge, key learning points, misconceptions, keywords, starter quiz, learning cycles (each with explanation, checks, practice, feedback), exit quiz. The teacher can change each part before moving on. Updates come back as JSON patches to the plan.
  - Retrieval finds similar Oak lessons, reranks them, and the teacher may pick one as the basis ("content anchoring"). Oak cites a UK government study in which a curated retrieval corpus raised accuracy from 67% to 92%.
  - Oak's evaluator scores each lesson on 19 Likert criteria and 5 true-or-false checks (24 benchmarks). On 4,985 lessons the criterion that scored consistently below average was quiz distractors ("minimally different answers"). Twenty teachers (mean 14 years' experience; 19 analysed) rated 311 questions. Judge against teachers: mean squared error 3.83, quadratic weighted kappa 0.17. After the teachers' reasons were coded into the judge prompt with examples: 2.95 and 0.32.
  - Oak says it plans an "improvement agent" that uses the evaluator's feedback to fix a lesson before the user sees it and to "suggest specific areas for users to check carefully".
  - Common Sense Media's 2025 assessment of AI teacher assistants (Khanmigo, MagicSchool, Curipod, Gemini in Classroom): overall "moderate risk". It reports plausible-looking outputs with one-sided content and bias, and tools that "make it too easy to export content directly to classrooms without review", for example slides pushed to a projector in one click.
  - Teacher use: 53% of US English, mathematics and science teachers used AI for planning or teaching in 2024-25 (n = 8,601), 13% at least weekly; 25% a year earlier. In a 24-teacher case study, teachers who asked AI for ideas about the plan as well as for materials reported gains in workload and quality, and teachers who asked only for materials did not.
- **Caveats:** Oak's papers are by the builder. The kappa of 0.32 is still low agreement. Common Sense Media's report is a structured product test, not a controlled study. The case study is 24 self-reports.
- **Grade:** C, with B for the Oak judge-agreement study.
- **Sources:** [OakAutoEval2025] [OakATRS2024] [OakGuide] [CSM2025] [RAND2025] [Kaufman2025] [Keppler2024]

### 2. Model-drafted parts of a plan

No study I found reports a clean three-way split (accepted as-is, edited, rejected) for every part. What each study did report is set out below, with the nearest equivalent.

#### 2.1 Learning objectives

- **Finding:** Objectives are drafted at close to human level. The usual defects are vagueness and over-reach, which an expert fixes by narrowing.
- **Evidence:**
  - Sridhar et al. 2023: 127 GPT-4 objectives for an AI practitioner course. All began with an action verb, the verbs fit the intended Bloom level (understand for concept modules; apply, analyze, evaluate, create for projects), and the authors call them "largely sensible".
  - Doyle et al. 2025: about 2,000 generated course-level and unit-level objectives against human ones from 6 university courses and 95 Coursera courses, two expert evaluators. Share of unit-level objectives meeting each SMART standard, generated against human: measurable 67.3% against 70.0%, relevant 86.9% against 89.1%, specific 51.9% against 68.6%, achievable 75.4% against 90.4%. The paper text confirms a statistically significant deficit on specific and achievable and parity on measurable and relevant. Alignment with course goals: generated preferred 79 times, human 19. Alignment with unit topics: generated 74, human 86. Ordering and non-overlap at unit level: human preferred (35 against 19 on overlap, 37 against 27 on order); at course level the generated lists were judged better organized.
  - Instructional Agents: objectives were the highest-rated artifact (3.17 to 3.87 in autonomous mode).
- **Caveats:** The human objectives in Doyle's set were scraped from real syllabi and not screened for quality. Evaluators were also the paper's senior authors.
- **Grade:** B.
- **Sources:** [Sridhar2023] [Doyle2025] [Yao2025]

#### 2.2 Knowledge components and skill tags

- **Finding:** A model's skill label matches the course author's about half the time, and where they differ experts more often prefer the model's.
- **Evidence:** Moore, Schmucker, Mitchell and Stamper 2024, 80 questions per domain, GPT-4. Exact match: chemistry 45 of 80 (56%), e-learning 28 of 80 (35%). Human label within the model's top five: 79% to 80% (chemistry), 56% to 63% (e-learning). In the 87 mismatches, three experts per domain preferred the model's label in 57 (chemistry 23 of 35, e-learning 32 of 52; p = 0.017). The paper's two domain counts add up to 55, and it states 57 of 87 for the pooled test.
- **Caveats:** Preference is not validity: no learner data tested whether either labeling predicts performance. Stream 1 covers the learner-data methods.
- **Grade:** B.
- **Sources:** [Moore2024KC]

#### 2.3 Multiple-choice questions: stems, keys and overall flaws

- **Finding:** Stems and keys are mostly sound. The excess defects over human-written items are a few percent of hard errors (wrong fact, two correct options), items that are too easy, and a skew to recall.
- **Evidence:**
  - Hong Kong emergency-medicine licensing exam, ChatGPT-4o, 100 AI items against 100 written by 26 experts, reviewed by 6 specialists, sat by 24 candidates. Factual errors 6% against 4%. Irrelevant 6% against 0%. Wrong difficulty 14% against 1%. Duplicates 14% against 7%. Item-writing flaws 37% against 35%. Difficulty index 0.78 against 0.69 (higher is easier). Discrimination 0.22 against 0.26 (not significant); 38% of AI items and 51% of human items reached the 0.3 standard. AI items sat more often at remember and understand (chi-square 14.27, p = 0.003).
  - Doughty et al. 2024, Python courses, 651 GPT-4 items against 449 human items for 246 objectives, 13 annotators. More than one correct option: 4.9% against 1.1%. A distractor that gives the answer away: 4.0% against 0.9%. Not aligned to the objective's level: 12.0% against 20.5%. Unrelated to the objective: 4.8% against 12.0%. Share of generated items passing every criterion: 81.7%.
  - Other psychometric studies: 50 against 50 items, 18 residents: difficulty 0.76 against 0.65, 56% of AI items "easy" against 36%, 36% of AI items flagged problematic on point-biserial against 24%. 21 against 25 items, 161 students, ChatGPT 3.5: equal difficulty, discrimination .24 against .36. 40 AI against 40 novice and 40 expert items, 234 students, after panel review of all items: equal difficulty and discrimination, AI distractor efficiency 39% against 55% (novice) and 48% (expert).
  - Lee et al. 2024, mathematics: four teachers judged 70% of generated stems, keys and explanations valid.
  - Flaw detection: on 200 student-written questions a rule-based checker agreed with human raters on 90.87% of flaw classifications and GPT-4 on 78.89%. The later SAQUET toolkit reports over 94% accuracy at detecting the flaws humans found.
- **Caveats:** Reviewers in the Hong Kong study knew which items were AI-written. Prompts did not request higher-order items. Medical items dominate this literature; the one programming study shows the same two-correct-options problem.
- **Grade:** B.
- **Sources:** [HKMCQ2025] [Doughty2024] [EMPsych2025] [AcadMed2024] [MedTeach2025] [Lee2024] [Moore2023] [SAQUET2024]

#### 2.4 Distractors built from misconceptions

- **Finding:** A model produces wrong answers that are mathematically reasonable and misses the wrong answers real students give. This is the widest gap between model and expert in the literature.
- **Evidence:**
  - Feng et al. 2024, 1,400 Eedi mathematics questions with student response data. Best method (three similar questions as examples): all three generated distractors matched the human set for 10.95% of questions, at least one matched for 73.85%, and 38.52% of generated distractors matched. Zero-shot chain-of-thought with GPT-4: 4.24%, 65.02%, 30.39%. In 20 sampled questions where nothing matched, the authors judged the generated distractors plausible in 14.
  - Scarlatos et al. 2024: generate ten, rank by a model trained on which options students actually chose, keep three. Two mathematics teachers rated generated distractors 2.67 against 3.26 for human ones (p < 0.05) on likelihood of being chosen. Head to head: human better 62%, tie 16%, generated better 22%. The ranking model ordered distractors as well as the teachers did (Kendall tau 0.30 against 0.27 with real student choices).
  - Lee et al. 2024 (a pilot with four teachers): of 60 generated distractors with a stated misconception and feedback, the teachers judged 37% (22) valid on all three, and rated the set 2.5 of 5, saying most "do not reflect what students typically make in the classroom". The authors count two of those 22 as in fact invalid.
  - Oak's own evaluator found distractor quality its weakest benchmark across 4,985 lessons.
- **Caveats:** Human-authored distractors are an imperfect standard: Feng notes many questions have one or two real distractors and placeholders. Teachers themselves predict student choices only weakly (tau 0.27). All the evidence is school mathematics.
- **Grade:** B.
- **Sources:** [Feng2024] [Scarlatos2024] [Lee2024] [OakAutoEval2025]

#### 2.5 Worked examples, explanations and hints

- **Finding:** Step-by-step examples and explanations are usually correct and readable. Their typical defect is pitching above the novice.
- **Evidence:**
  - Jury et al. 2024 (WorkedGen): 14 generated worked examples rated by two experts. One had incorrect code. 93% had clear explanations. All but one had a step count in the intended 3 to 10 range. Common weaknesses: uncommented code and assumed prior knowledge (for example object-oriented concepts). About 400 first-year students used the tool 787 times and rated it positively. Prompt chaining was needed: generating example and keywords in one prompt "led to inconsistent outputs".
  - Denny et al. 2023: students rated model-written code examples with explanations the same as student-written ones (median 4 of 5 on code correctness, explanation correctness and helpfulness, no significant differences).
  - Reza et al. 2025 (PromptHive): ten subject experts wrote and iterated prompts, the model wrote hints for a college algebra textbook. Randomized comparison with 358 learners recruited, of whom 225 were analysed after exclusions (per the arXiv version of the paper): gain 8.13% with the generated hints, 7.47% with hand-written hints, difference not significant (p = 0.688). Expert workload on NASA-TLX fell from 55.17 to 26.73.
  - Stream 5 reports the companion evidence: 32% of raw ChatGPT mathematics hints wrong, falling to 0 to 13% after self-consistency voting.
- **Caveats:** Jury's expert sample is 14 examples. Denny's ratings are by students. PromptHive hints still needed 3 to 6 hours of manual fixes by two authors to fit the tutor's format.
- **Grade:** B, with A for the PromptHive learning comparison.
- **Sources:** [Jury2024] [Denny2023] [Reza2025]

#### 2.6 Concepts and prerequisite ordering

- **Finding:** Models extract course concepts that experts rate above an existing dataset's labels. Prerequisite direction between two concepts is harder, and I found no study that reports the share of model-proposed prerequisite links an expert accepts.
- **Evidence:** Yang et al. 2025: four experts rated GPT-generated concepts for 20 sampled courses above the MOOCCube ground-truth concepts (the journal version gives up to 3.7 against 2.68 on a 5-point scale). On 100 concept pairs, GPT-4o was best at prerequisite prediction, more context helped the strongest model and hurt weaker ones, and the authors say many prerequisite relations are "subtle and challenging to identify, even for human evaluators".
- **Caveats:** Stream 1 reports that expert prerequisite orders themselves agree with learner data on 56% of unit pairs, so expert acceptance is a weak test here.
- **Grade:** B to C.
- **Sources:** [Yang2025]

#### 2.7 Summary by part

- **Drafted well (near human, light edits):** learning objectives; concept lists; skill tags; question stems and keys; alignment of questions to objectives; worked-example code and explanations; hints when a check step is present; content accuracy of explanations.
- **Drafted adequately (systematic, fixable bias):** difficulty calibration (too easy); cognitive level (too much recall); specificity and scope of objectives; assumed prior knowledge in examples; ordering within a unit.
- **Drafted badly (expert must supply or replace):** misconceptions and the distractors built on them; what this group of learners will find hard; fidelity to a requested pedagogy across a whole plan; sources and citations.
- **Grade:** B for the pattern, drawn from the studies above.
- **Sources:** [Doyle2025] [Moore2024KC] [HKMCQ2025] [Doughty2024] [Feng2024] [Scarlatos2024] [Lee2024] [Jury2024] [Grossmann2025] [Trust2025]

### 3. Pipeline structure: what staging improves long generated documents, and where the human gates sit

#### 3.1 Outline first, then write

- **Finding:** Separating research and outlining from writing improves the organization and coverage of long generated documents, and the outline is the stage experts find most useful to receive.
- **Evidence:**
  - STORM (Shao et al., NAACL 2024) researches a topic by simulated multi-perspective question asking, builds an outline, then writes with citations. Against human Wikipedia articles: ROUGE-1 45.82 for STORM, 44.26 for an outline-driven retrieval baseline, 28.52 for retrieval with no outline, 25.62 for direct generation. STORM with its outline stage removed: 26.77. Outline coverage (heading soft recall): 86.26% for STORM against 80.23% for direct generation with GPT-3.5, 92.73% against 87.66% with GPT-4.
  - Ten experienced Wikipedia editors rated 20 article pairs: 25 percentage points more STORM articles were judged organized and 10 points more had good coverage than the outline baseline. All ten agreed STORM would help their pre-writing stage and 80% said it would help them edit an article on a new topic.
  - The same editors found the weak spots: 7 of 10 said the articles sounded "emotional" or "unneutral" because the tone and bias of web sources transferred, and low verifiability scores came from "red herring fallacy or overspeculation", where the article "introduce[s] unverifiable connections between different pieces of information". The authors: "addressing such verifiability issues is more nuanced, surpassing basic fact-checking."
  - Co-STORM lets a person steer the research conversation and shows a live mind map. 20 volunteers; 70% preferred it to a search engine and 78% to a retrieval chatbot; participants rated 89% of the agents' turns they reviewed as steering toward a new and interesting direction.
  - LessonPlanner, Aila and Instructional Agents all put objectives and outline before materials.
- **Caveats:** STORM's automatic rubric scores come from an evaluator model the authors say may overrate machine text. Co-STORM's study has 10 and 9 people per comparison. The task is encyclopedia articles, not lesson plans.
- **Grade:** B.
- **Sources:** [Shao2024] [CoSTORM2024] [Fan2024] [OakATRS2024] [Yao2025]

#### 3.2 Chained prompts against one big prompt

- **Finding:** Doing draft, critique and revision as separate calls beats asking for all three in one prompt, and long single outputs lose track of their instructions.
- **Evidence:**
  - Sun et al. 2024, instruction-controlled summarization, 100 articles. Refined summaries from three chained GPT-4 calls beat a one-step GPT-4 baseline in 77 of 100 comparisons; the single "stepwise" prompt won 53. Human evaluation agreed. It was a different comparison: two postgraduate students compared the chained summary with the single-prompt summary directly, on 30 of the 100 pairs, and with GPT-4 the chained summary won 14, tied 8 and lost 8. The authors suspect the single prompt "might produce a simulated refinement process": the first draft inside a stepwise prompt is weaker, as if written to be corrected.
  - Jury et al. 2024: chaining the worked-example prompt and the keyword prompt "greatly improved the quality of the keywords"; one combined prompt "led to inconsistent outputs". They cite an earlier chaining study (Wu et al.) in which chained prompts beat one-step prompts 82% of the time.
  - LongGenBench (ICLR 2025): models held to their instructions at short lengths and "performance gradually degraded as the text generation extended beyond the 4,000" token mark, on 16,000 and 32,000 token tasks.
  - Anthropic's practitioner guide describes prompt chaining as trading latency for accuracy "by making each LLM call an easier task" and gives as its example "writing an outline of a document, checking that the outline meets certain criteria, then writing the document based on the outline."
- **Caveats:** Sun's main evaluation is a model judge, on one task. The Anthropic guide is vendor advice with no data attached.
- **Grade:** B, with C for the vendor guidance.
- **Sources:** [Sun2024] [Jury2024] [LongGenBench2024] [Anthropic2024]

#### 3.3 Self-critique, evaluator-optimizer loops and multi-agent review

- **Finding:** A critique-and-revise loop reliably improves qualities a reader can see on the page (clarity, structure, completeness against a stated rubric). It does not reliably find factual or reasoning errors unless an outside check supplies the signal. Adding more agents adds failure modes.
- **Evidence:**
  - Self-Refine (Madaan et al. 2023): one model drafts, critiques and revises. About 20 points absolute average gain over seven tasks. Mathematics: 92.9 to 93.1 for GPT-4. The authors: feedback on math said "everything looks good" in 94% of ChatGPT cases, and gains were "much bigger (5%+) if an external source can identify if the current math answer is incorrect."
  - Huang et al. 2024: without external feedback, models "struggle to self-correct their responses" on reasoning and "at times, their performance even degrades". Earlier reported gains depended on oracle labels. Multi-agent debate was "no better than self-consistency" at an equal number of responses. The paper's Table 3 gives examples: GPT-3.5 on a math set (GSM8K) 75.9% to 74.7%, GPT-4 95.5% to 89.0% after two rounds.
  - Debate or Vote (2025): across seven benchmarks "Majority Voting alone accounts for most of the performance gains typically attributed to" multi-agent debate.
  - Cemri et al. (NeurIPS 2025): 1,600 annotated traces from 7 multi-agent frameworks; failure rates 41% to 86.7%; 14 failure modes in three groups: system design, inter-agent misalignment, task verification (44.2%, 32.3% and 23.5% of failures). Common single modes: step repetition 15.7%, failing to recognize completion 12.4%, disobeying the task specification 11.8%.
  - In education: Hu et al.'s simulate, reflect, revise loop raised plan ratings on every dimension (Topic 1.3). Instructional Agents with all role agents scored above a single agent (2.33). EduPlanner's loop raised its own judge's score from 49 to 88.
  - In the Hong Kong exam study, reviewers' comments were pasted back to the model for revision, the same loop with a human as evaluator.
- **Caveats:** The negative results are on question answering with one right answer. Lesson-plan quality is closer to the open-ended tasks where Self-Refine gained most. The education results are builder-run and rated on rubrics close to what the loop optimizes.
- **Grade:** B.
- **Sources:** [Madaan2023] [Huang2024] [DebateOrVote2025] [Cemri2025] [Hu2025] [Yao2025] [EduPlanner2025] [HKMCQ2025]

#### 3.4 Rubric-based automatic evaluation

- **Finding:** A model applying a written rubric separates clearly bad from clearly good on narrow, concrete criteria and agrees poorly with experts on judgment criteria until expert reasoning and examples are written into its prompt. Rules beat models where a rule can be written.
- **Evidence:** Oak: weighted kappa 0.17 to 0.32 on distractor quality after teacher-informed prompt changes (Topic 1.4). Instructional Agents: model reviewers "consistently assign moderate and tightly clustered scores (typically between 2.9 and 3.1)" while humans "produce a broader range"; the authors rely on human ratings. Moore et al. 2023: rule-based 90.87% against GPT-4 78.89% agreement with human flaw labels. STORM: the authors "acknowledge the possibility of the evaluator LLM overrating machine-generated text". Stream 5 reports the same pattern for tutor rubrics (narrow yes-or-no criteria reach human-level agreement, broad ones fail).
- **Caveats:** Judge models have changed since these studies. Oak's result covers one criterion.
- **Grade:** B.
- **Sources:** [OakAutoEval2025] [Yao2025] [Moore2023] [Shao2024]

#### 3.5 Where the human gates sit in systems that report data

- **Finding:** Systems that report outcomes put a human decision at the plan or outline, and a second one on the finished content. Approval rates are high at the plan and much lower at the content. A pause after every stage gives the best quality and costs the most human time.
- **Evidence:**
  - HULA at Atlassian (2,600 practitioners by September 2024): a plan was generated for 527 of 663 work items (79%). Engineers approved 433 plans (82%). Code was generated for 376 (87%). A pull request was raised for 95 (25%). 56 were merged (59%). End to end, 8% of work items produced merged code. In a survey of 109 users, 61% agreed the generated code "could be easily understood and modified".
  - Instructional Agents, four modes for one course: autonomous (0 human minutes, average rating 2.73 to 3.24), catalog-guided (10 to 15 minutes supplying structured preferences up front, 3.36 to 3.63), feedback-guided (20 to 30 minutes reviewing finished outputs and rerunning stages, 3.10 to 3.78), full co-pilot (pause after each subtask, 30 to 45 minutes, 3.55 to 3.98). Up-front structured input did better on objectives and syllabus; after-the-fact feedback did better on assessments and slides.
  - Aila: the teacher confirms or changes each of ten sections in order. In the trial this design read to teachers as "requiring more involvement", and many preferred easier tools.
  - STORM: editors valued the tool for the pre-writing stage (unanimous) more than as a finished article.
  - The prototype builder in this project has one gate, after the prerequisite chain (context for the report writer, not evidence).
- **Caveats:** HULA has no comparison group. Instructional Agents' human minutes are the authors' estimates over five courses. No study randomizes where the gate sits.
- **Grade:** C, with B for the Instructional Agents mode comparison.
- **Sources:** [HULA2024] [Yao2025] [NFERAila2026] [OakGuide] [Shao2024]

### 4. Human finishing an AI draft: does it work?

#### 4.1 Speed and average quality rise, most for the less skilled

- **Finding:** Randomized studies of knowledge work with generative AI find large time savings and quality gains on average, concentrated among less skilled workers.
- **Evidence:**
  - Noy and Zhang (Science 2023): 453 college-educated professionals, 20 to 30 minute writing tasks, GPT-3.5. Time down 40%, graded quality up 18%, inequality between workers down.
  - Brynjolfsson, Li and Raymond (QJE 2025): 5,172 support agents, staggered rollout. Issues resolved per hour up 15% on average (14% in the 2023 working paper), about 30% to 34% for the least skilled and least experienced, with "small gains in speed and small declines in quality" for the most skilled. Agents with two months' tenure and the tool performed like agents with over six months without it.
  - Peng et al. 2023: 95 freelance developers, one JavaScript task, GitHub Copilot. Completers finished in 71.17 against 160.89 minutes, 55.8% faster (95% CI 21% to 89%). 35 per group completed.
  - Dell'Acqua et al. (Organization Science, online 2026; working paper 2023): 758 BCG consultants, GPT-4, 18 tasks inside the model's competence. 12.2% more tasks completed, 25.1% faster, higher graded quality, with the bottom half of performers gaining most.
- **Caveats:** Noy and Zhang's tasks were "relatively generic writing", set through short, self-contained prompts, with no factual-accuracy requirement. Peng's interval is wide and code quality was not measured. Brynjolfsson's is one firm and not randomized at the individual level.
- **Grade:** A.
- **Sources:** [Noy2023] [Brynjolfsson2025] [Peng2023] [DellAcqua2023]

#### 4.2 The pair is often worse than the better member alone

- **Finding:** Averaged over many experiments, a human working with an AI does better than the human alone and worse than whichever of the two is better alone. Who is better alone decides the sign.
- **Evidence:**
  - Vaccaro, Almaatouq and Malone (Nature Human Behaviour 2024): 106 experiments, 370 effect sizes, 2020 to mid-2023. Human plus AI against the better of the two: g = -0.23 (95% CI -0.39 to -0.07). Against the human alone: g = +0.64. Decision tasks: g = -0.27. Creation tasks: g = +0.19 (CI -0.09 to +0.48, 34 effect sizes), significantly different from decision tasks. When the human alone outperformed the AI: g = +0.46. When the AI alone outperformed the human: g = -0.54. Heterogeneity I2 = 97.7%. Explanations and confidence displays did not moderate the result on average.
  - Noy and Zhang: 33% of the treatment group submitted ChatGPT's output unedited and 53% edited first. Editors were active a mean 3.3 minutes after pasting, most for 0 to 2 minutes. "There was no correlation between how long a participant was active after pasting in the ChatGPT text and the grade they ultimately received, and treated respondents who used ChatGPT did not receive higher average grades than the raw ChatGPT output."
  - Dell'Acqua et al.: on one task built to lie outside the model's competence, 84.5% of the control group gave the correct recommendation against 60% and 70.6% in the two AI groups, 19 points lower on average. The 2023 working paper prints the two AI figures as 60% and 70%; the 70.6% is from the journal version (unconfirmed).
  - Counter-evidence that human involvement adds quality: Instructional Agents' ratings rose with each added level of human involvement (Topic 3.5), and in Hu et al. the best plans started from a human draft.
- **Caveats:** About 85% of the effect sizes in the meta-analysis are decision tasks and about 10% creation tasks. Most participants were crowd workers. Few generative-AI studies report AI-alone performance, which is what the "better of the two" comparison needs. The meta-analysis authors note that in high-stakes settings a human may be worth keeping to catch rare costly errors even when average performance drops.
- **Grade:** A.
- **Sources:** [Vaccaro2024] [Noy2023] [DellAcqua2023] [Yao2025] [Hu2025]

#### 4.3 Experts on familiar, high-standard work can be slowed down

- **Finding:** When the reviewer is already expert, knows the material deeply, and holds the output to a high standard, the time spent prompting, waiting, reading and fixing can exceed the time saved, and the reviewer does not notice.
- **Evidence:**
  - METR (2025): 16 experienced open-source developers, 246 real issues on repositories they had worked on for 5 years on average, each issue randomized to AI allowed or not. With AI, issues took 19% longer (CI +2% to +39%). Developers forecast a 24% speedup beforehand and estimated a 20% speedup afterwards. 75% said they read every line of generated code, 56% said they often made major changes to clean it up, and about 9% of time with AI went to reviewing and cleaning outputs.
  - METR's February 2026 update, with 57 developers: an estimated 18% speedup for returning developers (CI -38% to +9%) and 4% for new recruits (CI -15% to +9%). METR calls the data "an unreliable signal" because developers who would not work without AI declined to join and 30% to 50% withheld tasks they did not want to do without AI. METR believes developers are "more sped up" in early 2026 than in early 2025.
  - Brynjolfsson et al.: the most skilled agents saw small quality declines.
- **Caveats:** Sixteen developers. Early-2025 tools. The setting (mature codebase, deep prior knowledge, strict review standards) is the opposite of drafting a new lesson plan from scratch, and close to an expert correcting a plan in their own specialty.
- **Grade:** A, small sample.
- **Sources:** [METR2025] [METR2026] [Brynjolfsson2025]

### 5. Post-editing of machine translation, the closest mature analogue

#### 5.1 Productivity and quality against translating from scratch

- **Finding:** With a draft of adequate quality, post-editing is faster than translating from scratch and the final quality is equal or better.
- **Evidence:**
  - Plitt and Masselot 2010 (Autodesk): 12 professional translators, four target languages, 144,648 source words. Post-editing raised throughput 74% on average, "MT saved 43% of the" translation time; the paper gives the individual range as 20% to 131%. Every translator gained. A blind QA team checked samples against Autodesk's standard.
  - Green, Heer and Manning 2013: controlled experiment, three language pairs, 2012 Google Translate. Post-editing reduced time (p < 0.001) and improved quality rank (p < 0.001).
  - Guerberof 2009: 8 translators, 791 words. Mean productivity gain over new text: 25% for machine-translated segments, 11% for translation-memory matches in the 80% to 90% band, with very high variance.
  - Parra Escartin and Arcedillo 2015: 10 in-house translators on a real job. Post-editing gained 24.09% over translating from scratch overall.
- **Caveats:** Small samples, one company's content in each study, and throughput measured inside the editor only (no research, queries or self-revision time).
- **Grade:** B.
- **Sources:** [Plitt2010] [Green2013] [Guerberof2009] [Parra2015]

#### 5.2 The quality floor below which post-editing stops paying

- **Finding:** Editing beats starting over only when the draft is already close to the final text. In the one study that mapped the break-even point, it lay at about a 75% character-level match between draft and final, or BLEU about 45.
- **Evidence:**
  - Parra Escartin and Arcedillo 2015: sorting segments by how close the draft was to the translator's final text, "the last band where productivity gain is reported is the 75-80 range. The productivity for 70-75 falls slightly below translation from-scratch average, while below 70 the productivity loss is more dramatic." For BLEU the last band with a gain was 45 to 50, with 35 to 45 slightly below from-scratch and a sharp drop under 35. For TER the tipping point lay between 30 and 40.
  - The same paper notes this matches industry practice: translation-memory matches under 75% are paid at the full rate "following the general assumption that they do not yield any productivity increase". A handbook chapter states matches below 70% "are usually not taken into account, as adaptation often proves more strenuous than retranslation".
  - Gain by band in that study: 100% matches +127%, 85% to 94% matches +77%, 75% to 84% matches +23%.
  - Sanchez-Torron and Koehn 2016: 9 translators, 9 systems of graded quality. Each BLEU point cut post-editing time by 0.16 seconds a word, about 3% to 4%, and "the MT system with the lowest BLEU score produced the output that was post-edited to the lowest quality".
  - Segments from machine translation were slower to edit than translation-memory segments needing the same amount of change. The authors' explanation: the memory tool highlights what differs, and with machine output the translator must first find what is wrong.
- **Caveats:** One language pair, one rule-based engine, ten translators. A match score measures surface overlap with the final text, which has no direct counterpart for a lesson plan.
- **Grade:** B.
- **Sources:** [Parra2015] [SanchezTorron2016] [HansenSchirraChapter]

#### 5.3 The bias post-editing leaves in the text

- **Finding:** Post-edited text stays closer to the machine draft than a from-scratch translation would be: simpler vocabulary, more normalized, and more shaped by the source.
- **Evidence:**
  - Toral 2019, three datasets, five translation directions: "In all cases the lexical variety in PE is lower than in HT, and again in all cases, that of MT is lower than that of PE." Post-edits also showed lower lexical density and part-of-speech sequences closer to the source language. Toral names this "post-editese" and traces it to the machine output the editor starts from.
  - Green et al. 2013: post-edited translations were significantly closer in edit distance to the suggestion than unaided translations were, in all three languages (p < 0.001), which the authors read as priming.
  - Sanchez-Torron and Koehn: worse drafts produced worse final texts.
  - The same pattern appears in AI-assisted writing generally (Topic 6.3).
- **Caveats:** Toral's datasets are small and his measures are surface statistics. Most quality studies count errors, which is why they find post-editing "equal or better" while missing this effect.
- **Grade:** B.
- **Sources:** [Toral2019] [Green2013] [SanchezTorron2016]

### 6. Automation bias and anchoring

#### 6.1 Over-acceptance of wrong machine output is robust and hard to train away

- **Finding:** People follow wrong advice from an automated aid more often than they would err alone. The effect appears in novices and experts and is not removed by practice, training or instructions to verify.
- **Evidence:**
  - Goddard, Roudsari and Wyatt 2012: systematic review, 74 studies from 13,821 papers. Pooled from four healthcare studies: wrong advice raised the rate of wrong decisions against no decision support, risk ratio 1.26 (95% CI 1.11 to 1.44). "Negative consultations", where a correct answer was changed to a wrong one after consulting the system, occurred in 6% to 11% of cases across four prospective studies. Mediators: workload, time pressure, task complexity, trust, and low task experience. Mitigators named: training, accountability, the position of advice on screen, updated confidence levels on output, and giving information in place of a recommendation.
  - Parasuraman and Manzey 2010 (review): complacency "is found in both naive and expert participants and cannot be overcome with simple practice"; automation bias "cannot be prevented by training or instructions, and can affect decision making in individuals as well as in teams". Complacency is strongest under multitasking and with constant high reliability, is "largely eliminated when automation reliability is varied over time", and appears even when the automation fails only once in the operator's experience.
  - Goddard cites one study (Madhavan and Wiegmann) placing the reliability at which performance degrades or the aid is abandoned at 70%, and notes that "lower levels of reliability can paradoxically inspire better performance due to lower complacency levels".
- **Caveats:** Most primary studies are aviation and clinical decision support with a discrete right answer. The pooled estimate rests on four studies.
- **Grade:** A for the pooled risk ratio (small), B for the reviews.
- **Sources:** [Goddard2012] [Parasuraman2010]

#### 6.2 How often reviewers miss known errors in AI output

- **Finding:** In the two studies that identified errors in fluent model output beforehand and measured detection, reviewers missed about two thirds of them. In both, the errors were the model's own and none was planted.
- **Evidence:**
  - Biro and colleagues (npj Digital Medicine 2025): 20 practicing primary-care physicians each handled 18 patient messages with a ChatGPT-4 draft reply to edit. Four drafts (22%) contained an error that two physicians had identified beforehand in the model's output: a typo-type inaccuracy, outdated advice, and two potentially harmful omissions. Each erroneous draft was insufficiently corrected by 13 to 15 of the 20 physicians (65% to 75%), and 7 to 9 (35% to 45%) sent it with no edit at all. Mean miss rate 2.67 of 4 (66.6%). One physician caught all four. 80% of the same physicians agreed the drafts reduced cognitive workload and 75% found them safe.
  - Si et al. (NAACL 2024): people verifying claims. With a correct ChatGPT explanation, accuracy 87%. With a wrong one, 35%, against 49% with no aid and 54% with retrieved passages on the same claims. Time spent was the same on right and wrong explanations. Showing arguments for both sides raised accuracy on the wrong cases to 56% and lowered it on the right cases from 87% to 73%.
  - Noy and Zhang: a third of professionals submitted model output untouched (Topic 4.2).
  - Dell'Acqua et al. 2023: on the outside-the-frontier task, AI users were 19 points less likely to reach the right answer.
- **Caveats:** Biro's study is a simulation with 20 physicians and four erroneous drafts, one of which only repeats the patient's own misspelling of a drug name. Si's participants were crowd workers on unfamiliar topics. Neither involves a subject expert reviewing lesson content in their own field.
- **Grade:** B.
- **Sources:** [Biro2025] [Si2024] [Noy2023] [DellAcqua2023]

#### 6.3 Better drafts lower effort, and drafts pull the final text toward themselves

- **Finding:** Reviewers put in less effort as the machine gets better, which can leave the pair worse off than with a weaker machine. Separately, text finished from an AI draft stays close to the draft and makes different authors' outputs more alike.
- **Evidence:**
  - Dell'Acqua 2022: 181 professional recruiters screened 44 resumes with no AI, a 75%-accurate AI, an 85%-accurate AI, or a perfect predictor. Both imperfect AIs helped against no AI. The 75% AI group outperformed the 85% AI group in every specification (significant on the 1 to 10 accuracy scale, p about 0.05), spent more time and made more clicks per resume. The effect was strongest among experienced recruiters. Following the better AI more closely lowered accuracy.
  - Doshi and Hauser 2024: 293 writers, 600 raters. One to five GPT-4 story ideas raised rated novelty 5.4% to 8.1% and usefulness 3.7% to 9.0%, and made stories about 9% to 11% more similar to each other on a similarity scale. Read through secondary summaries.
  - Padmakumar and He 2024: 38 writers, essays written with an instruction-tuned model were significantly more alike than essays written alone; the similarity came from the model's text. Read through secondary summaries.
  - A 2026 meta-analysis of human-AI co-creation: 19 studies, 61 effect sizes, homogenization d = 0.334. The first pass also reports a 95% CI of 0.094 to 0.574 and high heterogeneity (unconfirmed). The citation check could reach the abstract only.
  - Post-editing shows the same pull (Topic 5.3).
  - Tohidi et al. 2006: users shown a single design "give significantly higher ratings and were more reluctant to criticize than when presented with the same design in a group of three".
- **Caveats:** Dell'Acqua 2022 is a working paper on a decision task. The homogenization studies measure creative writing. For a lesson plan, sameness across plans may matter less than a plan inheriting the model's defaults (recall-level activities, generic misconceptions), which no study has measured directly.
- **Grade:** A for the single experiments, B for the meta-analysis (no risk-of-bias assessment).
- **Sources:** [DellAcqua2022] [Doshi2024] [Padmakumar2024] [HomogMeta2026] [Tohidi2006]

#### 6.4 Review size, speed and fatigue

- **Finding:** Reviewers find fewer defects per unit as the amount under review and the review speed rise. The best-known limits come from a vendor study of code review.
- **Evidence:** Cisco study reported by SmartBear, 2,500 reviews of 3.2 million lines by 50 programmers: defects found per line fall sharply above 200 to 400 lines in one sitting and approach zero beyond 400; above about 500 lines an hour a review "becomes a rubber stamp"; performance drops after 60 to 90 minutes; authors who annotate their change first find defects themselves. Google's median reviewed change is 24 lines. In three open-source projects, components whose changes were approved without discussion or reviewed faster than 200 lines an hour had more defects after release. PromptHive's experts reported that hand-authoring quality fell as people tired: "you could see the effort decreasing from like the first question".
- **Caveats:** The Cisco figures are not peer-reviewed and I read them through a secondary handbook page, as with the Google and open-source figures. Code review is an analogy to plan review, not a test of it.
- **Grade:** C.
- **Sources:** [CiscoSmartBear] [Reza2025]

### 7. What makes a draft cheap and safe to correct

#### 7.1 Confidence and uncertainty shown beside the output

- **Finding:** Showing how confident the system is changes when people follow it. It raises accuracy only when the person has knowledge the system lacks. Phrasing uncertainty in the first person lowers blind agreement.
- **Evidence:**
  - Zhang, Liao and Bellamy 2020: 72 crowd workers on a prediction task, AI 75% accurate, humans 65%. Confidence scores increased switching to the AI above 80% confidence (interaction p < .001) and did not improve the accuracy of the final decisions. The authors: trust calibration alone is not sufficient; it depends on "whether the human can bring in enough unique knowledge to complement the AI's errors". Local explanations had no effect on trust or accuracy.
  - Bansal et al. 2021: about 100 crowd workers per condition on three tasks, AI matched to human accuracy. Recommendation plus confidence beat both human and AI alone (for example 0.89 against 0.84 and 0.82 on one task). Adding explanations gave no further gain and "increased the chance that humans will accept the AI's recommendation, regardless of its correctness".
  - Kim et al. 2024: medical yes-or-no questions with a search-style AI answer correct half the time. Agreement with the AI: 80.9% with no uncertainty wording, 74.8% with "I'm not sure, but", 77.6% with impersonal wording. First-person uncertainty raised accuracy when the AI was wrong (43.6% to 52.0%) and lowered confidence in the answer (3.95 to 3.66 of 5). The 43.6% and 52.0% compare answers inside the first-person group: wrong AI answers stated plainly against wrong AI answers worded with doubt. Between the groups, the share of correct answers was 63.9% with no uncertainty wording and 72.8% with first-person wording, and on the questions the AI got wrong it was 33.0% against 47.8%.
  - Vaccaro et al.: across the meta-analysis, including the AI's confidence or an explanation did not change human-AI performance on average.
- **Caveats:** All crowd workers on classification tasks. Model-stated confidence in generated prose is a different object from a classifier's probability, and I did not review its calibration.
- **Grade:** A (lab RCTs).
- **Sources:** [Zhang2020] [Bansal2021] [Kim2024] [Vaccaro2024]

#### 7.2 Explanations, checking cost and cognitive forcing

- **Finding:** People check an AI output when checking is cheaper than the alternative and skip it otherwise. Interface designs that force a moment of independent thought reduce over-reliance and are disliked.
- **Evidence:**
  - Vasconcelos et al. 2023: five maze studies, N = 731. Over-reliance fell with explanations when the task was hard and the explanation made verification easy, fell when the explanation was easier to read, and fell when the reward for accuracy rose. The authors: earlier null results arise from "the explanation not sufficiently reducing the costs of verifying the AI's prediction".
  - Bucinca, Malaya and Gajos 2021: N = 199, simulated AI 75% accurate. Three cognitive forcing designs (one of them shows the AI answer only on request) against plain explanations and against no AI. On items where the AI was wrong, forcing produced significantly more correct decisions and, for one sub-task, significantly less over-reliance. Over-reliance was not eliminated, and people with no AI still did best on those items. Participants rated the designs that worked best as the least preferred. High need-for-cognition participants benefited more.
  - Si et al.: retrieved source passages gave 54% accuracy on the cases where the model was wrong, against 35% for a model explanation.
- **Caveats:** Lab tasks with crowd workers. Forcing functions tested on single decisions, not on reviewing a document.
- **Grade:** A (lab RCTs).
- **Sources:** [Vasconcelos2023] [Bucinca2021] [Si2024]

#### 7.3 Provenance and citations beside each claim

- **Finding:** Reviewers ask for sources and do better with source passages than with model explanations. Model-attached citations are themselves wrong often enough to need checking.
- **Evidence:** Liu, Zhang and Liang 2023 audited four generative search engines by hand: 51.5% of generated sentences were fully supported by their citations and 74.5% of citations supported their sentence. STORM's editors scored verifiability lower where the article connected sourced facts in unsupported ways. LessonPlanner's teachers asked for "clear sources" for generated examples. STORM's reviewing tool showed the article "side-by-side" with its citation snippets. Oak grounds generation in its own corpus and reports a cited accuracy gain from 67% to 92% with curated retrieval.
- **Caveats:** The citation audit is of 2023 consumer search products. I found no experiment that varies citation display and measures a reviewer's error catch rate on instructional content.
- **Grade:** B for the audit, C for the practice reports.
- **Sources:** [Liu2023] [Shao2024] [Fan2024] [OakAutoEval2025] [Si2024]

#### 7.4 Small units, alternatives, checklists and flagged questions

- **Finding:** Evidence favors small review units and comparison among alternatives. Checklist evidence is mixed: large effects in the hospitals that piloted the surgical checklist, a flat result where it was mandated province-wide, and no robust overall effect in a code-review experiment with mostly novice reviewers.
- **Evidence:**
  - Small units: Topic 6.4. Google reviews changes with a median of 24 lines.
  - Alternatives: Tohidi et al. 2006 (single design rated higher and criticized less than the same design among three). Dow et al. 2010: designers who made several prototypes in parallel before feedback produced ads with more clicks and higher expert ratings than designers who iterated on one. LessonPlanner's teachers said choosing among varied generated content took effort and was worth it: "Before, I could only stick to one basic idea." Si et al.'s two-sided explanations helped on wrong cases and hurt on right ones.
  - Checklists: Haynes et al. 2009, eight hospitals, 3,733 patients before and 3,955 after a 19-item checklist: deaths 1.5% to 0.8% (p = 0.003), complications 11.0% to 7.0% (p < 0.001). Urbach et al. 2014, 101 Ontario hospitals under a mandate, over 100,000 procedures before and after: mortality 0.71% to 0.65% (p = 0.13), complications 3.86% to 3.82% (p = 0.29). A stepped-wedge trial in two hospitals (Haugen et al. 2015) found complications falling from 19.9% to 11.5%. Goncalves et al. 2022, about 70 developers randomized to ad hoc review, a checklist, or a guided checklist: defect-finding was low in every group and the study found no robust overall benefit, with some advantage for the checklist on the most complex change.
  - Editable output: in a factorial survey on Amershi's guidelines, 56 of 60 respondents (93%) preferred the product whose AI suggestion could be edited.
  - Flagged open questions: Oak plans for its system to "suggest specific areas for users to check carefully". Horvitz's principles call for dialog to resolve key uncertainties and for scoping the service down when uncertain. I found no measurement of whether flagging raises the reviewer's catch rate.
- **Caveats:** Haynes is before-and-after with observers present. The Ontario data are administrative. Bucinca et al. list checklists among cognitive forcing functions, which ties this to Topic 7.2.
- **Grade:** B.
- **Sources:** [Tohidi2006] [Dow2010] [Fan2024] [Haynes2009] [Urbach2014] [ChecklistTrial2015] [Goncalves2022] [AmershiSurvey2023] [OakAutoEval2025] [Horvitz1999]

#### 7.5 Design guidelines

- **Finding:** The two standard guideline sets both treat the AI's output as a starting point the user refines, and both say to do less when unsure.
- **Evidence:**
  - Amershi et al. 2019: 18 guidelines, tested by 49 practitioners against 20 products (785 examples found: 313 applications, 277 violations). Relevant ones: G9 "Support efficient correction. Make it easy to edit, refine, or recover when the AI system is wrong." G10 "Scope services when in doubt." G11 "Make clear why the system did what it did." G8 "Support efficient dismissal."
  - Horvitz 1999, twelve principles, including: "Employing dialog to resolve key uncertainties", weighed against "the costs of potentially bothering a user needlessly"; "Minimizing the cost of poor guesses"; "Scoping precision of service to match uncertainty", with "a preference for 'doing less' but doing it correctly under uncertainty" to "minimize the need for costly undoing or backtracking"; and "Providing mechanisms for efficient agent-user collaboration to refine results", designing "with the assumption that users may often wish to complete or refine an analysis provided by an agent."
- **Caveats:** Guidelines validated for relevance and clarity, not for outcome effects.
- **Grade:** D.
- **Sources:** [Amershi2019] [Horvitz1999]

### 8. Authoring cost baselines

- **Finding:** Published baselines run from about 40 hours of development per hour of classroom teaching to about 200 to 300 hours per hour of intelligent tutoring. The measured reductions from model drafting with expert review are in the range of 4 to 30 times for specific components. No study reports hours per finished hour for a complete model-drafted, expert-finished adaptive lesson.
- **Evidence:**
  - Intelligent tutoring systems: Aleven, McLaren, Sewall and Koedinger 2009 take as baseline "development time estimates reported in the literature, which for ITSs range from 200:1 (Woolf & Cunningham, 1987) to 300:1 (Murray, 2003)", and estimate the Algebra and Geometry Cognitive Tutors at about 200 hours per instructional hour. With CTAT example-tracing tutors, larger projects came in at 107:1, 87:1, 85:1 and 48:1, "3-4 times better than the historical averages", and 4 to 8 times more cost-effective counting cheaper non-programmer authors. Projects of an hour or less did not beat the baseline. The authors call these "rough estimates" from retrospective accounting.
  - E-learning and classroom training: Chapman Alliance 2010 survey (249 organizations, 3,947 developers): instructor-led 43:1 (range 22 to 82), basic e-learning 79:1 (49 to 125), interactive e-learning 184:1 (127 to 267); a secondary page gives 490:1 for advanced simulation-style e-learning. Kapp and Defelice's 2009 survey (47 respondents): classroom 43 to 185 hours, e-learning without a template 93 to 152 (text only), 122 to 186 (moderate interactivity), 154 to 243 (high interactivity).
  - Measured with model drafting:
    - 100 exam questions: 24.5 person-hours with ChatGPT-4o drafting and expert review against 96 person-hours by the usual expert process. The drafting itself took 2 hours against 71. Review and revision make up the rest. [HKMCQ2025]
    - Hints for a college algebra textbook: authoring time cut "by a factor of 30", from "several months to just a few hours", plus 3 to 6 hours each for two authors to fix format issues. [Reza2025]
    - A full university course package (objectives, syllabus, assessments, slides, scripts): 2.2 to 4.7 hours of model run time, 22 to 36 US cents of compute, 0 to 45 minutes of faculty time, with ratings that "reflect estimated instructor revision effort before classroom deployment" between 2.7 and 4.0 of 5. [Yao2025]
    - Teacher lesson preparation: 31% and 24% less time in the two trials (Topic 1.1).
- **Caveats:** The baseline surveys are self-report, old, and count whole teams. The Hong Kong comparison is one exam. PromptHive's "months" is the authors' account of earlier manual work, not a timed control. Instructional Agents does not report how long faculty then took to bring materials to classroom standard.
- **Grade:** C for baselines, B for the Hong Kong timing, C for the others.
- **Sources:** [Aleven2009] [Chapman2010] [KappDefelice2009] [HKMCQ2025] [Reza2025] [Yao2025] [EEF2024] [NFERAila2026]

### 9. Teachers always adapt materials

- **Finding:** Teachers read, select, adapt and replace what a curriculum offers as a matter of course. Adaptation that keeps each activity's purpose can improve results, and adaptation that strips the purpose out destroys them. Materials that explain their own reasoning are the proposed way to steer adaptation.
- **Evidence:**
  - Remillard 2005 (review of research on mathematics curriculum use): studies show teachers "read, make meaning of, evaluate, adopt, adapt, and replace the offerings of the curriculum"; teaching is "a responsive and improvisational activity that cannot be scripted"; and curriculum use is "a participatory relationship between the teacher and the curriculum" in which "features of the curriculum matter to curriculum use as much as characteristics of the teacher".
  - Davis and Krajcik 2005: educative curriculum materials should "speak to" teachers "about the ideas underlying the tasks rather than merely guiding their actions", making "visible the developers' pedagogical judgments". Each of their nine design heuristics has three parts: what the materials provide, the rationale, and how the teacher can use and adapt the idea. They use "heuristics" on purpose because the recommendations lack "a level of empirical testing".
  - Kim et al. 2017: cluster-randomized trial, 27 high-poverty elementary schools, summer literacy program delivered as written against the same program with structured teacher adaptations (grade-level teams, an improvement goal, local data). The adaptation group scored 0.12 SD higher on reading comprehension. Adherence to essential lesson elements was 87% in the core group and 83% in the adaptation group. Adaptations mostly extended procedures and raised family and student engagement.
  - Lethal mutations: Ann Brown's term, reported through secondary sources, for adaptations that no longer capture "the pedagogical essence of the intervention". Examples given by the University of Pittsburgh's Institute for Learning: a bridging or closing task skipped for time, student sense-making lost "because too much scaffolding is given", a hands-on experience dropped. The same source proposes "integrity" in place of fidelity: an adaptation holds integrity "if [it] also serve[s] the same function as that which was adapted".
  - Malik et al. 2024: six expert teachers' adaptations followed three stages (observe students, form a strategy, implement).
  - Both EEF trials: teachers used AI output for parts of lessons and adapted it; Aila outputs "often required adaptation before being classroom ready".
- **Caveats:** This literature is about school teachers adapting published curricula for classes. Kim's adaptations were structured and supported, which is the condition under which they helped. The lethal-mutation examples are practitioner reports.
- **Grade:** A for Kim 2017, B for Remillard's review, D for the design heuristics, C for the lethal-mutation accounts.
- **Sources:** [Remillard2005] [DavisKrajcik2005] [Kim2017] [IFL2018] [EarlyMath2023] [Malik2024] [EEF2024] [NFERAila2026]

### 10. Building a plan from expert interviews

- **Finding:** Direct evidence is thin. One study shows that findings from structured expert interviews, written into the prompt alongside the source curriculum, produce content teachers prefer to expert-written content. Two others show that a model interviewing or analyzing transcripts by itself captures the basic structure and misses part of the expert's knowledge.
- **Evidence:**
  - Malik, Abdi, Wang and Demszky 2024: cognitive task analysis interviews with 6 expert mathematics teachers (10 or more years' experience) about how and why they changed curriculum materials, structured around what student responses they expected, how they wanted to respond, and why. Three model conditions built from the findings. Best: original curriculum warm-up plus expert-informed prompt (preferred to expert warm-ups by 0.545 on a -2 to +2 scale). Adding the individual expert's own commentary for that lesson did no better than the curriculum context alone (overall 0.410, not significantly different).
  - Johnson 2025 (conference review of AI-assisted cognitive task analysis) reports Van den Bent et al. 2025: AI-led interviews "were more time-efficient but captured less information overall. The ontologies built from AI-led interviews were missing approximately 20-30% of the classes that appeared in the human-led interview ontologies." The AI-built ontologies had more consistent taxonomy. The review concludes hybrid human-AI methods are the most promising and lists automation bias and expert disengagement as risks.
  - An ASEE 2024 paper used GPT-4 to extract entities and relationships from interviews with 28 engineering faculty: the model and a human analyst each found relationships the other missed, the model "tended to identify more basic relationships, while manual analysis identified more nuanced relationships", and the authors did not support fully automatic use.
  - PromptHive (Topic 2.5) is the nearest deployed pattern: experts' knowledge enters as prompts they write and test against many problems at once, and hints generated that way matched hand-written hints on learning.
  - Stream 1 supplies the reason the interview matters: experts omit about 70% of what they do when describing it unaided, and training built from cognitive task analysis outperforms training built from expert self-report (g = 0.871).
- **Caveats:** Malik's result is for one small artifact. The Van den Bent figures are secondhand. I found no study that turns interview transcripts into a full lesson plan and has experts rate the result part by part.
- **Grade:** B for Malik, C for the others.
- **Sources:** [Malik2024] [Johnson2025] [ASEE2024] [Reza2025]

### 11. Is there evidence for a number like 80 percent?

- **Finding:** I found no study that measures how complete or correct an AI draft must be before finishing it beats starting from scratch, for lesson plans or any instructional content. Four adjacent bodies of evidence give numbers, and they do not agree on a single figure because they measure different things.
- **Evidence:**
  - Break-even for editing against rewriting (translation): about a 75% match between draft and final text; productivity loss below 70%; full rate paid below 75% by industry convention. [Parra2015] [HansenSchirraChapter]
  - Reliability at which operators stop benefiting from an aid: about 70% in one study cited by a systematic review. [Goddard2012]
  - Reliability at which reviewers start to coast: recruiters with an 85%-accurate aid did worse than recruiters with a 75%-accurate aid. [DellAcqua2022]
  - Share of AI output that survives expert review, by stage, in deployed or studied systems: 82% of plans approved and about 8% of work items ending in merged code [HULA2024]; 74% to 95% of tutor messages approved unchanged in Stream 5's sources; 56% and 35% of skill tags matching the expert's [Moore2024KC]; 37% of misconception-distractor sets valid [Lee2024]; 94% of hard-error-free exam items (6% factual errors) with 37% carrying some item-writing flaw [HKMCQ2025].
  - Whether the human's finishing adds value depends on who is better alone: +0.46 when the human is, -0.54 when the AI is. [Vaccaro2024]
- **Caveats:** A "percent complete" for a lesson plan is undefined in this literature. The translation threshold is a surface-overlap measure. Acceptance rates measure the reviewer's behavior as well as the draft's quality, and Topic 6 shows reviewers accept wrong output.
- **Grade:** D for any specific percentage applied to lesson plans. The component numbers keep their own grades.
- **Sources:** [Parra2015] [Goddard2012] [DellAcqua2022] [HULA2024] [Moore2024KC] [Lee2024] [HKMCQ2025] [Vaccaro2024]

## Disagreements and open questions

1. **Does a human finishing an AI draft beat the AI alone?** The evidence does not support "always".
   - Against: the meta-analysis finds the pair worse than the better member on average (g = -0.23) and worse still when the AI alone is the stronger member (g = -0.54) [Vaccaro2024]. Professionals who edited ChatGPT's text scored no higher than the raw text [Noy2023]. Teachers rated model-written warm-ups above expert-written ones [Malik2024], and experts preferred model skill tags and course-level objectives over the human originals [Moore2024KC] [Doyle2025].
   - For: the pair gains when the human is the stronger member (g = +0.46) and probably gains on creation tasks (g = +0.19, uncertain) [Vaccaro2024]. Ratings rose with every added level of human involvement in one course-generation system [Yao2025]. The best-rated plans in Hu's study began as a human draft [Hu2025]. Experts clearly beat the model on misconceptions and distractors [Scarlatos2024] [Lee2024].
   - What reconciles them: the human adds value on the parts where the human is better than the model, and subtracts or adds nothing elsewhere. The "human-written" comparison sets in several studies were ordinary, unscreened work, which flatters the model.
2. **How much time is saved?** Measured in randomized trials of teachers: 24% to 31% of preparation time, or 25 to 49 minutes a week [EEF2024] [NFERAila2026]. Self-reported by weekly users: 5.9 hours a week [Gallup2025]. For a component with expert review: 4 times (exam questions) [HKMCQ2025] to a claimed 30 times (hints) [Reza2025]. For experts on their own mature work: 19% slower in early 2025 [METR2025], direction reversed and unmeasurable in early 2026 [METR2026]. Developers in the METR trial believed they had been sped up by 20% while being slowed by 19%, so self-report is unreliable in both directions.
3. **Does a better draft give a better result?** Translation says yes, monotonically: each quality point cuts editing time and the worst system gave the worst final text [SanchezTorron2016] [Parra2015]. The recruiter experiment says a better aid can give a worse result because reviewers coast [DellAcqua2022], and the automation literature agrees that constant high reliability breeds complacency [Parasuraman2010]. The two sets of studies differ in what the human must do: a translator has to produce every sentence, while a screener can simply accept.
4. **Do explanations and confidence displays help the reviewer?** No average effect across the meta-analysis [Vaccaro2024]; explanations raise acceptance of wrong output [Bansal2021]; confidence calibrates trust with no accuracy gain [Zhang2020]. Against that, explanations help when they make checking cheaper than doing [Vasconcelos2023], source passages beat explanations [Si2024], and first-person uncertainty wording lowers blind agreement [Kim2024].
5. **Do checklists work?** Deaths nearly halved in the eight-hospital pilot [Haynes2009]; no significant change under a province-wide mandate [Urbach2014]; no robust effect in a code-review experiment [Goncalves2022]. The pilot hospitals chose to adopt the checklist and were observed, and the mandate was compliance-driven.
6. **Does self-critique work?** About 20 points better on open-ended tasks [Madaan2023]; flat or harmful on reasoning without an external check [Huang2024]. Education-specific loops report large gains [Hu2025] [EduPlanner2025], and one of them is scored by its own judge.
7. **Do multiple agents help?** A role-agent pipeline beat a single agent in one builder-run ablation (2.33 against roughly 3) [Yao2025]. Independent work finds multi-agent frameworks failing 41% to 86.7% of tasks and debate adding nothing beyond voting [Cemri2025] [DebateOrVote2025] [Huang2024].
8. **Is the lower-order bias in generated plans and questions a model limit or a prompt gap?** Two studies note they never asked for higher-order work [HKMCQ2025] [Trust2025]. Objectives prompted with explicit Bloom levels came out at the right level [Sridhar2023]. Yet asking for a "highly interactive" plan did not lift cognitive demand [Trust2025], and plans drifted away from a requested inquiry focus [Grossmann2025].
9. **Does a stepwise, high-involvement design get used?** Aila's section-by-section flow is what the review literature would favor, and teachers in its trial used it for 7% of lessons, used it less over time, and often preferred easier tools [NFERAila2026]. Cognitive forcing designs show the same trade: the designs that reduce over-reliance most are liked least [Bucinca2021].
10. **Open questions I found no study on:**
    - The error-catch rate of a subject expert reviewing an AI-drafted lesson plan in their own field, with and without any of the review aids in Topic 7.
    - Any experiment that varies where the human gate sits in a generation pipeline.
    - Any definition or measurement of "percent complete" for a lesson plan.
    - The share of model-proposed prerequisite links, parallel check tasks, and gate assignments that experts accept. Stream 1 has data on parallel-item difficulty, none on expert acceptance.
    - Whether the model's stated confidence about its own generated lesson content is calibrated enough to direct a reviewer's attention.
    - Any of the above for adult professional learners or software-tool content. Nearly all the education evidence is school and university teaching, and medical exams.
    - A full lesson plan built from expert-interview transcripts and rated part by part.

## What the sources themselves recommend

These are the sources' own prescriptions, attributed. This file adds none of its own.

**On using model-drafted lesson content**

- The EEF trial's teacher guide "recommended checking materials carefully before sharing them with students", because models "may hallucinate incorrect facts or made-up sources". [EEF2024]
- "We recommend that teachers critically evaluate and revise AI generated lesson plans to provide more expansive, creative, and critical learning experiences." [Trust2025]
- Models "demonstrate potential as preliminary planning tools" and "necessitate thorough evaluation and substantive modification by teachers possessing robust pedagogical content knowledge". [Grossmann2025]
- "A hybrid AI-human framework is recommended, where AI handles the initial question generation and educators focus on review, refinement, and validation", with "regular feedback loops", prompts that ask for higher-order and case-based items, and a separate fact-checking tool. [HKMCQ2025]
- "We strongly recommend that educators and teachers review the quality of LLM-generated distractors thoroughly before deploying them." Over-generate candidates and rank them with a model trained on real student choices. [Scarlatos2024]
- Supply the model with the original curriculum material plus a prompt built from expert practice. [Malik2024]
- Use AI for ideas about the plan as well as for materials; output-only use made teachers feel they "got it done" more than "got it right". [Keppler2024, wording from a secondary summary]
- Integrate teacher assistants with the adopted curriculum, and add friction before generated content reaches students. [CSM2025]

**On building the generator**

- Ground generation in a curated corpus, anchor to a chosen exemplar, codify the pedagogy in the prompt with examples and non-examples, keep "decision-making by the teacher at a granular level", and evaluate every output automatically against explicit benchmarks. Improve the judge by coding expert justifications into its prompt with few-shot examples. [OakAutoEval2025] [OakATRS2024]
- Use prompt chaining and one-shot examples; one combined prompt gave inconsistent output. [Jury2024] Prefer chained draft, critique and refine calls to a single stepwise prompt. [Sun2024]
- Separate the pre-writing stage (research, outline) from writing; add a step that sifts sources for bias before writing. [Shao2024]
- Give self-correction an external signal; put every requirement in the first prompt; compare any multi-call method with simple repeated sampling at equal cost. [Huang2024] [Madaan2023]
- "Find the simplest solution possible, and only increase complexity when needed." Use prompt chaining with programmatic checks between steps for tasks that decompose cleanly; use an evaluator-optimizer loop "when we have clear evaluation criteria"; show the agent's planning steps; pause "for human feedback at checkpoints". [Anthropic2024, vendor guidance]
- Treat multi-agent failures as design problems: tighten specifications, agent roles and verification. [Cemri2025]
- Rely on human ratings for quality claims; model reviewers cluster their scores. [Yao2025]
- Have subject experts write and iterate the prompts themselves, seeing output across many problems at once. [Reza2025]
- Let a human refine the plan before code is generated, and review the code after. [HULA2024]

**On the human review step**

- Mitigate automation bias through training, accountability for the decision, the position of advice on screen, updated confidence levels on the output, and giving information in place of a recommendation. [Goddard2012]
- Vary the reliability the operator experiences, and train attention strategies; simple practice does not remove complacency. [Parasuraman2010]
- Reduce over-reliance by making people commit effort before or while seeing the AI output, and expect users to dislike it; fit the intervention to the user's motivation. [Bucinca2021]
- Make verification cheaper than doing the task, or the reviewer will skip it. [Vasconcelos2023]
- Prefer showing retrieved source passages to showing a model explanation for high-stakes checking; contrastive (both-sides) explanations help when the model is wrong and hurt when it is right. [Si2024]
- Express uncertainty in the first person where the system is unsure. [Kim2024]
- "Support efficient correction"; "scope services when in doubt"; "make clear why the system did what it did". [Amershi2019]
- Use dialog to resolve key uncertainties, weigh the cost of bothering the user, and prefer "doing less but doing it correctly under uncertainty". [Horvitz1999]
- Present alternatives: a single design draws inflated ratings and less criticism. [Tohidi2006] Develop several options in parallel before seeking feedback. [Dow2010]
- Review at most 200 to 400 lines at a time, at under about 500 lines an hour, for at most 60 to 90 minutes, and have the author annotate the change first. [CiscoSmartBear, vendor study via secondary page]
- Improve "design, training, and error-detection mechanisms" for AI drafting tools, since physicians who missed errors still rated the drafts safe. [Biro2025]
- Shift research and design effort from explanations and confidence displays to "the baseline performance of the human and AI alone, the type of task they perform, and the division of labour between them". [Vaccaro2024]
- In translation, highlight what the draft changed or where it is doubtful: translators edited memory matches faster than machine output needing the same edits because the tool showed the differences. [Parra2015]

**On materials meant to be adapted**

- Curriculum materials should make "visible the developers' pedagogical judgments" and give the rationale behind each recommendation so teachers can adapt with understanding. [DavisKrajcik2005]
- Judge adaptations by integrity (does the changed activity still serve the function of the original) in place of fidelity to the script. [IFL2018]
- Give teachers structured opportunities to adapt, around a stated goal and local data. [Kim2017]
- Study and design for the teacher-curriculum relationship; features of the materials shape use as much as the teacher does. [Remillard2005]
- Combine human-led and AI-assisted knowledge elicitation; guard against automation bias and expert disengagement. [Johnson2025]

## Source list

Core sources (read in depth)

- [EEF2024] Roy, P., Poet, H., Staunton, R., Aston, K., Thomas, D. (2024). ChatGPT in Lesson Preparation: A Teacher Choices Trial. Evaluation report, NFER for the Education Endowment Foundation, December 2024. https://d2tic4wvo1iusb.cloudfront.net/production/documents/projects/chatgpt_in_lesson_planning_-_evaluation_report.pdf?v=1737473227. Summary: https://www.nfer.ac.uk/media/fn0p4nvb/nfer-classroom-chatgpt-for-lesson-planning-summary.pdf. Project page: https://educationendowmentfoundation.org.uk/projects-and-evaluation/projects/choices-in-edtech-using-generative-ai-chatgpt-for-ks3-science-lesson-preparation-2024-teacher-choices-trial. Read: full text.
- [OakAutoEval2025] Clark, H.-B., Dowland, M., Benton, L., Budai, R., Keskin, I. K., Searle, E., Gregory, M., Hodierne, M., Gayne, W., Roberts, J. (2025; Oak National Academy). Auto-Evaluation: A Critical Measure in Driving Improvements in Quality and Safety of AI-Generated Lesson Resources. arXiv preprint 2502.10410. https://arxiv.org/pdf/2502.10410. Short version, titled "Improving Quality and Safety in AI-Generated Lessons": https://ceur-ws.org/Vol-3994/short2.pdf. Read: full text.
- [Trust2025] Trust, T., Maloy, R., Xu, C., Pelletier, K. (2025). Civic education in the age of AI: Should we trust AI-generated lesson plans? Contemporary Issues in Technology and Teacher Education, 25(3), 418-442. https://citejournal.org/wp-content/uploads/2025/08/v25i3Socialstudies2.pdf. Read: full text.
- [Fan2024] Fan, H., Chen, G., Wang, X., Peng, Z. (2024). LessonPlanner: Assisting Novice Teachers to Prepare Pedagogy-Driven Lesson Plans with Large Language Models. UIST 2024. https://dl.acm.org/doi/fullHtml/10.1145/3654777.3676390. Open access: https://arxiv.org/abs/2408.01102. Read: full text.
- [Yao2025] Yao, H., Xu, W., Turnau, J., Kellam, N., Wei, H. (2025). Instructional Agents: Reducing Teaching Faculty Workload through Multi-Agent Instructional Design. arXiv preprint 2508.19611. https://arxiv.org/abs/2508.19611. Read: full text.
- [Malik2024] Malik, R., Abdi, D., Wang, R., Demszky, D. (2024). Scaffolding Middle-School Mathematics Curricula With Large Language Models. EdWorkingPaper 24-1028, Annenberg Institute (working paper). https://doi.org/10.26300/b47y-mh41. Open access (August 2024 version): https://files.eric.ed.gov/fulltext/ED660105.pdf. The EdWorkingPapers page now serves an April 2025 version with the same comparison results. Read: full text.
- [HKMCQ2025] Law, A. K. K., So, J., Lui, C. T., Choi, Y. F., Cheung, K. H., Hung, K. K., Graham, C. A. (2025). AI versus human-generated multiple-choice questions for medical education: a cohort study in a high-stakes examination. BMC Medical Education. https://doi.org/10.1186/s12909-025-06796-6. Open access: https://pmc.ncbi.nlm.nih.gov/articles/PMC11806894/. Read: full text.
- [Feng2024] Feng, W., Lee, J., McNichols, H., Scarlatos, A., Smith, D., Woodhead, S., Otero Ornelas, N., Lan, A. (2024). Exploring Automated Distractor Generation for Math Multiple-choice Questions via Large Language Models. Findings of NAACL 2024, 3067-3082. https://aclanthology.org/2024.findings-naacl.193/. Open access: https://arxiv.org/abs/2404.02124. Read: full text.
- [Scarlatos2024] Scarlatos, A., Feng, W., Smith, D., Woodhead, S., Lan, A. (2024). Improving Automated Distractor Generation for Math Multiple-choice Questions with Overgenerate-and-rank. BEA Workshop 2024, 222-231. https://aclanthology.org/2024.bea-1.19/. Open access: https://arxiv.org/abs/2405.05144. Read: full text.
- [Shao2024] Shao, Y., Jiang, Y., Kanell, T. A., Xu, P., Khattab, O., Lam, M. S. (2024). Assisting in Writing Wikipedia-like Articles From Scratch with Large Language Models (STORM). NAACL 2024, 6252-6278. https://aclanthology.org/2024.naacl-long.347/. Open access: https://arxiv.org/abs/2402.14207. Read: full text.
- [Noy2023] Noy, S., Zhang, W. (2023). Experimental evidence on the productivity effects of generative artificial intelligence. Science, 381(6654), 187-192. https://www.science.org/doi/10.1126/science.adh2586. Open access: https://shakkednoy.com/noy-zhang.pdf. Read: full text.
- [Vaccaro2024] Vaccaro, M., Almaatouq, A., Malone, T. (2024). When combinations of humans and AI are useful: A systematic review and meta-analysis. Nature Human Behaviour, 8(12), 2293-2303. https://www.nature.com/articles/s41562-024-02024-1. Read: full text.
- [DellAcqua2023] Dell'Acqua, F., McFowland, E., Mollick, E., Lifshitz, H., Kellogg, K. C., Rajendran, S., Krayer, L., Candelon, F., Lakhani, K. R. (working paper 2023; journal version 2026). Navigating the Jagged Technological Frontier: Field Experimental Evidence of the Effects of Artificial Intelligence on Knowledge Worker Productivity and Quality. Organization Science, 37(2), 403-423. https://pubsonline.informs.org/doi/full/10.1287/orsc.2025.21838. Working paper: https://papers.ssrn.com/sol3/papers.cfm?abstract_id=4573321&stream=top. Open copy of the working paper: https://mitsloan.mit.edu/sites/default/files/2023-10/SSRN-id4573321.pdf. Read: full text (journal version).
- [DellAcqua2022] Dell'Acqua, F. (2022). Falling Asleep at the Wheel: Human/AI Collaboration in a Field Experiment on HR Recruiters. Working paper. Hosted copies: https://www.almendron.com/tribuna/wp-content/uploads/2023/09/falling-asleep-at-the-whee.pdf and https://anacanhoto.com/wp-content/uploads/2024/08/554ee-fallingasleepatthewheel-fabriziodellacqua.pdf. Read: full text.
- [Biro2025] Biro, J. M., Handley, J. L., McCurry, J. M., Visconti, A., Weinfeld, J., Trafton, J. G., Ratwani, R. M. (2025). Opportunities and risks of artificial intelligence in patient portal messaging in primary care. npj Digital Medicine, 8:222. https://www.nature.com/articles/s41746-025-01586-2. Open access: https://pmc.ncbi.nlm.nih.gov/articles/PMC12022076/. Read: full text.
- [Parra2015] Parra Escartin, C., Arcedillo, M. (2015). Living on the edge: productivity gain thresholds in machine translation evaluation metrics. 4th Workshop on Post-Editing Technology and Practice, MT Summit XV. https://aclanthology.org/2015.mtsummit-wptp.4.pdf. Companion paper: https://aclanthology.org/2015.mtsummit-papers.11.pdf. Read: full text.
- [Goddard2012] Goddard, K., Roudsari, A., Wyatt, J. C. (2012). Automation bias: a systematic review of frequency, effect mediators, and mitigators. Journal of the American Medical Informatics Association, 19(1), 121-127. https://doi.org/10.1136/amiajnl-2011-000089. Open access: https://pmc.ncbi.nlm.nih.gov/articles/PMC3240751/. Read: full text.

Supporting research sources

- [NFERAila2026] Roy, P., Poet, H., Staunton, R., Aston, K., Thomas, D., Styles, B. (2026). Lesson planning using AI lesson assistant, Aila - A Teacher Choices Trial. NFER for the Education Endowment Foundation. https://www.nfer.ac.uk/publications/lesson-planning-using-ai-lesson-assistant-aila-a-teacher-choices-trial. Read: excerpts (publication summary page; full report not opened). The citation check read a hosted copy of the full evaluation report (October 2026): https://gsa.uk.com/wp-content/uploads/2026/10/aila-lesson-planning_teacher-choices_evaluation-report.pdf.
- [Grossmann2025] Grossmann, L., Koberstein-Schwarz, M., Kruger, D., Krell, M. (2025). Lesson planning with ChatGPT for inquiry-based biology instruction - A(I) roll of the dice? International Journal of Science Education, 48(15), 2356-2375 (online October 2025). https://www.tandfonline.com/doi/full/10.1080/09500693.2025.2567509. Read: excerpts.
- [Powell2024] Powell, W., Courchesne, S. (2024). Opportunities and risks involved in using ChatGPT to create first grade science lesson plans. PLOS ONE. https://journals.plos.org/plosone/article?id=10.1371%2Fjournal.pone.0305337. Read: excerpts.
- [Hu2025] Hu, B., Zhu, J., Pei, Y., Gu, X. (2025). Exploring the potential of LLM to enhance teaching plans through teaching simulation. npj Science of Learning. https://www.nature.com/articles/s41539-025-00300-x. Read: full text.
- [Hu2024] Hu, B., Zheng, L., Zhu, J., Ding, L., Wang, Y., Gu, X. (2024). Teaching Plan Generation and Evaluation With GPT-4: Unleashing the Potential of LLM in Instructional Design. IEEE Transactions on Learning Technologies. https://doi.org/10.1109/tlt.2024.3384765. Read: abstract only.
- [EduPlanner2025] Zhang, X., Zhang, C., Sun, J., Xiao, J., Yang, Y., Luo, Y. (2025). EduPlanner: LLM-Based Multi-Agent Systems for Customized and Intelligent Instructional Design (the journal prints "Multiagent"). IEEE Transactions on Learning Technologies, 18, 416-427. https://doi.org/10.1109/tlt.2025.3561332. Open access: https://arxiv.org/abs/2504.05370. Read: full text.
- [Sridhar2023] Sridhar, P., Doyle, A., Agarwal, A., Bogart, C., Savelka, J., Sakr, M. (2023). Harnessing LLMs in Curricular Design: Using GPT-4 to Support Authoring of Learning Objectives. CEUR Workshop Proceedings Vol-3487. https://ceur-ws.org/Vol-3487/paper9.pdf. Open access: https://arxiv.org/abs/2306.17459. Read: excerpts.
- [Doyle2025] Doyle, A., Sridhar, P., Agarwal, A., Savelka, J., Sakr, M. (2025). A comparative study of AI-generated and human-crafted learning objectives in computing education. Journal of Computer Assisted Learning, 41(1), e13092. https://doi.org/10.1111/jcal.13092. Read: full text.
- [Moore2024KC] Moore, S., Schmucker, R., Mitchell, T., Stamper, J. (2024). Automated Generation and Tagging of Knowledge Components from Multiple-Choice Questions. Learning @ Scale 2024. https://doi.org/10.1145/3657604.3662030. Open access: https://arxiv.org/abs/2405.20526. Read: full text.
- [Moore2023] Moore, S., Nguyen, H. A., Chen, T., Stamper, J. (2023). Assessing the Quality of Multiple-Choice Questions Using GPT-4 and Rule-Based Methods. EC-TEL 2023. https://arxiv.org/abs/2307.08161. Author copy: https://stevenjamesmoore.com/assets/papers/ectel23_full_moore.pdf. Read: excerpts.
- [SAQUET2024] Moore, S., Costello, E., Nguyen, H. A., Stamper, J. (2024). An Automatic Question Usability Evaluation Toolkit. AIED 2024 (LNCS, 31-46); arXiv preprint 2405.20529. https://arxiv.org/abs/2405.20529. Read: excerpts.
- [Doughty2024] Doughty, J., Wan, Z., Bompelli, A., and 12 others (2024). A Comparative Study of AI-Generated (GPT-4) and Human-crafted MCQs in Programming Education. ACE 2024, 114-123. https://doi.org/10.1145/3636243.3636256. Open access: https://arxiv.org/html/2312.03173. Read: full text.
- [EMPsych2025] Kaya, M., Sonmez, E., Halici, A., Yildirim, H., Coskun, A. (2025). Comparison of AI-generated and clinician-designed multiple-choice questions in emergency medicine exam: a psychometric analysis. BMC Medical Education. https://link.springer.com/article/10.1186/s12909-025-07528-6. Read: abstract only.
- [AcadMed2024] Laupichler, M. C., Rother, J. F., Grunwald Kadow, I. C., Ahmadi, S., Raupach, T. (2024). Large Language Models in Medical Education: Comparing ChatGPT- to Human-Generated Exam Questions. Academic Medicine, 99(5), 508-512. https://www.ingentaconnect.com/content/wk/acm/2024/00000099/00000005/art00014. DOI: https://doi.org/10.1097/ACM.0000000000005626. Read: abstract only.
- [MedTeach2025] Wu, H., Lee, D., Zerner, T., Court-Kowalski, S., Devitt, P., Palmer, E. (2025). A comparison of the psychometric properties of GPT-4 versus human novice and expert authors of clinically complex MCQs in a mock examination of Australian medical students. Medical Teacher, 48(1), 74-84 (online June 2025). https://www.tandfonline.com/doi/full/10.1080/0142159X.2025.2513418. Read: abstract only.
- [Lee2024] Lee, J., Smith, D., Woodhead, S., Lan, A. (2024). Math Multiple Choice Question Generation via Human-Large Language Model Collaboration. EDM 2024 (per the arXiv record); arXiv preprint 2405.00864. https://arxiv.org/html/2405.00864. Read: excerpts.
- [Jury2024] Jury, B., Lorusso, A., Leinonen, J., Denny, P., Luxton-Reilly, A. (2024). Evaluating LLM-generated Worked Examples in an Introductory Programming Course. ACE 2024, 77-86. https://dl.acm.org/doi/10.1145/3636243.3636252. Open access: https://juholeinonen.com/assets/pdf/jury2024evaluating.pdf. Read: full text.
- [Denny2023] Denny, P., Khosravi, H., Hellas, A., Leinonen, J., Sarsa, S. (2023). Can We Trust AI-Generated Educational Content? Comparative Analysis of Human and AI-Generated Learning Resources. arXiv preprint 2306.10509. https://arxiv.org/abs/2306.10509. Read: excerpts.
- [Reza2025] Reza, M., Anastasopoulos, I., Bhandari, S., Pardos, Z. A. (2025). PromptHive: Bringing Subject Matter Experts Back to the Forefront with Collaborative Prompt Engineering for Educational Content Creation. CHI 2025. https://dlnext.acm.org/doi/10.1145/3706598.3714051. Open access: https://arxiv.org/abs/2410.16547. Read: full text.
- [Yang2025] Yang, T., Ren, B., Gu, C., He, T., Ma, B., Konomi, S. (2025). Examining GPT's Capability to Generate and Map Course Concepts and Their Relationships. arXiv preprint 2504.08856. https://arxiv.org/html/2504.08856. Journal version (Leveraging LLMs for Automated Extraction and Structuring of Educational Concepts and Relationships): https://www.mdpi.com/2504-4990/7/3/103. Read: excerpts.
- [CoSTORM2024] Jiang, Y., Shao, Y., Ma, D., Semnani, S. J., Lam, M. S. (2024). Into the Unknown Unknowns: Engaged Human Learning through Participation in Language Model Agent Conversations (Co-STORM). EMNLP 2024, 9917-9955. https://aclanthology.org/2024.emnlp-main.554/. Open access: https://arxiv.org/abs/2408.15232. Read: excerpts.
- [Sun2024] Sun, S., Yuan, R., Cao, Z., Li, W., Liu, P. (2024). Prompt Chaining or Stepwise Prompt? Refinement in Text Summarization. Findings of ACL 2024, 7551-7558. https://aclanthology.org/2024.findings-acl.449/. Open access: https://arxiv.org/abs/2406.00507. Read: excerpts.
- [LongGenBench2024] Wu, Y., Hee, M. S., Hu, Z., Lee, R. K.-W. (2024). LongGenBench: Benchmarking Long-Form Generation in Long Context LLMs. ICLR 2025. https://arxiv.org/abs/2409.02076. Read: excerpts.
- [Madaan2023] Madaan, A., Tandon, N., Gupta, P., and others (2023). Self-Refine: Iterative Refinement with Self-Feedback. NeurIPS 2023 (Advances in Neural Information Processing Systems 36, per Crossref); arXiv preprint 2303.17651. https://arxiv.org/abs/2303.17651. Read: excerpts (abstract and results table).
- [Huang2024] Huang, J., Chen, X., Mishra, S., Zheng, H. S., Yu, A. W., Song, X., Zhou, D. (2024). Large Language Models Cannot Self-Correct Reasoning Yet. ICLR 2024; arXiv preprint 2310.01798. https://arxiv.org/abs/2310.01798. Per-model figures first taken from the secondary summary at https://www.alphaxiv.org/abs/2310.01798, then found in the paper's Table 3 by the citation check. Read: excerpts (abstract and introduction).
- [DebateOrVote2025] Choi, H. K., Zhu, X., Li, S. (2025). Debate or Vote: Which Yields Better Decisions in Multi-Agent Large Language Models? NeurIPS 2025 spotlight (per the arXiv record); arXiv preprint 2508.17536. https://arxiv.org/abs/2508.17536. Read: secondary summary at https://mlsanity.com/paper/2508.17536/ (reproduces the abstract).
- [Cemri2025] Cemri, M., Pan, M. Z., Yang, S., and 10 others (2025). Why Do Multi-Agent LLM Systems Fail? NeurIPS 2025 Datasets and Benchmarks Track. https://proceedings.neurips.cc/paper_files/paper/2025/file/b1041e52d3be19f0a9bc491657488e4a-Paper-Datasets_and_Benchmarks_Track.pdf. Read: excerpts.
- [HULA2024] Takerngsaksiri, W., Pasuksmit, J., Thongtanunam, P., Tantithamthavorn, C., Zhang, R., Jiang, F., Li, J., Cook, E., Chen, K., Wu, M. (2024). Human-In-the-Loop Software Development Agents. ICSE SEIP 2025; arXiv preprint 2411.12924. https://arxiv.org/abs/2411.12924. Company blog: https://www.atlassian.com/blog/how-we-build/hula-blog-autodev-paper-human-in-the-loop-software-development-agents. Read: excerpts.
- [Brynjolfsson2025] Brynjolfsson, E., Li, D., Raymond, L. (2025). Generative AI at Work. Quarterly Journal of Economics, 140(2), 889-942. https://academic.oup.com/qje/article/140/2/889/7990658. Working paper: https://www.nber.org/papers/w31161. Read: excerpts.
- [Peng2023] Peng, S., Kalliamvakou, E., Cihon, P., Demirer, M. (2023). The Impact of AI on Developer Productivity: Evidence from GitHub Copilot. arXiv preprint 2302.06590. https://arxiv.org/abs/2302.06590. Read: full text.
- [METR2025] Becker, J., Rush, N., Barnes, E., Rein, D. (2025; METR). Measuring the Impact of Early-2025 AI on Experienced Open-Source Developer Productivity. arXiv preprint 2507.09089. https://arxiv.org/html/2507.09089. Blog: https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/. Read: full text.
- [METR2026] Becker, J., Rush, N., Cunningham, T., Rein, D., Mahamud, K. (2026). We are Changing our Developer Productivity Experiment Design. METR blog, 24 February 2026. https://metr.org/blog/2026-02-24-uplift-update/. Read: excerpts.
- [Plitt2010] Plitt, M., Masselot, F. (2010). A Productivity Test of Statistical Machine Translation Post-Editing in a Typical Localisation Context. Prague Bulletin of Mathematical Linguistics, 93, 7-16. https://ufal.mff.cuni.cz/pbml/93/art-plitt-masselot.pdf. Read: full text.
- [Green2013] Green, S., Heer, J., Manning, C. D. (2013). The Efficacy of Human Post-Editing for Language Translation. CHI 2013, 439-448. https://doi.org/10.1145/2470654.2470718. Open copy: https://idl.cs.washington.edu/files/2013-PostEditing-CHI.pdf. The first-pass link https://nlp.stanford.edu/pubs/green+heer+manning.chi13.pdf timed out on 2026-11-22. Read: excerpts.
- [Guerberof2009] Guerberof, A. (2009). Productivity and quality in MT post-editing. MT Summit XII workshop. https://aclanthology.org/2009.mtsummit-btm.7.pdf. Read: excerpts.
- [SanchezTorron2016] Sanchez-Torron, M., Koehn, P. (2016). Machine Translation Quality and Post-Editor Productivity. AMTA 2016, 16-26. https://aclanthology.org/2016.amta-researchers.2/. Read: excerpts.
- [HansenSchirraChapter] Hansen-Schirra, S., Nitzke, J., Gutermuth, S., Maass, C., Rink, I. (2020). Technologies for the Translation of Specialised Texts into Easy Language (section 4.1 covers fuzzy matches). In Hansen-Schirra, S., Maass, C. (eds.), Easy Language Research: Text and User Perspectives. Frank & Timme. https://www.esv-elibrary.de/book/99.160005/9783732992676/00046/download. Book page: https://www.esv-elibrary.de/book/99.160005/9783732992676. Read: excerpts.
- [Toral2019] Toral, A. (2019). Post-editese: an Exacerbated Translationese. MT Summit 2019. https://aclanthology.org/W19-6627.pdf. Open access: https://arxiv.org/abs/1907.00900. Read: excerpts.
- [Parasuraman2010] Parasuraman, R., Manzey, D. H. (2010). Complacency and Bias in Human Use of Automation: An Attentional Integration. Human Factors, 52(3), 381-410. https://journals.sagepub.com/doi/10.1177/0018720810376055. Open deposit: https://api-depositonce.tu-berlin.de/server/api/core/bitstreams/cafd2873-814b-4c59-bab1-addd42e249d2/content. Read: excerpts.
- [Si2024] Si, C., Goyal, N., Wu, S. T., Zhao, C., Feng, S., Daume III, H., Boyd-Graber, J. (2024). Large Language Models Help Humans Verify Truthfulness - Except When They Are Convincingly Wrong. NAACL 2024. https://aclanthology.org/2024.naacl-long.81/. Open access: https://arxiv.org/html/2310.12558. Read: full text.
- [HomogMeta2026] de Rooij, A., Biskjaer, M. M. (2026). Generative AI Makes Creative Output More Homogeneous. European Conference on Cognitive Ergonomics 2026. https://dl.acm.org/doi/full/10.1145/3822301.3822304. Abstract on the Aarhus University record: https://pure.au.dk/portal/en/publications/e8873312-0b81-44d9-9141-bf8c94054611. Read: excerpts.
- [Doshi2024] Doshi, A. R., Hauser, O. P. (2024). Generative AI enhances individual creativity but reduces the collective diversity of novel content. Science Advances, 10(28), eadn5290. Open access: https://pmc.ncbi.nlm.nih.gov/articles/PMC11244532/. Read: secondary summary at https://annebecheru.com/writing/taste-in-the-ai-era/ and https://aiuxevidence.com/e/homogenization. The citation check found the figures used in this file in the paper itself.
- [Padmakumar2024] Padmakumar, V., He, H. (2024). Does Writing with Language Models Reduce Content Diversity? ICLR 2024. Open access: https://arxiv.org/abs/2309.05196. Read: secondary summary at https://aiuxevidence.com/e/homogenization and https://achrafez.com/posts/2026/from-human-like-text-to-human-voice.
- [Goncalves2022] Goncalves, P. W., Fregnan, E., Baum, T., Schneider, K., Bacchelli, A. (2022). Do explicit review strategies improve code review performance? Towards understanding the role of cognitive load. Empirical Software Engineering, 27(4). https://link.springer.com/article/10.1007/s10664-022-10123-8. Read: excerpts.
- [Zhang2020] Zhang, Y., Liao, Q. V., Bellamy, R. K. E. (2020). Effect of Confidence and Explanation on Accuracy and Trust Calibration in AI-Assisted Decision Making. FAT* 2020, 295-305. https://dl.acm.org/doi/10.1145/3351095.3372852. Open access: https://arxiv.org/abs/2001.02114. Read: excerpts.
- [Bansal2021] Bansal, G., Wu, T., Zhou, J., Fok, R., Nushi, B., Kamar, E., Ribeiro, M. T., Weld, D. (2021). Does the Whole Exceed its Parts? The Effect of AI Explanations on Complementary Team Performance. CHI 2021. https://dl.acm.org/doi/fullHtml/10.1145/3411764.3445717. Open access: https://arxiv.org/abs/2006.14779. Read: excerpts.
- [Vasconcelos2023] Vasconcelos, H., Jorke, M., Grunde-McLaughlin, M., Gerstenberg, T., Bernstein, M. S., Krishna, R. (2023). Explanations Can Reduce Overreliance on AI Systems During Decision-Making. Proceedings of the ACM on Human-Computer Interaction (CSCW). https://dl.acm.org/doi/10.1145/3579605. Open access: https://arxiv.org/html/2212.06823v2. Read: excerpts.
- [Bucinca2021] Bucinca, Z., Malaya, M. B., Gajos, K. Z. (2021). To Trust or to Think: Cognitive Forcing Functions Can Reduce Overreliance on AI in AI-assisted Decision-making. Proceedings of the ACM on Human-Computer Interaction (CSCW). https://dl.acm.org/doi/10.1145/3449287. Open access: https://arxiv.org/abs/2102.09692. Read: excerpts.
- [Kim2024] Kim, S. S. Y., Liao, Q. V., Vorvoreanu, M., Ballard, S., Wortman Vaughan, J. (2024). "I'm Not Sure, But...": Examining the Impact of Large Language Models' Uncertainty Expression on User Reliance and Trust. FAccT 2024. https://dl.acm.org/doi/fullHtml/10.1145/3630106.3658941. Open access: https://arxiv.org/html/2405.00623. Read: excerpts.
- [Liu2023] Liu, N. F., Zhang, T., Liang, P. (2023). Evaluating Verifiability in Generative Search Engines. Findings of EMNLP 2023. https://aclanthology.org/2023.findings-emnlp.467/. Open access: https://arxiv.org/html/2304.09848. Read: excerpts.
- [Amershi2019] Amershi, S., Weld, D., Vorvoreanu, M., and 10 others (2019). Guidelines for Human-AI Interaction. CHI 2019. https://dl.acm.org/doi/fullHtml/10.1145/3290605.3300233. Read: excerpts.
- [AmershiSurvey2023] Li, T., Vorvoreanu, M., DeBellis, D., Amershi, S. (2023). Assessing Human-AI Interaction Early through Factorial Surveys: A Study on the Guidelines for Human-AI Interaction. ACM Transactions on Computer-Human Interaction, 30(5). https://dl.acm.org/doi/10.1145/3511605. Read: excerpts.
- [Horvitz1999] Horvitz, E. (1999). Principles of Mixed-Initiative User Interfaces. CHI 1999, 159-166. https://dl.acm.org/doi/pdf/10.1145/302979.303030. Open access: https://www.erichorvitz.com/chi99horvitz.pdf. Read: full text.
- [Haynes2009] Haynes, A. B., Weiser, T. G., Berry, W. R., and 12 others for the Safe Surgery Saves Lives Study Group (2009). A Surgical Safety Checklist to Reduce Morbidity and Mortality in a Global Population. New England Journal of Medicine, 360(5), 491-499. https://www.nejm.org/doi/full/10.1056/NEJMsa0810119. Read: excerpts.
- [Urbach2014] Urbach, D. R., Govindarajan, A., Saskin, R., Wilton, A. S., Baxter, N. N. (2014). Introduction of Surgical Safety Checklists in Ontario, Canada. New England Journal of Medicine, 370, 1029-1038. https://www.nejm.org/doi/full/10.1056/NEJMsa1308261. Read: excerpts.
- [ChecklistTrial2015] Haugen, A. S., Softeland, E., Almeland, S. K., Sevdalis, N., Vonen, B., Eide, G. E., Nortvedt, M. W., Harthug, S. (2015). Effect of the World Health Organization Checklist on Patient Outcomes: A Stepped Wedge Cluster Randomized Controlled Trial. Annals of Surgery, 261(5), 821-828. https://journals.lww.com/annalsofsurgery/Fulltext/2015/05000/Effect_of_the_World_Health_Organization_Checklist.1.aspx. DOI: https://doi.org/10.1097/SLA.0000000000000716. Read: secondary summary (search tool summary of this page). The citation check read the PubMed abstract.
- [Tohidi2006] Tohidi, M., Buxton, W., Baecker, R., Sellen, A. (2006). Getting the right design and the design right. CHI 2006, 1243-1252. DOI 10.1145/1124772.1124960. Abstract page: https://app.cochat.ai/papers/10.1145/1124772.1124960. Read: abstract only.
- [Dow2010] Dow, S. P., Glassco, A., Kass, J., Schwarz, M., Schwartz, D. L., Klemmer, S. R. (2010). Parallel prototyping leads to better design results, more divergence, and increased self-efficacy. ACM Transactions on Computer-Human Interaction, 17(4). https://dl.acm.org/doi/10.1145/1879831.1879836. Read: abstract only.
- [Aleven2009] Aleven, V., McLaren, B. M., Sewall, J., Koedinger, K. R. (2009). A New Paradigm for Intelligent Tutoring Systems: Example-Tracing Tutors. International Journal of Artificial Intelligence in Education, 19(2), 105-154. Author copy: http://www.cs.cmu.edu/~bmclaren/pubs/AlevenEtAl-ExampleTracingTutors-IJAIED2009.pdf. Read: excerpts.
- [Remillard2005] Remillard, J. T. (2005). Examining Key Concepts in Research on Teachers' Use of Mathematics Curricula. Review of Educational Research, 75(2), 211-246. https://journals.sagepub.com/doi/10.3102/00346543075002211. Open copy: https://www.ime.unicamp.br/~hqsaearp/Disciplinas/AnaliseLivros/Bibliografia/Teoria/Examining%20Key%20Concepts%20in%20Research%20on.pdf. Read: excerpts.
- [DavisKrajcik2005] Davis, E. A., Krajcik, J. S. (2005). Designing Educative Curriculum Materials to Promote Teacher Learning. Educational Researcher, 34(3), 3-14. https://journals.sagepub.com/doi/10.3102/0013189X034003003. Open copy: https://compassproject.net/Sadhana/teaching/readings/DavisER.pdf (did not respond on 2026-11-22). Read: excerpts.
- [Kim2017] Kim, J. S., Burkhauser, M., Quinn, D. M., Guryan, J., Kingston, H. C., Aleman, K. (2017). Effectiveness of Structured Teacher Adaptations to an Evidence-Based Summer Literacy Program. Reading Research Quarterly, 52(4), 443-467. https://doi.org/10.1002/rrq.178. Open access: https://dash.harvard.edu/bitstreams/729fc67d-c54d-4fb5-a1ed-2e427b539295/download. Read: excerpts.
- [EarlyMath2023] Clements, D. H., Lizcano, R., Sarama, J. (2023). Research and Pedagogies for Early Math. Education Sciences, 13(8), 839. https://www.mdpi.com/2227-7102/13/8/839. Read: excerpts (used for its account of Ann Brown's "lethal mutation").
- [Johnson2025] Johnson, J. M. (2025). Cognitive Task Analysis in the Age of Artificial Intelligence. MODSIM World 2025. https://www.modsimworld.org/papers/2025/MODSIM_2025_paper_11.pdf. Read: excerpts.
- [ASEE2024] Ross, A., Katz, A., Chew, K. J., Matusovich, H. (2024). Stumbling Our Way Through Finding a Better Prompt: Using GPT-4 to Analyze Engineering Faculty's Mental Models of Assessment. ASEE Annual Conference 2024. https://doi.org/10.18260/1-2--48032. Read: abstract only.
- [Keppler2024] Keppler, S., Sinchaisri, W. P., Snyder, C. (2024). The Teacher's Knapsack: Generative AI and Teacher Productivity (the title now on the SSRN record; first posted as "Backwards Planning with Generative AI: Case Study Evidence from US K12 Teachers", which the Stanford page still shows). SSRN working paper. http://dx.doi.org/10.2139/ssrn.4924786. Abstract page: https://scale.stanford.edu/publications/backwards-planning-generative-ai-case-study-evidence-us-k12-teachers. Read: abstract only.

Survey, practitioner, vendor and secondary sources

- [RAND2025] Doss, C. J., Bozick, R., Schwartz, H. L., Chu, L., Rainey, L. R., Woo, A., Reich, J., Dukes, J. (2025). AI Use in Schools Is Quickly Increasing but Guidance Lags Behind: Findings from the RAND Survey Panels. RAND RR-A4180-1. https://www.rand.org/pubs/research_reports/RRA4180-1.html. Read: excerpts.
- [Kaufman2025] Kaufman, J. H., Woo, A., Eagan, J., Lee, S., Kassan, E. B. (2025). Uneven Adoption of Artificial Intelligence Tools Among U.S. Teachers and Principals in the 2023-2024 School Year. RAND RR-A134-25. https://www.rand.org/pubs/research_reports/RRA134-25.html. Read: abstract only (report summary page).
- [Gallup2025] Gallup and Walton Family Foundation (2025). Teaching for Tomorrow: Unlocking Six Weeks a Year With AI. https://news.gallup.com/poll/691967/three-teachers-weekly-saving-six-weeks-year.aspx. Report: https://static.waltonfamilyfoundation.org/df/fb/eba12807470a9402d7433cc47dba/teaching-for-tomorrow-unlocking-six-weeks-a-year-with-ai-report.pdf. Read: excerpts.
- [CSM2025] Common Sense Media (2025). AI Risk Assessment: AI Teacher Assistants. Updated 14 August 2025. https://www.commonsensemedia.org/sites/default/files/featured-content/files/csm-ai-risk-assessment-ai-teacher-assistants-final.pdf. Read: excerpts. Practitioner assessment.
- [OakATRS2024] UK Government (2024). Oak National Academy: Aila (Oak's AI Lesson Assistant). Algorithmic transparency record, published 17 December 2024. https://www.gov.uk/algorithmic-transparency-records/oak-national-academy-aila-oaks-ai-lesson-assistant. Read: excerpts. Deployed-system description.
- [OakGuide] Oak National Academy (undated). Getting started with Aila (prompt guide v11). https://support.thenational.academy/hubfs/Knowledge%20Base/Aila/aila-prompt-guide-v11.pdf?hsLang=en. Code repositories seen in search results: https://github.com/oaknational/oak-ai-lesson-assistant/ and https://github.com/oaknational/oak-ai-autoeval-tools. Read: excerpts. Vendor documentation.
- [Anthropic2024] Erik S. and Barry Zhang, as credited on the page (2024). Building effective agents. Anthropic engineering blog, 19 December 2024. https://www.anthropic.com/engineering/building-effective-agents. Read: full text. Vendor guidance.
- [CiscoSmartBear] SmartBear study of code review at Cisco (2,500 reviews), with Sadowski et al. 2018 on Google and McIntosh et al. on open-source projects. Read: secondary summary at https://sqrbok.github.io/content/verif/inspection/modern-code-review.html. SmartBear's own pages, found by the citation check: https://static0.smartbear.co/support/media/resources/cc/book/code-review-cisco-case-study.pdf (the case-study chapter) and https://smartbear.com/learn/code-review/best-practices-for-peer-code-review/.
- [Chapman2010] Chapman, B. (2010). How Long Does it Take to Create Learning? Chapman Alliance research study. Slide deck: https://www.slideshare.net/slideshow/how-long-does-it-take-to-create-learning/5198860. Read: secondary summary (deck excerpts plus the table at https://capytech.co.uk/e-learning-build-time-estimator/).
- [KappDefelice2009] Kapp, K. M., Defelice, R. A. (2009). Time to Develop One Hour of Training. ASTD/ATD. Read: secondary summary at http://johnnyonthespottraining.blogspot.com/2011/10/time-to-develop-one-hour-of-training.html (repost of the article) and the table reproduced at https://assets.td.org/m/2fe4cc13fae8a57e/original/111615_The-Art-and-Science-of-Training_Companion-PDF.pdf.
- [IFL2018] University of Pittsburgh Institute for Learning (2018). Educative Curricula and When a Little PD is Just Not Worth It. https://researchsites.pitt.edu/ifl/news/educative-curricula-and-when-little-pd-just-not-worth-it. Read: excerpts. Practitioner article.

## Citation check

- **Date:** 2026-11-22
- **Checker:** a second agent, working independently of the first pass
- **Method:** Every source-list entry was looked up today: Crossref for DOIs (title, authors, year, and a search for retraction or correction notices), the arXiv API for preprints, PubMed for the medical papers, and an HTTP GET for every other link. Publisher pages that answered 403 to a script were checked through Crossref, PubMed, OpenAlex, an open-access copy, or the page text saved by the web search tool. Then every number in Top findings, and 55 more figures or groups of figures in Findings by topic, were looked up in the cited source's own text, fetched today and searched for the figure.
- **Limits:** Findings-by-topic figures outside that sample stand as the first pass wrote them. Among them: the RAND sample size and weekly-use share in Topic 1.4, the Hu et al. plan counts, the Yang et al. ratings, the Amershi 2019 and factorial-survey counts, the Denny et al. medians, and most quotations in Topics 7.5 and 9 and in "What the sources themselves recommend".

**Counts**

- Sources swept: 94
- Sources fine: 90 (40 of them had a missing author list, now filled in from Crossref, arXiv, PubMed or the paper's first page; 11 more gained a venue, a volume, a working link or a note)
- Sources corrected: 4
- Sources dropped: 0
- Crossref retraction, correction or expression-of-concern notices across the 60 DOIs checked: 0
- Top-finding claims checked: 108
- Top-finding claims confirmed on the primary source: 102
- Top-finding claims confirmed on a secondary page only: 4 (the paper's own abstract as reproduced by PubMed for [Haynes2009] and [Urbach2014], by OpenAlex for [Tohidi2006], and by the Aarhus University repository record for [HomogMeta2026]; the publisher pages sat behind bot walls)
- Top-finding claims unconfirmed: 0
- Top-finding claims wrong: 2 (the word "planted" in finding 16 and the "358 learners" sample in finding 8; both corrected)
- Findings-by-topic figures checked beyond the top findings: 55
- Of those, confirmed on the primary source: 44
- Of those, confirmed on a secondary page only: 6 (PubMed abstracts for [AcadMed2024], [MedTeach2025] and the three checklist studies in Topic 7.4; the OpenAlex abstract for [ASEE2024]; the blog repost for [KappDefelice2009]; the Capytech page for the 490:1 and 3,947 figures in [Chapman2010])
- Of those, unconfirmed: 2
- Of those, wrong: 3 (two misquotes and one misattribution; all corrected)
- The sentence "No study measures an 80% complete threshold" in finding 20 is the first pass's own search result and sits outside these counts.

**What the six headline numbers measure**

- 24% to 31% planning time saved: one minus the ratio of teacher-diary preparation time, treatment over control (76% for Aila over ten weeks, 69% for ChatGPT in weeks 6 to 10). Both figures are in the trial reports. [NFERAila2026] [EEF2024]
- 37% valid: 22 of 60 GPT-4 distractors that four teachers in a pilot accepted together with the stated misconception and feedback. The authors count two of the 22 as in fact invalid. [Lee2024]
- 90% of activities: remember, understand and apply added together (2,017 of 2,230). Recall alone (remember) is 45%. [Trust2025]
- About two thirds missed: a mean of 2.67 of 4 erroneous ChatGPT drafts left insufficiently corrected by each of 20 physicians. The errors were the model's own. [Biro2025]
- g = -0.23: human plus AI against the better of human alone and AI alone, pooled over 370 effect sizes from 106 experiments. Against the human alone the same data give g = +0.64. [Vaccaro2024]
- About 75%: a target-side fuzzy match score (character 3-gram overlap between the raw machine translation and the translator's final text), below which post-editing was slower than translating from scratch. One language pair, one rule-based engine, ten translators. [Parra2015]

**Corrections to claims**

- [Biro2025] was: reviewers miss errors "planted" in AI drafts (finding 16, the Topic 6.2 heading, finding and caveat, the file title, and the core-source list). now: the four errors were ChatGPT's own, picked out beforehand by two physicians, and the wording says "errors in" or "known errors". One of the four only repeats the patient's own misspelling of a drug name.
- [Reza2025] was: the hint comparison ran with "358 learners" (finding 8, Topic 2.5). now: 358 learners recruited, 225 analysed after exclusions, as the arXiv version of the paper reports. The CHI page sat behind a bot wall.
- [EEF2024] was: the report says teachers "likely sent us only their best resources" (Topic 1.1). now: the report's own words, teachers "were able to choose which lessons they sent us, so there is likely to be some self-selection bias".
- [Noy2023] was: tasks quoted as "generic, self-contained" writing (Topic 4.1). now: "relatively generic writing", set through short, self-contained prompts, which is what the paper says.
- [Plitt2010] was: "later papers cite the individual range as 20% to 131%" (Topic 5.1). now: the paper itself gives that range.
- [Kaufman2025] was: "25% a year earlier" in finding 2 cited to [RAND2025] alone. now: [Kaufman2025] added, the RAND report that states the 25%.
- [Grossmann2025] was: students' conceptions "about 3% (search summary)" (finding 3, Topic 1.2). now: 3.3%, from the paper's results text.
- [Lee2024] was: "Teachers judged 37% of ... sets fully valid" with no sample named (finding 7, Topic 2.4), and "about 70%" of stems valid (Topic 2.3). now: four teachers in a pilot, 22 of 60 distractors, 70% of stems, and the authors' remark that two of the 22 are in fact invalid.
- [Scarlatos2024] was: "Teachers rated generated distractors 2.67" (finding 7). now: "In a second study two teachers rated", so the reader does not take it for the same teachers as [Lee2024].
- [EEF2024] was: "56.2 against 81.5 minutes a week" in finding 1 with no period. now: "in weeks 6 to 10 of the trial", the period of the primary outcome.
- [Huang2024] was: per-model accuracy drops credited to "a secondary summary" (Topic 3.3). now: credited to the paper's Table 3, where the four figures appear (GSM8K).
- [Moore2024KC] note added in Topic 2.2: the paper's two domain counts (23 and 32) add up to 55, while it reports 57 of 87 for the pooled test. The file's figures match the paper.
- [DellAcqua2023] note added in Topic 4.2: the 2023 working paper prints 60% and 70% for the two AI groups, where the file has 60% and 70.6%.
- "search summary" marks removed after each figure was found on the primary source: [Doyle2025] SMART percentages (Table 4), [Doughty2024] 81.7%, [Sun2024] 53 wins, [Kim2024] 43.6% to 52.0%, [Cemri2025] 44.2%, 32.3% and 23.5%, [ChecklistTrial2015] 19.9% to 11.5% (PubMed abstract), [Lee2024] 70%. The "What stayed unconfirmed" list near the top now records the result for each item.

**Corrections to sources**

- [AmershiSurvey2023] was: key AmershiSurvey2022, year 2022, authors not captured. now: Li, Vorvoreanu, DeBellis, Amershi (2023), ACM Transactions on Computer-Human Interaction 30(5), per Crossref. Key renamed in Topic 7.4 and in the source list.
- [Keppler2024] was: title "Backwards Planning with Generative AI: Case Study Evidence from US K12 Teachers". now: "The Teacher's Knapsack: Generative AI and Teacher Productivity", the title Crossref and OpenAlex hold for the same SSRN record. The earlier title stays in the entry because the Stanford page still shows it.
- [Kim2017] was: pages 443-468. now: 443-467, per Crossref. DOI added.
- [Green2013] was: a link to nlp.stanford.edu that timed out today, and a title flagged as taken from a search lead. now: title confirmed on the paper's first page, pages 439-448 and DOI from Crossref, and a working open copy added.
- Author lists filled in (nothing in these entries was wrong): [EEF2024], [OakAutoEval2025], [Yao2025], [HKMCQ2025], [Shao2024], [Vaccaro2024], [DellAcqua2023], [Biro2025], [Parra2015], [Hu2025], [EduPlanner2025], [Sridhar2023], [Doughty2024], [EMPsych2025], [AcadMed2024], [MedTeach2025], [Denny2023], [CoSTORM2024], [LongGenBench2024], [Madaan2023], [Huang2024], [DebateOrVote2025], [Cemri2025], [HULA2024], [METR2025], [HansenSchirraChapter] (year 2020 and chapter title too), [Si2024], [HomogMeta2026], [Goncalves2022], [Bansal2021], [Vasconcelos2023], [Bucinca2021], [Kim2024], [Liu2023], [Amershi2019], [Haynes2009], [ChecklistTrial2015] (Haugen et al., full title, volume and pages), [Aleven2009], [EarlyMath2023], [ASEE2024].
- Venue, volume or version details added: [Grossmann2025] 48(15); [Plitt2010] pages 7-16; [Lee2024] EDM 2024; [SAQUET2024] AIED 2024; [Madaan2023] NeurIPS 2023 confirmed on Crossref; [Huang2024] ICLR 2024; [HULA2024] ICSE SEIP 2025; [DebateOrVote2025] NeurIPS 2025; [DellAcqua2023] Organization Science 37(2), 403-423; [MedTeach2025] 48(1), 74-84; [Malik2024] an April 2025 version exists with the same comparison results; [OakAutoEval2025] the short CEUR version has a different title.
- Links added: [DellAcqua2023] open copy of the working paper; [Reza2025], [Padmakumar2024], [Bansal2021], [DebateOrVote2025] arXiv; [Doshi2024] PubMed Central; [HomogMeta2026] Aarhus University record; [NFERAila2026] hosted copy of the full evaluation report; [CiscoSmartBear] SmartBear's own case-study chapter and best-practice page; [AcadMed2024], [ChecklistTrial2015], [Kim2017] DOIs.

**Dropped sources**

- All 94 sources stay in the source list. Each one exists, and each says what the file cites it for.

**Claims left unconfirmed**

- [HomogMeta2026] the 95% CI of 0.094 to 0.574 and the "high heterogeneity" in Topic 6.3. The abstract confirms 19 studies, 61 effect sizes and d = .334. The full text sat behind the ACM bot wall and a repository login.
- [DellAcqua2023] the 70.6% for the second AI group in Topic 4.2. The 2023 working paper prints 70%. The journal text sat behind the INFORMS bot wall. The 84.5%, the 60% and the 19-point gap are in the working paper, and the journal abstract repeats the 19.

**Other notes**

- Links that did not load today, with the source confirmed another way: the NFER summary PDF listed under [EEF2024] (403 even with a browser user agent; the main report PDF and the EEF project page load); the compassproject.net copy of [DavisKrajcik2005] (no response; Crossref confirms the article); the alphaxiv summary under [Huang2024] (403; the paper itself now supplies the figures). Publisher pages at ACM, Sage, Wiley, Taylor and Francis, NEJM, Science, INFORMS, OUP, IEEE, MDPI, SSRN and ASEE answered scripts with 403 or a challenge page, and Crossref confirmed each of those records.
- Preprint against published version: [Brynjolfsson2025] differs between the 2023 working paper (14%, 34%, 5,179 agents) and the journal (15%, about 30%, 5,172 agents), and the file already reports both. [Reza2025] was checked on the arXiv version. [Malik2024] has the same results in its August 2024 and April 2025 versions. [METR2025] and its 2026 update are both in the file.
- Injection check: the arXiv PDF of [Reza2025] contains the words "Ignore previous instructions" inside a study participant's prompt that the paper quotes in its appendix. It was read as data and nothing was done with it. Every other fetched page was plain content.

**Corrections from the cleanup pass (2026-11-22)**

- [OakAutoEval2025] Was: "Oak's judge agreed with 20 teachers at a weighted kappa of 0.17" (finding 11). Now: Oak recruited 20 teachers and excluded one teacher's scores as unreliable, and its judge agreed with the remaining teachers at a weighted kappa of 0.17. Source: section "Case Study", subsection "Method".
- [Lee2024] Was: the "four educators" and "70%" figures "are in the paper's abstract" (the "What stayed unconfirmed" list). Now: both are in the paper's section 1.1, "Contributions", and the abstract gives the result in words. Source: section 1.1, "Contributions", of the arXiv full text.
- [Shao2024] [Sun2024] Was: "An outline stage and separate calls per stage both improve long generated documents" (headline of finding 9). Now: An outline stage improved long generated articles, and separate calls per stage improved summaries of news articles. Source: Shao et al., section 4.1, "Article Selection", and section 5.2, "Ablation Studies"; Sun et al., section 4.1, "Dataset", and "Limitations".
- [Sun2024] Was: absent. Topic 3.2 said "Human evaluation agreed" and stopped there. Now: the same words, followed by what the human evaluation was: two postgraduate students compared the chained summary with the single-prompt summary directly, on 30 of the 100 pairs, and with GPT-4 the chained summary won 14, tied 8 and lost 8. Source: section 6, "Exp III: Human Evaluation", and Table 2, row "GPT 4", read at https://arxiv.org/html/2406.00507.
- [Kim2024] Was: absent. Topic 7.1 gave "43.6% to 52.0%" for accuracy when the AI was wrong, beside the between-group agreement figures (80.9% and 74.8%), so it read as a between-group result. Now: Topic 7.1 says that the 43.6% and 52.0% compare answers inside the first-person group, and gives the between-group figures: 63.9% against 72.8% correct overall, and 33.0% against 47.8% correct on the questions the AI got wrong. Source: section 4.5, "Task Performance: Correct, Time", with Table 1, and section 5.1 with Figure 2(a), read at https://arxiv.org/html/2405.00623.

## Breadcrumbs

- **Breadcrumb 1**
  - severity: low
  - category: cap-hit
  - note: The file lists 94 sources against a target of 40 to 60, so depth is uneven: 30 were read as full text and 48 as excerpts.
- **Breadcrumb 2**
  - severity: medium
  - category: scope-drift
  - note: MagicSchool, Diffit and Khanmigo teacher tools are covered only through one independent class-level assessment, and Diffit has no independent quality study in this file.
- **Breadcrumb 3**
  - severity: low
  - category: vetting-fail
  - note: Several author lists were not captured and about ten figures rest on the search tool's summary of a page; each is marked at the point of use. (Citation check, 2026-11-22: the author lists are now filled in and those figures were found on the primary sources, so the marks are gone.)
- **Breadcrumb 4**
  - severity: low
  - category: other
  - note: Kapp and Defelice, Chapman Alliance, the Cisco code-review study, Doshi and Hauser, and Padmakumar and He were read through secondary pages, not the originals.
- **Breadcrumb 5**
  - severity: low
  - category: other
  - note: The NFER Aila trial (2026) was read from its publication summary page only; the full report on the EEF site was not opened.
- **Injection attempts:** zero pages contained instructions aimed at the reader agent.

