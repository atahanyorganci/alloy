use std::fmt;

use crate::ast::value::Value;

use super::Instruction;

pub struct CodeBlock {
    pub instructions: Vec<Instruction>,
    pub values: Vec<Value>,
}

impl fmt::Display for CodeBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, instruction) in self.instructions.iter().enumerate() {
            write!(f, "{i:>4}\t{instruction}")?;
        }
        Ok(())
    }
}
