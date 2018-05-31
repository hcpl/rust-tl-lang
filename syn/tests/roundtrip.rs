extern crate tl_lang_syn;
#[macro_use]
extern crate pretty_assertions;


use tl_lang_syn::print::Print;


#[cfg(not(feature = "debug-impls"))]
macro_rules! non_debug_assert_eq {
    ($left:expr, $right:expr) => {{
        let left = &{$left};
        let right = &{$right};

        if !(*left == *right) {
            panic!("assertion failed: `(left == right)`");
        }
    }};
    ($left:expr, $right:expr) => {
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

#[cfg(not(feature = "debug-impls"))]
macro_rules! any_debug_assert_eq {
    ($($args:tt)+) => {
        non_debug_assert_eq!($($args)+)
    }
}

#[cfg(feature = "debug-impls")]
macro_rules! any_debug_assert_eq {
    ($($args:tt)+) => {
        assert_eq!($($args)+)
    }
}


macro_rules! roundtrip_tests {
    ($($test_name:ident => $file_name:expr;)+) => {
        $(
            #[test]
            fn $test_name() {
                let original_string = include_str!($file_name);

                // Do a syntax tree-based roundtrip instead of a string-based one
                // because the string->tree->string conversion is lossy.
                let parsed_tree = tl_lang_syn::parse_file_str(original_string).unwrap();
                let generated_string = parsed_tree.display_wrapper().to_string();
                let parsed_tree2 = tl_lang_syn::parse_file_str(&generated_string).unwrap();

                any_debug_assert_eq!(parsed_tree, parsed_tree2);
            }
        )+
    };
}

roundtrip_tests! {
    roundtrip_small => "small.tl";
}
