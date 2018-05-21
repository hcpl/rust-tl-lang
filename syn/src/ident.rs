use span::Span;
use spanned::Spanned;
use synom::Synom;


/// An identifier: `channels`, `SendMessageAction`.
#[derive(Debug)]
pub struct Ident {
    span: Span,
    string: String,
}

impl Ident {
    pub fn new(span: Span, s: &str) -> Option<Ident> {
        if is_valid_ident(s) {
            Some(Ident {
                span,
                string: s.to_owned(),
            })
        } else {
            None
        }
    }

    pub fn as_str(&self) -> &str {
        &self.string
    }

    pub fn is_lowercase(&self) -> bool {
        match self.string.chars().next() {
            Some(c) => c.is_lowercase(),
            None    => unreachable!("There must be at least one char for any `Ident`"),
        }
    }

    pub fn is_uppercase(&self) -> bool {
        match self.string.chars().next() {
            Some(c) => c.is_uppercase(),
            None    => unreachable!("There must be at least one char for any `Ident`"),
        }
    }
}

impl Synom for Ident {
    named!(parse_str(&str) -> Ident, do_parse!(
        s: verify!(take_while!(is_ident_char), is_valid_ident) >>

        (Ident {
            string: s.to_owned(),
            span: Span::empty(),
        })
    ));
}

impl Spanned for Ident {
    fn span(&self) -> Span {
        self.span
    }
}


// Valid identifier chars must be:
// - uppercase Latin letter
// - lowercase Latin letter
// - decimal digit
// - underscore
fn is_ident_char(c: char) -> bool {
    is_ascii_alphanumeric(c) || c == '_'
}

// Valid identifiers must:
// - be non-empty
// - begin with either an uppercase or a lowercase Latin letter
// - contain an uppercase or a lowercase Latin letter, a decimal digit or an underscore in other
//   positions
fn is_valid_ident(s: &str) -> bool {
    let mut chars = s.chars();

    match chars.next() {
        Some(c) => is_ascii_alphabetic(c) && chars.all(is_ident_char),
        None    => false,
    }
}


// ========== COMPATIBILITY SHIMS ========== //

// If more efficient implementations are available, use them!
// Affected Rust versions: >= 1.24.0
#[cfg(char_stable_inherent_ascii_methods)]
mod ascii_shim {
    pub(super) fn is_ascii_alphanumeric(c: char) -> bool {
        char::is_ascii_alphanumeric(&c)
    }

    pub(super) fn is_ascii_alphabetic(c: char) -> bool {
        char::is_ascii_alphabetic(&c)
    }
}

// Otherwise, implement slower fallbacks.
// Affected Rust versions: <= 1.23.0
#[cfg(not(char_stable_inherent_ascii_methods))]
mod ascii_shim {
    use std::ascii::AsciiExt;

    pub(super) fn is_ascii_alphanumeric(c: char) -> bool {
        AsciiExt::is_ascii(&c) && char::is_alphanumeric(c)
    }

    pub(super) fn is_ascii_alphabetic(c: char) -> bool {
        AsciiExt::is_ascii(&c) && char::is_alphabetic(c)
    }
}

use self::ascii_shim::{is_ascii_alphanumeric, is_ascii_alphabetic};
