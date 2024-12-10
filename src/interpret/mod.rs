//! Interpretation side of the interpreter.

use crate::{compile::Program, src::Error};

/// The result of interpretation.
pub struct Output {
    pub strout: String,
}

/// Interpret a compiled program.
pub fn interpret(program: &Program) -> Result<Output, Error> {
    todo!()
}
