use std::borrow::Borrow;

use crate::parser::statement::{
    declare_assign_statement::{AssignmentStatement, DeclarationStatement},
    for_statement::ForStatement,
    if_statement::IfStatement,
    while_statement::WhileStatement,
};

use super::{expression::build_binary_expression, Expression, Rule, Statement};
use pest::iterators::{Pair, Pairs};

pub mod declare_assign_statement;
pub mod for_statement;
pub mod if_statement;
pub mod while_statement;

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

pub struct BlockStatement {
    body: Vec<Box<dyn Statement>>,
}

impl Statement for BlockStatement {
    fn eval(&self) {
        let statements: &Vec<_> = self.body.borrow();
        statements.iter().for_each(|statement| statement.eval());
    }

    fn build(pair: Pair<Rule>) -> Box<Self>
    where
        Self: Sized,
    {
        let mut inner = pair.into_inner();
        let statements = inner.next().unwrap();
        match statements.as_rule() {
            Rule::statements => {}
            _ => unreachable!(),
        }
        Box::from(BlockStatement {
            body: build_statements(&mut statements.into_inner()),
        })
    }
}

pub struct BreakStatement;

impl Statement for BreakStatement {
    fn eval(&self) {
        todo!()
    }

    fn build(pair: Pair<Rule>) -> Box<Self> {
        let mut inner = pair.into_inner();
        match inner.next().unwrap().as_rule() {
            Rule::k_break => Box::from(BreakStatement {}),
            _ => unreachable!(),
        }
    }
}
pub struct ContinueStatement;

impl Statement for ContinueStatement {
    fn eval(&self) {
        todo!()
    }

    fn build(pair: Pair<Rule>) -> Box<Self> {
        let mut inner = pair.into_inner();
        match inner.next().unwrap().as_rule() {
            Rule::k_continue => Box::from(ContinueStatement {}),
            _ => unreachable!(),
        }
    }
}

pub fn build_statement(pair: Pair<Rule>) -> Box<dyn Statement> {
    match pair.as_rule() {
        Rule::print_statement => PrintStatement::build(pair),
        Rule::if_statement => IfStatement::build(pair),
        Rule::declaration_statement => DeclarationStatement::build(pair),
        Rule::assignment_statement => AssignmentStatement::build(pair),
        Rule::while_statement => WhileStatement::build(pair),
        Rule::for_statement => ForStatement::build(pair),
        Rule::block_statement => BlockStatement::build(pair),
        Rule::continue_statement => ContinueStatement::build(pair),
        Rule::break_statement => BreakStatement::build(pair),
        _ => panic!("{}", pair),
    }
}

pub fn build_statements(pairs: &mut Pairs<Rule>) -> Vec<Box<dyn Statement>> {
    let mut statements = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::EOI => break,
            _ => {
                dbg!("{}", &pair);
                statements.push(build_statement(pair))
            }
        }
    }
    statements
}

#[cfg(test)]
mod test {
    use pest::{iterators::Pair, Parser};

    use crate::parser::{AlloyParser, Rule, Statement};

    use super::{BlockStatement, PrintStatement};

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

    fn build_block_statement(input: &str) -> Box<BlockStatement> {
        let pair = statement_pair(input).unwrap();
        BlockStatement::build(pair)
    }

    #[test]
    fn test_block_statement() {
        build_block_statement("{}");
        build_block_statement("{ print 24; }");
        build_block_statement("{ print 24; print 24; }");
        build_block_statement("{ print 24; print 24; print 24; }");
    }

    #[test]
    fn test_wrong_block_statements() {
        assert!(statement_pair("{ print 24; ").is_none());
        assert!(statement_pair("print 24; }").is_none());
    }
}
