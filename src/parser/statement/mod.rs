use crate::parser::statement::if_statement::IfStatement;

use super::{expression::build_binary_expression, Expression, Rule, Statement};
use pest::iterators::{Pair, Pairs};

pub mod if_statement;

pub struct PrintStatement {
    expr: Box<dyn Expression>,
}

impl Statement for PrintStatement {
    fn eval(&self) {
        let val = self.expr.eval();
        println!("{}", val);
    }

    fn build(pair: Pair<Rule>) -> Box<Self> {
        let mut inner = pair.into_inner();

        let k_print = inner.next().unwrap().as_rule();
        debug_assert_eq!(k_print, Rule::k_print);

        Box::from(PrintStatement {
            expr: build_binary_expression(inner),
        })
    }
}

pub fn build_statement(pair: Pair<Rule>) -> Box<dyn Statement> {
    match pair.as_rule() {
        Rule::print_statement => PrintStatement::build(pair),
        Rule::if_statement => IfStatement::build(pair),
        _ => unreachable!(),
    }
}

pub fn build_statements(pairs: &mut Pairs<Rule>) -> Vec<Box<dyn Statement>> {
    let mut statements = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::EOI => break,
            _ => statements.push(build_statement(pair)),
        }
    }
    statements
}

#[cfg(test)]
mod test {
    use pest::{iterators::Pair, Parser};

    use crate::parser::{AlloyParser, Rule, Statement};

    use super::PrintStatement;

    fn statement_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::program, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    fn build_print_statement(input: &str) -> Box<PrintStatement> {
        let pair = statement_pair(input).unwrap();
        PrintStatement::build(pair)
    }

    #[test]
    fn test_print_statement() {
        build_print_statement("print 1;");
        build_print_statement("print 1 * 2;");
        build_print_statement("print 3 < 5;");
        build_print_statement("print 24 - 12;");
        build_print_statement("print 124;");
    }

    #[test]
    fn test_wrong_print_statements() {
        assert!(statement_pair("print2;").is_none());
        assert!(statement_pair("print 2").is_none());
        assert!(statement_pair("print;").is_none());
    }
}
