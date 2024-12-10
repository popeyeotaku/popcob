use crate::src;

use super::Kw;

/// An actual input token.
pub struct Token {
    tkn_type: Tkn,
    pos: src::Pos,
    value: Option<String>,
}

impl Token {
    /// Construct a token without a value.
    pub fn new(tkn: Tkn, pos: src::Pos) -> Self {
        Self {
            tkn_type: tkn,
            pos,
            value: None,
        }
    }

    /// Construct a token with a value.
    pub fn valued(tkn: Tkn, pos: src::Pos, value: String) -> Self {
        Self {
            tkn_type: tkn,
            pos,
            value: Some(value),
        }
    }

    /// Return a flag if this token matches the `Tkn` enum type given.
    #[inline]
    pub fn tkn(&self, other: Tkn) -> bool {
        self.tkn_type == other
    }
}

/// The type of a token.
#[derive(Copy, Clone, PartialEq)]
pub enum Tkn {
    Literal,
    Word,
    Dot,
    Paragraph,
    Kw(Kw),
}
