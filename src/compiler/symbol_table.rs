use std::{collections::HashMap, convert::TryInto, mem};

use crate::ast::{value::Value, Identifier, IdentifierKind};

use super::{CompilerError, CompilerResult};

#[derive(Debug, Default)]
pub struct SymbolTable {
    table: HashMap<String, (IdentifierKind, u16)>,
    values: Vec<Value>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, identifier: Identifier) -> CompilerResult<u16> {
        if self.contains(&identifier.ident) {
            return Err(CompilerError::Redefinition);
        }

        let idx = self.next_identifier()?;
        self.table.insert(identifier.ident, (identifier.kind, idx));
        Ok(idx)
    }

    pub fn get(&self, ident: &str) -> Option<(IdentifierKind, u16)> {
        if let Some(identifier) = self.table.get(ident) {
            Some(*identifier)
        } else {
            None
        }
    }

    pub fn contains(&self, identifier: &str) -> bool {
        self.table.contains_key(identifier)
    }

    pub fn register_value(&mut self, value: Value) -> Result<u16, CompilerError> {
        let index = self.next_constant()?;
        self.values.push(value);
        Ok(index)
    }

    fn next_identifier(&self) -> Result<u16, CompilerError> {
        let count = self.table.len();
        match count.try_into() {
            Ok(index) => Ok(index),
            Err(_) => Err(CompilerError::VariableLimitReached),
        }
    }

    fn next_constant(&self) -> Result<u16, CompilerError> {
        let count = self.values.len();
        match count.try_into() {
            Ok(index) => Ok(index),
            Err(_) => Err(CompilerError::VariableLimitReached),
        }
    }

    pub fn get_symbol(&self, index: u16) -> Option<&String> {
        let result = self.table.iter().find(|(_, (_, idx))| *idx == index);

        match result {
            Some((identifier, _)) => Some(identifier),
            None => None,
        }
    }

    pub fn get_value(&self, index: u16) -> Option<&Value> {
        self.values.get(index as usize)
    }

    pub fn finish<'a>(&'a mut self) -> (Vec<Value>, Vec<&'a String>) {
        let values = mem::take(&mut self.values);
        let debug_symbols: Vec<_> = self.table.keys().collect();
        (values, debug_symbols)
    }
}
