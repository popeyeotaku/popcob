//! Procedure Division compilation.

use crate::src::Error;

use super::{
    tkn::{Kw, Muncher, Tkn},
    LitPath,
};

/// The compiled form of the procedure division.
pub struct ProcDiv {
    pub statements: Vec<Statement>,
}

/// A compiled statement.
pub enum Statement {
    Verb(Verb),
}

/// A compiled verb.
pub enum Verb {
    Display(Vec<(LitPath, String)>),
}

/// Compile the procedure division.
pub fn compile(m: &mut Muncher) -> Result<ProcDiv, Error> {
    m.skip_paragraphs();
    m.needem(&[Tkn::Kw(Kw::Procedure), Tkn::Kw(Kw::Division), Tkn::Dot])?;
    let mut statements: Vec<Statement> = Vec::new();
    while !m.at_end()
        && m.peekem(&[Tkn::Kw(Kw::Data), Tkn::Kw(Kw::Division)])
            .is_none()
    {
        statements.push(statement(m)?);
    }
    Ok(ProcDiv { statements })
}

/// Parse one statement.
fn statement(m: &mut Muncher) -> Result<Statement, Error> {
    todo!()
}
