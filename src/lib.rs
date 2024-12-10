//! PopCob - the World's First* COBOL-60 interpreter.

use std::error::Error;

/// Compile and execute the given source file.
///
/// Returns the string output file, if any.
pub fn execute(source_file: &str) -> Result<String, Box<dyn Error>> {
    todo!()
}

/// Execute source from a string instead of a file.
pub fn exec_str(source:&str) -> Result<String, Box<dyn Error>> {
    todo!()
}

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
