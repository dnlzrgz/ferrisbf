use std::{fs, io, path::PathBuf};

use clap::Parser;
use ferrisbf::{Machine, parse, run};

#[derive(Parser)]
#[command(name = "ferrisbf", version, about)]
struct Cli {
    /// Path to the Brainfuck source code.
    path: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let source = fs::read_to_string(&cli.path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", cli.path.display()));
    let program = parse(&source).expect("failed to parse program");
    let mut machine = Machine::new();
    run(&mut machine, &program, &mut io::stdin(), &mut io::stdout()).expect("runtime error");
}
