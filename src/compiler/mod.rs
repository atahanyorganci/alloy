use std::{borrow::Borrow, collections::HashMap, convert::TryInto, fmt};

use crate::parser::{statement::declare_assign_statement::VariableKind, value::Value};

pub trait Compile {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError>;
}

#[derive(Debug, Default)]
pub struct Compiler {
    symbol_table: SymbolTable,
    instructions: Vec<Instruction>,
}

fn get_width(mut len: usize) -> usize {
    let mut width = 0;
    while len > 0 {
        len /= 10;
        width += 1;
    }
    width
}

impl fmt::Display for Compiler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.instructions.len();
        let width = get_width(len);
        for i in 0..len {
            write!(f, "[{:1$}]\t", i, width)?;
            self.display_instruction(f, i)?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Compiler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn emit(&mut self, insruction: Instruction) {
        self.instructions.push(insruction);
    }

    pub fn get_identifer(&self, identifier: &String) -> Option<Symbol> {
        self.symbol_table.get_identifer_index(identifier)
    }

    pub fn register_const(&mut self, identifier: &String) -> Result<u16, CompilerError> {
        self.symbol_table.register_const(identifier.clone())
    }

    pub fn register_var(&mut self, identifier: &String) -> Result<u16, CompilerError> {
        self.symbol_table.register_var(identifier.clone())
    }

    pub fn register_value(&mut self, value: Value) -> Result<u16, CompilerError> {
        self.symbol_table.register_value(value)
    }

    fn display_instruction(&self, f: &mut fmt::Formatter<'_>, index: usize) -> fmt::Result {
        let instruction = self.instructions.get(index).unwrap();
        match instruction {
            Instruction::StoreSymbol(index) => {
                write!(f, "StoreSymbol\t")?;
                let symbol = self.symbol_table.get_symbol((*index).into()).unwrap();
                write!(f, "{} ({})", index, symbol)
            }
            Instruction::LoadSymbol(index) => {
                write!(f, "LoadSymbol\t")?;
                let symbol = self.symbol_table.get_symbol((*index).into()).unwrap();
                write!(f, "{} ({})", index, symbol)
            }
            Instruction::LoadValue(index) => {
                write!(f, "LoadValue\t")?;
                let symbol = self.symbol_table.get_value((*index).into()).unwrap();
                write!(f, "{} ({})", index, symbol)
            }
            _ => write!(f, "{:?}", instruction),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CompilerError {
    VariableLimitReached,
    ConstLimitReached,
    Redefinition,
    UndefinedIdentifer,
    AssignmentToConst,
}

#[derive(Debug, Default)]
pub struct SymbolTable {
    table: HashMap<String, Symbol>,
    values: HashMap<u16, Value>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_identifier_taken(&self, identifier: &String) -> bool {
        self.table.contains_key(identifier)
    }

    pub fn get_identifer_index(&self, identifier: &String) -> Option<Symbol> {
        match self.table.get(identifier) {
            Some(&symbol) => Some(symbol),
            None => None,
        }
    }

    pub fn register_var(&mut self, identifier: String) -> Result<u16, CompilerError> {
        if self.is_identifier_taken(&identifier) {
            return Err(CompilerError::Redefinition);
        }

        let index = self.get_next_identifier_index()?;
        self.table.insert(identifier, Symbol::variable(index));
        Ok(index)
    }

    pub fn register_const(&mut self, identifier: String) -> Result<u16, CompilerError> {
        if self.is_identifier_taken(&identifier) {
            return Err(CompilerError::Redefinition);
        }

        let index = self.get_next_identifier_index()?;
        self.table.insert(identifier, Symbol::constant(index));
        Ok(index)
    }

    pub fn register_value(&mut self, value: Value) -> Result<u16, CompilerError> {
        let count = self.values.len();
        let index: u16 = match count.try_into() {
            Ok(index) => index,
            Err(_) => return Err(CompilerError::ConstLimitReached),
        };
        self.values.insert(index, value);
        Ok(index)
    }

    fn get_next_identifier_index(&self) -> Result<u16, CompilerError> {
        let count = self.table.len();
        match count.try_into() {
            Ok(index) => Ok(index),
            Err(_) => return Err(CompilerError::VariableLimitReached),
        }
    }

    pub fn get_symbol(&self, index: u16) -> Option<&String> {
        let result = self
            .table
            .borrow()
            .into_iter()
            .find(|(_, symbol)| symbol.index == index);

        match result {
            Some((identifier, _)) => Some(identifier),
            None => None,
        }
    }

    pub fn get_value(&self, index: u16) -> Option<&Value> {
        self.values.get(&index)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Symbol {
    pub index: u16,
    pub kind: VariableKind,
}

impl Symbol {
    pub fn variable(index: u16) -> Self {
        Self {
            index,
            kind: VariableKind::Variable,
        }
    }

    pub fn constant(index: u16) -> Self {
        Self {
            index,
            kind: VariableKind::Constant,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    StoreSymbol(u16),
    LoadSymbol(u16),
    LoadValue(u16),
    Pop,
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
    UnaryMinus,
    UnaryNot,
}
