#[cfg(feature = "printing")]
use std::fmt;

use super::{Ident, SafeParameterizedPath};
#[cfg(feature = "parsing")]
use cursor::Cursor;
#[cfg(feature = "printing")]
use print::Print;
use span::Span;
use spanned::Spanned;
#[cfg(feature = "parsing")]
use synom::Synom;


/// The possible types that can appear in TL declarations.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub enum Type {
    Int(TypeInt),
    ParameterizedPath(TypeParameterizedPath),
    TypeParameter(TypeTypeParameter),
}

/// A special type of integers in range from 0 to 2^31-1 inclusive: `#`.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct TypeInt {
    pub hash_token: TLToken![#],
}

/// A type represented by a safe parameterized path: `contacts.Link`, `messages.Chats`.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct TypeParameterizedPath {
    pub safe_parameterized_path: SafeParameterizedPath,
}

/// A type parameter: `!X`.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct TypeTypeParameter {
    pub excl_token: TLToken![!],
    pub ident: Ident,
}


#[cfg(feature = "parsing")]
impl Synom for Type {
    named!(parse_cursor(Cursor) -> Type, alt_complete!(
        tlsyn!(TypeInt) => { Type::Int }
        |
        tlsyn!(TypeParameterizedPath) => { Type::ParameterizedPath }
        |
        tlsyn!(TypeTypeParameter) => { Type::TypeParameter }
    ));
}

#[cfg(feature = "parsing")]
impl Synom for TypeInt {
    named!(parse_cursor(Cursor) -> TypeInt, do_parse!(
        hash_token: tlpunct!(#) >>
        (TypeInt { hash_token })
    ));
}

#[cfg(feature = "parsing")]
impl Synom for TypeParameterizedPath {
    named!(parse_cursor(Cursor) -> TypeParameterizedPath, do_parse!(
        safe_parameterized_path: tlsyn!(SafeParameterizedPath) >>
        (TypeParameterizedPath { safe_parameterized_path })
    ));
}

#[cfg(feature = "parsing")]
impl Synom for TypeTypeParameter {
    named!(parse_cursor(Cursor) -> TypeTypeParameter, do_parse!(
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


#[cfg(feature = "printing")]
impl Print for Type {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Int(ref t) => t.print(f),
            Type::ParameterizedPath(ref t) => t.print(f),
            Type::TypeParameter(ref t) => t.print(f),
        }
    }
}

#[cfg(feature = "printing")]
impl Print for TypeInt {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.hash_token.print(f)
    }
}

#[cfg(feature = "printing")]
impl Print for TypeParameterizedPath {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.safe_parameterized_path.print(f)
    }
}

#[cfg(feature = "printing")]
impl Print for TypeTypeParameter {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.excl_token.print(f)?;
        self.ident.print(f)?;

        Ok(())
    }
}
