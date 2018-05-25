#[cfg(feature = "printing")]
use std::fmt;

#[cfg(feature = "parsing")]
use cursor::Cursor;
#[cfg(feature = "printing")]
use print::Print;
#[cfg(feature = "parsing")]
use synom::Synom;
use span::Span;
use spanned::Spanned;
#[cfg(feature = "parsing")]
use utils::is_decimal_digit;


/// An index pointing to the n-th bit of a `#` value (or, an `u32` value).
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct BitIndex {
    span: Span,
    index: u8,
}

impl BitIndex {
    pub fn new(span: Span, index: u8) -> Option<BitIndex> {
        if is_valid_nat_bit_index(index) {
            Some(BitIndex { span, index })
        } else {
            None
        }
    }

    pub unsafe fn new_unchecked(span: Span, index: u8) -> BitIndex {
        BitIndex { span, index }
    }

    pub fn index(&self) -> u8 {
        self.index
    }
}

fn is_valid_nat_bit_index(index_u8: u8) -> bool {
    const BIT_INDEX_MASK: u8 = 0b0001_1111;

    index_u8 & !BIT_INDEX_MASK == 0
}

#[cfg(feature = "eq-impls")]
impl Eq for BitIndex {}

#[cfg(feature = "eq-impls")]
impl PartialEq for BitIndex {
    fn eq(&self, other: &BitIndex) -> bool {
        self.index == other.index
    }
}

#[cfg(feature = "parsing")]
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

#[cfg(feature = "printing")]
impl Print for BitIndex {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.index, f)
    }
}
