//! The quiz rules, as plain functions.
//!
//! These are first rules, to be tuned in user trials, and this file is their
//! one home. Each function reads a quiz with the attempts it has seen. It
//! judges a new attempt, or it says what the page shows next. The page is
//! worked out from the attempts every time, so a rule change here needs no
//! change to what is stored.
//!
//! The rules today:
//!
//! - A wrong pick is disabled, with its feedback, and the student picks
//!   again from the options that remain. The right answer stays hidden
//!   until the student picks it.
//! - On a select-all quiz, one press submits the whole selection. Wrong
//!   picks are disabled, right picks stay ticked, and the page says when a
//!   right option is still unpicked.
//! - A free-text answer waits for the host's verdict. A verdict of
//!   incorrect lets the student answer again.

use serde::Serialize;

use super::quiz::{ChoiceAttempt, ChoiceQuiz, ResponseAttempt, ResponseQuiz, Verdict};
use super::refusal::Fault;

/// How the page draws one option of a choice quiz.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum OptionState {
    /// The student may pick it.
    Open,
    /// A wrong pick: dimmed, tagged, and out of the next attempt.
    Disabled,
    /// A right option of a solved quiz, tagged as correct.
    Right,
}

/// Whether a choice quiz still takes attempts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChoiceStatus {
    /// The quiz takes attempts.
    Open,
    /// The student found the answer.
    Solved,
}

/// What the page shows for a choice quiz.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ChoiceShown<'a> {
    pub(crate) status: ChoiceStatus,
    /// Whether the latest attempt left a right option unpicked.
    pub(crate) missing: bool,
    /// One entry for each option, in the order of the quiz.
    pub(crate) options: Vec<OptionShown>,
    /// The feedback the latest attempt released.
    pub(crate) feedback: Vec<&'a str>,
}

/// What the page shows for one option.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OptionShown {
    pub(crate) state: OptionState,
    /// Whether the option's box is ticked.
    pub(crate) checked: bool,
}

/// Says what the page shows for a choice quiz after the attempts so far.
pub(crate) fn show_choice(quiz: &ChoiceQuiz) -> ChoiceShown<'_> {
    let latest = quiz.attempts.last();
    let solved = latest.is_some_and(|attempt| attempt.correct);
    let in_latest = |index: usize| latest.is_some_and(|attempt| attempt.picks.contains(&index));

    let options = quiz
        .options
        .iter()
        .enumerate()
        .map(|(index, option)| {
            let was_picked = quiz
                .attempts
                .iter()
                .any(|attempt| attempt.picks.contains(&index));
            let state = if solved && option.correct {
                OptionState::Right
            } else if was_picked && !option.correct {
                OptionState::Disabled
            } else {
                OptionState::Open
            };
            // A right pick stays ticked. On a quiz with one right option
            // that is the solving pick alone, so an open quiz shows no tick
            // and gives nothing away.
            let checked = option.correct && (solved || (quiz.multiple && in_latest(index)));
            OptionShown { state, checked }
        })
        .collect();

    let missing = quiz.multiple
        && !solved
        && latest.is_some()
        && quiz
            .options
            .iter()
            .enumerate()
            .any(|(index, option)| option.correct && !in_latest(index));

    // A wrong attempt releases the feedback of its wrong picks. The solving
    // attempt releases the feedback of its picks, which are all right.
    let feedback = latest.map_or_else(Vec::new, |attempt| {
        attempt
            .picks
            .iter()
            .filter_map(|&index| quiz.options.get(index))
            .filter(|option| attempt.correct || !option.correct)
            .filter_map(|option| option.feedback.as_deref())
            .collect()
    });

    ChoiceShown {
        status: if solved {
            ChoiceStatus::Solved
        } else {
            ChoiceStatus::Open
        },
        missing,
        options,
        feedback,
    }
}

/// Judges a new attempt at a choice quiz.
///
/// The picks are option indexes in any order. A quiz with one right option
/// takes one pick, and a select-all quiz takes one or more.
pub(crate) fn judge_picks(quiz: &ChoiceQuiz, picks: &[usize]) -> Result<ChoiceAttempt, Fault> {
    let shown = show_choice(quiz);
    if shown.status == ChoiceStatus::Solved {
        return Err(Fault::Solved);
    }
    let mut picks = picks.to_vec();
    picks.sort_unstable();
    picks.dedup();
    if picks.is_empty() {
        return Err(Fault::NoPick);
    }
    if !quiz.multiple && picks.len() > 1 {
        return Err(Fault::OnePick);
    }
    for &pick in &picks {
        match shown.options.get(pick) {
            None => return Err(Fault::NoSuchOption { pick }),
            Some(option) if option.state == OptionState::Disabled => {
                return Err(Fault::OptionDisabled { pick });
            }
            Some(_) => {}
        }
    }
    let correct = quiz
        .options
        .iter()
        .enumerate()
        .all(|(index, option)| option.correct == picks.contains(&index));
    Ok(ChoiceAttempt { picks, correct })
}

/// Where a free-text quiz stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ResponseStatus {
    /// The quiz waits for an answer.
    Open,
    /// The answer waits for the host's verdict.
    Submitted,
    /// The host marked the answer correct.
    Right,
    /// The host marked the answer incorrect, and the student may try again.
    Wrong,
}

/// Says where a free-text quiz stands after the answers so far.
pub(crate) fn response_status(quiz: &ResponseQuiz) -> ResponseStatus {
    let Some(latest) = quiz.attempts.last() else {
        return ResponseStatus::Open;
    };
    match latest.mark.as_ref().map(|mark| mark.verdict) {
        None => ResponseStatus::Submitted,
        Some(Verdict::Correct) => ResponseStatus::Right,
        Some(Verdict::Incorrect) => ResponseStatus::Wrong,
    }
}

/// Judges whether a free-text quiz takes a new answer now.
pub(crate) fn accept_text(quiz: &ResponseQuiz, text: &str) -> Result<ResponseAttempt, Fault> {
    match response_status(quiz) {
        ResponseStatus::Submitted => return Err(Fault::AwaitingVerdict),
        ResponseStatus::Right => return Err(Fault::Solved),
        ResponseStatus::Open | ResponseStatus::Wrong => {}
    }
    let answer = text.trim();
    if answer.is_empty() {
        return Err(Fault::EmptyAnswer);
    }
    Ok(ResponseAttempt {
        answer: answer.to_owned(),
        mark: None,
    })
}

/// Says whether a free-text quiz holds an answer that a verdict can mark.
pub(crate) fn can_mark(quiz: &ResponseQuiz) -> bool {
    response_status(quiz) != ResponseStatus::Open
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::{Value, json};

    use crate::lesson::command;

    fn choice(multiple: bool, key: &[(bool, Option<&str>)]) -> ChoiceQuiz {
        let options: Vec<Value> = key
            .iter()
            .enumerate()
            .map(|(index, (correct, feedback))| {
                json!({ "text": format!("Option {index}"), "correct": correct, "feedback": feedback })
            })
            .collect();
        let args: command::PopQuiz = serde_json::from_value(
            json!({ "question": "Which?", "options": options, "multiple": multiple }),
        )
        .expect("the arguments have the right shape");
        ChoiceQuiz::try_from(args).expect("the quiz is valid")
    }

    /// Makes an attempt and records it when the rules accept it.
    fn attempt(quiz: &mut ChoiceQuiz, picks: &[usize]) -> Result<bool, Fault> {
        let judged = judge_picks(quiz, picks)?;
        let correct = judged.correct;
        quiz.record(judged);
        Ok(correct)
    }

    fn states(quiz: &ChoiceQuiz) -> Vec<(OptionState, bool)> {
        show_choice(quiz)
            .options
            .iter()
            .map(|option| (option.state, option.checked))
            .collect()
    }

    const OPEN: (OptionState, bool) = (OptionState::Open, false);
    const TICKED: (OptionState, bool) = (OptionState::Open, true);
    const DISABLED: (OptionState, bool) = (OptionState::Disabled, false);
    const RIGHT: (OptionState, bool) = (OptionState::Right, true);

    fn single() -> ChoiceQuiz {
        choice(
            false,
            &[
                (false, Some("Wrong, because of zero.")),
                (true, Some("Right, because of one.")),
                (false, None),
            ],
        )
    }

    #[test]
    fn a_fresh_quiz_shows_every_option_open() {
        let quiz = single();
        let shown = show_choice(&quiz);
        assert_eq!(shown.status, ChoiceStatus::Open);
        assert_eq!(states(&quiz), [OPEN, OPEN, OPEN]);
        assert!(!shown.missing);
        assert!(shown.feedback.is_empty());
    }

    #[test]
    fn a_wrong_pick_is_disabled_and_the_right_answer_stays_hidden() {
        let mut quiz = single();
        assert_eq!(attempt(&mut quiz, &[0]), Ok(false));
        let shown = show_choice(&quiz);
        assert_eq!(shown.status, ChoiceStatus::Open);
        assert_eq!(states(&quiz), [DISABLED, OPEN, OPEN]);
        assert_eq!(shown.feedback, ["Wrong, because of zero."]);
    }

    #[test]
    fn a_disabled_option_cannot_be_picked_again() {
        let mut quiz = single();
        assert_eq!(attempt(&mut quiz, &[0]), Ok(false));
        assert_eq!(
            attempt(&mut quiz, &[0]),
            Err(Fault::OptionDisabled { pick: 0 })
        );
        assert_eq!(quiz.attempts(), 1);
    }

    #[test]
    fn the_right_pick_solves_the_quiz_and_keeps_earlier_wrong_picks_marked() {
        let mut quiz = single();
        assert_eq!(attempt(&mut quiz, &[2]), Ok(false));
        assert_eq!(attempt(&mut quiz, &[1]), Ok(true));
        let shown = show_choice(&quiz);
        assert_eq!(shown.status, ChoiceStatus::Solved);
        assert_eq!(states(&quiz), [OPEN, RIGHT, DISABLED]);
        assert_eq!(shown.feedback, ["Right, because of one."]);
        assert_eq!(attempt(&mut quiz, &[0]), Err(Fault::Solved));
    }

    #[test]
    fn a_wrong_pick_without_feedback_releases_none() {
        let mut quiz = single();
        assert_eq!(attempt(&mut quiz, &[2]), Ok(false));
        assert!(show_choice(&quiz).feedback.is_empty());
    }

    #[test]
    fn picks_that_fit_no_option_are_faults() {
        let mut quiz = single();
        assert_eq!(attempt(&mut quiz, &[]), Err(Fault::NoPick));
        assert_eq!(attempt(&mut quiz, &[0, 1]), Err(Fault::OnePick));
        assert_eq!(
            attempt(&mut quiz, &[3]),
            Err(Fault::NoSuchOption { pick: 3 })
        );
        assert_eq!(quiz.attempts(), 0);
    }

    fn select_all() -> ChoiceQuiz {
        choice(
            true,
            &[
                (true, None),
                (true, None),
                (false, Some("Not this one.")),
                (true, None),
            ],
        )
    }

    #[test]
    fn select_all_disables_wrong_picks_and_keeps_right_picks_ticked() {
        let mut quiz = select_all();
        assert_eq!(attempt(&mut quiz, &[2, 0]), Ok(false));
        let shown = show_choice(&quiz);
        assert_eq!(states(&quiz), [TICKED, OPEN, DISABLED, OPEN]);
        assert!(shown.missing);
        assert_eq!(shown.feedback, ["Not this one."]);
    }

    #[test]
    fn select_all_says_when_a_right_option_is_still_unpicked() {
        let mut quiz = select_all();
        assert_eq!(attempt(&mut quiz, &[0, 1]), Ok(false));
        let shown = show_choice(&quiz);
        assert_eq!(states(&quiz), [TICKED, TICKED, OPEN, OPEN]);
        assert!(shown.missing);
        assert!(shown.feedback.is_empty());
    }

    #[test]
    fn select_all_with_every_right_option_and_a_wrong_one_misses_nothing() {
        let mut quiz = select_all();
        assert_eq!(attempt(&mut quiz, &[0, 1, 2, 3]), Ok(false));
        let shown = show_choice(&quiz);
        assert_eq!(states(&quiz), [TICKED, TICKED, DISABLED, TICKED]);
        assert!(!shown.missing);
    }

    #[test]
    fn select_all_is_solved_by_exactly_the_right_options() {
        let mut quiz = select_all();
        assert_eq!(attempt(&mut quiz, &[0, 2]), Ok(false));
        assert_eq!(attempt(&mut quiz, &[3, 1, 0, 0]), Ok(true));
        let shown = show_choice(&quiz);
        assert_eq!(shown.status, ChoiceStatus::Solved);
        assert_eq!(states(&quiz), [RIGHT, RIGHT, DISABLED, RIGHT]);
        assert!(!shown.missing);
    }

    fn short_answer() -> ResponseQuiz {
        let args: command::ShortAnswer = serde_json::from_value(json!({ "question": "Why?" }))
            .expect("the arguments have the right shape");
        ResponseQuiz::try_from(args).expect("the quiz is valid")
    }

    #[test]
    fn a_free_text_answer_waits_for_its_verdict() {
        let mut quiz = short_answer();
        assert_eq!(response_status(&quiz), ResponseStatus::Open);
        assert!(!can_mark(&quiz));
        assert_eq!(accept_text(&quiz, "  \n"), Err(Fault::EmptyAnswer));

        quiz.record(accept_text(&quiz, " Because. ").expect("an open quiz takes an answer"));
        assert_eq!(response_status(&quiz), ResponseStatus::Submitted);
        assert!(can_mark(&quiz));
        assert_eq!(accept_text(&quiz, "Again."), Err(Fault::AwaitingVerdict));
    }

    #[test]
    fn a_verdict_of_incorrect_reopens_the_quiz_and_correct_closes_it() {
        let mut quiz = short_answer();
        quiz.record(accept_text(&quiz, "Because.").expect("an open quiz takes an answer"));
        quiz.mark(Verdict::Incorrect, "Think again.".to_owned());
        assert_eq!(response_status(&quiz), ResponseStatus::Wrong);

        quiz.record(
            accept_text(&quiz, "Because of scope.").expect("a wrong answer may be retried"),
        );
        quiz.mark(Verdict::Correct, "Exactly.".to_owned());
        assert_eq!(response_status(&quiz), ResponseStatus::Right);
        assert_eq!(accept_text(&quiz, "More."), Err(Fault::Solved));
        assert_eq!(quiz.attempts(), 2);
    }
}
