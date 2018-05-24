use std::fmt;

use nom;

use cursor::Cursor;
use print::Print;
use span::Span;
use spanned::Spanned;
use synom::Synom;
use token::{SlashAsterisk, SlashSlash};


/// A single-line or multiline comment.
#[derive(Debug)]
pub enum Comment {
    SingleLine(CommentSingleLine),
    MultiLine(CommentMultiLine),
}

/// A `//...` comment spanning a single line.
#[derive(Debug)]
pub struct CommentSingleLine {
    pub slash_slash_token: SlashSlash,
    pub content_span: Span,
    pub content: String,
}

/// A `/*...*/` comment spanning multiple lines.
#[derive(Debug)]
pub struct CommentMultiLine {
    pub slash_asterisk_token: SlashAsterisk,
    pub content_span: Span,
    pub content: String,
}


impl Synom for Comment {
    named!(parse_cursor(Cursor) -> Comment, alt_complete!(
        tlsyn!(CommentSingleLine) => { Comment::SingleLine }
        |
        tlsyn!(CommentMultiLine) => { Comment::MultiLine }
    ));
}

impl Synom for CommentSingleLine {
    named!(parse_cursor(Cursor) -> CommentSingleLine, do_parse!(
        slash_slash_token: tlsyn!(SlashSlash) >>
        content: call!(nom::not_line_ending) >>

        (CommentSingleLine {
            slash_slash_token,
            content_span: content.span(),
            content: content.to_str().to_owned(),
        })
    ));
}

impl Synom for CommentMultiLine {
    named!(parse_cursor(Cursor) -> CommentMultiLine, do_parse!(
        content: slash_asterisks!() >>

        (CommentMultiLine {
            slash_asterisk_token: content.0,
            content_span: content.1.span(),
            content: content.1.to_str().to_owned(),
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
        self.slash_slash_token.span()
            .to(self.content_span)
    }
}

impl Spanned for CommentMultiLine {
    fn span(&self) -> Span {
        self.slash_asterisk_token.span()
            .to(self.content_span)
    }
}


impl Print for Comment {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Comment::SingleLine(ref t) => t.print(f),
            Comment::MultiLine(ref t) => t.print(f),
        }
    }
}

impl Print for CommentSingleLine {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("//")?;
        fmt::Display::fmt(&self.content, f)?;

        Ok(())
    }
}

impl Print for CommentMultiLine {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        SlashAsterisk::print(f, |f| {
            fmt::Display::fmt(&self.content, f)
        })
    }
}
