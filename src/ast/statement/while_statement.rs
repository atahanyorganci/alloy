use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::expression::Expression,
    compiler::{BlockType, Compile, Compiler, CompilerError, Instruction},
    parser::{self, Parse, ParserError, Rule},
};

use super::Statement;

#[derive(Debug)]
pub struct WhileStatement {
    condition: Expression,
    body: Vec<Statement>,
}

impl Compile for WhileStatement {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        compiler.enter_block(BlockType::While);

        let condition_label = compiler.place_label();
        self.condition.compile(compiler)?;
        let exit = compiler.emit_untargeted_jump_if_false();
        compiler.target_jump_on_exit(BlockType::While, exit);

        for statement in &self.body {
            statement.compile(compiler)?;
        }
        compiler.emit(Instruction::Jump(condition_label.target()?));
        compiler.exit_block();
        Ok(())
    }
}

impl Parse<'_> for WhileStatement {
    fn parse(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::while_statement);
        let mut inner = pair.into_inner();

        matches!(inner.next().unwrap().as_rule(), Rule::k_while);
        let expression = inner.next().unwrap();
        let condition = Expression::parse(expression)?;

        let statement_pairs = inner.next().unwrap().into_inner();
        let body = parser::parse_pairs(statement_pairs)?;

        Ok(WhileStatement { condition, body })
    }
}

impl fmt::Display for WhileStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{self, ParseResult, ParserError};

    use super::WhileStatement;

    fn parse_while(input: &str) -> ParseResult<()> {
        parser::parse_statement::<WhileStatement>(input)?;
        Ok(())
    }

    #[test]
    fn test_while_statement() -> Result<(), ParserError> {
        parse_while("while true {}")?;
        parse_while("while true { print 4; }")?;
        parse_while("while true { print 4; print 2; }")?;
        Ok(())
    }

    #[test]
    fn test_wrong_while_statements() {
        parse_while("while {}").unwrap_err();
        parse_while("while true").unwrap_err();
        parse_while("while true }").unwrap_err();
        parse_while("while true {").unwrap_err();
    }
}
