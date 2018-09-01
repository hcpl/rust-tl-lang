use span::Span;


macro_attr_many! {
    /// An index pointing to the n-th bit of a `#` value (or, an `u32` value).
    #[cfg_derive!(Clone, Debug)]
    pub struct BitIndex {
        span: Span,
        index: u8,
    }
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
    /// The index must satisfy `0 <= index < 32`.
    ///
    /// If conditions are not met, it is a violation of safety guarantees.
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
mod eq_impls {
    use super::*;

    impl Eq for BitIndex {}

    impl PartialEq for BitIndex {
        fn eq(&self, other: &BitIndex) -> bool {
            self.index == other.index
        }
    }
}

#[cfg(feature = "hash-impls")]
mod hash_impls {
    use std::hash::{Hash, Hasher};

    use super::*;

    impl Hash for BitIndex {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.index.hash(state);
        }
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
    use utils::parsing::is_decimal_digit;

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


#[cfg(test)]
mod tests {
    #[cfg(feature = "eq-impls")]
    use super::*;
    #[cfg(feature = "eq-impls")]
    use span::Span;
    #[cfg(feature = "eq-impls")]
    use utils::tests::test_span_permutations;
    #[cfg(all(feature = "eq-impls", feature = "hash-impls"))]
    use utils::tests::get_hasher_state;


    #[cfg(feature = "eq-impls")]
    #[test]
    fn valid_bit_indices() {
        for i in 0..32 {
            let actual = BitIndex::new(Span::zeroed(), i as u8);
            let expected = Some(BitIndex { span: Span::zeroed(), index: i as u8 });

            any_debug_assert_eq!(actual, expected);
        }

        for i in 32..256 {
            let actual = BitIndex::new(Span::zeroed(), i as u8);
            let expected = None;

            any_debug_assert_eq!(actual, expected);
        }
    }

    #[cfg(feature = "eq-impls")]
    fn test_bit_index_span_permutations<FT, FA1, FA2>(
        test_eq: FT,
        assert_when_eq: FA1,
        assert_when_ne: FA2,
    )
    where
        FT: Fn(&BitIndex, &BitIndex) -> bool,
        FA1: Fn(&BitIndex, &BitIndex),
        FA2: Fn(&BitIndex, &BitIndex),
    {
        for i1 in 0..32 {
            for i2 in 0..32 {
                test_span_permutations(
                    |span1| BitIndex::new(span1, i1).unwrap(),
                    |span2| BitIndex::new(span2, i2).unwrap(),
                    &test_eq,
                    &assert_when_eq,
                    &assert_when_ne,
                );
            }
        }
    }

    #[cfg(feature = "eq-impls")]
    #[test]
    fn eq_does_not_depend_on_span() {
        test_bit_index_span_permutations(
            |bi1, bi2| bi1.index() == bi2.index(),
            |bi1, bi2| any_debug_assert_eq!(bi1, bi2),
            |bi1, bi2| any_debug_assert_ne!(bi1, bi2),
        );
    }

    #[cfg(all(feature = "eq-impls", feature = "hash-impls"))]
    #[test]
    fn eq_hash_property() {
        test_bit_index_span_permutations(
            |bi1, bi2| bi1 == bi2,
            |bi1, bi2| any_debug_assert_eq!(get_hasher_state(bi1), get_hasher_state(bi2)),
            |bi1, bi2| any_debug_assert_ne!(get_hasher_state(bi1), get_hasher_state(bi2)),
        );
    }
}
