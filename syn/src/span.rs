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
    pub fn empty() -> Span {
        Span { begin: 0, end: 0 }
    }

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

    #[cfg(stable_nonzero_types)]
    pub fn new_nonzero(begin: NonZeroUsize, end: NonZeroUsize) -> Span {
        let (begin, end) = if begin <= end {
            (begin.get(), end.get())
        } else {
            (end.get(), begin.get())
        };

        Span { begin, end }
    }

    pub unsafe fn new_unchecked(begin: usize, end: usize) -> Span {
        Span { begin, end }
    }

    pub fn is_empty(&self) -> bool {
        self.assert_internal_consistency();
        self.begin == 0
    }

    pub fn to(self, other: Span) -> Span {
        self.assert_internal_consistency();
        other.assert_internal_consistency();

        Span {
            begin: cmp::min(self.begin, other.begin),
            end: cmp::max(self.end, other.end),
        }
    }

    pub fn union<I>(iter: I) -> Span
    where
        I: IntoIterator<Item = Span>,
    {
        let mut iter = iter.into_iter();

        match iter.next() {
            None => Span::empty(),
            Some(first) => iter.fold(first, |joined, other| joined.to(other)),
        }
    }

    fn assert_internal_consistency(&self) {
        assert!(
            (self.begin == 0 && self.end == 0) ||
            (self.begin != 0 && self.end != 0 && self.begin <= self.end)
        );
    }
}
