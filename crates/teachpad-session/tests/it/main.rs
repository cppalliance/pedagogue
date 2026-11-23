//! Session integration tests through the public API alone, one module per
//! side of a session: the host that writes a lesson and hears the student,
//! the reader that follows the session with a cursor, and the frames the
//! UI sends for the student.

#[path = "../common/mod.rs"]
mod common;

mod host;
mod reader;
mod student_frames;
