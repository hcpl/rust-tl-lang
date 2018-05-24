use std::fmt;

use cursor::Cursor;
use print::Print;
use synom::Synom;
use span::Span;
use spanned::Spanned;
use utils::is_decimal_digit;


const BIT_INDEX_MASK: u8 = 0b0001_1111;

/// An index pointing to the n-th bit of a `#` value (or, an `u32` value).
#[derive(Debug)]
pub struct BitIndex {
    span: Span,
    index: u8,
}

impl Synom for BitIndex {
    named!(parse_cursor(Cursor) -> BitIndex, do_parse!(
        index_str_cursor: take_while!(is_decimal_digit) >>
        index_raw: map_res!(value!(index_str_cursor.to_str()), str::parse) >>
        index: verify!(value!(index_raw), is_valid_nat_bit_index) >>
        span: value!(index_str_cursor.span()) >>

        (BitIndex { span, index })
    ));
}

impl Spanned for BitIndex {
    fn span(&self) -> Span {
        self.span
    }
}

impl Print for BitIndex {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.index, f)
    }
}


fn is_valid_nat_bit_index(index_u8: u8) -> bool {
    index_u8 & !BIT_INDEX_MASK == 0
}
