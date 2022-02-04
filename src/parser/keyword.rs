use nom::{branch::alt, bytes::complete::tag, error::context};

use super::{Input, ParserResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    If,
    Else,
    Print,
    While,
    For,
    Return,
    Var,
    Const,
    Continue,
    Break,
    In,
    And,
    Or,
    Not,
    Xor,
    Fn,
}

pub static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "if" => Keyword::If,
    "else" => Keyword::Else,
    "print" => Keyword::Print,
    "while" => Keyword::While,
    "for" => Keyword::For,
    "return" => Keyword::Return,
    "var" => Keyword::Var,
    "const" => Keyword::Const,
    "continue" => Keyword::Continue,
    "break" => Keyword::Break,
    "in" => Keyword::In,
    "and" => Keyword::And,
    "or" => Keyword::Or,
    "not" => Keyword::Not,
    "xor" => Keyword::Xor,
    "fn" => Keyword::Fn,
};

pub fn parse_if(input: Input<'_>) -> ParserResult<'_, Keyword> {
    let (input, word) = tag("if")(input)?;
    let keyword = KEYWORDS.get(word.input).unwrap();
    Ok((input, *keyword))
}

pub fn parse_else(input: Input<'_>) -> ParserResult<'_, Keyword> {
    let (input, word) = tag("else")(input)?;
    let keyword = KEYWORDS.get(word.input).unwrap();
    Ok((input, *keyword))
}

pub fn parse_print(input: Input<'_>) -> ParserResult<'_, Keyword> {
    let (input, word) = tag("print")(input)?;
    let keyword = KEYWORDS.get(word.input).unwrap();
    Ok((input, *keyword))
}

pub fn parse_while(input: Input<'_>) -> ParserResult<'_, Keyword> {
    let (input, word) = tag("while")(input)?;
    let keyword = KEYWORDS.get(word.input).unwrap();
    Ok((input, *keyword))
}

pub fn parse_for(input: Input<'_>) -> ParserResult<'_, Keyword> {
    let (input, word) = tag("for")(input)?;
    let keyword = KEYWORDS.get(word.input).unwrap();
    Ok((input, *keyword))
}

pub fn parse_return(input: Input<'_>) -> ParserResult<'_, Keyword> {
    let (input, word) = tag("return")(input)?;
    let keyword = KEYWORDS.get(word.input).unwrap();
    Ok((input, *keyword))
}

pub fn parse_var(input: Input<'_>) -> ParserResult<'_, Keyword> {
    let (input, word) = tag("var")(input)?;
    let keyword = KEYWORDS.get(word.input).unwrap();
    Ok((input, *keyword))
}
pub fn parse_const(input: Input<'_>) -> ParserResult<'_, Keyword> {
    let (input, word) = tag("const")(input)?;
    let keyword = KEYWORDS.get(word.input).unwrap();
    Ok((input, *keyword))
}

pub fn parse_continue(input: Input<'_>) -> ParserResult<'_, Keyword> {
    let (input, word) = tag("continue")(input)?;
    let keyword = KEYWORDS.get(word.input).unwrap();
    Ok((input, *keyword))
}

pub fn parse_break(input: Input<'_>) -> ParserResult<'_, Keyword> {
    let (input, word) = tag("break")(input)?;
    let keyword = KEYWORDS.get(word.input).unwrap();
    Ok((input, *keyword))
}

pub fn parse_in(input: Input<'_>) -> ParserResult<'_, Keyword> {
    let (input, word) = tag("in")(input)?;
    let keyword = KEYWORDS.get(word.input).unwrap();
    Ok((input, *keyword))
}

pub fn parse_and(input: Input<'_>) -> ParserResult<'_, Keyword> {
    let (input, word) = tag("and")(input)?;
    let keyword = KEYWORDS.get(word.input).unwrap();
    Ok((input, *keyword))
}

pub fn parse_or(input: Input<'_>) -> ParserResult<'_, Keyword> {
    let (input, word) = tag("or")(input)?;
    let keyword = KEYWORDS.get(word.input).unwrap();
    Ok((input, *keyword))
}

pub fn parse_not(input: Input<'_>) -> ParserResult<'_, Keyword> {
    let (input, word) = tag("not")(input)?;
    let keyword = KEYWORDS.get(word.input).unwrap();
    Ok((input, *keyword))
}

pub fn parse_xor(input: Input<'_>) -> ParserResult<'_, Keyword> {
    let (input, word) = tag("xor")(input)?;
    let keyword = KEYWORDS.get(word.input).unwrap();
    Ok((input, *keyword))
}

pub fn parse_fn(input: Input<'_>) -> ParserResult<'_, Keyword> {
    let (input, word) = tag("fn")(input)?;
    let keyword = KEYWORDS.get(word.input).unwrap();
    Ok((input, *keyword))
}

pub fn parse_keyword(input: Input<'_>) -> ParserResult<'_, Keyword> {
    context(
        "keyword",
        alt((
            parse_if,
            parse_else,
            parse_print,
            parse_while,
            parse_for,
            parse_return,
            parse_var,
            parse_const,
            parse_continue,
            parse_break,
            parse_in,
            parse_and,
            parse_or,
            parse_not,
            parse_xor,
            parse_fn,
        )),
    )(input)
}
