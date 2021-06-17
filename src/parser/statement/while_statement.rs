use pest::iterators::Pair;

use crate::parser::{
    expression::build_binary_expression, statement::build_statements, Expression, Rule, Statement,
};

pub struct WhileStatement {
    condition: Box<dyn Expression>,
    body: Vec<Box<dyn Statement>>,
}

impl Statement for WhileStatement {
    fn eval(&self) {
        todo!()
    }

    fn build(pair: Pair<Rule>) -> Box<Self> {
        let mut inner = pair.into_inner();

        let k_while = inner.next().unwrap().as_rule();
        debug_assert_eq!(k_while, Rule::k_while);

        let expression = inner.next().unwrap();
        let condition = build_binary_expression(expression.into_inner());

        let mut statement_pairs = inner.next().unwrap().into_inner();
        let body = build_statements(&mut statement_pairs);

        Box::from(WhileStatement { condition, body })
    }
}

#[cfg(test)]
mod test {
    use pest::{iterators::Pair, Parser};

    use crate::parser::{AlloyParser, Rule, Statement};

    use super::WhileStatement;

    fn statement_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::program, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    fn build_while_statement(input: &str) -> Box<WhileStatement> {
        let pair = statement_pair(input).unwrap();
        WhileStatement::build(pair)
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
