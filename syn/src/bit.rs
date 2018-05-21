use synom::Synom;
use span::Span;
use spanned::Spanned;
use utils::is_decimal_digit;


const BIT_INDEX_MASK: u8 = 0b00011111;

/// An index pointing to the n-th bit of a `#` value (or, an `u32` value).
#[derive(Debug)]
pub struct BitIndex {
    span: Span,
    index: u8,
}

impl Synom for BitIndex {
    named!(parse_str(&str) -> BitIndex, do_parse!(
        index_u8: map_res!(take_while!(is_decimal_digit), str::parse) >>
        index: verify!(value!(index_u8), is_valid_nat_bit_index) >>

        (BitIndex { span: Span::empty(), index })
    ));
}

impl Spanned for BitIndex {
    fn span(&self) -> Span {
        self.span
    }
}


fn is_valid_nat_bit_index(index_u8: u8) -> bool {
    index_u8 & !BIT_INDEX_MASK == 0
}
