use std::fmt;

use pest::iterators::{Pair, Pairs};

use crate::{
    compiler::{Compile, Compiler, CompilerResult, Instruction},
    parser::{parse_pairs, Parse, ParseResult, ParserError, Rule},
};

use super::{expression::Expression, statement::Statement};

pub struct ReturnStatement {
    expression: Option<Expression>,
}

impl fmt::Debug for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("ReturnStatement");
        if self.expression.is_some() {
            debug.field("expression", &self.expression);
        }
        debug.finish()
    }
}

impl fmt::Display for ReturnStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Parse<'_> for ReturnStatement {
    fn parse(pair: Pair<'_, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::return_statement);
        let mut inner = pair.into_inner();

        matches!(inner.next().unwrap().as_rule(), Rule::k_return);
        if let Some(expr) = inner.next() {
            Ok(Self {
                expression: Some(Expression::parse(expr)?),
            })
        } else {
            Ok(Self { expression: None })
        }
    }
}

impl Compile for ReturnStatement {
    fn compile(&self, _compiler: &mut Compiler) -> CompilerResult<()> {
        todo!()
    }
}

pub struct FunctionStatement {
    name: String,
    args: Vec<String>,
    body: Vec<Statement>,
}

impl fmt::Debug for FunctionStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("FunctionStatement");
        debug.field("name", &self.name);
        if !self.args.is_empty() {
            debug.field("args", &self.args);
        }
        debug.field("body", &self.body).finish()
    }
}

impl fmt::Display for FunctionStatement {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

fn pairs_to_boxed_slice<F, U>(pairs: Pairs<Rule>, f: F) -> ParseResult<Vec<U>>
where
    F: Fn(Pair<Rule>) -> ParseResult<U>,
{
    let (_, max) = pairs.size_hint();
    let mut out = if let Some(capacity) = max {
        Vec::with_capacity(capacity)
    } else {
        Vec::new()
    };
    for pair in pairs {
        out.push(f(pair)?);
    }
    Ok(out)
}

impl<'a> Parse<'a> for FunctionStatement {
    fn parse(pair: Pair<'a, Rule>) -> Result<Self, ParserError> {
        matches!(pair.as_rule(), Rule::function_statement);
        let mut inner = pair.into_inner();

        matches!(inner.next().unwrap().as_rule(), Rule::k_fn);

        let name_pair = inner.next().unwrap();
        let name = name_pair.as_str().to_string();

        let args_pairs = inner.next().unwrap().into_inner();
        let args = pairs_to_boxed_slice(args_pairs, |s| Ok(s.as_str().to_string()))?;

        let body_pairs = inner.next().unwrap().into_inner();
        let body = parse_pairs(body_pairs)?;

        Ok(Self { name, args, body })
    }
}

impl Compile for FunctionStatement {
    fn compile(&self, _compiler: &mut Compiler) -> CompilerResult<()> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{self, ParseResult};

    use super::FunctionStatement;

    fn parse_function(input: &str) -> ParseResult<()> {
        parser::parse_statement::<FunctionStatement>(input)?;
        Ok(())
    }

    #[test]
    fn test_function_statement() -> ParseResult<()> {
        parse_function("fn todo() {}")?;
        parse_function("fn display(x) { print x; }")?;
        parse_function("fn add(x, y) { return x + y; }")?;
        parse_function("fn add_mul(x, y, z) { return x * y + z; }")?;
        Ok(())
    }

    #[test]
    fn test_wrong_function_statements() {
        parse_function("fn print(x) { print x; }").unwrap_err();
        parse_function("fn add(x x) {}").unwrap_err();
        parse_function("fn add(x x x) {}").unwrap_err();
        parse_function("fn add(x, x x) {}").unwrap_err();
    }
}
