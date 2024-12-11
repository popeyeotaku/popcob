//! The parsing side of the interpreter.

use datadiv::DataDiv;
use envdiv::EnvDiv;
use procdiv::ProcDiv;
use tkn::{Kw, Muncher, Tkn, Token, TokenHolder, Tokens};

use crate::src::{Error, Src};

/// The compiled program.
pub struct Program {
    procdiv: ProcDiv,
    datadiv: DataDiv,
    envdiv: EnvDiv,
}

/// Compile a series of program source files.
pub fn compile(srcs: &[Src]) -> Result<Program, Error> {
    let t = Tokens::new(srcs)?;
    let mut m = t.muncher();
    let divs = split_divisions(&mut m)?;
    let mut procdiv = divs[0].muncher();
    let mut datadiv = divs[1].muncher();
    let mut envdiv = divs[2].muncher();

    let procdiv = procdiv::compile(&mut procdiv)?;
    let datadiv = datadiv::compile(&mut datadiv)?;
    let envdiv = envdiv::compile(&mut envdiv)?;
    Ok(Program {
        procdiv,
        datadiv,
        envdiv,
    })
}

mod datadiv;
mod envdiv;
mod procdiv;
mod tkn;

/// A literal or a path.
pub enum LitPath {
    Literal(String),
    Path(Vec<String>),
}

/// Split the different divisions.
fn split_divisions<'a,'b>(m: &'b mut Muncher<'a>) -> Result<[&'a [Token]; 3], Error> where 'b:'a {
    let proctkn = [Tkn::Kw(Kw::Procedure), Tkn::Kw(Kw::Division), Tkn::Dot];
    let datatkn = [Tkn::Kw(Kw::Data), Tkn::Kw(Kw::Division), Tkn::Dot];
    let envtkn = [Tkn::Kw(Kw::Environment), Tkn::Kw(Kw::Division), Tkn::Dot];

    m.skip_paragraphs();
    m.needem(&proctkn)?;
    let start = m.cur_index();
    while m.grabem(&datatkn).is_none() {
        m.early_eof()?;
        m.advance(1);
    }
    let procdiv = &m.tokens[start..m.cur_index()];

    m.needem(&datatkn)?;
    let start = m.cur_index();
    while m.grabem(&envtkn).is_none() {
        m.early_eof()?;
        m.advance(1);
    }
    let datadiv = &m.tokens[start..m.cur_index()];

    m.needem(&envtkn)?;
    let envdiv = m.token_slice();

    Ok([procdiv, datadiv, envdiv])
}
