//! Parsing interface for parsing a token stream into a syntax tree node.

use nom::{self, AtEof};

use cursor::Cursor;


pub(crate) mod private {
    /// `Sealed` stops crates other than `tl-lang-syn` from implementing the
    /// `Print` trait.
    pub trait Sealed {}
}


/// Parsing interface implemented by all types that can be parsed in a default
/// way from a string.
///
/// This trait is sealed and cannot be implemented for types outside of
/// `tl-lang-syn` to avoid breaking backwards compatibility when adding new
/// methods or derived traits.
pub trait Synom: Sized + private::Sealed {
    fn parse_cursor<'a>(input: Cursor<'a>) -> nom::IResult<Cursor<'a>, Self>;

    fn parse_str(input: &str) -> nom::IResult<&str, Self> {
        Self::parse_cursor(Cursor::new(input))
            .map(|(cursor, res)| (cursor.to_str(), res))
            .map_err(nom_err_to_str)
    }
}


/// Parser that can parse TL language schema string into a particular syntax
/// tree node.
pub trait Parser: Sized {
    type Output;

    fn parse_cursor<'a>(self, input: Cursor<'a>) -> Result<Self::Output, nom::Err<Cursor<'a>>>;

    fn parse_str(self, input: &str) -> Result<Self::Output, nom::Err<&str>> {
        self.parse_cursor(Cursor::new(input)).map_err(nom_err_to_str)
    }
}


impl<F, T> Parser for F
where
    F: for<'a> FnOnce(Cursor<'a>) -> nom::IResult<Cursor<'a>, T>,
{
    type Output = T;

    fn parse_cursor<'a>(self, input: Cursor<'a>) -> Result<Self::Output, nom::Err<Cursor<'a>>> {
        let (rest, value) = self(input)?;

        if rest.at_eof() {
            Ok(value)
        } else if rest.to_str() == input.to_str() {
            // parsed nothing
            parse_error(input, 100000)
        } else {
            parse_error(input, 200000)
        }
    }
}


fn nom_err_to_str<'a, E>(error: nom::Err<Cursor<'a>, E>) -> nom::Err<&'a str, E> {
    fn nom_context_to_str<'a, E>(context: nom::Context<Cursor<'a>, E>) -> nom::Context<&'a str, E> {
        match context {
            nom::Context::Code(input, kind) => nom::Context::Code(input.to_str(), kind),
        }
    }

    match error {
        nom::Err::Incomplete(needed) => nom::Err::Incomplete(needed),
        nom::Err::Error(context)     => nom::Err::Error(nom_context_to_str(context)),
        nom::Err::Failure(context)   => nom::Err::Failure(nom_context_to_str(context)),
    }
}

fn parse_error<'a, O, E>(input: Cursor<'a>, error: E) -> Result<O, nom::Err<Cursor<'a>, E>> {
    Err(nom::Err::Error(nom::Context::Code(input, nom::ErrorKind::Custom(error))))
}
