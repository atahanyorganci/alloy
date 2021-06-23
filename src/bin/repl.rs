use std::io::{self, Write};

use alloy::parser::{statement::build_statements, AlloyParser, Rule};
use pest::Parser;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "repl")]
struct Opt {
    /// Verbose mode
    #[structopt(short, long)]
    verbose: bool,

    /// If set AST will not be evaluated
    #[structopt(long)]
    no_eval: bool,
}

fn main() {
    let opt = Opt::from_args();

    println!("Alloylang REPL");
    inputline();
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim_end();
                if trimmed == "exit" {
                    break;
                }
                let mut parsed = AlloyParser::parse(Rule::program, trimmed).unwrap();
                if opt.verbose {
                    println!("{}", parsed);
                }
                if !opt.no_eval {
                    let statements = build_statements(&mut parsed);
                    for statement in statements {
                        statement.eval();
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
        inputline();
    }
}

fn inputline() {
    print!(">>> ");
    io::stdout().flush().unwrap();
}
