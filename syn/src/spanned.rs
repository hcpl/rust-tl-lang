//! A trait that can provide the `Span` of the complete contents of a syntax
//! tree node.

use span::Span;


/// A trait that can provide the `Span` of the complete contents of a syntax
/// tree node.
pub trait Spanned {
    /// Return a `Span` covering the complete contents of this syntax tree node,
    /// or [`Span::zeroed()`] if this node is empty.
    fn span(&self) -> Span;
}


impl<'a, T: Spanned + ?Sized> Spanned for &'a T {
    /// Return a `Span` of the value behind this reference.
    fn span(&self) -> Span {
        (**self).span()
    }
}

impl<T: Spanned + ?Sized> Spanned for Box<T> {
    /// Return a `Span` of the value in this box.
    fn span(&self) -> Span {
        (**self).span()
    }
}

impl<T: Spanned> Spanned for Option<T> {
    /// Return a `Span` of the contained value (if any), or [`Span::zeroed()`]
    /// (if not).
    fn span(&self) -> Span {
        match *self {
            Some(ref t) => t.span(),
            None => Span::zeroed(),
        }
    }
}

impl<'a, T: Spanned> Spanned for &'a [T] {
    /// Return a [`Span::union()`] of spans in this slice.
    fn span(&self) -> Span {
        Span::union(self.iter().map(Spanned::span))
    }
}

impl<T: Spanned> Spanned for Vec<T> {
    /// Return a [`Span::union()`] of spans in this vector.
    fn span(&self) -> Span {
        self.as_slice().span()
    }
}
