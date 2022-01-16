use std::{fmt, mem};

use crate::ast::{value::Value, Identifier, IdentifierKind};

use self::{code_block::CodeBlock, symbol_table::SymbolTable};

pub mod code_block;
pub mod symbol_table;

pub trait Compile {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError>;
}

type CompilerResult<T> = Result<T, CompilerError>;

#[derive(Debug, Default)]
pub struct Compiler {
    symbol_table: SymbolTable,
    instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn emit(&mut self, insruction: Instruction) {
        self.instructions.push(insruction);
    }

    pub fn register(&mut self, identifier: Identifier) -> CompilerResult<u16> {
        self.symbol_table.register(identifier)
    }

    pub fn get_identifier(&self, ident: &str) -> Option<(IdentifierKind, u16)> {
        self.symbol_table.get(ident)
    }

    pub fn register_value(&mut self, value: Value) -> Result<u16, CompilerError> {
        self.symbol_table.register_value(value)
    }

    pub fn finish<'a>(&'a mut self) -> (CodeBlock, Vec<&'a String>) {
        let instructions = mem::take(&mut self.instructions);
        let (values, debug_symbols) = self.symbol_table.finish();
        (
            CodeBlock {
                instructions,
                values,
            },
            debug_symbols,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CompilerError {
    VariableLimitReached,
    ConstLimitReached,
    Redefinition,
    UndefinedIdentifer,
    AssignmentToConst,
    InvalidInstruction,
    InvalidLabel,
    BreakOutsideLoop,
    ContinueOutsideLoop,
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    StoreSymbol(u16),
    LoadSymbol(u16),
    LoadValue(u16),
    Pop,
    // Display Instruction to be removed
    Display,
    // Jump Instructions
    Jump(u16),
    JumpIfTrue(u16),
    JumpIfFalse(u16),
    // Binary Operator Instructions
    BinaryAdd,
    BinarySubtract,
    BinaryMultiply,
    BinaryDivide,
    BinaryReminder,
    BinaryPower,
    BinaryLessThan,
    BinaryLessThanEqual,
    BinaryGreaterThan,
    BinaryGreaterThanEqual,
    BinaryEqual,
    BinaryNotEqual,
    BinaryLogicalAnd,
    BinaryLogicalOr,
    BinaryLogicalXor,
    // Unary Operators
    UnaryMinus,
    UnaryNot,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::StoreSymbol(idx) => write!(f, "StoreSymbol({idx})"),
            Instruction::LoadSymbol(idx) => write!(f, "LoadSymbol({idx})"),
            Instruction::LoadValue(idx) => write!(f, "LoadValue({idx})"),
            Instruction::Jump(idx) => write!(f, "Jump({idx})"),
            Instruction::JumpIfTrue(idx) => write!(f, "JumpIfTrue({idx})"),
            Instruction::JumpIfFalse(idx) => write!(f, "JumpIfFalse({idx})"),
            Instruction::Pop
            | Instruction::Display
            | Instruction::BinaryAdd
            | Instruction::BinarySubtract
            | Instruction::BinaryMultiply
            | Instruction::BinaryDivide
            | Instruction::BinaryReminder
            | Instruction::BinaryPower
            | Instruction::BinaryLessThan
            | Instruction::BinaryLessThanEqual
            | Instruction::BinaryGreaterThan
            | Instruction::BinaryGreaterThanEqual
            | Instruction::BinaryEqual
            | Instruction::BinaryNotEqual
            | Instruction::BinaryLogicalAnd
            | Instruction::BinaryLogicalOr
            | Instruction::BinaryLogicalXor
            | Instruction::UnaryMinus
            | Instruction::UnaryNot => write!(f, "{self:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CompilerError;

    fn compile(_input: &str) -> Result<(), CompilerError> {
        todo!()
    }

    #[test]
    fn compile_statements() {
        assert!(compile("5 + 12 * 4;").is_ok());
        assert!(compile("const x = 10 * 12; 10 * x;").is_ok());
        assert!(compile("const x = 10; var y = x; y = x * y;").is_ok());
        assert!(compile("if true { const x = 12; }").is_ok());
        assert!(compile("if true { const x = 12; } else { const y = 12; } const z = 12;").is_ok());
        assert!(compile("if true { const x = 12; } else { const y = 12; } const z = 12;").is_ok());
        assert!(compile("if true { const a = 0; } else if false { const b = 10; } else { const c = 20; } const d = 30;").is_ok());
        assert!(compile("if true { const a = 0; } else if false { const b = 10; } else if 0 { const c = 20; } const d = 30;").is_ok());
        assert!(compile("if true { const a = 0; } else if false { const b = 10; } else if 0 { const c = 20; } else { const d = 30; }").is_ok());
        assert!(compile("while true { const x = 12; } const y = 12;").is_ok());
        assert!(compile("while true { print 12; break; } print 54;").is_ok());
        assert!(compile("while true { print 12; continue; } print 12;").is_ok());
        assert!(compile("var count = 0; var first = 1; var second = 0; while count < 40 { print first; const temp = first; first = first + second; second = temp; } ").is_ok());
    }

    #[test]
    fn wont_compile_statements() {
        assert!(compile("const x = 5; x = 5;").is_err());
        assert!(compile("const x = 5; const x = 5;").is_err());
        assert!(compile("const x = 5; var x = 5;").is_err());
        assert!(compile("const x = x;").is_err());
        assert!(compile("var x = x;").is_err());
    }
}
