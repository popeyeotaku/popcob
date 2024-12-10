//! The parsing side of the interpreter.

use tkn::Tokens;

use crate::src::{Error, Src};

/// The compiled program.
pub struct Program {}

/// Compile a series of program source files.
pub fn compile(srcs: &[Src]) -> Result<Program, Error> {
    let t = Tokens::new(srcs)?;
    todo!()
}

mod tkn;
