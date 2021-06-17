use pest::iterators::Pair;

use crate::parser::{
    expression::build_binary_expression, statement::build_statements, Expression, Rule, Statement,
};

pub struct ForStatement {
    identifier: String,
    iterator: Box<dyn Expression>,
    body: Vec<Box<dyn Statement>>,
}

impl Statement for ForStatement {
    fn eval(&self) {
        todo!()
    }

    fn build(pair: Pair<Rule>) -> Box<Self> {
        let mut inner = pair.into_inner();

        let k_for = inner.next().unwrap().as_rule();
        debug_assert_eq!(k_for, Rule::k_for);

        let identifier_token = inner.next().unwrap();
        let identifier = match identifier_token.as_rule() {
            Rule::identifier => String::from(identifier_token.as_str()),
            _ => unreachable!(),
        };

        let k_in = inner.next().unwrap().as_rule();
        debug_assert_eq!(k_in, Rule::k_in);

        let expression = inner.next().unwrap();
        let iterator = build_binary_expression(expression.into_inner());

        let mut statement_pairs = inner.next().unwrap().into_inner();
        let body = build_statements(&mut statement_pairs);

        Box::from(ForStatement {
            identifier,
            iterator,
            body,
        })
    }
}

#[cfg(test)]
mod test {
    use pest::{iterators::Pair, Parser};

    use crate::parser::{AlloyParser, Rule, Statement};

    use super::ForStatement;

    fn statement_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::program, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    fn build_for_statement(input: &str) -> Box<ForStatement> {
        let pair = statement_pair(input).unwrap();
        ForStatement::build(pair)
    }

    #[test]
    fn test_for_statement() {
        build_for_statement("for i in 2 {}");
        build_for_statement("for i in 2 { break; }");
        build_for_statement("for i in 2 { continue; }");
        build_for_statement("for i in 2 { print 4; }");
        build_for_statement("for i in 2 { print 4; print 2; }");
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
