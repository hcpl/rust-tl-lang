use super::Item;


macro_attr_many! {
    /// A complete file of TL language source text.
    #[cfg_derive!(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct File {
        pub items: Vec<Item>,
    }
}

mod spanned {
    use super::*;
    use span::Span;
    use spanned::Spanned;
    use spanned::private::Sealed;

    impl Sealed for File {}

    impl Spanned for File {
        fn span(&self) -> Span {
            self.items.span()
        }
    }
}

#[cfg(feature = "parsing")]
mod parsing {
    use nom;

    use super::*;
    use cursor::Cursor;
    use synom::Synom;
    use synom::private::Sealed;

    impl Sealed for File {}

    impl Synom for File {
        named!(parse_cursor(Cursor) -> File, do_parse!(
            items: many0!(do_parse!(
                complete!(take_while!(char::is_whitespace)) >>
                item: tlsyn!(Item) >>
                opt!(call!(nom::line_ending)) >>
                (item)
            )) >>

            (File { items })
        ));
    }
}

#[cfg(feature = "printing")]
mod printing {
    use std::fmt;

    use super::*;
    use print::{Print, print_slice_with_separator};
    use print::private::Sealed;

    impl Sealed for File {}

    impl Print for File {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            print_slice_with_separator(&self.items, "\n", f).map(|_| ())
        }
    }
}
