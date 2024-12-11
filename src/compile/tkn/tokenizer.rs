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
}

pub fn tokenize(srcs: &[Src]) -> Result<Vec<Token>, Error> {
    let tokenizer = Tokenizer::new(srcs.iter().flat_map(Liner::new));
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
        Self { lines }
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
