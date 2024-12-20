use std::{fmt::Display, ops::Add, rc::Rc};

/// Indicates a position within a source file.
#[derive(Clone, Debug)]
pub struct Pos {
    pub filename: Rc<String>,
    pub linenum: u32,
    pub charpos: u16,
}

impl Pos {
    #[inline]
    pub fn new(filename: Rc<String>, linenum: u32, charpos: u16) -> Self {
        Self {
            filename,
            linenum,
            charpos,
        }
    }

    /// Return a clone of this Pos, except with the given character position.
    pub fn with_charpos(&self, charpos: u16) -> Self {
        Self::new(self.filename.clone(), self.linenum, charpos)
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}:{}:{}",
            self.filename, self.linenum, self.charpos
        ))
    }
}

/// An internal error type.
#[derive(Debug)]
pub struct Error {
    pos: Pos,
    text: String,
}

impl Error {
    pub fn new(pos: Pos, text: String) -> Self {
        Self { pos, text }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}", self.pos, self.text))
    }
}

impl std::error::Error for Error {}

/// Indicates a source file.
pub struct Src {
    pub filename: Rc<String>,
    pub text: String,
}

impl Src {
    pub fn new(filename: String, text: String) -> Self {
        Self {
            filename: Rc::new(filename),
            text,
        }
    }
}
