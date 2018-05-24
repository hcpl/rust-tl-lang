//! Tokens representing TL language punctuation, keywords, and delimiters.

use std::fmt;

use print::Print;
use span::Span;
use spanned::Spanned;
use synom::Synom;


macro_rules! tokens {
    (
        punct: {
            $($punct:tt pub struct $punct_name:ident #[$punct_doc:meta])*
        }
        delimiter: {
            $($delimiter_left:tt $delimiter_right:tt pub struct $delimiter_name:ident #[$delimiter_doc:meta])*
        }
        keyword: {
            $($keyword:tt pub struct $keyword_name:ident #[$keyword_doc:meta])*
        }
    ) => (
        $(token_punct_def! { #[$punct_doc] $punct pub struct $punct_name })*
        $(token_punct_parser! { $punct pub struct $punct_name })*
        $(token_delimiter! { #[$delimiter_doc] $delimiter_left $delimiter_right pub struct $delimiter_name })*
        $(token_keyword! { #[$keyword_doc] $keyword pub struct $keyword_name })*
    )
}

macro_rules! token_punct_def {
    (#[$doc:meta] $punct:tt pub struct $name:ident) => {
        #[derive(Clone)]
        #[$doc]
        ///
        /// Don't try to remember the name of this type -- use the [`TLToken!`]
        /// macro instead.
        ///
        /// [`Token!`]: index.html
        pub struct $name(pub Span);

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
            named!(parse_str(&str) -> $name, map!(tag!($punct), |_| {
                $name(Span::empty())
            }));
        }

        impl Spanned for $name {
            fn span(&self) -> Span {
                self.0
            }
        }

        impl Print for $name {
            fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str($punct)
            }
        }
    }
}

macro_rules! token_delimiter {
    (#[$doc:meta] $delimiter_left:tt $delimiter_right:tt pub struct $name:ident) => {
        #[$doc]
        ///
        /// Don't try to remember the name of this type -- use the [`TLToken!`]
        /// macro instead.
        ///
        /// [`Token!`]: index.html
        pub struct $name(pub Span);

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
            pub fn parse<'a, F, R>(
                s: &'a str,
                f: F,
            ) -> $crate::nom::IResult<&'a str, ($name, R)>
            where
                F: FnOnce(&'a str) -> $crate::nom::IResult<&'a str, R>,
            {
                // FIXME: Handle nesting (low-priority)
                let (rest, res) = delimited!(s, tag!($delimiter_left), call!(f), tag!($delimiter_right))?;
                Ok((rest, ($name(Span::empty()), res)))
            }

            pub fn print<F>(
                fmtr: &mut fmt::Formatter,
                f: F,
            ) -> fmt::Result
            where
                F: FnOnce(&mut fmt::Formatter) -> fmt::Result
            {
                fmtr.write_str($delimiter_left)?;
                f(fmtr)?;
                fmtr.write_str($delimiter_right)?;

                Ok(())
            }
        }

        impl Spanned for $name {
            fn span(&self) -> Span {
                self.0
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
        pub struct $name(pub Span);

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
            named!(parse_str(&str) -> $name, map!(tag!($keyword), |_| {
                $name(Span::empty())
            }));
        }

        impl Spanned for $name {
            fn span(&self) -> Span {
                self.0
            }
        }

        impl Print for $name {
            fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str($keyword)
            }
        }
    }
}

tokens! {
    punct: {
        "*"        pub struct Asterisk        /// `!`
        ","        pub struct Comma           /// `,`
        ":"        pub struct Colon           /// `:`
        "."        pub struct Dot             /// `.`
        "="        pub struct Equals          /// `=`
        "!"        pub struct Excl            /// `!`
        "#"        pub struct Hash            /// `#`
        "<"        pub struct LAngle          /// `<`
        "%"        pub struct Percent         /// `%`
        "+"        pub struct Plus            /// `+`
        "?"        pub struct Question        /// `?`
        ">"        pub struct RAngle          /// `>`
        ";"        pub struct Semicolon       /// `;`
        "//"       pub struct SlashSlash      /// `//`
    }
    delimiter: {
        "{"   "}"  pub struct Brace           /// `{...}`
        "["   "]"  pub struct Bracket         /// `[...]`
        "("   ")"  pub struct Paren           /// `(...)`
        "/*" "*/"  pub struct SlashAsterisk   /// `/*...*/`
    }
    keyword: {
        "Empty"    pub struct Empty           /// `Empty`
        "Final"    pub struct Final           /// `Final`
        "New"      pub struct New             /// `New`
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
    // No `($i:expr, //) => { call!($i, <$crate::token::SlashSlash as $crate::synom::Synom>::parse_str) };`
    // because you can't write `//` in Rust code without being interpreted as the start of a
    // single-line comment
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
    // No `(//) => { $crate::token::SlashSlash };` because you can't write `//` in Rust code
    // without being interpreted as the start of a single-line comment
}
