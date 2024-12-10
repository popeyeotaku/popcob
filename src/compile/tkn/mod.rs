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

mod kw;
mod token;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::src;

    use super::{Kw, Sentences, Tkn, Token};

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
}
