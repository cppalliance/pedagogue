---
produced: 2026-11-22
title: "Adaptive learning adaptation policies evidence review - stuck detection, wheel-spinning, productive failure, help and hint policy, scaffolding and fading, worked examples, human tutoring moves, remediation and misconception feedback, feedback types and timing, praise, expertise reversal, pretesting and test-out, overlearning, target difficulty and 85 percent rule, task selection and spaced repetition, learner control, does adaptivity pay (Pedagogue stream 4, Q7 to Q12)"
---

# Stream 4 - Adaptation policies: what to do when a student is stuck, and when a student is ahead

## How I checked things

- **Date of the work:** 2026-11-22.
- **Scope:** vision questions Q7 (productive struggle or spinning in place), Q8 (help), Q9 (remediation), Q10 (feedback), Q11 (acceleration) and Q12 (difficulty and sequence), plus what effective tutors do, learner control, and whether adaptivity pays. Rules in play: H6 to H14, H16 and H18.
- **Tools:** web search and web fetch. The search tool returns a short machine-written synthesis plus quoted page text, and it saves the full page text. I took numbers only from the quoted page text or the saved full text, never from the synthesis alone. Where a number sat only in the synthesis I left it out or marked it unconfirmed.
- **What I fetched:** 113 sources. For each source the source list says what I read: "full text", "full text (key sections)", "abstract only", or "secondary summary at URL". "Full text (key sections)" means I had the whole text and read the abstract, method, results and discussion passages that hold the numbers I quote.
- **Read in depth (the 16 that matter most):** Gong 2014 dissertation (the wheel-spinning data), Zhang et al. 2019, Kai et al. 2018, Ahtisham et al. 2026 (LLM tutor impasses), Sinha and Kapur 2021, Aleven et al. 2016 (help), Belland et al. 2017, Salden et al. 2010, Tetzlaff et al. 2025, Sleeman et al. 1989, Selent and Heffernan 2015, Van der Kleij et al. 2015, Wisniewski et al. 2020, Papousek et al. 2016, Wilson et al. 2019, Aleven et al. 2016 (adaptivity chapter).
- **Grades:** A = meta-analysis or randomized controlled trial. B = controlled or quasi-experimental study, or a careful analysis of log data with a comparison. C = practitioner or deployed-system report, or purely observational log data. D = expert opinion or theory with no direct test.
- **What stayed unconfirmed:**
  - Beck and Gong 2013 and Gong and Beck 2015: I read the abstracts only. The numbers I report come from Gong's 2014 dissertation, which holds the same analyses, and from later papers that cite them. Those later papers quote different prevalence figures (see "Disagreements").
  - Brown and Burton 1978 and VanLehn's book Mind Bugs (1990): not fetched. Their content is described through VanLehn 1983, Sleeman 1989 and Selent and Heffernan 2015.
  - Kulik and Kulik 1988, Sweller and Cooper 1985, Kapur 2008, D'Mello and Graesser 2012, Karich et al. 2014, Niemiec et al. 1996, Steenbergen-Hu et al. 2016, Reis et al. 1998, Brunmair and Richter 2019, Wang et al. 2024: abstract only. The Kulik and Kulik effect size (0.28) and the Niemiec effect size (-0.03) come from secondary tables (Hattie and Timperley 2007 Table 1, and the Visible Learning MetaX page).
  - Corbett 2001 (cognitive mastery +0.89 SD): the abstract is confirmed, the specific numbers come from a secondary summary in a WPI student report.
  - Kalyuga and Sweller 2005: sample and design come from the paper text, the effect size d = 0.69 comes from the summary panel on the Academia.edu page.
  - Clement et al. 2015: the user study (400 children) is confirmed, the specific outcome numbers are left out because I saw them only in the search synthesis.
  - Three 2024 to 2026 workplace and e-learning trials are cited by title because the fetched pages did not show the authors.
  - Update from the citation check (2026-11-22): the three trials now carry their authors, the Kalyuga and Sweller 2005 effect size and the Graesser 92 percent figure were found in the papers themselves, the Cen 2007 PDF turned out to be the published paper, and the memory-based author lists and year are confirmed. Details are in "Citation check" near the end of the file.
- **Populations:** most of the evidence comes from school children and undergraduates in mathematics and science. Evidence on working adults is thin. Where a study used adults or programming content, the entry says so.
- **Injection check:** every fetched page read as ordinary content, with zero instructions aimed at a reader agent.

## Top findings

1. **Wheel-spinning has a simple, checkable definition and it is common.** Beck and Gong define it as 10 practice opportunities on a skill with three right in a row still to come (15 opportunities in the Cognitive Tutor). In ASSISTments, a student who had not mastered a skill after 10 problems had a 0.089 probability of mastering it in the next 20. Their model estimates that 18 percent of student-skill pairs in ASSISTments and 25 percent in the Cognitive Algebra Tutor end in wheel-spinning, and that it eats between 18 and 54 percent of learning time. Grade B. [Beck2013] [Gong2014]
2. **Stuck students can be spotted after about four attempts using correctness alone.** A logistic regression on one feature, percent correct so far, flagged wheel-spinning after 4 steps with 93.5 percent precision and 77.1 percent recall (under the Predictive Stability++ definition). In the first three steps almost every model stayed below 20 percent precision and recall under the three-in-a-row definition. The two standard definitions of wheel-spinning agreed on only 14 to 42 percent of flagged cases. Grade B. [Zhang2019]
3. **Persistence past 10 problems is productive more often than the first definition implies.** When "productive" means mastery plus a passed retention test, 9.1 percent of student-problem-set pairs were wheel-spinning, against 46.9 percent under Beck and Gong's rule. The strongest warning sign was a mix of attempts with zero hints and more than one bottom-out hint in the last 8 problems (89 percent of such cases were wheel-spinning). Detector quality was modest (AUC 0.684, or 0.636 with student-level cross-validation). Grade B. [Kai2018]
4. **In an LLM tutor, the chance of recovery drops with every turn a student stays stuck, and repeating the same move makes it worse.** Across 1,260 sessions with a guided GPT-4o chemistry tutor, next-turn recovery fell from 39.7 percent at the first stuck turn to 26.2 percent at the third and 18.8 percent at the sixth or later. Each extra stuck turn cut the odds of recovery by 13 percent. A scripted question followed by another scripted question recovered 28.1 percent of the time, against 55.5 percent for the first one. Addressing the student's error grew more useful the longer the impasse lasted (+5 points at depth 1, +12 points at depth 4), and "does that make sense?" checks recovered only 8 to 14 percent. Observational, university students. Grade C. [Ahtisham2026]
5. **Deliberate failure before instruction pays for concepts and transfer, and only under set conditions.** Problem solving before instruction beat instruction first by g = 0.36 on conceptual knowledge and transfer (53 studies, 166 comparisons), and by 0.37 to 0.58 when the design followed productive-failure principles. Procedural knowledge showed g = -0.03. The effect reversed for grades 2 to 5 and for domain-general skills. A consolidation phase that builds on the student's own attempts is required. Grade A. [Sinha2021] [Kapur2016]
6. **Confusion helps only when it is resolved, and boredom is the state to worry about.** Contradictions raised posttest scores only for learners who actually became confused. Boredom was the most persistent state across three learning environments and the one tied to gaming the system and poorer learning. Frustration was less persistent and less harmful. Grade B. [DMello2014] [DMello2012] [Baker2010]
7. **Hints help less than tutoring-system designers assumed, and mostly at a medium skill level.** In Cognitive Tutor logs, hint use improved the next attempt only for students at medium skill. At low skill, clicking through hints went with poorer learning, and trying again went with better results. Students viewed 68 percent of pre-final hint levels for under 1 second, and after 3 errors on a step only 34 percent asked for a hint. Grade B. [Aleven2016help] [Roll2014] [Aleven2003]
8. **Hints the student asks for beat hints pushed after an error, in one small experiment.** Eighth graders gained more with on-demand hints than with proactive hints (effect size 0.35, 95 percent CI 0.02 to 0.74, 61 students in a within-subject design, one class period). Grade A (small). [Razzaq2010]
9. **The classic evidence for contingent tutoring did not replicate, and fading is unproven as a separate ingredient.** A 2025 preregistered direct replication of Wood, Wood and Middleton 1978 (285 three-year-olds) reproduced the tutoring procedure, and the original advantage of contingent instruction on post-test performance failed to appear. Computer-based scaffolding works overall (g = 0.46, 144 studies), with the same effect whether the scaffolds faded, grew, did both, or stayed fixed. Only 16.5 percent of outcomes involved fading. Grade A. [Smit2025] [Belland2017]
10. **Worked examples help novices, and fading them by measured mastery beats fading on a fixed schedule.** Worked examples: g = 0.48 in mathematics (55 studies). In the Geometry Cognitive Tutor, adaptive fading (show worked steps until estimated mastery passes 0.7, bring them back if it falls under 0.5) beat fixed fading and plain tutored problem solving on delayed posttests in a lab study and a classroom study. Backward fading (drop the last step first) with self-explanation prompts raised near and far transfer. Grade A. [Barbieri2023] [Salden2010] [Atkinson2003] [Renkl2003]
11. **Help that suits a novice slows an expert, and the two sides are unequal.** Meta-analysis of 60 experiments (5,924 participants): low-knowledge learners learn more with high assistance (d = 0.505), high-knowledge learners learn less with it (d = -0.428). Giving novices help matters more than withholding it from experts. Grade A. [Tetzlaff2025] [Kalyuga2007] [Kalyuga2003]
12. **Finer-grained interaction stops paying at the step level.** Against no tutoring: human tutors d = 0.79, step-based tutors 0.76, substep-based 0.40, answer-based 0.31. Tutorial dialogue beat reading only when novices studied content written above their level. When tutors were told to prompt and withhold explanations, students learned just as much (11 students). Grade A for the review, B for the single studies. [VanLehn2011] [VanLehn2007] [Chi2001]
13. **The best human tutors ask and hint far more than they tell, and they praise sparingly.** Lepper and Woolverton report that more than 90 percent of expert tutors' remarks are questions, that they give five or six hints in a row before telling, that they pick problems to diagnose and then debug specific errors, and that they give less explicit praise than weaker tutors. Ordinary tutors check understanding with "do you understand?" in 92 percent of cases. Grade C. [Lepper2002] [Graesser1995]
14. **Diagnosing the exact bug has a weak record against plain reteaching, with one exception in programming.** Human tutors who pointed out and explained each algebra error did the same as tutors who simply retaught the procedure (38 students, t(36) = .09). Two randomized trials of bug-specific messages in ASSISTments (222 and 649 students) changed nothing. Errors are unstable: students shift from one bug to another. In an introductory programming course, misconception-driven feedback raised a mid-unit test (7.19 against 5.91 points) and programming success (41 against 32 percent), and the final multiple-choice test came out equal. Grade A and B. [Sleeman1989] [Selent2015] [VanLehn1983] [Gusukuma2018]
15. **Text that names a misconception and refutes it beats plain exposition.** Refutation text: g = 0.41 over 44 comparisons (3,869 participants), stable across contexts, with effects that last weeks to months in the studies that tested delay. Grade A. [Schroeder2022] [Tippett2010]
16. **Feedback works in proportion to the information it holds.** In computer-based learning: elaborated feedback 0.49, the correct answer 0.32, right or wrong only 0.05 (40 studies, 70 effect sizes). For higher-order outcomes the overall effect was 0.59 against 0.31 for lower-order. Across 435 studies: high-information feedback d = 0.99, corrective 0.46, reward or punishment 0.24. Grade A. [VanderKleij2015] [Wisniewski2020]
17. **Feedback often backfires, and praise is the weakest form.** Kluger and DeNisi: average d = 0.41 and more than one third of feedback interventions lowered performance. Feedback loses force as it moves attention from the task to the self. Feedback with praise: 0.09, without: 0.34. Wisniewski et al. found 17 percent of effects negative. Grade A. [Kluger1996] [Hattie2007] [Wisniewski2020]
18. **Immediate feedback wins in applied and computer settings, delayed feedback wins in some lab studies of retention.** Computer-based meta-analysis: immediate 0.46, delayed 0.22. Lab study on multiple-choice tests: delayed feedback gave .70 against .60 correct after one week. Feedback after a right answer also helps (.93 against .79 retained). Letting students try again with a hint gave larger gains than showing the answer, and open-ended questions gave larger gains than multiple choice. Grade A and B. [VanderKleij2015] [Kulik1988] [Butler2007] [Butler2008] [Attali2015]
19. **The praise-for-effort result is much weaker than first reported.** Mueller and Dweck 1998 (six studies, 412 fifth graders) found large harm from intelligence praise. A close replication with 624 children found a small effect in one study (beta = -0.28, p = .049) and none in two more, and none of the eight motivation measures replicated. With 111 college students, process praise beat person praise on motivation after failure. Grade A and B. [Mueller1998] [Li2019] [Haimovitz2011] [Shute2008]
20. **A student's level can be read in a minute or two.** Asking for only the first step of a solution correlated up to .92 with full tests and cut test time by a factor of 2.5 to 4.9. Instruction adapted with such probes beat a yoked non-adapted version (30 students, d = 0.69 on efficiency). Grade B. [KalyugaSweller2004] [KalyugaSweller2005] [Kalyuga2005]
21. **A pretest teaches what it asks about and little else.** Prequestions: g = 0.54 for the tested material, g = 0.04 for the rest (97 and 91 effect sizes). Guessing an answer (g = 0.65) beat reading the question (g = 0.22). In the original study, pretested items were recalled at 75 percent against 56 percent with extra study time. Grade A. [StHilaire2024] [Richland2009] [Pan2023]
22. **Extra practice in the same sitting buys nothing that lasts, and known material can be skipped.** College students who solved 9 problems instead of 3 scored the same 1 and 4 weeks later. Cutting over-practice in the Cognitive Tutor saved 12 percent of time (22 minutes) with equal posttest and two-week retention scores. Removing 40 to 50 percent of the curriculum for students who had shown mastery left test scores unchanged (336 students). Pretest-based paths for 2,064 students cut time sharply, with outcomes equal in most modules and lower in two of seven. Grade A and B. [Rohrer2006] [Cen2007] [Reis1998] [ITSE2026] [Driskell1992]
23. **The success-rate target has thin support at any exact number, and the best evidence points below 95 percent.** Rosenshine's 80 percent comes from classroom correlations (82 against 73 percent). Wilson et al.'s 85 percent is a mathematical result for binary classification by gradient descent, shown in simulations, and it moves to 82 or 75 percent under other noise assumptions. In a randomized trial with about 37,000 learners, a 95 percent target gave the weakest learning and the best short-term retention of users, targets of 80, 65 and 50 percent learned alike, and learners rated practice as "appropriate" most often near 65 to 70 percent success. Grade A for the trial, D for the rules of thumb. [Rosenshine2012] [Wilson2019] [Papousek2016] [Lomas2013]
24. **Choosing the next task from a running estimate of the student's knowledge is the best-supported kind of adaptivity.** Cognitive mastery (keep practicing a skill until estimated knowledge reaches 0.95) added 0.89 SD over a fixed problem set at a cost of 40 percent more problems. Of 36 studies of reinforcement-learning sequencing, 21 beat all baselines, mostly where a psychological model of memory drove the policy. Personalized spaced review raised end-of-course retention by 16.5 percent over massed study and 10.0 percent over one-size spacing. Interleaving: g = 0.42 overall, 0.34 for mathematics, -0.39 for word learning. Grade A and B. [Corbett2001] [Aleven2016adapt] [Doroudi2019] [Lindsey2014] [Brunmair2019]
25. **Handing control to the learner neither helps nor hurts on average, and prior knowledge is the learner variable worth adapting to.** Learner control: g = 0.05 (18 studies) and -0.03 (24 studies). Learning-style matching: 9 of 65 contrasts significant. Showing students their own skill estimates helped learning in a small study and helped only with shared control over problem choice in a larger one. Grade A. [Karich2014] [Niemiec1996] [Aleven2016adapt] [Pashler2008] [Long2017]

## Findings by topic

### 1. Stuck or struggling productively (Q7, bears on H7 and H9)

#### 1.1 Wheel-spinning: definition and how common it is

- **Finding:** Wheel-spinning is practice that goes on without reaching mastery. The standard operational form is: 10 practice opportunities on one skill in ASSISTments (15 in the Cognitive Tutor) without three correct answers in a row. Past that point further practice almost never produces mastery.
- **Evidence:**
  - In ASSISTments, 45 percent of student-skill instances had not shown mastery after 10 problems, and after 20 more problems only another 4 percent had. That gives a conditional probability of 0.04 / 0.45 = 0.089 of ever mastering the skill by practice alone. About 40 percent of pairs in ASSISTments and 35 percent in the Cognitive Algebra Tutor were still short of mastery at the end of the data (up to 30 opportunities). [Gong2014]
  - Because many students stop before the checkpoint, the true rate is bounded: 8 to 45 percent of skill practice in ASSISTments and 8 to 35 percent in the Cognitive Algebra Tutor. The authors' model puts it at 18 percent of student-skill pairs in ASSISTments and 25 percent in the Cognitive Algebra Tutor. [Gong2014]
  - Time cost: wheel-spinning took between 18.3 and 54.5 percent of learning time in ASSISTments. In the Cognitive Algebra Tutor the optimistic figure was 27.4 percent and the model estimate 31.1 percent. [Gong2014]
  - Wheel-spinning overlapped with gaming the system: 28 percent of problems in one data set were wheel-spinning problems, and 3 percent were both gamed and wheel-spinning. [Gong2014]
  - In three newer data sets the rate under the same rule was 6.6 percent and 0.56 percent of student-skill pairs in two MATHia algebra sections and 10.2 percent in a geometry tutor. Under the Predictive Stability++ rule the rates were 24.2, 2.17 and 13.2 percent. [Zhang2019]
- **Caveats:**
  - All of this is observational log data from middle and high school mathematics.
  - The 10-opportunity number came from the design of ASSISTments, which stops a student after 10 problems in a day. Kai et al. say so directly. [Kai2018]
  - The rule ignores skill difficulty: some skills need more than 10 opportunities. [Kai2018] [Zhang2019]
  - Fancsali et al. argue that the construct is "ill-defined", that published rules do not match how real systems deliver practice (MATHia tracks several skills per problem and has its own mastery rule), and that conclusions from several Cognitive Tutor studies are "difficult to interpret at best". [Fancsali2020]
- **Grade:** B.
- **Sources:** [Beck2013] [Gong2014] [Gong2015] [Zhang2019] [Kai2018] [Fancsali2020]

#### 1.2 Detecting wheel-spinning early

- **Finding:** Simple correctness features detect future wheel-spinning after about four attempts. Before the fourth attempt detection is close to useless. Precision is better than recall.
- **Evidence:**
  - Gong and Beck's detector (logistic regression on generic features such as prior problem count, percent correct, hint use and response speed) reached 71.1 percent precision in the Cognitive Algebra Tutor and 76.6 percent in ASSISTments for the wheel-spinning class, with recall just above 50 percent (53.1 percent in ASSISTments). A real-time detector built on it would raise a false alarm 23 to 29 percent of the time. [Gong2014]
  - Zhang et al. compared six detectors on three data sets and two definitions. At step 4, Random Forest reached 77.2 percent precision and 63.5 percent recall under the three-in-a-row rule, and 90.8 and 81.4 percent under Predictive Stability++. A one-feature logistic regression on "Correct Response Percentage" reached 93.5 percent precision and 77.1 percent recall at step 4 under Predictive Stability++. Most models passed 60 percent on both measures after step 4 and 80 percent after step 6. In the first three steps almost all models were below 20 percent on both under the three-in-a-row rule. [Zhang2019]
  - The four features that kept ranking highest: correct response count, correct response percentage, response time relative to other students (z-score), and count of prior steps answered correctly at a normal speed. The "assistance score" (errors plus hints on a step) reached 68.4 percent precision and 60 percent recall at step 4. [Zhang2019]
  - The two definitions agreed on only 22.2, 14.1 and 41.6 percent of the cases either one flagged. [Zhang2019]
- **Caveats:** The authors say they "are not trying to recommend using the fourth step as a definitive criterion". The one-feature model misses students who get the first four wrong and the next four right. All data are from step-based mathematics tutors with many short steps per skill, which a lesson with one or two checks per milestone does not resemble.
- **Grade:** B.
- **Sources:** [Zhang2019] [Gong2014] [Gong2015]

#### 1.3 Productive persistence against wheel-spinning

- **Finding:** Among students who keep going past 10 problems, most turn out to be learning. What separates the two groups is a pattern of help use, and the separation is weak.
- **Evidence:**
  - Data: 23,896 students, 298 middle schools, 619 Skill Builder problem sets, school year 2014 to 2015. "Productive persistence" = three right in a row on or after the 10th problem plus a passed delayed retention test. By that rule 9.1 percent of 75,481 student-problem-set pairs were wheel-spinning. Under Beck and Gong's rule it would have been 46.9 percent of those pairs (12.3 percent of all 287,093 pairs). [Kai2018]
  - A J48 decision tree with 15 features reached AUC 0.684 with cross-validation by student-skill pair. An erratum reports about 0.05 lower (0.636) with cross-validation by student. A 3-feature version reached 0.628. [Kai2018]
  - Two signatures of wheel-spinning: (1) zero hints requested on at least one problem together with more than one bottom-out hint in the last 8 problems - 2,544 of 2,856 such pairs (89.07 percent) were wheel-spinning; (2) zero hints on at least one problem, at most one bottom-out hint, and little variation in the time between sessions on the problem set (standard deviation at most 2.53 days) - 5,027 of 6,457 pairs (77.85 percent). [Kai2018]
- **Caveats:** The population is already selected (only students who reached 10 problems). Passing one retention item is a noisy label. The spaced-return feature reflects ASSISTments' rule that the 11th problem waits for another day.
- **Grade:** B.
- **Sources:** [Kai2018]

#### 1.4 A deployed real-time indicator set with a learning benefit

- **Finding:** A classroom tool that shows teachers live "stuck" and "misuse" indicators from a tutoring system raised learning. Its thresholds are published.
- **Evidence:** Lumilo (smart glasses for teachers using an algebra tutor) flags: unproductive persistence (the wheel-spinning detectors above), hint abuse and hint avoidance, gaming, rapid non-deliberate attempts, idle for 2 minutes or more, a high or low recent error rate (under 30 percent or over 80 percent correct in the most recent 10 attempts), and errors on a step after every hint for that step has been used. In a 3-condition classroom experiment (343 students enrolled, 286 analyzed, 18 classrooms, 8 teachers), the full tool raised learning gains over business as usual (r = 0.21, 95 percent CI 0.13 to 0.28), and analytics added to plain monitoring (r = 0.11). Teachers with analytics spent more time with low-prior-knowledge students. [Holstein2018]
- **Caveats:** The response to each flag came from a human teacher. The study shows that the flags pick out moments where help pays, and it leaves open what an automated tutor should do at each flag.
- **Grade:** A (cluster-randomized classroom experiment).
- **Sources:** [Holstein2018]

#### 1.5 Being stuck in a chat with an LLM tutor

- **Finding:** In real dialogues with a guarded LLM tutor, how long a student has been stuck predicts recovery, and the useful tutor move changes with that length. This is the only source here that works from chat turns, which is the signal the Pedagogue has.
- **Evidence:**
  - Setting: a GPT-4o tutor that coaches university general-chemistry students through planning a solution, with a prompt that forbids solving the problem, asks one question at a time, and calls for "increasingly specific, targeted hints" after a wrong answer. 1,260 sessions, 20,462 student turns, 8 problems. [Ahtisham2026]
  - Impasse turns were defined from the text alone: (a) a conceptual error (answer coded completely incorrect, partially incorrect or not specific enough), (b) expressed uncertainty ("I think", "maybe", "is it...?"), or (c) help-seeking ("I don't understand", "idk", or a question that proposes no answer). 6,630 impasse turns in 4,170 episodes: 34.9 percent errors, 29.7 percent uncertainty, 35.4 percent help requests. An LLM annotator matched human coding at kappa .79 to .86. [Ahtisham2026]
  - Recovery (a completely correct answer on the next turn) by depth: 39.7 percent at the first stuck turn, 32.6 percent at the second, 26.2 percent at the third, 18.8 percent at the sixth or later. Each extra turn: adjusted odds ratio 0.87 (95 percent CI 0.81 to 0.94). Help-seeking started as the most recoverable type (47.0 percent) and fell hardest (25.3 percent at depth 3, 12.5 percent at depth 6 or more). Conceptual errors settled near 21 to 25 percent by depth 3. Uncertainty stayed near 32 percent through depth 4. [Ahtisham2026]
  - Tutor moves at the current turn: scripted question AOR 2.24, follow-up question 1.95, addressing the incorrect answer 1.36, partial answer 0.81, check for understanding 0.37. Direct answers showed slightly lower next-turn recovery at every depth (-3 to -2 points). [Ahtisham2026]
  - Sequences: first scripted question 55.5 percent recovery. Scripted question after scripted question 28.1 percent (AOR 0.48). Scripted question then addressing the error 39.8 percent. Follow-up after follow-up 33.1 percent. Partial answer after partial answer 19.6 percent. Any sequence ending in a check for understanding 7.9 to 13.6 percent. [Ahtisham2026]
  - With depth, the benefit of a scripted question fell from +20 points at onset to +3 points at the fourth stuck turn, follow-up questions fell from +15 to +3, and addressing the error rose from +5 to +12. [Ahtisham2026]
  - Early dropouts (20 percent of sessions, under 10 turns and never reaching execution) were caught in loops: once in the "relevant concepts" phase they stayed there with probability .74, and 60 percent of their turns after reaching strategy went back to concepts (18 percent for completers). [Ahtisham2026]
  - In a simulation on 150 impasses, an unguarded tutor gave the direct answer in 50.7 percent of replies (70 percent for conceptual errors). A tutor told only "do not give the answer" asked a follow-up question in 100 percent of replies and produced only 3 distinct move combinations. The guided tutor gave a direct answer in 19.3 percent of replies and varied its move by impasse type. [Ahtisham2026]
- **Caveats:** A 2026 arXiv preprint. The analysis is observational, so the links between moves and recovery are associations, and deeper impasses are a selected harder subset. "Recovery" is next-turn correctness, which is performance in the moment and says nothing about retention. The low score for checks for understanding is partly a measurement artifact, since "yes" cannot be coded as correct. One course, one tutor prompt.
- **Grade:** C (large observational log study, peer review pending).
- **Sources:** [Ahtisham2026]

#### 1.6 Productive failure and its conditions

- **Finding:** Letting learners attempt a problem before they are taught improves conceptual understanding and transfer when the attempt is followed by instruction that builds on their attempts. It does nothing for procedural skill, and it works against young children.
- **Evidence:**
  - Meta-analysis: 53 studies, 166 comparisons of problem solving then instruction (PS-I) against instruction then problem solving (I-PS). Conceptual knowledge and transfer: g = 0.36 (95 percent CI 0.20 to 0.51). With high fidelity to productive-failure design: g between 0.37 and 0.58. Procedural knowledge (51 comparisons): g = -0.03 (95 percent CI -0.20 to 0.15). Effects favored instruction first for second to fifth graders and for domain-general skills. [Sinha2021]
  - Kapur 2014, ninth graders learning standard deviation: productive failure against direct instruction, conceptual understanding d = 2.00 and transfer d = 1.52 (75 students), procedural knowledge equal. In a second study (111 students), productive failure beat "vicarious failure" (studying other students' failed solutions) at d = 1.35 and 1.23, and vicarious failure beat direct instruction on concepts only (d = 0.80). The number of solutions a student generated correlated with posttest concepts (r = .82) and transfer (r = .88). The effect sizes, sample sizes and correlations in this bullet are (unconfirmed): the citation check reached only the abstract, which states the direction of each result without these numbers. [Kapur2014]
  - Kapur 2008: eleventh graders who first struggled in groups on ill-structured physics problems produced poor solutions and later beat the well-structured-problem group on individual near and far transfer. [Kapur2008]
  - Conditions named by Kapur: the problem is hard enough to engage and within reach so the learner keeps going; it admits several solutions and representations; it activates prior knowledge, formal and intuitive; and a teacher then compares the student's solutions with the canonical one and points at the critical features. Unguided problem solving with no consolidation after it is "unproductive failure". [Kapur2016]
- **Caveats:**
  - The authors' p-curve "true effect" of g = 0.87 is an estimate after bias correction and a critical summary page argues that it should not be used as the effect size. [Sinha2021]
  - Most studies are in school mathematics and science, many from a small group of labs, and many are quasi-experimental.
  - This is evidence about a planned sequence (attempt first, then teach). It is weaker evidence about a student who is stuck in the middle of practice after being taught.
- **Grade:** A.
- **Sources:** [Sinha2021] [Kapur2014] [Kapur2008] [Kapur2016]

#### 1.7 Confusion, frustration and boredom

- **Finding:** Confusion marks an impasse and predicts learning when the impasse gets resolved. Unresolved confusion turns into frustration and then boredom. Boredom persists and goes with gaming and poor learning.
- **Evidence:**
  - Model: engagement, then an impasse produces confusion; resolution returns the learner to engagement; failure to resolve produces frustration, then boredom. Two studies with a computer tutor (32 to 35 minute sessions, affect judged at about 110 points per session) found the predicted confusion-engagement, boredom-frustration and confusion-frustration oscillations. [DMello2012]
  - Experiments that planted contradictions between two animated agents: posttest scores showed no overall difference between conditions (p = .246), and a significant condition by confusion interaction (p = .021). Learners who reported confusion in the contradiction conditions scored higher than the no-contradiction control. Where the contradiction failed to confuse, it changed nothing. [DMello2014]
  - Across three environments (a dialogue tutor, a puzzle game and an algebra tutor; United States and Philippines), boredom was the most persistent state in all three and was associated with poorer learning and with gaming the system. Frustration was less persistent and less tied to poor learning. Confusion and engaged concentration were the most common states. [Baker2010]
- **Signals available from text and quiz attempts:**
  - AutoTutor dialogue features alone (no sensors): confusion went with early position in a topic, the first few attempts, slower and shorter responses, low answer quality, "frozen" expressions such as "I don't know", less direct tutor help, and negative feedback. Frustration went with later position, long response times, good answers to the immediate question with poor answers on the broader topic, and negative feedback. Boredom came later in the session, after several attempts at the main question, and when the tutor gave more direct moves. Flow went with quick, long, good answers and positive feedback. [DMello2008]
  - Accuracy of detection from those features: against neutral, boredom 69 percent, confusion 68 percent, flow 71 percent, frustration 78 percent (chance 50 percent); four-way without neutral 54 percent (chance 25 percent). [DMello2008]
  - Text markers used as impasse definitions in an LLM tutor: hedges ("I think", "maybe"), general help requests ("idk"), questions that propose no answer, and wrong or vague answers. [Ahtisham2026]
  - From attempt logs: percent correct so far, hints used, bottom-out hints, errors after all hints are used, very fast responses, idle time over 2 minutes, under 30 percent correct in the last 10 attempts. [Zhang2019] [Kai2018] [Holstein2018]
- **Caveats:** The confusion experiments used undergraduates and deliberately planted contradictions. Affect detection from dialogue is moderate at best. The 2008 detectors predate LLMs.
- **Grade:** B.
- **Sources:** [DMello2012] [DMello2014] [Baker2010] [DMello2008]

### 2. Help (Q8, bears on H6, H7 and H16)

#### 2.1 The assistance dilemma

- **Finding:** How to balance giving information against withholding it is an open problem that the authors say needs "specific conditions and parameters". The firmest single result is that withholding right-or-wrong feedback is counterproductive.
- **Evidence:** In the Lisp tutor, immediate feedback, error flagging, feedback on demand and feedback only at the end were compared. The three feedback conditions learned equally and better than the end-only condition. Immediate-feedback students finished about three times faster than end-only students. Students with feedback on demand asked for it only after reaching a preliminary solution in 90 percent of exercises. [Koedinger2007]
- **Caveats:** A review by the tutors' own designers, focused on Cognitive Tutors.
- **Grade:** B.
- **Sources:** [Koedinger2007]

#### 2.2 Help seeking: students use help badly, and hints help only some of the time

- **Finding:** Students both overuse help (clicking to the answer) and avoid it. Help use improves learning mainly at a medium level of skill. Teaching students to seek help better changed their help use and left their domain learning unchanged.
- **Evidence:**
  - Early Geometry Cognitive Tutor data: students viewed 68 percent of the hint levels before the last one for under 1 second, and after 3 errors on a step the next action was a hint request only 34 percent of the time. Raw frequency of hint use correlated negatively with learning gains. [Aleven2016help]
  - The Help Tutor gave real-time feedback on help seeking ("Try to take more time to read the hint"; "Repeated errors may mean that you are not learning... Perhaps ask for a hint?"). Students then spent more time per hint level and asked for fewer levels, even months later. Domain learning did not improve. [Aleven2016help]
  - Roll et al.: 38 high school students on the Geometry Cognitive Tutor (2 months of use, 25,105 problem steps). Skill was split at Pknow 0.4 and 0.6. On low-skill steps 38 percent of actions were unproductive (28 percent inappropriate attempts). Asking for help on challenging steps went with learning in general. On low-skill steps, overusing help went with poorer learning and repeated failed attempts went with better performance on later steps. Hints helped the next opportunity when skill was medium. [Roll2014] [Aleven2016help]
  - Time spent on a bottom-out hint correlates positively with learning (students treat it as a worked example and self-explain). A high rate of bottom-out hints correlated with less learning in another system, and worked examples beat bottom-out hints in several tutor studies. [Aleven2016help]
  - Self-explanation prompts were more effective than on-demand principle-based help in one study, and having both available weakened the prompts. [Aleven2016help]
  - Students rarely ask for a hint before trying a step. They decide to seek help from error feedback. [Aleven2016help]
  - Review of 2003: learners with less prior knowledge need help more and judge their need for it worse. With the QUADRATIC contingent tutor (42 students aged 14 to 15), lower-knowledge students asked for help more and made more errors, and for them more help seeking went with better outcomes. A model of time per operation, errors and help requests explained 63 percent of the variance in pretest scores. [Aleven2003] [Wood1999]
- **Caveats:** The log analyses are correlational and the effect of hints at medium skill "was seen in tutor data but apparently was not strong enough to be measurable at post test". The authors call for replication in other data sets. High school geometry.
- **Grade:** B.
- **Sources:** [Aleven2016help] [Roll2014] [Aleven2003] [Wood1999]

#### 2.3 On-demand against proactive hints, and what a guarded LLM does to learning

- **Finding:** One small experiment favors hints on demand over hints pushed after each error. A large field experiment shows that an LLM that gives answers raises practice scores and lowers later unassisted performance, and that a hint-only version avoids the loss.
- **Evidence:**
  - 72 eighth graders, repeated measures, one class period, two algebra topics. Mean gain 0.137 with hints on demand against 0.04 with proactive hints (F(59,1) = 4.42, p = 0.04, effect size 0.35, 95 percent CI 0.02 to 0.74). The advantage held for students who asked for many hints and vanished for those who asked for few. An aptitude split by math ability showed no interaction. [Razzaq2010]
  - Nearly 1,000 high school math students in Turkey, four 90-minute sessions. Practice scores rose 48 percent with a plain GPT-4 chat ("GPT Base") and 127 percent with a tutor prompted to give hints and equipped with the teachers' solutions and common mistakes ("GPT Tutor"). On the later closed-book exam GPT Base students scored 17 percent below the control group, and GPT Tutor students matched the control group. [Bastani2025]
- **Caveats:** In the hint study the students were used to on-demand hints, the study lasted one period, and the pretest and posttest used the same items. In the GPT study the hint-only tutor matched the control group and did not beat it.
- **Grade:** A (one small experiment, one large field experiment).
- **Sources:** [Razzaq2010] [Bastani2025]

#### 2.4 Contingent tutoring and the levels of help

- **Finding:** The contingent-shift rule (more control after failure, less after success) is the core of the scaffolding idea, with five published levels of help. Its strongest experimental support failed a modern replication.
- **Evidence:**
  - Wood, Bruner and Ross 1976 observed 30 children aged 3 to 5 with a tutor and named six scaffolding functions: recruitment, reduction in degrees of freedom, direction maintenance, marking critical features, frustration control, and demonstration. The paper states that its observations "are not to be regarded as a test of an hypothesis". [Wood1976]
  - Wood, Wood and Middleton 1978 defined five levels: (1) general verbal encouragement, (2) specific verbal information, (3) selection or indicating what to do next, (4) preparing the material so the next move is easy, (5) demonstration. A contingent tutor moves up a level after failure and down after success. Children taught contingently did best after instruction. [Wood1978]
  - Preregistered direct replication, 2021 to 2022: 285 three-year-olds randomly assigned to demonstration, verbal, swing or contingent instruction. Tutor accuracy matched the original (68.9 against 70 percent). The advantage of contingent instruction on the children's independent performance after instruction (outcome, efficiency and autonomy were the measures) failed to replicate. [Smit2025]
  - A 2010 review found the few effectiveness studies positive, mostly one-to-one with simple tasks, and judged effectiveness research "scarce" because scaffolding is hard to measure. Its three defining features: contingency, fading, and transfer of responsibility. A small study (8 students per group) found fully contingent support best for long division at posttest and one month later. [vandePol2010]
- **Caveats:** The original and the replication used toddlers and a block-building task. The replication authors conclude that contingency needs to be defined with the teaching context and individual differences in mind.
- **Grade:** A for the replication, C for the original observations.
- **Sources:** [Wood1976] [Wood1978] [Smit2025] [vandePol2010] [Wood1999]

#### 2.5 Scaffolding in software, and fading

- **Finding:** Computer-based scaffolding has a medium positive effect that holds across ages, and the data show the same effect with and without fading.
- **Evidence:** 144 experimental studies, 333 outcomes, STEM, ill-structured problems. Overall g = 0.46. Scaffolding with fading, adding, both, or neither: effects all positive and statistically alike. Adjustment by performance, by fixed schedule, by learner choice, or none: alike. Generic against context-specific scaffolds: alike (82 percent of outcomes were context-specific). 64.9 percent of outcomes used scaffolding that never changed, and 16.5 percent included fading. The estimate for adult learners was the highest and rests on one outcome. The lowest was middle school at 0.37. [Belland2017]
- **Caveats:** The comparison of fading types is between studies, with few studies in the fading cells. The authors' earlier pilot found higher effects without fading. They say more studies could reveal differences.
- **Grade:** A.
- **Sources:** [Belland2017]

#### 2.6 Worked examples, fading worked steps, and erroneous examples

- **Finding:** For novices, studying worked examples beats solving equivalent problems. Removing worked steps one at a time, last step first, helps the move to independent solving. Fading tied to each student's measured mastery beats fading on a fixed schedule. Examples of errors help conceptual understanding and delayed retention.
- **Evidence:**
  - Sweller and Cooper 1985: worked examples took less time to process, and later problems of the same structure were solved faster with fewer errors. The gains were specific to problems identical in structure to the examples. [Sweller1985]
  - Meta-analysis, mathematics, 43 articles, 55 studies, 181 effect sizes: g = 0.48. Correct examples alone gave larger effects than incorrect examples alone or mixed. Adding self-explanation prompts lowered the effect. Equal benefit for initial acquisition and for practice. [Barbieri2023]
  - The fading sequence: a complete example, then an example with one step left blank, then more blanks, then the bare problem. The first problem-solving demand is a single step. The authors expected fading to help near transfer for certain and were unsure about far transfer. [Renkl2003]
  - Backward fading plus prompts to name the principle behind each step produced medium to large effects on near and far transfer in two experiments, with equal time on task. Experiment 1 had 78 university students. Backward fading (last steps removed first) had earlier been found better than forward fading. [Atkinson2003]
  - Adaptive fading in the Geometry Cognitive Tutor: steps for a theorem are shown worked-out until the knowledge-tracing estimate for that theorem reaches 0.7, then become open steps, and return to worked-out if the estimate falls below 0.5. Lab study (57 ninth and tenth graders in Germany; the classroom study had 51 ninth graders at a vocational school): immediate posttest means 7.13 (adaptive), 5.21 (fixed fading), 5.79 (problem solving); delayed posttest 6.67, 4.50, 4.66. The classroom study replicated the delayed-posttest advantage. Fixed fading and problem solving did the same. [Salden2010]
  - Erroneous examples, 208 sixth and seventh graders learning decimals: equal on the immediate posttest, and ahead on a posttest one week later (d = .62), with better judgment of their own answers (d = .49). Students liked the erroneous examples less. [Adams2014]
  - Incorrect examples in Algebra I: replacing 8 practice problems with examples plus self-explanation kept procedural scores (42 against 41 percent) and raised conceptual scores (63 against 58 percent) with 116 students. In a second experiment incorrect examples, alone or mixed, helped conceptual understanding most. [Booth2013]
  - Self-explanation prompts in general: g = .55 over 69 effect sizes (5,917 participants). [Bisra2018]
- **Caveats:** The two meta-analyses disagree about self-explanation prompts: positive in general (Bisra), negative as a moderator of worked examples in mathematics (Barbieri). Erroneous examples helped in single studies and were weaker than correct examples in the meta-analysis. Salden's samples were small. The benefit of worked examples reverses for learners who already know the material (section 6).
- **Grade:** A.
- **Sources:** [Sweller1985] [Barbieri2023] [Renkl2003] [Atkinson2003] [Salden2010] [Adams2014] [Booth2013] [Bisra2018]

### 3. What effective tutors actually do (bears on H7, H8 and H15)

#### 3.1 How fine the interaction needs to be

- **Finding:** Tutoring that responds at each step of a solution is about as effective as a human tutor. Going finer than steps adds nothing measurable (the "interaction plateau").
- **Evidence:**
  - Review of experiments: against no tutoring, human tutoring d = 0.79 and intelligent tutoring systems d = 0.76 (step-based). The author's table of pairwise comparisons: answer-based against no tutoring 0.31 (165 effects, 40 percent reliable), step-based against no tutoring 0.76 (28 effects, 68 percent reliable), substep-based against no tutoring 0.40 (26 effects), human against no tutoring 0.79 (10 effects), substep-based against step-based 0.16 (11 effects, none reliable), human against step-based 0.21 (10 effects, 30 percent reliable), human against substep-based -0.12 (5 effects). [VanLehn2011]
  - Stated limits: STEM task domains only, normal learners with prerequisites in place, human and computer tutors teaching the same content, one-to-one. [VanLehn2011]
  - Seven experiments in qualitative physics with content held equal: tutorial dialogue (human or computer) beat reading text with large effects when novices studied content written for intermediates. When novices studied novice-level content, or intermediates studied intermediate-level content, dialogue and text did the same, provided the readers had to answer questions as they read. [VanLehn2007]
- **Caveats:** The comparisons in the 2011 table come from different studies with different content, and the substep row is lower than the step row for reasons the review leaves open. The 2007 studies used one domain. Effect sizes for tutoring in general belong to stream Q13.
- **Grade:** A for the review, A for the 2007 experiments.
- **Sources:** [VanLehn2011] [VanLehn2007]

#### 3.2 Telling against prompting

- **Finding:** Students learn as much when the tutor prompts and scaffolds as when the tutor explains, and what students do in response predicts learning better than what tutors say.
- **Evidence:**
  - Naturalistic tutoring of eighth graders on the circulatory system (11 students, untrained tutors): tutors' explanations and students' responses to scaffolding correlated with shallow learning only. The one variable that correlated with deep learning was the student's own reflective comments. Pretest and reading ability explained more than half of the variance (0.558 for shallow, 0.705 for deep). [Chi2001]
  - Second study, same tutors told to prompt and to withhold explanations and feedback: tutor statements per session fell from 621 to 178, explanations from 327 to 7, direct feedback from 94 to 44; scaffolding statements rose from 31 to 100 (5 to 56 percent of tutor statements); student statements rose from 206 to 375. Learning was the same: knowledge pieces 22 to 45 percent; the hardest question categories 35 against 33 percent and 31 against 41 percent (naturalistic against prompting), with the group difference within noise after adjusting for pretest and reading ability. [Chi2001]
  - Ordinary (untrained) tutors follow a 5-step frame: tutor asks, student answers, tutor gives short feedback, the two improve the answer together over several turns (median 5 turns in a research-methods corpus, 10 in an algebra corpus), tutor checks understanding. The Socratic method, inquiry teaching, reciprocal training and modelling-scaffolding-fading were "virtually non-existent" among the 13 tutors. Students started only 5 percent of subtopics in one corpus and 10 percent in the other. Tutors often spliced in a complete or partial answer. They checked understanding with "do you understand?"-style questions 92 percent of the time. [Graesser1995]
- **Caveats:** Chi et al. had 11 students per study and the comparison between styles is across two studies. The 92 percent figure is in Graesser et al. itself (the passage on step 5) and is repeated in a secondary source that summarizes the paper.
- **Grade:** B.
- **Sources:** [Chi2001] [Graesser1995] [vandePol2010]

#### 3.3 The INSPIRE model of expert tutors

- **Finding:** Tutors with demonstrated results share seven traits: Intelligent, Nurturant, Socratic, Progressive, Indirect, Reflective, Encouraging.
- **Evidence (descriptive, from studies of expert tutors working mostly with remedial elementary students):**
  - Socratic: "more than 90% of the remarks that our best tutors make are likely to be in the form of questions". They give hints in place of answers and persist, "offering five or six hints in succession".
  - Progressive: they start with problems chosen to diagnose, then choose problems that give a chance to correct each observed "bug", then raise difficulty once the student is competent and confident, and repeat the cycle. Less effective tutors did not regularly do this.
  - Indirect: they avoid overt negative feedback, and relative to weaker tutors they are "less likely to provide explicit or effusive praise to students, especially praise directed at the person rather than the process".
  - Encouraging: they stress how hard the problem is, which gives the student an excuse for difficulty and raises the value of success.
  - On the balance between clear feedback and praise the authors say: "feedback has to be clear: students must know when their responses are right and when they are wrong", and "praise must always be credible". Tutors who told remedial students they had "a great math mind" after very simple problems got incredulous looks. Where tutor and student have yet to build a relationship, the tutor "may need to make more use of explicit praise, and overt statements of confidence in the student's ability, at the start of a session".
- **Caveats:** Descriptive studies of a small number of tutors, mostly with children who had a history of failure. The authors themselves note the Socratic style looks inefficient and that they rely on outcome data to back it. This is the pattern of successful tutors, with the causal contribution of each trait untested.
- **Grade:** C.
- **Sources:** [Lepper2002]

### 4. Remediation (Q9, bears on H8, H9 and H10)

#### 4.1 Correctives in mastery learning

- **Finding:** Bloom's procedure pairs each formative check with correctives matched to the items missed, then a second, parallel check. Guskey holds that correctives must differ from the first teaching. Students who pass the first check get enrichment.
- **Evidence:** Guskey describes the cycle: units of one to two weeks; a brief formative assessment; correctives "matched to each item or set of prompts within the assessment so that students need work on only those concepts or skills not yet mastered"; a second parallel assessment that "covers the same concepts and skills as the first, but includes slightly different problems", which checks that the correctives worked and gives a second chance at success; enrichment for those who passed. "By itself, however, feedback does little to help students improve their learning. Significant improvement requires feedback to be paired with correctives." Correctives should offer alternative approaches and additional time. Correctives done as homework "rarely experience success". [Guskey2007]
- **Caveats:** This is an expert's account of a method. Guskey's text ties "different" to learning styles and modalities, which the learning-styles literature does not support (section 9). The effect size of mastery learning as a whole belongs to stream Q2.
- **Grade:** D for the claim that correctives must be qualitatively different.
- **Sources:** [Guskey2007]

#### 4.2 Bug diagnosis against plain reteaching

- **Finding:** Students' procedural errors are systematic enough to catalogue, and unstable enough that fixing the catalogued bug has usually done no better than reteaching the correct method.
- **Evidence:**
  - Brown and Burton's BUGGY showed that wrong answers in subtraction come from systematic "bugs". Later work counted 77 bugs for subtraction alone. Repair theory explains them: a student whose procedure reaches an impasse applies a local "repair" (skip a step, back up) and may apply a different repair on the next similar problem, so the same student shifts between bugs over a short time ("bug migration"). [VanLehn1983] [Selent2015]
  - Sleeman et al. compared model-based remediation (the tutor points out each error and explains it before giving the correct procedure) with reteaching (the tutor just reteaches the procedure, with the student's earlier work taken away). 44 students aged 13 to 14 were randomly assigned and tutored for about 35 minutes; 38 were analyzed. Both groups improved (t(37) = 4.20, p < .001) and they did the same (t(36) = .09, p > .92). Two months later scores had fallen back to pretest level in both groups. The abstract sums up three studies: both forms beat no tutoring, and model-based remediation "is not clearly more effective than reteaching". In an earlier system study only 30 percent of student errors could be diagnosed at all. [Sleeman1989]
  - ASSISTments, one-step equations, two randomized trials. Study 1: large color-coded text messages specific to the predicted wrong answer, added to on-demand hints (222 students). Study 2: 20 to 30 second videos explaining the process behind the wrong answer, replacing the hint button (649 students). Completion rates (64 against 68 percent; 88 against 86 percent), problems to mastery (6.0 against 5.8; 7.3 against 6.9) and performance on the next problem after a first error were all the same across conditions. [Selent2015]
  - Introductory programming for non-majors: misconception-driven feedback (program analysis detects a mistake and the message addresses the misconception behind it) against the default feedback. Mid-unit multiple-choice test 7.19 against 5.91 (p < .0001, r = 0.37). Final multiple-choice test 7.73 against 7.86, the same. Programming problems solved correctly: 41 against 32 percent across three problems and 48 against 38 percent on the final two, a small effect. Roughly 220 treatment and 70 control students across semesters. [Gusukuma2018]
  - Shute's review draws the same lesson: "the cost of conducting extensive error analyses and cognitive diagnosis may not provide sufficient benefit to learning", and error analyses "are rarely complete and not always accurate". [Shute2008]
- **Caveats:** Sleeman's students had been taught algebra as rote procedure, and the authors suggest reteaching did well for that reason. Selent and Heffernan's controls already had good hints or the answer. Gusukuma's groups came from different semesters, so assignment was by cohort.
- **Grade:** A for the two ASSISTments trials and Sleeman's randomized comparison, B for the programming study.
- **Sources:** [VanLehn1983] [Sleeman1989] [Selent2015] [Gusukuma2018] [Shute2008]

#### 4.3 Conceptual change and refutation text

- **Finding:** For a wrong belief (as opposed to a slipped procedure), naming the belief, saying it is wrong, and explaining why works better than explaining the right idea alone.
- **Evidence:**
  - Posner et al.: four conditions for a learner to replace a central concept: dissatisfaction with the existing conception, and a new conception that is intelligible, initially plausible, and fruitful. Learners change concepts only when they believe "less radical changes will not work". [Posner1982]
  - Meta-analysis of between-subject experiments: 33 studies, 44 comparisons, 3,869 participants. Refutation text against other text with the same content: g = 0.41 (95 percent CI 0.30 to 0.51), heterogeneity I2 = 60.76. The authors call the effect consistent across a wide variety of contexts. [Schroeder2022]
  - Narrative review: refutation text is more likely than expository text to produce conceptual change; an expository format works better than narrative or cartoon; activating prior knowledge helps when paired with a warning that it may be wrong and can hinder when the prior knowledge is wrong and goes unflagged; effects lasted two weeks to two months in the studies that tested delay, from grade 3 to preservice teachers. [Tippett2010]
- **Caveats:** Science topics and reading tasks, mostly with school and university students. The meta-analysis compares text formats, so it says little about refutation delivered in dialogue after a specific wrong answer. Posner's conditions are theory.
- **Grade:** A for the meta-analysis, D for the four conditions.
- **Sources:** [Posner1982] [Schroeder2022] [Tippett2010]

#### 4.4 A second explanation from a new angle against a repeat

- **Finding:** The evidence on repeating an explanation against reframing it is indirect. Three lines bear on it, and a head-to-head experiment is still to be found.
- **Evidence:**
  - Remedial explanations adapted to the specific inference a learner failed to draw, combined with a prompt to revise the earlier answer, raised conceptual knowledge over the same lesson without them (40 eighth graders, d = 0.57). Revision prompts beat general "engaging" prompts at getting learners to use the remedial explanation. In the same paper, explanations with a gap that learners had to fill by inference beat complete explanations (d = 0.70, 83 students). [Roelle2015]
  - In LLM tutor logs, repeating the same kind of move after it failed once went with half the odds of recovery, and switching to a move that addresses the student's error held recovery steady (section 1.5). [Ahtisham2026]
  - Reteaching the correct procedure from scratch did as well as bug-specific remediation (section 4.2). [Sleeman1989]
  - Mastery-learning authors prescribe a different approach for correctives (section 4.1), and practitioner guides say the same ("reteach, don't repeat": change the representation, the analogy or the concrete example, and check again). [Guskey2007]
- **Caveats:** Each of these mixes "new angle" with "more time" or "more interaction".
- **Grade:** B for Roelle et al., C to D for the rest.
- **Sources:** [Roelle2015] [Ahtisham2026] [Sleeman1989] [Guskey2007]

#### 4.5 Going back to a prerequisite

- **Finding:** Stepping back to a prerequisite after failure rests on the structure of mastery learning and on deployed systems. A controlled comparison with "stay and reteach" is still to be found.
- **Evidence:**
  - Beck and Gong raise the cause directly: a student may lack the knowledge needed to solve problems on the skill, and the tutor's teaching actions may not supply it. [Beck2013]
  - A deployed adaptive training platform for utility employees routes learners "back to prerequisite nodes when performance on a node falls below the threshold", using a prerequisite graph written by curriculum experts. 97.1 percent of employees were rerouted at least once (mean 3.62 reroutings). The bundled system beat uniform training by d of about 0.29 on knowledge, with the advantage intact at 6 weeks. The design cannot separate rerouting from the rest of the system. [Frontiers2026]
  - A bandit sequencer (ZPDES) keeps a "zone" of activities on an expert-drawn prerequisite graph and, when an activity yields no progress, widens the zone to another exercise type and temporarily removes the harder activity. [Clement2015]
  - Murray and Arroyo note that skipping an unmastered unit means "missing prerequisites will eventually have to be dealt with when they are needed". [Murray2002]
- **Grade:** C.
- **Sources:** [Beck2013] [Frontiers2026] [Clement2015] [Murray2002]

### 5. Feedback (Q10, bears on H5, H15 and H18)

#### 5.1 How large the effect is and when it backfires

- **Finding:** Feedback has a medium positive effect on average with very wide spread, and a meaningful share of feedback interventions lower performance. What decides the direction is where the feedback points attention.
- **Evidence:**
  - Kluger and DeNisi: 607 effect sizes, 23,663 observations, average d = .41, and over one third of feedback interventions decreased performance. Feedback intervention theory: effectiveness falls as attention moves "closer to the self and away from the task". Moderators that held up: discouraging feedback weakens the effect; feedback on progress since earlier trials ("velocity") and feedback that gives the correct solution strengthen it. [Kluger1996]
  - Hattie and Timperley's retelling of the same data: 470 effect sizes from 131 studies, average 0.38, 32 percent negative. Feedback about changes from previous trials 0.55 against 0.28 without; feedback designed to discourage -0.14 against 0.33; praise 0.09 against 0.34 without praise. [Hattie2007]
  - Hattie and Timperley's synthesis of 12 meta-analyses (196 studies, 6,972 effect sizes): average 0.79. By type: cues 1.10, corrective feedback 0.37, reward 0.31, punishment 0.20, praise 0.14, programmed instruction -0.04. Their model has four levels: task, process, self-regulation, and self. They judge the first three useful and the self level (personal praise) rarely useful. [Hattie2007]
  - Wisniewski, Zierer and Hattie: 435 studies, 994 effect sizes, more than 61,000 participants. Average d = 0.55, or 0.48 after dropping 35 extreme values. 17 percent of effects negative. I2 = 86 percent. By type: reinforcement or punishment 0.24 (39 effects), corrective feedback 0.46 (238 effects), high-information feedback 0.99 (42 effects). Larger effects on cognitive and motor outcomes than on motivation and behavior. [Wisniewski2020]
- **Caveats:** Many of Kluger and DeNisi's studies were outside classrooms. The 0.79 average is an average of meta-analyses and the later direct meta-analysis puts it at 0.48. The counts of effect sizes for Kluger and DeNisi differ between their abstract and Hattie and Timperley's table.
- **Grade:** A.
- **Sources:** [Kluger1996] [Hattie2007] [Wisniewski2020]

#### 5.2 Kinds of feedback in computer-based learning

- **Finding:** In item-by-item computer feedback, an explanation beats the correct answer, which beats right-or-wrong alone. The gap is widest for higher-order outcomes.
- **Evidence:** 40 studies, 70 effect sizes (range -0.78 to 2.29). Elaborated feedback 0.49 (53 effects), knowledge of correct response 0.33 (9 effects, abstract rounds to 0.32), knowledge of results 0.05 (8 effects). Immediate 0.46 (58 effects), delayed 0.22 (12 effects). Lower-order outcomes 0.31, higher-order 0.59. University or college 0.44 (56 effects), primary and high school 0.34. Mathematics 0.93, social sciences 0.46, science 0.40, languages 0.25. Right-or-wrong feedback on higher-order outcomes: -0.01 (one effect). [VanderKleij2015]
- **Caveats:** The authors note these may be underestimates because elaborated feedback was sometimes compared against simpler feedback, and that the interaction of timing with outcome level did not reach significance. Most samples are university students.
- **Grade:** A.
- **Sources:** [VanderKleij2015]

#### 5.3 Timing

- **Finding:** Immediate feedback does better in applied settings and computer-based practice. Delayed feedback does better in some laboratory studies where every learner is made to process the feedback and the test comes days later.
- **Evidence:**
  - Kulik and Kulik, 53 studies: applied studies with real classroom quizzes "usually found immediate feedback to be more effective than delayed"; experimental studies of acquisition of test content "usually produced the opposite result"; list-learning studies were mixed. Average effect 0.28 as tabulated by Hattie and Timperley. [Kulik1988] [Hattie2007]
  - Butler, Karpicke and Roediger: undergraduates read passages and took a multiple-choice test. Feedback of any kind raised final recall over testing without feedback (.70 against .51, d = .91). Standard feedback and answer-until-correct gave identical recall (.71 and .71). Delayed against immediate: .73 against .68 after one day (10-minute delay, within noise), and .70 against .60 after one week with feedback delayed by a day (d = .47, 40 students). [Butler2007]
  - Shute's guidance tables: immediate feedback for difficult tasks and for retention of procedural or conceptual knowledge; delayed feedback for relatively simple tasks and to promote transfer ("this needs more research"); immediate, directive feedback with scaffolding for low-achieving learners; delayed, facilitative feedback for high-achieving learners, for whom verification may be enough. [Shute2008]
  - Cognitive Tutor experience: immediate right-or-wrong feedback after each step gave the same learning as flagged or on-demand feedback and much faster completion. [Koedinger2007]
- **Caveats:** In the lab studies learners were made to look at delayed feedback for a fixed time. In a live product a learner can skip it.
- **Grade:** A for the meta-analyses, B for the lab study, D for Shute's learner-level guidance.
- **Sources:** [Kulik1988] [Butler2007] [Shute2008] [VanderKleij2015] [Koedinger2007]

#### 5.4 Feedback after right answers, retries, and question format

- **Finding:** Feedback helps after correct answers too, especially uncertain ones. A second try with a hint beats being shown the answer. Open-ended questions produce more learning from the same feedback than multiple choice.
- **Evidence:**
  - Initially correct multiple-choice answers were retained better with feedback than without (.93 against .79, d = 0.76), and the gain was concentrated in low-confidence correct answers. Initially wrong answers were corrected at .82 with feedback against .03 without. [Butler2008]
  - Attali: mathematics problems as multiple choice or open-ended, with four feedback types. Gains on similar later problems: no feedback and immediate correct answer were similar; multiple tries then the correct answer beat the immediate correct answer; multiple tries with a hint after the first wrong answer beat multiple tries with the correct answer. Open-ended gave larger gains than multiple choice. [Attali2015]
  - A related study with eighth graders found that feedback-and-revise improved measurement precision for open-ended items and left multiple-choice items unchanged, which the authors explain by more effortful engagement with open-ended items. [Attali2016]
  - On multiple-choice items specifically, answer-until-correct and showing the correct answer gave identical later recall. [Butler2007]
- **Caveats:** Attali is read from the abstract. Scoring of second and third attempts belongs to stream Q3.
- **Grade:** A.
- **Sources:** [Butler2008] [Attali2015] [Attali2016] [Butler2007]

#### 5.5 Praise

- **Finding:** Praise is the least effective kind of feedback for performance. Praise aimed at the person or at ability brings a risk after failure, with a much smaller effect than the famous study reported. Praise for process is safer. Expert tutors use little explicit praise.
- **Evidence:**
  - Meta-analytic: praise 0.14 (11 meta-analyses), teacher praise 0.12; feedback containing praise 0.09 against 0.34 without. Kluger and DeNisi found praise and discouraging feedback both weakened feedback effects, which they explain by attention shifting to the self. [Hattie2007] [Kluger1996]
  - Mueller and Dweck: six studies, 412 fifth graders. Children praised for intelligence chose performance goals (69 percent, against 12 percent of those praised for effort), and after failure showed less persistence, less enjoyment, more low-ability attributions and worse performance. [Mueller1998]
  - Li and Bates replication: 624 Chinese children aged 9 to 13. Study 1 followed the original closely: effect on post-failure performance p = .049, beta = -0.28 (the original difference was about 1.3 SD), and none of the eight motivation and attribution measures replicated (mean p = 0.48). Studies 2 and 3 added an active control: the effect was within noise for moderate and for hard material (p = .189 to .974), with one significant effect in the reverse direction. [Li2019]
  - Adults: 111 college students did puzzles with person praise, process praise or none, then failed. Process praise raised intrinsic motivation and perceived competence more than person praise. Person praise lowered motivation for sophomores and juniors, process praise raised it for seniors, and freshmen showed no difference. [Haimovitz2011]
  - Review: with adults, praise raised time on task, interest and skilled performance in several studies. The authors' conditions for helpful praise: seen as sincere, attributes success to controllable causes, supports autonomy, conveys competence without social comparison, sets attainable standards. [Henderlong2002]
  - Expert tutors praise less than weak tutors and aim what praise they give at the process (section 3.3). [Lepper2002]
  - Shute's guideline: "Use praise sparingly, if at all." [Shute2008]
  - An LLM tutor's deployed prompt told it to "not praise incorrect answers", and its replies contained praise or encouragement 35.3 percent of the time. [Ahtisham2026]
- **Caveats:** The meta-analytic praise numbers measure performance. Motivation and persistence over weeks are measured far less often. The adult evidence is one study of 111 students plus older small studies. Effects of a progress display (such as a success strip) are a different thing from spoken praise, and a study that tests one against the other is still to be found.
- **Grade:** A for the meta-analytic numbers and the replication, B for the adult study, C for tutor practice.
- **Sources:** [Hattie2007] [Kluger1996] [Mueller1998] [Li2019] [Haimovitz2011] [Henderlong2002] [Lepper2002] [Shute2008]

### 6. Acceleration (Q11, bears on H12, H13 and H14)

#### 6.1 The expertise reversal effect

- **Finding:** Instructional help that raises learning for people who know little lowers it for people who know a lot. The effect is robust and lopsided: the gain for novices is larger than the loss for experts.
- **Evidence:**
  - Definition: techniques that are "highly effective with inexperienced learners can lose their effectiveness and even have negative consequences when used with more experienced learners". The proposed mechanism: knowledgeable learners must reconcile redundant guidance with what they already hold, which uses up working memory even when they try to ignore the guidance. [Kalyuga2003] [Kalyuga2007]
  - Examples from the 2007 review: the advantage of worked examples over problem solving disappeared as trainees gained experience and, after intensive training, problem solving became better for the more complex tasks. Narrated explanations of a diagram that helped novices became redundant, and after several sessions the diagram alone did better. [Kalyuga2007]
  - Meta-analysis: 60 experimental studies, 176 effect sizes, 5,924 participants, all with random assignment to more or less instructional assistance. Low prior knowledge: high assistance better, d = 0.505. High prior knowledge: high assistance worse, d = -0.428 (95 percent CI -0.647 to -0.209). Difference between the two within studies: d = 0.971 (95 percent CI 0.631 to 1.312). Heterogeneity is high (I2 about 88 to 91 percent). [Tetzlaff2025]
  - Moderators: how prior knowledge was measured, the educational level of the sample, and the content domain. The effect for low-knowledge learners was especially pronounced in higher education and weak in primary school (few studies). Evidence was less clear in the humanities and language learning. [Tetzlaff2025]
  - The earlier unweighted average difference of 1.72 (Kalyuga 2007) shrinks to 0.97 with meta-analytic weighting. [Tetzlaff2025]
- **Caveats:** "High prior knowledge" in most studies means the better-prepared half of a student sample, which is far from professional expertise. High heterogeneity means the size in any one setting is uncertain.
- **Grade:** A.
- **Sources:** [Kalyuga2003] [Kalyuga2007] [Tetzlaff2025]

#### 6.2 Rapid diagnostic assessment

- **Finding:** A learner's level in a narrow task area can be estimated quickly by asking for only the first step toward a solution, and instruction adapted with such probes outperformed the same instruction delivered without adaptation.
- **Evidence:**
  - First-step method: show a task briefly and ask the learner to give the first step. An expert may give the final answer at once; an intermediate gives the first operation of a detailed procedure; a novice starts a random search. Skipped intermediate steps indicate higher proficiency. [Kalyuga2005]
  - Validation with algebra and geometry in grades 9 to 10: correlation with traditional full-solution tests up to .92 (range .72 to .92 across studies), with test time cut by factors of 2.5 to 4.9. [KalyugaSweller2004] [Kalyuga2005]
  - Scoring example (2005 study): for the equation 4x + 3 = 2, typing the final answer earned 4 points, the step just before it 3, an answer at the level of a simpler equation 2, and a first intermediate step 1. At most one minute per answer. A "don't know" button scored zero. [KalyugaSweller2005]
  - Adaptive use: 30 tenth graders in 15 yoked pairs. The adapted group started at the stage their initial probes indicated (experts went straight to the problem-solving stage with four exercises) and moved between fully worked examples, faded examples and problems according to probes during training. Each control student received the exact sequence of the paired adapted student. The adapted group showed higher knowledge and cognitive-efficiency gains (d = 0.69 for efficiency, t(14) = 1.89, one-tailed p < .05, in the paper's results text). [KalyugaSweller2005]
- **Caveats:** Small samples, school algebra, one research group. The approach suits tasks with a clear step structure.
- **Grade:** B.
- **Sources:** [KalyugaSweller2004] [KalyugaSweller2005] [Kalyuga2005]

#### 6.3 The pretesting effect

- **Finding:** Asking questions before teaching improves later memory for the answers to those questions, even when nearly every pretest answer is wrong. Material the pretest did not ask about gains almost nothing.
- **Evidence:**
  - Richland, Kornell and Kao, five experiments with a two-page essay: pretest answers were 5 percent correct. Final test for pretested items 75 percent against 56 percent for the extended-study group (d = 1.1, experiment 1). Within the pretest group, tested items 75 percent against untested 50 percent. Untested items did the same as in the control group. The effect held when key concepts were italicized or bolded in both conditions (71 against 54 percent; 82 against 64 percent). [Richland2009]
  - Meta-analysis (preregistered): specific effect g = 0.54 (95 percent CI 0.42 to 0.66, 97 effects, 7,902 participants); 85 percent of effects positive. General effect on untested material g = 0.04 (95 percent CI -0.04 to 0.11, 91 effects). Guessing an answer g = 0.65 against 0.22 for reading the question. The general effect was slightly positive with massed study (0.11) and slightly negative with interleaved study (-0.07). [StHilaire2024]
  - Review: the effect appears with text, video, lectures and word pairs, and depends on studying the correct answers afterward. A general benefit is more likely when the untested content is closely related to the pretest questions. The authors call for more research on moderators and classroom practice. [Pan2023]
- **Caveats:** Mostly laboratory studies with immediate or one-week tests. The pretesting effect is a learning effect. Its use as a placement measure is a separate matter covered in 6.4.
- **Grade:** A.
- **Sources:** [Richland2009] [StHilaire2024] [Pan2023]

#### 6.4 Placement, testing out and compressed paths

- **Finding:** Letting learners skip what a pretest shows they know saves a large share of time with learning that is equal in most cases and lower in some. Evidence with working adults exists and is recent.
- **Evidence:**
  - 2,064 upper-secondary students, seven digital-skills (ECDL/ICDL) modules, randomized. In the adaptive path the pretest unlocked only the activities for the gaps it found; in the regular path everything was required. The adaptive path "markedly" reduced instructional time and content exposure in all seven modules. Outcomes: comparable in several modules, better for the adaptive group in Computer Essentials, and better for the regular group in Word Processing and Spreadsheets. The authors suggest practice-oriented modules need richer adaptive logic. [ITSE2026]
  - Manufacturing company employees, randomized with matched pairs: a design-thinking course of about three hours through an adaptive tool that skips content the learner shows they know (55 employees) against a standard LMS course (74 employees). The adaptive group spent almost 45 minutes less and improved 30 percent from pretest to posttest against 17 percent. The paper reports that far more of the adaptive group never started the course (45 against 14 percent). [iJAC2024]
  - Utility company employees, three-wave cluster-randomized field experiment (2,574 analyzed): a knowledge-graph platform with an entry diagnostic that sets a mastery estimate per node, picks the next node among those whose prerequisites are mastered, and reroutes to prerequisites on failure. Against uniform training with the same syllabus and hours: self-efficacy d about 0.26, transfer intention about 0.25, knowledge about 0.29, with the knowledge advantage holding at 6 weeks. [Frontiers2026]
  - Curriculum compacting: teachers removed 40 to 50 percent of the regular curriculum for high-ability students in grades 2 to 6 who showed prior mastery (336 students, national sample). Achievement scores on out-of-grade-level tests matched those of students who did the full curriculum. [Reis1998]
  - Acceleration in K-12 (second-order meta-analysis): accelerated students outperformed same-age peers (g = 0.70), matched older peers (g = 0.09), and across forms of acceleration the effect was g = 0.42. [Steenbergen2016]
  - Vendor practice: Edgenuity's pretest adds 10 questions before each lesson; a student who passes at a set threshold skips the lesson, and one who fails must complete every activity and a quiz. [Edgenuity]
- **Caveats:** The ECDL study reused item pools between pretest and posttest. The two workplace trials compare bundled systems. Compacting and acceleration studies select able students. The longest follow-up in these studies is 6 weeks, so retention of skipped material over months is unmeasured.
- **Grade:** A for the randomized trials, C for vendor practice.
- **Sources:** [ITSE2026] [iJAC2024] [Frontiers2026] [Reis1998] [Steenbergen2016] [Edgenuity]

#### 6.5 Overlearning and over-practice

- **Finding:** Practice beyond the point of first success, done in the same session, gives a boost that fades within weeks. Spreading the same practice over time is what lasts. Tutoring systems give many students more practice than they need.
- **Evidence:**
  - Rohrer and Taylor, experiment 2: college students learned one kind of mathematics problem, then solved 3 or 9 practice problems in one session (accuracy on the first three about 90 percent, on the extra six 95 percent). Tested 1 or 4 weeks later, the two groups scored the same (F < 1). Experiment 1: 10 problems massed against 5 + 5 a week apart gave equal scores at 1 week and a large advantage for spacing at 4 weeks. [Rohrer2006]
  - Meta-analysis of overlearning: 15 studies, 88 tests, 3,771 participants, overall d = .625 (.75 for cognitive tasks as quoted by Rohrer and Taylor), moderated by degree of overlearning, task type and retention interval. Only 7 of 51 cognitive comparisons used a retention interval over one week. In one cited experiment a threefold increase in study gave a 100 percent boost after 2 days and 7 percent after 19 days. [Driskell1992] [Rohrer2006]
  - Cognitive Tutor geometry: learning-curve analysis showed some skills over-practiced and some under-practiced. A version with re-set knowledge-tracing parameters was tested against the original with 110 students, randomly assigned. Learning gains, posttest and a retention test two weeks later were the same (p = 0.772 and p = 0.602). The optimized group saved about 22 minutes, a 12 percent cut in tutoring time (30 percent in one unit). A later summary by the same lab says 20 percent. [Cen2007] [Koedinger2010]
  - Cognitive mastery learning in the Lisp tutor added practice for students who needed it: +0.89 SD over a fixed problem set, for 40 percent more problems and about 15 percent more time. [Corbett2001]
- **Caveats:** Rohrer and Taylor used one procedure and one session. Cen's retention test came two weeks later, and the study had heavy attrition (110 enrolled, 62 with valid logs). How much a strong student can skip before retention months later suffers stays open in these sources.
- **Grade:** A.
- **Sources:** [Rohrer2006] [Driskell1992] [Cen2007] [Koedinger2010] [Corbett2001]

### 7. Difficulty (Q12)

#### 7.1 The 80 percent and 85 percent rules

- **Finding:** The two best-known target numbers rest on thin ground. One is a correlation from classrooms of children, the other is a theorem about a simple kind of learning shown in simulations.
- **Evidence:**
  - Rosenshine: in a study of fourth-grade mathematics, students of the most successful teachers answered 82 percent correctly and students of the least successful 73 percent. "The research also suggests that the optimal success rate for fostering student achievement appears to be about 80 percent." His reasoning: a high rate shows students are learning and still challenged, and a low rate means students practice errors. Effective teachers got there by teaching in small steps with practice after each. [Rosenshine2012]
  - Wilson et al.: for binary classification tasks learned by stochastic gradient descent, with Gaussian noise in the decision variable, the training error rate that maximizes the learning rate is 15.87 percent (about 85 percent accuracy). Demonstrated by simulation on a one-layer perceptron, a two-layer network on handwritten digits, and a model of monkeys learning a motion task. With Laplacian noise the optimum is 82 percent accuracy and with Cauchy noise 75 percent. The authors name open extensions: more than two categories and batch training. They relate the result to flow, desirable difficulties and the region of proximal learning. [Wilson2019]
- **Caveats:** Rosenshine's number is correlational and from young children. Wilson's paper reports simulations and reanalysis of an animal-learning model, with human learners absent from the experiments, and it concerns the speed of learning a perceptual classification, which leaves out motivation, memory over days, and transfer.
- **Grade:** D as rules for human instruction (C for Rosenshine's underlying classroom correlation).
- **Sources:** [Rosenshine2012] [Wilson2019]

#### 7.2 Experiments that set the difficulty

- **Finding:** When difficulty is assigned at random, easier practice keeps people going in the short run and teaches less. Very easy practice (about 95 percent success) is the clear loser for learning.
- **Evidence:**
  - Adaptive geography practice (multiple-choice and open questions), randomized target error rates of 5, 20, 35 and 50 percent, almost 3.3 million answers from about 37,000 learners. Survival after 10 questions: 89.2, 87.0, 84.0, 81.2 percent. Survival after 100 answers: the same in all conditions (26.0 to 26.5 percent). Return after 10 hours: 15.2, 16.0, 16.6, 16.8 percent. Learning on random reference questions: the 5 percent condition was "clearly the worst and differences among the others are small". Self-rated difficulty: "appropriate" most often in the 35 percent condition (66 percent of ratings) and least in the 5 percent condition (55 percent, with 35 percent "too easy"). Across conditions learners called the difficulty appropriate most often when their achieved error rate was around 30 percent, and at least a third wanted it easier or harder. [Papousek2016]
  - The authors' reading for their system: "start with easy questions 'to hook learners up' and then switch to more difficult questions". [Papousek2016]
  - Educational number-line game, two online experiments (10,000 and 70,000 sessions): "in almost all cases, subjects were more engaged and played longer when the game was easier", a linear relation where an inverted U was predicted. "The most engaging design conditions produced the slowest rates of learning." [Lomas2013]
  - Follow-up (over 20,000 sessions): when difficulty was assigned blindly the easiest levels were most motivating. When players chose their difficulty, moderately difficult levels were most motivating, and the inverted U came from lower persistence on levels labeled very easy or very hard. Moderate novelty was best. Suspense in close games helped. The authors' summary: "not too hard, not too boring". [Lomas2017]
- **Caveats:** Papousek's learners practiced facts voluntarily on an open website, with attrition that differed by condition (those who stayed in the easy condition knew less at the start). Lomas's players were children in a game where the unit was a session. The achieved error rates drifted toward each other after about 40 questions.
- **Grade:** A.
- **Sources:** [Papousek2016] [Lomas2013] [Lomas2017]

#### 7.3 The zone of proximal development made operational, flow, and desirable difficulties

- **Finding:** The zone can be defined from help use alone. The flow idea (match challenge to skill) has moderate correlational support that is weaker in education settings. "Desirable difficulty" names specific manipulations that lower performance now and raise learning later.
- **Evidence:**
  - Specific ZPD: three parameters set by the designer. H = the goal number of hints in a set of equivalent problems, DH = the allowed deviation, P = the minimum number of problems. With H* the actual number of hints, more than H + DH hints places the learner in the "confused zone" and fewer than H - DH in the "bored zone". The authors add a second method: ask the learner, with buttons for "confused", "bored", "too difficult", "too easy". They leave the values to "empirical testing". [Murray2002]
  - Flow meta-analysis: 28 studies, 9,620 participants. Challenge-skill balance and flow: Fisher's z = 0.52 (random effects, 95 percent CI 0.38 to 0.62). With intrinsic motivation: 0.32 (25 samples, 4,270 participants). The participant counts and the two pooled values are (unconfirmed): the citation check reached only the abstract, which gives the 28 studies and calls the relationship with flow moderate and the one with intrinsic motivation smaller. Correlations were weaker in work or education contexts, in individualistic cultures, and with experience sampling. [Fong2015]
  - Desirable difficulties, as summarized by Kapur: conditions that hinder performance in the short term (more task variability, unguided problem solving, reduced or delayed feedback) can help learning in the longer term, and conditions that maximize early performance (drill, rote) can produce "an illusion of learning". [Kapur2016]
- **Caveats:** The Specific ZPD paper is a proposal with an evaluation "not yet complete". The flow evidence is correlational and partly inflated by overlapping measures, as a secondary summary of the paper notes.
- **Grade:** D for the Specific ZPD, B for flow, D for the general principle of desirable difficulties as stated here (its components, spacing and testing, belong to stream Q6).
- **Sources:** [Murray2002] [Fong2015] [Kapur2016]

### 8. Choosing and ordering the next task (Q12)

#### 8.1 Mastery-based task selection

- **Finding:** Selecting each next problem from a per-skill estimate of the student's knowledge, and continuing until the estimate passes a threshold, is the form of adaptivity with the strongest record.
- **Evidence:**
  - Cognitive Tutors declare a skill mastered when knowledge tracing puts the probability that the student knows it at 0.95 or better. ASSISTments Skill Builders use three correct in a row without help, and after 10 items without that streak the system makes the student wait until the next day. [Pardos2011]
  - Cognitive mastery learning in the Lisp tutor: students who continued with individually selected problems until each rule was mastered scored 0.89 SD higher than students who did the fixed problem set, with 40 percent more problems and about 15 percent more time. Model tracing (step feedback and hints) had itself added 0.75 SD over conventional problem solving at one third of the time. [Corbett2001]
  - Review judgment: "task-loop adaptivity to students' knowledge growth may well be the single cell in our Adaptivity Grid where adaptive instruction is most effective", with the note that "it would be useful to see one or more replications". An early example: for German vocabulary, the two strategies that chose items from the student's response history beat learner control and random selection on a delayed posttest (Atkinson 1972). [Aleven2016adapt]
- **Caveats:** Corbett's numbers are read from a secondary summary. Knowledge tracing needs several observations per skill, more than a lesson with one or two checks per milestone provides.
- **Grade:** B.
- **Sources:** [Pardos2011] [Corbett2001] [Aleven2016adapt]

#### 8.2 Reinforcement learning and bandits

- **Finding:** Sequencing policies learned by reinforcement learning beat baselines in a little over half of published tests, mainly where items are independent and the policy rests on a psychological model of memory. For interdependent content and choice of activity type the record is weak.
- **Evidence:**
  - Review of 36 studies: 21 found at least one learned policy better than all baselines, 4 found a benefit only for lower-performing students, 4 were mixed, 10 found no difference, 1 favored the baseline. Significant effects were often d of 0.8 or more. Success clustered in paired-associate learning (vocabulary and facts) and concept-learning lab tasks, where models came from psychology. For activity types and interdependent content the models were purely data-driven. The authors' own fractions-tutor policy, chosen by a robust evaluation across several student models, did the same as the baseline. [Doroudi2019]
  - ZPDES bandit: the reward for an activity is learning progress, the success rate over the most recent half of a window minus the success rate over the earlier half. Activities that are already mastered and activities that are out of reach both score zero. The set of eligible activities (the "ZPD") moves along an expert-drawn graph of prerequisites. Tested with 400 children aged 7 to 8 against an expert-designed sequence. [Clement2015]
- **Caveats:** Publication bias likely favors positive results. Most successes involve fact learning. I left out ZPDES outcome numbers because I saw them only in a search summary.
- **Grade:** A for the review, B for ZPDES.
- **Sources:** [Doroudi2019] [Clement2015]

#### 8.3 Spaced-repetition schedulers

- **Finding:** Schedulers that fit a memory model to each learner and item beat fixed-interval rules at predicting recall and, in one classroom study, at raising retention. The practical setting they share is a target recall probability at review time.
- **Evidence:**
  - Leitner and Pimsleur: fixed rules. Pimsleur reviews at exponentially growing intervals; Leitner moves an item to a longer or shorter interval after each right or wrong answer. Both are special cases of half-life regression with fixed weights. [Settles2016]
  - Half-life regression (Duolingo): predicts recall as p = 2^(-delay / half-life), with the half-life learned from each learner's history with each word. On 12.9 million traces: mean absolute error 0.128 against 0.235 for Leitner and 0.445 for Pimsleur, and 0.175 for a constant prediction. Ranking quality (AUC) was near 0.54 for all methods. In live tests the paper reports a 12 percent gain in daily engagement, and Duolingo's blog reports 9.5 percent better retention for practice sessions. [Settles2016]
  - Personalized review in a semester-long middle-school Spanish course (179 students): a Bayesian model of student ability, item difficulty and study history chose review items. On a cumulative exam after the semester it beat massed study by 16.5 percent and generic spaced review by 10.0 percent, with time held equal. [Lindsey2014]
  - MEMORIZE: derived as optimal control of a memory model; the optimal review rate for an item is set by its current recall probability. In a natural experiment on about 5.2 million Duolingo user-word pairs, learners whose reviews happened to follow it forgot more slowly than those on uniform or threshold schedules. [Tabibian2019]
  - SSP-MMC (the research behind FSRS): memory model fit to 220 million review logs from a vocabulary app; in simulation it reached a target 466 days against 533 for a 90 percent recall threshold rule, 569 for Anki's SM-2 variant and 793 for MEMORIZE, a 12.6 percent saving over the threshold rule. [Ye2022]
  - FSRS in practice: schedules each review for the moment predicted recall falls to a "desired retention", 90 percent by default, with 70 to 97 percent called reasonable. On a public benchmark of Anki collections, FSRS v4.5 predicted recall with log loss 0.37 against 0.55 for SM-2. The project's claim of 20 to 30 percent fewer reviews than SM-2 comes with its own warning that it is "based on very old simulation results". [FSRSwiki] [FSRSbench]
- **Caveats:** All of this is for facts and vocabulary reviewed over weeks. The Duolingo and FSRS results are prediction accuracy, simulation, or engagement, with learning outcomes measured only by Lindsey et al. Review schedules belong mainly to stream Q6.
- **Grade:** A for Lindsey et al., B for the model comparisons, C for FSRS project claims.
- **Sources:** [Settles2016] [Lindsey2014] [Tabibian2019] [Ye2022] [FSRSwiki] [FSRSbench]

#### 8.4 Interleaving

- **Finding:** Mixing problem types helps when the types are easy to confuse, and hurts for word learning.
- **Evidence:** 59 studies, 238 effect sizes, 8,466 participants (mean age about 21). Overall g = 0.42 (95 percent CI 0.34 to 0.50). Paintings 0.67, mathematical tasks 0.34, expository texts and tastes unclear, words -0.39 (blocking better). Stronger with higher similarity between categories, lower similarity within categories, and more complex material. [Brunmair2019]
- **Caveats:** Much of the evidence is category learning from visual material.
- **Grade:** A.
- **Sources:** [Brunmair2019]

### 9. Who chooses: the learner or the program (bears on H2)

- **Finding:** Giving learners control over pace, sequence, practice and review has an average effect near zero. Program-chosen items beat learner-chosen items in the classic comparison. Two things shift the picture: letting learners choose their difficulty changes what they find motivating, and showing learners the program's estimate of their skills can help when they also share control.
- **Evidence:**
  - Meta-analysis, 18 studies, 29 effects: g = 0.05. "Near zero effects for all components of instruction (pacing, time, sequence, practice, review)", as quoted from the paper in a secondary summary. Moderate effects only in social studies and for comprehensive programs. Slightly larger for behavioral than academic outcomes. [Karich2014]
  - Earlier meta-analysis, 24 studies: small negative average (-0.03 as tabulated by Visible Learning MetaX). Reviews it summarizes say learner control works less well with younger and less able students. [Niemiec1996]
  - Item selection from response history beat learner control on a delayed vocabulary posttest (Atkinson 1972, as cited). [Aleven2016adapt]
  - With feedback available on demand, students waited until they had a preliminary solution in 90 percent of exercises before asking. [Koedinger2007]
  - Choice of difficulty: moderately hard levels were most motivating when self-selected, and the easiest when assigned. [Lomas2017]
  - Open learner models: skill meters in two constraint-based tutors (SQL and database design, university students) improved the performance of less able students relative to peers without them and helped students choose problems. Inspectable and negotiated models improved children's self-assessment accuracy. [Mitrovic2007] [Kerly2008] [Bull2016]
  - Classroom experiments with an equation tutor (302 students in grades 7 and 8, 2 by 2 design: open learner model, shared control over problem selection). Experiment 1 (56 students): the open learner model raised posttest scores (d = .56), and shared control by itself changed nothing. Experiment 2 (245 students): the main effects were absent and the open learner model helped only when shared control was offered. [Long2017]
- **Caveats:** Most learner-control studies are short and use school or college students. A practitioner summary of Karich et al. points out that long programs, where motivation matters more, are under-studied. A study of an adult learner overriding a mastery gate is still to be found.
- **Grade:** A for the meta-analyses, B for the open learner model studies.
- **Sources:** [Karich2014] [Niemiec1996] [Aleven2016adapt] [Koedinger2007] [Lomas2017] [Mitrovic2007] [Kerly2008] [Bull2016] [Long2017]

### 10. Does adaptivity itself pay?

- **Finding:** Yes on average, with effect sizes that shrink as study quality and scale grow. The learner variable with solid support is prior knowledge and its growth during instruction. Support for matching to learning styles is close to zero.
- **Evidence:**
  - Adaptivity Grid review: three time scales (design loop, task loop, step loop) by five kinds of learner characteristic (knowledge, path through the activity, affect and motivation, self-regulation, learning styles). Strongest evidence: adapting to knowledge, especially in the task loop. Design-loop adaptation (redesigning the course from data on where students struggle) also has "striking demonstrations". Adaptively fading support "may be effective". Adapting to affect has shown effects in all three loops in single studies. [Aleven2016adapt]
  - Learning styles: Pashler et al. found one study that came close to the required crossover evidence and judged it weak, while noting that "an individual student's prior knowledge is bound to determine what level and type of instructional activities are optimal". In computer-assisted instruction, 9 of 65 contrasts (14 percent) showed a significant style by treatment interaction. [Pashler2008] [Aleven2016adapt]
  - The expertise reversal meta-analysis presents itself as evidence for one aptitude-treatment interaction that does hold. [Tetzlaff2025]
  - Meta-analyses of adaptive systems: AI-enabled adaptive learning against non-adaptive, 45 studies, g = 0.70 on cognitive outcomes, moderated by publication type, study origin, duration and design. University students, 51 studies: g = 0.751 cognitive, 0.542 non-cognitive. Personalized learning technology in low- and middle-income countries, 16 randomized trials, 53,029 learners aged 6 to 15: 0.18 overall, and 0.35 for systems that adapt to the learner's level. [Wang2024] [WangJ2023] [Major2021]
  - Workplace: knowledge-graph adaptive training for utility employees, cluster-randomized, d about 0.29 on knowledge. [Frontiers2026]
  - Reinforcement-learning sequencing: four studies found benefits only for low performers. [Doroudi2019]
  - In the on-demand hint experiment, splitting students by math ability showed no interaction with hint type. [Razzaq2010]
- **Caveats:** The large effect sizes (0.70, 0.75) come from pools dominated by small studies and are moderated by publication type. The randomized field trials give 0.18 to 0.35. Comparisons are often bundled (adaptive software against ordinary teaching).
- **Grade:** A.
- **Sources:** [Aleven2016adapt] [Pashler2008] [Tetzlaff2025] [Wang2024] [WangJ2023] [Major2021] [Frontiers2026] [Doroudi2019]

### 11. The evidence lined up against the six things the report writer asked for

This section only regroups findings from sections 1 to 10. Every claim in it comes from those sections, and the choices stay with the report writer.

#### 11.1 A stuck detector: thresholds the evidence offers

| Signal | Threshold in the source | What it predicts | Quality | Source |
| --- | --- | --- | --- | --- |
| Practice opportunities on one skill without three right in a row | 10 (ASSISTments), 15 (Cognitive Tutor) | Probability of mastering later by more practice: 0.089 | Observational, school math | [Gong2014] [Beck2013] |
| Percent correct so far on the skill | Usable from the 4th attempt; most models under 20 percent precision and recall before that | Wheel-spinning with 93.5 percent precision and 77.1 percent recall at step 4 (one definition); 77.2 and 63.5 percent with Random Forest under the other | Observational, three data sets | [Zhang2019] |
| Recent error rate | Under 30 percent correct in the last 10 attempts | Flag for teacher attention; the full flag set raised learning (r = 0.21) | Cluster-randomized classroom experiment | [Holstein2018] |
| Errors after all hints are used | Any error on a step after its last hint | Same flag set | Same | [Holstein2018] |
| Idle time | 2 minutes or more | Possibly off-task | Same | [Holstein2018] |
| Hint pattern | Zero hints on at least one problem plus more than one bottom-out hint in the last 8 problems | 89 percent of such cases were wheel-spinning | Observational, AUC 0.636 to 0.684 overall | [Kai2018] |
| Hint speed | Under 1 second per hint level before the last | Help abuse (clicking through) | Observational | [Aleven2016help] |
| Stuck turns in a chat (wrong or vague answer, hedged answer, "idk", question with no proposed answer) | Recovery 39.7 percent at turn 1, 32.6 at turn 2, 26.2 at turn 3, 18.8 at turn 6 or more; odds fall 13 percent per turn | Next-turn correct answer | Observational, one LLM tutor, preprint | [Ahtisham2026] |
| Same tutor move repeated after it failed | Scripted question twice in a row: 28.1 percent recovery against 55.5 for the first | Next-turn correct answer | Same | [Ahtisham2026] |
| Dialogue cues of confusion | Slow, short, low-quality answers early in a topic, "I don't know"-type replies | Confusion against neutral at 68 percent accuracy | Lab, AutoTutor | [DMello2008] |
| Hints per problem set against a target | More than H + DH hints = "confused zone"; fewer than H - DH = "bored zone" | Designer-set, untested values | Proposal | [Murray2002] |

- **Share of stuck students caught:** recall of about 53 percent at 71 to 77 percent precision for the first detectors [Gong2014]; 63.5 to 81.4 percent recall at step 4 for the best later models, depending on the definition [Zhang2019].
- **What the evidence leaves open:** every attempt-based threshold comes from systems with many short steps per skill. A stuck rule for a lesson with one or two checks per milestone, and "two failures in a row" as a trigger, are both untested in what I found. The nearest evidence is the turn-depth curve from the LLM tutor study.

#### 11.2 Help order and fading: what the evidence offers

- **Levels of help from least to most direct:** general encouragement, specific verbal information, indicating what to do next, preparing the material, demonstration [Wood1978]. Experimental support for moving along these levels contingently is weak after the 2025 replication [Smit2025].
- **What expert tutors do:** ask first, hint in place of telling, give five or six hints in succession, choose the next problem to debug the error they saw [Lepper2002].
- **What the LLM tutor data show:** questions go with the highest recovery at the first stuck turn (+15 to +20 points) and lose it by the fourth (+3 points). Addressing the student's specific error gains value with depth (+5 to +12 points). Partial answers (AOR 0.81) and direct answers (-2 to -3 points) went with slightly lower next-turn recovery. Checks for understanding went with the lowest [Ahtisham2026].
- **What tutoring-system research shows:** hints help the next attempt mainly at medium skill; at low skill, clicking through hints goes with poorer learning [Roll2014]. Systems need a bottom-out hint so a student can get unstuck, and it works when the student studies it like a worked example [Aleven2016help]. Hints on request beat hints pushed after errors in one small experiment [Razzaq2010]. An LLM that hands out answers lowered later unassisted exam scores by 17 percent, and a hint-only version did the same as no AI at all [Bastani2025].
- **Worked examples as help:** g = 0.48 for novices [Barbieri2023], reversing for knowledgeable learners (d = -0.428 for high assistance) [Tetzlaff2025].
- **When to fade:** worked steps shown until estimated mastery reaches 0.7 and restored under 0.5 beat a fixed fading schedule on delayed tests [Salden2010]. Fade the last step first [Atkinson2003] [Renkl2003]. Across 333 outcomes, scaffolding with and without fading did equally well [Belland2017].
- **After help:** ASSISTments counts mastery as three right in a row without help [Pardos2011]. Hint effects seen inside the tutor were too small to show up on a posttest [Aleven2016help].

#### 11.3 Choosing among reteach, step back and misconception feedback: what the evidence offers

- **Reteach the correct method:** did as well as error-specific remediation for algebra procedures (t(36) = .09) [Sleeman1989].
- **Error-specific messages:** changed nothing in two randomized trials on one-step equations [Selent2015]; raised a mid-unit test and programming success in an introductory programming course, with the final concept test equal [Gusukuma2018]. Error-directed feedback went with better recovery at deeper impasses in LLM tutor logs [Ahtisham2026].
- **Why error-specific help underperforms for procedures:** many errors cannot be diagnosed (30 percent could in one early system) and students shift between bugs [Sleeman1989] [VanLehn1983].
- **Wrong beliefs (concepts):** refutation text g = 0.41 [Schroeder2022]. Conceptual change needs dissatisfaction with the old idea plus a new idea that is intelligible, plausible and fruitful [Posner1982].
- **A changed explanation:** prescribed by mastery learning [Guskey2007], supported indirectly by adapted remedial explanations with revision prompts (d = 0.57) [Roelle2015] and by the cost of repeated tutor moves [Ahtisham2026]. A direct test of new angle against repeat is missing.
- **Step back to a prerequisite:** used in deployed adaptive systems with prerequisite graphs [Frontiers2026] [Clement2015] and implied by wheel-spinning (more practice at the same level almost never works: 0.089) [Gong2014]. A controlled test against reteaching is missing.
- **After the remedy:** a second, parallel check on the same content [Guskey2007].

#### 11.4 Feedback by question type: what the evidence offers

| Question type | Evidence | Source |
| --- | --- | --- |
| Multiple choice, select all (marked by the app) | Right-or-wrong alone: 0.05. Correct answer: 0.32. Explanation: 0.49. Answer-until-correct and showing the answer gave the same later recall (.71 and .71). Retry with a hint after the first wrong answer beat retry ending in the answer, which beat the answer at once. Feedback after a right answer raised retention (.93 against .79), mostly for unsure answers. | [VanderKleij2015] [Butler2007] [Attali2015] [Butler2008] |
| Short answer, short essay, one-line code (marked by the tutor) | Open-ended items gave larger gains from feedback and retry than multiple choice. Higher-order outcomes: 0.59 overall, with right-or-wrong alone at -0.01. High-information feedback (what was wrong, why, and how to do better): d = 0.99 against 0.46 for corrective feedback. Misconception-driven feedback on code raised programming success (41 against 32 percent). | [Attali2015] [Attali2016] [VanderKleij2015] [Wisniewski2020] [Gusukuma2018] |
| Any type, timing | Immediate 0.46 against delayed 0.22 in computer-based learning. Delayed feedback better at one week in a lab study where every learner had to view it (.70 against .60). | [VanderKleij2015] [Butler2007] |
| Any type, tone | With praise 0.09, without 0.34. Discouraging feedback -0.14. Feedback on progress since earlier attempts 0.55 against 0.28. More than a third of feedback interventions lowered performance. | [Hattie2007] [Kluger1996] |

#### 11.5 Test-out and a fast path: what the evidence offers

- **Measuring level quickly:** first-step probes, r up to .92 with full tests, 2.5 to 4.9 times faster [KalyugaSweller2004] [Kalyuga2005]. Process measures from the first interactions (time per operation, errors, help requests) explained 63 percent of pretest variance in one tutor [Wood1999]. Low-knowledge learners misjudge their own need for help [Aleven2003] [Roll2014].
- **Skipping on a pretest:** large time savings; equal outcomes in most modules and lower in two practice-oriented modules of seven [ITSE2026]. Adults at work: 45 minutes saved of about 180, with more learning gain and more people never starting [iJAC2024]. Vendor rule: pass a 10-question pretest at a set threshold to skip the lesson [Edgenuity].
- **How much can go:** 40 to 50 percent of a curriculum for students who showed mastery, with test scores unchanged [Reis1998]. 12 percent of tutor time cut with equal two-week retention [Cen2007]. Nine practice problems in place of three bought nothing at 1 and 4 weeks [Rohrer2006].
- **Why a fast path matters beyond time:** high assistance lowers learning for knowledgeable learners (d = -0.428) [Tetzlaff2025]. Dialogue adds nothing over well-pitched text for learners at the right level [VanLehn2007].
- **Side effect of a pretest:** it teaches the items it asks about (g = 0.54) and leaves the rest alone (g = 0.04) [StHilaire2024].
- **What the evidence leaves open:** retention months after skipping, and test-out rules for judgment or conceptual content.

#### 11.6 Target success band and next-task rule: what the evidence offers

- **Band:** about 95 percent success gave the weakest learning; 50, 65 and 80 percent learned alike; learners most often rated practice "appropriate" near 65 to 70 percent success; easy practice kept more new users through the first 10 questions (89.2 against 81.2 percent) [Papousek2016]. Easier levels held game players longer and taught more slowly [Lomas2013]. Rules of thumb: about 80 percent [Rosenshine2012], about 85 percent with 75 to 85 depending on assumptions [Wilson2019]. A deployed "doing well" flag: over 80 percent correct in the last 10 attempts [Holstein2018].
- **Who sets difficulty:** moderately hard is most motivating when the learner picks it, easiest when it is assigned [Lomas2017].
- **Next-task rules with evidence:** keep selecting problems on a skill until estimated knowledge reaches 0.95 (+0.89 SD at 40 percent more problems) [Corbett2001] [Pardos2011]; pick the activity with the most recent learning progress inside a prerequisite graph [Clement2015]; schedule review of facts for when predicted recall falls to about 90 percent [FSRSwiki] [Lindsey2014]; interleave task types that are easy to confuse (g = 0.42 overall, 0.34 in mathematics) [Brunmair2019]; move from worked example to faded example to problem as mastery grows [Salden2010] [KalyugaSweller2005].
- **Rules without support:** sequencing learned purely from data for interdependent content [Doroudi2019]; learner-chosen sequence (g = 0.05) [Karich2014].

### 12. How the evidence bears on rules H6 to H14, H16 and H18

- **H6 (help rises with each failure):** The contingent-shift idea comes from [Wood1976] and [Wood1978]. The 1978 experiment failed a preregistered replication with toddlers [Smit2025]. Support from other directions: expert tutors escalate through several hints [Lepper2002], and in LLM tutor logs more specific, error-directed help gains value as an impasse deepens [Ahtisham2026]. On fading, the open part of H6: mastery-based fading with thresholds 0.7 and 0.5 beat a fixed schedule [Salden2010], while a meta-analysis found scaffolds work equally with and without fading [Belland2017].
- **H7 (the help ladder):** This exact order is untested in the sources. Pieces with evidence: starting with a question pays most at the first stuck turn [Ahtisham2026]; a diagnostic question followed by a problem chosen to debug the error is what expert tutors do [Lepper2002]; a partly worked step matches faded worked examples [Renkl2003] [Atkinson2003]. Against the ladder's reluctance to tell: tutor explanations can be replaced by prompts without loss [Chi2001], and novices learn more from worked examples than from solving (g = 0.48) [Barbieri2023]. On when struggle stops paying: recovery in chat falls to about one in four by the third stuck turn [Ahtisham2026], and productive failure pays for concepts and transfer, with zero effect on procedures [Sinha2021].
- **H8 (missed quiz question: new-angle explanation, one more try, then the student chooses):** Explanation after a miss is supported (0.49 against 0.32 and 0.05) [VanderKleij2015], as is a retry with a hint [Attali2015]. A "new angle" is prescribed [Guskey2007] and untested head to head. Letting the student decide to move on sits in the learner-control literature, where average effects are near zero [Karich2014].
- **H9 (two failures in a row send the lesson back one milestone):** This threshold and this destination are untested in the sources. Published stuck thresholds are 10 opportunities in step-based tutors [Beck2013], detection from the 4th attempt [Zhang2019], and falling recovery by the 2nd and 3rd stuck chat turn [Ahtisham2026]. Routing to a prerequisite is deployed practice [Frontiers2026].
- **H10 (two or three listed misconceptions per milestone):** Lists of common mistakes were part of the prompt of the LLM tutor that avoided harm in a large field trial [Bastani2025]. Error-specific feedback has a mixed record: null for simple equations [Selent2015] [Sleeman1989], positive in programming mid-course [Gusukuma2018], positive for conceptual misbeliefs as refutation text [Schroeder2022]. Errors are unstable and often undiagnosable [VanLehn1983] [Sleeman1989], and Shute's guidelines say to "minimize use of extensive error analyses and diagnosis" [Shute2008].
- **H12 (stated prior knowledge shortens the chain):** Measured knowledge works: first-step probes [KalyugaSweller2004], pretest-gated paths [ITSE2026] [iJAC2024]. Self-report is less sure for low-knowledge learners: Roll et al. suggest they overestimate their ability, and the 2003 review finds they judge their need for help poorly [Roll2014] [Aleven2003].
- **H13 (a right answer moves the student on at once):** Extra same-session practice after success adds nothing lasting [Rohrer2006] [Cen2007]. The opposite risk, moving on from a single lucky answer, belongs to the mastery-criteria stream (Q2); here the relevant fact is that detectors need about four observations before they say anything reliable [Zhang2019].
- **H14 (transfer milestones):** For learners who are ahead, less guidance is better [Tetzlaff2025]; mastery learning gives them enrichment [Guskey2007]; compacting replaces mastered content with "more appropriate learning activities" [Reis1998]; attempting a hard problem before instruction benefits conceptual understanding and transfer [Sinha2021].
- **H16 (a right answer after a hint stays out of the mastery count):** Consistent with deployed practice (three in a row without help) [Pardos2011] and with the finding that hint effects are local and small [Aleven2016help]. Time spent studying a bottom-out hint does correlate with learning [Aleven2016help].
- **H18 (name the specific move in place of praise; the app's green strip praises):** Naming the move is task- and process-level feedback, the kinds with the largest effects [Hattie2007] [Wisniewski2020]. Praise adds little to performance (0.09 against 0.34) [Hattie2007], expert tutors use little of it [Lepper2002], and Shute advises "sparingly, if at all" [Shute2008]. Two points on the other side: learners must know clearly when they are right [Lepper2002], and confirming a right answer improves retention of unsure answers [Butler2008]. The harm from ability praise is much smaller than first reported [Li2019]. For adults, one study favors process praise over person praise [Haimovitz2011]. A comparison of a visual success signal with verbal praise is still to be found.

## Disagreements and open questions

### Disagreements in the sources

1. **How common is wheel-spinning?** Reports of the same original work differ: about 38 percent of students not mastering after 10 opportunities (Kai et al. citing Beck and Gong) [Kai2018]; 45 percent not mastered after 10 problems, with a modeled rate of 18 percent in ASSISTments and 25 percent in the Cognitive Algebra Tutor [Gong2014]; 16 and 6 percent (Zhang et al. citing Gong and Beck 2015) [Zhang2019]; 28 percent of time (a secondary citation) [Sharada2017]. New data give 0.56 to 24.2 percent depending on data set and definition [Zhang2019], and 9.1 percent when a retention test defines success [Kai2018]. The two leading definitions agree on under half of flagged cases.
2. **Is persistence past the threshold bad?** Beck and Gong: the student "will probably never master the skill" [Beck2013]. Kai et al.: most students who persist past 10 problems go on to master and retain, so Beck and Gong's definition "contains a great deal of productive persistence. Not all students who struggle are spinning their wheels." [Kai2018]. Fancsali et al.: the construct is ill-defined [Fancsali2020].
3. **Contingent tutoring.** The founding experiment [Wood1978] against its failed replication [Smit2025].
4. **Is fading necessary?** Scaffolding theory says yes [vandePol2010]. A meta-analysis of 333 outcomes finds equal effects with and without it [Belland2017]. Adaptive fading beat fixed fading in two small studies [Salden2010].
5. **Self-explanation prompts.** g = .55 in general [Bisra2018]; a negative moderator of worked examples in mathematics [Barbieri2023]; less effective when on-demand help is also available [Aleven2016help].
6. **Erroneous examples.** Better delayed retention and conceptual understanding in single studies [Adams2014] [Booth2013]; smaller effects than correct examples alone in the meta-analysis [Barbieri2023].
7. **Seeking help or avoiding it.** Help avoidance went with better next-step performance at low skill [Roll2014] and with worse transfer in another analysis; the authors write "we do not quite know how to reconcile these findings" [Aleven2016help]. Help seeking went with better outcomes for low-knowledge students in an older contingent tutor [Wood1999].
8. **Tell or ask.** Asking: expert tutors [Lepper2002], equal learning when explanations are suppressed [Chi2001], answer-giving LLMs harm later performance [Bastani2025]. Telling: worked examples for novices [Barbieri2023] [Sweller1985], instruction first for procedures and young learners [Sinha2021], questions losing their value as an impasse deepens [Ahtisham2026]. The sources reconcile these through prior knowledge: guidance for novices, less for the knowledgeable [Tetzlaff2025] [Kalyuga2007].
9. **Finer interaction.** The intuition that more interactive tutoring is always better against the interaction plateau [VanLehn2011] and equal results for dialogue and well-pitched text [VanLehn2007].
10. **Diagnose the bug or just reteach.** Null results for procedures [Sleeman1989] [Selent2015] against positive results in programming [Gusukuma2018] and for conceptual misbeliefs [Schroeder2022].
11. **Feedback timing.** Immediate better in applied and computer settings [Kulik1988] [VanderKleij2015]; delayed better in lab studies of retention [Butler2007]. Hattie and Timperley list both "immediate versus delayed" (0.24) and "delayed versus immediate" (0.34) as positive [Hattie2007].
12. **How big is the feedback effect?** 0.79 (average of meta-analyses) [Hattie2007], 0.48 to 0.55 [Wisniewski2020], 0.41 [Kluger1996]. Kluger and DeNisi's counts differ between their abstract (607 effects, over a third negative) and Hattie and Timperley's table (470 effects, 32 percent negative).
13. **Praise.** A large harm from ability praise in children [Mueller1998] against a small or absent one in replication [Li2019]. Praise as weak feedback for performance [Kluger1996] against reviews finding that praise raises interest and persistence in adults under conditions [Henderlong2002].
14. **Target difficulty.** About 80 or 85 percent [Rosenshine2012] [Wilson2019] against equal learning from 50 to 80 percent and worst learning at 95 percent [Papousek2016]. Engagement and learning pull in opposite directions in the short term [Lomas2013] [Papousek2016].
15. **Overlearning.** A moderate benefit in the meta-analysis [Driskell1992] against none at 1 and 4 weeks [Rohrer2006]. The time saved by removing over-practice is 12 percent in the study text and 20 percent in a later summary [Cen2007] [Koedinger2010].
16. **Learner control.** Near zero on average [Karich2014] [Niemiec1996]; positive when combined with an open learner model in one of two experiments [Long2017]; and it changes what difficulty people find motivating [Lomas2017].
17. **Size of the adaptivity effect.** 0.70 to 0.75 in broad meta-analyses [Wang2024] [WangJ2023] against 0.18 to 0.35 in randomized field trials [Major2021] [Frontiers2026].

### Open questions the sources do not answer

- A stuck threshold for lessons with few scored checks per skill, where the "four attempts" and "ten opportunities" rules cannot apply.
- Whether going back to a prerequisite beats reteaching in place, and whether to go back one step or to the true prerequisite.
- Whether a second explanation from a new angle beats repeating the first.
- How much practice a strong learner can skip before retention suffers over months. The longest delays here are 2 to 6 weeks.
- Whether any of the thresholds hold for working adults. The adult evidence is three workplace trials of bundled systems and a handful of undergraduate lab studies.
- Whether praise helps or hurts an adult learner over a course of weeks, and how a visual success signal compares with words.
- How the turn-depth findings from one LLM chemistry tutor transfer to other subjects, prompts and models. The source is an unreplicated preprint with next-turn correctness as its outcome.
- How to combine attempt signals with chat signals in one detector. Each source here uses one kind or the other.

## What the sources themselves recommend

These are the sources' own prescriptions, each attributed. They are recorded as evidence of expert judgment.

### On stuck students

- Gong and Beck: favor precision over recall in a real-time wheel-spinning detector, so that proactive help is rarely sent by mistake, and use the detector to trigger a different teaching strategy. [Gong2014] Beck and Gong also call for changes to tutor design to address students who lack the knowledge to proceed. [Beck2013]
- Zhang et al.: try a Random Forest detector, and use a one-feature model on percent correct as the baseline. They decline to name step 4 as a definitive cut-off. [Zhang2019]
- Fancsali et al.: match the detector's mastery and timeliness rules to how the real system delivers practice. [Fancsali2020]
- Kai et al.: define wheel-spinning with a retention check, since persistence often pays. [Kai2018]
- Ahtisham et al.: "graduated assistance: tutor policies that condition the type and degree of support on how long a student has been stuck". A guardrail such as "do not give the answer" changes what a tutor withholds and does not tell it how to adapt. Use repeated non-progress as an early signal of a stalled dialogue. [Ahtisham2026]
- Holstein et al.: direct human attention in real time to situations the tutoring system is ill-suited to handle. [Holstein2018]
- Kapur: for productive failure, use a problem that is challenging and within reach, admits several solutions, activates prior knowledge, and is followed by instruction that compares the student's solutions with the canonical one. [Kapur2016]
- Baker et al.: put effort into detecting and responding to boredom and confusion, and into interventions that break the "vicious cycles" of persistent boredom. [Baker2010]
- Murray and Arroyo: set a target number of hints per problem set with an allowed deviation, treat more as confusion and fewer as boredom, and also let learners say "confused", "bored", "too difficult" or "too easy". [Murray2002]

### On help

- Koedinger and Aleven: keep immediate yes-or-no feedback on each step. Treat the balance of giving and withholding as a matter for experiments that yield "specific conditions and parameters". [Koedinger2007]
- Aleven et al.: keep principle-based hints on demand, and keep a bottom-out hint so students can get unstuck. A hint should state the action, the conditions under which it applies, the principle that justifies it, and how the principle applies to this step, briefly. Support self-explanation of bottom-out hints. Consider prompted self-explanations with feedback as a supplement to or replacement for hints (they note this is untested). They cite evidence that breaking a problem into steps works better than one long hint. [Aleven2016help]
- Razzaq and Heffernan: provide hints on demand. [Razzaq2010]
- Bastani et al.: build safeguards into an AI tutor: hints without the answer, plus teacher-written solutions and common mistakes in the prompt. [Bastani2025]
- Renkl and Atkinson; Atkinson, Renkl and Merrill: move from a complete example to independent solving by removing one step at a time, last step first, and prompt the learner to name the principle behind each worked step. They call the combination "highly recommendable". [Renkl2003] [Atkinson2003]
- Salden et al.: fade worked steps per skill when the mastery estimate reaches 0.7 and restore them under 0.5. [Salden2010]
- Barbieri et al.: use correct worked examples for both first learning and practice; pairing them with self-explanation prompts "may not be a fruitful design modification". [Barbieri2023]
- Adams et al.: have learners identify, explain and correct errors in worked solutions for more durable learning, and do not judge materials by how much students like them. [Adams2014]
- Tetzlaff et al.: "The lower the domain knowledge of the learner, the more assistance should be provided", and adapt to knowledge gained during the lesson as well as knowledge brought to it. [Tetzlaff2025]
- Kalyuga: replace worked examples with faded examples and then problems as task-specific expertise grows, using rapid diagnostic probes to decide. [Kalyuga2007] [KalyugaSweller2005]
- VanLehn: step-level interaction is enough; finer dialogue has not added learning. [VanLehn2011]
- Lepper and Woolverton: be Socratic (questions and hints), progressive (diagnose, debug, then raise difficulty), indirect in negative feedback, and credible in praise. [Lepper2002]

### On remediation

- Guskey (after Bloom): pair every formative check with correctives matched to the items missed, make the correctives different from the first teaching, do them inside the lesson time, then give a second parallel check. Give enrichment to those who passed. [Guskey2007]
- Selent and Heffernan: "focus less on how or why a student arrived at a specific wrong answer" and look to other ways of tutoring. [Selent2015]
- Sleeman et al.: where students learned a procedure by rote, reteaching it is as good as error-specific remediation. [Sleeman1989]
- Tippett: identify the specific misconception, use refutation text in expository form, warn readers that their prior idea may be wrong, and combine the text with other activities. [Tippett2010]
- Schroeder and Kucera: use refutation text; it is cheap and robust across contexts. [Schroeder2022]
- Roelle et al.: when a learner fails an inference, give a remedial explanation adapted to that failure together with a prompt to revise the earlier answer. [Roelle2015]

### On feedback

- Kluger and DeNisi: keep feedback on the task and away from the self. [Kluger1996]
- Hattie and Timperley: give feedback at the task, process and self-regulation levels; self-level praise rarely helps. [Hattie2007]
- Shute: focus on the task, give elaborated feedback in manageable units, be specific and clear, keep it "as simple as possible but no simpler", give feedback after the learner has attempted a solution, avoid comparisons with other learners, be cautious with overall grades, "use praise sparingly, if at all", do not interrupt an engaged learner, avoid "progressive hints that always terminate with the correct answer", and "minimize use of extensive error analyses and diagnosis". Use immediate feedback for difficult tasks and for low-achieving learners, and consider delayed feedback for simple tasks, for transfer, and for high-achieving learners. [Shute2008]
- Wisniewski et al.: treat different forms of feedback as different treatments; feedback that explains what was wrong, why, and how to improve has the largest effect. [Wisniewski2020]
- Van der Kleij et al.: prefer elaborated feedback, above all for higher-order outcomes. [VanderKleij2015]
- Butler et al.: feedback should follow correct answers as well as wrong ones, since it rescues unsure correct answers; in their experiments delayed feedback gave better long-term retention than immediate feedback. [Butler2007] [Butler2008]
- Attali: reports that a retry with a hint after a first wrong answer, and the open-ended format, gave the largest gains, and discusses what that means for assessments built for learning. [Attali2015]
- Henderlong and Lepper: praise that is sincere, tied to controllable causes, free of social comparison, and attainable in its standards. [Henderlong2002]

### On acceleration, difficulty and sequence

- St. Hilaire et al.; Pan and Carpenter: if prequestions are used, have learners attempt an answer, and expect the benefit to stay with the questions asked. [StHilaire2024] [Pan2023]
- Rohrer and Taylor: spread practice across sessions in place of piling it into one; they "cast doubt on the utility of mathematics assignments that include many problems of the same type". [Rohrer2006]
- Cen et al.: use learning-curve analysis to find over-practiced and under-practiced skills and re-set the mastery parameters. [Cen2007]
- The ECDL study authors: calibrate pretest-based paths to the kind of course, and add support for low performers and for practice-oriented modules. [ITSE2026]
- Rosenshine: aim for about 80 percent success by teaching in small steps with practice after each, and reteach when students struggle so that they do not practice errors. [Rosenshine2012]
- Wilson et al.: for the class of learners they model, hold training accuracy near 85 percent by adjusting difficulty as skill grows. [Wilson2019]
- Papousek et al.: for their kind of system, start with easy questions and then move to harder ones; evaluate with learning and long-term use together, since short-term engagement misleads. [Papousek2016]
- Lomas et al.: make games "not too hard, not too boring": easy, with moderate novelty and suspense; tell players in advance how hard a level is to encourage challenge-seeking. [Lomas2013] [Lomas2017]
- Aleven et al.: adapt first to knowledge, in task selection and in redesign from data. [Aleven2016adapt]
- Doroudi et al.: combine theory-driven and data-driven models for sequencing, and test a policy against several student models before putting it in front of students. [Doroudi2019]
- Brunmair and Richter: use interleaving for material with confusable categories and with caution for expository texts and words. [Brunmair2019]
- FSRS project: set one number, the desired retention (default 90 percent, reasonable range 70 to 97), and let the scheduler do the rest. [FSRSwiki]
- Karich et al.: "there does not seem to be an advantage to giving the learner control over any particular instructional component". [Karich2014]
- Long and Aleven: show learners the tutor's estimate of their skills and share control over problem selection. [Long2017]
- Pashler et al.: do not match instruction to learning styles; attend to prior knowledge. [Pashler2008]

## Source list

Every URL below is one I fetched or saw in a search result. "Full text (key sections)" means the whole text was available and I read the passages that hold the numbers quoted. Where a fetched page did not show the author list, the entry says so.

- [Adams2014] Adams, D. M., McLaren, B. M., Durkin, K., Mayer, R. E., Rittle-Johnson, B., Isotani, S., and van Velsen, M. (2014). Using erroneous examples to improve mathematics learning with a web-based tutoring system. Computers in Human Behavior, 36, 401-411. https://doi.org/10.1016/j.chb.2014.03.053. Open access: http://www.cs.cmu.edu/~bmclaren/pubs/AdamsEtAl-UsingErrExToImproveMathLearningWithWebTutoringSys-CHB2014.pdf. Read: full text (key sections).
- [Ahtisham2026] Ahtisham, B., Vanacore, K., Napoli, A., Arens, J., Ionova, K., Cohn, C., Salehi, S., and Kizilcec, R. (2026). Examining Variation in How Guided AI Tutors Resolve Student Impasses. arXiv preprint 2609.38346. https://arxiv.org/abs/2609.38346. Read: full text.
- [Aleven2003] Aleven, V., Stahl, E., Schworm, S., Fischer, F., and Wallace, R. (2003). Help Seeking and Help Design in Interactive Learning Environments. Review of Educational Research, 73(3), 277-320. https://journals.sagepub.com/doi/10.3102/00346543073003277. Open access: https://www.academia.edu/61770205/Help_Seeking_and_Help_Design_in_Interactive_Learning_Environments. Read: full text (key sections).
- [Aleven2016adapt] Aleven, V., McLaughlin, E. A., Glenn, R. A., and Koedinger, K. R. (2016). Instruction Based on Adaptive Learning Technologies. In Handbook of Research on Learning and Instruction, 2nd edition, Routledge. https://www.taylorfrancis.com/chapters/edit/10.4324/9781315736419-26/instruction-based-adaptive-learning-technologies-vincent-aleven-elizabeth-mclaughlin-amos-glenn-kenneth-koedinger. Open access (prepublication, archived copy): http://web.archive.org/web/20221020044037/https://www.cs.cmu.edu/afs/.cs.cmu.edu/Web/People/aleven/Papers/2016/Aleven_etal_Handbook2016_AdaptiveLearningTechnologies_Prepub.pdf. Read: full text (key sections). The original cs.cmu.edu address of this copy returned 404 on 2026-11-22, so the link points to the Internet Archive snapshot.
- [Aleven2016help] Aleven, V., Roll, I., McLaren, B. M., and Koedinger, K. R. (2016). Help Helps, But Only So Much: Research on Help Seeking with Intelligent Tutoring Systems. International Journal of Artificial Intelligence in Education, 26(1), 205-223. https://doi.org/10.1007/s40593-015-0089-1. Open access: http://www.cs.cmu.edu/~bmclaren/pubs/AlevenEtAl-HelpHelpsButOnlySoMuch-IJAIED2016.pdf. Read: full text.
- [Atkinson2003] Atkinson, R. K., Renkl, A., and Merrill, M. M. (2003). Transitioning From Studying Examples to Solving Problems: Effects of Self-Explanation Prompts and Fading Worked-Out Steps. Journal of Educational Psychology, 95(4), 774-783. https://doi.org/10.1037/0022-0663.95.4.774. Open access: https://mrbartonmaths.com/resourcesnew/8.%20Research/Making%20the%20most%20of%20examples/Fading%20out%20and%20Prompts.pdf. Read: full text (key sections).
- [Attali2015] Attali, Y. (2015). Effects of multiple-try feedback and question type during mathematics problem solving on performance in similar problems. Computers & Education, 86, 260-267. https://doi.org/10.1016/j.compedu.2015.08.011. Abstract page: https://dl.acm.org/doi/10.1016/j.compedu.2015.08.011. Read: abstract only.
- [Attali2016] Attali, Y., Laitusis, C., and Stone, E. (2016). Differences in Reaction to Immediate Feedback and Opportunity to Revise Answers for Multiple-Choice and Open-Ended Questions. Educational and Psychological Measurement. https://journals.sagepub.com/doi/10.1177/0013164415612548. Read: abstract only.
- [Baker2010] Baker, R. S. J. d., D'Mello, S. K., Rodrigo, M. M. T., and Graesser, A. C. (2010). Better to be frustrated than bored: The incidence, persistence, and impact of learners' cognitive-affective states during interactions with three different computer-based learning environments. International Journal of Human-Computer Studies, 68(4), 223-241. https://doi.org/10.1016/j.ijhcs.2009.12.003. Open access: https://learninganalytics.upenn.edu/ryanbaker/BDRG-IJHCS-Final.pdf. Read: full text (key sections).
- [Barbieri2023] Barbieri, C. A., Miller-Cotto, D., Clerjuste, S. N., and Chawla, K. (2023). A Meta-analysis of the Worked Examples Effect on Mathematics Performance. Educational Psychology Review, 35, article 11. https://doi.org/10.1007/s10648-023-09745-1. Open access: https://danamillercotto.com/uploads/4/7/7/2/47725475/barbieri_et_al__2023__we_meta-analysis.pdf. Read: full text (key sections).
- [Bastani2025] Bastani, H., Bastani, O., Sungu, A., Ge, H., Kabakci, O., and Mariman, R. (2025). Generative AI without guardrails can harm learning: Evidence from high school mathematics. PNAS, 122(26), e2422633122. https://www.pnas.org/doi/10.1073/pnas.2422633122. Open access: https://pmc.ncbi.nlm.nih.gov/articles/PMC12232635/. Read: full text (key sections). A correction published on 2025-08-20 (https://doi.org/10.1073/pnas.2518204122) fixes one author's affiliation and leaves the results as they were.
- [Beck2013] Beck, J. E., and Gong, Y. (2013). Wheel-Spinning: Students Who Fail to Master a Skill. AIED 2013, Lecture Notes in Computer Science 7926, 431-440. https://doi.org/10.1007/978-3-642-39112-5_44. Abstract page: https://www.researchgate.net/publication/290777423_Wheel-Spinning_Students_Who_Fail_to_Master_a_Skill. Read: abstract only. The numbers come from [Gong2014].
- [Belland2017] Belland, B. R., Walker, A. E., Kim, N. J., and Lefler, M. (2017). Synthesizing Results From Empirical Research on Computer-Based Scaffolding in STEM Education: A Meta-Analysis. Review of Educational Research, 87(2), 309-344. https://doi.org/10.3102/0034654316670999. Open access: https://pmc.ncbi.nlm.nih.gov/articles/PMC5347356/. Read: full text.
- [Bisra2018] Bisra, K., Liu, Q., Nesbit, J. C., Salimi, F., and Winne, P. H. (2018). Inducing Self-Explanation: a Meta-Analysis. Educational Psychology Review, 30(3), 703-725. https://doi.org/10.1007/s10648-018-9434-x. Open access: https://gwern.net/doc/psychology/spaced-repetition/2018-bisra.pdf. Read: full text (key sections).
- [Booth2013] Booth, J. L., Lange, K. E., Koedinger, K. R., and Newton, K. J. (2013). Using example problems to improve student learning in algebra: Differentiating between correct and incorrect examples. Learning and Instruction, 25, 24-34. https://doi.org/10.1016/j.learninstruc.2012.11.002. Open access: https://files.eric.ed.gov/fulltext/ED543090.pdf. Read: full text (key sections).
- [Brunmair2019] Brunmair, M., and Richter, T. (2019). Similarity matters: A meta-analysis of interleaved learning and its moderators. Psychological Bulletin, 145(11), 1029-1052. https://doi.org/10.1037/bul0000209. Open access (accepted manuscript): https://www.uni-wuerzburg.de/fileadmin/06020400/2019/Brunmair_Richter_in_press__2019_META-ANALYSIS_OF_INTERLEAVED_LEARNING.pdf. Read: abstract and the overall-effect results.
- [Bull2016] Bull, S. (2016). Negotiated learner modelling to maintain today's learner models. Research and Practice in Technology Enhanced Learning, 11, article 10. https://link.springer.com/article/10.1186/s41039-016-0035-3. Read: search excerpt only. The author, first given from memory, is confirmed by Crossref (citation check, 2026-11-22).
- [Butler2007] Butler, A. C., Karpicke, J. D., and Roediger, H. L. (2007). The effect of type and timing of feedback on learning from multiple-choice tests. Journal of Experimental Psychology: Applied, 13(4), 273-281. https://doi.org/10.1037/1076-898x.13.4.273. Open access: https://learninglab.psych.purdue.edu/downloads/2007/2007_Butler_Karpicke_Roediger_JEPA.pdf. Read: full text (key sections).
- [Butler2008] Butler, A. C., Karpicke, J. D., and Roediger, H. L. (2008). Correcting a metacognitive error: Feedback increases retention of low-confidence correct responses. Journal of Experimental Psychology: Learning, Memory, and Cognition, 34(4), 918-928. https://doi.org/10.1037/0278-7393.34.4.918. Open access: https://learninglab.psych.purdue.edu/downloads/2008/2008_Butler_Karpicke_Roediger_JEPLMC.pdf. Read: full text (key sections).
- [Cen2007] Cen, H., Koedinger, K. R., and Junker, B. (2007). Is Over Practice Necessary? Improving Learning Efficiency with the Cognitive Tutor through Educational Data Mining. AIED 2007, Frontiers in Artificial Intelligence and Applications 158, 511-518. https://ebooks.iospress.nl/volumearticle/3568. Open access (authors' PDF): https://pact.cs.cmu.edu/koedinger/pubs/Cen%20&%20Koedinger%20AIED07.pdf. Read: full text. The authors' PDF is this same paper: its first page carries the published title and an abstract identical to the one on the IOS Press page. "Do Less and Learn More - A First Study on Optimizing Tutor Curriculum using Learning Factors Analysis" is only the title stored in the PDF's document properties.
- [Chi2001] Chi, M. T. H., Siler, S. A., Jeong, H., Yamauchi, T., and Hausmann, R. G. (2001). Learning from human tutoring. Cognitive Science, 25(4), 471-533. https://onlinelibrary.wiley.com/doi/10.1207/s15516709cog2504_1. Open access: https://education.asu.edu/sites/g/files/litvpz656/files/lcl/image3_2.pdf. Read: abstract and regression results in the full text; session counts and test scores from the secondary summary at https://research.elamur.ai/free/en/papers/chi-2001-learning-human-tutoring/.
- [Clement2015] Clement, B., Roy, D., Oudeyer, P.-Y., and Lopes, M. (2015). Multi-Armed Bandits for Intelligent Tutoring Systems. Journal of Educational Data Mining, 7(2), 20-48. https://jedm.educationaldatamining.org/index.php/JEDM/article/view/JEDM111. Open access: https://arxiv.org/pdf/1310.3174v2.pdf. Read: full text (method sections). Outcome numbers unconfirmed.
- [Corbett2001] Corbett, A. T. (2001). Cognitive Computer Tutors: Solving the Two-Sigma Problem. User Modeling 2001, Lecture Notes in Computer Science 2109, 137-147. https://doi.org/10.1007/3-540-44566-8_14. ACM record: https://dl.acm.org/doi/10.5555/647664.733401. Read: abstract only; effect sizes from the secondary summary at https://digitalcommons.wpi.edu/iqp-all/1197.
- [DMello2008] D'Mello, S. K., Craig, S. D., Witherspoon, A., McDaniel, B., and Graesser, A. (2008). Automatic detection of learner's affect from conversational cues. User Modeling and User-Adapted Interaction, 18(1-2), 45-80. https://doi.org/10.1007/s11257-007-9037-6. Open access: https://www.academia.edu/966135/Automatic_detection_of_learners_affect_from_conversational_cues. Abstract with accuracies: https://asu.elsevierpure.com/en/publications/automatic-detection-of-learners-affect-from-conversational-cues/. Read: abstract and results section.
- [DMello2012] D'Mello, S., and Graesser, A. (2012). Dynamics of affective states during complex learning. Learning and Instruction, 22(2), 145-157. https://doi.org/10.1016/j.learninstruc.2011.10.001. Abstract page: https://eric.ed.gov/?id=EJ950444. Read: abstract only.
- [DMello2014] D'Mello, S., Lehman, B., Pekrun, R., and Graesser, A. (2014). Confusion can be beneficial for learning. Learning and Instruction, 29, 153-170. https://doi.org/10.1016/j.learninstruc.2012.05.003. Publisher page: https://www.sciencedirect.com/science/article/pii/S0959475212000357. Open access (preprint): https://www.researchgate.net/publication/234102071_Confusion_can_be_beneficial_for_learning. Read: full text (key sections).
- [Doroudi2019] Doroudi, S., Aleven, V., and Brunskill, E. (2019). Where's the Reward? A Review of Reinforcement Learning for Instructional Sequencing. International Journal of Artificial Intelligence in Education, 29(4), 568-620. https://eric.ed.gov/?id=EJ1235264. Open access (author version): https://cpb-us-e2.wpmucdn.com/faculty.sites.uci.edu/dist/f/847/files/2019/11/IJAIED-RL-Review-Author-Version.pdf. Read: full text (key sections).
- [Driskell1992] Driskell, J. E., Willis, R. P., and Copper, C. (1992). Effect of overlearning on retention. Journal of Applied Psychology, 77(5), 615-622. https://doi.org/10.1037//0021-9010.77.5.615. Open access: https://mrbartonmaths.com/resourcesnew/8.%20Research/Memory%20and%20Revision/Overlearning%20on%20retention.pdf. Read: abstract and general results.
- [Edgenuity] Imagine Edgenuity. Pretesting overview (vendor help page). https://help.imagineedgenuity.com/hc/en-us/articles/360042275474-Pretesting-overview. Read: page excerpt. Vendor source, used for deployed practice only.
- [Fancsali2020] Fancsali, S. E., Holstein, K., Sandbothe, M., Ritter, S., McLaren, B. M., and Aleven, V. (2020). Towards Practical Detection of Unproductive Struggle. AIED 2020, Lecture Notes in Computer Science 12164, 92-97. https://doi.org/10.1007/978-3-030-52240-7_17. Open access: https://pmc.ncbi.nlm.nih.gov/articles/PMC7334700/. Read: full text (key sections).
- [Fong2015] Fong, C. J., Zaleski, D. J., and Leach, J. K. (2015). The challenge-skill balance and antecedents of flow: A meta-analytic investigation. The Journal of Positive Psychology, 10(5), 425-446. https://doi.org/10.1080/17439760.2014.967799. Open access: https://www.researchgate.net/publication/267450034_The_challenge-skill_balance_and_antecedents_of_flow_A_meta-analytic_investigation. Read: abstract and main results. A secondary summary that discusses measurement overlap: https://pursuit-of-happiness.org/science-of-happiness/finding-flow.
- [Frontiers2026] Tang, C., Cao, Y., Shen, Q., Kang, S., Yang, Y., Chang, P., and Fan, M. (2026). AI-driven adaptive training for utility employees: a three-wave cluster-randomized field experiment on cognitive-load and personalization-fit pathways. Frontiers in Psychology, 17, 1944626. https://doi.org/10.3389/fpsyg.2026.1944626. PDF: https://www.frontiersin.org/journals/psychology/articles/10.3389/fpsyg.2026.1944626/pdf. Read: abstract and method. Authors and volume added from Crossref in the citation check.
- [FSRSbench] open-spaced-repetition project (2023). The Benchmark (fsrs4anki wiki, revision of 7 September 2023). https://github.com/open-spaced-repetition/fsrs4anki/wiki/The-Benchmark/6fb3f813edf7f845272a9aa1d683bb35403345e5. Read: full page. Project documentation.
- [FSRSwiki] open-spaced-repetition project. ABC of FSRS (awesome-fsrs wiki). https://github.com/open-spaced-repetition/awesome-fsrs/wiki/ABC-of-FSRS. Read: page excerpt. Project documentation.
- [Gong2014] Gong, Y. (2014). Student Modeling in Intelligent Tutoring Systems. PhD dissertation, Worcester Polytechnic Institute (advisor J. E. Beck). https://digital.wpi.edu/downloads/9306sz45f. Read: full text (the wheel-spinning chapters).
- [Gong2015] Gong, Y., and Beck, J. E. (2015). Towards Detecting Wheel-Spinning: Future Failure in Mastery Learning. Learning @ Scale 2015, 67-74. https://dl.acm.org/doi/10.1145/2724660.2724673. Read: abstract only. The numbers come from [Gong2014].
- [Graesser1995] Graesser, A. C., Person, N. K., and Magliano, J. P. (1995). Collaborative dialogue patterns in naturalistic one-to-one tutoring. Applied Cognitive Psychology, 9(6), 495-522. https://onlinelibrary.wiley.com/doi/10.1002/acp.2350090604. Open access: https://talkbank.org/class/access/0docs/graesser1995.pdf. Read: full text (key sections). The 92 percent figure is in this paper itself (the passage on step 5, assessing the student's understanding) and is repeated in the secondary source http://www.cs.iit.edu/%7Ecircsim/documents/mgmai99.pdf.
- [Guskey2007] Guskey, T. R. (2007). Closing Achievement Gaps: Revisiting Benjamin S. Bloom's "Learning for Mastery". Journal of Advanced Academics, 19(1), 8-31. https://journals.sagepub.com/doi/10.4219/jaa-2007-704. Open access: https://tguskey.com/wp-content/uploads/Mastery-Learning-5-Revisiting-Blooms-Learning-for-Mastery.pdf. Read: full text (key sections).
- [Gusukuma2018] Gusukuma, L., Bart, A. C., Kafura, D., and Ernst, J. (2018). Misconception-Driven Feedback: Results from an Experimental Study. ICER 2018, 160-168. https://doi.org/10.1145/3230977.3231002. Open access: https://acbart.github.io/papers/p160-gusukuma.pdf. Read: full text (key sections). The author list, first given from memory, is confirmed by Crossref and by the PDF (citation check, 2026-11-22).
- [Haimovitz2011] Haimovitz, K., and Corpus, J. H. (2011). Effects of person versus process praise on student motivation: stability and change in emerging adulthood. Educational Psychology, 31(5), 595-609. https://doi.org/10.1080/01443410.2011.585950. Abstract page: https://eric.ed.gov/?id=EJ932918. Read: abstract only. The authors, first given from memory, are confirmed by Crossref and ERIC (citation check, 2026-11-22); both list the second author as Jennifer Henderlong Corpus.
- [Hattie2007] Hattie, J., and Timperley, H. (2007). The Power of Feedback. Review of Educational Research, 77(1), 81-112. https://journals.sagepub.com/doi/10.3102/003465430298487. Open access: https://assess.ucr.edu/sites/default/files/2019-02/hattietimperley_2007.pdf. Read: full text (key sections).
- [Henderlong2002] Henderlong, J., and Lepper, M. R. (2002). The Effects of Praise on Children's Intrinsic Motivation: A Review and Synthesis. Psychological Bulletin, 128(5), 774-795. https://pubmed.ncbi.nlm.nih.gov/12206194/. Open access: https://www.reed.edu/psychology/motivation/assets/downloads/Henderlong_Lepper_2002.pdf. Read: abstract and the passage on adults.
- [Holstein2018] Holstein, K., McLaren, B. M., and Aleven, V. (2018). Student Learning Benefits of a Mixed-Reality Teacher Awareness Tool in AI-Enhanced Classrooms. AIED 2018. https://link.springer.com/chapter/10.1007/978-3-319-93843-1_12. Open access: http://www.cs.cmu.edu/~bmclaren/pubs/HolsteinMcLarenAleven-StudentLearningBenefitsMixedRealityTeacherAwarnessTool-AIED2018.pdf. Read: full text (key sections).
- [iJAC2024] Lynch, D., and DeGenarro, M. (2024). Using a Random Controlled Trial to Explore the Impact of AI-Enabled Learning. International Journal of Advanced Corporate Learning (iJAC), 17(4), 58-68. https://doi.org/10.3991/ijac.v17i4.48321. Read: full text (key sections). Authors, year and pages added from Crossref in the citation check.
- [ITSE2026] Pagano, A., Mehrvarz, M., McLaren, B. M., Marengo, A., and Santamato, V. (2026). Efficiency and effectiveness of adaptive learning paths: evidence from a large-scale study. Interactive Technology and Smart Education, 23(4), 581-608. https://doi.org/10.1108/itse-12-2025-0370. Read: abstract only. Authors, volume and pages added from Crossref in the citation check.
- [Kai2018] Kai, S., Almeda, M. V., Baker, R. S., Heffernan, C., and Heffernan, N. (2018). Decision Tree Modeling of Wheel-Spinning and Productive Persistence in Skill Builders. Journal of Educational Data Mining, 10(1), 36-71. https://jedm.educationaldatamining.org/index.php/JEDM/article/view/210. Open access: https://learninganalytics.upenn.edu/ryanbaker/ShiminJEDM.pdf. Erratum: https://learninganalytics.upenn.edu/ryanbaker/JEDM-Erratum-2018-v1.pdf. Read: full text (key sections) and the erratum.
- [Kalyuga2003] Kalyuga, S., Ayres, P., Chandler, P., and Sweller, J. (2003). The Expertise Reversal Effect. Educational Psychologist, 38(1), 23-31. https://doi.org/10.1207/s15326985ep3801_4. Open access: https://ro.uow.edu.au/cgi/viewcontent.cgi?article=1141&context=edupapers. Read: abstract and opening sections.
- [Kalyuga2005] Kalyuga, S. (2005). When Less is More in Cognitive Diagnosis. Proceedings of the Annual Meeting of the Cognitive Science Society, 27. https://escholarship.org/uc/item/0899206c. PDF: https://escholarship.org/content/qt0899206c/qt0899206c.pdf. Read: full text (key sections). The eScholarship cover page gives the publication date as 2005, so the key changed from Kalyuga2006 to Kalyuga2005 in the citation check.
- [Kalyuga2007] Kalyuga, S. (2007). Expertise Reversal Effect and Its Implications for Learner-Tailored Instruction. Educational Psychology Review, 19(4), 509-539. https://doi.org/10.1007/s10648-007-9054-3. Open access: https://www.uky.edu/~gmswan3/EDC608/Kalyuga2007_Article_ExpertiseReversalEffectAndItsI.pdf. Read: full text (key sections).
- [KalyugaSweller2004] Kalyuga, S., and Sweller, J. (2004). Measuring Knowledge to Optimize Cognitive Load Factors During Instruction. Journal of Educational Psychology, 96(3), 558-568. https://doi.org/10.1037/0022-0663.96.3.558. Read: abstract only.
- [KalyugaSweller2005] Kalyuga, S., and Sweller, J. (2005). Rapid dynamic assessment of expertise to improve the efficiency of adaptive e-learning. Educational Technology Research and Development, 53(3), 83-93. https://doi.org/10.1007/bf02504800. Open access: https://www.academia.edu/1405551/Rapid_dynamic_assessment_of_expertise_to_improve_the_efficiency_of_adaptive_e_learning. Read: full text (method sections). The effect size d = 0.69 was first taken from the summary panel on that page; the citation check found it in the paper's own results text (efficiency gains, t(14) = 1.89, one-tailed p < .05, Cohen's d 0.69).
- [Kapur2008] Kapur, M. (2008). Productive Failure. Cognition and Instruction, 26(3), 379-424. https://www.tandfonline.com/doi/full/10.1080/07370000802212669. Read: abstract only.
- [Kapur2014] Kapur, M. (2014). Productive Failure in Learning Math. Cognitive Science. https://onlinelibrary.wiley.com/doi/full/10.1111/cogs.12107. Read: full text (key sections).
- [Kapur2016] Kapur, M. (2016). Examining Productive Failure, Productive Success, Unproductive Failure, and Unproductive Success in Learning. Educational Psychologist, 51(2), 289-299. https://www.tandfonline.com/doi/full/10.1080/00461520.2016.1155457. Open access: https://www.nomanis.com.au/hubfs/Nomanis_June2023/Pdf/Examining%20Productive%20Failure%20Productive%20Success%20Unproductive%20Failure%20and%20Unproductive%20Success%20in%20Learning.pdf. Read: full text (key sections).
- [Karich2014] Karich, A. C., Burns, M. K., and Maki, K. E. (2014). Updated Meta-Analysis of Learner Control Within Educational Technology. Review of Educational Research, 84(3), 392-410. https://doi.org/10.3102/0034654314526064. Abstract page: https://journals.sagepub.com/doi/10.3102/0034654314526064. Read: abstract only; the page-404 quote comes from the secondary summary at https://www.worklearning.com/2017/01/18/giving-learners-more-control-in-elearning-what-does-the-research-say/.
- [Kerly2008] Kerly, A., and Bull, S. (2008). Children's Interactions with Inspectable and Negotiated Learner Models. ITS 2008, Lecture Notes in Computer Science, 132-141. https://doi.org/10.1007/978-3-540-69132-7_18. Read: abstract only. The earlier link pointed at an internal ACM platform host (acm-prod.literatumonline.com), so it now uses the DOI.
- [Kluger1996] Kluger, A. N., and DeNisi, A. (1996). The Effects of Feedback Interventions on Performance: A Historical Review, a Meta-Analysis, and a Preliminary Feedback Intervention Theory. Psychological Bulletin, 119(2), 254-284. https://doi.org/10.1037/0033-2909.119.2.254. Open access: https://www.mrbartonmaths.com/resourcesnew/8.%20Research/Marking%20and%20Feedback/The%20effects%20of%20feedback%20interventions.pdf. Read: full text (key sections).
- [Koedinger2007] Koedinger, K. R., and Aleven, V. (2007). Exploring the Assistance Dilemma in Experiments with Cognitive Tutors. Educational Psychology Review, 19(3), 239-264. DOI 10.1007/s10648-007-9049-0 (as printed on the fetched PDF). Open access: https://pact.cs.cmu.edu/pubs/Koedinger%20Aleven%2007.pdf. Read: full text (key sections).
- [Koedinger2010] Koedinger, K. R., Baker, R. S. J. d., Cunningham, K., Skogsholm, A., Leber, B., and Stamper, J. (2010). A Data Repository for the EDM Community: The PSLC DataShop. https://pact.cs.cmu.edu/koedinger/pubs/Koedinger,%20Baker,%20Cunningham,%20Skogsholm,%20Leber,%20Stamper%202010.pdf. Read: the passage on the over-practice study.
- [Kulik1988] Kulik, J. A., and Kulik, C.-L. C. (1988). Timing of Feedback and Verbal Learning. Review of Educational Research, 58(1), 79-97. https://journals.sagepub.com/doi/10.3102/00346543058001079. Read: abstract only; the effect size 0.28 from Table 1 of [Hattie2007].

- [Lepper2002] Lepper, M. R., and Woolverton, M. (2002). The wisdom of practice: Lessons learned from the study of highly effective tutors. In J. Aronson (ed.), Improving Academic Achievement: Impact of Psychological Factors on Education, 135-158. https://stafforini.com/works/lepper-2002-wisdom-practice-lessons/. Open access: https://pdf.stafforini.com/lepper-2002-wisdom-practice-lessons.pdf. Read: full text (key sections).
- [Li2019] Li, Y., and Bates, T. C. (2019). You can't change your basic ability, but you work at things, and that's how we get hard things done: Testing the role of growth mindset on response to setbacks, educational attainment, and cognitive ability. Journal of Experimental Psychology: General. https://doi.org/10.1037/xge0000669. Open access: https://www.pure.ed.ac.uk/ws/files/116143044/Dweck_China_JEPG_Rev2_002_.pdf. Read: full text (key sections).
- [Lindsey2014] Lindsey, R. V., Shroyer, J. D., Pashler, H., and Mozer, M. C. (2014). Improving Students' Long-Term Knowledge Retention Through Personalized Review. Psychological Science, 25(3). https://journals.sagepub.com/doi/10.1177/0956797613504302. Open access: http://www.rob-lindsey.com/papers/2014/psychsci.pdf. Read: abstract and method.
- [Lomas2013] Lomas, D., Patel, K., Forlizzi, J. L., and Koedinger, K. R. (2013). Optimizing challenge in an educational game using large-scale design experiments. CHI 2013, 89-98. https://dl.acm.org/doi/10.1145/2470654.2470668. Open access: https://pact.cs.cmu.edu/pubs/Lomas,%20Forlizzi%20&%20Koedinger%202013.pdf. Read: full text (key sections).
- [Lomas2017] Lomas, J. D., Koedinger, K., Patel, N., Shodhan, S., Poonwala, N., and Forlizzi, J. L. (2017). Is Difficulty Overrated? The Effects of Choice, Novelty and Suspense on Intrinsic Motivation in Educational Games. CHI 2017, 1028-1039. https://dl.acm.org/doi/10.1145/3025453.3025638. Open access: http://library.usc.edu.ph/ACM/CHI%202017/1proc/p1028.pdf. Read: full text (key sections).
- [Long2017] Long, Y., and Aleven, V. (2017). Enhancing learning outcomes through self-regulated learning support with an Open Learner Model. User Modeling and User-Adapted Interaction. https://dl.acm.org/doi/10.1007/s11257-016-9186-6. Full text copy: https://studylib.net/doc/26309930/s11257-016-9186-6. Read: full text (key sections).
- [Major2021] Major, L., Francis, G. A., and Tsapali, M. (2021). The effectiveness of technology-supported personalised learning in low- and middle-income countries: A meta-analysis. British Journal of Educational Technology, 52(5), 1935-1964. https://bera-journals.onlinelibrary.wiley.com/doi/full/10.1111/bjet.13116. Read: abstract and main results.
- [Mitrovic2007] Mitrovic, A., and Martin, B. (2007). Evaluating the Effect of Open Student Models on Self-Assessment. International Journal of Artificial Intelligence in Education, 17(2). https://doi.org/10.3233/irg-2007-17(2)03. Read: abstract only.
- [Mueller1998] Mueller, C. M., and Dweck, C. S. (1998). Praise for Intelligence Can Undermine Children's Motivation and Performance. Journal of Personality and Social Psychology, 75(1), 33-52. https://doi.org/10.1037//0022-3514.75.1.33. Open access: https://www.columbia.edu/cu/psychology/courses/3615/Readings/Mueller_Dweck.pdf. Read: full text (key sections).
- [Murray2002] Murray, T., and Arroyo, I. (2002). Toward Measuring and Maintaining the Zone of Proximal Development in Adaptive Instructional Systems. ITS 2002. https://doi.org/10.1007/3-540-47987-2_75. Full text copy: https://pdfcookie.com/documents/toward-measuring-and-maintaining-the-zone-of-proximal-development-in-adaptive-instructional-systems-9mlxqpek3273. Read: full text.
- [Niemiec1996] Niemiec, R. P., Sikorski, C., and Walberg, H. J. (1996). Learner-Control Effects: A Review of Reviews and a Meta-Analysis. Journal of Educational Computing Research, 15(2), 157-174. https://doi.org/10.2190/jv1u-eq5p-x2pb-pdba. Read: abstract only; the effect size -0.03 from the secondary table at https://www.visiblelearningmetax.com/influences/view/student_control_over_learning.
- [Pan2023] Pan, S. C., and Carpenter, S. K. (2023). Prequestioning and Pretesting Effects: a Review of Empirical Research, Theoretical Perspectives, and Implications for Educational Practice. Educational Psychology Review, 35, article 97. https://link.springer.com/article/10.1007/s10648-023-09814-5. Read: full text (key sections).
- [Papousek2016] Papousek, J., Stanislav, V., and Pelanek, R. (2016). Impact of Question Difficulty on Engagement and Learning. ITS 2016, Lecture Notes in Computer Science 9684, 267-272. https://doi.org/10.1007/978-3-319-39583-8_28. Open access: https://www.fi.muni.cz/~xpelanek/publications/its-target-difficulty.pdf. Longer technical report: https://www.fi.muni.cz/reports/files/2016/FIMU-RS-2016-02.pdf. Read: full text (technical report).
- [Pardos2011] Pardos, Z. A., and Heffernan, N. T. (2011). KT-IDEM: Introducing Item Difficulty to the Knowledge Tracing Model. UMAP 2011. https://web.cs.wpi.edu/~nth/pubs_and_grants/papers/2011/UMAP/Pardos%20Introducing%20Item%20Difficulty.pdf. Read: the passage describing the mastery rules of ASSISTments and the Cognitive Tutors.
- [Pashler2008] Pashler, H., McDaniel, M., Rohrer, D., and Bjork, R. (2008). Learning Styles: Concepts and Evidence. Psychological Science in the Public Interest. https://journals.sagepub.com/doi/full/10.1111/j.1539-6053.2009.01038.x. Read: full text (key sections).
- [Posner1982] Posner, G. J., Strike, K. A., Hewson, P. W., and Gertzog, W. A. (1982). Accommodation of a scientific conception: Toward a theory of conceptual change. Science Education, 66(2), 211-227. https://onlinelibrary.wiley.com/doi/full/10.1002/sce.3730660207. Open access: https://eclass.uoa.gr/modules/document/file.php/PHS122/%CE%91%CF%81%CE%B8%CF%81%CE%B1/Posner_Strike_Hewson_Gertzog.pdf. Read: the section on conditions of accommodation.
- [Razzaq2010] Razzaq, L., and Heffernan, N. T. (2010). Hints: Is It Better to Give or Wait to Be Asked? ITS 2010, 349-358. https://dl.acm.org/doi/10.1007/978-3-642-13388-6_39. Open access: https://web.cs.wpi.edu/~nth/pubs_and_grants/ITS%202010/RazzaqHintsIsItBettertoGiveorWaittobeAsked.pdf. Read: full text (key sections).
- [Reis1998] Reis, S. M., Westberg, K. L., Kulikowich, J. M., and Purcell, J. H. (1998). Curriculum Compacting and Achievement Test Scores: What Does the Research Say? Gifted Child Quarterly, 42(2), 123-129. https://doi.org/10.1177/001698629804200206. Read: abstract only.
- [Renkl2003] Renkl, A., and Atkinson, R. K. (2003). Structuring the Transition From Example Study to Problem Solving in Cognitive Skill Acquisition: A Cognitive Load Perspective. Educational Psychologist, 38(1), 15-22. https://www.tandfonline.com/doi/abs/10.1207/S15326985EP3801_3. Open access: https://mrbartonmaths.com/resourcesnew/8.%20Research/Explicit%20Instruction/Structuring%20the%20Transition%20From%20Example%20Study%20to%20Problem%20Solving.pdf. Read: full text (key sections).
- [Richland2009] Richland, L. E., Kornell, N., and Kao, L. S. (2009). The pretesting effect: Do unsuccessful retrieval attempts enhance learning? Journal of Experimental Psychology: Applied, 15(3), 243-257. https://doi.org/10.1037/a0016496. Open access: https://learninglab.uchicago.edu/Pre-Testing_files/RichlandKornellKao.pdf. Read: full text (key sections).
- [Roelle2015] Roelle, J., Mueller, C., Roelle, D., and Berthold, K. (2015). Learning from Instructional Explanations: Effects of Prompts Based on the Active-Constructive-Interactive Framework. PLoS ONE, 10(4), e0124115. https://journals.plos.org/plosone/article?id=10.1371%2Fjournal.pone.0124115. Read: full text (key sections).
- [Rohrer2006] Rohrer, D., and Taylor, K. (2006). The effects of overlearning and distributed practise on the retention of mathematics knowledge. Applied Cognitive Psychology, 20(9), 1209-1224. https://doi.org/10.1002/acp.1266. Open access: https://files.eric.ed.gov/fulltext/ED505642.pdf. Read: full text (key sections). The earlier open-access link (uweb.cas.usf.edu) timed out on 2026-11-22, so it was replaced with the ERIC copy of the same paper.
- [Roll2014] Roll, I., Baker, R. S. J. d., Aleven, V., and Koedinger, K. R. (2014). On the Benefits of Seeking (and Avoiding) Help in Online Problem-Solving Environments. Journal of the Learning Sciences, 23(4), 537-560. https://doi.org/10.1080/10508406.2014.883977. Open access: https://learninganalytics.upenn.edu/ryanbaker/Roll%20HT%20JLS%20clean.pdf. Read: full text (key sections).
- [Rosenshine2012] Rosenshine, B. (2012). Principles of Instruction: Research-Based Strategies That All Teachers Should Know. American Educator, 36(1). https://www.aft.org/sites/default/files/Rosenshine.pdf. Read: full text.
- [Salden2010] Salden, R. J. C. M., Aleven, V., Schwonke, R., and Renkl, A. (2010). The expertise reversal effect and worked examples in tutored problem solving. Instructional Science, 38(3), 289-307. https://doi.org/10.1007/s11251-009-9107-8. Open access: http://www.cee.uma.pt/ron/Salden%20et%20al.%20-%20The%20Expertise%20Reversal%20Effect%20and%20Worked%20Examples.pdf. Read: full text (key sections).
- [Schroeder2022] Schroeder, N. L., and Kucera, A. C. (2022). Refutation Text Facilitates Learning: a Meta-Analysis of Between-Subjects Experiments. Educational Psychology Review, 34(2), 957-987. https://doi.org/10.1007/s10648-021-09656-z. Open access: https://pmc.ncbi.nlm.nih.gov/articles/PMC8784251/. Read: full text (key sections).
- [Selent2015] Selent, D., and Heffernan, N. (2015). When More Intelligent Tutoring in the Form of Buggy Messages Does Not Help. AIED 2015, Lecture Notes in Artificial Intelligence 9112, 768-771. DOI 10.1007/978-3-319-19773-9_112 (as printed on the paper). Fetched at https://d-miller.github.io/DRK12/topic1/8362.pdf. Read: full text.
- [Settles2016] Settles, B., and Meeder, B. (2016). A Trainable Spaced Repetition Model for Language Learning. ACL 2016. https://aclanthology.org/P16-1174.pdf. Company blog on the same work: https://blog.duolingo.com/how-we-learn-how-you-learn/. Read: full text (key sections); blog excerpt.
- [Sharada2017] Sharada, N., Shashi, M., and Xiong, X. (2017). Identification of Wheel Spinning Cases while Learning and Retaining a Skill in Intelligent Tutoring Systems. International Journal of Computer Applications, 179(3). https://www.ijcaonline.org/archives/volume179/number3/sharada-2017-ijca-915906.pdf. Read: search excerpt only. Used only as an example of a secondary figure for wheel-spinning time.
- [Shute2008] Shute, V. J. (2008). Focus on Formative Feedback. Review of Educational Research, 78(1), 153-189. https://journals.sagepub.com/doi/10.3102/0034654307313795. Open access: https://myweb.fsu.edu/vshute/pdf/shute%202008%5Fb.pdf. Read: the guideline tables and the timing section.
- [Sinha2021] Sinha, T., and Kapur, M. (2021). When Problem Solving Followed by Instruction Works: Evidence for Productive Failure. Review of Educational Research, 91(5), 761-798. https://doi.org/10.3102/00346543211019105. Full text page: https://journals.sagepub.com/doi/full/10.3102/00346543211019105. Read: full text (key sections). A critical secondary summary: https://teachingdb.com/source/sinha-kapur-2021-productive-failure.
- [Sleeman1989] Sleeman, D., Kelly, A. E., Martinak, R., Ward, R. D., and Moore, J. L. (1989). Studies of Diagnosis and Remediation with High School Algebra Students. Cognitive Science, 13(4), 551-568. https://doi.org/10.1016/0364-0213(89)90023-2. Abstract page: https://onlinelibrary.wiley.com/doi/abs/10.1207/s15516709cog1304_3. Repository record: https://eprints.hud.ac.uk/id/eprint/7290/ (the search tool's saved copy of this page held the full text that the citation check used). Read: full text (key sections).
- [Smit2025] Smit, N., de Kleijn, R., Wicherts, J. M., and van de Pol, J. (2025). What It Takes to Tutor - A Preregistered Direct Replication of the Scaffolding Experimental Study by D. Wood et al. (1978). Journal of Educational Psychology, 117(8), 1313-1329. https://doi.org/10.1037/edu0000965. Open access: https://research-portal.uu.nl/ws/files/275996783/2026-36551-001.pdf. Read: abstract and method.
- [Steenbergen2016] Steenbergen-Hu, S., Makel, M. C., and Olszewski-Kubilius, P. (2016). What One Hundred Years of Research Says About the Effects of Ability Grouping and Acceleration on K-12 Students' Academic Achievement: Findings of Two Second-Order Meta-Analyses. Review of Educational Research, 86(4), 849-899. https://doi.org/10.3102/0034654316675417. Read: abstract only.
- [StHilaire2024] St. Hilaire, K. J., Chan, J. C. K., and Ahn, D. (2024). Guessing as a learning intervention: A meta-analytic review of the prequestion effect. Psychonomic Bulletin & Review, 31(2). https://doi.org/10.3758/s13423-023-02353-8. Read: full text (key sections).
- [Sweller1985] Sweller, J., and Cooper, G. A. (1985). The Use of Worked Examples as a Substitute for Problem Solving in Learning Algebra. Cognition and Instruction, 2(1), 59-89. https://www.tandfonline.com/doi/abs/10.1207/s1532690xci0201_3. Read: abstract only.
- [Tabibian2019] Tabibian, B., Upadhyay, U., De, A., Zarezade, A., Schoelkopf, B., and Gomez-Rodriguez, M. (2019). Enhancing human learning via spaced repetition optimization. PNAS, 116(10), 3988-3993. https://www.pnas.org/doi/10.1073/pnas.1815156116. Open access: https://pmc.ncbi.nlm.nih.gov/articles/PMC6410796/. Read: abstract and method.
- [Tetzlaff2025] Tetzlaff, L., Simonsmeier, B., Peters, T., and Brod, G. (2025). A cornerstone of adaptivity - A meta-analysis of the expertise reversal effect. Learning and Instruction, 98, 102142. https://doi.org/10.1016/j.learninstruc.2025.102142. Open access: https://www.pedocs.de/volltexte/2026/34113/pdf/Learn_and_Instr_2025_Tetzlaff_u.a._A_cornerstone_of_adaptivity.pdf. Read: full text (key sections).
- [Tippett2010] Tippett, C. D. (2010). Refutation Text in Science Education: A Review of Two Decades of Research. International Journal of Science and Mathematics Education, 8(6), 951-970. https://doi.org/10.1007/s10763-010-9203-x. Open access: https://www.academia.edu/22717094/Refutation_Text_in_Science_Education_A_Review_of_Two_Decades_of_Research. Read: full text (key sections).
- [VanderKleij2015] Van der Kleij, F. M., Feskens, R. C. W., and Eggen, T. J. H. M. (2015). Effects of Feedback in a Computer-Based Learning Environment on Students' Learning Outcomes: A Meta-Analysis. Review of Educational Research, 85(4), 475-511. https://journals.sagepub.com/doi/10.3102/0034654314564881. Open access: https://www.researchgate.net/publication/272923307_Effects_of_Feedback_in_a_Computer-Based_Learning_Environment_on_Students'_Learning_Outcomes_A_Meta-Analysis. Read: full text (key sections).
- [vandePol2010] van de Pol, J., Volman, M., and Beishuizen, J. (2010). Scaffolding in Teacher-Student Interaction: A Decade of Research. Educational Psychology Review, 22(3), 271-296. https://link.springer.com/article/10.1007/s10648-010-9127-6. Read: full text (key sections).
- [VanLehn1983] VanLehn, K. (1983). Human procedural skill acquisition: theory, model and psychological validation. Proceedings of the National Conference on Artificial Intelligence (AAAI-83), 420-423. https://aaai.org/Papers/AAAI/1983/AAAI83-055.pdf. Copy first fetched: https://scispace.com/pdf/human-procedural-skill-acquisition-theory-model-and-y0znv15war.pdf. Read: the passages on bugs and repair theory. Year and venue confirmed on the AAAI page in the citation check. Stands in for VanLehn's book Mind Bugs (1990), which I did not fetch.
- [VanLehn2007] VanLehn, K., Graesser, A. C., Jackson, G. T., Jordan, P., Olney, A., and Rose, C. P. (2007). When Are Tutorial Dialogues More Effective Than Reading? Cognitive Science, 31(1), 3-62. https://doi.org/10.1080/03640210709336984. Open access: https://olney.ai/pdf/When-are-tutorial-dialogues-more-effective-than-reading.pdf. Read: abstract and key sections.
- [VanLehn2011] VanLehn, K. (2011). The Relative Effectiveness of Human Tutoring, Intelligent Tutoring Systems, and Other Tutoring Systems. Educational Psychologist, 46(4), 197-221. https://www.tandfonline.com/doi/full/10.1080/00461520.2011.611369. Read: abstract; the table of pairwise effects from the author's slides at https://www.nectec.or.th/icce2011/speakers/ICCE2011_kurt_20111117.pdf.
- [Wang2024] Wang, X., Huang, R., Sommer, M., Pei, B., Shidfar, P., Rehman, M. S., Ritzhaupt, A. D., and Martin, F. (2024). The Efficacy of Artificial Intelligence-Enabled Adaptive Learning Systems From 2010 to 2022 on Learner Outcomes: A Meta-Analysis. Journal of Educational Computing Research. https://journals.sagepub.com/doi/10.1177/07356331241240459. Read: abstract only.
- [WangJ2023] Wang, J., Zhou, T., Ye, T., and Luo, J. (2023). Research on the effects of adaptive learning on learning outcomes of university students: based on a meta-analysis of 51 experimental and quasi-experimental studies (in Chinese, with an English abstract). Chongqing Higher Education Research, 11(6), 116-127. https://www.cqgjxh.com/upload/file/202311/1699461900690492.pdf. Archived copy: http://web.archive.org/web/20260420000251/https://www.cqgjxh.com/upload/file/202311/1699461900690492.pdf. Read: abstract and main-effect results. The publisher host timed out on 2026-11-22, so the check used the archived copy.
- [Wilson2019] Wilson, R. C., Shenhav, A., Straccia, M., and Cohen, J. D. (2019). The Eighty Five Percent Rule for optimal learning. Nature Communications, 10, 4646. https://www.nature.com/articles/s41467-019-12552-4. Read: full text.
- [Wisniewski2020] Wisniewski, B., Zierer, K., and Hattie, J. (2020). The Power of Feedback Revisited: A Meta-Analysis of Educational Feedback Research. Frontiers in Psychology, 10, 3087. https://www.frontiersin.org/journals/psychology/articles/10.3389/fpsyg.2019.03087/full. Open access: https://pmc.ncbi.nlm.nih.gov/articles/PMC6987456/. Read: full text (key sections).
- [Wood1976] Wood, D., Bruner, J. S., and Ross, G. (1976). The Role of Tutoring in Problem Solving. Journal of Child Psychology and Psychiatry, 17(2), 89-100. https://doi.org/10.1111/j.1469-7610.1976.tb00381.x. Read: full text (key sections).
- [Wood1978] Wood, D., Wood, H., and Middleton, D. (1978). An Experimental Evaluation of Four Face-to-Face Teaching Strategies. International Journal of Behavioral Development, 1(2), 131-147. https://journals.sagepub.com/doi/10.1177/016502547800100203. Read: abstract only; the five levels from the secondary summary at http://web.archive.org/web/20240417212809/https://www.psy.gla.ac.uk/~steve/courses/archive/CERE09-10-safari-archive/LaurenClark/webarchive-index.html (the live psy.gla.ac.uk page timed out on 2026-11-22, so the link points to the Internet Archive snapshot). The same five levels are listed in [Smit2025].
- [Wood1999] Wood, H., and Wood, D. (1999). Help seeking, learning and contingent tutoring. Computers & Education, 33(2-3), 153-169. https://doi.org/10.1016/s0360-1315(99)00030-5. Open access: https://www.tlu.ee/~kpata/haridustehnoloogiaTLU/tutoring.pdf. Read: full text (key sections).
- [Ye2022] Ye, J., Su, J., and Cao, Y. (2022). A Stochastic Shortest Path Algorithm for Optimizing Spaced Repetition Scheduling. KDD 2022, 4381-4390. https://dl.acm.org/doi/10.1145/3534678.3539081. Read: abstract and simulation results.
- [Zhang2019] Zhang, C., Huang, Y., Wang, J., Lu, D., Fang, W., Stamper, J., Fancsali, S., Holstein, K., and Aleven, V. (2019). Early Detection of Wheel Spinning: Comparison across Tutors, Models, Features, and Operationalizations. EDM 2019, 468-473. https://files.eric.ed.gov/fulltext/ED599222.pdf. Read: full text.

Named leads from the brief that I did not fetch and therefore do not cite directly:

- Brown, J. S., and Burton, R. R. (1978). Diagnostic models for procedural bugs in basic mathematical skills. Cognitive Science, 2(2), 155-192. Described through [Selent2015] and [VanLehn1983].
- VanLehn, K. (1990). Mind Bugs: The Origins of Procedural Misconceptions. MIT Press. Described through [VanLehn1983].
- Jung, L. A. (2024). Thriving in the Zone of Productive Struggle. ASCD. A practitioner article that the prototype cites for its help ladder. Not fetched in this stream.

## Citation check

- **Date:** 2026-11-22.
- **Checker:** a second agent, working independently of the agent that wrote the file.
- **Method:** Every source-list entry was swept. Each DOI went to the Crossref API (title, authors, year, venue, update notices) and to OpenAlex (retraction flag, abstract). The arXiv paper went to the arXiv API. Every other URL got an HTTP GET with a look at the status and the page title. Where a publisher page answered with a 403 or a bot challenge, the paper was confirmed through Crossref and, where one could be fetched, an open copy (author PDF, repository, PubMed Central, ERIC, Internet Archive).
- **Deep check:** Every number and attributed claim in "Top findings" was read against the cited source or an open copy of it, fetched today. About 60 further claims in "Findings by topic" were checked the same way, chosen for being thresholds, rates and effect sizes that back a design rule. For seven sources behind bot walls, the text read was the page text that the web search tool saved today: the Sage full text for [Sinha2021], ResearchGate or Academia.edu copies of the papers for [VanderKleij2015], [Tippett2010] and [KalyugaSweller2005], the Huddersfield repository page for [Sleeman1989], and repository or ACM abstract pages for [DMello2014] and [Attali2015].
- **Injection check:** every fetched page read as ordinary article, repository, vendor or project documentation content.

### Counts

- Sources swept: 113
- Sources fine: 95 (6 of them gained a note or volume and page details: [Bastani2025], [Bull2016], [Graesser1995], [Gusukuma2018], [Haimovitz2011], [KalyugaSweller2005])
- Sources corrected: 18
- Sources dropped: 0
- Top-finding claims checked: 90
- Confirmed on the primary source: 87
- Confirmed on a secondary page only: 3
- Unconfirmed: 0
- Wrong: 0
- Findings-by-topic claims checked: 60 (58 confirmed on the primary source, 2 unconfirmed)
- Retractions: 0 among the 94 sources with a registered DOI, by Crossref update notices and the OpenAlex retraction flag

### Corrections to the source list

- [Aleven2016adapt] was: open-access link to a cs.cmu.edu address that returns 404 today. now: Internet Archive snapshot of the same prepublication PDF (fetched and read today).
- [Cen2007] was: "read through the authors' PDF of the same study under the title Do Less and Learn More". now: the authors' PDF is the published paper itself, with the published title and the same abstract as the IOS Press page on its first page; the other title sits only in the PDF's document properties. Series and volume added (Frontiers in Artificial Intelligence and Applications 158).
- [Corbett2001] was: only the ACM identifier 10.5555/647664.733401, which Crossref does not register. now: Springer DOI 10.1007/3-540-44566-8_14 added, with the series volume (LNCS 2109).
- [DMello2008] was: "D'Mello, S. K., Craig, S. D., and colleagues", venue without volume. now: all five authors (D'Mello, Craig, Witherspoon, McDaniel, Graesser), 18(1-2), 45-80.
- [DMello2014] was: publisher page link only. now: DOI 10.1016/j.learninstruc.2012.05.003 added.
- [Frontiers2026] was: "Authors not shown on the fetched page". now: Tang, Cao, Shen, Kang, Yang, Chang and Fan, Frontiers in Psychology 17, 1944626, with the DOI link.
- [iJAC2024] was: "Authors not shown on the fetched page (about 2024, volume 17 issue 4)". now: Lynch and DeGenarro (2024), 17(4), 58-68.
- [ITSE2026] was: "Authors not shown on the fetched page". now: Pagano, Mehrvarz, McLaren, Marengo and Santamato (2026), 23(4), 581-608.
- [Kalyuga2005] was: key Kalyuga2006, "year not shown on the fetched page, about 2005 to 2006". now: 2005 (Proceedings of the Annual Meeting of the Cognitive Science Society, 27, per the eScholarship cover page), key renamed in all 6 places, direct PDF link added.
- [Kerly2008] was: link to acm-prod.literatumonline.com, an internal platform host. now: DOI link, with series and pages.
- [Major2021] was: "Major, L., and colleagues". now: Major, Francis and Tsapali, 52(5), 1935-1964.
- [Rohrer2006] was: open-access link to uweb.cas.usf.edu, which timed out today. now: ERIC copy ED505642 of the same paper (fetched and read today).
- [Sharada2017] was: "Sharada and colleagues". now: Sharada, Shashi and Xiong, from the first page of the PDF.
- [Sleeman1989] was: abstract page only. now: Huddersfield repository record added; the full text behind it confirmed the 44 students assigned, 38 analyzed, t(37) = 4.20 and t(36) = .09.
- [VanLehn1983] was: "about 1983; year as I recall it", with a scispace copy and the venue missing. now: 1983 confirmed, AAAI-83 proceedings, 420-423, with the AAAI link.
- [WangJ2023] was: "Wang, J., Zhou, T., Ye, T., and colleagues", venue missing, link timing out today. now: fourth author Luo added, Chongqing Higher Education Research 11(6), 116-127, Internet Archive copy added (fetched and read today; g = 0.751 and 0.542 confirmed).
- [Wood1978] was: secondary summary on a psy.gla.ac.uk page that timed out today. now: Internet Archive snapshot of that page (fetched today), plus a pointer to [Smit2025], which lists the same five levels.
- [Ye2022] was: "Ye, J., and colleagues". now: Ye, Su and Cao, with pages 4381-4390.

### Corrections to claims and their provenance

- [Graesser1995] was: "The 92 percent figure comes from a secondary source" (section 3.2 caveat and the source entry). now: the figure is in Graesser et al. itself, in the passage on step 5 ("In most cases (92% of the time) the tutor simply asked the student whether the student understood the answer"). The number stands.
- [KalyugaSweller2005] was: d = 0.69 "as summarized on the fetched page" (section 6.2 and the source entry). now: found in the paper's results text (efficiency gains, t(14) = 1.89, one-tailed p < .05, Cohen's d 0.69). The number stands.
- All other checked numbers match their sources as written, so the body text keeps them.

### Dropped sources

- All 113 sources exist and stay in the source list.

### Top-finding claims confirmed on a secondary page only

- [VanLehn2011] Substep-based tutoring 0.40 and answer-based tutoring 0.31 (top finding 12): confirmed on the author's ICCE 2011 slides (nectec.or.th), which reproduce the paper's table. The paper's abstract, fetched today, confirms the other two values (human tutors 0.79, intelligent tutoring systems 0.76).
- [Corbett2001] Cognitive mastery +0.89 SD at 40 percent more problems (top finding 24): confirmed on the WPI student report (digitalcommons.wpi.edu/iqp-all/1197), which summarizes the paper. The paper's abstract, fetched today, describes the study in words only. The 0.95 mastery threshold in the same bullet is confirmed on [Pardos2011].
- [Niemiec1996] Learner control -0.03 (top finding 25): confirmed on the Visible Learning MetaX page. The paper's abstract, fetched today, confirms the 24 studies and an average effect that was "small and negative".

### Claims left unconfirmed

- [Kapur2014] Section 1.6: d = 2.00 and 1.52 (75 students), d = 1.35, 1.23 and 0.80 (111 students), r = .82 and .88. The publisher page sits behind a bot wall, and the abstract fetched today states only the direction of each result. Marked "(unconfirmed)" in place.
- [Fong2015] Section 7.3: 9,620 participants, Fisher's z = 0.52 with its interval, 0.32 with 25 samples and 4,270 participants. The publisher and ResearchGate pages sit behind bot walls, and the abstract fetched today confirms the 28 studies and the direction only. Marked "(unconfirmed)" in place.

### Items the first agent flagged, and how each was settled

- Author lists given from memory: [Bull2016], [Gusukuma2018] and [Haimovitz2011] all match Crossref.
- Publication year given from memory: [VanLehn1983] is 1983 (AAAI-83). The year left open on [Kalyuga2005] is 2005.
- Three trials cited by title only: [Frontiers2026], [iJAC2024] and [ITSE2026] now carry authors, volume and pages from Crossref.
- Cen, Koedinger and Junker 2007: the PDF that was read is the published AIED 2007 paper. The 12 percent saving (about 22 minutes) is the paper's own figure. The 20 percent figure is a one-line summary in [Koedinger2010] ("took 20% less time"). The file already treats 12 percent as the study result, and that stands.

### Other observations

- Correction notices: [Bastani2025] has a 2025 correction that changes one author's affiliation only. [Doroudi2019] is covered by a one-line publisher notice that applies to five articles of the journal issue (IJAIED 29(4)) as a group.
- [Doroudi2019] In the author version, Table 2 lists 11 studies under "not significant" and its first row sums to 41, while the running text says "Ten studies" and "36 studies". The file follows the running text, and the top-finding figure (21 of 36) is the paper's own sentence.
- [Ahtisham2026] is still a single-version arXiv preprint (v1, 2026-09-29). The abstract gives the per-turn drop as 12.7 percent (AOR 0.873) and the body text as 13 percent (AOR 0.87), so the file's 13 percent stands.
- [Tetzlaff2025] The pedocs PDF link timed out from the checking machine while the pedocs record page for the same item loaded through the web fetch tool, so the link stays.
- Publisher and aggregator pages for about 100 of the 193 links (Sage, Wiley, Taylor and Francis, APA, ACM, Springer, ScienceDirect, PNAS, ResearchGate, Academia.edu, PubMed) answered the shell with a 403 or a challenge page. Each of those papers is confirmed by Crossref, and the links stay.
- [Mueller1998] The 412 fifth graders in top finding 19 is the sum of the six study samples in the paper (128, 51, 88, 51, 46, 48).
- Named leads outside the keyed list: Brown and Burton 1978 is confirmed by Crossref (Cognitive Science 2(2), 155-192). VanLehn's Mind Bugs is confirmed through a 1991 review of the book in Artificial Intelligence. Jung 2024 is a magazine article outside Crossref and stays an unfetched lead, cited by the prototype and kept apart from this file's evidence.

## Breadcrumbs

- **Breadcrumb 1**
  - severity: low
  - category: cap-hit
  - note: The source count (113) is well above the 40 to 60 target because the brief named over 70 leads across ten topics.
- **Breadcrumb 2**
  - severity: medium
  - category: vetting-fail
  - note: About 23 sources were read as abstracts only or through secondary summaries, and each is labeled in the source list and in "How I checked things".
- **Breadcrumb 3**
  - severity: medium
  - category: vetting-fail
  - note: The only chat-turn evidence on stuck students with an LLM tutor is one unreplicated 2026 arXiv preprint, graded C and labeled wherever it is used.
- **Breadcrumb 4**
  - severity: low
  - category: ambiguous-input
  - note: Cen, Koedinger and Junker 2007 was read through an author PDF with a different title, and the time saving is 12 percent there against 20 percent in a later summary by the same lab.
- **Breadcrumb 5**
  - severity: low
  - category: vetting-fail
  - note: Brown and Burton 1978, VanLehn's Mind Bugs and Jung 2024 were not fetched, and three recent trials are cited by title because the fetched pages did not show authors.
- **Breadcrumb 6**
  - severity: low
  - category: scope-drift
  - note: Sections 11 and 12 regroup the findings against the six requested outputs and against rules H6 to H18, which goes one step beyond the prescribed structure and stays descriptive.
- **Breadcrumb 7**
  - severity: low
  - category: other
  - note: A few author lists and one publication year are given from memory where the fetched page showed only a title, and each such entry says so.

