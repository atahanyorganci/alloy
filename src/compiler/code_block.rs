use std::fmt;

use crate::ast::value::Value;

use super::Instruction;

#[derive(Debug)]
pub enum PrettyInstruction<'a> {
    Plain(Instruction),
    Symbol {
        instruction: Instruction,
        identifier: &'a str,
    },
    Value {
        instruction: Instruction,
        value: &'a Value,
    },
}

impl fmt::Display for PrettyInstruction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plain(instruction) => {
                write!(f, "{instruction:?}")
            }
            Self::Symbol {
                instruction,
                identifier,
            } => write!(f, "{instruction}\t{identifier}"),
            Self::Value { instruction, value } => write!(f, "{instruction}\t{value}"),
        }
    }
}

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

impl CodeBlock {
    pub fn disassemble(&self, debug_symbols: &[&String]) -> String {
        self.instructions
            .iter()
            .map(|instruction| match *instruction {
                Instruction::Store(idx) => PrettyInstruction::Symbol {
                    instruction: *instruction,
                    identifier: debug_symbols[idx],
                },
                Instruction::StoreFast(idx) => PrettyInstruction::Symbol {
                    instruction: *instruction,
                    identifier: debug_symbols[idx as usize],
                },
                Instruction::Load(idx) => PrettyInstruction::Symbol {
                    instruction: *instruction,
                    identifier: debug_symbols[idx],
                },
                Instruction::LoadFast(idx) => PrettyInstruction::Symbol {
                    instruction: *instruction,
                    identifier: debug_symbols[idx as usize],
                },
                Instruction::LoadValue(idx) => PrettyInstruction::Value {
                    instruction: *instruction,
                    value: &self.values[idx as usize],
                },
                _ => PrettyInstruction::Plain(*instruction),
            })
            .enumerate()
            .map(|(i, pretty)| format!("{i:>4}\t{pretty}\n"))
            .collect()
    }
}
