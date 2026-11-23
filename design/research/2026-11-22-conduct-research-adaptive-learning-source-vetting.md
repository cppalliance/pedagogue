---
produced: 2026-11-22
title: "Adaptive learning research stream 7 - web source discovery and vetting for lesson plans and LLM tutors: source credibility rubrics (CRAAP, SIFT, lateral reading, Achieve OER, MERLOT), LLM credibility and relevance judging, link rot and content drift, readability and prior knowledge, prerequisite graph extraction, citation faithfulness and hallucination rates, claim verification (FActScore, SAFE, CoVe), prompt injection from fetched pages (dual LLM, CaMeL, AgentDojo, spotlighting), deep research search strategy, archiving and provenance"
---

# Stream 7 - Web source discovery and vetting (finding, judging and safely reading web material for a lesson)

## How I checked things

- Date of research: 2026-11-22.
- Context read first: the Pedagogue vision (rule H11, question Q18) and the tutor prototype (Step 1 prerequisite chain, Step 3 Pass B six-test rubric, injection-defense directive, Sideband Drill-down Protocol).
- Tools: a web search tool and a web fetch tool. The search tool returns a short machine-written synthesis, excerpts from each result page, and for some results the full page text saved for searching. The fetch tool returns the page text.
- What "Read" means in the source list: "full text" means the full page text was fetched and I read the passages that hold the findings (abstract, method, results, limitations) by targeted search inside it, not cover to cover. "Search excerpt" means I saw only the passages the search tool returned. "Abstract only" and "secondary summary at URL" mean what they say.
- Depth: about 30 sources were read at full-text level. The ones that matter most and got the closest reading are [Liu2023], [Tow2025], [BBCEBU2025], [Wu2024], [Walters2023], [Chelli2024], [JillWatson2024], [Min2023], [Wei2024], [Dhuliawala2023], [Willison2023], [Willison2025], [Beurer2025], [Debenedetti2025], [Nasr2025], [Hines2024], [AgentDojo2024], [Wineburg2019], [Breakstone2018], [Zittrain2021], [McNamara1996], [BrowseComp2025] and [AnthropicResearch2025].
- Numbers: every number in this file was seen on a fetched page or in a search-result excerpt of the page. Where a number comes from the search tool's own synthesis or from a secondary page, the text says so ("search summary", "secondary summary", "reported").
- Machine-written syntheses from the search tool were used only as pointers. Five numbers rest on them alone and are labelled "search summary": the Achieve Rubric II score descriptors, the 23 to 34% variance figure for readability formulas, the LLM-human QWK values for logic hints, the 76.35% drift figure, and SelfCheckGPT's 20 samples.
- Unconfirmed after this pass: the effect size of lateral-reading instruction; EdReports' method; Lexile validity; what quotation of an all-rights-reserved page permits; the Ahrefs and Originality.ai AI-content figures at their primary sources; the exact wording of OpenAI, Microsoft, Google and Anthropic documentation quoted through practitioner blogs; RefD's accuracy numbers; the task setting behind the arXiv 2410.11531 accuracy figures; the journal venues of several papers read as arXiv preprints; and a few author lists flagged in the source list.
- Injection handling: several fetched pages contain example attack strings (Willison's posts, OWASP scenarios). They were treated as quoted data. No page addressed an instruction to the reader, and nothing on any page changed the scope of this work.
- Citation check, same day: a second agent re-fetched every source and corrected this file in place. The `## Citation check` section near the end lists what changed. It settles several items in the two bullets above: all five "search summary" numbers were found on their primary sources, the arXiv 2410.11531 figures turned out to measure something else and that source was dropped, and the author lists and several venues were filled in.

## Top findings

Where the five things the report writer asked for are covered: (1) vetting rubric and how reliably a model applies it: findings 1 to 5 and topic 1; (2) error rates for citations and summaries and what cuts them: findings 12 to 18 and topics 5 and 6; (3) a provenance scheme a reviewer can check fast: findings 6, 7, 17 and 24 and topics 6.4, 9.2 and 9.3; (4) a reader design safe against hostile pages: findings 19 to 22 and topic 7; (5) evidence bearing on live search during a lesson: findings 13, 15, 16 and 19 to 23 and topics 5.5, 7 and 8.

1. **Skilled evaluators judge a source by leaving it, and page-feature checklists are easy to fool.** Fact checkers scored a perfect 2.0 on judging a deceptive site where historians scored 0.7 and Stanford students 0.16; only 5% of 197 undergraduates identified the PR firm behind a site that passes the CRAAP checklist. Six lessons on lateral reading produced a significant gain in a matched-control field study (271 vs 228 students; effect size unconfirmed). Grade B. [Wineburg2019], [Breakstone2018], [Wineburg2022]
2. **Rubrics for open educational resources name the teaching-quality dimensions and score each object separately.** Achieve uses eight rubrics on a 0 to 3 scale (alignment to objective, quality of explanation, practice exercises, assessment, deeper learning, accessibility and others). MERLOT uses three dimensions and two independent expert reviewers, with a third on disagreement. Neither reports a link to measured learning. Grade C. [AchieveOER], [MERLOT], [MERLOTForm]
3. **A model's credibility rating of a source agrees only moderately with experts.** Across nine LLMs and 7,523 news domains, agreement with expert ratings averaged Spearman rho 0.50, while the models agreed with each other at 0.79, so several models are not independent judges. Grade B. [Yang2023]
4. **A model can judge topical relevance about as well as trained people, and the result swings with prompt wording.** Cohen's kappa against gold labels ranged from 0.20 to 0.64 across 32 prompts (human labellers 0.24 to 0.58); an independent pilot found 0.26 to 0.40. Grade B. [Thomas2024], [Faggioli2023]
5. **Evidence that a model can judge teaching quality is thin.** On tutor explanations, model-expert agreement was QWK 0.30 to 0.59 for clarity-type criteria and 0.16 for a pedagogy-specific one, against expert-expert 0.79 to 0.92; one study found GPT-4 favoured HTML-formatted explanations. No study rates whole web pages. Grade B, application unconfirmed. [Tithi2025], [MathExplain2024], [Moore2024]
6. **Stored URLs rot at a steady rate.** 38% of pages from 2013 were gone by 2023; 25% of deep links in New York Times articles were dead (6% after about two years, 43% after about twelve); 54% of Wikipedia pages have a dead reference. Grade B. [Pew2024], [Zittrain2021]
7. **Live links often no longer say what was cited, and later archives often miss the cited version.** 13% of still-working NYT links had drifted significantly; a representative archived copy existed for only about 30% of web references in science papers, and of the references that had such a copy and a page still live to compare it with, over 75% had drifted. Grade B. [Zittrain2021], [Jones2016]
8. **Wikipedia was close to Britannica on science accuracy and weaker on structure.** 162 vs 123 errors across 42 entries, four serious errors each; reviewers called several Wikipedia articles poorly structured. Grade B. [Giles2005]
9. **Readability formulas are weak difficulty measures, and a model's rating does better without modelling the reader.** GPT-4 Turbo ratings correlated r = 0.76 with teacher judgments (human split-half 0.63); formulas rest on word and sentence length and old school-age norms. Grade B. [Crossley2022], [TSAR2024]
10. **Prior knowledge changes which text helps.** Low-knowledge readers scored .41 on a fully coherent text and .27 on a gappy one; high-knowledge readers scored .55 and .61. The benefit of gaps for experts is fragile across replications; the harm of gaps for novices is stable. Grade B. [McNamara1996], [McNamaraKintsch1996], [OReilly2007]
11. **People agree only moderately on prerequisite links, and models are about as good as a single annotator.** Human agreement runs from kappa 0.19 to 0.74, mostly 0.3 to 0.55; individual annotators hit 0.75 precision against consensus; zero-shot GPT-4 reached about 75% accuracy. Grade B. [Talukdar2012], [Liang2017], [PRET2018], [Yang2024Graph]
12. **In generative search, about half of sentences are fully supported by their citations.** 51.5% of sentences fully supported and 74.5% of citations accurate across four engines; perceived helpfulness correlated -0.96 with citation precision. Grade B. [Liu2023]
13. **Later audits show the problem persists.** Deep-research tools' citation accuracy was 78 to 90%; eight AI search tools answered more than 60% of source-identification queries wrongly; 45% of AI-assistant news answers had a significant issue and 31% a sourcing issue; 50 to 90% of medical answers were not fully supported by their own sources. Grade B. [DRBench2025], [Tow2025], [BBCEBU2025], [Wu2024]
14. **References written from model memory are often invented, and model literature search has low recall.** 55% (GPT-3.5) and 18% (GPT-4) of citations fabricated; 28.6% to 91.4% hallucinated in a systematic-review task, with recall of about 12 to 14%. Grade B. [Walters2023], [Chelli2024]
15. **Summarizing a supplied document still adds unsupported content.** Vendor-measured rates run from 3.3% to above 10% of summaries for current models, and 39 to 74% on deliberately hard human-labelled cases. Grade C and B. [Vectara2025], [FaithBench2025]
16. **A teaching assistant built to answer from vetted courseware passed 77% of test questions, against 31% for a generic file-retrieval assistant on the same underlying model, and about a quarter of its answers still failed.** Both systems answer by retrieving from course material, so the gap comes from retrieval and prompting design, not from grounding as such (the paper names the shared model and implies, without saying so, that the files were the same); accuracy was over 90% on textbook questions and 75 to 80% on syllabus questions. Grade B. [JillWatson2024], [AIALOE2024]
17. **Claim-by-claim checking works and is cheap, and requiring a verbatim quote makes each claim checkable by code.** ChatGPT biographies were 58% supported at claim level; an automated checker agreed with crowd annotators 72% of the time, won 76% of sampled disagreements and cost $0.19 against $4.00; a model matched a doctor panel on source support 88% of the time (preprint figure; 88.7% in the 2025 published version). A quote-citing model was judged high quality on 80% and 67% of answers. API-level quote extraction "removes fabricated references, not misread ones". Grade B, vendor practice C. [Min2023], [Wei2024], [Wu2024], [Menick2022], [Csorba2026]
18. **Self-verification in separate prompts cuts hallucination.** Chain-of-verification raised FActScore from 55.9 to 71.4 and list precision from 0.17 to 0.36, and worked best when verification answers could not see the draft. Grade B. [Dhuliawala2023]
19. **Every model follows instructions planted in fetched content some of the time.** GPT-4 agents were hijacked in 24% of cases (47% with a stronger prompt) in 2024; in a public competition reported in 2026 all 13 frontier models were breached, at 0.5% to 8.5% of attempts. Grade B. [InjecAgent2024], [GraySwan2026]
20. **Marking and filtering defenses look strong on fixed tests and fail against adaptive attackers.** Datamarking cut attack success from about 50% to about 3%; under adaptive attack a Spotlighting defense (which the attacking paper describes as delimiter marking, so not shown to be the datamarking variant) was broken more than 95% of the time, and 12 published defenses were bypassed at above 90% for most. Grade B. [Hines2024], [Nasr2025]
21. **Structural isolation is the defense with guarantees.** The rule shared by the dual LLM pattern, the six design patterns and CaMeL: once a model has read untrusted text, nothing it produces may trigger a consequential action. CaMeL completed 77% of benchmark tasks with provable security against 84% undefended. Grade B for CaMeL, D for the pattern arguments. [Willison2023], [Beurer2025], [Debenedetti2025]
22. **A quarantined reader can still be made to misreport the page.** CaMeL's authors exclude "text-to-text attacks" such as a falsified summary; free-text notes passed onward can re-inject the next model; only constrained outputs (a number, a fixed category, a reference) are safe to hand to a model with tools. No study measures the corrupted-summary rate for a tools-free reader. Grade B to D. [Debenedetti2025], [Willison2023], [Beurer2025]
23. **More query phrasings and more persistence find more, with fast-diminishing returns on phrasings.** Fusing query variants raised NDCG@10 from 0.182 to 0.254 at ten variants and 0.262 at one hundred. On hard lookups a deep-research agent found 51.5%, GPT-4o with browsing 1.9%, and unaided humans 29.2%. Research agents chose SEO content farms until given source-quality rules. Grade B and C. [Benham2019], [BrowseComp2025], [AnthropicResearch2025]
24. **The documented way to keep a source checkable is to store the original URL, the read date and a snapshot together at citation time.** The Robust Links specification, perma.cc and Wikipedia's citation fields all do this; the Internet Archive's bot and volunteers had repaired more than 9 million dead Wikipedia links by 2018. Grade C. [RobustLinks], [PermaGuide], [WikiArchive], [IABlog2018]

## Findings by topic

### 1. Judging source quality

**1.1 Checklists that judge a page by its own features (CRAAP) are easy to fool. Skilled fact checkers leave the page and check what others say about it (lateral reading).**

- Finding: In an expert-novice study, professional fact checkers judged web sources faster and more accurately than PhD historians and Stanford undergraduates. The fact checkers "read laterally": after a quick scan they opened new tabs and searched for what other sources said about the site. Historians and students "read vertically", staying on the site, and were taken in by logos, domain names and polished About pages.
- Evidence: 45 participants (10 PhD historians, 10 fact checkers, 25 Stanford undergraduates), think-aloud on live websites. On a task comparing the American Academy of Pediatrics with the American College of Pediatricians (a small splinter group), conclusions were scored 0 to 2. For the College site: fact checkers mean 2.0 (SD 0), historians 0.7 (SD 0.95), students 0.16 (SD 0.37), Kruskal-Wallis H(2) = 27.5, p < .001. For the Academy site: fact checkers 2.0, historians 1.2, students 0.4. 60% of students picked the splinter group as the more reliable site. 9 of 25 students never left the original site.
- Evidence (critique of CRAAP): Breakstone, McGrew, Ortega, Smith and Wineburg (2018, Phi Delta Kappan) walk the site minimumwage.com through the CRAAP test (currency, relevance, authority, accuracy, purpose) and show it passes each item: recent updates, relevant content, an About page with a street address, clean prose, a stated research purpose. The site belongs to the Employment Policies Institute, which the New York Times and Salon reported to be a front group funded by a Washington public relations firm working for the food and beverage industry. When 197 undergraduates were asked whether the site was reliable and were free to search the web, only 5% identified the PR firm behind it; of 95 11th graders in an AP history course, 10% did. Students often justified trust with checklist items such as the date of an article. The authors note that the common checklists run to 10 to 30 questions and "none of the checklists is based on research about what skilled people actually do". A 2024 open-access Springer chapter by the same group reports that Caulfield traced the CRAAP test's roots to a 1978 guide (search-result excerpt).
- Caveats: Small sample, three tasks, social and political topics. The fact checkers were from elite outlets. The study shows what experts do, it does not by itself show that teaching the method works (see 1.2). A slide deck in the University of Alberta repository ("Cut the CRAAP") notes that SIFT-style journalistic methods tend "to work best for mass media and may be less effective for some other types of information", which matters for technical teaching material where the question is less "who is behind this" and more "is this explanation correct and good".
- Grade: B (controlled expert-novice comparison). The CRAAP critique is C (worked example plus two classroom samples with no control group).
- Sources: [Wineburg2019], [Breakstone2018], [Breakstone2024], [CutTheCRAAP].

**1.2 Lateral reading can be taught, and the gain shows up in controlled field studies.**

- Finding: Six classroom lessons on lateral reading improved high school students' ability to judge the credibility of digital content, compared with matched control classrooms.
- Evidence: Wineburg, Breakstone, McGrew, Smith and Ortega (2022, Journal of Educational Psychology 114(5), 893-909): six 50-minute lessons in a district-mandated government course, treatment n = 271, control n = 228, matched control design, multilevel linear mixed model. Students in treatment classrooms "grew significantly in their ability to judge the credibility of digital content". The pilot (n = 441 students, six teachers) found a significant pre to post gain and led the team "to teach fewer strategies more robustly".
- Caveats: The abstract and the supplement I read do not give the effect size, so the size of the gain is unconfirmed here. Matched control rather than fully randomized at student level (the SSRN abstract calls the analysis "cluster-randomized, repeated-measures"). The learners are teenagers judging civic content, and the transfer to an LLM applying the same moves is an inference.
- Grade: B.
- Sources: [Wineburg2022].

**1.3 SIFT gives the four lateral-reading moves in a form short enough to put in a prompt.**

- Finding: Caulfield's SIFT is: Stop; Investigate the source; Find better coverage; Trace claims, quotes and media to the original context.
- Evidence: Described in the "Cut the CRAAP" deck and the Educating for American Democracy research brief (search-result excerpts). I did not find a trial of SIFT as such that is separate from the lateral-reading trials above.
- Caveats: SIFT is a teaching mnemonic. Its evidence base is the lateral-reading research.
- Grade: D for SIFT as a named method, B for the underlying lateral-reading evidence.
- Sources: [CutTheCRAAP], [Wineburg2022].

**1.4 Rubrics built for open educational resources name the dimensions of teaching quality, and they are designed to be scored per object on a short scale.**

- Finding: The Achieve OER rubrics (licensed CC BY 3.0) score any stand-alone learning object on eight separate rubrics with a 0 to 3 scale plus N/A. MERLOT peer review uses three dimensions with 1 to 5 stars and two independent reviewers.
- Evidence (Achieve): The eight rubrics are I. Degree of Alignment to Standards (or Learning Objectives), II. Quality of Explanation of the Subject Matter, III. Utility of Materials Designed to Support Teaching, IV. Quality of Assessment, V. Quality of Technological Interactivity, VI. Quality of Instructional and Practice Exercises, VII. Opportunities for Deeper Learning, VIII. Assurance of Accessibility. Scale: 3 Superior, 2 Strong, 1 Limited, 0 Very Weak or None, N/A. For Rubric II, the search summary of the ISKME adaptation gives: 3 is comprehensive, connects related concepts and makes main ideas clear; 2 is understandable but lacks those connections; 1 is correct but cursory; 0 is confusing or contains errors. The rubrics are applied "to the smallest meaningful unit" and rate "the potential, not actual, effectiveness" of an object. Alignment (Rubric I) can only be scored after someone proposes which objective the object serves.
- Evidence (MERLOT): Three areas: Quality of Content, Potential Effectiveness as a Teaching Tool, Ease of Use. Each material goes to two peer reviewers who work independently; if the two reviews differ significantly a third reviewer is assigned. The review form (V 17.6) lists item-level checks. Under Quality of Content: clear and concise, complete demonstration of the concept, current and relevant, supported by appropriate research, self-contained, accurate. Under Potential Effectiveness: identifies learning objectives, identifies prerequisite knowledge, reinforces concepts progressively, builds on prior concepts, demonstrates relationships between concepts, is efficient ("could learn a lot in short time"). A review must average three stars to be posted. Reviewers must be higher-education instructors with expertise in the field.
- Caveats: Both rubrics rate potential effectiveness by expert judgment. Neither page I read reports inter-rater reliability or a link from rubric score to measured learning. EdReports (K-12 curriculum reviews by educator teams) was on the lead list and I did not fetch it, so its method stays unconfirmed in this file.
- Grade: C (deployed expert-review practice, no outcome validation found).
- Sources: [AchieveOER], [MERLOT], [MERLOTForm], [IATUL2003].

**1.5 A model's rating of a source's credibility agrees only moderately with human experts.**

- Finding: Models rate news domains in a way that agrees strongly with each other and only moderately with expert ratings, and the ratings shift with an assigned political persona.
- Evidence: Yang and Menczer audited nine LLMs from OpenAI, Google and Meta on 7,523 news domains with expert ratings. Agreement among models: average Spearman rho = 0.79. Agreement with human expert ratings: average rho = 0.50. Both are rank correlations computed only over sources a model agreed to rate (3,077 of the 7,523, or 40.9%, were rated by all nine models), so they do not cover the sources models declined to rate. Larger models more often refused to rate little-known sources, and smaller models made more errors. Default configurations showed a liberal lean, and assigning a partisan role biased ratings toward congruent sources.
- Caveats: Read as arXiv v3, which its arXiv record lists as published at the ACM Web Science Conference 2025 (an earlier version covered ChatGPT only; the ACM copy itself was not opened). The task is rating a domain from the model's memory, with no page content and no search. It covers news outlets, and technical teaching sites were not tested. High model-to-model agreement means that asking several models does not give independent opinions.
- Grade: B (audit with a stated method).
- Sources: [Yang2023].

**1.6 A model's judgment of relevance (is this page on topic for this need) can match trained human labellers, and it is sensitive to prompt wording.**

- Finding: With a prompt tuned against gold labels, LLM relevance labels were as accurate as human labellers at Bing. Agreement swings widely with the prompt.
- Evidence: Thomas, Spielman, Craswell and Mitra (arXiv 2309.10621, Microsoft): on TREC-Robust, Cohen's kappa against gold labels ranged from 0.20 to 0.64 across 32 prompt variants. For comparison they cite kappa 0.24 to 0.52 for crowd workers and 0.58 for lab workers (Damessie et al. 2017), and 0.52 between two groups of trained TREC assessors (Cormack et al. 1998). 6 of 32 prompts beat 0.58. "LLM performance varies with prompt features, but also varies unpredictably with simple paraphrases." Faggioli et al. (ICTIR 2023) ran a pilot: GPT-3.5 against TREC-8 assessors, kappa 0.38; on TREC-DL 2021 binarized labels, kappa 0.26 ("fair"). They argue for a spectrum of human-machine collaboration over full automation.
- Caveats: Relevance to a search topic is a narrower judgment than "on-topic for this milestone and pedagogically strong". Thomas et al. are Microsoft authors reporting on their own system. Faggioli et al. is a perspectives paper with a pilot experiment.
- Grade: B.
- Sources: [Thomas2024], [Faggioli2023].

**1.7 Evidence on a model judging teaching quality is thin and mixed: fair to good agreement on clarity and correctness, weak agreement on the pedagogical dimensions.**

- Finding: No study I found tests a model rating the teaching quality of web pages against expert OER reviewers. The nearest evidence is on rating explanations and hints with a rubric, where models agree with experts on surface criteria and diverge on pedagogy-specific ones.
- Evidence: (a) Logic-tutor hints (arXiv 2505.04736, since published in Computers and Education: Artificial Intelligence, 2025): two experts rated 210 LLM-written explanations on a 1 to 4 rubric (consistency, clarity, justification, subgoaling). The v1 preprint text said 240 while its own table caption said 210; the published version says 210 in both places. Expert-expert agreement was QWK 0.79 to 0.92. The paper's agreement table gives LLM-human QWK of 0.30 to 0.59 for consistency, clarity and justification and 0.16 for subgoaling, where the 0.16 did not reach significance. The LLM grader overestimated subgoaling (2.47 vs the humans' 1.66). (b) Math self-explanations (APSCE ICLEA paper): with self-consistency voting, gpt-4o-mini reached Cohen's kappa about 0.63 against experts whose own Fleiss kappa was 0.67, on only 12 explanations. (c) Teacher-written math explanations (NSF PAR 10470442, anonymized copy): four LLMs (GPT-4, GPT-3.5, Vicuna, PaLM 2) rated explanations on accuracy, clarity, notation and strategy. Exact-score kappa between LLM raters was mostly 0 to 0.2, while "relaxed" kappa (adjacent scores count as agreement) was above 0.67. GPT-4 "consistently favored explanations formatted with HTML", so presentation swayed the quality score. (d) Moore, Bier and Stamper (HCOMP 2024) compared 84 crowdworkers with GPT-4, Gemini 1.5 Pro and Claude 3 Opus applying two validated rubrics to 30 multiple-choice and short-answer questions in six domains: Prolific crowdworkers "consistently delivered the highest quality assessments", GPT-4 was the best LLM, and "LLMs can match this accuracy in specific evaluative criteria". They argue for a hybrid of model and human judgment.
- Caveats: All small studies, several of them preprints or workshop papers. None rates whole web pages. Model agreement with humans is highest when the human rubric is concrete and the scale is coarse. The HTML-formatting effect is a warning that a model's "pedagogically strong" verdict can track page polish, the same trap the CRAAP critique describes for people.
- Grade: B for the individual studies, with the application to web-page vetting unconfirmed.
- Sources: [Tithi2025], [SelfExplain2025], [MathExplain2024], [Moore2024].

### 2. Reliability of the open web as a source

**2.1 Wikipedia's accuracy on science topics was close to Britannica's in the one blinded expert comparison, and both contained errors.**

- Finding: Expert reviewers found about four inaccuracies per Wikipedia science entry and about three per Britannica entry.
- Evidence: Giles (2005, Nature 438, 900-901): 50 entry pairs sent to experts, 42 usable reviews, reviewers blind to the source. Factual errors, omissions or misleading statements: 162 in Wikipedia, 123 in Britannica. Serious errors (such as misreading an important concept): 8 in total, 4 in each. Several reviewers called the Wikipedia articles "poorly structured and confusing".
- Evidence (review): Mesgari et al. (2015, JASIST) systematically reviewed 110 peer-reviewed studies of Wikipedia content and found "little consensus on many aspects of Wikipedia's content"; the research clusters on quality (comprehensiveness, currency, readability, reliability) and size.
- Caveats: Giles was a news feature, not a peer-reviewed study, with 42 entries from 2005, science only. Britannica disputed the method. Readability and structure were the reviewers' main complaint about Wikipedia, which bears on teaching use: accurate is not the same as well explained.
- Grade: B (Giles: blinded expert comparison reported as a news feature). B (Mesgari: systematic review, abstract only).
- Sources: [Giles2005], [Mesgari2015].

**2.2 Stored URLs die at a steady rate: a quarter of pages sampled across 2013 to 2023 were gone by late 2023, 38% of the ones a full decade old, and deep links fare worse.**

- Finding: Link rot is large, grows with age, and hits exactly the kind of deep link a lesson plan stores.
- Evidence: Pew Research Center (2024): 25% of all pages sampled from 2013 to 2023 were inaccessible by October 2023 (a pooled figure over yearly snapshots aged zero to ten years, so it is lower than the ten-year loss); 38% of pages from 2013 were gone, against 8% of pages from 2023. 23% of news pages and 21% of government pages had at least one broken link. 54% of English Wikipedia pages had at least one dead link in References (the report itself gives the sample as 50,000 pages and says 11% of all reference links were dead). Pew counted only pages returning one of nine definitive error codes, and says changed content was "beyond the scope".
- Evidence: Zittrain, Bowers and Stanton (2021): 2,283,445 external links in 553,693 New York Times articles, 1996 to mid-2019. 72% were deep links. 25% of deep links were completely inaccessible: 6% of links from 2018, 43% from 2008, 72% from 1998. 53% of articles with deep links had at least one rotted link.
- Evidence: Klein et al. (2014, PLOS ONE): over one million web references from over 3.5 million STM articles, 1997 to 2012. For recent publication years, about one in five STM articles suffered from reference rot, and most articles that cite web-at-large resources were affected.
- Caveats: Pew's figure is a floor, because soft 404s, parked domains and paywall redirects return success codes. Rates differ by host type, and none of these studies isolates university course pages or official documentation.
- Grade: B.
- Sources: [Pew2024], [Zittrain2021], [Klein2014].

**2.3 A live URL often no longer says what it said when it was stored (content drift).**

- Finding: Content drift is as common as link rot or more so, and a status-code check cannot see it.
- Evidence: Zittrain et al. (2021): in a human-reviewed sample of 4,500 still-reachable links, 13% had drifted significantly from what the journalist cited; 4% for 2019 links, 25% for 2009 links. Jones, Van de Sompel et al. (2016, PLOS ONE): of 1,059,742 web references in STM articles, representative archived snapshots existed for about 30% (313,591). Comparing those snapshots with the live page, over 75% had drifted from the content originally referenced (the search summary gives 76.35%). For 14.33% of references no archived copy existed at all.
- Caveats: Drift thresholds are text-similarity measures, and some drift is harmless (a fixed typo, a new sidebar). Documentation pages are meant to drift as software versions change, which is its own problem for a lesson pinned to a version.
- Grade: B.
- Sources: [Zittrain2021], [Jones2016].

**2.4 Licences set what a lesson may copy. Creative Commons terms are simple to read off a page, and they differ on adaptation and commercial use.**

- Finding: Six CC licences, from most to least permissive: CC BY, CC BY-SA, CC BY-ND, CC BY-NC, CC BY-NC-SA, CC BY-NC-ND.
- Evidence (Creative Commons licence page): All six require attribution. BY and BY-SA allow commercial use and adaptation (SA requires the adapted material to be licensed under the same or a compatible licence). ND licences allow copying "in unadapted form only". NC licences allow use "for noncommercial purposes only", where noncommercial means "not primarily intended for or directed towards commercial advantage or monetary compensation". The page also says that much AI use of content "falls outside what our licenses can govern, because this use often sits outside what copyright ... was built to address", and points to separate guidance on CC licensing and AI.
- Caveats: This file does not contain legal advice and I did not fetch a legal source on quotation and fair use, so what short quotation or summary of an all-rights-reserved page permits stays unconfirmed here. The CC page itself is the licensor's own summary.
- Grade: C (authoritative primary description of the licences).
- Sources: [CCLicenses].

**2.5 Search results contain a growing share of machine-written and search-optimized pages, and research agents pick them unless told otherwise.**

- Finding: The measured share of AI-written content depends heavily on the detector and the sample, and the best academic study of search quality found that higher-ranked pages in a commercial niche were more monetized and of lower text quality.
- Evidence: Bevendorff, Wiegmann, Potthast and Stein (ECIR 2024, "Is Google Getting Worse? A Longitudinal Investigation of SEO Spam in Search Engines"): about 7,400 product-review queries tracked over a year across Google, Bing and DuckDuckGo. Per the Search Engine Land summary, quality was measured by vocabulary richness, readability, affiliate links, SEO signals and page structure, and the authors conclude that "search engines measurably target SEO and affiliate spam with their ranking updates" with "a noticeable, yet mostly short-lived, effect".
- Evidence (industry, secondary summaries only): Ahrefs (2025) is reported to have found that 74% of about 900,000 newly created pages contained AI-generated material (seen only on a marketing page that cites it). Originality.ai's tracker is reported to show AI content rising from 2.27% of sampled sites before 2019 to a peak of 19.56% in July 2025 (seen only on a secondary summary page). These two numbers measure different things and neither was read at the primary source.
- Evidence (deployed system): Anthropic's research-agent team reports that human testers "noticed that our early agents consistently chose SEO-optimized content farms over authoritative but less highly-ranked sources like academic PDFs or personal blogs. Adding source quality heuristics to our prompts helped resolve this issue."
- Caveats: Bevendorff covers "best X" product queries, a niche long targeted by affiliate spam, and the Search Engine Land author argues it does not generalize to search overall. AI-content detectors have their own error rates. No study I found measures the share of low-quality pages for programming-tutorial queries. On paywalls and login walls I found no rate for teaching material; the Tow Center audit (see 5.4) shows AI search tools citing syndicated copies and sometimes returning content from publishers that block their crawlers.
- Grade: B (Bevendorff, read via abstract and secondary summary), C (Anthropic practice report), D (industry figures via secondary pages, unconfirmed).
- Sources: [Bevendorff2024], [SELand2025], [AnthropicResearch2025], [Contentifai2025], [Maxintel2026].

### 3. Matching material to the learner's level

**3.1 Readability formulas measure word and sentence length. They explain a minority of the variance in expert judgments of difficulty and ignore cohesion and prior knowledge.**

- Finding: Classic formulas (Flesch Reading Ease, Flesch-Kincaid, Dale-Chall) use proxies: characters or syllables per word for vocabulary, words per sentence for syntax. They were normed on small, old, school-age corpora.
- Evidence: Crossley et al. (published online March 2022, Behavior Research Methods, the CLEAR corpus paper): traditional formulas "often lack strong construct validity", omit text cohesion, and "have been normed using readers from specific age groups and using small corpora of texts from specific domains". Flesch Reading Ease and Dale-Chall were normed on 350 texts written in the 1920s for grades 3 to 12. The search summary of the same paper reports that formulas explained about 23 to 34% of the variance in teacher-based readability scores on CLEAR (4,724 excerpts).
- Caveats: I did not find a Lexile-specific validation in this pass, so claims about Lexile stay unconfirmed. All of this is for general prose and school-age norms. Technical text for adult engineers (code, jargon, API names) is outside what these formulas were built on.
- Grade: B.
- Sources: [Crossley2022].

**3.2 A model's zero-shot difficulty rating tracks human judgments better than the formulas, with a large unexplained remainder.**

- Finding: GPT-4 class models asked to rate readability from 1 to 100 correlate with teacher judgments at about r = 0.75.
- Evidence: "Measuring and Modifying the Readability of English Texts with GPT-4" (TSAR 2024 workshop proceedings, pages 126-134): on the CLEAR corpus (4,724 excerpts of 140 to 200 words), GPT-4 Turbo ratings correlated r = 0.76 with human readability scores and GPT-4o mini r = 0.74 (R squared about 0.58 and 0.54). The correlation between two random splits of the human raters was r = 0.63. A linear model with word frequency, concreteness and age of acquisition explained about 36% of variance. The two models correlated r = 0.81 with each other.
- Caveats: The authors call it a proof of concept and note "considerable variance in human judgments remains unexplained" and that readability depends on audience, purpose and context. The rating is of general ease, with no model of a specific reader's prior knowledge. The authors are Sean Trott and Pamela D. Rivière (confirmed on the ACL Anthology PDF in the citation check).
- Grade: B.
- Sources: [TSAR2024].

**3.3 Prior knowledge changes which text helps: low-knowledge readers learn more from a fully explicit, cohesive text, and high-knowledge readers can learn as much or more from a text that leaves gaps.**

- Finding: The "best" explanation is relative to the reader. This is the text-level form of the expertise reversal effect that the vision document raises in Q11.
- Evidence: McNamara, E. Kintsch, Songer and W. Kintsch (1996, Cognition and Instruction 14(1), 1-43), Experiment 2: four versions of a biology text crossing local and global coherence, readers split by domain knowledge. Proportion correct on post-reading questions, averaged over question types: high-knowledge readers scored .55 on the maximally coherent text (CM) and .61 on the minimally coherent text (cm), with .41 and .40 on the mixed versions; low-knowledge readers scored .41 on CM and .27 on cm. On problem-solving questions, high-knowledge readers scored .46 (CM) vs .61 (cm), and low-knowledge readers .30 (CM) vs .17 (cm). The benefit of the gappy text for high-knowledge readers showed on inference and problem-solving measures, not on recall.
- Evidence (boundary conditions): McNamara and Kintsch (1996, Discourse Processes) found a high-coherence advantage on recall and multiple choice, a low-coherence advantage for high-knowledge readers on a sorting task in Experiment 1, and that sorting result "failed to replicate" in Experiment 2, where high-knowledge readers still did better on open-ended questions after the low-coherence text. O'Reilly and McNamara (2007, Discourse Processes, n = 143 college students) found "the benefit of low-cohesion text was restricted to less skilled, high-knowledge readers, whereas skilled comprehenders with high knowledge benefited from a high-cohesion text".
- Caveats: Small samples (the Experiment 2 F tests have 48 error degrees of freedom), junior high students, school science texts, and the reverse effect is fragile. The stable part is the first half: low-knowledge readers are hurt by gaps. For adult engineers the practical reading is that an explicit, cohesive source is the safe default and a terse reference page is suitable only once the learner has shown the prerequisite knowledge.
- Grade: B (controlled experiments, with a partial failure to replicate recorded).
- Sources: [McNamara1996], [McNamaraKintsch1996], [OReilly2007].

### 4. Extracting prerequisite structure from web material

**4.1 People agree with each other only moderately on which concept is a prerequisite of which. Agreement runs from kappa 0.19 to 0.74 across studies, and most sit between 0.3 and 0.55.**

- Finding: Prerequisite links are partly a matter of judgment, and annotators with less expertise mark more prerequisites.
- Evidence: Talukdar and Cohen (2012): crowd workers labelling Wikipedia page pairs in five domains, average kappa 0.54 (range 0.47 to 0.63). Liang, Ye, Wu, Pursel and Giles (AAAI 2017): 13 CS graduate students, three labels per pair, 3,544 candidate concept pairs, Fleiss kappa 0.42; scored against the majority vote, individual annotators had precision 0.75 (SD 0.13) and F-score 0.75 (SD 0.08). Alzetta et al. (PRET, CLiC-it): four annotators on a textbook chapter, Fleiss kappa 38.5%; 55.7% of the pairs marked as prerequisites were marked by only one of four annotators; pairwise Cohen's kappa ranged from 25% to 58%. The same paper summarises other work: Gordon et al. (2016) Pearson correlation 36%, Fabbri et al. (2018) Fleiss kappa 30%, Chaplot et al. (2016) 19%. A higher figure comes from UPreG (EDM 2021): 25 subject-matter experts in groups of five on 250 pairs, Fleiss kappa 0.74, with the system's extracted pairs judged 82% accurate.
- Evidence: PRET notes that "when the competence in the domain is lower, a person tends to consider a higher number of prerequisites for each concept", and that removing cycles and adding transitive links raised agreement slightly (38.50% to 39.94%).
- Caveats: Studies differ in domain, annotator expertise, and whether "prerequisite" means strictly required or merely helpful. The high UPreG figure was for judging pairs a system proposed, an easier task than free annotation.
- Grade: B.
- Sources: [Talukdar2012], [Liang2017], [PRET2018], [UPreG2021].

**4.2 Pre-LLM methods mined prerequisite links from Wikipedia links, course catalogues and MOOC transcripts, with accuracy modestly above chance to moderately good.**

- Finding: Three signal sources recur: how pages link to each other, course prerequisite listings, and the order and distribution of concepts in lectures.
- Evidence: Talukdar and Cohen (2012): a classifier using Wikipedia hyperlinks, edit histories and page content, trained on other domains, reached 58.82% average accuracy against a 50.22% random baseline. Liang, Wu, Huang and Giles (EMNLP 2015) proposed RefD, "a simple link-based metric" that "models the relation by measuring how differently two concepts refer to each other" (description from Semantic Scholar; the accuracy numbers were not on a page I read). Liang et al. (AAAI 2017) recovered concept prerequisites from 654 CS courses and 639 course prerequisite pairs scraped from 11 US university catalogues; RefD, which uses the whole Wikipedia link graph, still beat the course-only method on precision at 100. Pan, Li, Li and Tang (ACL 2017) learned prerequisite relations among MOOC concepts on three Coursera datasets, improving F1 by 5.9 to 48.0% over earlier methods. Gordon et al. (ACL 2016) built concept graphs from a scientific corpus with an information-theoretic dependency measure.
- Caveats: For RefD and Gordon I read only abstract-level descriptions. These methods need a fixed concept list and a corpus, and their accuracy was judged against the noisy human labels described in 4.1.
- Grade: B.
- Sources: [Talukdar2012], [Liang2015], [Liang2017], [Pan2017], [Gordon2016].

**4.3 Current models predict prerequisite links at roughly 75% accuracy zero-shot, about the level at which individual human annotators agree with a consensus.**

- Finding: A model alone is a usable first-pass prerequisite judge and falls well short of a reliable one.
- Evidence: Yang, Yang, Ouyang, She, Feng, Jiang, Lecue, Lu and Li (arXiv 2402.14293, preprint, LectureBankCD with 322 NLP, 201 CV and 100 bioinformatics concepts): zero-shot GPT-4 reached 74.59% accuracy and 77.71 F1 overall, above the best supervised baseline (Node2vec, 74.54% accuracy). Adding retrieval did not consistently help (GPT-4 with RAG 73.86%), which the authors suggest may reflect the model having already seen the data. A 2025 study (arXiv 2504.08856, preprint) had GPT models judge 100 concept pairs and notes that "the prerequisite relationships between many concept pairs are inherently subtle and challenging to identify, even for human evaluators". A zero-shot benchmark on 3,196 ESCO skills (arXiv 2507.18479, preprint) found models' predicted prerequisites semantically close to expert-defined ones (BERTScore F1 about 0.81 to 0.83), a similarity measure and not an accuracy.
- Caveats: All preprints. Benchmarks are balanced pair-classification tasks, which is easier than building a whole graph where true links are rare (1.54% of pairs in PRET). Models may have memorized public datasets.
- Grade: B.
- Sources: [Yang2024Graph], [GPTConcepts2025], [ESCOPrereq2025].

**4.4 What this says about the "2 or more independent sources must agree" rule.**

- Finding: The evidence supports requiring corroboration and warns that agreement between two sources is weaker proof than it looks.
- Evidence: Human annotators reach only moderate agreement (4.1), so a single source's ordering is one opinion. Majority vote over three annotators is the standard way these studies build ground truth (Liang 2017, Talukdar 2012). University course catalogues are a real, scrapeable prerequisite signal (Liang 2017). Model ratings of sources agree with each other far more than with experts (rho 0.79 vs 0.50 in Yang and Menczer), so two model-derived judgments are not independent. I found no study that tests a "two sources agree" rule directly.
- Caveats: Web sources copy one another (syllabi reuse textbook orderings, blog posts paraphrase each other), so independence has to be judged, and no source I read gives a method for that.
- Grade: D for the rule itself (inference from B-grade evidence on agreement).
- Sources: [Liang2017], [Talukdar2012], [PRET2018], [Yang2023].

### 5. Grounded generation and citation faithfulness

**5.1 In generative search engines, about half of generated sentences are fully supported by the cited source, and about a quarter of citations do not support their sentence.**

- Finding: Citations in AI search answers look trustworthy and often fail a human check. The answers people rate as most helpful have the least accurate citations.
- Evidence: Liu, Zhang and Liang (2023; arXiv 2304.09848, Stanford): human audit (34 annotators) of Bing Chat, NeevaAI, perplexity.ai and YouChat. On average 51.5% of generated sentences were fully supported by their citations (citation recall) and 74.5% of citations supported their sentence (citation precision). Citation precision was inversely correlated with perceived utility (r = -0.96). Recall was 58.5 on NaturalQuestions queries with a long answer vs 47.8 on other queries, and 44.3 on open-ended essay questions that have no extractive answer on the web. The authors' explanation: systems that copy or closely paraphrase are precise and feel less useful.
- Evidence: Gao, Yen, Yu and Chen (2023; arXiv 2305.14627, Princeton; ALCE benchmark): "on the ELI5 dataset, around 50% generations of our ChatGPT and GPT-4 baselines are not fully supported by the cited passages". A closed-book model with citations added afterward had good correctness and "much worse citation quality". Sampling several answers and keeping the one with the best automatic citation score (rerank) consistently improved citation quality. More retrieved passages did not help ChatGPT much: correctness plateaued at the top 1 passage and citation quality at the top 3.
- Caveats: The 2023 systems are several model generations old. Sentence-level scoring understates errors inside multi-claim sentences. Later audits below show the problem persists in different forms.
- Grade: B.
- Sources: [Liu2023], [Gao2023].

**5.2 Later audits (2024 to 2025) still find that a large share of cited claims are not supported by the cited page, including with retrieval.**

- Finding: Retrieval reduces fabricated sources and does not make statements faithful to them.
- Evidence: Wu, Wu, Casasola, Zhang, Wei, Nguyen, Riantawan, Shi, Ho and Zou (SourceCheckup, arXiv 2402.02008, preprint as read; Stanford): 1,200 medical questions, over 40,000 statement-source pairs, five commercial LLMs. "Between about 50% to 90% of LLM responses are not fully supported by the sources they provide." GPT-4 with retrieval failed to produce any source in over 20% of responses; its response-level support was close to 80% for MayoClinic-style questions and about 30% for open-ended Reddit questions. The automated verifier (GPT-4) agreed with a doctor panel 88% of the time, against 85.2% agreement among doctors. The paper has since been published with a larger study (Nature Communications, 2025): seven LLMs, 800 questions and about 58,000 statement-source pairs, the same 50% to 90% range, verifier agreement of 88.7% on 400 pairs against 86.1% between doctors, and 55% of responses fully supported for the best system (GPT-4o with web search).
- Evidence: DeepResearch Bench (arXiv 2506.11763, preprint; 100 PhD-level tasks in 22 fields): citation accuracy was 90.24% for Perplexity Deep Research, 81.44% for Gemini 2.5 Pro Deep Research, 77.96% for OpenAI Deep Research and 83.59% for Grok Deeper Search, judged by an LLM. The score is the share of extracted statement-URL pairs the judge marked as supported, averaged per task, with a task that yields no citable statement counted as zero. Gemini averaged 111.21 effective citations per task and OpenAI 40.79. So in the best deep-research products of early 2025, roughly 1 in 10 to 1 in 5 cited statements was not supported by its source.
- Evidence: Tow Center (Jaźwińska and Chandrasekar, March 2025): 8 AI search tools, 1,600 queries built from excerpts of 200 news articles (the excerpt returned the source in the top three Google results). The tools gave incorrect answers to more than 60% of queries: 37% for Perplexity, 94% for Grok 3. ChatGPT got 134 of 200 wrong and signalled low confidence 15 times. More than half of Gemini and Grok 3 responses cited fabricated or broken URLs; 154 of 200 Grok 3 citations led to error pages. Paid tiers answered more questions correctly and also gave more confidently wrong answers.
- Evidence: BBC and EBU (October 2025; 22 public broadcasters, 18 countries, 14 languages; 2,709 evaluated responses to 30 core news questions from free ChatGPT, Copilot, Gemini and Perplexity, generated late May to early June 2025): 45% of responses had at least one significant issue; 31% had a significant sourcing issue (unsupported by the cited source, no source, or wrong attribution); 20% had a significant accuracy issue. Gemini had significant sourcing issues in 72% of responses, the others below 25%. Of 1,053 responses containing a direct quote, 12% had a significant problem with the quote. In the BBC-only comparison with the earlier BBC study, significant issues fell from 51% to 37%.
- Caveats: Tow and BBC/EBU test news, where publishers block crawlers and content is fresh, so the rates are likely worse than for stable technical documentation. DeepResearch Bench uses an LLM judge. SourceCheckup is medical. None tests teaching content.
- Grade: B.
- Sources: [Wu2024], [DRBench2025], [Tow2025], [BBCEBU2025].

**5.3 Without retrieval, models invent references at high rates. Newer models invent fewer and still get details wrong.**

- Finding: A reference list written from a model's memory cannot be trusted without checking each entry.
- Evidence: Walters and Wilder (2023, Scientific Reports): 84 short literature reviews on 42 topics, 636 citations. 55% of GPT-3.5 citations and 18% of GPT-4 citations were fabricated. Among the real ones, 43% (GPT-3.5) and 24% (GPT-4) had substantive errors. Their review of six earlier studies found 51% of 732 ChatGPT citations fabricated.
- Evidence: Chelli et al. (2024, JMIR 26:e53164): asked to reproduce the reference lists of 11 systematic reviews, hallucination rates were 39.6% (55/139) for GPT-3.5, 28.6% (34/119) for GPT-4 and 91.4% (95/104) for Bard. Precision was 9.4%, 13.4% and 0%. Recall was 11.9% and 13.7% for the GPT models.
- Caveats: 2023-era models without search. The Chelli recall figures matter for a lesson-plan builder: even when references were real, the models found about one in eight of the papers a human review found.
- Grade: B.
- Sources: [Walters2023], [Chelli2024].

**5.4 When a model summarizes a document it was handed, it still adds unsupported content in a few percent to over ten percent of summaries, and more on hard cases.**

- Finding: A reader step that compresses a fetched page into notes has its own error rate, separate from the page's accuracy.
- Evidence: Vectara's hallucination leaderboard (vendor benchmark; a detector model, HHEM, scores each summary): on the new dataset of over 7,700 articles (November 2025), the best model was at a 3.3% hallucination rate, and several frontier reasoning models (Claude Sonnet 4.5, GPT-5, Grok-4, DeepSeek-R1) were above 10%; Gemini 3 Pro was 13.6%. Vectara notes the new rates "are much higher than before" because the articles are longer and harder.
- Evidence: FaithBench (arXiv 2410.13210; a NAACL 2025 paper per the project's GitHub page): 660 human-annotated summaries of 66 passages by 10 models, deliberately sampled from cases where detectors disagree. Rates of summaries with unwanted hallucination ran from 39.34% (GPT-4o) to 73.77% (Command-R). Annotators were 11 people, most with hallucination-detection expertise.
- Caveats: The Vectara rate is produced by Vectara's own classifier, which has unknown error. FaithBench is a hard-case sample, so its rates are an upper range and do not describe typical documents. Reasoning models scoring worse than small models is Vectara's own observation and has no independent test that I read.
- Grade: B (FaithBench), C (Vectara leaderboard).
- Sources: [Vectara2025], [FaithBench2025].

**5.5 In education, a purpose-built assistant answering from instructor-approved material passed about 77% of test questions, against about 31% for a generic file-retrieval assistant on the same model, with fewer harmful answers, and about a quarter of its answers still failed.**

- Finding: Retrieval over vetted courseware is the best-documented design for a teaching assistant, and its residual error is large enough to need a fallback ("I don't know") and spot checks.
- Evidence: Taneja, Maiti, Kakar, Guruprasad, Rao and Goel (Jill Watson, arXiv 2405.11070; Georgia Tech): 150 questions written by four students from a syllabus, e-book and video transcripts, including deliberately unanswerable ones. Pass rates: Jill Watson (ChatGPT plus dense retrieval, grounded prompting and an "I don't know" path) 76.7%; OpenAI Assistants (file upload plus retrieval) on the same underlying model 31.3% (the paper implies, and does not state, that the uploaded files were the same); the legacy rule-based Jill Watson 26.0%. Of the failures, 5.7% were harmful (misleading) for Jill Watson vs 16.5% for OpenAI Assistants; 57.1% vs 68.0% were retrieval failures. Inter-rater kappa 0.76. A later project page reports accuracy by source type: "for sources such as syllabi, Jill gets about 75-80% questions right, while for textbooks it typically gets more than 90%". A companion paper's abstract reports deployment in classes with close to 1,300 students at Georgia Tech and in two community colleges [Kakar2024].
- Evidence (practice): Khan Academy reports that Khanmigo uses a separate "math agent" that "verifies calculations and checks mathematical expressions in real time", and tracks math error rate and answer give-away as guardrail metrics across more than 15 million tutoring threads. Restricting that agent to the student's work so far cut answer give-aways by 50%.
- Caveats: Jill Watson's test is 150 synthetic questions from one course, graded by the team. The comparison shows the retrieval design matters as much as retrieval itself (same model, 31% vs 77%). Khan's post gives no error rate for the math agent.
- Grade: B (Jill Watson), C (Khan Academy).
- Sources: [JillWatson2024], [AIALOE2024], [Khan2026].

**5.6 STORM shows what a source-handling pipeline for long grounded writing looks like, and what it still gets wrong.**

- Finding: STORM (Stanford) researches a topic by simulated multi-perspective question asking, filters sources by rule, and writes with citations. Experienced Wikipedia editors found it better organized than a retrieval baseline and flagged a new error type: unsupported connections between true facts.
- Evidence: The STORM paper (arXiv 2402.14207 v2, April 2024; Stanford; Shao, Jiang, Kanell, Xu, Khattab and Lam, NAACL 2024 per the arXiv record): search results are "evaluated using a rule-based filter according to the Wikipedia guideline" on reliable sources. An LLM judge found 84.83% of sentences supported by their citations, so about 15% unsupported. Manual analysis of unsupported sentences found the main causes were "improper inferences and inaccurate paraphrasing, rather than hallucinating non-existent contents". Ten editors (each with 500 or more edits) rated articles; more STORM articles were judged organized (+25% absolute) and broad (+10%). Where editors preferred the baseline, they gave STORM lower verifiability in over 50% of those cases, citing "red herring fallacy or overspeculation". Editors also saw bias from internet sources show up in the generated articles.
- Caveats: The paper "does not optimize generating text with citations". The 84.83% figure is from a small LLM judge (Mistral 7B). FreshWiki topics are recent news-like topics.
- Grade: B.
- Sources: [Shao2024].

### 6. Checking facts in generated teaching content

**6.1 Breaking text into atomic claims and checking each against a source gives a usable factual-precision score, and automated checkers now match or beat crowd annotators.**

- Finding: Decompose-then-verify is the standard method for scoring long generated text, and it exposes how error-prone unchecked generation is.
- Evidence: Min et al. (FActScore, 2023; arXiv 2305.14251): biographies were split into atomic facts (26 to 41 per generation) and each checked against Wikipedia by human annotators. Share of supported facts: InstructGPT 42%, ChatGPT 58%, search-augmented PerplexityAI 71%. For ChatGPT, the score fell from 80% for frequent entities to 16% for the rarest. Human checking cost about $4 per generation. Their automated estimator (retrieval plus a language model) came within 2% of the human score.
- Evidence: Wei et al. (SAFE, 2024; arXiv 2403.18802, Google DeepMind): an LLM agent splits a response into individual facts, issues Google Search queries for each, and judges support. On about 16,000 facts from the FActScore data, SAFE agreed with crowd annotators 72% of the time. On 100 sampled disagreements re-labelled by researchers with full web access, SAFE was right in 76% and the human annotators in 19%. Cost: $0.19 per response vs $4.00 for human annotation, more than 20 times cheaper.
- Caveats: FActScore measures precision only. The authors state it "does not consider factual recall" and "does not penalize a model that abstains". Wanner et al. ("A Closer Look at Claim Decomposition", arXiv 2403.11903) show the score is "sensitive to the decomposition method used". Jiang et al. (Core, arXiv 2407.03572) show the score can be inflated by adding trivial or repeated true claims: on biographies, an uninformative-text generator scored 83.0% on raw FActScore against 54.0% for the normal model, and 36.2% vs 49.6% once only unique, informative claims counted. VeriScore (arXiv 2406.19276) points out that these metrics assume every claim is verifiable. SAFE's ground truth for the 100 disagreements was set by the paper's own authors. Author names for the decomposition and Core papers were confirmed on arXiv in the citation check.
- Grade: B.
- Sources: [Min2023], [Wei2024], [Wanner2024], [Core2024], [VeriScore2024].

**6.2 Having the model draft, then write verification questions, then answer them without seeing the draft, cuts hallucinated content substantially, with no external source needed.**

- Finding: Chain-of-Verification (CoVe) works best when the verification answers are produced in separate prompts that do not include the draft.
- Evidence: Dhuliawala et al. (2023; arXiv 2309.11495): with Llama 65B, on Wikidata list questions precision rose from 0.17 to 0.36 and the average number of hallucinated entities per answer fell from 2.95 to 0.68, while correct entities fell from 0.59 to 0.38. On long biographies FActScore rose from 55.9 to 71.4 (a 28% gain) and the number of facts fell from 16.6 to 12.3. The factored variant beat the joint variant (63.7 vs 60.8), and adding an explicit cross-check step ("factor+revise") took it from 63.7 to 71.4. On MultiSpanQA, F1 rose from 0.39 to 0.48.
- Caveats: Tested on an older open model with few-shot prompts. The gain is partly from saying less (fewer facts). The model checks itself from memory, so facts it does not know stay wrong. The paper notes that a retrieval-based system still did better on very rare facts.
- Grade: B.
- Sources: [Dhuliawala2023].

**6.3 Asking the same question several times and flagging claims that are not stable across samples detects many hallucinations without any source.**

- Finding: Sampling consistency is a cheap screen for unsupported claims.
- Evidence: Manakul et al. (SelfCheckGPT, 2023; arXiv 2303.08896): on GPT-3 biographies with manual labels, the prompt-based variant reached 93.42 AUC-PR for detecting non-factual sentences (random baseline 72.96) and 0.78 Pearson correlation with human passage-level factuality. The search summary notes the method samples 20 responses. A 2026 preprint (CQC-RAG, arXiv 2606.13438) applies the same idea across rewritten queries and reports gains of 4.76 and 9.12 exact-match points over a multi-query baseline on TriviaQA and MuSiQue, attributing them to consistency checking and not to wider retrieval.
- Caveats: A model can be consistently wrong. The base rate of non-factual sentences in the SelfCheckGPT data was high (random AUC-PR about 73), which flatters the metric. Author names for SelfCheckGPT were confirmed on arXiv in the citation check.
- Grade: B.
- Sources: [SelfCheck2023], [CQCRAG2026].

**6.4 Requiring a verbatim supporting quote makes a claim checkable at a glance and removes fabricated pointers. It does not prove the quote supports the claim.**

- Finding: Quote-level grounding splits verification into a mechanical part (is the quote really in the source) and a judgment part (does the quote support the claim). The first can be enforced by code.
- Evidence: Menick et al. (GopherCite, 2022; arXiv 2203.11147, DeepMind): the model answers with an inline verbatim quote from a retrieved document, with decoding constrained so quotes are exact. Human raters found responses high quality 80% of the time on a NaturalQuestions subset and 67% on an ELI5 subset. The paper states the two-part split directly: "special syntax that can be automatically parsed to ensure that a quote is verbatim from a source, and human preferences to determine whether the quote supports the claimed answer". The model can abstain when unsure. The paper also says it does "not explicitly mitigate untrustworthy sources".
- Evidence (vendor practice, secondary summaries): Anthropic's guidance, as summarised on two practitioner pages, is to let the model say "I don't know", to have it extract word-for-word quotes first for documents over about 20,000 tokens, and after drafting to "find a supporting quote for each claim and to retract any claim it cannot support". Anthropic's Citations API extracts the cited text itself, so citations are "guaranteed to contain valid pointers to the documents you provided". One summary adds the limit in plain words: "That removes fabricated references, not misread ones." Vendor-reported figures at launch: up to 15% better recall accuracy than custom implementations, and a customer (Endex) reporting source hallucinations falling from 10% to 0%.
- Evidence (audit): Even quoted text goes wrong in the wild. BBC and EBU found 12% of AI-assistant responses containing a direct quote had a significant problem with that quote (altered or misattributed).
- Evidence: Weller et al. ("According to..." prompting, arXiv 2305.13252): telling a model to ground its answer in a named corpus increases the share of its output that appears verbatim in that corpus (their QUIP-Score) and often improves task accuracy. This is quoting from memory, not from a supplied page.
- Caveats: I read the Anthropic material only through secondary summaries, so the exact wording of the vendor docs is unconfirmed. The Endex number is a customer testimonial. GopherCite is a 2022 system.
- Grade: B (GopherCite), C (vendor practice), B (BBC/EBU).
- Sources: [Menick2022], [Csorba2026], [Botnet2026], [BBCEBU2025], [Weller2023].

**6.5 A model can check whether a source supports a statement about as well as expert humans agree with each other.**

- Finding: The "does this source support this sentence" judgment can be delegated to a model as a first pass.
- Evidence: SourceCheckup: GPT-4 agreed with a doctor panel on 88.0% of 284 statement-source pairs, vs 85.2% average agreement between doctors. On 110 pairs it flagged as unsupported, doctors agreed 95.8% of the time. SAFE: 72% raw agreement with crowd labels and right in 76% of disagreements. Anthropic's research team reports that a single LLM judge scoring factual accuracy, citation accuracy, completeness, source quality and tool efficiency "was the most consistent and aligned with human judgements".
- Caveats: FaithBench was built from the cases where automated detectors disagree and reports that such hard cases remain common. Model verifiers share blind spots with model writers.
- Grade: B.
- Sources: [Wu2024], [Wei2024], [AnthropicResearch2025], [FaithBench2025].

### 7. Prompt injection from fetched pages

**7.1 Any page a model reads can contain instructions, and models follow them. This was shown on deployed products in 2023 and is still ranked the top risk.**

- Finding: Indirect prompt injection means an attacker plants text where the model will retrieve it. The model cannot reliably tell data from instructions.
- Evidence: Greshake et al. (2023; arXiv 2302.12173): introduced indirect prompt injection against LLM-integrated applications, with a taxonomy of impacts "including data theft, worming, information ecosystem contamination", and argued that these applications "blur the line between data and instructions". OWASP LLM01:2025 lists prompt injection first, notes injected text need not be visible to humans, and says "it is unclear if there are fool-proof methods of prevention". Its listed impacts include "content manipulation leading to incorrect or biased outputs". Willison (2025) lists more than a dozen production systems exploited this way from 2023 to 2025.
- Evidence (benchmarks): InjecAgent (Zhan et al., 2024; arXiv 2403.02691): 1,054 test cases, 17 user tools, 62 attacker tools, 30 agents. ReAct-prompted GPT-4 was attacked successfully 24% of the time, and 47% with a reinforcing "hacking prompt"; fine-tuned GPT-4 7.1%; Llama2-70B over 80%. BIPIA (Yi et al., 2023; arXiv 2312.14197): all tested models were vulnerable, more capable models were more vulnerable to text attacks, and summarization had a higher attack success rate than question answering, which the authors link to the prompt template having no user instruction after the external content. AgentDojo (Debenedetti et al., 2024; arXiv 2406.13352): 97 tasks and 629 security cases; attacks succeeded against the best agents in under 25% of cases with no defense.
- Evidence (public competition, paper dated March 2026): Gray Swan's indirect prompt injection competition (arXiv 2603.15714, preprint; run with the UK AI Security Institute, US CAISI and frontier labs; the paper does not give the dates it ran): 464 participants, 271,588 deduplicated attempts against 13 frontier models in tool-calling, coding and computer-use settings, 8,648 successful attacks across 41 scenarios. "All models proved vulnerable", from 0.5% attack success (Claude Opus 4.5) to 8.5% (Gemini 2.5 Pro). Universal attack strategies transferred across 21 of 41 behaviors. Success here required both the harmful action and concealing it from the user. The authors call the result "an upper bound on single-turn agent vulnerability rather than real-world attack frequencies".
- Caveats: Benchmarks with fixed attacks understate adaptive attackers (7.3). Per-attempt rates are low for the best models, and an attacker who plants text on a public page gets as many attempts as there are readers.
- Grade: B.
- Sources: [Greshake2023], [OWASP2025], [Willison2025], [InjecAgent2024], [BIPIA2023], [AgentDojo2024], [GraySwan2026].

**7.2 Marking, delimiting and warning (prompt-level defenses) cut attack success a lot against fixed attacks.**

- Finding: Telling the model that fetched text is data, and transforming that text so its origin is visible, lowers attack success from tens of percent to a few percent in static tests.
- Evidence: Hines et al. (Spotlighting, 2024; arXiv 2403.14720, Microsoft): three variants. Delimiting (wrap the text in special tokens) cut attack success by about half on GPT-3.5-Turbo and "could be easily subverted by an attacker who gains knowledge of our system prompt". Datamarking (interleave a marker character through the whole text) cut it from about 50% to 3.10% on GPT-3.5-Turbo and from 40% to 0.00% on text-davinci-003. Encoding (base64, ROT13) is a further variant for capable models. Defensive instructions alone had "only a negligible impact". BIPIA proposed similar black-box defenses (boundary awareness, explicit reminder) that reduced and did not eliminate attacks.
- Evidence: AgentDojo, with GPT-4o against its strongest fixed attack: a prompt-injection detector brought attack success to 8% and "has too many false positives ... and significantly degrades utility". A "tool filter" (the model picks the tools it needs before seeing any untrusted data) brought it to 7.5%. Repeating the user prompt after each tool call helped and "is unlikely to withstand adaptive attacks".
- Caveats: See 7.3. These numbers are for attacks written before the defense was known.
- Grade: B.
- Sources: [Hines2024], [BIPIA2023], [AgentDojo2024].

**7.3 Filter-style and prompt-style defenses fail against attackers who adapt. Twelve published defenses, most of which had reported near-zero attack success, were broken at above 90% for most.**

- Finding: A defense that works on a fixed test set gives no assurance against someone who studies it.
- Evidence: Nasr, Carlini, Sitawarin, Schulhoff, Hayes, Tramèr and colleagues from OpenAI, Anthropic, Google DeepMind, ETH Zürich and others ("The Attacker Moves Second", arXiv 2510.09023, preprint, 2025): adaptive attacks (gradient-based, reinforcement learning, random search and human red-teaming) bypassed 12 recent defenses "with attack success rate above 90% for most; importantly, the majority of defenses originally reported near-zero attack success rates". Spotlighting and prompt sandwiching, at about 1% attack success under AgentDojo's static attacks, went above 95% under search-based adaptive attack (the paper describes the Spotlighting it tested as marking text with delimiter tokens, so this is not shown to be a test of the datamarking variant in 7.2); human participants produced 265 successful attacks on spotlighting and 178 on prompt sandwiching. Detectors (Protect AI, PromptGuard, Model Armor) were bypassed over 90% of the time, PIGuard 71%. In a $20,000 red-teaming competition, humans succeeded in 100% of the scenarios tested where the automated search succeeded in 69%.
- Evidence (vendor figures): Anthropic (November 2025): against an internal adaptive attacker given 100 attempts per environment, its browser agent's attack success rate was about 1% with new safeguards on Claude Opus 4.5, and "a 1% attack success rate ... still represents meaningful risk. No browser agent is immune to prompt injection". A third-party summary of the chart gives 1.4% for Opus 4.5 with safeguards, 7.6% with the earlier safeguards and 8.0% for the model alone; and for the August 2025 research preview, 23.6% without mitigations and 11.2% with them on 123 fixed test cases. Willison on vendor guardrail products: "in web application security 95% is very much a failing grade".
- Evidence (vendor statements, secondary summaries only): OpenAI is quoted as saying prompt injection "is unlikely to ever be fully 'solved'" (December 2025). Microsoft is quoted describing spotlighting as "a probabilistic technique" and advising to "design systems with the expectation that some attacks will succeed", and (October 2026) "we should not ask a model to be its own security authority". Google's defenses are described as including "Markdown sanitization and suspicious URL redaction". A 2026 edition of the OWASP LLM Top 10 is reported to still rank prompt injection first. I did not read these vendor pages directly.
- Caveats: The adaptive-attack paper is a preprint by authors who include staff of the three largest model vendors. Anthropic's numbers are internal, graded by its own model, with no independent replication. The vendor quotations in the last bullet come from two practitioner blogs.
- Grade: B (Nasr et al.), C (vendor reports), D (vendor quotes via secondary pages, unconfirmed).
- Sources: [Nasr2025], [AnthropicPI2025], [Omnipedia2025], [Willison2025], [Primores2026], [ZNova2026].

**7.4 Designs that limit what a compromised reader can do hold up where filters fail. The common principle: after a model has read untrusted text, nothing it produces may trigger a consequential action.**

- Finding: The secure designs separate a model that reads untrusted content and has no tools from a model that has tools and never sees untrusted content, with ordinary code between them.
- Evidence (dual LLM): Willison (2023): a Privileged LLM takes trusted input and uses tools; a Quarantined LLM reads untrusted content, "does not have access to tools, and is expected to have the potential to go rogue at any moment"; a Controller (ordinary code) passes the quarantined output around as an opaque variable such as $VAR2. "It is absolutely crucial that unfiltered content output by the Quarantined LLM is never forwarded on to the Privileged LLM." The one exception: output that can be verified, "like classifying text into a fixed set of categories". Chained prompts are "another dangerous vector". Willison's own verdict: "This solution is pretty bad" because of complexity and because the user can still be socially engineered by what the quarantined model wrote.
- Evidence (lethal trifecta): Willison (2025): data theft needs three things together: access to private data, exposure to untrusted content, and a way to communicate externally. Outbound links and auto-loaded images count as external communication, since data can be encoded in a URL. Vendors mostly fixed real incidents "by locking down the exfiltration vector".
- Evidence (design patterns): Beurer-Kellner et al. (2025; arXiv 2506.08837, preprint, a multi-institution author group): six patterns (action-selector, plan-then-execute, LLM map-reduce, dual LLM, code-then-execute, context-minimization) under one principle: "once an LLM agent has ingested untrusted input, it must be constrained so that it is impossible for that input to trigger any consequential actions". In map-reduce, one isolated model processes each untrusted document and its output must either go to non-LLM code or "satisfy certain safety constraints", for example "a regex that ensures the output of map is a number". Then "a malicious file is now restricted to tricking the 'map' LLM" about that one file. The paper states that restricted outputs should also not pose downstream risks "such as exfiltrating sensitive information (e.g., via embedded links) or manipulating future agent behavior (e.g., harmful responses to a user query)". Its conclusions: general-purpose agents cannot give reliable guarantees with current models; application-specific agents can be secured by design; "no single pattern is likely to suffice".
- Evidence (CaMeL): Debenedetti et al. (2025; arXiv 2503.18813, preprint; code released under google-research): a privileged model turns the user's request into a program; untrusted data is processed by a quarantined model and tracked with capabilities, so it "can never impact the program flow", and security policies are enforced when tools are called. It solved 77% of AgentDojo tasks with provable security vs 84% for an undefended system. The paper shows the plain dual LLM pattern is not enough: a poisoned document can make the quarantined model return data (an address, a file name) that the unchanged plan then acts on.
- Caveats: These designs target agents with tools and private data. No paper I read evaluates a tutor that only reads pages and writes notes. Usability costs are real: CaMeL loses 7 points of task completion and the paper discusses user fatigue from approval prompts.
- Grade: B (CaMeL, with benchmark), D (dual LLM and trifecta are expert design arguments), C to D (design patterns paper: case studies with no attack benchmark).
- Sources: [Willison2023], [Willison2025], [Beurer2025], [Debenedetti2025].

**7.5 What a quarantined reader still leaks, according to its own designers.**

- Finding: Isolation stops a hostile page from triggering tool calls. It does not stop a hostile page from changing what the reader says about the page.
- Evidence: CaMeL's stated limits: it "cannot defend against text-to-text attacks which have no consequences on the data flow, e.g., an attack prompting the assistant to summarize an email to something different than the actual content", nor against injection-induced phishing shown to the user. It is also open to side channels (an exception or a conditional fetch can leak one bit at a time), which a strict mode reduces. The paper notes that tracking where displayed content came from lets the interface show the origin to the user. Willison: the quarantined model's output is tainted, so displaying it can socially engineer the user, and any free text passed onward can re-inject the next model. Design patterns paper: the only safe hand-offs from a reader to a tool-using model are constrained formats (a number, a boolean, a fixed category) or symbolic references. OWASP lists content manipulation and biased output among injection impacts. BIPIA found summarization tasks more injectable than question answering, plausibly because nothing from the user follows the external text in the prompt.
- Caveats: No source I read measures how often a no-tools reader produces a corrupted summary under attack, as distinct from an agent taking a harmful action. That rate for a notes-only reader is an open gap.
- Grade: B to D as in 7.4.
- Sources: [Debenedetti2025], [Willison2023], [Beurer2025], [OWASP2025], [BIPIA2023].

### 8. Search strategy

**8.1 Several phrasings of the same need, fused, find more relevant material than one query. The gain is steep for the first few variants and flat after about ten.**

- Finding: Query variation is one of the better-evidenced ways to raise retrieval effectiveness.
- Evidence: Benham, Mackenzie, Moffat and Culpepper (ACM TOIS 37(4), Article 41 (unconfirmed), 2019, "Boosting Search Performance Using Query Variations"; ClueWeb12B, UQV100 collection of 10,835 crowd-written query variations for 100 topics): fusing the results of more variants raised NDCG@10 from 0.182 (1 variant) to 0.210 (2), 0.236 (5), 0.254 (10), 0.256 (20) and 0.262 (100). The paper cites earlier findings that fusing related queries on one system beat fusing one query across many better systems, and that varying the query retrieves at least as diverse a set of relevant documents as varying the retrieval system. A related thesis (Benham) reports that combining query variations and system variations in one fusion pool "can dramatically increase" recall-sensitive metrics. A 2017 ACM paper on the same UQV100 collection (abstract only) studied how inconsistent five systems' results are across phrasings of one need.
- Caveats: Laboratory test collections and classic rankers, not a commercial web search API or a search for teaching material.
- Grade: B.
- Sources: [Benham2019], [BenhamThesis], [Bailey2017].

**8.2 Deployed research agents plan first, start with short broad queries, split work across parallel searchers, and need explicit source-quality rules.**

- Finding: Vendor engineering reports describe a consistent recipe, backed by internal evaluations only.
- Evidence (Anthropic, 2025, engineering blog): a lead agent plans, saves the plan, and spawns subagents with "an objective, an output format, guidance on the tools and sources to use, and clear task boundaries"; without these, subagents "duplicate work, leave gaps". Effort rules are written into the prompt: "simple fact-finding requires just 1 agent with 3-10 tool calls", comparisons 2 to 4 subagents with 10 to 15 calls each. "Start wide, then narrow down": agents "default to overly long, specific queries that return few results". A separate citation step attributes claims to sources after research ends. The multi-agent system beat a single agent by 90.2% on an internal evaluation. Token use explained 80% of performance variance on BrowseComp. Multi-agent runs used about 15 times the tokens of a chat. Early agents preferred "SEO-optimized content farms" until source-quality heuristics were added. Evaluation began with about 20 representative queries.
- Evidence (STORM): research by asking questions from several perspectives, breaking each question into search queries, and filtering results by a rule-based reliable-sources filter before writing.
- Caveats: Vendor self-report with internal metrics and no independent test. "90.2%" is relative improvement on an undisclosed evaluation.
- Grade: C.
- Sources: [AnthropicResearch2025], [Shao2024].

**8.3 Measured recall of browsing agents on hard-to-find facts is about half at best, and persistence is what separates agents from single searches.**

- Finding: Even the strongest deep-research agent of early 2025 missed about half of hard lookups, and GPT-4o with browsing found almost nothing.
- Evidence: BrowseComp (Wei et al., 2025; arXiv 2504.12516, OpenAI): 1,266 questions built to need persistent browsing. Accuracy: GPT-4o 0.6%, GPT-4o with browsing 1.9%, o1 (no browsing) 9.9%, OpenAI Deep Research 51.5%. Human trainers, without AI help, solved 29.2% and gave up on 70.8% after about two hours. Over 64 trials per question, Deep Research solved 16% of tasks every time and failed 14% every time. A footnote says Deep Research was trained on data that specifically teaches this kind of task.
- Evidence: Liu et al. 2023: citation recall drops when no page answers the query directly (44.3 on open-ended essay questions vs 63.4 on questions with a short extractive answer). ALCE: answers did not improve with more retrieved passages beyond the top 1 to 3 for ChatGPT.
- Caveats: BrowseComp questions are puzzle-like and have one short answer, which differs from "find the best tutorial". No benchmark I found measures recall of high-quality teaching material. On stopping rules, I found no direct evidence beyond the flattening curve in 8.1 and the effort budgets in 8.2.
- Grade: B.
- Sources: [BrowseComp2025], [Liu2023], [Gao2023].

**8.4 Known-good source lists and allowlists are used in practice. Direct tests of them for teaching material are missing.**

- Finding: Systems that work in education restrict the source set (Jill Watson answers only from instructor-approved courseware; STORM filters by a reliable-sources rule; Khan Academy checks math with a dedicated agent). The Wikipedia studies and the lateral-reading work give the rationale. I found no experiment comparing an allowlist against open search for lesson building.
- Evidence: See 5.5, 5.6 and 1.1. Jill Watson's accuracy varied by source type: about 75 to 80% on syllabi and over 90% on textbooks, so even inside a vetted set the kind of document matters.
- Caveats: An allowlist trades recall for precision, and nobody has measured that trade for this use.
- Grade: C.
- Sources: [JillWatson2024], [AIALOE2024], [Shao2024], [Khan2026].

### 9. Keeping sources alive

**9.1 Snapshot the page when you cite it. Archives made later often do not hold the version you read.**

- Finding: For most web references in scholarly papers, no archived copy representative of the cited version existed.
- Evidence: Jones et al. (2016): representative archived snapshots existed for about 30% of 1,059,742 references; 14.33% had no archived copy at all; for 31.58%, snapshots before and after the citation date existed and differed too much to stand for the cited version. Klein et al. (2014) and Zittrain et al. (2021) give the decay rates in 2.2 and 2.3.
- Grade: B.
- Sources: [Jones2016], [Klein2014], [Zittrain2021].

**9.2 The documented practice is: store the original URL, the date you read it, and a snapshot URL together.**

- Finding: The Robust Links specification (from the Hiberlink and Memento work) and Wikipedia's citation practice both pair the live URL with a dated archive copy.
- Evidence: Robust Links: "It recommends creating one or more snapshots of a resource around the time of linking to it, for example, in a web archive or a versioning system", and conveying three things on the link: the original URL (data-originalurl, required), the intended linking date (data-versiondate, required) and the snapshot URL or URLs (data-versionurl, recommended). It names the Internet Archive's "Save Page Now" and perma.cc as snapshot services, and versioning systems (each version of a hosted file has its own URL) as an alternative. Perma.cc (developed at Harvard, launched in fall 2013): preserves "the entire webpage, not just a screenshot", so readers "will always be able to see what a webpage looked like when the link was created"; more than 150 law journals use it (Stanford Law library guide). Wikipedia: editors are encouraged to add an archive link "as a part of each citation ... at the same time that each citation is created", with archive-url, archive-date and url-status fields. The Internet Archive archives URLs added to Wikipedia at about 20 million per week, and its bot plus volunteers had repaired more than 9 million broken links across 22 language editions by October 2018.
- Caveats: Archives cannot capture pages behind logins, some dynamic pages, or sites that block crawlers. I found no source on content hashes or on re-check schedules as such. The nearest evidence is the text-similarity method Jones et al. used to detect drift, and the age curves in 2.2 (for example 6% of deep links dead after about two years, 8% of pages gone within the year they were sampled), which indicate how fast a stored list decays.
- Grade: C (documented practice and specifications).
- Sources: [RobustLinks], [PermaGuide], [WikiArchive], [IABlog2018], [Jones2016].

**9.3 Storing the extracted notes or quote next to the URL is what makes a later check possible.**

- Finding: A claim stored with its verbatim quote, source URL, read date and snapshot can be re-verified by string match even after the live page changes. A claim stored with a bare URL cannot.
- Evidence: This follows from GopherCite's split of verification into a mechanical quote check and a human support judgment (6.4), from the content-drift rates (2.3), and from the Robust Links fields (9.2). CaMeL adds that tracking the origin of each piece of content lets an interface show the user where it came from (7.5). I found no study that measures reviewer time with and without stored quotes.
- Grade: D (inference from B and C evidence).
- Sources: [Menick2022], [Zittrain2021], [RobustLinks], [Debenedetti2025].

## Disagreements and open questions

**Disagreements in the evidence**

- **Checklist versus lateral reading.** CRAAP-style checklists are still widely taught. The Stanford group's studies show experts do not use them and that a front-group site passes one [Wineburg2019], [Breakstone2018]. The counterpoint from librarians is that lateral reading and SIFT fit news and advocacy content best and "may be less effective for some other types of information" [CutTheCRAAP]. For technical teaching pages the question "who is behind this" is often easy (official docs, a university) and the hard question is "is this explanation correct and well built", which lateral reading does not answer.
- **Can a model replace a human relevance or quality judge?** Thomas et al. report LLM labels "as accurate as human labellers" at Bing [Thomas2024]. Faggioli et al. found kappa of 0.26 to 0.40 against TREC assessors and argue against full automation [Faggioli2023]. Yang and Menczer found only moderate agreement with experts on credibility (rho 0.50) [Yang2023]. The studies differ in prompt tuning: Thomas et al. selected the prompt against gold labels, and their own range across prompts was kappa 0.20 to 0.64.
- **Does retrieval fix hallucination?** Jill Watson passed 77% of test questions against 31% for a generic retrieval assistant on the same model, a gain from retrieval design over vetted courseware [JillWatson2024]. SourceCheckup concludes "retrieval augmentation by itself is not a silver bullet": GPT-4 with retrieval still left a large share of responses unsupported and gave no source at all in over 20% [Wu2024]. Retrieval did not improve prerequisite prediction [Yang2024Graph]. FActScore found the search-augmented system best and still at 71% [Min2023]. Both sides fit one reading: retrieval removes invented sources, and the faithfulness of each statement to its source still has to be checked.
- **Do prompt-level injection defenses work?** Spotlighting cut attack success from about 50% to about 3% [Hines2024], and AgentDojo found detectors and tool filtering at about 8% [AgentDojo2024]. Under adaptive attack, Spotlighting (in a delimiter form) and prompt repetition were broken at over 95% and the same injection detector at over 90%; tool filtering was not among the twelve defenses tested [Nasr2025]. The originating papers tested fixed attacks.
- **Vendor robustness claims versus independent tests.** Anthropic reports about 1% attack success for its best browser configuration on its own adaptive test [AnthropicPI2025]. The independent Gray Swan competition found 0.5% to 8.5% across 13 frontier models and no model immune [GraySwan2026]. These are different tests and neither is a real-world attack frequency.
- **Do stronger models hallucinate less when summarizing?** Vectara's leaderboard shows several frontier reasoning models above 10% while a small model leads at 3.3% [Vectara2025]. This rests on Vectara's own detector. FaithBench (human-labelled) ranks models in broadly the same order as the leaderboard on hard cases [FaithBench2025]. No independent human-labelled study of ordinary-difficulty documents was found.
- **Is automated fact checking better than people?** SAFE's authors report it wins 76% of disagreements with crowd annotators [Wei2024]. Critics show decompose-then-verify scores move with the decomposition method [Wanner2024] and can be gamed with trivial true claims [Core2024]. FActScore's own authors note it ignores recall [Min2023].
- **The reverse cohesion effect.** McNamara et al. (1996) found high-knowledge readers learned more from a less coherent text [McNamara1996]. A companion study failed to replicate part of it [McNamaraKintsch1996], and a later study found the effect only for less skilled readers [OReilly2007]. The half that holds across studies is that low-knowledge readers need the explicit version.
- **How far experts agree on prerequisites.** Reported agreement ranges from kappa 0.19 to 0.74 [PRET2018], [UPreG2021]. The spread tracks task design: judging pairs a system proposed gives high agreement, free annotation of a whole concept set gives low agreement.
- **Is web search getting worse?** Bevendorff et al. found more SEO and affiliate content in higher-ranked product-review results, and also that search engines' updates had a real if short-lived effect [Bevendorff2024]. A practitioner critique says one study of one spam-prone niche cannot be generalized [SELand2025]. Industry figures for the share of AI-written pages range from about 20% of sampled sites to 74% of new pages, measured in incompatible ways and read here only through secondary pages [Maxintel2026], [Contentifai2025].
- **Wikipedia's accuracy.** Giles found Wikipedia close to Britannica on science [Giles2005]. Britannica disputed the study, and the systematic review found "little consensus" [Mesgari2015].

**Open questions I could not close**

- No study was found of a model rating the teaching quality of whole web pages against expert OER reviewers. The nearest evidence is on short explanations, hints and quiz questions (1.7).
- No study measures how often a tools-free reader model returns corrupted notes when the page attacks it. The injection benchmarks measure harmful actions by tool-using agents, and CaMeL explicitly leaves text-to-text attacks out of scope (7.5).
- No experiment compares a domain allowlist with open search for building lessons, and no evidence-based stopping rule for source search was found beyond the flattening gain from extra query variants (8.1, 8.4).
- No source was found on content hashes or re-check schedules for stored URLs (9.2).
- The effect size of lateral-reading instruction is unconfirmed here (abstract and supplement only).
- EdReports and Lexile were named leads and were not fetched. Their methods and validity stay unconfirmed in this file.
- What quoting or summarizing an all-rights-reserved page permits was not checked against a legal source.
- The independence of two web sources that agree (the prototype's prerequisite rule) is not addressed by any paper I read.
- All citation-faithfulness audits are on news, medicine or general questions. None is on programming-tool teaching content.

## What the sources themselves recommend

These are the sources' own prescriptions, attributed. They are not recommendations for the Pedagogue.

**On judging sources**

- Wineburg and McGrew; Breakstone et al.: before reading an unfamiliar site closely, leave it and search for what others say about who is behind it ("lateral reading"); evaluate the result list before clicking ("click restraint"); plan before diving in ("taking bearings"). Teach "fewer strategies more robustly" [Wineburg2019], [Wineburg2022], [Breakstone2018].
- Caulfield, as reported: Stop; Investigate the source; Find better coverage; Trace claims, quotes and media to the original context [CutTheCRAAP].
- Achieve: score each learning object separately on each applicable rubric, on a 0 to 3 scale, at "the smallest meaningful unit", and score alignment only after an objective has been proposed for the object [AchieveOER].
- MERLOT: two independent expert reviewers per material, a third when they differ significantly, and a written "concerns" note whenever a rating is below the top score [MERLOT], [MERLOTForm].
- Thomas et al.: tune and select the judging prompt against a small set of high-quality gold labels, and keep auditing, because performance "varies unpredictably with simple paraphrases" [Thomas2024].
- Faggioli et al.: keep humans in the loop along a "human-machine collaboration spectrum" for relevance judgments [Faggioli2023].
- Moore, Bier and Stamper: use "a hybrid approach to educational content evaluation, integrating the scalability of AI with the nuanced judgment of humans" [Moore2024].
- Anthropic research team: add explicit source-quality heuristics to research prompts so agents prefer authoritative sources over SEO content, and keep manual testing because it catches "subtle source selection biases" [AnthropicResearch2025].

**On citations and fact checking**

- Liu, Zhang and Liang: measure both citation recall (every statement supported) and citation precision (every citation supports its statement) [Liu2023].
- Gao et al.: sample several answers and keep the best by citation score; do not add citations after a closed-book answer [Gao2023].
- Walters and Wilder; Chelli et al.: references produced by a model "undergo rigorous validation" by the authors; do not use such tools as "the sole or primary means" of a literature search [Walters2023], [Chelli2024].
- Min et al.; Wei et al.: break generated text into atomic claims and check each against a trusted source or search results [Min2023], [Wei2024].
- Jiang et al. (Core): count only unique, informative claims [Core2024].
- Dhuliawala et al.: verify with questions answered in separate prompts that do not see the draft, then revise [Dhuliawala2023].
- Menick et al.: answer with a verbatim quote that code can check against the source, and abstain when support is weak [Menick2022].
- Anthropic, via secondary summaries: allow "I don't know"; extract word-for-word quotes first for long documents; after drafting, find a quote for each claim and retract claims with none; restrict the model to the supplied documents [Csorba2026], [Botnet2026].
- Taneja et al. (Jill Watson): ground answers in instructor-approved courseware, prompt for answers grounded in the retrieved context, and use an "I don't know" response when the query cannot be answered [JillWatson2024].
- Khan Academy: verify math with a dedicated agent and track error and answer give-away rates as guardrail metrics in every experiment [Khan2026].
- BBC and EBU: AI assistants "are still not a reliable way to access and consume news"; the study asks for safeguards and better sourcing [BBCEBU2025].
- Tow Center: tools should decline or signal uncertainty when they cannot find the source, in place of confident wrong answers [Tow2025].

**On prompt injection**

- Willison: never forward unfiltered output of a quarantined model to a privileged one; pass references or validated fixed categories; avoid combining private data, untrusted content and external communication; restrict outbound links and images to an allowlist; do not rely on guardrail products [Willison2023], [Willison2025].
- Beurer-Kellner et al.: "Prioritize the development of application-specific agents that adhere to secure design patterns and clearly define trust boundaries", and "use a combination of design patterns" [Beurer2025].
- Debenedetti et al. (CaMeL): fix control flow from the trusted request before reading untrusted data, track data origin, enforce policies at tool calls, and show the origin of content to the user [Debenedetti2025].
- OWASP LLM01:2025: constrain model behavior; define and validate output formats with deterministic code; filter inputs and outputs; least privilege; human approval for high-risk actions; segregate and label external content; run adversarial tests [OWASP2025].
- Hines et al.: mark untrusted text throughout (datamarking or encoding) and tell the model about the marking [Hines2024].
- Nasr et al.: evaluate any defense against adaptive attackers and human red-teamers who know the defense; do not trust near-zero rates on static benchmarks [Nasr2025].
- Anthropic: treat even a 1% attack success rate as meaningful risk [AnthropicPI2025].
- Gray Swan authors: refresh attack sets through repeated public competitions because benchmarks saturate [GraySwan2026].

**On search**

- Anthropic research team: plan first; give each searcher an objective, output format, source guidance and boundaries; scale effort to query complexity with explicit budgets; start with short broad queries and narrow; start evaluation with about 20 real queries [AnthropicResearch2025].
- Benham et al.: fuse results from several query variations [Benham2019].
- STORM authors: ask questions from multiple perspectives, and filter sources with an explicit reliable-sources rule [Shao2024].

**On keeping sources alive**

- Robust Links: at linking time create a snapshot and record the original URL, the linking date and the snapshot URL [RobustLinks].
- Wikipedia: add an archive link "at the same time that each citation is created" [WikiArchive].
- Zittrain et al.; Klein et al.; Jones et al.: archive cited pages proactively, since later archives often miss the cited version [Zittrain2021], [Klein2014], [Jones2016].

## Source list

Read status key: "full text" means I read the relevant sections of the fetched page, "abstract only", or "secondary summary at URL". Where a page was reached through a search tool, "search excerpt" means I saw only the passages the tool returned. Preprints are labelled.

**Judging source quality**

- [Wineburg2019] Wineburg, S., and McGrew, S. (2019). Lateral Reading and the Nature of Expertise: Reading Less and Learning More When Evaluating Digital Information. Teachers College Record 121(11). https://journals.sagepub.com/doi/10.1177/016146811912101102 Open access: https://stacks.stanford.edu/file/yk133ht8603/Wineburg%20McGrew_Lateral%20Reading%20and%20the%20Nature%20of%20Expertise.pdf Read: abstract plus search excerpts of the full-text PDF (results section).
- [Breakstone2018] Breakstone, J., McGrew, S., Ortega, T., Smith, M., and Wineburg, S. (2018). Why we need a new approach to teaching digital literacy. Phi Delta Kappan (Kappan Online), 26 Feb 2018. https://kappanonline.org/breakstone-need-new-approach-teaching-digital-literacy/ Read: full text (key sections).
- [Breakstone2024] Breakstone, J., McGrew, S., Smith, M., and Wineburg, S. (2024). Distinguishing Credible from Sham: Supporting Young People to Navigate Online. In Handbook of Children and Screens, pages 485-489. Springer open-access chapter, online 6 December 2024. https://link.springer.com/chapter/10.1007/978-3-031-69362-5_66 Read: search excerpt.
- [CutTheCRAAP] Blizzard, K., Librarian, University of Alberta (the slides carry no date; they cite a February 2021 source, so 2021 or later). Cut the CRAAP: Teaching Information Evaluation in the Misinformation Age (slides). https://ualberta.scholaris.ca/server/api/core/bitstreams/0ba27848-034b-49de-8fc6-96e0cd16126c/content Read: search excerpt.
- [Wineburg2022] Wineburg, S., Breakstone, J., McGrew, S., Smith, M. D., and Ortega, T. (2022). Lateral reading on the open Internet: A district-wide field study in high school government classes. Journal of Educational Psychology 114(5), 893-909. https://doi.org/10.1037/edu0000740 Open access: https://papers.ssrn.com/sol3/papers.cfm?abstract_id=3936112 ERIC record: https://eric.ed.gov/?id=EJ1372738 Supplement: https://supp.apa.org/psycarticles/supplemental/edu0000740/EDU_2020_0630_Supplemental_Materials.pdf Read: abstract only, plus a supplement excerpt.
- [AchieveOER] ISKME (2017). Comprehensive OER Evaluation Tool, adapted for higher education from Achieve's Rubrics for Evaluating Open Education Resource (OER) Objects (Achieve, CC BY 3.0). https://tlp-lpa.ca/ld.php?content_id=34114519 Read: search excerpt of the adapted rubric document.
- [MERLOT] MERLOT. MERLOT Peer Review Information. https://info.merlot.org/merlothelp/MERLOT_Peer_Review_Information.htm Read: search excerpt.
- [MERLOTForm] MERLOT. Peer Review Report Form V 17.6. https://info.merlot.org/merlothelp/assets/docs/Peer_Review_form_CC_0519.pdf Read: search excerpt (item lists).
- [IATUL2003] Orhun, E. (2003). Web-Based Educational Resources for Learning and Online Teaching in Higher Education: the MERLOT Project. IATUL conference. https://docs.lib.purdue.edu/iatul/2003/papers/40 Read: search excerpt.
- [Yang2023] Yang, K.-C., and Menczer, F. (2023 preprint, v3 revised February 2025). Accuracy and Political Bias of News Source Credibility Ratings by Large Language Models. Published in Proceedings of the 17th ACM Web Science Conference (WebSci '25), pages 127-137, https://doi.org/10.1145/3717867.3717903 Preprint: https://arxiv.org/abs/2304.00228 Read: full text (abstract, methods, results excerpts).
- [Thomas2024] Thomas, P., Spielman, S., Craswell, N., and Mitra, B. (2023/2024). Large language models can accurately predict searcher preferences. Preprint as read; published at SIGIR 2024, pages 1930-1940, https://doi.org/10.1145/3626772.3657707 Preprint: https://arxiv.org/abs/2309.10621 Read: full text (results excerpts).
- [Faggioli2023] Faggioli, G., et al. (2023). Perspectives on Large Language Models for Relevance Judgment. ICTIR 2023. DOI 10.1145/3578337.3605136. Open access: https://arxiv.org/abs/2304.09161 Read: full text (abstract, pilot results).
- [Tithi2025] Tithi, S. D., Ramesh, A. K., DiMarco, C., Tian, X., Alam, N., Fazeli, K., and Barnes, T. (2025). The Promise and Limits of LLMs in Constructing Proofs and Hints for Logic Problems in Intelligent Tutoring Systems. Published in Computers and Education: Artificial Intelligence 9, 100490, https://doi.org/10.1016/j.caeai.2025.100490 (arXiv v2 is the journal version). Preprint v1 as read: https://arxiv.org/html/2505.04736v1 Read: search excerpt.
- [SelfExplain2025] Yamauchi, T., Flanagan, B., Dai, Y., Kita, T., and Ogata, H. (2025). A Rubric-based LLM Automatic Grading of Mathematical Reasoning in Self-explained Answers. Proceedings of the 1st International Conference on Learning Evidence and Analytics (ICLEA), APSCE. https://library.apsce.net/index.php/ICLEA/article/download/5504/5436/6676 Read: search excerpt.
- [MathExplain2024] Worden, E., Croteau, E., Cheng, L., McReynolds, A., and Heffernan, N. (2024, the date on the NSF repository record, which lists it as submitted to LAK 2024 and in review). Leveraging Large Language Models for Evaluating Explanations in Math Education. Anonymized copy: https://par.nsf.gov/servlets/purl/10470442 Record with authors: https://par.nsf.gov/biblio/10470442 Read: abstract and search excerpt.
- [Moore2024] Moore, S., Bier, N., and Stamper, J. (2024). Assessing Educational Quality: Comparative Analysis of Crowdsourced, Expert, and AI-Driven Rubric Applications. HCOMP 2024. https://par.nsf.gov/servlets/purl/10553100 Read: abstract only.

**Reliability of the open web**

- [Giles2005] Giles, J. (2005). Internet encyclopaedias go head to head. Nature 438, 900-901. https://www.nature.com/articles/438900a Read: search excerpt of the article text. The Nature page now shows only a preview; the full text was checked on a 2006 Wayback Machine copy of the same article.
- [Mesgari2015] Mesgari, M., et al. (2015). "The sum of all human knowledge": A systematic review of scholarly research on the content of Wikipedia. JASIST. https://asistdl.onlinelibrary.wiley.com/doi/10.1002/asi.23172 Read: abstract only.
- [Pew2024] Chapekis, A., Bestvater, S., Remy, E., and Rivero, G. (2024). When Online Content Disappears. Pew Research Center, 17 May 2024. https://www.pewresearch.org/data-labs/2024/05/17/when-online-content-disappears/ PDF: https://www.pewresearch.org/wp-content/uploads/sites/20/2024/05/pl_2024.05.17_link-rot_report.pdf Read: search excerpt of the report page. The "50,000 pages, 11% of reference links" detail was first seen in a secondary summary at https://www.computerworld.com/article/2121634/the-web-is-disappearing-is-it-its-fault.html and is in the report PDF itself.
- [Zittrain2021] Zittrain, J., Bowers, J., and Stanton, C. (2021). The Paper of Record Meets an Ephemeral Web: An Examination of Linkrot and Content Drift within The New York Times. Library Innovation Lab, Harvard Law School. Record page: https://dash.harvard.edu/handle/1/37367405 PDF (returned a bot check to scripts on 2026-11-22; a Wayback Machine copy of the same PDF was read instead): https://dash.harvard.edu/bitstreams/10737af1-7d3d-459b-8ef8-177a6b27add8/download SSRN: https://papers.ssrn.com/sol3/papers.cfm?abstract_id=3833133 Read: search excerpt of the full text (summary and methods).
- [Klein2014] Klein, M., Van de Sompel, H., et al. (2014). Scholarly Context Not Found: One in Five Articles Suffers from Reference Rot. PLOS ONE. https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0115253 Read: full text (abstract and results excerpts).
- [Jones2016] Jones, S. M., Van de Sompel, H., Shankar, H., Klein, M., Tobin, R., and Grover, C. (2016). Scholarly Context Adrift: Three out of Four URI References Lead to Changed Content. PLOS ONE 11(12): e0167475. https://doi.org/10.1371/journal.pone.0167475 Open access: https://journals.plos.org/plosone/article?id=10.1371%2Fjournal.pone.0167475 Read: abstract and search excerpts.
- [CCLicenses] Creative Commons. About CC licenses ("Sharing Openly, Sharing Globally"). https://creativecommons.org/share-your-work/cclicenses/ Read: full text.
- [Bevendorff2024] Bevendorff, J., Wiegmann, M., Potthast, M., and Stein, B. (2024). Is Google Getting Worse? A Longitudinal Investigation of SEO Spam in Search Engines. ECIR 2024, Advances in Information Retrieval Part III, pages 56-71, https://doi.org/10.1007/978-3-031-56063-7_4 Open-access copy: https://downloads.webis.de/publications/papers/bevendorff_2024a.pdf Proceedings listing: https://dl.acm.org/doi/proceedings/10.1007/978-3-031-56063-7 Read: abstract opening only; findings via secondary summary at https://searchengineland.com/is-google-really-getting-worse-actually-its-complicated-455079
- [SELand2025] Götza, P. (2025). Is Google really getting worse? (Actually, it's complicated). Search Engine Land, 7 May 2025. Practitioner article. https://searchengineland.com/is-google-really-getting-worse-actually-its-complicated-455079 Read: search excerpt.
- [Contentifai2025] Contentifai (marketing page citing Ahrefs 2025, "74% of New Webpages Include AI Content (Study of 900k Pages)"). https://www.contentifai.agency/category/announcements/ Read: search excerpt. Secondary summary, unconfirmed at the primary source.
- [Maxintel2026] Walsch, N. (2026). Search Engine Market Share 2026. https://maxintel.org/search-engine-landscape-2026.html Read: search excerpt. Secondary summary of Originality.ai and the Leipzig study, unconfirmed at the primary sources.

**Matching material to the learner's level**

- [Crossley2022] Crossley, S., et al. (published online 16 March 2022). A large-scaled corpus for assessing text readability. Behavior Research Methods. https://link.springer.com/article/10.3758/s13428-022-01802-x Read: search excerpt (background section).
- [TSAR2024] Trott, S., and Rivière, P. D. (2024). Measuring and Modifying the Readability of English Texts with GPT-4. Proceedings of TSAR 2024, pages 126-134. https://aclanthology.org/2024.tsar-1.13/ PDF: https://aclanthology.org/2024.tsar-1.13.pdf Read: search excerpt of the full text (methods and results).
- [McNamara1996] McNamara, D. S., Kintsch, E., Songer, N. B., and Kintsch, W. (1996). Are Good Texts Always Better? Interactions of Text Coherence, Background Knowledge, and Levels of Understanding in Learning From Text. Cognition and Instruction 14(1), 1-43. https://www.tandfonline.com/doi/abs/10.1207/s1532690xci1401_1 Copy: https://www.academia.edu/783348/Are_Good_Texts_Always_Better_Interactions_of_Text_Coherence_Background_Knowledge_and_Levels_of_Understanding_In_Learning_From_Text Read: abstract plus search excerpts of the full text (Experiment 2 table).
- [McNamaraKintsch1996] McNamara, D. S., and Kintsch, W. (1996). Learning from texts: Effects of prior knowledge and text coherence. Discourse Processes. https://doi.org/10.1080/01638539609544975 Read: abstract only.
- [OReilly2007] O'Reilly, T., and McNamara, D. S. (2007). Reversing the Reverse Cohesion Effect: Good Texts Can Be Better for Strategic, High-Knowledge Readers. Discourse Processes. https://doi.org/10.1080/01638530709336895 Read: abstract only.

**Prerequisite structure**

- [Talukdar2012] Talukdar, P. P., and Cohen, W. W. (2012). Crowdsourced Comprehension: Predicting Prerequisite Structure in Wikipedia. 7th Workshop on Innovative Use of NLP for Building Educational Applications, 307-315. https://aclanthology.org/W12-2037/ PDF: https://aclanthology.org/W12-2037.pdf Read: full text (tables and results excerpts).
- [Liang2015] Liang, C., Wu, Z., Huang, W., and Giles, C. L. (2015). Measuring Prerequisite Relations Among Concepts. EMNLP 2015, 1668-1674. https://aclanthology.org/D15-1193/ Read: title page only; description via Semantic Scholar excerpt at https://www.semanticscholar.org/paper/Recovering-Concept-Prerequisite-Relations-from-Liang-Ye/946131231afc2af62cfe5c14092b1523ed057d19
- [Liang2017] Liang, C., Ye, J., Wu, Z., Pursel, B., and Giles, C. L. (2017). Recovering Concept Prerequisite Relations from University Course Dependencies. AAAI 2017. https://doi.org/10.1609/aaai.v31i1.10550 Read: search excerpts of the full text (dataset, labelling, human performance). The author list was confirmed on Crossref in the citation check.
- [Pan2017] Pan, L., Li, C., Li, J., and Tang, J. (2017). Prerequisite Relation Learning for Concepts in MOOCs. ACL 2017. https://aclanthology.org/P17-1133/ Read: abstract only.
- [Gordon2016] Gordon, J., Zhu, L., Galstyan, A., Natarajan, P., and Burns, G. (2016). Modeling Concept Dependencies in a Scientific Corpus. ACL 2016. https://aclanthology.org/P16-1082/ Read: title page only; agreement figure via [PRET2018].
- [PRET2018] Alzetta, C., Koceva, F., Passalacqua, S., Torre, I., and Adorni, G. (2018). PRET: Prerequisite-Enriched Terminology. A Case Study on Educational Texts. In Proceedings of the Fifth Italian Conference on Computational Linguistics (CLiC-it 2018), pages 14-20. Accademia University Press (OpenEdition), https://doi.org/10.4000/books.aaccademia.3028 Page: https://books.openedition.org/aaccademia/3028 Open copy: https://ceur-ws.org/Vol-2253/paper64.pdf Read: search excerpt of the full text (agreement section).
- [UPreG2021] Sabnis, V., Abhinav, K., Subramanian, V., Dubey, A., and Bhat, P. (2021). UPreG: An Unsupervised approach for building the Concept Prerequisite Graph. EDM 2021. https://educationaldatamining.org/EDM2021/virtual/static/pdf/EDM21_paper_252.pdf Read: search excerpt.
- [Yang2024Graph] Yang, R., Yang, B., Ouyang, S., She, T., Feng, A., Jiang, Y., Lecue, F., Lu, J., and Li, I. (2024). Leveraging Large Language Models for Concept Graph Recovery and Question Answering in NLP Education. Preprint. https://arxiv.org/abs/2402.14293 Read: search excerpt of the full text (main results table).
- [GPTConcepts2025] Yang, T., Ren, B., Gu, C., He, T., Ma, B., and Konomi, S. (2025). Examining GPT's Capability to Generate and Map Course Concepts and Their Relationships. Preprint. https://arxiv.org/abs/2504.08856 Read: search excerpt.
- [ESCOPrereq2025] Le, N. L., and Abel, M.-H. (2025). How Well Do LLMs Predict Prerequisite Skills? Zero-Shot Comparison to Expert-Defined Concepts. Preprint. https://arxiv.org/abs/2507.18479 Read: abstract and results table excerpt.

**Grounded generation and citation faithfulness**

- [Liu2023] Liu, N. F., Zhang, T., and Liang, P. (2023). Evaluating Verifiability in Generative Search Engines. Preprint as read. https://arxiv.org/abs/2304.09848 Read: full text.
- [Gao2023] Gao, T., Yen, H., Yu, J., and Chen, D. (2023). Enabling Large Language Models to Generate Text with Citations (ALCE). Preprint as read. https://arxiv.org/abs/2305.14627 Read: full text (abstract, intro, analysis excerpts).
- [Wu2024] Wu, K., Wu, E., Casasola, A., Zhang, A., Wei, K., Nguyen, T., Riantawan, S., Shi, P., Ho, D., and Zou, J. (2024). How well do LLMs cite relevant medical references? An evaluation framework and analyses. Preprint. https://arxiv.org/abs/2402.02008 Read: full text (abstract, results excerpts). Since published with a larger study and different numbers as Wu, K., Wu, E., Wei, K., Zhang, A., Casasola, A., Nguyen, T., Riantawan, S., Shi, P., Ho, D., and Zou, J. (2025). An automated framework for assessing how well LLMs cite relevant medical references. Nature Communications 16, 3615. https://doi.org/10.1038/s41467-025-58551-6 The numbers in this file are the preprint's unless marked.
- [DRBench2025] Du, M., Xu, B., Zhu, C., Wang, X., and Mao, Z. (2025). DeepResearch Bench: A Comprehensive Benchmark for Deep Research Agents. Preprint. https://arxiv.org/abs/2506.11763 Read: full text (abstract, results table).
- [Tow2025] Jaźwińska, K., and Chandrasekar, A. (2025). AI Search Has a Citation Problem. Tow Center for Digital Journalism, Columbia Journalism Review, 6 March 2025. https://www.cjr.org/tow_center/we-compared-eight-ai-search-engines-theyre-all-bad-at-citing-news.php Read: full text.
- [BBCEBU2025] BBC and European Broadcasting Union (2025). News Integrity in AI Assistants: An international PSM study. https://www.ebu.ch/Report/MIS-BBC/NI_AI_2025.pdf Press release: https://www.bbc.co.uk/mediacentre/2025/new-ebu-research-ai-assistants-news-content/ Read: full text (summary, method, accuracy and quote sections).
- [Walters2023] Walters, W. H., and Wilder, E. I. (2023). Fabrication and errors in the bibliographic citations generated by ChatGPT. Scientific Reports. https://www.nature.com/articles/s41598-023-41032-5 Read: full text (abstract, background, methods).
- [Chelli2024] Chelli, M., et al. (2024). Hallucination Rates and Reference Accuracy of ChatGPT and Bard for Systematic Reviews: Comparative Analysis. Journal of Medical Internet Research 26:e53164. https://www.jmir.org/2024/1/e53164 Read: full text (abstract, results, conclusions).
- [Vectara2025] Vectara (2025). Introducing the next generation of Vectara's Hallucination Leaderboard. Vendor blog, 19 Nov 2025. https://www.vectara.com/blog/introducing-the-next-generation-of-vectaras-hallucination-leaderboard Repository: https://github.com/vectara/hallucination-leaderboard Read: search excerpt of the blog; repository front page.
- [FaithBench2025] Bao, F. S., Li, M., Qu, R., Luo, G., Wan, E., Tang, Y., Fan, W., Tamber, M. S., Kazi, S., Sourabh, V., Qi, M., Tu, R., Xu, C., Gonzales, M., Mendelevitch, O., and Ahmad, A. (arXiv October 2024; NAACL 2025). FaithBench: A Diverse Hallucination Benchmark for Summarization by Modern LLMs. NAACL 2025 per the repository. https://arxiv.org/html/2410.13210 Repository: https://github.com/vectara/FaithBench Read: search excerpt of the full text (annotation, results table).
- [JillWatson2024] Taneja, K., Maiti, P., Kakar, S., Guruprasad, P., Rao, S., and Goel, A. K. (2024). Jill Watson: A Virtual Teaching Assistant powered by ChatGPT. Preprint. https://arxiv.org/html/2405.11070 Read: search excerpt of the full text (results).
- [AIALOE2024] National AI Institute for Adult Learning and Online Education. The Return of Jill Watson. Project report page. https://aialoe.org/the-return-of-jill-watson/ Read: search excerpt.
- [Kakar2024] Kakar, S., et al. (2024). Jill Watson: Scaling and Deploying an AI Conversational Agent in Online Classrooms. In Generative Intelligence and Intelligent Tutoring Systems (ITS 2024, Springer LNCS 14798), pages 78-90. https://doi.org/10.1007/978-3-031-63028-6_7 Link first recorded (returns a bot check): https://one-ro.literatumonline.com/doi/10.1007/978-3-031-63028-6_7 Read: abstract only. Authors confirmed on Crossref: Kakar, Maiti, Taneja, Nandula, Nguyen, Zhao, Nandan and Goel.
- [Khan2026] Khan Academy (2026). How Khan Academy Is Building a Better AI Tutor: Our Most Recent Learnings. Blog, May 2026. https://blog.khanacademy.org/how-khan-academy-is-building-a-better-ai-tutor-our-most-recent-learnings/ Read: full text.
- [Shao2024] Shao, Y., et al. (2024). Assisting in Writing Wikipedia-like Articles From Scratch with Large Language Models (STORM). Preprint v2 as read. https://arxiv.org/abs/2402.14207 Read: full text (abstract, method, evaluation, appendix excerpts).

**Checking facts**

- [Min2023] Min, S., et al. (2023). FActScore: Fine-grained Atomic Evaluation of Factual Precision in Long Form Text Generation. Preprint as read. https://arxiv.org/abs/2305.14251 Read: full text (abstract, intro, estimator, limitations).
- [Wei2024] Wei, J., et al. (2024). Long-form factuality in large language models (SAFE and LongFact). Preprint as read. https://arxiv.org/abs/2403.18802 Read: full text (abstract, SAFE evaluation, appendix excerpts).
- [Wanner2024] Wanner, M., et al. (2024). A Closer Look at Claim Decomposition. Preprint. https://arxiv.org/html/2403.11903 Read: abstract and search excerpt. Authors confirmed on arXiv: Wanner, Ebner, Jiang, Dredze and Van Durme.
- [Core2024] Jiang, Z., et al. (2024). Core: Robust Factual Precision with Informative Sub-Claim Identification. Preprint. https://arxiv.org/html/2407.03572 Read: search excerpt (intro, results table). Authors confirmed on arXiv: Jiang, Zhang, Weir, Ebner, Wanner, Sanders, Khashabi, Liu and Van Durme.
- [VeriScore2024] Song, Y., Kim, Y., and Iyyer, M. (2024). VeriScore: Evaluating the factuality of verifiable claims in long-form text generation. Preprint. https://arxiv.org/abs/2406.19276 Read: abstract excerpt only.
- [Dhuliawala2023] Dhuliawala, S., et al. (2023). Chain-of-Verification Reduces Hallucination in Large Language Models. Preprint as read. https://arxiv.org/abs/2309.11495 Read: full text (method, results).
- [SelfCheck2023] Manakul, P., Liusie, A., and Gales, M. (2023). SelfCheckGPT: Zero-Resource Black-Box Hallucination Detection for Generative Large Language Models. https://arxiv.org/html/2303.08896v3 Read: abstract and results table excerpt. Authors confirmed on arXiv (EMNLP 2023 per the arXiv record).
- [CQCRAG2026] Sun, Y., Liu, S., and Shao, J. (2026). CQC-RAG: Robust Retrieval-Augmented Generation via Cross-Query Consistency. Preprint. https://arxiv.org/html/2606.13438 Read: abstract and search excerpt.
- [Menick2022] Menick, J., et al. (2022). Teaching language models to support answers with verified quotes (GopherCite). Preprint. https://arxiv.org/abs/2203.11147 Read: full text (abstract, intro, conclusion).
- [Weller2023] Weller, O., et al. (2023). "According to ...": Prompting Language Models Improves Quoting from Pre-Training Data. Preprint as read. https://arxiv.org/abs/2305.13252 Read: abstract and intro.
- [Csorba2026] Csorba, B. LLM hallucinations: grounding and citations. Practitioner blog. https://balazscsorba.com/blog/llm-hallucination-grounding-citations Read: search excerpt. Secondary summary of Anthropic documentation.
- [Botnet2026] Botnet resources. When should a research agent extract word-for-word quotes from its sources? Practitioner page. https://botnet.com/resources/research-when-to-quote-extraction Read: search excerpt. Secondary summary of Anthropic documentation.

**Prompt injection**

- [Greshake2023] Greshake, K., et al. (2023). Not what you've signed up for: Compromising Real-World LLM-Integrated Applications with Indirect Prompt Injection. https://arxiv.org/abs/2302.12173 Read: abstract and intro.
- [Willison2023] Willison, S. (2023). The Dual LLM pattern for building AI assistants that can resist prompt injection. Blog, 25 April 2023, with an April 2025 update. https://simonwillison.net/2023/Apr/25/dual-llm-pattern/ Read: full text.
- [Willison2025] Willison, S. (2025). The lethal trifecta for AI agents: private data, untrusted content, and external communication. Blog, 16 June 2025. https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/ Read: full text.
- [Debenedetti2025] Debenedetti, E., et al. (2025). Defeating Prompt Injections by Design (CaMeL). Preprint. https://arxiv.org/abs/2503.18813 Read: full text (abstract, dual-LLM critique, limitations, side channels).
- [Beurer2025] Beurer-Kellner, L., et al. (2025). Design Patterns for Securing LLM Agents against Prompt Injections. Preprint. https://arxiv.org/abs/2506.08837 Read: full text (principle, six patterns, recommendations).
- [OWASP2025] OWASP Gen AI Security Project. LLM01:2025 Prompt Injection. https://genai.owasp.org/llmrisk/llm01-prompt-injection/ Read: full text.
- [AgentDojo2024] Debenedetti, E., Zhang, J., Balunovic, M., Beurer-Kellner, L., Fischer, M., and Tramèr, F. (2024). AgentDojo: A Dynamic Environment to Evaluate Prompt Injection Attacks and Defenses for LLM Agents. https://arxiv.org/abs/2406.13352 Read: full text (abstract, intro, defenses section).
- [InjecAgent2024] Zhan, Q., Liang, Z., Ying, Z., and Kang, D. (2024). InjecAgent: Benchmarking Indirect Prompt Injections in Tool-Integrated Large Language Model Agents. https://arxiv.org/abs/2403.02691 Read: abstract, intro, results excerpt.
- [BIPIA2023] Yi, J., Xie, Y., Zhu, B., Kiciman, E., Sun, G., Xie, X., and Wu, F. (2023). Benchmarking and Defending Against Indirect Prompt Injection Attacks on Large Language Models. https://arxiv.org/abs/2312.14197 Read: abstract and analysis excerpts.
- [Hines2024] Hines, K., Lopez, G., Hall, M., Zarfati, F., Zunger, Y., and Kıcıman, E. (2024). Defending Against Indirect Prompt Injection Attacks With Spotlighting. Preprint (Microsoft). https://arxiv.org/abs/2403.14720 Read: full text (method and results).
- [Nasr2025] Nasr, M., Carlini, N., Sitawarin, C., Schulhoff, S. V., Hayes, J., Ilie, M., Pluto, J., Song, S., Chaudhari, H., Shumailov, I., Thakurta, A., Xiao, K. Y., Terzis, A., and Tramèr, F. (2025). The Attacker Moves Second: Stronger Adaptive Attacks Bypass Defenses Against LLM Jailbreaks and Prompt Injections. Preprint. https://arxiv.org/abs/2510.09023 Read: full text (abstract, per-defense results, lessons).
- [GraySwan2026] Dziemian, M., Lin, M., Fu, X., Nowak, M., et al. (31 authors, Gray Swan and collaborators) (2026). How Vulnerable Are AI Agents to Indirect Prompt Injections? Insights from a Large-Scale Public Competition. Preprint. https://arxiv.org/abs/2603.15714 Read: abstract and search excerpts (method, limits).
- [AnthropicPI2025] Anthropic (2025). Mitigating the risk of prompt injections in browser use. Vendor research post, 24 Nov 2025. https://www.anthropic.com/research/prompt-injection-defenses Read: search excerpt.
- [Omnipedia2025] Omnipedia. Mitigating the risk of prompt injections in browser use (summary page). https://omnipedia.tech/library/mitigating-the-risk-of-prompt-injections-in-browser-use Read: search excerpt. Secondary summary of Anthropic's chart values and the August 2025 pilot figures.
- [Primores2026] Primores. Is it safe to connect ChatGPT or Claude to your Gmail? Practitioner blog. https://primores.org/blog/is-it-safe-to-connect-ai-to-email/ Read: search excerpt. Secondary summary of vendor statements.
- [ZNova2026] ZNova. Prompt Injection Attack Examples: Real Cases From 2026. Practitioner blog. https://znovastudio.com/insights/prompt-injection-attack-examples Read: search excerpt. Secondary summary of vendor statements.

**Search strategy**

- [Benham2019] Benham, R., Mackenzie, J., Moffat, A., and Culpepper, J. S. (2019). Boosting Search Performance Using Query Variations. ACM Transactions on Information Systems 37(4), Article 41 (unconfirmed; Crossref lists pages 1-25 and gives no article number). https://doi.org/10.1145/3345001 Full text: https://dl.acm.org/doi/fullHtml/10.1145/3345001 Open-access copy: https://arxiv.org/abs/1811.06147 Read: search excerpt of the full text (abstract, setup, Table 2).
- [BenhamThesis] Benham, R. (2018). Improving Recall In Text Retrieval Using Rank Fusion. Honours thesis, RMIT University, 5 June 2018. https://rodgerbenham.github.io/thesis.pdf Read: abstract only.
- [Bailey2017] Bailey, P., Moffat, A., Scholer, F., and Thomas, P. (2017). Retrieval Consistency in the Presence of Query Variations. SIGIR 2017, pages 395-404. ACM. https://doi.org/10.1145/3077136.3080839 Read: abstract only.
- [BrowseComp2025] Wei, J., Sun, Z., Papay, S., McKinney, S., Han, J., Fulford, I., Chung, H. W., Passos, A. T., Fedus, W., and Glaese, A. (2025). BrowseComp: A Simple Yet Challenging Benchmark for Browsing Agents. Preprint (OpenAI). https://arxiv.org/abs/2504.12516 Read: full text (abstract, human and model results).
- [AnthropicResearch2025] Anthropic (2025). How we built our multi-agent research system. Vendor engineering blog. https://www.anthropic.com/engineering/multi-agent-research-system Read: full text (key sections).

**Keeping sources alive**

- [RobustLinks] Robust Links specification (Hiberlink and Memento projects). https://robustlinks.org/spec/ Mirror: https://mementoweb.org/robustlinks/spec/ Read: search excerpt of the specification.
- [PermaGuide] Stanford Law School library. Perma.cc for Journal Members. https://guides.law.stanford.edu/perma Read: search excerpt.
- [WikiArchive] Wikipedia. Wikipedia:Archiving a source. https://en.wikipedia.org/wiki/Wikipedia:Archiving_a_source Read: search excerpt.
- [IABlog2018] Graham, M. (2018). More than 9 million broken links on Wikipedia are now rescued. Internet Archive Blogs, 1 Oct 2018. https://blog.archive.org/2018/10/01/more-than-9-million-broken-links-on-wikipedia-are-now-rescued/ Read: search excerpt.

## Dropped sources

- [AGENTiGraph2024] Zhao, X., Blum, M., Yang, R., Yang, B., Márquez Carpintero, L., Pina-Navarro, M., Wang, T., Li, X., Li, H., Fu, Y., Wang, R., Zhang, J., and Li, I. (2024). AGENTiGraph: An Interactive Knowledge Graph Platform for LLM-based Chatbots Utilizing Private Data. Preprint. https://arxiv.org/abs/2410.11531 Why dropped: the paper is real, and the 78.45% (GPT-4) and 83.34% (GPT-4o) figures in its Table 1 are zero-shot accuracy at sorting user queries into seven task types. Topic 4.3 used them as prerequisite-link prediction accuracy, which the paper does not report. The sentence that cited it is gone and the 4.3 heading now reads "roughly 75%".

## Citation check

- Date: 2026-11-22.
- Checker: a second agent, working independently of the one that wrote the file.
- Method: every source-list entry was looked up again today. DOIs went to the Crossref API, arXiv papers to the arXiv API, and every other link got an HTTP GET with the status and page title recorded. Where a publisher answered with a bot wall (403, 429 or a challenge page), the paper was confirmed another way: Crossref, OpenAlex, PubMed Central or Europe PMC, an author-hosted or arXiv copy, or a Wayback Machine copy of the same page. For every bullet in Top findings, and for 63 further numbers in Findings by topic, the full text of the cited source was fetched and searched for the number and for what it measures.
- Retractions: none found. Crossref lists one correction notice, for [Jones2016], and it fixes an author affiliation only.
- Injection handling: no fetched page addressed an instruction to the checker. Example attack strings printed inside [Nasr2025], [Debenedetti2025], [AgentDojo2024], [Willison2023] and [OWASP2025] were read as quoted data and nothing was acted on.

**Counts**

- Sources swept: 95
- Sources fine: 68
- Sources corrected (a wrong or missing author, year, title, venue or link fixed): 26
- Sources dropped: 1
- Top-finding claims checked: 74
- Top-finding claims confirmed on the primary source: 70
- Top-finding claims confirmed on a secondary page only: 1 (the sentence that API-level quote extraction "removes fabricated references, not misread ones" is on Csorba's blog, which summarises Anthropic's Citations documentation)
- Top-finding claims unconfirmed: 0
- Top-finding claims wrong: 3 (in each the number is right and the wording around it claimed something the source does not say; see findings 16, 20 and 23 below)
- Further numbers checked in Findings by topic: 63, counting several figures from one table as one. 54 confirmed on the primary source, 4 on a secondary page only (each already labelled that way in the text), 1 unconfirmed, 4 wrong.

**What the five most-used numbers measure**

- About half of AI-search sentences fully supported [Liu2023]: confirmed. 51.5% is citation recall, the share of sentences needing verification that human annotators judged fully supported by the citations attached to them, averaged over Bing Chat, NeevaAI, perplexity.ai and YouChat in early 2023. The 74.5% is citation precision, the share of citations that support their sentence.
- Deep-research citation error of 10 to 22% [DRBench2025]: confirmed as one minus "citation accuracy" (90.24% for Perplexity down to 77.96% for OpenAI). An LLM judge decides whether each extracted statement-URL pair is supported, the score is averaged per task, and a task with no citable statement scores zero. It is a judged estimate over 100 tasks. The judge (Gemini 2.5 Flash) matched human labels on 96% of "support" and 92% of "not support" cases in the paper's own spot check of 100 pairs.
- Over 90% bypass under adaptive attack [Nasr2025]: confirmed. It is the attack success rate of the strongest adaptive attack against each of 12 defenses, "above 90% for most". The 12 cover prompting, training, filtering-model and secret-knowledge defenses against both jailbreaks and prompt injection, each on its own benchmark, so the figures are not comparable across defenses. For filter-style detectors alone the paper reports over 90% for Protect AI, PromptGuard and Model Armor and 71% for PIGuard. The paper is a preprint under review.
- Model-to-expert credibility agreement of rho 0.50 [Yang2023]: confirmed. It is the average Spearman rank correlation between each of nine models' ratings of a news domain, given from memory, and an aggregated expert rating, over the domains a model agreed to rate. It is a ranking agreement, with page content and search left out of the task.
- Link rot of about a quarter within a decade [Pew2024], [Zittrain2021]: the figures are confirmed and the phrase was loose. Pew's 25% pools pages sampled in every year from 2013 to 2023, so their ages run from zero to ten years; for pages a full decade old Pew found 38%. Pew counts only hard failures (nine error codes or a dead domain), so both are floors. The New York Times figure is 25% of all deep links published from 1996 to mid-2019, 43% for links from 2008.

**Corrections to claims**

- [JillWatson2024], finding 16, topic 5.5 and the retrieval disagreement. Was: "Grounding a teaching assistant in vetted courseware raised its pass rate from 31% to 77%" with "same model and files". Now: Jill Watson passed 76.7% and OpenAI's generic Assistants file-retrieval service passed 31.3% on the same underlying model, so the gap is between two retrieval designs, both fed course material. The paper names the shared model and only implies the files were the same. (Wrong as worded.)
- [Hines2024], [Nasr2025], finding 20 and topic 7.3. Was: datamarking cut attack success to about 3% and "under adaptive attack the same defense was broken more than 95% of the time". Now: the defense broken at over 95% was a Spotlighting variant that the attacking paper describes as delimiter marking, so the datamarking variant itself is not shown to have been tested. (Wrong as worded.)
- [BrowseComp2025], finding 23 and topic 8.3. Was: "a model with one search step 1.9%" and "a model with a single search step found almost nothing". Now: "GPT-4o with browsing 1.9%". The paper does not describe that model as limited to one search. (Wrong as worded.)
- [AGENTiGraph2024], topic 4.3. Was: models predict prerequisite links "at roughly 75 to 83% accuracy", citing 78.45% and 83.34% from arXiv 2410.11531. Now: "roughly 75%", resting on [Yang2024Graph] alone, because the 2410.11531 figures are query-classification accuracy. (Wrong.)
- [Tithi2025], topic 1.7. Was: "two experts rated 240 LLM-written explanations". Now: 210, as in the published version; the v1 preprint said 240 in its text and 210 in its table caption. (Wrong.)
- [Nasr2025], [AgentDojo2024], disagreement on prompt-level defenses. Was: "The same defenses were broken at over 95% by adaptive attackers". Now: Spotlighting (delimiter form) and prompt repetition at over 95%, the same injection detector at over 90%, and tool filtering was not among the twelve defenses tested. (Wrong in part.)
- [Pew2024], topic 2.2 heading. Was: "roughly a quarter of pages vanish within a decade". Now: "a quarter of pages sampled across 2013 to 2023 were gone by late 2023, 38% of the ones a full decade old". The evidence bullet also now says the 25% is pooled over snapshots aged zero to ten years. (Loose wording.)
- [Pew2024], topic 2.2. Was: the 50,000-page sample and the 11% of reference links were credited to a Computerworld summary. Now: credited to the Pew report, where both appear.
- [Jones2016], finding 7. Was: "over 75% of web references in science papers had drifted". Now: over 75% of the references that had a representative archived copy and a page still live to compare (184,065 of 241,091).
- [Nasr2025], topic 7.3 heading. Was: "Twelve published defenses that reported near-zero attack success". Now: "Twelve published defenses, most of which had reported near-zero attack success", matching the abstract.
- [GraySwan2026], finding 19 and topic 7.1. Was: "in a 2026 public competition", "sponsored by UK AISI and frontier labs". Now: "a public competition reported in 2026", "run with the UK AI Security Institute, US CAISI and frontier labs". The paper is dated March 2026 and gives no dates for the three-week competition.
- [Wu2024], finding 17 and topic 5.2. Was: preprint numbers only. Now: the 2025 Nature Communications version is noted beside them (seven LLMs, 800 questions, about 58,000 pairs, 88.7% verifier agreement on 400 pairs, 86.1% between doctors, 55% of GPT-4o web-search responses fully supported). The 50% to 90% headline range is the same in both versions.
- [Yang2023], topic 1.5. Was: the two correlations stated bare, and the paper called a preprint. Now: the text says they are computed over sources the models agreed to rate (3,077 rated by all nine), and that the arXiv record lists publication at WebSci '25.
- [DRBench2025], topic 5.2. Was: citation accuracy "judged by an LLM". Now: the text also says how the score is built (share of statement-URL pairs judged supported, averaged per task, zero for a task with no citable statement).
- [McNamaraKintsch1996], topic 3.3. Was: the sorting result "failed to replicate" in Experiment 2. Now: the same sentence adds that in Experiment 2 high-knowledge readers still did better on open-ended questions after the low-coherence text, as the abstract says.
- Recollection flags cleared after confirming authors: [TSAR2024] (Trott and Rivière), [Shao2024], [Wanner2024], [Core2024], [SelfCheck2023], [Liang2017].
- The five "search summary" numbers are all on their primary sources: the Rubric II descriptors [AchieveOER], 23 to 34% of variance [Crossley2022], the LLM-human QWK values [Tithi2025], 76.35% [Jones2016], and 20 samples [SelfCheck2023]. The "search summary" wording in the topic text was left as written, except in topic 1.7 where the corrected sentence now cites the paper's own table.
- Finding 10, [McNamara1996]: the low-knowledge means (.41 and .27) are printed in Table 2 of the publisher PDF. The row of high-knowledge means is missing from that PDF's text layer, and .55 and .61 were recomputed from the four question-type cells in the same table (.545 and .605).

**Corrections to source-list entries**

- [MathExplain2024]. Was: key [MathExplain2023], "Anonymous-author copy (2023)". Now: Worden, Croteau, Cheng, McReynolds and Heffernan, dated 2024 on the NSF repository record; key renamed in all three places it appears.
- [Breakstone2024]. Was: "Breakstone, J., McGrew, S., Smith, M., et al., Wineburg, S." Now: the four named authors and no others, with the book title (Handbook of Children and Screens, pages 485-489).
- [CutTheCRAAP]. Was: "Author not captured ... (year not captured)". Now: Blizzard, K.; the slides carry no date and cite a February 2021 source.
- [AchieveOER]. Was: titled "Rubrics for Evaluating Open Education Resource (OER) Objects". Now: the linked document's own title, "Comprehensive OER Evaluation Tool" (ISKME, 2017), adapted from the Achieve rubrics.
- [IATUL2003]. Was: "Author not captured". Now: Orhun, E.
- [Yang2023]. Was: "Preprint". Now: published in the Proceedings of the 17th ACM Web Science Conference (WebSci '25), pages 127-137, DOI 10.1145/3717867.3717903, per its arXiv record and Crossref. Key kept because the arXiv paper dates from 2023.
- [Thomas2024]. Was: "Preprint as read". Now: also published at SIGIR 2024, pages 1930-1940, DOI 10.1145/3626772.3657707.
- [Tithi2025]. Was: "Preprint", linked to v1. Now: published in Computers and Education: Artificial Intelligence 9, 100490, DOI 10.1016/j.caeai.2025.100490; arXiv v2 is that journal version.
- [SelfExplain2025]. Was: "Authors not captured". Now: Yamauchi, Flanagan, Dai, Kita and Ogata, in the Proceedings of the 1st International Conference on Learning Evidence and Analytics.
- [Zittrain2021]. Was: one PDF link, which now answers scripts with a bot check. Now: the DASH record page is added beside it.
- [Bevendorff2024]. Was: a proceedings listing only. Now: the chapter DOI 10.1007/978-3-031-56063-7_4, pages 56-71, and the authors' open-access PDF.
- [TSAR2024]. Was: "Authors not captured". Now: Trott, S., and Rivière, P. D.
- [PRET2018]. Was: "(2018, year per the citations in the text)", publisher only. Now: 2018 confirmed on Crossref, in the Proceedings of CLiC-it 2018, pages 14-20, with the DOI and an open CEUR copy.
- [UPreG2021]. Was: "Authors not captured". Now: Sabnis, Abhinav, Subramanian, Dubey and Bhat.
- [GPTConcepts2025]. Was: "Authors not captured". Now: Yang, Ren, Gu, He, Ma and Konomi.
- [ESCOPrereq2025]. Was: "Authors not captured". Now: Le, N. L., and Abel, M.-H.
- [Wu2024]. Was: "Preprint". Now: the entry also gives the published version, Nature Communications 16, 3615 (2025), under a new title and with a larger study. Key kept because the numbers in this file are the 2024 preprint's.
- [DRBench2025]. Was: "Authors not captured". Now: Du, Xu, Zhu, Wang and Mao.
- [FaithBench2025]. Was: "Vectara and collaborators". Now: Bao, F. S., and fifteen co-authors, named in full.
- [Kakar2024]. Was: a literatumonline.com link that returns a bot check. Now: the DOI link, ITS 2024, LNCS 14798, pages 78-90, with the old link kept beside it.
- [VeriScore2024]. Was: "Authors not captured". Now: Song, Kim and Iyyer.
- [CQCRAG2026]. Was: "Authors not captured". Now: Sun, Liu and Shao.
- [GraySwan2026]. Was: "Gray Swan and collaborators". Now: Dziemian, Lin, Fu, Nowak, et al. (31 authors).
- [Benham2019]. Was: "Article 41". Now: "Article 41 (unconfirmed)", and the arXiv copy (1811.06147) is added as the open-access link.
- [BenhamThesis]. Was: "Improving Recall In Text Retrieval (thesis)", undated. Now: "Improving Recall In Text Retrieval Using Rank Fusion", Honours thesis, RMIT University, 5 June 2018.
- [Bailey2017]. Was: "Authors not captured". Now: Bailey, Moffat, Scholer and Thomas, SIGIR 2017, pages 395-404.

**Dropped sources**

- [AGENTiGraph2024]: moved to `## Dropped sources`. The paper is real and its numbers measure query classification, so the prerequisite-accuracy claim that rested on it was removed.

**Claims left unconfirmed**

- [Benham2019]: the article number "Article 41". Crossref and OpenAlex list pages 1-25 with no article number, and the ACM page returned a bot wall. Marked "(unconfirmed)" in topic 8.1 and in the source list.
- [Wineburg2022]: the effect size of the lateral-reading lessons. The file already says so. The abstract, read on the ERIC record, confirms the design, the sample sizes and the significant gain.
- [Contentifai2025], [Maxintel2026]: the Ahrefs figure (74% of about 900,000 new pages) and the Originality.ai figures (2.27% rising to 19.56%). Both secondary pages say what the file reports; the primary studies were not opened. Already labelled in topic 2.5.
- [Omnipedia2025]: the chart values 1.4%, 7.6% and 8.0% and the August 2025 pilot figures (23.6% and 11.2% on 123 cases). The summary page says what the file reports. The Anthropic post itself [AnthropicPI2025] was opened and confirms the 1% wording, the 100 attempts per environment and the "meaningful risk" quotation. Already labelled in topic 7.3.
- [Primores2026], [ZNova2026]: the OpenAI, Microsoft and Google quotations and the 2026 OWASP ranking. The two blogs carry them as quoted; the vendor pages were not opened. Already labelled in topic 7.3.
- [Csorba2026], [Botnet2026]: the wording of Anthropic's guidance and the Citations launch figures (up to 15% better recall accuracy, Endex 10% to 0%). The two practitioner pages carry them; Anthropic's pages were not opened. Already labelled in topic 6.4.
- [CutTheCRAAP], [AIALOE2024]: the year of each. Neither page shows a publication date.
- [JillWatson2024]: that OpenAI Assistants was given the same course files as Jill Watson. The paper implies it; the text now says so.

## Breadcrumbs

- severity: low. category: cap-hit. note: EdReports, Lexile validity and a legal source on quotation were named leads and were not fetched, so they are marked unconfirmed.
- severity: medium. category: vetting-fail. note: Several numbers rest on secondary summaries (Ahrefs 74%, Originality.ai 19.56%, OpenAI, Microsoft and Google quotations, Anthropic chart values and Citations figures); each is labelled where it appears.
- severity: low. category: other. note: A few author lists and venues were not visible on the pages read and are flagged as recollection in the text or source list.
- severity: low. category: scope-drift. note: The source list runs past the 60-source target because secondary pages that supplied a number are listed as their own entries.
- severity: low. category: injection-refused. note: No fetched page addressed instructions to me; the example attack strings quoted on the Willison and OWASP pages were treated as data and nothing was acted on.

