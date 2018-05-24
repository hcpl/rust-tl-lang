use std::fmt;

use super::Ident;
use punctuated::{Count, Punctuated, TrailingPunctuation, Whitespace};
use span::Span;
use spanned::Spanned;
use synom::Synom;
use token::Paren;


/// A dot-separated list of identifiers.
#[derive(Debug)]
pub struct Path {
    pub segments: Punctuated<Ident, TLToken![.]>,
}

/// A dot-separated list of identifiers with optional generic arguments.
#[derive(Debug)]
pub struct ParameterizedPath {
    pub path: Path,
    pub args: Option<GenericArguments>,
}

/// Generic arguments for parameterized paths.
#[derive(Debug)]
pub enum GenericArguments {
    AngleBracketed(AngleBracketedGenericArguments),
    SpaceSeparated(SpaceSeparatedGenericArguments),
}

/// A comma-separated list of generic arguments enclosed in angle tokens.
#[derive(Debug)]
pub struct AngleBracketedGenericArguments {
    pub langle_token: TLToken![<],
    pub args: Punctuated<ParameterizedPath, TLToken![,]>,
    pub rangle_token: TLToken![>],
}

/// A space-separated list of generic arguments.
#[derive(Debug)]
pub struct SpaceSeparatedGenericArguments {
    pub args: Vec<ParameterizedPath>,
}

/// A dot-separated list of identifiers with optional generic arguments that
/// spans over a single token tree group.
#[derive(Debug)]
pub enum SafeParameterizedPath {
    SpaceImmune(SafeParameterizedPathSpaceImmune),
    Parenthesized(SafeParameterizedPathParenthesized),
}

/// A parameterized path that spans a single token tree group.
#[derive(Debug)]
pub struct SafeParameterizedPathSpaceImmune {
    pub path: Path,
    pub args: Option<AngleBracketedGenericArguments>,
}

/// An arbitrary parameterized path enclosed in parentheses.
#[derive(Debug)]
pub struct SafeParameterizedPathParenthesized {
    pub paren_token: Paren,
    pub parameterized_path: ParameterizedPath,
}


impl Synom for Path {
    named!(parse_str(&str) -> Path, do_parse!(
        segments: call!(|s| Punctuated::parse(
            s,
            TrailingPunctuation::None,
            Count::OneOrMore,
            Whitespace::None,
        )) >>

        (Path { segments })
    ));
}

impl Synom for ParameterizedPath {
    named!(parse_str(&str) -> ParameterizedPath, do_parse!(
        path: tlsyn!(Path) >>
        args: opt!(tlsyn!(GenericArguments)) >>

        (ParameterizedPath { path, args })
    ));
}

impl Synom for GenericArguments {
    named!(parse_str(&str) -> GenericArguments, alt_complete!(
        tlsyn!(AngleBracketedGenericArguments) => { GenericArguments::AngleBracketed }
        |
        tlsyn!(SpaceSeparatedGenericArguments) => { GenericArguments::SpaceSeparated }
    ));
}

impl Synom for AngleBracketedGenericArguments {
    named!(parse_str(&str) -> AngleBracketedGenericArguments, do_parse!(
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
    named!(parse_str(&str) -> SpaceSeparatedGenericArguments, do_parse!(
        args: sp!(many1!(tlsyn!(ParameterizedPath))) >>

        (SpaceSeparatedGenericArguments { args })
    ));
}

impl Synom for SafeParameterizedPath {
    named!(parse_str(&str) -> SafeParameterizedPath, alt_complete!(
        tlsyn!(SafeParameterizedPathSpaceImmune) => { SafeParameterizedPath::SpaceImmune }
        |
        tlsyn!(SafeParameterizedPathParenthesized) => { SafeParameterizedPath::Parenthesized }
    ));
}

impl Synom for SafeParameterizedPathSpaceImmune {
    named!(parse_str(&str) -> SafeParameterizedPathSpaceImmune, do_parse!(
        path: tlsyn!(Path) >>
        args: opt!(tlsyn!(AngleBracketedGenericArguments)) >>

        (SafeParameterizedPathSpaceImmune { path, args })
    ));
}

impl Synom for SafeParameterizedPathParenthesized {
    named!(parse_str(&str) -> SafeParameterizedPathParenthesized, do_parse!(
        parameterized_path: parens!(tlsyn!(ParameterizedPath)) >>

        (SafeParameterizedPathParenthesized {
            paren_token: parameterized_path.0,
            parameterized_path: parameterized_path.1,
        })
    ));
}


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


impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.segments.print(f, Whitespace::None)
    }
}

impl fmt::Display for ParameterizedPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.path, f)?;
        if let Some(ref args) = self.args {
            fmt::Display::fmt(args, f)?;
        }

        Ok(())
    }
}

impl fmt::Display for GenericArguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenericArguments::AngleBracketed(ref t) => fmt::Display::fmt(t, f),
            GenericArguments::SpaceSeparated(ref t) => fmt::Display::fmt(t, f),
        }
    }
}

impl fmt::Display for AngleBracketedGenericArguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.langle_token, f)?;
        self.args.print(f, Whitespace::Present)?;
        fmt::Display::fmt(&self.rangle_token, f)?;

        Ok(())
    }
}

impl fmt::Display for SpaceSeparatedGenericArguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut args_iter = self.args.iter();

        let first_arg = args_iter.next().unwrap();
        fmt::Display::fmt(first_arg, f)?;

        for other_arg in args_iter {
            fmt::Display::fmt(" ", f)?;
            fmt::Display::fmt(other_arg, f)?;
        }

        Ok(())
    }
}

impl fmt::Display for SafeParameterizedPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SafeParameterizedPath::SpaceImmune(ref t) => fmt::Display::fmt(t, f),
            SafeParameterizedPath::Parenthesized(ref t) => fmt::Display::fmt(t, f),
        }
    }
}

impl fmt::Display for SafeParameterizedPathSpaceImmune {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.path, f)?;
        if let Some(ref args) = self.args {
            fmt::Display::fmt(args, f)?;
        }

        Ok(())
    }
}

impl fmt::Display for SafeParameterizedPathParenthesized {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Paren::print(f, |f| {
            fmt::Display::fmt(&self.parameterized_path, f)
        })
    }
}
