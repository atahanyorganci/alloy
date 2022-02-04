use alloy::{
    ast::statement::Statement,
    compiler::{Compile, Compiler},
    parser,
};

use rustyline::error::ReadlineError;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "repl")]
struct Alloy {
    /// Verbose mode
    #[structopt(short, long)]
    verbose: bool,
}

impl Alloy {
    pub fn consume(&self, compiler: &mut Compiler, line: &str) {
        if line.is_empty() {
            return;
        }
        match parser::parse(line) {
            Ok(statements) => {
                self.compile(compiler, statements);
            }
            Err(err) => eprintln!("{err:?}"),
        }
    }

    pub fn compile(&self, compiler: &mut Compiler, statements: Vec<Statement>) {
        for statement in statements {
            if self.verbose {
                println!("{:?}", statement);
            }
            if let Err(error) = statement.compile(compiler) {
                eprintln!("{error}");
                return;
            }
        }
        let (code_block, debug_symbols) = compiler.finish();
        let dis = code_block.disassemble(&debug_symbols);
        println!("{dis}");
    }
}

fn main() {
    let alloy = Alloy::from_args();

    let mut editor = rustyline::Editor::<()>::new();
    let mut compiler = Compiler::new();

    println!("Alloylang REPL");
    loop {
        let readline = editor.readline(">>> ");
        match readline {
            Ok(line) if line == "exit" => break,
            Ok(line) => alloy.consume(&mut compiler, line.as_str()),
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => break,
            Err(err) => eprintln!("Unexpected error encountered {err}."),
        }
    }
}
