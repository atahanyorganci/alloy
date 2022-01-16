use std::fmt;

use pest::iterators::Pair;

use crate::{
    compiler::{Compile, Compiler, CompilerError, Instruction},
    parser::{ASTNode, ParserError, Rule},
};

#[derive(PartialEq, Eq)]
pub struct IdentifierExpression {
    ident: String,
}

impl Compile for IdentifierExpression {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        let instruction = match compiler.get_identifier(&self.ident) {
            Some((_, idx)) => Instruction::LoadSymbol(idx),
            None => return Err(CompilerError::UndefinedIdentifer),
        };
        compiler.emit(instruction);
        Ok(())
    }
}

impl ASTNode<'_> for IdentifierExpression {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::identifier);
        let ident = String::from(pair.as_str());
        Ok(IdentifierExpression { ident })
    }
}

impl fmt::Debug for IdentifierExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ident)
    }
}

impl fmt::Display for IdentifierExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ident)
    }
}

#[cfg(test)]
mod tests {
    use pest::{iterators::Pair, Parser};

    use crate::parser::{ASTNode, AlloyParser, Rule};

    use super::IdentifierExpression;

    fn identifier_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::identifier, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    fn build_identifier_ast(input: &str) -> Result<IdentifierExpression, ()> {
        match identifier_pair(input) {
            Some(pair) => match IdentifierExpression::build(pair) {
                Ok(identifier) => Ok(identifier),
                Err(_) => Err(()),
            },
            None => Err(()),
        }
    }

    #[test]
    fn test_valid_identifiers() -> Result<(), ()> {
        build_identifier_ast("abc")?;
        build_identifier_ast("abc12")?;
        build_identifier_ast("a12bc")?;
        build_identifier_ast("Abc")?;
        build_identifier_ast("ABC12")?;
        build_identifier_ast("a12BC")?;
        build_identifier_ast("abc_12")?;
        build_identifier_ast("a_12bc")?;
        Ok(())
    }

    #[test]
    fn test_invalid_identifiers() {
        assert!(build_identifier_ast("_abc").is_err());
        assert!(build_identifier_ast("__abc").is_err());
        assert!(build_identifier_ast("12abc").is_err());
        assert!(build_identifier_ast("_12abc").is_err());
        assert!(build_identifier_ast("1_abc").is_err());
        assert!(build_identifier_ast("1_2abc").is_err());
    }

    #[test]
    fn test_keywords_as_identifiers() {
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
