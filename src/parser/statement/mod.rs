use std::fmt;

use pest::iterators::{Pair, Pairs};

use crate::{
    compiler::{Compile, Compiler, CompilerError, Instruction},
    parser::statement::{
        declare_assign_statement::{AssignmentStatement, DeclarationStatement},
        for_statement::ForStatement,
        if_statement::IfStatement,
        while_statement::WhileStatement,
    },
};

use super::{expression::Expression, ASTNode, ParserError, Rule};

pub mod declare_assign_statement;
pub mod for_statement;
pub mod if_statement;
pub mod while_statement;

#[derive(Debug)]
pub enum Statement {
    Print(PrintStatement),
    If(IfStatement),
    Declaration(DeclarationStatement),
    Assignment(AssignmentStatement),
    While(WhileStatement),
    For(ForStatement),
    Block(BlockStatement),
    Continue(ContinueStatement),
    Break(BreakStatement),
    Expression(ExpressionStatement),
}

impl From<PrintStatement> for Statement {
    fn from(s: PrintStatement) -> Self {
        Self::Print(s)
    }
}

impl From<IfStatement> for Statement {
    fn from(s: IfStatement) -> Self {
        Self::If(s)
    }
}

impl From<DeclarationStatement> for Statement {
    fn from(s: DeclarationStatement) -> Self {
        Self::Declaration(s)
    }
}

impl From<AssignmentStatement> for Statement {
    fn from(s: AssignmentStatement) -> Self {
        Self::Assignment(s)
    }
}

impl From<WhileStatement> for Statement {
    fn from(s: WhileStatement) -> Self {
        Self::While(s)
    }
}

impl From<ForStatement> for Statement {
    fn from(s: ForStatement) -> Self {
        Self::For(s)
    }
}

impl From<BlockStatement> for Statement {
    fn from(s: BlockStatement) -> Self {
        Self::Block(s)
    }
}

impl From<ContinueStatement> for Statement {
    fn from(s: ContinueStatement) -> Self {
        Self::Continue(s)
    }
}

impl From<BreakStatement> for Statement {
    fn from(s: BreakStatement) -> Self {
        Self::Break(s)
    }
}

impl From<ExpressionStatement> for Statement {
    fn from(s: ExpressionStatement) -> Self {
        Self::Expression(s)
    }
}

// impl Compile for Statement {
//     fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
//         match self {
//             Statement::Print(s) => s.compile(compiler),
//             Statement::Block(s) => s.compile(compiler),
//             Statement::If(s) => s.compile(compiler),
//             Statement::Declaration(s) => s.compile(compiler),
//             Statement::Assignment(s) => s.compile(compiler),
//             Statement::While(s) => s.compile(compiler),
//             Statement::For(s) => s.compile(compiler),
//             Statement::Continue(s) => s.compile(compiler),
//             Statement::Break(s) => s.compile(compiler),
//             Statement::Expression(s) => s.compile(compiler),
//         }
//     }
// }

impl ASTNode<'_> for Statement {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        let statement = match pair.as_rule() {
            Rule::print_statement => PrintStatement::build(pair)?.into(),
            Rule::if_statement => IfStatement::build(pair)?.into(),
            Rule::declaration_statement => DeclarationStatement::build(pair)?.into(),
            Rule::assignment_statement => AssignmentStatement::build(pair)?.into(),
            Rule::while_statement => WhileStatement::build(pair)?.into(),
            Rule::for_statement => ForStatement::build(pair)?.into(),
            Rule::block_statement => BlockStatement::build(pair)?.into(),
            Rule::continue_statement => ContinueStatement::build(pair)?.into(),
            Rule::break_statement => BreakStatement::build(pair)?.into(),
            Rule::expression_statement => ExpressionStatement::build(pair)?.into(),
            _ => unreachable!(),
        };
        Ok(statement)
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Print(s) => write!(f, "{}", s),
            Statement::Block(s) => write!(f, "{}", s),
            Statement::If(s) => write!(f, "{}", s),
            Statement::Declaration(s) => write!(f, "{}", s),
            Statement::Assignment(s) => write!(f, "{}", s),
            Statement::While(s) => write!(f, "{}", s),
            Statement::For(s) => write!(f, "{}", s),
            Statement::Continue(s) => write!(f, "{}", s),
            Statement::Break(s) => write!(f, "{}", s),
            Statement::Expression(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug)]
pub struct PrintStatement {
    expression: Expression,
}

impl Compile for PrintStatement {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        compiler.emit(Instruction::Display);
        Ok(())
    }
}

impl ASTNode<'_> for PrintStatement {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::print_statement);

        let mut inner = pair.into_inner();
        matches!(inner.next().unwrap().as_rule(), Rule::k_print);

        let expression = Expression::build(inner.next().unwrap())?;
        Ok(PrintStatement { expression })
    }
}

impl fmt::Display for PrintStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub struct BlockStatement {
    body: Vec<Statement>,
}

impl Compile for BlockStatement {
    fn compile(&self, _compiler: &mut Compiler) -> Result<(), CompilerError> {
        todo!()
    }
}

impl ASTNode<'_> for BlockStatement {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::block_statement);
        let mut inner = pair.into_inner();

        let statements = inner.next().unwrap();
        matches!(statements.as_rule(), Rule::statements);
        let body = build_statements(&mut statements.into_inner())?;
        Ok(BlockStatement { body })
    }
}

impl fmt::Display for BlockStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub struct BreakStatement;

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

impl ASTNode<'_> for BreakStatement {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::break_statement);
        Ok(Self {})
    }
}

impl fmt::Display for BreakStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BreakStatement")
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    expression: Expression,
}

// impl Compile for ExpressionStatement {
//     fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
//         self.expression.compile(compiler)?;
//         compiler.emit(Instruction::Pop);
//         Ok(())
//     }
// }

impl ASTNode<'_> for ExpressionStatement {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::expression_statement);

        let expression_pair = pair.into_inner().next().unwrap();
        let expression = Expression::build(expression_pair)?;
        Ok(Self { expression })
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

impl ASTNode<'_> for ContinueStatement {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::continue_statement);
        Ok(Self {})
    }
}

impl fmt::Display for ContinueStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "continue;")
    }
}

pub fn build_statements(pairs: &mut Pairs<Rule>) -> Result<Vec<Statement>, ParserError> {
    let mut statements = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::EOI => break,
            _ => statements.push(Statement::build(pair)?),
        }
    }
    Ok(statements)
}

#[cfg(test)]
mod test {
    use pest::{iterators::Pair, Parser};

    use crate::parser::{ASTNode, AlloyParser, ParserError, Rule};

    use super::{BlockStatement, PrintStatement};

    fn statement_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::program, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    fn build_print(input: &str) -> Result<PrintStatement, ParserError> {
        let pair = statement_pair(input).unwrap();
        PrintStatement::build(pair)
    }

    #[test]
    fn test_print_statement() -> Result<(), ParserError> {
        build_print("print 1;")?;
        build_print("print 1 * 2;")?;
        build_print("print 3 < 5;")?;
        build_print("print 24 - 12;")?;
        build_print("print 124;")?;
        Ok(())
    }

    #[test]
    fn test_wrong_print_statements() {
        assert!(statement_pair("print 2").is_none());
        assert!(statement_pair("print;").is_none());
    }

    fn build_block(input: &str) -> Result<BlockStatement, ParserError> {
        let pair = statement_pair(input).unwrap();
        BlockStatement::build(pair)
    }

    #[test]
    fn test_block_statement() -> Result<(), ParserError> {
        build_block("{}")?;
        build_block("{ print 24; }")?;
        build_block("{ print 24; print 24; }")?;
        build_block("{ print 24; print 24; print 24; }")?;
        Ok(())
    }

    #[test]
    fn test_wrong_block_statements() {
        assert!(statement_pair("{ print 24; ").is_none());
        assert!(statement_pair("print 24; }").is_none());
    }
}
