use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::expression::Expression,
    compiler::{Compile, Compiler, CompilerError},
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
    fn compile(&self, _compiler: &mut Compiler) -> Result<(), CompilerError> {
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
    use pest::{iterators::Pair, Parser};

    use crate::parser::{AlloyParser, Parse, ParserError, Rule};

    use super::ForStatement;

    fn statement_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::program, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    fn build_statement(input: &str) -> Result<ForStatement, ParserError> {
        let pair = statement_pair(input).unwrap();
        ForStatement::parse(pair)
    }

    #[test]
    fn test_for_statement() -> Result<(), ParserError> {
        build_statement("for i in 2 {}")?;
        build_statement("for i in 2 { break; }")?;
        build_statement("for i in 2 { continue; }")?;
        build_statement("for i in 2 { print 4; }")?;
        build_statement("for i in 2 { print 4; print 2; }")?;
        Ok(())
    }

    #[test]
    fn test_wrong_for_statements() {
        assert!(statement_pair("for i in {}").is_none());
        assert!(statement_pair("for i 2 {}").is_none());
        assert!(statement_pair("for in 2 {}").is_none());
        assert!(statement_pair("for i in 2").is_none());
        assert!(statement_pair("for i in 2 }").is_none());
        assert!(statement_pair("for i in 2 {").is_none());
    }
}
