extern crate tl_lang_syn;
#[macro_use]
extern crate pretty_assertions;


use tl_lang_syn::print::Print;


macro_rules! roundtrip_tests {
    ($($test_name:ident => $file_name:expr;)+) => {
        $(
            #[test]
            fn $test_name() {
                let original_string = include_str!($file_name);

                // Do a syntax tree-based roundtrip instead of a string-based one
                // because the string->tree->string conversion is lossy.
                let parsed_tree = tl_lang_syn::parse_file(original_string).unwrap();
                let generated_string = parsed_tree.display_wrapper().to_string();
                let parsed_tree2 = tl_lang_syn::parse_file(&generated_string).unwrap();

                assert_eq!(parsed_tree, parsed_tree2);
            }
        )+
    };
}

roundtrip_tests! {
    roundtrip_small => "small.tl";
}
