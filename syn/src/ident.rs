use span::Span;


macro_attr_many! {
    /// An identifier: `channels`, `SendMessageAction`, `X`, etc..
    #[cfg_derive!(Clone, Debug)]
    pub struct Ident {
        span: Span,
        string: String,
    }
}

impl Ident {
    /// Create a new `Ident` with the given `span` and the given `string` if
    /// the string is a valid TL language identifier.
    pub fn new(span: Span, string: &str) -> Option<Ident> {
        if is_valid_ident(string) {
            Some(Ident {
                span,
                string: string.to_owned(),
            })
        } else {
            None
        }
    }

    /// Create a new `Ident` with the given `span` and the given `string`
    /// without checking the string.
    ///
    /// # Safety
    ///
    /// The string must be a valid TL language identifier.
    ///
    /// If conditions are not met, it is a violation of safety guarantees.
    pub unsafe fn new_unchecked(span: Span, string: &str) -> Ident {
        Ident {
            span,
            string: string.to_owned(),
        }
    }

    /// Extract a string view into this `Ident`.
    pub fn as_str(&self) -> &str {
        &self.string
    }

    /// Return true if the first character of this `Ident` is lowercase, and
    /// false otherwise.
    pub fn is_lowercase(&self) -> bool {
        match self.string.chars().next() {
            Some(c) => c.is_lowercase(),
            None    => unreachable!("There must be at least one char for any `Ident`"),
        }
    }

    /// Return true if the first character of this `Ident` is uppercase, and
    /// false otherwise.
    pub fn is_uppercase(&self) -> bool {
        match self.string.chars().next() {
            Some(c) => c.is_uppercase(),
            None    => unreachable!("There must be at least one char for any `Ident`"),
        }
    }
}

#[cfg(feature = "eq-impls")]
mod eq_impls {
    use super::*;

    impl Eq for Ident {}

    impl PartialEq for Ident {
        fn eq(&self, other: &Ident) -> bool {
            self.string == other.string
        }
    }
}

#[cfg(feature = "hash-impls")]
mod hash_impls {
    use std::hash::{Hash, Hasher};

    use super::*;

    impl Hash for Ident {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.string.hash(state)
        }
    }
}

mod spanned {
    use super::*;
    use span::Span;
    use spanned::Spanned;
    use spanned::private::Sealed;

    impl Sealed for Ident {}

    impl Spanned for Ident {
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

    impl Sealed for Ident {}

    impl Synom for Ident {
        named!(parse_cursor(Cursor) -> Ident, do_parse!(
            ident_str_cursor: take_while!(is_ident_char) >>
            ident_str: verify!(value!(ident_str_cursor.to_str()), is_valid_ident) >>

            (Ident {
                span: ident_str_cursor.span(),
                string: ident_str.to_owned(),
            })
        ));
    }
}

#[cfg(feature = "printing")]
mod printing {
    use std::fmt;

    use super::*;
    use print::Print;
    use print::private::Sealed;

    impl Sealed for Ident {}

    impl Print for Ident {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(&self.string, f)
        }
    }
}


// A valid identifier char must be either of:
// * uppercase Latin letter
// * lowercase Latin letter
// * decimal digit
// * underscore
fn is_ident_char(c: char) -> bool {
    match c {
        'A' ... 'Z' |
        'a' ... 'z' |
        '0' ... '9' |
        '_'         => true,
        _           => false,
    }
}

// A valid identifier beginning char must be either of:
// * uppercase Latin letter
// * lowercase Latin letter
fn is_ident_beginning_char(c: char) -> bool {
    match c {
        'A' ... 'Z' |
        'a' ... 'z' => true,
        _           => false,
    }
}

// A valid identifier must:
// + be non-empty
// + begin with either an uppercase or a lowercase Latin letter
// + contain an uppercase or a lowercase Latin letter, a decimal digit or an underscore in other
//   positions
fn is_valid_ident(s: &str) -> bool {
    let mut chars = s.chars();

    match chars.next() {
        Some(c) => is_ident_beginning_char(c) && chars.all(is_ident_char),
        None    => false,
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
    fn test_ident_span_permutations<FT, FA1, FA2>(
        test_eq: FT,
        assert_when_eq: FA1,
        assert_when_ne: FA2,
    )
    where
        FT: Fn(&Ident, &Ident) -> bool,
        FA1: Fn(&Ident, &Ident),
        FA2: Fn(&Ident, &Ident),
    {
        let idents = ["B", "j", "x_z", "EasyHaricotPlantInformedFacetItemPlant", "r2d2"];

        assert!(idents.iter().all(|ident| is_valid_ident(ident)));

        for ident1 in &idents {
            for ident2 in &idents {
                test_span_permutations(
                    |span1| Ident { span: span1, string: (*ident1).to_owned() },
                    |span2| Ident { span: span2, string: (*ident2).to_owned() },
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
        test_ident_span_permutations(
            |x, y| x.string == y.string,
            |x, y| any_debug_assert_eq!(x, y),
            |x, y| any_debug_assert_ne!(x, y),
        );
    }

    #[cfg(all(feature = "eq-impls", feature = "hash-impls"))]
    #[test]
    fn eq_hash_property() {
        test_ident_span_permutations(
            |x, y| x == y,
            |x, y| any_debug_assert_eq!(get_hasher_state(x), get_hasher_state(y)),
            |x, y| any_debug_assert_ne!(get_hasher_state(x), get_hasher_state(y)),
        );
    }
}
