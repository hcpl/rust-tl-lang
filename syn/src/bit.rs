use span::Span;


/// An index pointing to the n-th bit of a `#` value (or, an `u32` value).
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct BitIndex {
    span: Span,
    index: u8,
}

impl BitIndex {
    /// Create a new `BitIndex` with the given `span` and the given `index` if
    /// `0 <= index < 32`.
    pub fn new(span: Span, index: u8) -> Option<BitIndex> {
        if is_valid_nat_bit_index(index) {
            Some(BitIndex { span, index })
        } else {
            None
        }
    }

    /// Create a new `BitIndex` with the given `span` and the given `index`
    /// without checking.
    ///
    /// # Safety
    ///
    /// The index must satisfy `0 <= index < 32`
    pub unsafe fn new_unchecked(span: Span, index: u8) -> BitIndex {
        BitIndex { span, index }
    }

    /// Get the index.
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

mod spanned {
    use super::*;
    use span::Span;
    use spanned::Spanned;
    use spanned::private::Sealed;

    impl Sealed for BitIndex {}

    impl Spanned for BitIndex {
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
    use utils::is_decimal_digit;

    impl Sealed for BitIndex {}

    impl Synom for BitIndex {
        named!(parse_cursor(Cursor) -> BitIndex, do_parse!(
            index_str_cursor: take_while!(is_decimal_digit) >>
            index_raw: map_res!(value!(index_str_cursor.to_str()), str::parse) >>
            index: verify!(value!(index_raw), is_valid_nat_bit_index) >>
            span: value!(index_str_cursor.span()) >>

            (BitIndex { span, index })
        ));
    }
}

#[cfg(feature = "printing")]
mod printing {
    use std::fmt;

    use super::*;
    use print::Print;
    use print::private::Sealed;

    impl Sealed for BitIndex {}

    impl Print for BitIndex {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(&self.index, f)
        }
    }
}
