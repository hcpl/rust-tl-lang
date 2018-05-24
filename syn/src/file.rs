use std::fmt;

use nom;

use super::Item;
use span::Span;
use spanned::Spanned;
use synom::Synom;


#[derive(Debug)]
pub struct File {
    pub items: Vec<Item>,
}

impl Synom for File {
    named!(parse_str(&str) -> File, do_parse!(
        items: many0!(do_parse!(
            complete!(take_while!(char::is_whitespace)) >>
            item: tlsyn!(Item) >>
            call!(nom::line_ending) >>
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

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iter_items = self.items.iter();

        match iter_items.next() {
            None => (),
            Some(ref first) => {
                fmt::Display::fmt(first, f)?;

                for other in iter_items {
                    fmt::Display::fmt("\n", f)?;
                    fmt::Display::fmt(other, f)?;
                }
            },
        }

        Ok(())
    }
}
