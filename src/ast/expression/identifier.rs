use std::fmt;

use pest::iterators::Pair;

use crate::{
    compiler::{Compile, Compiler, CompilerError, Instruction},
    parser::{ASTNode, ParserError, Rule},
};

pub struct Identifier {
    identifier: String,
}

impl Compile for Identifier {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        let instruction = match compiler.get_identifer(&self.identifier) {
            Some(symbol) => Instruction::LoadSymbol(symbol.index),
            None => return Err(CompilerError::UndefinedIdentifer),
        };
        compiler.emit(instruction);
        Ok(())
    }
}

impl ASTNode<'_> for Identifier {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::identifier);
        let identifier = String::from(pair.as_str());
        Ok(Identifier { identifier })
    }
}

impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use pest::{iterators::Pair, Parser};

    use crate::parser::{AlloyParser, Rule};

    fn identifier_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::identifier, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    #[test]
    fn test_wrong_identifiers() {
        assert!(identifier_pair("if").is_none());
        assert!(identifier_pair("else").is_none());
        assert!(identifier_pair("print").is_none());
        assert!(identifier_pair("while").is_none());
        assert!(identifier_pair("for").is_none());
        assert!(identifier_pair("return").is_none());
        assert!(identifier_pair("var").is_none());
        assert!(identifier_pair("const").is_none());
        assert!(identifier_pair("and").is_none());
        assert!(identifier_pair("or").is_none());
        assert!(identifier_pair("not").is_none());
        assert!(identifier_pair("xor").is_none());
        assert!(identifier_pair("continue").is_none());
        assert!(identifier_pair("break").is_none());
        assert!(identifier_pair("in").is_none());
    }
}
