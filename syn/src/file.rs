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
