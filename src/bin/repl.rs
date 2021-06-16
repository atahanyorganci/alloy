use alloy::parser::{expression::build_binary_expression, AlloyParser, Rule};
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
                let parsed = AlloyParser::parse(Rule::program, trimmed).unwrap();
                let result = build_binary_expression(parsed);
                println!("{} = {}", result, result.eval());
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
