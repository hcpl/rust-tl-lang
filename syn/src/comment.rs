use span::Span;
use token::{SlashAsterisk, SlashSlash};


/// A single-line or multiline comment.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub enum Comment {
    SingleLine(CommentSingleLine),
    MultiLine(CommentMultiLine),
}

/// A `//...` comment spanning a single line.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct CommentSingleLine {
    pub slash_slash_token: SlashSlash,
    pub content_span: Span,
    pub content: String,
}

/// A `/*...*/` comment spanning multiple lines.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct CommentMultiLine {
    pub slash_asterisk_token: SlashAsterisk,
    pub content_span: Span,
    pub content: String,
}


#[cfg(feature = "eq-impls")]
mod eq_impls {
    use super::*;

    impl Eq for CommentSingleLine {}
    impl Eq for CommentMultiLine {}

    impl PartialEq for CommentSingleLine {
        fn eq(&self, other: &CommentSingleLine) -> bool {
            self.content == other.content
        }
    }

    impl PartialEq for CommentMultiLine {
        fn eq(&self, other: &CommentMultiLine) -> bool {
            self.content == other.content
        }
    }
}


#[cfg(feature = "hash-impls")]
mod hash_impls {
    use std::hash::{Hash, Hasher};

    use super::*;

    impl Hash for CommentSingleLine {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.content.hash(state);
        }
    }

    impl Hash for CommentMultiLine {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.content.hash(state);
        }
    }
}


mod spanned {
    use super::*;
    use span::Span;
    use spanned::Spanned;
    use spanned::private::Sealed;

    impl Sealed for Comment {}
    impl Sealed for CommentSingleLine {}
    impl Sealed for CommentMultiLine {}

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
}


#[cfg(feature = "parsing")]
mod parsing {
    use nom;

    use super::*;
    use cursor::Cursor;
    use synom::Synom;
    use synom::private::Sealed;

    impl Sealed for Comment {}
    impl Sealed for CommentSingleLine {}
    impl Sealed for CommentMultiLine {}

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
}


#[cfg(feature = "printing")]
mod printing {
    use std::fmt;

    use super::*;
    use print::Print;
    use print::private::Sealed;

    impl Sealed for Comment {}
    impl Sealed for CommentSingleLine {}
    impl Sealed for CommentMultiLine {}

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
}
