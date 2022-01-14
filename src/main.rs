use alloy::parser::{statement::build_statements, AlloyParser, Rule};
use pest::Parser;
use rustyline::error::ReadlineError;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "repl")]
struct Alloy {
    /// Verbose mode
    #[structopt(short, long)]
    verbose: bool,
    /// If set AST will not be evaluated
    #[structopt(long)]
    no_eval: bool,
}

impl Alloy {
    pub fn consume(&self, line: &str) {
        if line == "exit" || line == "" {
            return;
        }
        let mut parsed = AlloyParser::parse(Rule::program, line).unwrap();
        if self.verbose {
            println!("{}", parsed);
        }
        if !self.no_eval {
            let statements = build_statements(&mut parsed);
            for statement in statements {
                statement.eval();
            }
        }
    }
}

fn main() {
    let alloy = Alloy::from_args();

    let mut editor = rustyline::Editor::<()>::new();

    println!("Alloylang REPL");
    loop {
        let readline = editor.readline(">>> ");
        match readline {
            Ok(line) => alloy.consume(line.as_str()),
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => break,
            Err(err) => eprintln!("Unexpected error encountered {}.", err),
        }
    }
}
