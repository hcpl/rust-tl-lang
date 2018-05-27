use span::Span;


/// A 32-bit number which identifies a TL combinator.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct Id {
    pub span: Span,
    pub id: u32,
}

#[cfg(feature = "eq-impls")]
mod eq_impls {
    use super::*;

    impl Eq for Id {}

    impl PartialEq for Id {
        fn eq(&self, other: &Id) -> bool {
            self.id == other.id
        }
    }
}

#[cfg(feature = "hash-impls")]
mod hash_impls {
    use std::hash::{Hash, Hasher};

    use super::*;

    impl Hash for Id {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }
}

mod spanned {
    use super::*;
    use span::Span;
    use spanned::Spanned;
    use spanned::private::Sealed;

    impl Sealed for Id {}

    impl Spanned for Id {
        fn span(&self) -> Span {
            self.span
        }
    }
}

#[cfg(feature = "parsing")]
mod parsing {
    use super::*;
    use cursor::Cursor;
    use synom::Synom;
    use synom::private::Sealed;
    use utils::{is_hex_digit, u32_from_hex_str};

    impl Sealed for Id {}

    impl Synom for Id {
        named!(parse_cursor(Cursor) -> Id, do_parse!(
            // (8, 8) doesn't work for `storage.fileJpeg#7efe0e = storage.FileType;`
            //id_cursor: take_while_m_n!(8, 8, is_hex_digit) >>
            // Cap at 8 hex digits, because ids are 32-bit numbers, but there
            // must be at least one
            id_cursor: take_while_m_n!(1, 8, is_hex_digit) >>
            id: map_res!(value!(id_cursor.to_str()), u32_from_hex_str) >>
            span: value!(id_cursor.span()) >>

            (Id { span, id })
        ));
    }
}

#[cfg(feature = "printing")]
mod printing {
    use std::fmt;

    use super::*;
    use print::Print;
    use print::private::Sealed;

    impl Sealed for Id {}

    impl Print for Id {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::LowerHex::fmt(&self.id, f)
        }
    }
}
