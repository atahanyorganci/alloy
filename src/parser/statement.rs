use std::borrow::Borrow;

use super::{expression::build_binary_expression, Rule};
use pest::iterators::{Pair, Pairs};

use super::{Expression, Statement};

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

pub struct IfStatement {
    condition: Box<dyn Expression>,
    statements: Vec<Box<dyn Statement>>,
}

impl Statement for IfStatement {
    fn eval(&self) {
        let condition = self.condition.eval();
        if condition.into() {
            let statements: &Vec<_> = self.statements.borrow();
            statements.iter().for_each(|statement| statement.eval());
        }
    }

    fn build(pair: Pair<Rule>) -> Box<Self> {
        let mut inner = pair.into_inner();

        let k_if = inner.next().unwrap().as_rule();
        debug_assert_eq!(k_if, Rule::k_if);

        let expression = inner.next().unwrap();
        let condition = build_binary_expression(expression.into_inner());

        let mut statement_pairs = inner.next().unwrap().into_inner();
        let statements = build_statements(&mut &mut statement_pairs);

        Box::from(IfStatement {
            condition,
            statements,
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
            _ => {
                let statement = build_statement(pair);
                statements.push(statement);
            }
        }
    }
    statements
}
