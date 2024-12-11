use std::{iter::Enumerate, rc::Rc, str::Lines};

use crate::src::{self, Error, Src};

use super::Token;

/// Returns lines from source code.
struct Liner<'a> {
    filename: Rc<String>,
    lines: Enumerate<Lines<'a>>,
}

impl<'a> Iterator for Liner<'a> {
    type Item = (src::Pos, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((i, line)) = self.lines.next() {
            Some((
                src::Pos::new(self.filename.clone(), (i + 1) as u32, 1),
                line,
            ))
        } else {
            None
        }
    }
}

impl<'a> Liner<'a> {
    pub fn new(src: &'a Src) -> Self {
        Self {
            filename: src.filename.clone(),
            lines: src.text.lines().enumerate(),
        }
    }
}

/// Tokenizes source input.
struct Tokenizer<'a, I>
where
    I: Iterator<Item = (src::Pos, &'a str)>,
{
    lines: I,
    cur_line: Option<(src::Pos, &'a str)>,
    i: usize,
}

pub fn tokenize(srcs: &[Src]) -> Result<Vec<Token>, Error> {
    let tokenizer = Tokenizer::new(srcs.iter().flat_map(Liner::new).fuse());
    let mut tokens: Vec<Token> = Vec::with_capacity(tokenizer.size_hint().0);
    for maybe_token in tokenizer {
        tokens.push(maybe_token?);
    }
    Ok(tokens)
}

impl<'a, I> Tokenizer<'a, I>
where
    I: Iterator<Item = (src::Pos, &'a str)>,
{
    pub fn new(lines: I) -> Self {
        let mut t = Self {
            lines,
            cur_line: None,
            i: 0,
        };
        t.refresh_line();
        t
    }

    /// Update the current line if needbe.
    ///
    /// If the current line is None, it rereads it from our internal iterator.
    fn refresh_line(&mut self) {
        if self.cur_line.is_none() {
            self.cur_line = self.lines.next();
            self.i = 0;
        }
    }

    /// Return the current source position.
    fn srcpos(&mut self) -> Option<src::Pos> {
        if let Some((p, _)) = &self.cur_line {
            Some(p.with_charpos((self.i + 1) as u16))
        } else {
            None
        }
    }

    /// Return the current input character.
    fn peekc(&mut self) -> Option<char> {
        loop {
            if let Some((_, l)) = &self.cur_line {
                if let Some(c) = l.chars().nth(self.i) {
                    return Some(c);
                } else {
                    self.cur_line = None;
                    self.refresh_line();
                }
            } else {
                return None;
            }
        }
    }

    /// Advance to the next input character.
    fn skipc(&mut self) {
        self.i += 1;
    }

    /// Return the current input character and advance.
    pub fn nextc(&mut self) -> char {
        if let Some(c) = self.peekc() {
            self.skipc();
            c
        } else {
            panic!("unexpected end of input (try less to check with peekc first!)");
        }
    }

    /// Return a flag for if we're at the end of input.
    pub fn at_end(&mut self) -> bool {
        self.peekc().is_none()
    }
}

impl<'a, I> Iterator for Tokenizer<'a, I>
where
    I: Iterator<Item = (src::Pos, &'a str)>,
{
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::src::Src;

    use super::{Liner, Tokenizer};

    #[test]
    fn test_peekc() {
        let s = "foo";
        let srcs = [Src::new("{test}".to_owned(), s.to_owned())];
        let mut t = Tokenizer::new(srcs.iter().flat_map(Liner::new).fuse());
        for c in s.chars() {
            assert_eq!(c, t.nextc())
        }
        assert!(t.at_end());
    }
}
