use std::fmt;

use super::{Ident, SafeParameterizedPath};
use span::Span;
use spanned::Spanned;
use synom::Synom;


/// The possible types that can appear in TL declarations.
#[derive(Debug)]
pub enum Type {
    Int(TypeInt),
    ParameterizedPath(TypeParameterizedPath),
    TypeParameter(TypeTypeParameter),
}

/// A special type of integers in range from 0 to 2^31-1 inclusive: `#`.
#[derive(Debug)]
pub struct TypeInt {
    pub hash_token: TLToken![#],
}

/// A type represented by a safe parameterized path: `contacts.Link`, `messages.Chats`.
#[derive(Debug)]
pub struct TypeParameterizedPath {
    pub safe_parameterized_path: SafeParameterizedPath,
}

/// A type parameter: `!X`.
#[derive(Debug)]
pub struct TypeTypeParameter {
    pub excl_token: TLToken![!],
    pub ident: Ident,
}


impl Synom for Type {
    named!(parse_str(&str) -> Type, alt_complete!(
        tlsyn!(TypeInt) => { Type::Int }
        |
        tlsyn!(TypeParameterizedPath) => { Type::ParameterizedPath }
        |
        tlsyn!(TypeTypeParameter) => { Type::TypeParameter }
    ));
}

impl Synom for TypeInt {
    named!(parse_str(&str) -> TypeInt, do_parse!(
        hash_token: tlpunct!(#) >>
        (TypeInt { hash_token })
    ));
}

impl Synom for TypeParameterizedPath {
    named!(parse_str(&str) -> TypeParameterizedPath, do_parse!(
        safe_parameterized_path: tlsyn!(SafeParameterizedPath) >>
        (TypeParameterizedPath { safe_parameterized_path })
    ));
}

impl Synom for TypeTypeParameter {
    named!(parse_str(&str) -> TypeTypeParameter, do_parse!(
        excl_token: tlpunct!(!) >>
        ident: tlsyn!(Ident) >>
        (TypeTypeParameter { excl_token, ident })
    ));
}


impl Spanned for Type {
    fn span(&self) -> Span {
        match *self {
            Type::Int(ref t) => t.span(),
            Type::ParameterizedPath(ref t) => t.span(),
            Type::TypeParameter(ref t) => t.span(),
        }
    }
}

impl Spanned for TypeInt {
    fn span(&self) -> Span {
        self.hash_token.span()
    }
}

impl Spanned for TypeParameterizedPath {
    fn span(&self) -> Span {
        self.safe_parameterized_path.span()
    }
}

impl Spanned for TypeTypeParameter {
    fn span(&self) -> Span {
        self.excl_token.span()
            .to(self.ident.span())
    }
}


impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Int(ref t) => fmt::Display::fmt(t, f),
            Type::ParameterizedPath(ref t) => fmt::Display::fmt(t, f),
            Type::TypeParameter(ref t) => fmt::Display::fmt(t, f),
        }
    }
}

impl fmt::Display for TypeInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.hash_token, f)
    }
}

impl fmt::Display for TypeParameterizedPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.safe_parameterized_path, f)
    }
}

impl fmt::Display for TypeTypeParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.excl_token, f)?;
        fmt::Display::fmt(&self.ident, f)?;

        Ok(())
    }
}
