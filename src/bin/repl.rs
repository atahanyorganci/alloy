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
}

fn main() {
    let opt = Opt::from_args();

    println!("Alloylang REPL");
    inputline();
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(count) => {
                let trimmed = input.trim_end();
                if count == 0 || trimmed == "exit" {
                    break;
                }
                let mut parsed = AlloyParser::parse(Rule::program, trimmed).unwrap();
                if opt.verbose {
                    println!("{}", parsed);
                }
                match build_statements(&mut parsed) {
                    Ok(statements) => {
                        for statement in statements {
                            println!("{:?}", statement)
                        }
                    }
                    Err(e) => eprintln!("{:?}", e),
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
