//! PopCob - the World's First* COBOL-60 interpreter.

use clap::Parser;

fn main() {
    let args = Args::parse();
    popcob::execute(&args.source).unwrap();
}

#[derive(clap::Parser)]
#[command(about, version)]
struct Args {
    source: String,
}
