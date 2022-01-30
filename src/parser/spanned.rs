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
