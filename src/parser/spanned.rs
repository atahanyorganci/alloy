use std::fmt;

/// `Spanned<T>` is a wrapper around `T` that holds start and
/// end positions of the AST node in the source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> {
    pub ast: T,
    pub start: usize,
    pub end: usize,
}

impl<T> PartialEq<T> for Spanned<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.ast == *other
    }
}

impl<T> fmt::Display for Spanned<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ast)
    }
}
