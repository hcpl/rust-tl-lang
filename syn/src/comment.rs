use nom;

use synom::Synom;


/// A single-line or multiline comment.
#[derive(Debug)]
pub enum Comment {
    SingleLine(CommentSingleLine),
    MultiLine(CommentMultiLine),
}

/// A `//...` comment spanning a single line.
#[derive(Debug)]
pub struct CommentSingleLine {
    content: String,
}

/// A `/*...*/` comment spanning multiple lines.
#[derive(Debug)]
pub struct CommentMultiLine {
    content: String,
}


impl Synom for Comment {
    named!(parse_str(&str) -> Comment, alt_complete!(
        tlsyn!(CommentSingleLine) => { Comment::SingleLine }
        |
        tlsyn!(CommentMultiLine) => { Comment::MultiLine }
    ));
}

impl Synom for CommentSingleLine {
    named!(parse_str(&str) -> CommentSingleLine, do_parse!(
        tag!("//") >>
        content: call!(nom::not_line_ending) >>

        (CommentSingleLine {
            content: content.to_owned(),
        })
    ));
}

impl Synom for CommentMultiLine {
    named!(parse_str(&str) -> CommentMultiLine, do_parse!(
        tag!("/*") >>
        content: take_until!("*/") >>
        tag!("*/") >>

        (CommentMultiLine {
            content: content.to_owned(),
        })
    ));
}
