use super::Ident;
use punctuated::Punctuated;
use token::Paren;


macro_attr_many! {
    /// A dot-separated list of identifiers.
    #[cfg_derive!(Clone, Debug, Eq, PartialEq, Hash)]
    pub struct Path {
        pub segments: Punctuated<Ident, TLToken![.]>,
    }

    /// A dot-separated list of identifiers with optional generic arguments.
    #[cfg_derive!(Clone, Debug, Eq, PartialEq, Hash)]
    pub struct ParameterizedPath {
        pub path: Path,
        pub args: Option<GenericArguments>,
    }

    /// Generic arguments for parameterized paths.
    #[cfg_derive!(Clone, Debug, Eq, PartialEq, Hash)]
    pub enum GenericArguments {
        AngleBracketed(AngleBracketedGenericArguments),
        SpaceSeparated(SpaceSeparatedGenericArguments),
    }

    /// A comma-separated list of generic arguments enclosed in angle tokens.
    #[cfg_derive!(Clone, Debug, Eq, PartialEq, Hash)]
    pub struct AngleBracketedGenericArguments {
        pub langle_token: TLToken![<],
        pub args: Punctuated<ParameterizedPath, TLToken![,]>,
        pub rangle_token: TLToken![>],
    }

    /// A space-separated list of generic arguments.
    #[cfg_derive!(Clone, Debug, Eq, PartialEq, Hash)]
    pub struct SpaceSeparatedGenericArguments {
        pub args: Vec<ParameterizedPath>,
    }

    /// A dot-separated list of identifiers with optional generic arguments that
    /// spans over a single token tree group.
    #[cfg_derive!(Clone, Debug, Eq, PartialEq, Hash)]
    pub enum SafeParameterizedPath {
        SpaceImmune(SafeParameterizedPathSpaceImmune),
        Parenthesized(SafeParameterizedPathParenthesized),
    }

    /// A parameterized path that spans a single token tree group.
    #[cfg_derive!(Clone, Debug, Eq, PartialEq, Hash)]
    pub struct SafeParameterizedPathSpaceImmune {
        pub path: Path,
        pub args: Option<AngleBracketedGenericArguments>,
    }

    /// An arbitrary parameterized path enclosed in parentheses.
    #[cfg_derive!(Clone, Debug, Eq, PartialEq, Hash)]
    pub struct SafeParameterizedPathParenthesized {
        pub paren_token: Paren,
        pub parameterized_path: ParameterizedPath,
    }
}


mod spanned {
    use super::*;
    use span::Span;
    use spanned::Spanned;
    use spanned::private::Sealed;

    impl Sealed for Path {}
    impl Sealed for ParameterizedPath {}
    impl Sealed for GenericArguments {}
    impl Sealed for AngleBracketedGenericArguments {}
    impl Sealed for SpaceSeparatedGenericArguments {}
    impl Sealed for SafeParameterizedPath {}
    impl Sealed for SafeParameterizedPathSpaceImmune {}
    impl Sealed for SafeParameterizedPathParenthesized {}

    impl Spanned for Path {
        fn span(&self) -> Span {
            self.segments.span()
        }
    }

    impl Spanned for ParameterizedPath {
        fn span(&self) -> Span {
            self.path.span()
                .to(self.args.span())
        }
    }

    impl Spanned for GenericArguments {
        fn span(&self) -> Span {
            match *self {
                GenericArguments::AngleBracketed(ref t) => t.span(),
                GenericArguments::SpaceSeparated(ref t) => t.span(),
            }
        }
    }

    impl Spanned for AngleBracketedGenericArguments {
        fn span(&self) -> Span {
            self.langle_token.span()
                .to(self.args.span())
                .to(self.rangle_token.span())
        }
    }

    impl Spanned for SpaceSeparatedGenericArguments {
        fn span(&self) -> Span {
            self.args.span()
        }
    }

    impl Spanned for SafeParameterizedPath {
        fn span(&self) -> Span {
            match *self {
                SafeParameterizedPath::SpaceImmune(ref t) => t.span(),
                SafeParameterizedPath::Parenthesized(ref t) => t.span(),
            }
        }
    }

    impl Spanned for SafeParameterizedPathSpaceImmune {
        fn span(&self) -> Span {
            self.path.span()
                .to(self.args.span())
        }
    }

    impl Spanned for SafeParameterizedPathParenthesized {
        fn span(&self) -> Span {
            self.paren_token.span()
                .to(self.parameterized_path.span())
        }
    }
}


#[cfg(feature = "parsing")]
mod parsing {
    use nom;

    use super::*;
    use cursor::Cursor;
    use punctuated::{Count, TrailingPunctuation, Whitespace};
    use synom::Synom;
    use synom::private::Sealed;

    impl Sealed for Path {}
    impl Sealed for ParameterizedPath {}
    impl Sealed for GenericArguments {}
    impl Sealed for AngleBracketedGenericArguments {}
    impl Sealed for SpaceSeparatedGenericArguments {}
    impl Sealed for SafeParameterizedPath {}
    impl Sealed for SafeParameterizedPathSpaceImmune {}
    impl Sealed for SafeParameterizedPathParenthesized {}

    impl Synom for Path {
        named!(parse_cursor(Cursor) -> Path, do_parse!(
            segments: call!(|s| Punctuated::<Ident, TLToken![.]>::parse(
                s,
                TrailingPunctuation::None,
                Count::OneOrMore,
                Whitespace::None,
            )) >>

            (Path { segments })
        ));
    }

    impl Synom for ParameterizedPath {
        named!(parse_cursor(Cursor) -> ParameterizedPath, do_parse!(
            path: tlsyn!(Path) >>
            call!(nom::space0) >>
            args: opt!(tlsyn!(GenericArguments)) >>

            (ParameterizedPath { path, args })
        ));
    }

    impl Synom for GenericArguments {
        named!(parse_cursor(Cursor) -> GenericArguments, alt_complete!(
            tlsyn!(AngleBracketedGenericArguments) => { GenericArguments::AngleBracketed }
            |
            tlsyn!(SpaceSeparatedGenericArguments) => { GenericArguments::SpaceSeparated }
        ));
    }

    impl Synom for AngleBracketedGenericArguments {
        named!(parse_cursor(Cursor) -> AngleBracketedGenericArguments, do_parse!(
            langle_token: tlpunct!(<) >>
            args: call!(|s| Punctuated::<ParameterizedPath, TLToken![,]>::parse(
                s,
                TrailingPunctuation::Optional,
                Count::OneOrMore,
                Whitespace::Present,
            )) >>
            rangle_token: tlpunct!(>) >>

            (AngleBracketedGenericArguments { langle_token, args, rangle_token })
        ));
    }

    impl Synom for SpaceSeparatedGenericArguments {
        named!(parse_cursor(Cursor) -> SpaceSeparatedGenericArguments, do_parse!(
            args: many1!(with_afterspace!(tlsyn!(ParameterizedPath))) >>

            (SpaceSeparatedGenericArguments { args })
        ));
    }

    impl Synom for SafeParameterizedPath {
        named!(parse_cursor(Cursor) -> SafeParameterizedPath, alt_complete!(
            tlsyn!(SafeParameterizedPathSpaceImmune) => { SafeParameterizedPath::SpaceImmune }
            |
            tlsyn!(SafeParameterizedPathParenthesized) => { SafeParameterizedPath::Parenthesized }
        ));
    }

    impl Synom for SafeParameterizedPathSpaceImmune {
        named!(parse_cursor(Cursor) -> SafeParameterizedPathSpaceImmune, do_parse!(
            path: tlsyn!(Path) >>
            args: opt!(tlsyn!(AngleBracketedGenericArguments)) >>

            (SafeParameterizedPathSpaceImmune { path, args })
        ));
    }

    impl Synom for SafeParameterizedPathParenthesized {
        named!(parse_cursor(Cursor) -> SafeParameterizedPathParenthesized, do_parse!(
            parameterized_path: parens!(tlsyn!(ParameterizedPath)) >>

            (SafeParameterizedPathParenthesized {
                paren_token: parameterized_path.0,
                parameterized_path: parameterized_path.1,
            })
        ));
    }
}


#[cfg(feature = "printing")]
mod printing {
    use std::fmt;

    use super::*;
    use print::Print;
    use print::private::Sealed;
    use punctuated::{Count, Whitespace};

    impl Sealed for Path {}
    impl Sealed for ParameterizedPath {}
    impl Sealed for GenericArguments {}
    impl Sealed for AngleBracketedGenericArguments {}
    impl Sealed for SpaceSeparatedGenericArguments {}
    impl Sealed for SafeParameterizedPath {}
    impl Sealed for SafeParameterizedPathSpaceImmune {}
    impl Sealed for SafeParameterizedPathParenthesized {}

    impl Print for Path {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.segments.print(f, Count::OneOrMore, Whitespace::None)
        }
    }

    impl Print for ParameterizedPath {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.path.print(f)?;
            if let Some(GenericArguments::SpaceSeparated(_)) = self.args {
                f.write_str(" ")?;
            }
            self.args.print(f)?;

            Ok(())
        }
    }

    impl Print for GenericArguments {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                GenericArguments::AngleBracketed(ref t) => t.print(f),
                GenericArguments::SpaceSeparated(ref t) => t.print(f),
            }
        }
    }

    impl Print for AngleBracketedGenericArguments {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.langle_token.print(f)?;
            self.args.print(f, Count::OneOrMore, Whitespace::Present)?;
            self.rangle_token.print(f)?;

            Ok(())
        }
    }

    impl Print for SpaceSeparatedGenericArguments {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let (first, rest) = self.args.split_first().unwrap();  // There must be at least one argument

            first.print(f)?;

            for other in rest {
                f.write_str(" ")?;
                other.print(f)?;
            }

            Ok(())
        }
    }

    impl Print for SafeParameterizedPath {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                SafeParameterizedPath::SpaceImmune(ref t) => t.print(f),
                SafeParameterizedPath::Parenthesized(ref t) => t.print(f),
            }
        }
    }

    impl Print for SafeParameterizedPathSpaceImmune {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.path.print(f)?;
            self.args.print(f)?;

            Ok(())
        }
    }

    impl Print for SafeParameterizedPathParenthesized {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            Paren::print(f, |f| {
                self.parameterized_path.print(f)
            })
        }
    }
}
