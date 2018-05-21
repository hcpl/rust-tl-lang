//! A punctuated sequence of syntax tree nodes separated by punctuation.

use std::option;
use std::vec;

use nom;

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
    pub fn parse(
        input: &str,
        trailing_punct: TrailingPunctuation,
        count: Count,
        whitespace: Whitespace,
    ) -> nom::IResult<&str, Self> {
        Self::parse_with(input, T::parse_str, trailing_punct, count, whitespace)
    }
}

impl<T, P> Punctuated<T, P>
where
    P: Synom,
{
    fn parse_with(
        mut input: &str,
        parse: fn(&str) -> nom::IResult<&str, T>,
        trailing_punct: TrailingPunctuation,
        count: Count,
        whitespace: Whitespace,
    ) -> nom::IResult<&str, Self> {
        let actual_parse = |s| match whitespace {
            Whitespace::None => parse(s),
            Whitespace::Present => sp!(s, parse),
        };

        let mut res = Punctuated::new();

        // get the first element
        let (rest, res) = match actual_parse(input) {
            Err(_) => (input, res),
            Ok((i, o)) => {
                if i == input {
                    return parse_error(input, 0);
                }
                input = i;
                res.push_value(o);

                // get the separator first
                while let Ok((i2, s)) = P::parse_str(input) {
                    if i2 == input {
                        break;
                    }

                    // get the element next
                    if let Ok((i3, o3)) = actual_parse(i2) {
                        if i3 == i2 {
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
                    if let Ok((after, sep)) = P::parse_str(input) {
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

fn parse_error<O, E>(input: &str, error: E) -> nom::IResult<&str, O, E> {
    Err(nom::Err::Error(nom::Context::Code(input, nom::ErrorKind::Custom(error))))
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


#[derive(Debug)]
pub enum TrailingPunctuation {
    None,
    Optional,
}

#[derive(Debug)]
pub enum Count {
    ZeroOrMore,
    OneOrMore,
}

#[derive(Debug)]
pub enum Whitespace {
    None,
    Present,
}
