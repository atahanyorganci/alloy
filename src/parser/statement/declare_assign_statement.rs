use pest::iterators::Pair;

use crate::parser::{expression::build_binary_expression, Expression, Rule, Statement};

pub enum Type {
    Const,
    Variable,
}

pub struct DeclarationStatement {
    identifier: String,
    initial_value: Option<Box<dyn Expression>>,
    modifier: Type,
}

impl Statement for DeclarationStatement {
    fn eval(&self) {
        todo!()
    }

    fn build(pair: Pair<Rule>) -> Box<Self> {
        let mut inner = pair.into_inner();

        let modifier_keyword = inner.next().unwrap();
        let modifier = match modifier_keyword.as_rule() {
            Rule::k_const => Type::Const,
            Rule::k_var => Type::Variable,
            _ => unreachable!(),
        };

        let identifier_token = inner.next().unwrap();
        let identifier = match identifier_token.as_rule() {
            Rule::identifier => String::from(identifier_token.as_str()),
            _ => unreachable!(),
        };

        let initial_value = match inner.next() {
            Some(token) => Some(build_binary_expression(token.into_inner())),
            None => None,
        };

        Box::from(DeclarationStatement {
            identifier,
            initial_value,
            modifier,
        })
    }
}

pub struct AssignmentStatement {
    identifier: String,
    value: Box<dyn Expression>,
}

impl Statement for AssignmentStatement {
    fn eval(&self) {
        todo!()
    }

    fn build(pair: Pair<Rule>) -> Box<Self> {
        let mut inner = pair.into_inner();

        let identifier_token = inner.next().unwrap();
        let identifier = match identifier_token.as_rule() {
            Rule::identifier => String::from(identifier_token.as_str()),
            _ => unreachable!(),
        };

        let expression = inner.next().unwrap();
        let value = build_binary_expression(expression.into_inner());

        Box::from(AssignmentStatement { identifier, value })
    }
}

#[cfg(test)]
mod test {
    use pest::{iterators::Pair, Parser};

    use crate::parser::{AlloyParser, Rule, Statement};

    use super::DeclarationStatement;

    fn statement_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::program, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    fn build_declaration_statement(input: &str) -> Box<DeclarationStatement> {
        let pair = statement_pair(input).unwrap();
        DeclarationStatement::build(pair)
    }

    #[test]
    fn test_declaration_statement() {
        build_declaration_statement("var myVar;");
        build_declaration_statement("var myVar = 2;");
        build_declaration_statement("const myConst = 2;");
    }

    #[test]
    fn test_wrong_declaration_statements() {
        assert!(statement_pair("const myConst;").is_none());
        assert!(statement_pair("var myVar").is_none());
        assert!(statement_pair("var myVar = 2").is_none());
        assert!(statement_pair("const myVar = 2").is_none());
        assert!(statement_pair("const const = 2").is_none());
        assert!(statement_pair("const var = 2").is_none());
        assert!(statement_pair("const if = 2").is_none());
    }
}
