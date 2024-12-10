//! PopCob - the World's First* COBOL-60 interpreter.

use std::fs;

use src::{Error, Src};

/// Compile and execute the given source file.
///
/// Returns the string output file, if any.
pub fn execute(paths: &[String]) -> Result<String, Box<dyn std::error::Error>> {
    let mut sources: Vec<Src> = Vec::with_capacity(paths.len());
    for path in paths {
        let text = fs::read_to_string(path)?;
        sources.push(Src::new(path.to_string(), text));
    }
    Ok(exec(&sources)?)
}

/// Execute source from a string instead of a file.
pub fn exec_str(source: &str) -> Result<String, Error> {
    let sources = [Src::new("{string}".to_string(), source.to_string())];
    exec(&sources)
}

/// Compile and execute from some source files.
fn exec(srcs: &[Src]) -> Result<String, Error> {
    let program = compile::compile(srcs)?;
    let output = interpret::interpret(&program)?;
    Ok(output.strout)
}

mod compile;
mod interpret;
mod src;

#[cfg(test)]
mod tests {
    use crate::exec_str;

    #[test]
    fn test_hello_world() {
        let src = "
000100 PROCEDURE DIVISION.
000200 MAIN. DISPLAY \"HELLO WORLD\" UPON OUTPUT.
000300 DATA DIVISION.
000400 ENVIRONMENT DIVISION.
000500 CONFIGURATION SECTION.
000600 SOURCE-COMPUTER POPCOB.
000700 OBJECT-COMPUTER POPCOB.
000800 SPECIAL NAMES. STROUT IS OUTPUT.";
        assert_eq!(exec_str(src).unwrap(), "HELLO WORLD");
    }
}
