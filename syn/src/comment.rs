use std::fmt;

use nom;

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
    named!(parse_str(&str) -> Comment, alt_complete!(
        tlsyn!(CommentSingleLine) => { Comment::SingleLine }
        |
        tlsyn!(CommentMultiLine) => { Comment::MultiLine }
    ));
}

impl Synom for CommentSingleLine {
    named!(parse_str(&str) -> CommentSingleLine, do_parse!(
        slash_slash_token: tlsyn!(SlashSlash) >>
        content: call!(nom::not_line_ending) >>

        (CommentSingleLine {
            slash_slash_token,
            content_span: Span::empty(),
            content: content.to_owned(),
        })
    ));
}

impl Synom for CommentMultiLine {
    named!(parse_str(&str) -> CommentMultiLine, do_parse!(
        content: slash_asterisks!() >>

        (CommentMultiLine {
            slash_asterisk_token: content.0,
            content_span: Span::empty(),
            content: content.1.to_owned(),
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


impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Comment::SingleLine(ref t) => fmt::Display::fmt(t, f),
            Comment::MultiLine(ref t) => fmt::Display::fmt(t, f),
        }
    }
}

impl fmt::Display for CommentSingleLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt("//", f)?;
        fmt::Display::fmt(&self.content, f)?;

        Ok(())
    }
}

impl fmt::Display for CommentMultiLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        SlashAsterisk::print(f, |f| {
            fmt::Display::fmt(&self.content, f)
        })
    }
}
