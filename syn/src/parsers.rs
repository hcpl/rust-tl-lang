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
        $crate::token::SlashAsterisk::parse($i, |i| take_until!(i, $crate::token::SlashAsterisk::RIGHT))
    };
}


macro_rules! with_afterspace {
    ($i:expr, $submac:ident!( $($args:tt)* )) => {
        {
            let i = {$i};

            match $submac!(i, $($args)*) {
                Err(e) => Err(e),
                Ok((rest1, o)) => match nom::space0(rest1) {
                    Err(e) => Err(e),
                    Ok((rest2, _space)) => Ok((rest2, o)),
                },
            }
        }
    };
    ($i:expr, $f:expr) => {
        with_afterspace!($i, call!($f))
    };
}
