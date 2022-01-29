use std::str::{CharIndices, Chars};

use nom::{Compare, InputIter, InputLength, InputTake, UnspecializedInput};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input<'a> {
    pub input: &'a str,
    pub position: usize,
}

impl<'a> From<&'a str> for Input<'a> {
    fn from(input: &'a str) -> Self {
        Self { input, position: 0 }
    }
}

impl<'a> PartialEq<&str> for Input<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.input == *other
    }
}

impl<'a> InputTake for Input<'a> {
    #[inline]
    fn take(&self, count: usize) -> Self {
        let input = self.input.take(count);
        let position = self.position + count;
        Self { input, position }
    }

    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (suffix, prefix) = self.input.take_split(count);
        let prefix = Self {
            input: prefix,
            position: self.position,
        };
        let suffix = Self {
            input: suffix,
            position: count,
        };
        (suffix, prefix)
    }
}

impl<'a> InputIter for Input<'a> {
    type Item = char;
    type Iter = CharIndices<'a>;
    type IterElem = Chars<'a>;

    fn iter_indices(&self) -> Self::Iter {
        self.input.iter_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.input.iter_elements()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.input.position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        self.input.slice_index(count)
    }
}

impl<'a> InputLength for Input<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        self.input.input_len()
    }
}

impl<'a> Compare<&str> for Input<'a> {
    fn compare(&self, t: &str) -> nom::CompareResult {
        self.input.compare(t)
    }

    fn compare_no_case(&self, t: &str) -> nom::CompareResult {
        self.input.compare_no_case(t)
    }
}

impl<'a> UnspecializedInput for Input<'a> {}

impl<'a> Into<&'a str> for Input<'a> {
    fn into(self) -> &'a str {
        self.input
    }
}