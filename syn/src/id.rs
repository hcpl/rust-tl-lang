use std::fmt;

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
    named!(parse_str(&str) -> Id, do_parse!(
        // Doesn't work for `storage.fileJpeg#7efe0e = storage.FileType;`
        //id: map_res!(take_while_m_n!(8, 8, is_hex_digit), u32_from_hex_str) >>
        id: map_res!(take_while!(is_hex_digit), u32_from_hex_str) >>

        (Id { span: Span::empty(), id })
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
