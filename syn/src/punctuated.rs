//! A punctuated sequence of syntax tree nodes separated by punctuation.

use std::fmt;
use std::option;
use std::vec;

use nom;

use cursor::Cursor;
use print::Print;
use span::Span;
use spanned::Spanned;
use synom::Synom;


#[derive(Debug)]
pub struct Punctuated<T, P> {
    inner: Vec<(T, P)>,
    last: Option<Box<T>>,
}

impl<T, P> Punctuated<T, P> {
    pub fn new() -> Punctuated<T, P> {
        Punctuated {
            inner: Vec::new(),
            last: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty() && self.last.is_none()
    }

    pub fn len(&self) -> usize {
        self.inner.len() + if self.last.is_some() { 1 } else { 0 }
    }

    pub fn into_pairs(self) -> IntoPairs<T, P> {
        IntoPairs {
            inner: self.inner.into_iter(),
            last: self.last.map(|t| *t).into_iter(),
        }
    }

    pub fn push_value(&mut self, value: T) {
        assert!(self.empty_or_trailing());
        self.last = Some(Box::new(value));
    }

    pub fn push_punct(&mut self, punctuation: P) {
        assert!(self.last.is_some());
        let last = self.last.take().unwrap();
        self.inner.push((*last, punctuation));
    }

    pub fn empty_or_trailing(&self) -> bool {
        self.last.is_none()
    }
}

impl<T, P> Punctuated<T, P>
where
    P: Default,
{
    pub fn push(&mut self, value: T) {
        if !self.empty_or_trailing() {
            self.push_punct(Default::default());
        }
        self.push_value(value);
    }
}

impl<T, P> Punctuated<T, P>
where
    T: Synom,
    P: Synom,
{
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
            Whitespace::Present => sp!(s, parse),
        };

        let mut res = Punctuated::new();

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


#[derive(Debug)]
pub enum Pair<T, P> {
    Punctuated(T, P),
    End(T),
}


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


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrailingPunctuation {
    None,
    Optional,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Count {
    ZeroOrMore,
    OneOrMore,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Whitespace {
    None,
    Present,
}
