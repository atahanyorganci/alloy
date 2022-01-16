use std::{collections::HashMap, convert::TryInto, fmt, mem};

use crate::ast::{value::Value, Identifier, IdentifierKind};

use self::{code_block::CodeBlock, symbol_table::SymbolTable};

pub mod code_block;
pub mod symbol_table;

pub trait Compile {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError>;
}

type CompilerResult<T> = Result<T, CompilerError>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BlockType {
    Block,
    If,
    For,
    While,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub struct Label(usize);

impl Into<Label> for usize {
    fn into(self) -> Label {
        Label(self)
    }
}

impl From<Label> for usize {
    fn from(label: Label) -> Self {
        label.0
    }
}

impl Label {
    pub fn target(self) -> Result<u16, CompilerError> {
        if let Ok(t) = self.0.try_into() {
            Ok(t)
        } else {
            Err(CompilerError::InstructionLimitReached)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub struct JumpRef {
    idx: usize,
}

impl Into<usize> for JumpRef {
    fn into(self) -> usize {
        self.idx
    }
}

#[derive(Debug, Default)]
pub struct Compiler {
    symbol_table: SymbolTable,
    instructions: Vec<Instruction>,
    blocks: Vec<BlockType>,
    unplaced_labels: HashMap<usize, Vec<JumpRef>>,
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

    pub fn enter_block(&mut self, block_type: BlockType) {
        self.blocks.push(block_type)
    }

    pub fn exit_block(&mut self) {
        self.blocks.pop().unwrap();
        let block_idx = self.blocks.len();
        if let Some(registered) = self.unplaced_labels.remove(&block_idx) {
            for jump in registered {
                self.target_jump(jump);
            }
        }
    }

    pub fn emit_jump(&mut self, jump: Instruction) -> JumpRef {
        match jump {
            Instruction::Jump(_) | Instruction::JumpIfTrue(_) | Instruction::JumpIfFalse(_) => {
                let idx = self.instructions.len();
                self.instructions.push(jump);
                JumpRef { idx }
            }
            _ => unreachable!(),
        }
    }

    pub fn emit_untargeted_jump(&mut self) -> JumpRef {
        self.emit_jump(Instruction::UNPLACED_JUMP)
    }

    pub fn emit_untargeted_jump_if_false(&mut self) -> JumpRef {
        self.emit_jump(Instruction::UNPLACED_JUMP_IF_FALSE)
    }

    pub fn emit_untargeted_jump_if_true(&mut self) -> JumpRef {
        self.emit_jump(Instruction::UNPLACED_JUMP_IF_TRUE)
    }

    pub fn place_label(&mut self) -> Label {
        self.instructions.len().into()
    }

    pub fn target_jump(&mut self, jump: JumpRef) {
        let idx: usize = jump.into();
        let target = self.current();
        let jump = match self.instructions[idx] {
            Instruction::Jump(_) => Instruction::Jump(target),
            Instruction::JumpIfTrue(_) => Instruction::JumpIfTrue(target),
            Instruction::JumpIfFalse(_) => Instruction::JumpIfFalse(target),
            _ => unreachable!(),
        };
        self.instructions[idx] = jump;
    }

    pub fn target_jump_on_exit(&mut self, block_type: BlockType, jump: JumpRef) {
        for (i, current) in self.blocks.iter().enumerate().rev() {
            if *current == block_type {
                if let Some(vec) = self.unplaced_labels.get_mut(&i) {
                    vec.push(jump);
                } else {
                    let labels = vec![jump];
                    self.unplaced_labels.insert(i, labels);
                }
            }
        }
    }

    fn current(&self) -> u16 {
        self.instructions.len().try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CompilerError {
    VariableLimitReached,
    Redefinition,
    UndefinedIdentifer,
    AssignmentToConst,
    InstructionLimitReached,
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

impl Instruction {
    const UNPLACED_JUMP: Instruction = Instruction::Jump(0);
    const UNPLACED_JUMP_IF_TRUE: Instruction = Instruction::JumpIfTrue(0);
    const UNPLACED_JUMP_IF_FALSE: Instruction = Instruction::JumpIfFalse(0);
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
