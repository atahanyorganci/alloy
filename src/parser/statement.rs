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
    else_if_statements: Vec<ElseIfStatement>,
    else_statement: Option<ElseStatement>,
}

impl Statement for IfStatement {
    fn eval(&self) {
        let condition = self.condition.eval();
        if condition.into() {
            let statements: &Vec<_> = self.statements.borrow();
            statements.iter().for_each(|statement| statement.eval());
        } else {
            let statements: &Vec<_> = self.else_if_statements.borrow();
            for else_if_statement in statements {
                if else_if_statement.do_eval() {
                    return;
                }
            }
            match self.else_statement.borrow() {
                Some(else_statement) => else_statement.eval(),
                None => {}
            }
        }
    }

    fn build(pair: Pair<Rule>) -> Box<Self> {
        let mut inner = pair.into_inner();

        // If body
        let mut if_body = inner.next().unwrap().into_inner();
        let k_if = if_body.next().unwrap().as_rule();
        debug_assert_eq!(k_if, Rule::k_if);

        let expression = if_body.next().unwrap();
        let condition = build_binary_expression(expression.into_inner());

        let mut statement_pairs = if_body.next().unwrap().into_inner();
        let statements = build_statements(&mut statement_pairs);

        let mut else_if_statements = Vec::new();
        let mut else_statement = None;

        // ElseIfs or Else
        for else_pair in inner.into_iter() {
            match else_pair.as_rule() {
                Rule::else_if_body => else_if_statements.push(*ElseIfStatement::build(else_pair)),
                Rule::else_body => else_statement = Some(*ElseStatement::build(else_pair)),
                _ => unreachable!(),
            }
        }

        Box::from(IfStatement {
            condition,
            statements,
            else_if_statements,
            else_statement,
        })
    }
}

pub struct ElseIfStatement {
    condition: Box<dyn Expression>,
    statements: Vec<Box<dyn Statement>>,
}

impl Statement for ElseIfStatement {
    fn eval(&self) {
        self.do_eval();
    }

    fn build(pair: Pair<Rule>) -> Box<Self> {
        let mut inner = pair.into_inner();

        let k_else = inner.next().unwrap().as_rule();
        debug_assert_eq!(k_else, Rule::k_else);

        let k_if = inner.next().unwrap().as_rule();
        debug_assert_eq!(k_if, Rule::k_if);

        let expression = inner.next().unwrap();
        let condition = build_binary_expression(expression.into_inner());

        let mut statement_pairs = inner.next().unwrap().into_inner();
        let statements = build_statements(&mut statement_pairs);

        Box::from(ElseIfStatement {
            condition,
            statements,
        })
    }
}

impl ElseIfStatement {
    pub fn do_eval(&self) -> bool {
        let condition = self.condition.eval().into();
        if condition {
            let statements: &Vec<_> = self.statements.borrow();
            statements.iter().for_each(|statement| statement.eval());
        }
        condition
    }
}

pub struct ElseStatement {
    statements: Vec<Box<dyn Statement>>,
}

impl Statement for ElseStatement {
    fn eval(&self) {
        let statements: &Vec<_> = self.statements.borrow();
        statements.iter().for_each(|statement| statement.eval());
    }

    fn build(pair: Pair<Rule>) -> Box<Self> {
        let mut inner = pair.into_inner();

        let k_else = inner.next().unwrap().as_rule();
        debug_assert_eq!(k_else, Rule::k_else);

        let mut statement_pairs = inner.next().unwrap().into_inner();
        let statements = build_statements(&mut statement_pairs);

        Box::from(ElseStatement { statements })
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

    use super::{IfStatement, PrintStatement};

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

    fn build_if_statement(input: &str) -> Box<IfStatement> {
        let pair = statement_pair(input).unwrap();
        IfStatement::build(pair)
    }

    #[test]
    fn test_if_statement() {
        build_if_statement("if true {}");
        build_if_statement("if false { print 2; }");
        build_if_statement("if false {} else if true {}");
        build_if_statement("if false {} else if true {} else if true {}");
        build_if_statement("if false {} else if true {} else if true {} else if true {} ");
        build_if_statement("if false {} else if true {} else {}");
        build_if_statement("if false {} else {}");
    }

    #[test]
    fn test_wrong_if_statements() {
        assert!(statement_pair("if {}").is_none());
        assert!(statement_pair("if true print 2; }").is_none());
        assert!(statement_pair("if true { print 2;").is_none());
    }
}
