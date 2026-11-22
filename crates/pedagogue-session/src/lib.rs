//! The lesson model, the chat transcript and the session handle of Pedagogue.
//!
//! A host fills textbook pages through page commands, talks with the student
//! through the chat, and hears back what the student did. This crate holds
//! that state and its rules, and it is the one crate a host depends on.
//! ARCHITECTURE.md at the repository root holds the code map.

#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the session handle that applies commands to the lesson is yet to land"
    )
)]
mod lesson;
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the session handle that hands out these sentences is yet to land"
    )
)]
mod wording;
