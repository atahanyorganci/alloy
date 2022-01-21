use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::{
        expression::Expression,
        identifier::{Identifier, IdentifierKind},
    },
    compiler::{Compile, Compiler, CompilerError, CompilerResult, Instruction},
    parser::{Parse, ParserError, Rule},
};

#[derive(Debug)]
pub struct DeclarationStatement {
    identifier: Identifier,
    initial_value: Option<Expression>,
}

impl Compile for DeclarationStatement {
    fn compile(&self, compiler: &mut Compiler) -> CompilerResult<()> {
        if let Some(expr) = &self.initial_value {
            expr.compile(compiler)?;
        }
        let idx = compiler.register(self.identifier.clone())?;
        if self.initial_value.is_some() {
            compiler.emit(Instruction::StoreSymbol(idx));
        }
        Ok(())
    }
}

impl Parse<'_> for DeclarationStatement {
    fn parse(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
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
            Some(token) => Some(Expression::parse(token)?),
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
    fn compile(&self, compiler: &mut Compiler) -> CompilerResult<()> {
        match compiler.get_identifier(&self.identifier) {
            Some((IdentifierKind::Variable, idx)) => {
                self.value.compile(compiler)?;
                compiler.emit(Instruction::StoreSymbol(idx));
                Ok(())
            }
            Some((IdentifierKind::Constant, _)) => Err(CompilerError::AssignmentToConst),
            None => Err(CompilerError::UndefinedIdentifer(
                self.identifier.to_owned(),
            )),
        }
    }
}

impl Parse<'_> for AssignmentStatement {
    fn parse(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::assignment_statement);
        let mut inner = pair.into_inner();

        let identifier_token = inner.next().unwrap();
        matches!(identifier_token.as_rule(), Rule::identifier);
        let identifier = String::from(identifier_token.as_str());

        let expression = inner.next().unwrap();
        let value = Expression::parse(expression)?;

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
    use crate::parser::{self, ParseResult};

    use super::{AssignmentStatement, DeclarationStatement};

    fn parse_declaration(input: &str) -> ParseResult<()> {
        parser::parse_statement::<DeclarationStatement>(input)?;
        Ok(())
    }

    fn parse_assignment(input: &str) -> ParseResult<()> {
        parser::parse_statement::<AssignmentStatement>(input)?;
        Ok(())
    }

    #[test]
    fn test_declaration_statement() -> ParseResult<()> {
        parse_declaration("var myVar;")?;
        parse_declaration("var myVar = 2;")?;
        parse_declaration("const myConst = 2;")?;
        Ok(())
    }

    #[test]
    fn test_assignment_statement() -> ParseResult<()> {
        parse_assignment("myVar = 120;")?;
        parse_assignment("myVar = true;")?;
        parse_assignment("myVar = 12 * 12 - 12;")?;
        Ok(())
    }

    #[test]
    fn test_wrong_declaration_statements() {
        parse_declaration("const myConst;").unwrap_err();
        parse_declaration("var myVar").unwrap_err();
        parse_declaration("var myVar = 2").unwrap_err();
        parse_declaration("const myVar = 2").unwrap_err();
        parse_declaration("const const = 2;").unwrap_err();
        parse_declaration("const var = 2;").unwrap_err();
        parse_declaration("const if = 2;").unwrap_err();
    }
}
