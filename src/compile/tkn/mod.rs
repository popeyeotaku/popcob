use crate::src::{Error, Src};
pub use kw::{Kw, KW_TAB};
pub use token::{Tkn, Token};

/// Handles various operations on the program tokens.
pub struct Tokens {
    tokens: Vec<Token>,
}

impl Tokens {
    pub fn new(sources: &[Src]) -> Result<Self, Error> {
        todo!()
    }
}

/// Allows consuming tokens via Tkn match.
pub struct Muncher<'a> {
    pub tokens: &'a [Token],
    i: usize,
}

impl<'a> Muncher<'a> {
    pub fn new(tkns: &'a [Token]) -> Self {
        Self { tokens: tkns, i: 0 }
    }

    /// Skip any leading paragraphs.
    pub fn skip_paragraphs(&mut self) {
        while self.grab(Tkn::Paragraph).is_some() {}
    }

    /// If we're out of tokens, issue an error.
    pub fn early_eof(&mut self) -> Result<(), Error> {
        if self.at_end() {
            Err(self.error("unexpected end of file".to_owned()))
        } else {
            Ok(())
        }
    }

    /// Return a flag for if we're out of tokens.
    #[inline]
    pub fn at_end(&self) -> bool {
        self.i >= self.tokens.len()
    }

    /// Skip the next i tokens.
    #[inline]
    pub fn advance(&mut self, i: usize) {
        self.i += i;
    }

    /// Return the current token index.
    #[inline]
    pub fn cur_index(&self) -> usize {
        self.i
    }

    /// If the current Token at the given position the given Tkn, return it.
    #[inline]
    fn peek_at(&mut self, i: usize, tkn: Tkn) -> Option<&'a Token> {
        self.tokens.get(i).filter(|t| t.tkn(tkn))
    }

    /// If the current Token matches the given Tkn, return it.
    pub fn peek(&mut self, tkn: Tkn) -> Option<&'a Token> {
        self.peek_at(self.i, tkn)
    }

    /// If the current Token matches the given Tkn, skip past it and
    /// return it.
    pub fn grab(&mut self, tkn: Tkn) -> Option<&'a Token> {
        if let Some(t) = self.peek(tkn) {
            self.i += 1;
            Some(t)
        } else {
            None
        }
    }

    /// If the next n Tokens match the given n Tkns, return them all.
    pub fn peekem(&mut self, tkns: &[Tkn]) -> Option<Vec<&'a Token>> {
        let mut tokens: Vec<&Token> = Vec::with_capacity(tkns.len());
        for (j, tkn) in tkns.iter().enumerate() {
            if let Some(t) = self.peek_at(self.i + j, *tkn) {
                tokens.push(t);
            } else {
                return None;
            }
        }
        Some(tokens)
    }

    /// If the next n Tokens match the given n Tkns, return them all, and advance past them.
    pub fn grabem(&mut self, tkns: &[Tkn]) -> Option<Vec<&'a Token>> {
        if let Some(t) = self.peekem(tkns) {
            self.i += t.len();
            Some(t)
        } else {
            None
        }
    }

    /// Construct an error from the current tokne.
    pub fn error(&self, msg: String) -> Error {
        self.tokens
            .get(self.i)
            .unwrap_or(self.tokens.last().unwrap())
            .error(msg)
    }

    /// If we can't grab the given token, issue an error.
    pub fn need(&mut self, tkn: Tkn) -> Result<&'a Token, Error> {
        if let Some(t) = self.grab(tkn) {
            Ok(t)
        } else {
            Err(self.error(format!("expected {:?}", tkn)))
        }
    }

    /// If we can't grab the given tokens, issue an error.
    pub fn needem(&mut self, tkns: &[Tkn]) -> Result<Vec<&'a Token>, Error> {
        if let Some(t) = self.grabem(tkns) {
            Ok(t)
        } else {
            Err(self.error(format!("expected {:?}", tkns)))
        }
    }
}

/// Iterates over sentences.
///
/// A sentence is defined as a series of tokens seperated by a Dot token.
pub struct Sentences<'a> {
    tokens: &'a [Token],
    i: usize,
}

impl<'a> Iterator for Sentences<'a> {
    type Item = &'a [Token];

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.tokens.len() {
            let start = self.i;
            let mut stop = self.i;
            while stop < self.tokens.len() {
                stop += 1;
                if self.tokens[stop - 1].tkn(Tkn::Dot) {
                    break;
                }
            }
            self.i = stop;
            Some(&self.tokens[start..stop])
        } else {
            None
        }
    }
}

/// An iterator over paragraphs.
pub struct Paragraphs<'a> {
    tokens: &'a [Token],
    i: usize,
}

impl<'a> Iterator for Paragraphs<'a> {
    type Item = &'a [Token];

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.tokens.len() && self.tokens[self.i].tkn(Tkn::Paragraph) {
            self.i += 1;
        }
        if self.i < self.tokens.len() {
            let start = self.i;
            let mut stop = start;
            while stop < self.tokens.len() && !self.tokens[stop].tkn(Tkn::Paragraph) {
                stop += 1;
            }
            self.i = stop;
            Some(&self.tokens[start..stop])
        } else {
            None
        }
    }
}

/// Allows creation of various Token iterators for anything that holds Token references.
pub trait TokenHolder {
    fn token_slice(&self) -> &[Token];
    fn muncher(&self) -> Muncher {
        Muncher {
            tokens: self.token_slice(),
            i: 0,
        }
    }
    fn sentences(&self) -> Sentences {
        Sentences {
            tokens: self.token_slice(),
            i: 0,
        }
    }
    fn paragraphs(&self) -> Paragraphs {
        Paragraphs {
            tokens: self.token_slice(),
            i: 0,
        }
    }
}

impl TokenHolder for Tokens {
    #[inline]
    fn token_slice(&self) -> &[Token] {
        &self.tokens
    }
}

impl TokenHolder for Paragraphs<'_> {
    #[inline]
    fn token_slice(&self) -> &[Token] {
        self.tokens
    }
}

impl TokenHolder for Sentences<'_> {
    #[inline]
    fn token_slice(&self) -> &[Token] {
        self.tokens
    }
}

impl TokenHolder for Muncher<'_> {
    #[inline]
    fn token_slice(&self) -> &[Token] {
        &self.tokens[self.i..]
    }
}

impl TokenHolder for &[Token] {
    fn token_slice(&self) -> &[Token] {
        self
    }
}

mod kw;
mod token;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::src;

    use super::{Kw, Paragraphs, Sentences, Tkn, Token};

    #[test]
    fn test_sentence() {
        let pos = src::Pos::new(Rc::new("{test}".to_string()), 1, 1);
        let tkns = [
            Token::new(Tkn::Kw(Kw::Procedure), pos.clone()),
            Token::new(Tkn::Kw(Kw::Division), pos.clone()),
            Token::new(Tkn::Dot, pos.clone()),
            Token::new(Tkn::Kw(Kw::Data), pos.clone()),
            Token::new(Tkn::Kw(Kw::Division), pos.clone()),
            Token::new(Tkn::Dot, pos.clone()),
            Token::new(Tkn::Kw(Kw::Environment), pos.clone()),
            Token::new(Tkn::Kw(Kw::Division), pos.clone()),
            Token::new(Tkn::Dot, pos.clone()),
        ];
        let sentences = [
            [Tkn::Kw(Kw::Procedure), Tkn::Kw(Kw::Division), Tkn::Dot],
            [Tkn::Kw(Kw::Data), Tkn::Kw(Kw::Division), Tkn::Dot],
            [Tkn::Kw(Kw::Environment), Tkn::Kw(Kw::Division), Tkn::Dot],
        ];
        let i = Sentences {
            tokens: &tkns,
            i: 0,
        };
        for (left, right) in i.zip(&sentences) {
            assert_eq!(left.len(), right.len());
            for (l, r) in left.iter().zip(right) {
                assert!(l.tkn(*r));
            }
        }
    }

    #[test]
    fn test_paragraphs() {
        let pos = src::Pos::new(Rc::new("{test}".to_string()), 1, 1);
        let tkns = [
            Token::new(Tkn::Kw(Kw::Procedure), pos.clone()),
            Token::new(Tkn::Kw(Kw::Division), pos.clone()),
            Token::new(Tkn::Dot, pos.clone()),
            Token::new(Tkn::Paragraph, pos.clone()),
            Token::new(Tkn::Kw(Kw::Data), pos.clone()),
            Token::new(Tkn::Kw(Kw::Division), pos.clone()),
            Token::new(Tkn::Dot, pos.clone()),
            Token::new(Tkn::Paragraph, pos.clone()),
            Token::new(Tkn::Kw(Kw::Environment), pos.clone()),
            Token::new(Tkn::Kw(Kw::Division), pos.clone()),
            Token::new(Tkn::Dot, pos.clone()),
            Token::new(Tkn::Paragraph, pos.clone()),
        ];
        let paragraphs = [
            [Tkn::Kw(Kw::Procedure), Tkn::Kw(Kw::Division), Tkn::Dot],
            [Tkn::Kw(Kw::Data), Tkn::Kw(Kw::Division), Tkn::Dot],
            [Tkn::Kw(Kw::Environment), Tkn::Kw(Kw::Division), Tkn::Dot],
        ];
        let i = Paragraphs {
            tokens: &tkns,
            i: 0,
        };
        for (left, right) in i.zip(&paragraphs) {
            assert_eq!(left.len(), right.len());
            for (l, r) in left.iter().zip(right) {
                assert!(l.tkn(*r));
            }
        }
    }
}
