//! Tokens representing TL language punctuation, keywords, and delimiters.

#[cfg(feature = "printing")]
use std::fmt;

#[cfg(feature = "parsing")]
use cursor::Cursor;
#[cfg(feature = "printing")]
use print::Print;
#[cfg(feature = "printing")]
use print::private::Sealed as PrintSealed;
use span::Span;
use spanned::Spanned;
use spanned::private::Sealed as SpannedSealed;
#[cfg(feature = "parsing")]
use synom::Synom;
#[cfg(feature = "parsing")]
use synom::private::Sealed as SynomSealed;


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
        #[$doc]
        ///
        /// Don't try to remember the name of this type -- use the [`TLToken!`]
        /// macro instead.
        ///
        /// [`Token!`]: index.html
        #[cfg_attr(feature = "clone-impls", derive(Clone))]
        #[cfg_attr(feature = "debug-impls", derive(Debug))]
        pub struct $name(pub Span);

        #[cfg(feature = "eq-impls")]
        impl ::std::cmp::Eq for $name {}

        #[cfg(feature = "eq-impls")]
        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, _other: &$name) -> bool {
                true
            }
        }

        #[cfg(feature = "hash-impls")]
        impl ::std::hash::Hash for $name {
            fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {
                // No state to hash -- do nothing
            }
        }
    }
}

macro_rules! token_punct_parser {
    ($punct:tt pub struct $name:ident) => {
        #[cfg(feature = "parsing")]
        impl SynomSealed for $name {}

        #[cfg(feature = "parsing")]
        impl Synom for $name {
            named!(parse_cursor(Cursor) -> $name, map!(tag!($punct), |cursor| {
                $name(cursor.span())
            }));
        }

        impl SpannedSealed for $name {}

        impl Spanned for $name {
            fn span(&self) -> Span {
                self.0
            }
        }

        #[cfg(feature = "printing")]
        impl PrintSealed for $name {}

        #[cfg(feature = "printing")]
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
        #[cfg_attr(feature = "clone-impls", derive(Clone))]
        #[cfg_attr(feature = "debug-impls", derive(Debug))]
        pub struct $name(pub Span);

        #[cfg(feature = "eq-impls")]
        impl ::std::cmp::Eq for $name {}

        #[cfg(feature = "eq-impls")]
        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, _other: &$name) -> bool {
                true
            }
        }

        #[cfg(feature = "hash-impls")]
        impl ::std::hash::Hash for $name {
            fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {
                // No state to hash -- do nothing
            }
        }

        impl $name {
            #[cfg(feature = "parsing")]
            pub fn parse<'a, F, R>(
                input: Cursor<'a>,
                f: F,
            ) -> $crate::nom::IResult<Cursor<'a>, ($name, R)>
            where
                F: FnOnce(Cursor<'a>) -> $crate::nom::IResult<Cursor<'a>, R>,
            {
                // FIXME: Handle nesting (low-priority)
                let (rest, res_cursor) =
                    delimited!(input, tag!($delimiter_left), call!(f), tag!($delimiter_right))?;

                let begin = input.offset();
                let end = rest.offset();
                assert!(1 <= begin && begin <= end);
                let span = unsafe { Span::new_unchecked(begin, end) };

                Ok((rest, ($name(span), res_cursor)))
            }

            #[cfg(feature = "printing")]
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

        impl SpannedSealed for $name {}

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
        #[cfg_attr(feature = "clone-impls", derive(Clone))]
        #[cfg_attr(feature = "debug-impls", derive(Debug))]
        pub struct $name(pub Span);

        #[cfg(feature = "eq-impls")]
        impl ::std::cmp::Eq for $name {}

        #[cfg(feature = "eq-impls")]
        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, _other: &$name) -> bool {
                true
            }
        }

        #[cfg(feature = "hash-impls")]
        impl ::std::hash::Hash for $name {
            fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {
                // No state to hash -- do nothing
            }
        }

        #[cfg(feature = "parsing")]
        impl SynomSealed for $name {}

        #[cfg(feature = "parsing")]
        impl Synom for $name {
            named!(parse_cursor(Cursor) -> $name, map!(tag!($keyword), |cursor| {
                $name(cursor.span())
            }));
        }

        impl SpannedSealed for $name {}

        impl Spanned for $name {
            fn span(&self) -> Span {
                self.0
            }
        }

        #[cfg(feature = "printing")]
        impl PrintSealed for $name {}

        #[cfg(feature = "printing")]
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
        // Technically not a keyword since it is a usual ident in source text,
        // but it has a special meaning in single-line comments
        "LAYER"    pub struct Layer           /// `LAYER`
    }
}


#[cfg(feature = "parsing")]
macro_rules! tlpunct {
    ($i:expr, *) => { call!($i, <$crate::token::Asterisk as $crate::synom::Synom>::parse_cursor) };
    ($i:expr, ,) => { call!($i, <$crate::token::Comma as $crate::synom::Synom>::parse_cursor) };
    ($i:expr, :) => { call!($i, <$crate::token::Colon as $crate::synom::Synom>::parse_cursor) };
    ($i:expr, .) => { call!($i, <$crate::token::Dot as $crate::synom::Synom>::parse_cursor) };
    ($i:expr, =) => { call!($i, <$crate::token::Equals as $crate::synom::Synom>::parse_cursor) };
    ($i:expr, !) => { call!($i, <$crate::token::Excl as $crate::synom::Synom>::parse_cursor) };
    ($i:expr, #) => { call!($i, <$crate::token::Hash as $crate::synom::Synom>::parse_cursor) };
    ($i:expr, <) => { call!($i, <$crate::token::LAngle as $crate::synom::Synom>::parse_cursor) };
    ($i:expr, %) => { call!($i, <$crate::token::Percent as $crate::synom::Synom>::parse_cursor) };
    ($i:expr, +) => { call!($i, <$crate::token::Plus as $crate::synom::Synom>::parse_cursor) };
    ($i:expr, ?) => { call!($i, <$crate::token::Question as $crate::synom::Synom>::parse_cursor) };
    ($i:expr, >) => { call!($i, <$crate::token::RAngle as $crate::synom::Synom>::parse_cursor) };
    ($i:expr, ;) => { call!($i, <$crate::token::Semicolon as $crate::synom::Synom>::parse_cursor) };
    // No `($i:expr, //) => { call!($i, <$crate::token::SlashSlash as $crate::synom::Synom>::parse_cursor) };`
    // because you can't write `//` in Rust code without being interpreted as the start of a
    // single-line comment
}

#[cfg(feature = "parsing")]
macro_rules! tlkeyword {
    ($i:expr, empty) => { call!($i, <$crate::token::Empty as $crate::synom::Synom>::parse_cursor) };
    ($i:expr, final) => { call!($i, <$crate::token::Final as $crate::synom::Synom>::parse_cursor) };
    ($i:expr, new) => { call!($i, <$crate::token::New as $crate::synom::Synom>::parse_cursor) };
    ($i:expr, LAYER) => { call!($i, <$crate::token::Layer as $crate::synom::Synom>::parse_cursor) };
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

    (Empty) => { $crate::token::Empty };
    (Final) => { $crate::token::Final };
    (New) => { $crate::token::New };
    (LAYER) => { $crate::token::Layer };
}
