#[cfg(not(feature = "parsing"))]
#[derive(Debug)]
pub struct ParseError {
    _private_dummy_to_prevent_from_constructing: (),
}


#[cfg(feature = "parsing")]
mod parsing {
    use std::error;
    use std::fmt;

    use nom;

    use cursor::Cursor;
    use span::Span;

    #[derive(Debug)]
    pub struct ParseError {
        pub(crate) span: Span,
        pub(crate) error_kind: nom::ErrorKind<u32>,
    }

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    impl error::Error for ParseError {
        fn description(&self) -> &str {
            self.error_kind.description()
        }
    }

    impl ParseError {
        pub fn slice_into<'a>(&self, input: &'a str) -> &'a str {
            let begin = self.span.begin();
            let end = self.span.end();

            &input[begin..end]
        }

        pub fn from_nom_err_cursor<'a>(error: nom::Err<Cursor<'a>, u32>) -> Option<ParseError> {
            fn from_error_kind<'a>(context: nom::Context<Cursor<'a>, u32>) -> ParseError {
                match context {
                    nom::Context::Code(cursor, error_kind) => {
                        let span = cursor.span();

                        ParseError { span, error_kind }
                    },
                    _ => unreachable!(),
                }
            }

            match error {
                nom::Err::Incomplete(_needed) => None,
                nom::Err::Error(context) => Some(from_error_kind(context)),
                nom::Err::Failure(context) => Some(from_error_kind(context)),
            }
        }
    }

    pub type ParseResult<T> = Result<T, ParseError>;
}

#[cfg(feature = "parsing")]
pub use self::parsing::*;
