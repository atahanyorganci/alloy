use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::{expression::Expression, statement::build_statements},
    parser::{ASTNode, ParserError, Rule},
};

use super::Statement;

#[derive(Debug)]
pub struct WhileStatement {
    condition: Box<Expression>,
    body: Vec<Statement>,
}

// impl Compile for WhileStatement {
//     fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
//         let context = compiler.push_loop_context();

//         self.condition.compile(compiler)?;
//         compiler.emit_jump(Instruction::JumpIfFalse(0), context.start_label())?;
//         for statement in &self.body {
//             statement.compile(compiler)?;
//         }
//         compiler.emit(Instruction::Jump(context.start_label().target()));
//         compiler.pop_context()?;
//         Ok(())
//     }
// }

impl ASTNode<'_> for WhileStatement {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::while_statement);
        let mut inner = pair.into_inner();

        matches!(inner.next().unwrap().as_rule(), Rule::k_while);
        let expression = inner.next().unwrap();
        let condition = Box::from(Expression::build(expression)?);

        let mut statement_pairs = inner.next().unwrap().into_inner();
        let body = build_statements(&mut statement_pairs)?;

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
    use pest::{iterators::Pair, Parser};

    use crate::parser::{ASTNode, AlloyParser, ParserError, Rule};

    use super::WhileStatement;

    fn statement_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::program, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    fn build(input: &str) -> Result<WhileStatement, ParserError> {
        let pair = statement_pair(input).unwrap();
        WhileStatement::build(pair)
    }

    #[test]
    fn test_while_statement() -> Result<(), ParserError> {
        build("while true {}")?;
        build("while true { print 4; }")?;
        build("while true { print 4; print 2; }")?;
        Ok(())
    }

    #[test]
    fn test_wrong_while_statements() {
        assert!(statement_pair("while {}").is_none());
        assert!(statement_pair("while true").is_none());
        assert!(statement_pair("while true }").is_none());
        assert!(statement_pair("while true {").is_none());
    }
}
