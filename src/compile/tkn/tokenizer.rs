use std::usize;

use crate::src::{self, Error, Src};

use super::Token;

pub fn tokenize(srcs: &[Src]) -> Result<Vec<Token>, Error> {
    todo!()
}

struct Line {
    start_pos: src::Pos,
    chars: Vec<char>,
}

/// Indicates which part of the text the line starts on.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Field {
    A,
    B,
    C,
}

/// Starting position of each field, after the leading sequence field.
const A_FIELD: usize = 8 - 1;
const B_FIELD: usize = A_FIELD + 4;
const C_FIELD: usize = B_FIELD + 4;

impl Line {
    pub fn new(start_pos: src::Pos, text: &str) -> Self {
        let chars = text.replace('\t', "        ").chars().collect();
        Self { start_pos, chars }
    }

    /// Return the main part of the line, starting with the first non-blank after the sequence field,
    /// and indicating which part of the line it started in (A/B/C field).
    /// Also returns the starting position of the text.
    pub fn text(&self) -> (&[char], Field, src::Pos) {
        let start = {
            let mut i = A_FIELD;
            while i < self.chars.len() {
                if self.chars[i] == ' ' {
                    i += 1;
                } else {
                    break;
                }
            }
            i
        };
        let mut text = &self.chars[start..];
        while text.last() == Some(&' ') {
            text = &text[..text.len() - 1]
        }

        let field = {
            if text.is_empty() {
                Field::B
            } else {
                match start {
                    A_FIELD..B_FIELD => Field::A,
                    B_FIELD..C_FIELD => Field::B,
                    _ => Field::C,
                }
            }
        };
        (text, field, self.start_pos.with_charpos((start + 1) as u16))
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::src;

    use super::{Field, Line};

    #[test]
    fn test_line() {
        let s = "000000 A_FIELD
000100     B_FIELD
000200 \tC_FIELD   \t";
        let results = [
            ("A_FIELD", Field::A),
            ("B_FIELD", Field::B),
            ("C_FIELD", Field::C),
        ];
        let name = Rc::new("{s}".to_string());
        let lines = s
            .lines()
            .enumerate()
            .map(|(i, l)| Line::new(src::Pos::new(name.clone(), (i + 1) as u32, 1), l));
        for (line, result) in lines.zip(results) {
            let (ltext, lfield, _) = line.text();
            let (rtext, rfield) = result;
            assert_eq!(&String::from_iter(ltext.iter()), rtext);
            assert_eq!(lfield, rfield);
        }
    }
}
