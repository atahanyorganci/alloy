use std::fmt;

use pest::iterators::Pair;

use crate::{
    compiler::{Compile, Compiler, CompilerError, Instruction},
    parser::{expression::build_expression, ASTNode, Expression, Rule, Statement},
};

#[derive(Debug, Clone, Copy)]
pub enum VariableKind {
    Constant,
    Variable,
}

#[derive(Debug)]
pub struct DeclarationStatement {
    identifier: String,
    initial_value: Option<Box<dyn Expression>>,
    kind: VariableKind,
}

impl Statement for DeclarationStatement {
    fn eval(&self) {
        todo!()
    }
}

impl Compile for DeclarationStatement {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        match &self.initial_value {
            Some(initial_value) => initial_value.compile(compiler)?,
            None => {}
        }
        let index = match self.kind {
            VariableKind::Constant => {
                assert!(self.initial_value.is_some());
                compiler.register_const(&self.identifier)?
            }
            VariableKind::Variable => compiler.register_var(&self.identifier)?,
        };
        if self.initial_value.is_some() {
            compiler.emit(Instruction::StoreSymbol(index));
        }
        Ok(())
    }
}

impl ASTNode for DeclarationStatement {
    fn build(pair: Pair<Rule>) -> Option<Box<Self>>
    where
        Self: Sized,
    {
        let mut inner = match pair.as_rule() {
            Rule::declaration_statement => pair.into_inner(),
            _ => return None,
        };

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
            Some(token) => build_expression(token),
            None => None,
        };

        Some(Box::from(DeclarationStatement {
            identifier,
            initial_value,
            kind: modifier,
        }))
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
    value: Box<dyn Expression>,
}

impl Compile for AssignmentStatement {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        let index = match compiler.get_identifer(&self.identifier) {
            Some(symbol) => match symbol.kind {
                VariableKind::Constant => return Err(CompilerError::AssignmentToConst),
                VariableKind::Variable => symbol.index,
            },
            None => return Err(CompilerError::UndefinedIdentifer),
        };
        self.value.compile(compiler)?;
        compiler.emit(Instruction::StoreSymbol(index));
        Ok(())
    }
}

impl Statement for AssignmentStatement {
    fn eval(&self) {
        todo!()
    }
}

impl ASTNode for AssignmentStatement {
    fn build(pair: Pair<Rule>) -> Option<Box<Self>>
    where
        Self: Sized,
    {
        let mut inner = match pair.as_rule() {
            Rule::assignment_statement => pair.into_inner(),
            _ => return None,
        };

        let identifier_token = inner.next().unwrap();
        let identifier = match identifier_token.as_rule() {
            Rule::identifier => String::from(identifier_token.as_str()),
            _ => unreachable!(),
        };

        let expression = inner.next().unwrap();
        let value = build_expression(expression).unwrap();

        Some(Box::from(AssignmentStatement { identifier, value }))
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

    use crate::parser::{ASTNode, AlloyParser, Rule};

    use super::{AssignmentStatement, DeclarationStatement};

    fn statement_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::program, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    fn build_declaration_statement(input: &str) -> Box<DeclarationStatement> {
        let pair = statement_pair(input).unwrap();
        DeclarationStatement::build(pair).unwrap()
    }

    #[test]
    fn test_declaration_statement() {
        build_declaration_statement("var myVar;");
        build_declaration_statement("var myVar = 2;");
        build_declaration_statement("const myConst = 2;");
    }

    fn build_assignment_statement(input: &str) -> Box<AssignmentStatement> {
        let pair = statement_pair(input).unwrap();
        AssignmentStatement::build(pair).unwrap()
    }

    #[test]
    fn test_assignment_statement() {
        build_assignment_statement("myVar = 120;");
        build_assignment_statement("myVar = true;");
        build_assignment_statement("myVar = 12 * 12 - 12;");
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
