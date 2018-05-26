//! A region of source text.

use std::cmp;
#[cfg(stable_nonzero_types)]
use std::num::NonZeroUsize;


/// A region of source text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Span {
    begin: usize,
    end: usize,
}

impl Span {
    /// Create a new `Span` in zeroed state.
    pub fn zeroed() -> Span {
        Span { begin: 0, end: 0 }
    }

    /// Create a new `Span`, ensuring that the resulting `Span` is in either the
    /// zeroed or the normal state.
    pub fn new(begin: usize, end: usize) -> Span {
        let (begin, end) = if begin == 0 || end == 0 {
            (0, 0)
        } else if begin <= end {
            (begin, end)
        } else {
            (end, begin)
        };

        Span { begin, end }
    }

    /// Create a new `Span`, ensuring that the resulting `Span` is in the normal
    /// state.
    #[cfg(stable_nonzero_types)]
    pub fn new_nonzero(begin: NonZeroUsize, end: NonZeroUsize) -> Span {
        let (begin, end) = if begin <= end {
            (begin.get(), end.get())
        } else {
            (end.get(), begin.get())
        };

        Span { begin, end }
    }

    /// Create a new `Span` without checking.
    ///
    /// # Safety
    ///
    /// `begin` and `end` must be either both zero or both non-zero such as
    /// `begin <= end`.
    pub unsafe fn new_unchecked(begin: usize, end: usize) -> Span {
        Span { begin, end }
    }

    /// Return true if this `Span` is in zeroed state, and false otherwise.
    pub fn is_zeroed(&self) -> bool {
        self.assert_internal_consistency();
        self.begin == 0
    }

    /// Return a `Span` that encloses both `self` and `other`.
    ///
    /// If either of the spans is zeroed, the result is a zeroed `Span`.
    pub fn to(self, other: Span) -> Span {
        self.assert_internal_consistency();
        other.assert_internal_consistency();

        if self.is_zeroed() || other.is_zeroed() {
            Span::zeroed()
        } else {
            Span {
                begin: cmp::min(self.begin, other.begin),
                end: cmp::max(self.end, other.end),
            }
        }
    }

    /// Create a `Span` that encloses all spans in the given iterator.
    ///
    /// If the iterator is empty or if any span in this iterator is zeroed the
    /// result is a zeroed `Span`.
    pub fn union<I>(iter: I) -> Span
    where
        I: IntoIterator<Item = Span>,
    {
        let mut iter = iter.into_iter();

        match iter.next() {
            None => Span::zeroed(),
            Some(first) => iter.fold(first, |joined, other| joined.to(other)),
        }
    }

    /// Checks if this `Span` upholds the guarantee that it only has two states:
    /// * zeroed: both `begin` and `end` equal zero;
    /// * normal: both `begin` and `end` not equal zero and, additionally,
    ///   `begin <= end`.
    fn assert_internal_consistency(&self) {
        assert!(
            (self.begin == 0 && self.end == 0) ||
            (self.begin != 0 && self.end != 0 && self.begin <= self.end)
        );
    }
}
