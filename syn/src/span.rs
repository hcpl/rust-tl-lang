//! A region of source text.
//!
//! A [`Span`] consists of two `usize` that point to the beginning and the end
//! of a chunk of text.
//!
//! Any valid `Span` exists in two states:
//!
//! - **`Zeroed`** — when both `begin` and `end` are zero.
//! - **`Normal`** — when both `begin` and `end` are not zero; additionally,
//!   `begin` must be less or equal to `end`.
//!
//! These states are expressed via the [`SpanState`] enum.
//!
//! The distinction between two states is useful to express the optional nature
//! of `Span`s without having to resort to the `Option` type — it currently
//! imposes a memory overhead of 1 machine word because `Span` is not subject to
//! non-zero type optimization. Since `Span` is meant to be copied around a lot,
//! we'd like to decrease the memory footprint as much as possible.

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
    ///
    /// If either of the arguments is zero, the result is a zeroed `Span`.
    ///
    /// Otherwise, the result is a normal `Span` with the values potentially
    /// swapped to fulfill the `begin <= end` requirement.
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
    ///
    /// Potentially swaps the values to satisfy the `begin <= end` constraint.
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
    /// `begin` and `end` must be either both zero or both non-zero such that
    /// `begin <= end`.
    pub unsafe fn new_unchecked(begin: usize, end: usize) -> Span {
        Span { begin, end }
    }

    /// Return the state of this `Span`.
    pub fn state(&self) -> SpanState {
        self.assert_internal_consistency();

        match self.begin {
            0 => SpanState::Zeroed,
            _ => SpanState::Normal,
        }
    }

    /// Return a `Span` that encloses both `self` and `other`.
    ///
    /// If one of the spans is zeroed, the result is another `Span`.
    ///
    /// If both spans are zeroed, the result is a zeroed `Span`.
    pub fn to(self, other: Span) -> Span {
        self.assert_internal_consistency();
        other.assert_internal_consistency();

        use self::SpanState::*;

        match (self.state(), other.state()) {
            (Normal, Normal) => {
                Span {
                    begin: cmp::min(self.begin, other.begin),
                    end: cmp::max(self.end, other.end),
                }
            },
            (Normal, Zeroed) => self,
            (Zeroed, Normal) => other,
            (Zeroed, Zeroed) => Span::zeroed(),
        }
    }

    /// Create a `Span` that encloses all spans in the given iterator.
    ///
    /// If the iterator is empty or if all spans in the iterator are zeroed the
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


/// The state of a `Span`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpanState {
    /// The state when both `begin` and `end` are zero.
    Zeroed,
    /// The state when both `begin` and `end` are non-zero, and also `begin <= end`.
    Normal,
}


#[cfg(test)]
mod tests {
    use std::mem;

    use super::Span;

    #[test]
    fn ensure_size_of_span_is_2_words() {
        assert_eq!(mem::size_of::<Span>(), 2 * mem::size_of::<usize>());
    }

    #[test]
    fn zeroed() {
        assert_eq!(Span::zeroed(), Span { begin: 0, end: 0 });
    }

    mod new {
        use super::super::Span;

        #[test]
        fn zero_zero() {
            assert_eq!(Span::new(0, 0), Span { begin: 0, end: 0 });
        }

        #[test]
        fn zero_nonzero() {
            assert_eq!(Span::new(0, 1), Span { begin: 0, end: 0 });
        }

        #[test]
        fn nonzero_zero() {
            assert_eq!(Span::new(usize::max_value(), 0), Span { begin: 0, end: 0 });
        }

        #[test]
        fn nonzero_nonzero_less() {
            assert_eq!(Span::new(1, 1000), Span { begin: 1, end: 1000 });
        }

        #[test]
        fn nonzero_nonzero_equal() {
            assert_eq!(Span::new(65_535, 65_535), Span { begin: 65_535, end: 65_535 });
        }

        #[test]
        fn nonzero_nonzero_greater() {
            assert_eq!(Span::new(6743, 1928), Span { begin: 1928, end: 6743 });
        }
    }

    mod to {
        use super::super::Span;

        #[test]
        fn zero_zero() {
            let span1 = Span::zeroed();
            let span2 = Span::zeroed();
            assert_eq!(span1.to(span2), Span { begin: 0, end: 0 });
        }

        #[test]
        fn zero_nonzero() {
            let span1 = Span::zeroed();
            let span2 = Span::new(98, 34);
            assert_eq!(span1.to(span2), Span { begin: 34, end: 98 });
        }

        #[test]
        fn nonzero_zero() {
            let span1 = Span::new(111, 222);
            let span2 = Span::zeroed();
            assert_eq!(span1.to(span2), Span { begin: 111, end: 222 });
        }

        #[test]
        fn nonzero_nonzero_disjoint() {
            let span1 = Span::new(10, 20);
            let span2 = Span::new(35, 78);
            assert_eq!(span1.to(span2), Span { begin: 10, end: 78 });
        }

        #[test]
        fn nonzero_nonzero_at_junction_no_intersection() {
            let span1 = Span::new(49, 37);
            let span2 = Span::new(24, 37);
            assert_eq!(span1.to(span2), Span { begin: 24, end: 49 });
        }

        #[test]
        fn nonzero_nonzero_at_junction_with_intersection() {
            let span1 = Span::new(948, 647);
            let span2 = Span::new(732, 948);
            assert_eq!(span1.to(span2), Span { begin: 647, end: 948 });
        }

        #[test]
        fn nonzero_nonzero_intersection_non_subset() {
            let span1 = Span::new(168, 9);
            let span2 = Span::new(75, 322);
            assert_eq!(span1.to(span2), Span { begin: 9, end: 322 });
        }

        #[test]
        fn nonzero_nonzero_intersection_subset() {
            let span1 = Span::new(1, 100);
            let span2 = Span::new(5, 50);
            assert_eq!(span1.to(span2), Span { begin: 1, end: 100 });
        }
    }

    mod assert_internal_consistency {
        use super::super::Span;

        #[test]
        fn begin_zero_end_zero() {
            let span = unsafe { Span::new_unchecked(0, 0) };
            span.assert_internal_consistency();
        }

        #[test]
        fn begin_nonzero_end_nonzero_less() {
            let span = unsafe { Span::new_unchecked(30_000, 45_000) };
            span.assert_internal_consistency();
        }

        #[test]
        fn begin_nonzero_end_nonzero_equal() {
            let span = unsafe { Span::new_unchecked(27_871, 27_871) };
            span.assert_internal_consistency();
        }

        #[test]
        #[should_panic]
        fn begin_zero_end_nonzero() {
            let span = unsafe { Span::new_unchecked(0, 42) };
            span.assert_internal_consistency();
        }

        #[test]
        #[should_panic]
        fn begin_nonzero_end_zero() {
            let span = unsafe { Span::new_unchecked(2018, 0) };
            span.assert_internal_consistency();
        }

        #[test]
        #[should_panic]
        fn begin_nonzero_end_nonzero_greater() {
            let span = unsafe { Span::new_unchecked(10, 5) };
            span.assert_internal_consistency();
        }
    }
}
