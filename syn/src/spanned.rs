//! A trait that can provide the `Span` of the complete contents of a syntax tree node.

use span::Span;


pub trait Spanned {
    fn span(&self) -> Span;
}


impl<'a, T: Spanned + ?Sized> Spanned for &'a T {
    fn span(&self) -> Span {
        (**self).span()
    }
}

impl<T: Spanned + ?Sized> Spanned for Box<T> {
    fn span(&self) -> Span {
        (**self).span()
    }
}

// FIXME: Use correct initial spanning
impl<T: Spanned> Spanned for Option<T> {
    fn span(&self) -> Span {
        match *self {
            Some(ref t) => t.span(),
            None => Span::empty(),
        }
    }
}

impl<'a, T: Spanned> Spanned for &'a [T] {
    fn span(&self) -> Span {
        Span::union(self.iter().map(Spanned::span))
    }
}

impl<T: Spanned> Spanned for Vec<T> {
    fn span(&self) -> Span {
        self.as_slice().span()
    }
}
