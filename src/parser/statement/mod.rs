use std::{borrow::Borrow, fmt};

use pest::iterators::{Pair, Pairs};

use crate::{
    compiler::{Compile, Compiler, CompilerError, Instruction},
    parser::{
        expression::build_expression,
        statement::{
            declare_assign_statement::{AssignmentStatement, DeclarationStatement},
            for_statement::ForStatement,
            if_statement::IfStatement,
            while_statement::WhileStatement,
        },
    },
};

use super::{ASTNode, Expression, Rule, Statement};

pub mod declare_assign_statement;
pub mod for_statement;
pub mod if_statement;
pub mod while_statement;

#[derive(Debug)]
pub struct PrintStatement {
    expression: Box<dyn Expression>,
}

impl Statement for PrintStatement {
    fn eval(&self) {
        let val = self.expression.eval();
        println!("{}", val);
    }
}

impl Compile for PrintStatement {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        compiler.emit(Instruction::Display);
        Ok(())
    }
}

impl ASTNode for PrintStatement {
    fn build(pair: Pair<Rule>) -> Option<Box<Self>>
    where
        Self: Sized,
    {
        let mut inner = match pair.as_rule() {
            Rule::print_statement => pair.into_inner(),
            _ => return None,
        };
        let k_print = inner.next().unwrap().as_rule();
        debug_assert_eq!(k_print, Rule::k_print);

        let expression_pair = inner.next().unwrap();
        let expression = build_expression(expression_pair).unwrap();
        Some(Box::from(PrintStatement { expression }))
    }
}

impl fmt::Display for PrintStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub struct BlockStatement {
    body: Vec<Box<dyn Statement>>,
}

impl Compile for BlockStatement {
    fn compile(&self, _compiler: &mut Compiler) -> Result<(), CompilerError> {
        todo!()
    }
}

impl Statement for BlockStatement {
    fn eval(&self) {
        let statements: &Vec<_> = self.body.borrow();
        statements.iter().for_each(|statement| statement.eval());
    }
}

impl ASTNode for BlockStatement {
    fn build(pair: Pair<Rule>) -> Option<Box<Self>>
    where
        Self: Sized,
    {
        let mut inner = match pair.as_rule() {
            Rule::block_statement => pair.into_inner(),
            _ => return None,
        };
        let statements = inner.next().unwrap();
        let body = match statements.as_rule() {
            Rule::statements => build_statements(&mut statements.into_inner()),
            _ => unreachable!(),
        };
        Some(Box::from(BlockStatement { body }))
    }
}

impl fmt::Display for BlockStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub struct BreakStatement;

impl Statement for BreakStatement {
    fn eval(&self) {
        todo!()
    }
}

impl Compile for BreakStatement {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        match compiler.get_loop_context() {
            Some(context) => {
                let loop_end = *context.end_label();
                compiler.emit_jump(Instruction::Jump(0), &loop_end)
            }
            None => Err(CompilerError::BreakOutsideLoop),
        }
    }
}

impl ASTNode for BreakStatement {
    fn build(pair: Pair<Rule>) -> Option<Box<Self>>
    where
        Self: Sized,
    {
        match pair.as_rule() {
            Rule::break_statement => Some(Box::from(Self {})),
            _ => None,
        }
    }
}

impl fmt::Display for BreakStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BreakStatement")
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    expression: Box<dyn Expression>,
}

impl Statement for ExpressionStatement {
    fn eval(&self) {
        self.expression.eval();
    }
}

impl Compile for ExpressionStatement {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        self.expression.compile(compiler)?;
        compiler.emit(Instruction::Pop);
        Ok(())
    }
}

impl ASTNode for ExpressionStatement {
    fn build(pair: Pair<Rule>) -> Option<Box<Self>>
    where
        Self: Sized,
    {
        let expression_pair = match pair.as_rule() {
            Rule::expression_statement => pair.into_inner().next().unwrap(),
            _ => return None,
        };
        let expression = build_expression(expression_pair).unwrap();
        Some(Box::from(Self { expression }))
    }
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub struct ContinueStatement;

impl Compile for ContinueStatement {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        match compiler.get_loop_context() {
            Some(context) => {
                let loop_end = *context.start_label();
                compiler.emit_jump(Instruction::Jump(0), &loop_end)
            }
            None => Err(CompilerError::BreakOutsideLoop),
        }
    }
}

impl Statement for ContinueStatement {
    fn eval(&self) {
        todo!()
    }
}

impl ASTNode for ContinueStatement {
    fn build(pair: Pair<Rule>) -> Option<Box<Self>>
    where
        Self: Sized,
    {
        match pair.as_rule() {
            Rule::continue_statement => Some(Box::from(ContinueStatement {})),
            _ => None,
        }
    }
}

impl fmt::Display for ContinueStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "continue;")
    }
}

pub fn build_statement(pair: Pair<Rule>) -> Box<dyn Statement> {
    match pair.as_rule() {
        Rule::print_statement => PrintStatement::build(pair).unwrap(),
        Rule::if_statement => IfStatement::build(pair).unwrap(),
        Rule::declaration_statement => DeclarationStatement::build(pair).unwrap(),
        Rule::assignment_statement => AssignmentStatement::build(pair).unwrap(),
        Rule::while_statement => WhileStatement::build(pair).unwrap(),
        Rule::for_statement => ForStatement::build(pair).unwrap(),
        Rule::block_statement => BlockStatement::build(pair).unwrap(),
        Rule::continue_statement => ContinueStatement::build(pair).unwrap(),
        Rule::break_statement => BreakStatement::build(pair).unwrap(),
        Rule::expression_statement => ExpressionStatement::build(pair).unwrap(),
        _ => panic!("{}", pair),
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

    use crate::parser::{ASTNode, AlloyParser, Rule};

    use super::{BlockStatement, PrintStatement};

    fn statement_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::program, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    fn build_print_statement(input: &str) -> Box<PrintStatement> {
        let pair = statement_pair(input).unwrap();
        PrintStatement::build(pair).unwrap()
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
        assert!(statement_pair("print 2").is_none());
        assert!(statement_pair("print;").is_none());
    }

    fn build_block_statement(input: &str) -> Box<BlockStatement> {
        let pair = statement_pair(input).unwrap();
        BlockStatement::build(pair).unwrap()
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
