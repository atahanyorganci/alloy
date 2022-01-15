use std::fmt;

use pest::iterators::Pair;

use crate::parser::{expression::Expression, ASTNode, ParserError, Rule};

pub struct UnaryExpression {
    operator: UnaryOperator,
    expression: Box<Expression>,
}

// impl Compile for UnaryExpression {
//     fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
//         self.expression.compile(compiler)?;
//         match self.operator {
//             UnaryOperator::Plus => {}
//             UnaryOperator::Minus => compiler.emit(Instruction::UnaryMinus),
//             UnaryOperator::Not => compiler.emit(Instruction::UnaryNot),
//         }
//         Ok(())
//     }
// }

impl ASTNode<'_> for UnaryExpression {
    fn build(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        let mut inner = match pair.as_rule() {
            Rule::unprecedent_unary_expression | Rule::precedent_unary_expression => {
                pair.into_inner()
            }
            _ => unreachable!(),
        };
        let operator = match inner.next().unwrap().as_rule() {
            Rule::not => UnaryOperator::Not,
            Rule::minus => UnaryOperator::Minus,
            Rule::plus => UnaryOperator::Plus,
            _ => unreachable!(),
        };
        let expression = Expression::build(inner.next().unwrap())?;
        let expression = Box::from(expression);
        Ok(Self {
            operator,
            expression,
        })
    }
}

impl fmt::Debug for UnaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.operator {
            UnaryOperator::Plus | UnaryOperator::Minus => {
                write!(f, "({}{:?})", self.operator, self.expression)
            }
            UnaryOperator::Not => write!(f, "({} {:?})", self.operator, self.expression),
        }
    }
}

impl fmt::Display for UnaryExpression {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Clone, Copy)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
}

impl fmt::Debug for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Plus => write!(f, "+"),
            UnaryOperator::Minus => write!(f, "-"),
            UnaryOperator::Not => write!(f, "not"),
        }
    }
}
