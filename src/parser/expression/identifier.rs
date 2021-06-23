use std::fmt;

use pest::iterators::Pair;

use crate::parser::{value::Value, ASTNode, Expression, Rule};

pub struct IdentifierExpression {
    identifier: String,
}

impl Expression for IdentifierExpression {
    fn eval(&self) -> Value {
        todo!()
    }
}

impl ASTNode for IdentifierExpression {
    fn build(pair: Pair<Rule>) -> Option<Box<Self>>
    where
        Self: Sized,
    {
        match pair.as_rule() {
            Rule::identifier => Some(Box::from(IdentifierExpression {
                identifier: String::from(pair.as_str()),
            })),
            _ => None,
        }
    }
}

impl fmt::Debug for IdentifierExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

impl fmt::Display for IdentifierExpression {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use pest::{iterators::Pair, Parser};

    use crate::parser::{AlloyParser, Rule};

    fn identifier_pair(input: &str) -> Option<Pair<Rule>> {
        match AlloyParser::parse(Rule::identifier, input) {
            Ok(mut pairs) => Some(pairs.next().unwrap()),
            Err(_) => None,
        }
    }

    #[test]
    fn test_wrong_identifiers() {
        assert!(identifier_pair("if").is_none());
        assert!(identifier_pair("else").is_none());
        assert!(identifier_pair("print").is_none());
        assert!(identifier_pair("while").is_none());
        assert!(identifier_pair("for").is_none());
        assert!(identifier_pair("return").is_none());
        assert!(identifier_pair("var").is_none());
        assert!(identifier_pair("const").is_none());
        assert!(identifier_pair("and").is_none());
        assert!(identifier_pair("or").is_none());
        assert!(identifier_pair("not").is_none());
        assert!(identifier_pair("xor").is_none());
        assert!(identifier_pair("continue").is_none());
        assert!(identifier_pair("break").is_none());
        assert!(identifier_pair("in").is_none());
    }
}
