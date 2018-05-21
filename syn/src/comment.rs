use nom;

use span::Span;
use spanned::Spanned;
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
    span: Span,
    content: String,
}

/// A `/*...*/` comment spanning multiple lines.
#[derive(Debug)]
pub struct CommentMultiLine {
    span: Span,
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
            span: Span::empty(),
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
            span: Span::empty(),
            content: content.to_owned(),
        })
    ));
}


impl Spanned for Comment {
    fn span(&self) -> Span {
        match *self {
            Comment::SingleLine(ref t) => t.span(),
            Comment::MultiLine(ref t) => t.span(),
        }
    }
}

impl Spanned for CommentSingleLine {
    fn span(&self) -> Span {
        self.span
    }
}

impl Spanned for CommentMultiLine {
    fn span(&self) -> Span {
        self.span
    }
}
