use core::fmt;

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
    pub const UNPLACED_JUMP: Instruction = Instruction::Jump(0);
    pub const UNPLACED_JUMP_IF_TRUE: Instruction = Instruction::JumpIfTrue(0);
    pub const UNPLACED_JUMP_IF_FALSE: Instruction = Instruction::JumpIfFalse(0);
}
