use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::{
        expression::Expression,
        identifier::{Identifier, IdentifierKind},
    },
    compiler::{Compile, Compiler, CompilerError, Instruction},
    parser::{ASTNode, ParserError, Rule},
};

#[derive(Debug)]
pub struct DeclarationStatement {
    identifier: Identifier,
    initial_value: Option<Expression>,
}

impl Compile for DeclarationStatement {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        let idx = compiler.register(self.identifier.clone())?;
        if let Some(expr) = &self.initial_value {
            expr.compile(compiler)?;
            compiler.emit(Instruction::StoreSymbol(idx));
        }
        Ok(())
    }
}

impl ASTNode<'_> for DeclarationStatement {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::declaration_statement);
        let mut inner = pair.into_inner();

        let kind_keyword = inner.next().unwrap();
        let kind = match kind_keyword.as_rule() {
            Rule::k_const => IdentifierKind::Constant,
            Rule::k_var => IdentifierKind::Variable,
            _ => unreachable!(),
        };

        let ident_token = inner.next().unwrap();
        let identifier = match ident_token.as_rule() {
            Rule::identifier => {
                let ident = String::from(ident_token.as_str());
                Identifier { ident, kind }
            }
            _ => unreachable!(),
        };

        let initial_value = match inner.next() {
            Some(token) => Some(Expression::build(token)?),
            None => None,
        };

        Ok(DeclarationStatement {
            identifier,
            initial_value,
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
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        match compiler.get_identifier(&self.identifier) {
            Some((IdentifierKind::Variable, idx)) => {
                self.value.compile(compiler)?;
                compiler.emit(Instruction::StoreSymbol(idx));
                Ok(())
            }
            Some((IdentifierKind::Constant, _)) => Err(CompilerError::AssignmentToConst),
            None => Err(CompilerError::UndefinedIdentifer),
        }
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
    fn test_assignment_statement() -> Result<(), ParserError> {
        build_assignment("myVar = 120;")?;
        build_assignment("myVar = true;")?;
        build_assignment("myVar = 12 * 12 - 12;")?;
        Ok(())
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
