use std::fmt;

use nom::{
    branch::alt,
    character::complete::multispace0,
    combinator::{map, opt, peek},
    error::context,
};

use crate::ast::value::Value;

use super::{
    identifier::parse_identifier,
    literal::parse_value,
    map_spanned,
    operator::{parse_operator, parse_unary_operator, Operator},
    Input, Spanned, SpannedResult,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Identifier(String),
    Value(Value),
    Binary {
        op: Spanned<Operator>,
        lhs: Box<Spanned<Expr>>,
        rhs: Box<Spanned<Expr>>,
    },
    Unary {
        op: Spanned<Operator>,
        operand: Box<Spanned<Expr>>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Identifier(ident) => write!(f, "{ident}"),
            Expr::Value(value) => write!(f, "{value}"),
            Expr::Binary { op, lhs, rhs } => write!(f, "({lhs} {op} {rhs})"),
            Expr::Unary { op, operand } => write!(f, "({op} {operand})"),
        }
    }
}

fn parse_identifer_atom(input: Input<'_>) -> SpannedResult<'_, Expr> {
    map(parse_identifier, |a| {
        map_spanned(a, |v| Expr::Identifier(v))
    })(input)
}

fn parse_value_atom(input: Input<'_>) -> SpannedResult<'_, Expr> {
    map(parse_value, |a| map_spanned(a, |v| Expr::Value(v)))(input)
}

fn parse_atom(input: Input<'_>) -> SpannedResult<'_, Expr> {
    context("atom", alt((parse_identifer_atom, parse_value_atom)))(input)
}

fn parse_prefix_expression(input: Input<'_>) -> SpannedResult<'_, Expr> {
    let start = input.position;
    let (input, op) = parse_unary_operator(input)?;
    let ((), r_bp) = op.prefix_binding_power();
    let (input, _whitespace) = multispace0(input)?;
    let (input, operand) = parse_expression_bp(input, r_bp)?;
    let unary = Expr::Unary {
        op,
        operand: Box::from(operand),
    };
    let spanned = Spanned {
        ast: unary,
        start,
        end: input.position,
    };
    Ok((input, spanned))
}

pub fn parse_expression(input: Input<'_>) -> SpannedResult<'_, Expr> {
    parse_expression_bp(input, 0)
}

fn parse_expression_bp(input: Input<'_>, min_bp: u8) -> SpannedResult<'_, Expr> {
    let (mut input, mut expr) = alt((parse_prefix_expression, parse_atom))(input)?;
    loop {
        let (next_input, _whitespace) = multispace0(input)?;
        input = next_input;

        // Use `peek` to avoid consuming if binding power of operator is lower than `min_bp`.
        let op = match peek(opt(parse_operator))(input) {
            Ok((next_input, Some(op))) => {
                input = next_input;
                op
            }
            Ok((next_input, None)) => return Ok((next_input, expr)),
            Err(err) => return Err(err),
        };
        // Get operator's binding power
        let (l_bp, r_bp) = op.infix_binding_power();

        if l_bp < min_bp {
            // Since binding power of operator is lower than `min_bp`, we stop
            return Ok((input, expr));
        }
        // Consume operator token
        input = parse_operator(input)?.0;

        let (next_input, _whitespace) = multispace0(input)?;
        input = next_input;

        // Parse right-hand side of expression
        let (next_input, rhs) = parse_expression_bp(input, r_bp)?;
        input = next_input;
        expr = Spanned {
            start: expr.start,
            end: rhs.end,
            ast: Expr::Binary {
                op,
                lhs: Box::new(expr),
                rhs: Box::new(rhs),
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::expression::{parse_expression, Expr};

    macro_rules! assert_expr {
        ($lhs:expr, $rhs:expr) => {
            let (input, expr) = parse_expression($lhs.into()).unwrap();
            assert_eq!(input, "");
            assert_eq!(format!("{expr}"), $rhs.to_string());
        };
    }

    #[test]
    fn test_identifier_expression() {
        let (input, identifier) = parse_expression("a".into()).unwrap();
        assert_eq!(input, "");
        assert_eq!(identifier, Expr::Identifier("a".into()));
    }

    #[test]
    fn test_value_expression() {
        let (input, identifier) = parse_expression("1234".into()).unwrap();
        assert_eq!(input, "");
        assert_eq!(identifier, Expr::Value(1234.into()));
    }

    #[test]
    fn test_binary_expression() {
        // test binary expression with different operators
        assert_expr!("1 + 2", "(1 + 2)");
        assert_expr!("1 - 2", "(1 - 2)");
        assert_expr!("1 * 2", "(1 * 2)");
        assert_expr!("1 / 2", "(1 / 2)");
        assert_expr!("1 % 2", "(1 % 2)");

        assert_expr!("1 + 2 * 3", "(1 + (2 * 3))");
        assert_expr!("1 * 2 + 3", "((1 * 2) + 3)");
        assert_expr!("1 + 2 * 3 + 4", "((1 + (2 * 3)) + 4)");
        assert_expr!("1 + 2 * 3 + 4 * 5", "((1 + (2 * 3)) + (4 * 5))");
        assert_expr!("1 + 2 * 3 + 4 * 5 + 6", "(((1 + (2 * 3)) + (4 * 5)) + 6)");

        assert_expr!("1 + 5 * 6 < 2 + 3", "((1 + (5 * 6)) < (2 + 3))");
        assert_expr!(
            "1 + 5 * 6 < 2 + 3 and true",
            "(((1 + (5 * 6)) < (2 + 3)) and true)"
        );
        assert_expr!(
            "1 + 5 * 6 < 2 + 3 and true",
            "(((1 + (5 * 6)) < (2 + 3)) and true)"
        );
    }

    #[test]
    fn test_unary_expressions() {
        assert_expr!("not true", "(not true)");
        assert_expr!("+4", "(+ 4)");
        assert_expr!("-4", "(- 4)");
        assert_expr!("--4", "(- (- 4))");
        assert_expr!("true and not false", "(true and (not false))");
        assert_expr!("not false and true", "((not false) and true)");
    }

    #[test]
    fn test_associativity_of_exponent() {
        assert_expr!("1 ** 2 ** 3", "((1 ** 2) ** 3)");
    }
}
