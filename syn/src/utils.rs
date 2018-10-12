#[cfg(feature = "parsing")]
pub(crate) mod parsing {
    use std::num;


    pub(crate) fn is_decimal_digit(c: char) -> bool {
        char::is_digit(c, 10)
    }

    pub(crate) fn is_hex_digit(c: char) -> bool {
        char::is_digit(c, 16)
    }

    pub(crate) fn u32_from_hex_str(s: &str) -> Result<u32, num::ParseIntError> {
        u32::from_str_radix(s, 16)
    }
}


macro_rules! macro_attr_many {
    (
        $(
            $(#[$($attrs:tt)*])*
            pub $item_type:tt $name:ident { $($it:tt)* }
        )*
    ) => {
        $(
            macro_attr! {
                $(#[$($attrs)*])*
                pub $item_type $name { $($it)* }
            }
        )*
    };
}

macro_rules! cfg_derive {
    (
        ($($args:ident),*),
        then $cb:tt,
        $(#[$($attrs:tt)*])*
        pub $($it:tt)*
    ) => {
        cfg_derive! { @expand
            ($($args),*),
            then $cb,
            $(#[$($attrs)*])*
            [pub] $($it)*
        }
    };

    (@expand
        (Clone $(, $args:ident)*),
        then $cb:tt,
        $(#[$($attrs:tt)*])*
        [$it_first_tt:tt] $($it:tt)*
    ) => {
        cfg_derive! { @expand
            ($($args),*),
            then $cb,
            $(#[$($attrs)*])*
            #[cfg_attr(feature = "clone-impls", derive(Clone))]
            [$it_first_tt] $($it)*
        }
    };

    (@expand
        (Debug $(, $args:ident)*),
        then $cb:tt,
        $(#[$($attrs:tt)*])*
        [$it_first_tt:tt] $($it:tt)*
    ) => {
        cfg_derive! { @expand
            ($($args),*),
            then $cb,
            $(#[$($attrs)*])*
            #[cfg_attr(feature = "debug-impls", derive(Debug))]
            [$it_first_tt] $($it)*
        }
    };

    (@expand
        (Eq $(, $args:ident)*),
        then $cb:tt,
        $(#[$($attrs:tt)*])*
        [$it_first_tt:tt] $($it:tt)*
    ) => {
        cfg_derive! { @expand
            ($($args),*),
            then $cb,
            $(#[$($attrs)*])*
            #[cfg_attr(feature = "eq-impls", derive(Eq))]
            [$it_first_tt] $($it)*
        }
    };

    (@expand
        (Hash $(, $args:ident)*),
        then $cb:tt,
        $(#[$($attrs:tt)*])*
        [$it_first_tt:tt] $($it:tt)*
    ) => {
        cfg_derive! { @expand
            ($($args),*),
            then $cb,
            $(#[$($attrs)*])*
            #[cfg_attr(feature = "hash-impls", derive(Hash))]
            [$it_first_tt] $($it)*
        }
    };

    (@expand
        (PartialEq $(, $args:ident)*),
        then $cb:tt,
        $(#[$($attrs:tt)*])*
        [$it_first_tt:tt] $($it:tt)*
    ) => {
        cfg_derive! { @expand
            ($($args),*),
            then $cb,
            $(#[$($attrs)*])*
            #[cfg_attr(feature = "eq-impls", derive(PartialEq))]
            [$it_first_tt] $($it)*
        }
    };

    (@expand
        ($($args:ident),*),
        then $cb:tt,
        $(#[$($attrs:tt)*])*
        [$it_first_tt:tt] $($it:tt)*
    ) => {
        macro_attr_callback! {
            $cb,
            $(#[$($attrs)*])*
            #[derive($($args),*)]
            $it_first_tt $($it)*
        }
    };
}


#[cfg(test)]
#[macro_use]
pub(crate) mod tests {
    #[cfg(all(feature = "eq-impls", feature = "hash-impls"))]
    use std::collections::hash_map::DefaultHasher;
    #[cfg(all(feature = "eq-impls", feature = "hash-impls"))]
    use std::hash::{Hash, Hasher};

    #[cfg(feature = "eq-impls")]
    use span::Span;


    #[cfg(all(not(feature = "debug-impls"), feature = "eq-impls"))]
    macro_rules! non_debug_assert_eq {
        ($left:expr, $right:expr) => {{
            let left = &{$left};
            let right = &{$right};

            if !(*left == *right) {
                panic!("assertion failed: `(left == right)`");
            }
        }};
        ($left:expr, $right:expr,) => {
            non_debug_assert_eq!($left, $right)
        };
        ($left:expr, $right:expr, $($args:tt)+) => {{
            let left = &{$left};
            let right = &{$right};

            if !(*left == *right) {
                panic!("assertion failed: `(left == right)`: {}", format_args!($($args)+));
            }
        }};
    }

    #[cfg(all(not(feature = "debug-impls"), feature = "eq-impls"))]
    macro_rules! any_debug_assert_eq {
        ($($args:tt)+) => {
            non_debug_assert_eq!($($args)+)
        }
    }

    #[cfg(all(feature = "debug-impls", feature = "eq-impls"))]
    macro_rules! any_debug_assert_eq {
        ($($args:tt)+) => {
            assert_eq!($($args)+)
        }
    }


    #[cfg(all(not(feature = "debug-impls"), feature = "eq-impls"))]
    macro_rules! non_debug_assert_ne {
        ($left:expr, $right:expr) => {{
            let left = &$left;
            let right = &$right;

            if !(*left != *right) {
                panic!("assertion failed: `(left != right)`");
            }
        }};
        ($left:expr, $right:expr,) => {
            non_debug_assert_eq!($left, $right)
        };
        ($left:expr, $right:expr, $($args:tt)+) => {{
            let left = &$left;
            let right = &$right;

            if !(*left != *right) {
                panic!("assertion failed: `(left != right)`: {}", format_args!($($args)+));
            }
        }};
    }

    #[cfg(all(not(feature = "debug-impls"), feature = "eq-impls"))]
    macro_rules! any_debug_assert_ne {
        ($($args:tt)+) => {
            non_debug_assert_ne!($($args)+)
        }
    }

    #[cfg(all(feature = "debug-impls", feature = "eq-impls"))]
    macro_rules! any_debug_assert_ne {
        ($($args:tt)+) => {
            assert_ne!($($args)+)
        }
    }


    #[cfg(feature = "eq-impls")]
    pub(crate) fn test_span_permutations<T, FN1, FN2, FT, FA1, FA2>(
        new1: FN1,
        new2: FN2,
        test_eq: FT,
        assert_when_eq: FA1,
        assert_when_ne: FA2,
    )
    where
        FN1: Fn(Span) -> T,
        FN2: Fn(Span) -> T,
        FT: Fn(&T, &T) -> bool,
        FA1: Fn(&T, &T),
        FA2: Fn(&T, &T),
    {
        // FIXME: Provide a mutable pool of spans (e.g. an infinite iterator or a random generator)
        // for cases when constructing a value of type `T` requires multiple spans.
        let spans = [
            Span::zeroed(),
            Span::new(1, 1),
            Span::new(283, 82374),
            Span::new(usize::max_value(),  usize::max_value()),
        ];

        for span1 in &spans {
            for span2 in &spans {
                let value1 = new1(*span1);
                let value2 = new2(*span2);

                if test_eq(&value1, &value2) {
                    assert_when_eq(&value1, &value2);
                } else {
                    assert_when_ne(&value1, &value2);
                }
            }
        }
    }

    #[cfg(all(feature = "eq-impls", feature = "hash-impls"))]
    pub(crate) fn get_hasher_state<T: Hash>(bi: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        bi.hash(&mut hasher);
        hasher.finish()
    }
}
