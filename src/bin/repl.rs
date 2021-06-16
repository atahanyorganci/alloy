use alloy::parser::{statement::build_statements, AlloyParser, Rule};
use pest::Parser;
use std::io::{self, Write};

fn main() {
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
                let statements = build_statements(&mut parsed);
                for statement in statements {
                    statement.eval();
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
