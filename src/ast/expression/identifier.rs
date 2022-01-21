use std::fmt;

use pest::iterators::Pair;

use crate::{
    compiler::{Compile, Compiler, CompilerError, Instruction},
    parser::{Parse, ParserError, Rule},
};

#[derive(PartialEq, Eq)]
pub struct IdentifierExpression {
    pub ident: String,
}

impl Compile for IdentifierExpression {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        let instruction = match compiler.get_identifier(&self.ident) {
            Some((_, idx)) => Instruction::LoadSymbol(idx),
            None => return Err(CompilerError::UndefinedIdentifer(self.ident.to_owned())),
        };
        compiler.emit(instruction);
        Ok(())
    }
}

impl Parse<'_> for IdentifierExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
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
    use crate::parser::{parse_rule, ParseResult, Rule};

    use super::IdentifierExpression;

    fn parse_identifer(input: &str) -> ParseResult<IdentifierExpression> {
        parse_rule::<IdentifierExpression>(Rule::identifier, input)
    }

    #[test]
    fn test_valid_identifiers() -> ParseResult<()> {
        parse_identifer("abc")?;
        parse_identifer("abc12")?;
        parse_identifer("a12bc")?;
        parse_identifer("Abc")?;
        parse_identifer("ABC12")?;
        parse_identifer("a12BC")?;
        parse_identifer("abc_12")?;
        parse_identifer("a_12bc")?;
        Ok(())
    }

    #[test]
    fn test_invalid_identifiers() {
        parse_identifer("_abc").unwrap_err();
        parse_identifer("__abc").unwrap_err();
        parse_identifer("12abc").unwrap_err();
        parse_identifer("_12abc").unwrap_err();
        parse_identifer("1_abc").unwrap_err();
        parse_identifer("1_2abc").unwrap_err();
    }

    #[test]
    fn test_keywords_as_identifiers() {
        parse_identifer("if").unwrap_err();
        parse_identifer("else").unwrap_err();
        parse_identifer("print").unwrap_err();
        parse_identifer("while").unwrap_err();
        parse_identifer("for").unwrap_err();
        parse_identifer("return").unwrap_err();
        parse_identifer("var").unwrap_err();
        parse_identifer("const").unwrap_err();
        parse_identifer("and").unwrap_err();
        parse_identifer("or").unwrap_err();
        parse_identifer("not").unwrap_err();
        parse_identifer("xor").unwrap_err();
        parse_identifer("continue").unwrap_err();
        parse_identifer("break").unwrap_err();
        parse_identifer("in").unwrap_err();
    }
}
