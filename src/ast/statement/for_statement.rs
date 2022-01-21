use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::expression::Expression,
    compiler::{Compile, Compiler, CompilerResult},
    parser::{self, Parse, ParserError, Rule},
};

use super::Statement;

#[derive(Debug)]
pub struct ForStatement {
    identifier: String,
    iterator: Expression,
    body: Vec<Statement>,
}

impl Compile for ForStatement {
    fn compile(&self, _compiler: &mut Compiler) -> CompilerResult<()> {
        todo!()
    }
}

impl Parse<'_> for ForStatement {
    fn parse(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::for_statement);
        let mut inner = pair.into_inner();

        matches!(inner.next().unwrap().as_rule(), Rule::k_for);

        let identifier_token = inner.next().unwrap();
        let identifier = match identifier_token.as_rule() {
            Rule::identifier => String::from(identifier_token.as_str()),
            _ => unreachable!(),
        };

        matches!(inner.next().unwrap().as_rule(), Rule::k_in);
        let expression = inner.next().unwrap();
        let iterator = Expression::parse(expression)?;

        let statement_pairs = inner.next().unwrap().into_inner();
        let body = parser::parse_pairs(statement_pairs)?;

        Ok(ForStatement {
            identifier,
            iterator,
            body,
        })
    }
}

impl fmt::Display for ForStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{self, ParseResult};

    use super::ForStatement;

    fn parse_for(input: &str) -> ParseResult<()> {
        parser::parse_statement::<ForStatement>(input)?;
        Ok(())
    }

    #[test]
    fn test_for_statement() -> ParseResult<()> {
        parse_for("for i in 2 {}")?;
        parse_for("for i in 2 { break; }")?;
        parse_for("for i in 2 { continue; }")?;
        parse_for("for i in 2 { print 4; }")?;
        parse_for("for i in 2 { print 4; print 2; }")?;
        Ok(())
    }

    #[test]
    fn test_wrong_for_statements() {
        parse_for("for i in {}").unwrap_err();
        parse_for("for i 2 {}").unwrap_err();
        parse_for("for in 2 {}").unwrap_err();
        parse_for("for i in 2").unwrap_err();
        parse_for("for i in 2 }").unwrap_err();
        parse_for("for i in 2 {").unwrap_err();
    }
}
