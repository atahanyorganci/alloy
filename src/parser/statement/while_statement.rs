use std::fmt;

use pest::iterators::Pair;

use crate::{
    compiler::{Compile, Compiler, CompilerError},
    parser::{
        expression::build_expression, statement::build_statements, ASTNode, Expression, Rule,
        Statement,
    },
};

#[derive(Debug)]
pub struct WhileStatement {
    condition: Box<dyn Expression>,
    body: Vec<Box<dyn Statement>>,
}

impl Compile for WhileStatement {
    fn compile(&self, _compiler: &mut Compiler) -> Result<(), CompilerError> {
        todo!()
    }
}

impl Statement for WhileStatement {
    fn eval(&self) {
        todo!()
    }
}

impl ASTNode for WhileStatement {
    fn build(pair: Pair<Rule>) -> Option<Box<Self>>
    where
        Self: Sized,
    {
        let mut inner = match pair.as_rule() {
            Rule::while_statement => pair.into_inner(),
            _ => return None,
        };

        let k_while = inner.next().unwrap().as_rule();
        debug_assert_eq!(k_while, Rule::k_while);

        let expression = inner.next().unwrap();
        let condition = build_expression(expression).unwrap();

        let mut statement_pairs = inner.next().unwrap().into_inner();
        let body = build_statements(&mut statement_pairs);

        Some(Box::from(WhileStatement { condition, body }))
    }
}

impl fmt::Display for WhileStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use pest::{iterators::Pair, Parser};

    use crate::parser::{ASTNode, AlloyParser, Rule};

    use super::WhileStatement;

    fn statement_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::program, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    fn build_while_statement(input: &str) -> Box<WhileStatement> {
        let pair = statement_pair(input).unwrap();
        WhileStatement::build(pair).unwrap()
    }

    #[test]
    fn test_while_statement() {
        build_while_statement("while true {}");
        build_while_statement("while true { print 4; }");
        build_while_statement("while true { print 4; print 2; }");
    }

    #[test]
    fn test_wrong_while_statements() {
        assert!(statement_pair("while {}").is_none());
        assert!(statement_pair("while true").is_none());
        assert!(statement_pair("while true }").is_none());
        assert!(statement_pair("while true {").is_none());
    }
}
