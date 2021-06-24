use std::fmt;

use pest::iterators::Pair;

use crate::{
    compiler::{Compile, Compiler, CompilerError, Instruction},
    parser::{expression::build_expression, value::Value, ASTNode, Expression, Rule},
};

pub struct UnaryExpression {
    operator: UnaryOperator,
    expression: Box<dyn Expression>,
}

impl Expression for UnaryExpression {
    fn eval(&self) -> Value {
        todo!()
    }
}

impl Compile for UnaryExpression {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        self.expression.compile(compiler)?;
        match self.operator {
            UnaryOperator::Plus => {}
            UnaryOperator::Minus => compiler.emit(Instruction::UnaryMinus),
            UnaryOperator::Not => compiler.emit(Instruction::UnaryNot),
        }
        Ok(())
    }
}

impl ASTNode for UnaryExpression {
    fn build(pair: Pair<Rule>) -> Option<Box<Self>>
    where
        Self: Sized,
    {
        let mut inner = match pair.as_rule() {
            Rule::unprecedent_unary_expression | Rule::precedent_unary_expression => {
                pair.into_inner()
            }
            _ => return None,
        };
        let operator = match inner.next().unwrap().as_rule() {
            Rule::not => UnaryOperator::Not,
            Rule::minus => UnaryOperator::Minus,
            Rule::plus => UnaryOperator::Plus,
            _ => unreachable!(),
        };
        let expression = build_expression(inner.next().unwrap()).unwrap();
        Some(Box::from(Self {
            operator,
            expression,
        }))
    }
}

impl fmt::Debug for UnaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{:?})", self.operator, self.expression)
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
            UnaryOperator::Not => write!(f, "not "),
        }
    }
}
