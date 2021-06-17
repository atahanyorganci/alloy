use std::{borrow::Borrow, fmt};

use pest::iterators::Pair;

use crate::parser::{
    expression::build_expression, statement::build_statements, ASTNode, Expression, Rule, Statement,
};

#[derive(Debug)]
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
}

impl ASTNode for IfStatement {
    fn build(pair: Pair<Rule>) -> Option<Box<Self>> {
        let mut inner = match pair.as_rule() {
            Rule::if_statement => pair.into_inner(),
            _ => return None,
        };

        // If body
        let mut if_body = inner.next().unwrap().into_inner();
        let k_if = if_body.next().unwrap().as_rule();
        debug_assert_eq!(k_if, Rule::k_if);

        let expression = if_body.next().unwrap();
        let condition = build_expression(expression).unwrap();

        let mut statement_pairs = if_body.next().unwrap().into_inner();
        let statements = build_statements(&mut statement_pairs);

        let mut else_if_statements = Vec::new();
        let mut else_statement = None;

        // ElseIfs or Else
        for else_pair in inner.into_iter() {
            match else_pair.as_rule() {
                Rule::else_if_body => {
                    else_if_statements.push(*ElseIfStatement::build(else_pair).unwrap())
                }
                Rule::else_body => else_statement = Some(*ElseStatement::build(else_pair).unwrap()),
                _ => unreachable!(),
            }
        }

        Some(Box::from(IfStatement {
            condition,
            statements,
            else_if_statements,
            else_statement,
        }))
    }
}

impl fmt::Display for IfStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub struct ElseIfStatement {
    condition: Box<dyn Expression>,
    statements: Vec<Box<dyn Statement>>,
}

impl Statement for ElseIfStatement {
    fn eval(&self) {
        self.do_eval();
    }
}

impl ASTNode for ElseIfStatement {
    fn build(pair: Pair<Rule>) -> Option<Box<Self>>
    where
        Self: Sized,
    {
        let mut inner = match pair.as_rule() {
            Rule::else_if_body => pair.into_inner(),
            _ => return None,
        };

        let k_else = inner.next().unwrap().as_rule();
        debug_assert_eq!(k_else, Rule::k_else);

        let k_if = inner.next().unwrap().as_rule();
        debug_assert_eq!(k_if, Rule::k_if);

        let expression = inner.next().unwrap();
        let condition = build_expression(expression).unwrap();

        let mut statement_pairs = inner.next().unwrap().into_inner();
        let statements = build_statements(&mut statement_pairs);

        Some(Box::from(ElseIfStatement {
            condition,
            statements,
        }))
    }
}

impl fmt::Display for ElseIfStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
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

#[derive(Debug)]
pub struct ElseStatement {
    statements: Vec<Box<dyn Statement>>,
}

impl Statement for ElseStatement {
    fn eval(&self) {
        let statements: &Vec<_> = self.statements.borrow();
        statements.iter().for_each(|statement| statement.eval());
    }
}
impl ASTNode for ElseStatement {
    fn build(pair: Pair<Rule>) -> Option<Box<Self>>
    where
        Self: Sized,
    {
        let mut inner = match pair.as_rule() {
            Rule::else_body => pair.into_inner(),
            _ => return None,
        };

        let k_else = inner.next().unwrap().as_rule();
        debug_assert_eq!(k_else, Rule::k_else);

        let mut statement_pairs = inner.next().unwrap().into_inner();
        let statements = build_statements(&mut statement_pairs);

        Some(Box::from(ElseStatement { statements }))
    }
}

impl fmt::Display for ElseStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use pest::{iterators::Pair, Parser};

    use crate::parser::{ASTNode, AlloyParser, Rule};

    use super::IfStatement;

    fn statement_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::program, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    fn build_if_statement(input: &str) -> Box<IfStatement> {
        let pair = statement_pair(input).unwrap();
        IfStatement::build(pair).unwrap()
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
