//! Tokens representing TL language punctuation, keywords, and delimiters.

use synom::Synom;


macro_rules! tokens {
    (
        punct: {
            $($punct:tt pub struct $punct_name:ident/$len:tt #[$punct_doc:meta])*
        }
        delimiter: {
            $($delimiter:tt pub struct $delimiter_name:ident #[$delimiter_doc:meta])*
        }
        keyword: {
            $($keyword:tt pub struct $keyword_name:ident #[$keyword_doc:meta])*
        }
    ) => (
        $(token_punct_def! { #[$punct_doc] $punct pub struct $punct_name/$len })*
        $(token_punct_parser! { $punct pub struct $punct_name })*
        $(token_delimiter! { #[$delimiter_doc] $delimiter pub struct $delimiter_name })*
        $(token_keyword! { #[$keyword_doc] $keyword pub struct $keyword_name })*
    )
}

macro_rules! token_punct_def {
    (#[$doc:meta] $punct:tt pub struct $name:ident/$len:tt) => {
        #[derive(Clone)]
        #[$doc]
        ///
        /// Don't try to remember the name of this type -- use the [`TLToken!`]
        /// macro instead.
        ///
        /// [`Token!`]: index.html
        pub struct $name;

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(stringify!($name))
            }
        }

        impl ::std::cmp::Eq for $name {}

        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, _other: &$name) -> bool {
                true
            }
        }
    }
}

macro_rules! token_punct_parser {
    ($punct:tt pub struct $name:ident) => {
        impl Synom for $name {
            named!(parse_str(&str) -> $name, map!(tag!($punct), |_| $name));
        }
    }
}

macro_rules! token_delimiter {
    (#[$doc:meta] $delimiter:tt pub struct $name:ident) => {
        #[$doc]
        ///
        /// Don't try to remember the name of this type -- use the [`TLToken!`]
        /// macro instead.
        ///
        /// [`Token!`]: index.html
        pub struct $name;

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(stringify!($name))
            }
        }

        impl ::std::cmp::Eq for $name {}

        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, _other: &$name) -> bool {
                true
            }
        }

        impl $name {
            pub fn parse<F, R>(
                s: &str,
                f: F,
            ) -> $crate::nom::IResult<&str, ($name, R)>
            where
                F: FnOnce(&str) -> $crate::nom::IResult<&str, R>,
            {
                let (left, right) = match $delimiter {
                    "{" => ('{', '}'),
                    "[" => ('[', ']'),
                    "(" => ('(', ')'),
                    _ => unreachable!(),
                };

                // FIXME: Handle nesting (low-priority)
                let (rest, res) = delimited!(s, char!(left), call!(f), char!(right))?;
                Ok((rest, ($name, res)))
            }
        }
    }
}

macro_rules! token_keyword {
    (#[$doc:meta] $keyword:tt pub struct $name:ident) => {
        #[$doc]
        ///
        /// Don't try to remember the name of this type -- use the [`Token!`]
        /// macro instead.
        ///
        /// [`Token!`]: index.html
        pub struct $name;

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(stringify!($name))
            }
        }

        impl ::std::cmp::Eq for $name {}

        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, _other: &$name) -> bool {
                true
            }
        }

        impl Synom for $name {
            named!(parse_str(&str) -> $name, map!(tag!($keyword), |_| $name));
        }
    }
}

tokens! {
    punct: {
        "*"      pub struct Asterisk/1    /// `!`
        ","      pub struct Comma/1       /// `,`
        ":"      pub struct Colon/1       /// `:`
        "."      pub struct Dot/1         /// `.`
        "="      pub struct Equals/1      /// `=`
        "!"      pub struct Excl/1        /// `!`
        "#"      pub struct Hash/1        /// `#`
        "<"      pub struct LAngle/1      /// `<`
        "%"      pub struct Percent/1     /// `%`
        "+"      pub struct Plus/1        /// `+`
        "?"      pub struct Question/1    /// `?`
        ">"      pub struct RAngle/1      /// `>`
        ";"      pub struct Semicolon/1   /// `;`
    }
    delimiter: {
        "{"      pub struct Brace         /// `{...}`
        "["      pub struct Bracket       /// `[...]`
        "("      pub struct Paren         /// `(...)`
    }
    keyword: {
        "Empty"  pub struct Empty         /// `Empty`
        "Final"  pub struct Final         /// `Final`
        "New"    pub struct New           /// `New`
    }
}


macro_rules! tlpunct {
    ($i:expr, *) => { call!($i, <$crate::token::Asterisk as $crate::synom::Synom>::parse_str) };
    ($i:expr, ,) => { call!($i, <$crate::token::Comma as $crate::synom::Synom>::parse_str) };
    ($i:expr, :) => { call!($i, <$crate::token::Colon as $crate::synom::Synom>::parse_str) };
    ($i:expr, .) => { call!($i, <$crate::token::Dot as $crate::synom::Synom>::parse_str) };
    ($i:expr, =) => { call!($i, <$crate::token::Equals as $crate::synom::Synom>::parse_str) };
    ($i:expr, !) => { call!($i, <$crate::token::Excl as $crate::synom::Synom>::parse_str) };
    ($i:expr, #) => { call!($i, <$crate::token::Hash as $crate::synom::Synom>::parse_str) };
    ($i:expr, <) => { call!($i, <$crate::token::LAngle as $crate::synom::Synom>::parse_str) };
    ($i:expr, %) => { call!($i, <$crate::token::Percent as $crate::synom::Synom>::parse_str) };
    ($i:expr, +) => { call!($i, <$crate::token::Plus as $crate::synom::Synom>::parse_str) };
    ($i:expr, ?) => { call!($i, <$crate::token::Question as $crate::synom::Synom>::parse_str) };
    ($i:expr, >) => { call!($i, <$crate::token::RAngle as $crate::synom::Synom>::parse_str) };
    ($i:expr, ;) => { call!($i, <$crate::token::Semicolon as $crate::synom::Synom>::parse_str) };
}


macro_rules! TLToken {
    (*) => { $crate::token::Asterisk };
    (,) => { $crate::token::Comma };
    (:) => { $crate::token::Colon };
    (.) => { $crate::token::Dot };
    (=) => { $crate::token::Equals };
    (!) => { $crate::token::Excl };
    (#) => { $crate::token::Hash };
    (<) => { $crate::token::LAngle };
    (%) => { $crate::token::Percent };
    (+) => { $crate::token::Plus };
    (?) => { $crate::token::Question };
    (>) => { $crate::token::RAngle };
    (;) => { $crate::token::Semicolon };
}
