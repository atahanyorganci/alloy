use std::fmt;

use pest::iterators::Pair;

use crate::{
    compiler::{Compile, Compiler, CompilerError, Instruction},
    parser::{self, Parse, ParserError, Rule},
};

use self::{
    declare_assign_statement::{AssignmentStatement, DeclarationStatement},
    for_statement::ForStatement,
    if_statement::IfStatement,
    while_statement::WhileStatement,
};

use super::{
    expression::Expression,
    function::{FunctionStatement, ReturnStatement},
};

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
    Function(FunctionStatement),
    Return(ReturnStatement),
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

impl From<FunctionStatement> for Statement {
    fn from(s: FunctionStatement) -> Self {
        Self::Function(s)
    }
}

impl From<ReturnStatement> for Statement {
    fn from(s: ReturnStatement) -> Self {
        Self::Return(s)
    }
}

impl Compile for Statement {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        match self {
            Statement::Print(s) => s.compile(compiler),
            Statement::Block(s) => s.compile(compiler),
            Statement::If(s) => s.compile(compiler),
            Statement::Declaration(s) => s.compile(compiler),
            Statement::Assignment(s) => s.compile(compiler),
            Statement::While(s) => s.compile(compiler),
            Statement::For(s) => s.compile(compiler),
            Statement::Continue(s) => s.compile(compiler),
            Statement::Break(s) => s.compile(compiler),
            Statement::Expression(s) => s.compile(compiler),
            Statement::Function(s) => s.compile(compiler),
            Statement::Return(s) => s.compile(compiler),
        }
    }
}

impl Parse<'_> for Statement {
    fn parse(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        let statement = match pair.as_rule() {
            Rule::print_statement => PrintStatement::parse(pair)?.into(),
            Rule::if_statement => IfStatement::parse(pair)?.into(),
            Rule::declaration_statement => DeclarationStatement::parse(pair)?.into(),
            Rule::assignment_statement => AssignmentStatement::parse(pair)?.into(),
            Rule::while_statement => WhileStatement::parse(pair)?.into(),
            Rule::for_statement => ForStatement::parse(pair)?.into(),
            Rule::block_statement => BlockStatement::parse(pair)?.into(),
            Rule::continue_statement => ContinueStatement::parse(pair)?.into(),
            Rule::break_statement => BreakStatement::parse(pair)?.into(),
            Rule::expression_statement => ExpressionStatement::parse(pair)?.into(),
            Rule::function_statement => FunctionStatement::parse(pair)?.into(),
            Rule::return_statement => ReturnStatement::parse(pair)?.into(),
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
            Statement::Function(s) => write!(f, "{}", s),
            Statement::Return(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug)]
pub struct PrintStatement {
    expression: Expression,
}

impl Compile for PrintStatement {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        self.expression.compile(compiler)?;
        compiler.emit(Instruction::Display);
        Ok(())
    }
}

impl Parse<'_> for PrintStatement {
    fn parse(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::print_statement);

        let mut inner = pair.into_inner();
        matches!(inner.next().unwrap().as_rule(), Rule::k_print);

        let expression = Expression::parse(inner.next().unwrap())?;
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
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        for statement in &self.body {
            statement.compile(compiler)?;
        }
        Ok(())
    }
}

impl Parse<'_> for BlockStatement {
    fn parse(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::block_statement);
        let body = parser::parse_pairs(pair.into_inner())?;
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
        let jump = compiler.emit_untargeted_jump();
        match compiler.target_jump_on_loop_exit(jump) {
            Some(_) => Ok(()),
            None => Err(CompilerError::BreakOutsideLoop),
        }
    }
}

impl Parse<'_> for BreakStatement {
    fn parse(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
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

impl Compile for ExpressionStatement {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        self.expression.compile(compiler)?;
        compiler.emit(Instruction::Pop);
        Ok(())
    }
}

impl Parse<'_> for ExpressionStatement {
    fn parse(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::expression_statement);

        let expression_pair = pair.into_inner().next().unwrap();
        let expression = Expression::parse(expression_pair)?;
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
        let jump = compiler.emit_untargeted_jump();
        match compiler.target_jump_on_loop_exit(jump) {
            Some(_) => Ok(()),
            None => Err(CompilerError::ContinueOutsideLoop),
        }
    }
}

impl Parse<'_> for ContinueStatement {
    fn parse(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::continue_statement);
        Ok(Self {})
    }
}

impl fmt::Display for ContinueStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "continue;")
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{self, ParseResult};

    use super::{BlockStatement, PrintStatement};

    fn parse_print(input: &str) -> ParseResult<()> {
        parser::parse_statement::<PrintStatement>(input)?;
        Ok(())
    }

    fn parse_block(input: &str) -> ParseResult<()> {
        parser::parse_statement::<BlockStatement>(input)?;
        Ok(())
    }

    #[test]
    fn test_print_statement() -> ParseResult<()> {
        parse_print("print 1;")?;
        parse_print("print 1 * 2;")?;
        parse_print("print 3 < 5;")?;
        parse_print("print 24 - 12;")?;
        parse_print("print 124;")?;
        Ok(())
    }

    #[test]
    fn test_wrong_print_statements() {
        parse_print("print 2").unwrap_err();
        parse_print("print;").unwrap_err();
    }

    #[test]
    fn test_block_statement() -> ParseResult<()> {
        parse_block("{}")?;
        parse_block("{ print 24; }")?;
        parse_block("{ print 24; print 24; }")?;
        parse_block("{ print 24; print 24; print 24; }")?;
        Ok(())
    }

    #[test]
    fn test_wrong_block_statements() {
        parse_block("{ print 24; ").unwrap_err();
        parse_block("print 24; }").unwrap_err();
    }
}
