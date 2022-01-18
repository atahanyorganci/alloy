use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::expression::Expression,
    compiler::{BlockType, Compile, Compiler, CompilerError},
    parser::{self, Parse, ParserError, Rule},
};

use super::Statement;

pub struct ConditionalStatement {
    condition: Expression,
    statements: Vec<Statement>,
}

impl Compile for ConditionalStatement {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        self.condition.compile(compiler)?;
        let condition_failed = compiler.emit_untargeted_jump();
        for statement in &self.statements {
            statement.compile(compiler)?;
        }
        let exit = compiler.emit_untargeted_jump();
        compiler.target_jump_on_exit(BlockType::If, exit);
        compiler.target_jump(condition_failed);
        Ok(())
    }
}

pub struct IfStatement {
    if_statement: ConditionalStatement,
    else_if_statements: Vec<ElseIfStatement>,
    else_statement: Option<ElseStatement>,
}

impl fmt::Debug for IfStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("IfStatement");
        debug
            .field("condition", &self.if_statement.condition)
            .field("body", &self.if_statement.statements);
        if self.has_else_if() {
            debug.field("else_if_statements", &self.else_if_statements);
        }
        if self.has_else() {
            debug.field("else_statement", &self.else_statement);
        }
        debug.finish()
    }
}

impl fmt::Display for IfStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Parse<'_> for IfStatement {
    fn parse(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::if_statement);
        let mut inner = pair.into_inner();

        // If body
        let mut if_body = inner.next().unwrap().into_inner();
        matches!(if_body.next().unwrap().as_rule(), Rule::k_if);

        let expression = if_body.next().unwrap();
        let condition = Expression::parse(expression)?;

        let statement_pairs = if_body.next().unwrap().into_inner();
        let statements = parser::parse_pairs(statement_pairs)?;
        let if_statement = ConditionalStatement {
            condition,
            statements,
        };

        let mut else_if_statements = Vec::new();
        let mut else_statement = None;

        // ElseIfs or Else
        for else_pair in inner {
            match else_pair.as_rule() {
                Rule::else_if_body => else_if_statements.push(ElseIfStatement::parse(else_pair)?),
                Rule::else_body => else_statement = Some(ElseStatement::parse(else_pair)?),
                _ => unreachable!(),
            }
        }

        Ok(IfStatement {
            if_statement,
            else_if_statements,
            else_statement,
        })
    }
}

impl Compile for IfStatement {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        compiler.enter_block(BlockType::If);
        self.if_statement.compile(compiler)?;
        for else_if_statement in &self.else_if_statements {
            else_if_statement.compile(compiler)?;
        }
        if let Some(ref else_statement) = self.else_statement {
            else_statement.compile(compiler)?;
        }
        compiler.exit_block();
        Ok(())
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

pub struct ElseIfStatement(ConditionalStatement);

impl fmt::Debug for ElseIfStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ElseIfStatement")
            .field("condition", &self.0.condition)
            .field("body", &self.0.statements)
            .finish()
    }
}

impl fmt::Display for ElseIfStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Parse<'_> for ElseIfStatement {
    fn parse(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::else_if_body);
        let mut inner = pair.into_inner();

        matches!(inner.next().unwrap().as_rule(), Rule::k_else);
        matches!(inner.next().unwrap().as_rule(), Rule::k_if);

        let expression = inner.next().unwrap();
        let condition = Expression::parse(expression).unwrap();

        let statement_pairs = inner.next().unwrap().into_inner();
        let statements = parser::parse_pairs(statement_pairs)?;

        Ok(ElseIfStatement(ConditionalStatement {
            condition,
            statements,
        }))
    }
}

impl Compile for ElseIfStatement {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        self.0.compile(compiler)
    }
}

#[derive(Debug)]
pub struct ElseStatement {
    statements: Vec<Statement>,
}

impl fmt::Display for ElseStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Parse<'_> for ElseStatement {
    fn parse(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        let mut inner = pair.into_inner();

        matches!(inner.next().unwrap().as_rule(), Rule::k_else);

        let statement_pairs = inner.next().unwrap().into_inner();
        let statements = parser::parse_pairs(statement_pairs)?;

        Ok(ElseStatement { statements })
    }
}

impl Compile for ElseStatement {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        for statement in &self.statements {
            statement.compile(compiler)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use pest::{iterators::Pair, Parser};

    use crate::parser::{AlloyParser, Parse, ParserError, Rule};

    use super::IfStatement;

    fn statement_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::program, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    fn build_if_statement(input: &str) -> Result<IfStatement, ParserError> {
        let pair = statement_pair(input).unwrap();
        IfStatement::parse(pair)
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
