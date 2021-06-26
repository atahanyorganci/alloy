use std::{
    borrow::Borrow,
    collections::HashMap,
    convert::TryInto,
    fmt,
    hash::{Hash, Hasher},
};

use crate::parser::{statement::declare_assign_statement::VariableKind, value::Value};

pub trait Compile {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError>;
}

#[derive(Debug, Hash, Clone, Copy)]
pub struct Context {
    kind: ContextKind,
    start: Label,
    end: Label,
}

impl Context {
    pub fn start_label<'a>(&'a self) -> &'a Label {
        &self.start
    }

    pub fn end_label<'a>(&'a self) -> &'a Label {
        &self.end
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum ContextKind {
    If,
    Loop,
}

#[derive(Debug, Default)]
pub struct Compiler {
    symbol_table: SymbolTable,
    instructions: Vec<Instruction>,
    labels: HashMap<Label, Vec<usize>>,
    label_count: usize,
    context: Vec<Context>,
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

    pub fn make_label(&mut self) -> Label {
        let label: Label = self.label_count.into();
        self.labels.insert(label.clone(), Vec::new());
        self.label_count += 1;
        label
    }

    pub fn make_label_now(&mut self) -> Label {
        let target = self.instructions.len().try_into().unwrap();
        let mut label = self.make_label();
        label.set_target(target);
        label
    }

    pub fn emit_jump(&mut self, jump: Instruction, label: &Label) -> Result<(), CompilerError> {
        match jump {
            Instruction::Jump(_) | Instruction::JumpIfTrue(_) | Instruction::JumpIfFalse(_) => {}
            _ => return Err(CompilerError::InvalidInstruction),
        }
        match self.labels.get_mut(label) {
            Some(labels) => {
                let instruction_index = self.instructions.len();
                labels.push(instruction_index);
                self.instructions.push(jump);
                Ok(())
            }
            None => Err(CompilerError::InvalidLabel),
        }
    }

    pub fn place_label(&mut self, label: &Label) -> Result<(), CompilerError> {
        let target = match label.target {
            Some(target) => target,
            None => return Err(CompilerError::InvalidLabel),
        };
        let indexes = match self.labels.get(label) {
            Some(indexes) => indexes,
            None => return Err(CompilerError::InvalidLabel),
        };
        for index in indexes {
            let instruction = self.instructions.get(*index).unwrap();
            let patched = match instruction {
                Instruction::Jump(_) => Instruction::Jump(target),
                Instruction::JumpIfTrue(_) => Instruction::JumpIfTrue(target),
                Instruction::JumpIfFalse(_) => Instruction::JumpIfFalse(target),
                _ => return Err(CompilerError::InvalidInstruction),
            };
            self.instructions[*index] = patched;
        }
        Ok(())
    }

    pub fn place_label_here(&mut self, mut label: Label) -> Result<(), CompilerError> {
        let target = self.instructions.len().try_into().unwrap();
        label.set_target(target);
        self.place_label(&label)
    }

    pub fn push_if_context(&mut self) -> Context {
        self.push_context(ContextKind::If)
    }

    pub fn push_loop_context(&mut self) -> Context {
        self.push_context(ContextKind::Loop)
    }

    fn push_context(&mut self, kind: ContextKind) -> Context {
        let start = self.make_label_now();
        let end = self.make_label();
        let context = Context { start, end, kind };
        self.context.push(context.clone());
        context
    }

    pub fn get_context(&self) -> Option<&Context> {
        self.context.last()
    }

    pub fn get_loop_context(&self) -> Option<&Context> {
        self.context
            .iter()
            .rev()
            .find(|context| context.kind == ContextKind::Loop)
    }

    pub fn pop_context(&mut self) -> Result<(), CompilerError> {
        if let Some(context) = self.context.pop() {
            self.place_label_here(context.start)?;
            self.drop_label(&context.start);
            Ok(())
        } else {
            Err(CompilerError::AssignmentToConst)
        }
    }

    pub fn drop_label(&mut self, label: &Label) {
        self.labels.remove(label);
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

#[derive(Debug, Default, Clone, Copy)]
pub struct Label {
    count: usize,
    target: Option<u16>,
}

impl Label {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    pub fn target(&self) -> u16 {
        self.target.unwrap()
    }

    pub fn set_target(&mut self, target: u16) {
        self.target = Some(target);
    }
}

impl PartialEq for Label {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

impl Eq for Label {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Hash for Label {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.count);
    }
}

impl From<usize> for Label {
    fn from(count: usize) -> Self {
        Self {
            count,
            target: None,
        }
    }
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

#[cfg(test)]
mod tests {
    use std::{
        collections::{hash_map::DefaultHasher, HashMap},
        hash::{Hash, Hasher},
    };

    use pest::Parser;

    use crate::{
        compiler::Label,
        parser::{statement::build_statement, AlloyParser, Rule},
    };

    use super::{Compiler, CompilerError};

    fn compile(input: &str) -> Result<(), CompilerError> {
        let statements = AlloyParser::parse(Rule::program, input).unwrap();
        let mut compiler = Compiler::new();
        for statement in statements {
            match statement.as_rule() {
                Rule::EOI => break,
                _ => build_statement(statement).compile(&mut compiler)?,
            }
        }
        println!("{}", compiler);
        Ok(())
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
    }

    #[test]
    fn wont_compile_statements() {
        assert!(compile("const x = 5; x = 5;").is_err());
        assert!(compile("const x = 5; const x = 5;").is_err());
        assert!(compile("const x = 5; var x = 5;").is_err());
        assert!(compile("const x = x;").is_err());
        assert!(compile("var x = x;").is_err());
    }

    #[test]
    fn test_label_equality() {
        let first = Label::new();
        let mut second = Label::new();
        second.set_target(124);
        assert_eq!(first, second)
    }

    #[test]
    fn test_label_hash() {
        let first = Label::new();
        let mut second = Label::new();
        second.set_target(124);

        let mut hasher = DefaultHasher::new();
        first.hash(&mut hasher);
        let first_hash = hasher.finish();

        hasher = DefaultHasher::new();
        second.hash(&mut hasher);
        let second_hash = hasher.finish();

        assert_eq!(first_hash, second_hash);
    }

    #[test]
    fn test_label_as_key() {
        let first = Label::new();
        let mut second = Label::new();
        second.set_target(124);

        let mut map = HashMap::new();
        map.insert(first, 1);
        assert!(map.contains_key(&second));
    }
}
