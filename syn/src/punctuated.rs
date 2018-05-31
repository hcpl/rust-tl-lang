//! A punctuated sequence of syntax tree nodes separated by punctuation.

use std::option;
use std::slice;
use std::vec;


/// A punctuated sequence of syntax tree nodes of type `T` separated by
/// punctuation of type `P`.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub struct Punctuated<T, P> {
    inner: Vec<(T, P)>,
    last: Option<Box<T>>,
}

impl<T, P> Punctuated<T, P> {
    /// Create an empty punctuated sequence.
    pub fn new() -> Punctuated<T, P> {
        Punctuated {
            inner: Vec::new(),
            last: None,
        }
    }

    /// Determine whether this punctuated sequence is empty, meaning it contains
    /// no syntax tree nodes or punctuation.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty() && self.last.is_none()
    }

    /// Return the number of syntax tree nodes in this punctuated sequence.
    ///
    /// This is the number of nodes of type `T`, not counting the punctuation of
    /// type `P`.
    pub fn len(&self) -> usize {
        self.inner.len() + if self.last.is_some() { 1 } else { 0 }
    }

    /// Borrow the first punctuated pair in this sequence.
    pub fn first(&self) -> Option<Pair<&T, &P>> {
        self.pairs().next()
    }

    /// Mutably borrow the first punctuated pair in this sequence.
    pub fn first_mut(&mut self) -> Option<Pair<&mut T, &mut P>> {
        self.pairs_mut().next()
    }

    /// Borrow the last punctuated pair in this sequence.
    pub fn last(&self) -> Option<Pair<&T, &P>> {
        if self.last.is_some() {
            self.last.as_ref().map(|t| Pair::End(t.as_ref()))
        } else {
            self.inner
                .last()
                .map(|&(ref t, ref p)| Pair::Punctuated(t, p))
        }
    }

    /// Mutably borrow the last punctuated pair in this sequence.
    pub fn last_mut(&mut self) -> Option<Pair<&mut T, &mut P>> {
        if self.last.is_some() {
            self.last.as_mut().map(|t| Pair::End(t.as_mut()))
        } else {
            self.inner
                .last_mut()
                .map(|&mut (ref mut t, ref mut p)| Pair::Punctuated(t, p))
        }
    }

    /// Return an iterator over borrowed syntax tree nodes of type `&T`.
    pub fn iter(&self) -> Iter<T, P> {
        Iter {
            inner: self.inner.iter(),
            last: self.last.as_ref().map(|t| t.as_ref()).into_iter(),
        }
    }

    /// Return an iterator over mutably borrowed syntax tree nodes of type
    /// `&mut T`.
    pub fn iter_mut(&mut self) -> IterMut<T, P> {
        IterMut {
            inner: self.inner.iter_mut(),
            last: self.last.as_mut().map(|t| t.as_mut()).into_iter(),
        }
    }

    /// Return an iterator over the contents of this sequence as borrowed
    /// punctuated pairs.
    pub fn pairs(&self) -> Pairs<T, P> {
        Pairs {
            inner: self.inner.iter(),
            last: self.last.as_ref().map(|t| t.as_ref()).into_iter(),
        }
    }

    /// Return an iterator over the contents of this sequence as mutably
    /// borrowed punctuated pairs.
    pub fn pairs_mut(&mut self) -> PairsMut<T, P> {
        PairsMut {
            inner: self.inner.iter_mut(),
            last: self.last.as_mut().map(|t| t.as_mut()).into_iter(),
        }
    }

    /// Return an iterator over the contents of this sequence as owned
    /// punctuated pairs.
    pub fn into_pairs(self) -> IntoPairs<T, P> {
        IntoPairs {
            inner: self.inner.into_iter(),
            last: self.last.map(|t| *t).into_iter(),
        }
    }

    /// Append a syntax tree node onto the end of this punctuated sequence. The
    /// sequence must previously have a trailing punctuation.
    ///
    /// Use [`push`] instead if the punctuated sequence may or may not already
    /// have trailing punctuation.
    ///
    /// # Panics
    ///
    /// This method panics if the sequence does not already have a trailing
    /// punctuation when this method is called.
    ///
    /// [`push`]: Punctuated::push()
    pub fn push_value(&mut self, value: T) {
        assert!(self.empty_or_trailing());
        self.last = Some(Box::new(value));
    }

    /// Append a trailing punctuation onto the end of this punctuated sequence.
    /// The sequence must be non-empty and must not already have trailing
    /// punctuation.
    ///
    /// # Panics
    ///
    /// This method panics if the sequence is empty or already has a trailing
    /// punctuation.
    pub fn push_punct(&mut self, punctuation: P) {
        assert!(self.last.is_some());
        let last = self.last.take().unwrap();
        self.inner.push((*last, punctuation));
    }

    /// Return true if either this `Punctuated` is empty, or it has a trailing
    /// punctuation.
    pub fn empty_or_trailing(&self) -> bool {
        self.last.is_none()
    }
}

impl<T, P> Punctuated<T, P>
where
    P: Default,
{
    /// Append a syntax tree node onto the end of this punctuated sequence.
    ///
    /// If there is not a trailing punctuation in this sequence when this method
    /// is called, the default value of punctuation type `P` is inserted before
    /// the given value of type `T`.
    pub fn push(&mut self, value: T) {
        if !self.empty_or_trailing() {
            self.push_punct(Default::default());
        }
        self.push_value(value);
    }
}

impl<T, P> IntoIterator for Punctuated<T, P> {
    type Item = T;
    type IntoIter = IntoIter<T, P>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.inner.into_iter(),
            last: self.last.map(|t| *t).into_iter(),
        }
    }
}

impl<'a, T, P> IntoIterator for &'a Punctuated<T, P> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T, P>;

    fn into_iter(self) -> Self::IntoIter {
        Punctuated::iter(self)
    }
}

impl<'a, T, P> IntoIterator for &'a mut Punctuated<T, P> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T, P>;

    fn into_iter(self) -> Self::IntoIter {
        Punctuated::iter_mut(self)
    }
}

mod spanned {
    use super::*;
    use span::Span;
    use spanned::Spanned;
    use spanned::private::Sealed;

    impl<T, P> Sealed for Punctuated<T, P>
    where
        T: Sealed,
        P: Sealed,
    {}

    impl<T, P> Spanned for Punctuated<T, P>
    where
        T: Spanned,
        P: Spanned,
    {
        fn span(&self) -> Span {
            let spans_iter = self.inner
                .iter()
                .map(|&(ref t, ref p)| t.span().to(p.span()));

            Span::union(spans_iter).to(self.last.span())
        }
    }
}

#[cfg(feature = "parsing")]
mod parsing {
    use nom;

    use super::*;
    use cursor::Cursor;
    use synom::Synom;

    impl<T, P> Punctuated<T, P>
    where
        T: Synom,
        P: Synom,
    {
        /// Parse syntax tree nodes with punctuation in between using 3
        /// configuration options:
        ///
        /// - `TrailingPunctuation` — whether `parse` must try to consume a
        ///   trailing punctuation if any.
        /// - `Count` — how many syntax tree nodes `parse` is allowed to
        ///   consume.
        /// - `Whitespace` — whether `parse` should ignore whitespace.
        pub fn parse<'a>(
            input: Cursor<'a>,
            trailing_punct: TrailingPunctuation,
            count: Count,
            whitespace: Whitespace,
        ) -> nom::IResult<Cursor<'a>, Self> {
            Self::parse_with(input, T::parse_cursor, trailing_punct, count, whitespace)
        }
    }

    impl<T, P> Punctuated<T, P>
    where
        P: Synom,
    {
        fn parse_with<'a>(
            mut input: Cursor<'a>,
            parse: fn(Cursor) -> nom::IResult<Cursor, T>,
            trailing_punct: TrailingPunctuation,
            count: Count,
            whitespace: Whitespace,
        ) -> nom::IResult<Cursor<'a>, Self> {
            let actual_parse = |s| match whitespace {
                Whitespace::None => parse(s),
                Whitespace::Present => with_afterspace!(s, parse),
            };

            let mut res = Punctuated::new();
            input = nom::space0(input)?.0;

            // get the first element
            let (rest, res) = match actual_parse(input) {
                Err(_) => (input, res),
                Ok((i, o)) => {
                    if i.to_str() == input.to_str() {
                        return parse_error(input, 0);
                    }
                    input = i;
                    res.push_value(o);

                    // get the separator first
                    while let Ok((i2, s)) = P::parse_cursor(input) {
                        if i2.to_str() == input.to_str() {
                            break;
                        }

                        // get the element next
                        if let Ok((i3, o3)) = actual_parse(i2) {
                            if i3.to_str() == i2.to_str() {
                                break;
                            }
                            res.push_punct(s);
                            res.push_value(o3);
                            input = i3;
                        } else {
                            break;
                        }
                    }
                    if let TrailingPunctuation::Optional = trailing_punct {
                        if let Ok((after, sep)) = P::parse_cursor(input) {
                            res.push_punct(sep);
                            input = after;
                        }
                    }
                    (input, res)
                },
            };

            if let Count::OneOrMore = count {
                if res.is_empty() {
                    return parse_error(input, 0);
                }
            }

            Ok((rest, res))
        }
    }

    fn parse_error<'a, O, E>(input: Cursor<'a>, error: E) -> nom::IResult<Cursor<'a>, O, E> {
        Err(nom::Err::Error(nom::Context::Code(input, nom::ErrorKind::Custom(error))))
    }
}

#[cfg(feature = "printing")]
mod printing {
    use std::fmt;

    use super::*;
    use print::Print;

    impl<T, P> Punctuated<T, P>
    where
        T: Print,
        P: Print,
    {
        pub fn print(
            &self,
            f: &mut fmt::Formatter,
            count: Count,
            whitespace: Whitespace,
        ) -> fmt::Result {
            self.print_with(f, Print::print, count, whitespace)
        }
    }

    impl<T, P> Punctuated<T, P>
    where
        P: Print,
    {
        fn print_with(
            &self,
            f: &mut fmt::Formatter,
            print: fn(&T, &mut fmt::Formatter) -> fmt::Result,
            count: Count,
            whitespace: Whitespace,
        ) -> fmt::Result {
            if count == Count::OneOrMore && self.is_empty() {
                panic!("The must be at least one element in this `Punctuated`!");
            }

            let maybe_space = match whitespace {
                Whitespace::None => "",
                Whitespace::Present => " ",
            };

            for &(ref t, ref p) in &self.inner {
                print(t, f)?;
                f.write_str(maybe_space)?;
                p.print(f)?;
                f.write_str(maybe_space)?;
            }

            if let Some(ref last) = self.last {
                print(last, f)?;
            }

            Ok(())
        }
    }
}


/// An iterator over borrowed values of type `&T`.
#[derive(Debug)]
pub struct Iter<'a, T: 'a, P: 'a> {
    inner: slice::Iter<'a, (T, P)>,
    last: option::IntoIter<&'a T>,
}

impl<'a, T, P> Iterator for Iter<'a, T, P> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|pair| &pair.0)
            .or_else(|| self.last.next())
    }
}

impl<'a, T, P> ExactSizeIterator for Iter<'a, T, P> {
    fn len(&self) -> usize {
        self.inner.len() + self.last.len()
    }
}


/// An iterator over mutably borrowed values of type `&mut T`.
#[derive(Debug)]
pub struct IterMut<'a, T: 'a, P: 'a> {
    inner: slice::IterMut<'a, (T, P)>,
    last: option::IntoIter<&'a mut T>,
}

impl<'a, T, P> Iterator for IterMut<'a, T, P> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|pair| &mut pair.0)
            .or_else(|| self.last.next())
    }
}

impl<'a, T, P> ExactSizeIterator for IterMut<'a, T, P> {
    fn len(&self) -> usize {
        self.inner.len() + self.last.len()
    }
}


/// An iterator over owned values of type `T`.
#[derive(Debug)]
pub struct IntoIter<T, P> {
    inner: vec::IntoIter<(T, P)>,
    last: option::IntoIter<T>,
}

impl<T, P> Iterator for IntoIter<T, P> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|pair| pair.0)
            .or_else(|| self.last.next())
    }
}

impl<T, P> ExactSizeIterator for IntoIter<T, P> {
    fn len(&self) -> usize {
        self.inner.len() + self.last.len()
    }
}


/// A single syntax tree node of type `T` followed by its trailing punctuation
/// of type `P` if any,
#[derive(Debug)]
pub enum Pair<T, P> {
    Punctuated(T, P),
    End(T),
}

impl<T, P> Pair<T, P> {
    /// Extracts the syntax tree node from this punctuated pair, discarding the
    /// following punctuation.
    pub fn into_value(self) -> T {
        match self {
            Pair::Punctuated(t, _) | Pair::End(t) => t,
        }
    }

    /// Borrows the syntax tree node from this punctuated pair.
    pub fn value(&self) -> &T {
        match *self {
            Pair::Punctuated(ref t, _) | Pair::End(ref t) => t,
        }
    }

    /// Mutably borrows the syntax tree node from this punctuated pair.
    pub fn value_mut(&mut self) -> &mut T {
        match *self {
            Pair::Punctuated(ref mut t, _) | Pair::End(ref mut t) => t,
        }
    }

    /// Borrows the punctuation from this punctuated pair, unless this pair is
    /// the final one and there is no trailing punctuation.
    pub fn punct(&self) -> Option<&P> {
        match *self {
            Pair::Punctuated(_, ref d) => Some(d),
            Pair::End(_) => None,
        }
    }
}


/// An iterator over borrowed pairs of type `Pair<&T, &P>`.
#[derive(Debug)]
pub struct Pairs<'a, T: 'a, P: 'a> {
    inner: slice::Iter<'a, (T, P)>,
    last: option::IntoIter<&'a T>,
}

impl<'a, T, P> Iterator for Pairs<'a, T, P> {
    type Item = Pair<&'a T, &'a P>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|&(ref t, ref p)| Pair::Punctuated(t, p))
            .or_else(|| self.last.next().map(Pair::End))
    }
}

impl<'a, T, P> ExactSizeIterator for Pairs<'a, T, P> {
    fn len(&self) -> usize {
        self.inner.len() + self.last.len()
    }
}


/// An iterator over mutably borrowed pairs of type `Pair<&mut T, &mut P>`.
#[derive(Debug)]
pub struct PairsMut<'a, T: 'a, P: 'a> {
    inner: slice::IterMut<'a, (T, P)>,
    last: option::IntoIter<&'a mut T>,
}

impl<'a, T, P> Iterator for PairsMut<'a, T, P> {
    type Item = Pair<&'a mut T, &'a mut P>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|&mut (ref mut t, ref mut p)| Pair::Punctuated(t, p))
            .or_else(|| self.last.next().map(Pair::End))
    }
}

impl<'a, T, P> ExactSizeIterator for PairsMut<'a, T, P> {
    fn len(&self) -> usize {
        self.inner.len() + self.last.len()
    }
}


/// An iterator over owned pairs of type `Pair<T, P>`.
#[derive(Debug)]
pub struct IntoPairs<T, P> {
    inner: vec::IntoIter<(T, P)>,
    last: option::IntoIter<T>,
}

impl<T, P> Iterator for IntoPairs<T, P> {
    type Item = Pair<T, P>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(t, p)| Pair::Punctuated(t, p))
            .or_else(|| self.last.next().map(Pair::End))
    }
}

impl<T, P> ExactSizeIterator for IntoPairs<T, P> {
    fn len(&self) -> usize {
        self.inner.len() + self.last.len()
    }
}


/// A configuration option that controls whether [`Punctuated::parse`] must try
/// to consume a trailing punctuation if any.
#[cfg(any(feature = "parsing", feature = "printing"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrailingPunctuation {
    /// Do not attempt to consume a trailing punctuation.
    None,
    /// Try to consume a trailing punctuation if any.
    Optional,
}

/// A configuration option that controls how many syntax tree nodes
/// [`Punctuated::parse`] is allowed to consume.
#[cfg(any(feature = "parsing", feature = "printing"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Count {
    /// Any number of syntax tree nodes is accepted.
    ZeroOrMore,
    /// At least one syntax tree node must exist, but there is no upper bound.
    OneOrMore,
}

/// A configuration option that controls whether whitespace should be treated
/// specially.
///
/// E.g. whether [`Punctuated::parse`] should ignore whitespace or whether
/// [`Punctuated::print`] should print with whitespace between items.
#[cfg(any(feature = "parsing", feature = "printing"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Whitespace {
    /// Treat whitespace as any other characters in the input.
    None,
    /// Treat whitespace specially.
    Present,
}
