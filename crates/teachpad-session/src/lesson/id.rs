//! Ids of the parts of a lesson: its blocks and its pages.

use std::fmt;
use std::num::NonZeroU32;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

/// The id of one block, unique across a lesson.
///
/// Ids run `b1`, `b2` and so on, in the order the blocks were added. The
/// receipt of a command names the id of the block it added, and that id is
/// how a later command or a student answer points at a quiz.
///
/// # Examples
///
/// ```
/// use teachpad_session::BlockId;
///
/// let id: BlockId = serde_json::from_str("\"b7\"")?;
/// assert_eq!(id.to_string(), "b7");
/// # Ok::<(), serde_json::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(NonZeroU32);

impl BlockId {
    /// The id of the first block of a lesson.
    pub(crate) const FIRST: Self = Self(NonZeroU32::MIN);

    /// Returns the id of the block added after this one.
    pub(crate) fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }

    /// Reads an id from its text form, such as `b7`.
    fn parse(text: &str) -> Option<Self> {
        let digits = text.strip_prefix('b')?;
        let is_plain = digits.bytes().all(|byte| byte.is_ascii_digit());
        if !is_plain || digits.starts_with('0') {
            return None;
        }
        digits.parse().ok().map(Self)
    }
}

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "b{}", self.0)
    }
}

impl Serialize for BlockId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for BlockId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        Self::parse(&text).ok_or_else(|| {
            de::Error::custom(format_args!(
                "`{text}` is not a block id; ids look like `b7`"
            ))
        })
    }
}

/// The number of one page. Pages are numbered from 1.
///
/// # Examples
///
/// ```
/// use teachpad_session::PageNumber;
///
/// let page: PageNumber = serde_json::from_str("2")?;
/// assert_eq!(page.to_string(), "2");
/// assert!(serde_json::from_str::<PageNumber>("0").is_err());
/// # Ok::<(), serde_json::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PageNumber(NonZeroU32);

impl PageNumber {
    /// The number of the first page of a lesson.
    pub(crate) const FIRST: Self = Self(NonZeroU32::MIN);

    /// Returns the number of the page after this one.
    pub(crate) fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

impl fmt::Display for PageNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn block_ids_run_from_b1() {
        let first = BlockId::FIRST;
        assert_eq!(first.to_string(), "b1");
        assert_eq!(first.next().to_string(), "b2");
    }

    #[test]
    fn a_block_id_travels_as_its_text() {
        let id = BlockId::FIRST.next();
        let wire = serde_json::to_value(id).expect("an id serializes");
        assert_eq!(wire, json!("b2"));
        let back: BlockId = serde_json::from_value(wire).expect("the text is an id");
        assert_eq!(back, id);
    }

    #[test]
    fn text_that_is_not_an_id_is_refused_by_name() {
        for text in ["7", "b", "b0", "b07", "b+7", "b-1", "B7", "b7 ", "c7"] {
            let error =
                serde_json::from_value::<BlockId>(json!(text)).expect_err("the text is not an id");
            assert_eq!(
                error.to_string(),
                format!("`{text}` is not a block id; ids look like `b7`"),
            );
        }
    }

    #[test]
    fn page_numbers_run_from_1_and_travel_as_numbers() {
        let first = PageNumber::FIRST;
        assert_eq!(first.to_string(), "1");
        assert_eq!(first.next().to_string(), "2");
        assert_eq!(
            serde_json::to_value(first.next()).expect("serializes"),
            json!(2)
        );
        let back: PageNumber = serde_json::from_value(json!(2)).expect("2 is a page number");
        assert_eq!(back, first.next());
        assert!(serde_json::from_value::<PageNumber>(json!(0)).is_err());
    }
}
