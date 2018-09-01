use span::Span;


macro_attr_many! {
    /// A 32-bit number which identifies a TL combinator.
    #[cfg_derive!(Clone, Debug)]
    pub struct Id {
        pub span: Span,
        pub id: u32,
    }
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
    use utils::parsing::{is_hex_digit, u32_from_hex_str};

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


#[cfg(test)]
mod tests {
    #[cfg(feature = "eq-impls")]
    use super::*;
    #[cfg(feature = "eq-impls")]
    use utils::tests::test_span_permutations;
    #[cfg(all(feature = "eq-impls", feature = "hash-impls"))]
    use utils::tests::get_hasher_state;


    #[cfg(feature = "eq-impls")]
    fn test_id_span_permutations<FT, FA1, FA2>(
        test_eq: FT,
        assert_when_eq: FA1,
        assert_when_ne: FA2,
    )
    where
        FT: Fn(&Id, &Id) -> bool,
        FA1: Fn(&Id, &Id),
        FA2: Fn(&Id, &Id),
    {
        let ids = [0, 1, 2, 100, 0xFF, 0xFFFF, 0xFFFFFFFF];

        for id1 in &ids {
            for id2 in &ids {
                test_span_permutations(
                    |span1| Id { span: span1, id: *id1 },
                    |span2| Id { span: span2, id: *id2 },
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
        test_id_span_permutations(
            |x, y| x.id == y.id,
            |x, y| any_debug_assert_eq!(x, y),
            |x, y| any_debug_assert_ne!(x, y),
        );
    }

    #[cfg(all(feature = "eq-impls", feature = "hash-impls"))]
    #[test]
    fn eq_hash_property() {
        test_id_span_permutations(
            |x, y| x == y,
            |x, y| any_debug_assert_eq!(get_hasher_state(x), get_hasher_state(y)),
            |x, y| any_debug_assert_ne!(get_hasher_state(x), get_hasher_state(y)),
        );
    }
}
