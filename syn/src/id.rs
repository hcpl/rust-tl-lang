use std::fmt;

use cursor::Cursor;
use print::Print;
use span::Span;
use spanned::Spanned;
use synom::Synom;
use utils::{is_hex_digit, u32_from_hex_str};


/// A 32-bit number which identifies a TL combinator.
#[derive(Debug)]
pub struct Id {
    span: Span,
    id: u32,
}

impl Id {
    pub fn new(span: Span, id: u32) -> Self {
        Id { span, id }
    }
}

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

impl Spanned for Id {
    fn span(&self) -> Span {
        self.span
    }
}

impl Print for Id {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.id, f)
    }
}
