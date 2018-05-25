#[cfg(feature = "printing")]
use std::fmt;

#[cfg(feature = "parsing")]
use nom;

use super::Item;
#[cfg(feature = "parsing")]
use cursor::Cursor;
#[cfg(feature = "printing")]
use print::{Print, print_slice_with_separator};
use span::Span;
use spanned::Spanned;
#[cfg(feature = "parsing")]
use synom::Synom;


#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct File {
    pub items: Vec<Item>,
}

#[cfg(feature = "parsing")]
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

impl Spanned for File {
    fn span(&self) -> Span {
        self.items.span()
    }
}

#[cfg(feature = "printing")]
impl Print for File {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        print_slice_with_separator(&self.items, "\n", f).map(|_| ())
    }
}
