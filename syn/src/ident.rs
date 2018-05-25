#[cfg(feature = "printing")]
use std::fmt;

#[cfg(feature = "parsing")]
use cursor::Cursor;
#[cfg(feature = "printing")]
use print::Print;
use span::Span;
use spanned::Spanned;
#[cfg(feature = "parsing")]
use synom::Synom;


/// An identifier: `channels`, `SendMessageAction`.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
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

    pub unsafe fn new_unchecked(span: Span, s: &str) -> Ident {
        Ident {
            span,
            string: s.to_owned(),
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

#[cfg(feature = "eq-impls")]
impl Eq for Ident {}

#[cfg(feature = "eq-impls")]
impl PartialEq for Ident {
    fn eq(&self, other: &Ident) -> bool {
        self.string == other.string
    }
}

#[cfg(feature = "parsing")]
impl Synom for Ident {
    named!(parse_cursor(Cursor) -> Ident, do_parse!(
        ident_str_cursor: take_while!(is_ident_char) >>
        ident_str: verify!(value!(ident_str_cursor.to_str()), is_valid_ident) >>

        (Ident {
            span: ident_str_cursor.span(),
            string: ident_str.to_owned(),
        })
    ));
}

impl Spanned for Ident {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg(feature = "printing")]
impl Print for Ident {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.string, f)
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
