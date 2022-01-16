use std::io::{self, Write};

use alloy::{
    ast::statement::{build_statements, Statement},
    compiler::{Compile, Compiler},
    parser::{AlloyParser, Rule},
};
use pest::Parser;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "repl")]
struct Opt {
    /// Verbose mode
    #[structopt(short, long)]
    verbose: bool,
}

pub fn compile(compiler: &mut Compiler, statements: Vec<Statement>) {
    for statement in statements {
        println!("{:?}", statement);
        if let Err(error) = statement.compile(compiler) {
            eprintln!("{error:?}");
            break;
        }
    }
    let (code_block, debug_symbols) = compiler.finish();
    let dis = code_block.disassemble(&debug_symbols);
    println!("{dis}");
}

fn main() {
    let opt = Opt::from_args();

    println!("Alloylang REPL");
    inputline();
    let mut compiler = Compiler::new();
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
                    Ok(statements) => compile(&mut compiler, statements),
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
