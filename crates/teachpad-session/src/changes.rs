//! Versions of a session, and the changes between them.

use crate::frames::Frame;

/// A point in the history of a session.
///
/// The version of a session rises with every change. A reader keeps the
/// version it last saw as its cursor, and asks for what changed since.
///
/// # Examples
///
/// ```
/// use teachpad_session::{Session, Version};
///
/// let (session, _events) = Session::new();
/// let seen = session.changes_since(Version::ZERO).version();
/// session.assistant_message("Read this section, then try the quiz.");
/// assert!(session.changes_since(seen).version() > seen);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Version(u64);

impl Version {
    /// The cursor of a reader that has seen nothing yet.
    pub const ZERO: Self = Self(0);

    /// Returns the version after this one.
    pub(crate) fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

/// What changed in a session since a cursor.
///
/// # Examples
///
/// ```
/// use teachpad_session::{Session, Version};
/// use serde_json::json;
///
/// let (session, _events) = Session::new();
/// session.apply("new_page", &json!({}))?;
/// let changes = session.changes_since(Version::ZERO);
/// // The new page, then the chat.
/// assert_eq!(changes.frames().len(), 2);
/// assert!(session.changes_since(changes.version()).frames().is_empty());
/// # Ok::<(), teachpad_session::CommandError>(())
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Changes {
    version: Version,
    frames: Vec<Frame>,
}

impl Changes {
    /// Collects the frames that lead up to a version.
    pub(crate) fn new(version: Version, frames: Vec<Frame>) -> Self {
        Self { version, frames }
    }

    /// Returns the version these changes lead up to. It is the reader's
    /// next cursor.
    ///
    /// # Examples
    ///
    /// ```
    /// use teachpad_session::{Session, Version};
    ///
    /// let (session, _events) = Session::new();
    /// let cursor = session.changes_since(Version::ZERO).version();
    /// assert!(cursor > Version::ZERO);
    /// ```
    #[must_use]
    pub fn version(&self) -> Version {
        self.version
    }

    /// Returns the frames to send: each page that changed, in order, then
    /// the chat if it changed.
    ///
    /// # Examples
    ///
    /// ```
    /// use teachpad_session::{Session, Version};
    ///
    /// let (session, _events) = Session::new();
    /// let changes = session.changes_since(Version::ZERO);
    /// let wire = serde_json::to_value(changes.frames())?;
    /// assert_eq!(wire[0]["type"], "chat");
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    #[must_use]
    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }
}
