use std::fmt;

use super::Ident;
use print::Print;
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


impl Print for Path {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.segments.print(f, Count::OneOrMore, Whitespace::None)
    }
}

impl Print for ParameterizedPath {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.path.print(f)?;
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
