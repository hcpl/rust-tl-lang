use cursor::Cursor;


macro_rules! tlsyn {
    ($i:expr, $t:ty) => {
        <$t as $crate::synom::Synom>::parse_cursor($i)
    };
}


macro_rules! braces {
    ($i:expr, $submac:ident!( $($args:tt)* )) => {
        $crate::token::Brace::parse($i, |i| $submac!(i, $($args)*))
    };

    ($i:expr, $f:expr) => {
        braces!($i, call!($f));
    };
}

macro_rules! brackets {
    ($i:expr, $submac:ident!( $($args:tt)* )) => {
        $crate::token::Bracket::parse($i, |i| $submac!(i, $($args)*))
    };

    ($i:expr, $f:expr) => {
        brackers!($i, call!($f));
    };
}

macro_rules! parens {
    ($i:expr, $submac:ident!( $($args:tt)* )) => {
        $crate::token::Paren::parse($i, |i| $submac!(i, $($args)*))
    };

    ($i:expr, $f:expr) => {
        parens!($i, call!($f));
    };
}

macro_rules! slash_asterisks {
    ($i:expr,) => {
        $crate::token::SlashAsterisk::parse($i, |i| take_until!(i, "*/"))
    };
}


named!(pub space(Cursor) -> Cursor, eat_separator!(" \t"));

macro_rules! sp {
    ($i:expr, $($args:tt)*) => {
        {
            use $crate::parsers::space;
            use $crate::nom::{AtEof, Convert};

            match sep!($i, space, $($args)*) {
                Err(e)      => Err(e),
                Ok((i1, o)) => {
                    match space(i1) {
                        Err($crate::nom::Err::Incomplete(_)) => {
                            assert!(i1.at_eof());
                            Ok((i1, o))
                        },
                        Err(e)      => Err($crate::nom::Err::convert(e)),
                        Ok((i2, _)) => Ok((i2, o)),
                    }
                },
            }
        }
    }
}