use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::{expression::Expression, statement::build_statements},
    compiler::{Compile, Compiler, CompilerError},
    parser::{ASTNode, ParserError, Rule},
};

use super::Statement;

#[derive(Debug)]
pub struct IfStatement {
    condition: Expression,
    statements: Vec<Statement>,
    else_if_statements: Vec<ElseIfStatement>,
    else_statement: Option<ElseStatement>,
}

impl Compile for IfStatement {
    fn compile(&self, _compiler: &mut Compiler) -> Result<(), CompilerError> {
        todo!()
    }
}

impl ASTNode<'_> for IfStatement {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::if_statement);
        let mut inner = pair.into_inner();

        // If body
        let mut if_body = inner.next().unwrap().into_inner();
        matches!(if_body.next().unwrap().as_rule(), Rule::k_if);

        let expression = if_body.next().unwrap();
        let condition = Expression::build(expression)?;

        let mut statement_pairs = if_body.next().unwrap().into_inner();
        let statements = build_statements(&mut statement_pairs)?;

        let mut else_if_statements = Vec::new();
        let mut else_statement = None;

        // ElseIfs or Else
        for else_pair in inner {
            match else_pair.as_rule() {
                Rule::else_if_body => else_if_statements.push(ElseIfStatement::build(else_pair)?),
                Rule::else_body => else_statement = Some(ElseStatement::build(else_pair)?),
                _ => unreachable!(),
            }
        }

        Ok(IfStatement {
            condition,
            statements,
            else_if_statements,
            else_statement,
        })
    }
}

impl fmt::Display for IfStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl IfStatement {
    fn has_else(&self) -> bool {
        self.else_statement.is_some()
    }

    fn has_else_if(&self) -> bool {
        !self.else_if_statements.is_empty()
    }
}

#[derive(Debug)]
pub struct ElseIfStatement {
    condition: Expression,
    statements: Vec<Statement>,
}

impl Compile for ElseIfStatement {
    fn compile(&self, _compiler: &mut Compiler) -> Result<(), CompilerError> {
        todo!()
    }
}

impl ASTNode<'_> for ElseIfStatement {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::else_if_body);
        let mut inner = pair.into_inner();

        matches!(inner.next().unwrap().as_rule(), Rule::k_else);
        matches!(inner.next().unwrap().as_rule(), Rule::k_if);

        let expression = inner.next().unwrap();
        let condition = Expression::build(expression).unwrap();

        let mut statement_pairs = inner.next().unwrap().into_inner();
        let statements = build_statements(&mut statement_pairs)?;

        Ok(ElseIfStatement {
            condition,
            statements,
        })
    }
}

impl fmt::Display for ElseIfStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub struct ElseStatement {
    statements: Vec<Statement>,
}

impl Compile for ElseStatement {
    fn compile(&self, _compiler: &mut Compiler) -> Result<(), CompilerError> {
        todo!()
    }
}

impl ASTNode<'_> for ElseStatement {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        let mut inner = pair.into_inner();

        matches!(inner.next().unwrap().as_rule(), Rule::k_else);

        let mut statement_pairs = inner.next().unwrap().into_inner();
        let statements = build_statements(&mut statement_pairs)?;

        Ok(ElseStatement { statements })
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

    use crate::parser::{ASTNode, AlloyParser, ParserError, Rule};

    use super::IfStatement;

    fn statement_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::program, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    fn build_if_statement(input: &str) -> Result<IfStatement, ParserError> {
        let pair = statement_pair(input).unwrap();
        IfStatement::build(pair)
    }

    #[test]
    fn test_if_statement() -> Result<(), ParserError> {
        build_if_statement("if true {}")?;
        build_if_statement("if false { print 2; }")?;
        build_if_statement("if false {} else if true {}")?;
        build_if_statement("if false {} else if true {} else if true {}")?;
        build_if_statement("if false {} else if true {} else if true {} else if true {} ")?;
        build_if_statement("if false {} else if true {} else {}")?;
        build_if_statement("if false {} else {}")?;
        Ok(())
    }

    #[test]
    fn test_wrong_if_statements() {
        assert!(statement_pair("if {}").is_none());
        assert!(statement_pair("if true print 2; }").is_none());
        assert!(statement_pair("if true { print 2;").is_none());
    }
}
