use alloy::parser::{value::build_value, AlloyParser, Rule};
use pest::Parser;
use std::io::{self, Write};

fn main() {
    println!("Alloylang REPL");
    inputline();
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim_end() == "exit" {
                    break;
                }
                let mut parsed = AlloyParser::parse(Rule::number, input.as_str()).unwrap();
                let result = build_value(parsed.next().unwrap());
                println!("{:?}", result);
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
