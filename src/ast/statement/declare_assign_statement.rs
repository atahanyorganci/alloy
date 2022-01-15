use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::expression::Expression,
    compiler::{Compile, Compiler, CompilerError},
    parser::{ASTNode, ParserError, Rule},
};

#[derive(Debug, Clone, Copy)]
pub enum VariableKind {
    Constant,
    Variable,
}

#[derive(Debug)]
pub struct DeclarationStatement {
    identifier: String,
    initial_value: Option<Expression>,
    kind: VariableKind,
}

impl Compile for DeclarationStatement {
    fn compile(&self, _compiler: &mut Compiler) -> Result<(), CompilerError> {
        todo!()
    }
}

impl ASTNode<'_> for DeclarationStatement {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::declaration_statement);
        let mut inner = pair.into_inner();

        let modifier_keyword = inner.next().unwrap();
        let modifier = match modifier_keyword.as_rule() {
            Rule::k_const => VariableKind::Constant,
            Rule::k_var => VariableKind::Variable,
            _ => unreachable!(),
        };

        let identifier_token = inner.next().unwrap();
        let identifier = match identifier_token.as_rule() {
            Rule::identifier => String::from(identifier_token.as_str()),
            _ => unreachable!(),
        };

        let initial_value = match inner.next() {
            Some(token) => Some(Expression::build(token)?),
            None => None,
        };

        Ok(DeclarationStatement {
            identifier,
            initial_value,
            kind: modifier,
        })
    }
}

impl fmt::Display for DeclarationStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub struct AssignmentStatement {
    identifier: String,
    value: Expression,
}

impl Compile for AssignmentStatement {
    fn compile(&self, _compiler: &mut Compiler) -> Result<(), CompilerError> {
        todo!()
    }
}

impl ASTNode<'_> for AssignmentStatement {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::assignment_statement);
        let mut inner = pair.into_inner();

        let identifier_token = inner.next().unwrap();
        matches!(identifier_token.as_rule(), Rule::identifier);
        let identifier = String::from(identifier_token.as_str());

        let expression = inner.next().unwrap();
        let value = Expression::build(expression)?;

        Ok(AssignmentStatement { identifier, value })
    }
}

impl fmt::Display for AssignmentStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use pest::{iterators::Pair, Parser};

    use crate::parser::{ASTNode, AlloyParser, ParserError, Rule};

    use super::{AssignmentStatement, DeclarationStatement};

    fn statement_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::program, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    fn build_declaration(input: &str) -> Result<DeclarationStatement, ParserError> {
        let pair = statement_pair(input).unwrap();
        DeclarationStatement::build(pair)
    }

    #[test]
    fn test_declaration_statement() -> Result<(), ParserError> {
        build_declaration("var myVar;")?;
        build_declaration("var myVar = 2;")?;
        build_declaration("const myConst = 2;")?;
        Ok(())
    }

    fn build_assignment(input: &str) -> Result<AssignmentStatement, ParserError> {
        let pair = statement_pair(input).unwrap();
        AssignmentStatement::build(pair)
    }

    #[test]
    fn test_assignment_statement() {
        build_assignment("myVar = 120;");
        build_assignment("myVar = true;");
        build_assignment("myVar = 12 * 12 - 12;");
    }

    #[test]
    fn test_wrong_declaration_statements() {
        assert!(statement_pair("const myConst;").is_none());
        assert!(statement_pair("var myVar").is_none());
        assert!(statement_pair("var myVar = 2").is_none());
        assert!(statement_pair("const myVar = 2").is_none());
        assert!(statement_pair("const const = 2;").is_none());
        assert!(statement_pair("const var = 2;").is_none());
        assert!(statement_pair("const if = 2;").is_none());
    }
}
