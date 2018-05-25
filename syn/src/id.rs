use span::Span;


/// A 32-bit number which identifies a TL combinator.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct Id {
    pub span: Span,
    pub id: u32,
}

#[cfg(feature = "eq-impls")]
impl Eq for Id {}

#[cfg(feature = "eq-impls")]
impl PartialEq for Id {
    fn eq(&self, other: &Id) -> bool {
        self.id == other.id
    }
}

mod spanned {
    use super::*;
    use spanned::Spanned;

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
    use utils::{is_hex_digit, u32_from_hex_str};

    impl Synom for Id {
        named!(parse_cursor(Cursor) -> Id, do_parse!(
            // (8, 8) doesn't work for `storage.fileJpeg#7efe0e = storage.FileType;`
            //id_cursor: take_while_m_n!(8, 8, is_hex_digit) >>
            // Cap at 8 hex digits, because ids are 32-bit numbers, but the must be
            // at least one
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

    impl Print for Id {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::LowerHex::fmt(&self.id, f)
        }
    }
}
