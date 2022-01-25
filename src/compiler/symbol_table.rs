use std::{collections::HashMap, mem};

use crate::ast::{
    identifier::{Identifier, IdentifierKind},
    value::Value,
};

use super::{CompilerError, CompilerResult};

#[derive(Debug, Default)]
pub struct SymbolTable {
    table: HashMap<String, (IdentifierKind, usize)>,
    values: Vec<Value>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, identifier: Identifier) -> CompilerResult<usize> {
        if self.contains(&identifier.ident) {
            return Err(CompilerError::Redefinition(identifier.ident));
        }
        let idx = self.next_identifier();
        self.table.insert(identifier.ident, (identifier.kind, idx));
        Ok(idx)
    }

    pub fn get(&self, ident: &str) -> Option<(IdentifierKind, usize)> {
        self.table.get(ident).copied()
    }

    pub fn contains(&self, identifier: &str) -> bool {
        self.table.contains_key(identifier)
    }

    pub fn register_value(&mut self, value: Value) -> usize {
        let index = self.next_constant();
        self.values.push(value);
        index
    }

    fn next_identifier(&self) -> usize {
        self.table.len()
    }

    fn next_constant(&self) -> usize {
        self.values.len()
    }

    pub fn get_symbol(&self, index: usize) -> Option<&String> {
        let result = self.table.iter().find(|(_, (_, idx))| *idx == index);

        match result {
            Some((identifier, _)) => Some(identifier),
            None => None,
        }
    }

    pub fn get_value(&self, index: u16) -> Option<&Value> {
        self.values.get(index as usize)
    }

    pub fn finish(&mut self) -> (Vec<Value>, Vec<&'_ String>) {
        let values = mem::take(&mut self.values);
        let debug_symbols: Vec<_> = self.table.keys().collect();
        (values, debug_symbols)
    }
}
