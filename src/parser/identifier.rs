use nom::{
    bytes::complete::{take_while, take_while1},
    combinator::not,
    error::context,
    sequence::pair,
    AsChar,
};

use super::{keyword::parse_keyword, Input, SpannedResult};

/// Parse an Alloy identifier which starts with either '\_' or any
/// alphabetic character and is followed by any alphanumeric character
/// or '\_'.
///
/// # Examples
///
/// ```
/// use alloy::parser::identifier::parse_identifier;
///
/// let (input, identifier) = parse_identifier("CamelCase".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(identifier, "CamelCase".to_string());
///
/// let (input, identifier) = parse_identifier("snake_case".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(identifier, "snake_case".to_string());
///
/// let (input, identifier) = parse_identifier("_ignored".into()).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(identifier, "_ignored".to_string());
///
/// assert!(parse_identifier("1i".into()).is_err());
/// assert!(parse_identifier("if".into()).is_err());
/// assert!(parse_identifier("var".into()).is_err());
/// assert!(parse_identifier("const".into()).is_err());
/// ```
///
/// # Errors
///
/// This function will return an error if input doesn't contain a valid identifier.
pub fn parse_identifier(input: Input<'_>) -> SpannedResult<'_, String> {
    let start = input.position;
    let (input, _) = context("identifier", not(parse_keyword))(input)?;
    let (input, (prefix, suffix)) = pair(
        take_while1(|c: char| c.is_alpha() || c == '_'),
        take_while(|c: char| c.is_alphanum() || c == '_'),
    )(input)?;
    let identifier = format!("{prefix}{suffix}");
    let spanned = super::Spanned {
        ast: identifier,
        start,
        end: input.position,
    };
    Ok((input, spanned))
}
