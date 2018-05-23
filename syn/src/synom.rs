//! Parsing interface for parsing a token stream into a syntax tree node.

use nom;


/// Parsing interface implemented by all types that can be parsed in a default way from a string.
pub trait Synom: Sized {
    fn parse_str(input: &str) -> nom::IResult<&str, Self>;
}


/// Parser that can parse TL language schema string into a particular syntax tree node.
pub trait Parser: Sized {
    type Output;

    fn parse_str(self, s: &str) -> Result<Self::Output, nom::Err<&str>>;
}

impl<F, T> Parser for F
where
    F: FnOnce(&str) -> nom::IResult<&str, T>,
{
    type Output = T;

    fn parse_str(self, s: &str) -> Result<T, nom::Err<&str>> {
        let (rest, t) = self(s)?;
        if rest.is_empty() {
            Ok(t)
        } else if rest == s {
            // parsed nothing
            parse_error(s, 100000)
        } else {
            parse_error(s, 200000)
        }
    }
}

fn parse_error<O, E>(input: &str, error: E) -> Result<O, nom::Err<&str, E>> {
    Err(nom::Err::Error(nom::Context::Code(input, nom::ErrorKind::Custom(error))))
}
