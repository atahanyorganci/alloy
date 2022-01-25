use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    // Load & Store operations
    StoreFast(u8),
    LoadFast(u8),
    LoadFastValue(u8),
    Store(usize),
    Load(usize),
    LoadValue(usize),
    // Remove value on top of stack
    Pop,
    // Display Instruction to be removed
    Display,
    // Jump Instructions
    JumpShort(u8),
    JumpShortIfZero(u8),
    Jump(usize),
    JumpIfZero(usize),
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
            Instruction::StoreFast(u) => write!(f, "StoreFast({u})"),
            Instruction::LoadFast(u) => write!(f, "LoadFast({u})"),
            Instruction::LoadFastValue(u) => write!(f, "LoadFastValue({u})"),
            Instruction::Store(u) => write!(f, "Store({u})"),
            Instruction::Load(u) => write!(f, "Load({u})"),
            Instruction::LoadValue(u) => write!(f, "LoadValue({u})"),
            Instruction::JumpShort(u) => write!(f, "JumpShort({u})"),
            Instruction::JumpShortIfZero(u) => write!(f, "JumpShortIfZero({u})"),
            Instruction::Jump(u) => write!(f, "Jump({u})"),
            Instruction::JumpIfZero(u) => write!(f, "JumpIfZero({u})"),
        }
    }
}

impl Instruction {
    pub const UNPLACED_JUMP: Instruction = Instruction::Jump(0);
    pub const UNPLACED_JUMP_IF_ZERO: Instruction = Instruction::JumpIfZero(0);
}
