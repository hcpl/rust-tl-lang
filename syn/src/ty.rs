use super::{Ident, SafeParameterizedPath};


macro_attr_many! {
    // FIXME: handle arbitrary parameterized paths when inside parentheses.
    /// The possible types that can appear in TL declarations.
    #[cfg_derive!(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum Type {
        Int(TypeInt),
        ParameterizedPath(TypeParameterizedPath),
        TypeParameter(TypeTypeParameter),
        Bare(TypeBare),
    }

    /// A special type of integers in range from 0 to 2^31-1 inclusive: `#`.
    #[cfg_derive!(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct TypeInt {
        pub hash_token: TLToken![#],
    }

    /// A type represented by a safe parameterized path: `contacts.Link`, `messages.Chats`.
    #[cfg_derive!(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct TypeParameterizedPath {
        pub safe_parameterized_path: SafeParameterizedPath,
    }

    /// A type parameter: `!X`.
    #[cfg_derive!(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct TypeTypeParameter {
        pub excl_token: TLToken![!],
        pub ident: Ident,
    }

    /// A bare type: `%(Tuple X n)`.
    #[cfg_derive!(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct TypeBare {
        pub percent_token: TLToken![%],
        pub inner: Box<Type>,
    }
}


mod spanned {
    use super::*;
    use span::Span;
    use spanned::Spanned;
    use spanned::private::Sealed;

    impl Sealed for Type {}
    impl Sealed for TypeInt {}
    impl Sealed for TypeParameterizedPath {}
    impl Sealed for TypeTypeParameter {}
    impl Sealed for TypeBare {}

    impl Spanned for Type {
        fn span(&self) -> Span {
            match *self {
                Type::Int(ref t) => t.span(),
                Type::ParameterizedPath(ref t) => t.span(),
                Type::TypeParameter(ref t) => t.span(),
                Type::Bare(ref t) => t.span(),
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

    impl Spanned for TypeBare {
        fn span(&self) -> Span {
            self.percent_token.span()
                .to(self.inner.span())
        }
    }
}


#[cfg(feature = "parsing")]
mod parsing {
    use super::*;
    use cursor::Cursor;
    use synom::Synom;
    use synom::private::Sealed;

    impl Sealed for Type {}
    impl Sealed for TypeInt {}
    impl Sealed for TypeParameterizedPath {}
    impl Sealed for TypeTypeParameter {}
    impl Sealed for TypeBare {}

    impl Synom for Type {
        named!(parse_cursor(Cursor) -> Type, alt_complete!(
            tlsyn!(TypeInt) => { Type::Int }
            |
            tlsyn!(TypeParameterizedPath) => { Type::ParameterizedPath }
            |
            tlsyn!(TypeTypeParameter) => { Type::TypeParameter }
            |
            tlsyn!(TypeBare) => { Type::Bare }
        ));
    }

    impl Synom for TypeInt {
        named!(parse_cursor(Cursor) -> TypeInt, do_parse!(
            hash_token: tlpunct!(#) >>
            (TypeInt { hash_token })
        ));
    }

    impl Synom for TypeParameterizedPath {
        named!(parse_cursor(Cursor) -> TypeParameterizedPath, do_parse!(
            safe_parameterized_path: tlsyn!(SafeParameterizedPath) >>
            (TypeParameterizedPath { safe_parameterized_path })
        ));
    }

    impl Synom for TypeTypeParameter {
        named!(parse_cursor(Cursor) -> TypeTypeParameter, do_parse!(
            excl_token: tlpunct!(!) >>
            ident: tlsyn!(Ident) >>
            (TypeTypeParameter { excl_token, ident })
        ));
    }

    impl Synom for TypeBare {
        named!(parse_cursor(Cursor) -> TypeBare, do_parse!(
            percent_token: tlpunct!(%) >>
            inner: map!(tlsyn!(Type), Box::new) >>
            (TypeBare { percent_token, inner })
        ));
    }
}


#[cfg(feature = "printing")]
mod printing {
    use std::fmt;

    use super::*;
    use print::Print;
    use print::private::Sealed;

    impl Sealed for Type {}
    impl Sealed for TypeInt {}
    impl Sealed for TypeParameterizedPath {}
    impl Sealed for TypeTypeParameter {}
    impl Sealed for TypeBare {}

    impl Print for Type {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Type::Int(ref t) => t.print(f),
                Type::ParameterizedPath(ref t) => t.print(f),
                Type::TypeParameter(ref t) => t.print(f),
                Type::Bare(ref t) => t.print(f),
            }
        }
    }

    impl Print for TypeInt {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.hash_token.print(f)
        }
    }

    impl Print for TypeParameterizedPath {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.safe_parameterized_path.print(f)
        }
    }

    impl Print for TypeTypeParameter {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.excl_token.print(f)?;
            self.ident.print(f)?;

            Ok(())
        }
    }

    impl Print for TypeBare {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.percent_token.print(f)?;
            self.inner.print(f)?;

            Ok(())
        }
    }
}
