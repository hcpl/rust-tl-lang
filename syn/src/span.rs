use std::cmp;


/// A region of source text.
#[derive(Clone, Copy, Debug)]
pub struct Span {
    start: u32,
    end: u32,
}

impl Span {
    pub fn empty() -> Span {
        Span { start: 0, end: 0 }
    }

    pub fn new(start: u32, end: u32) -> Span {
        let (start, end) = if start == 0 || end == 0 {
            (0, 0)
        } else if start <= end {
            (start, end)
        } else {
            (end, start)
        };

        Span { start, end }
    }

    pub fn is_empty(&self) -> bool {
        self.assert_internal_consistency();
        self.start == 0
    }

    pub fn to(self, other: Span) -> Span {
        self.assert_internal_consistency();

        Span {
            start: cmp::min(self.start, other.start),
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
            (self.start == 0 && self.end == 0) ||
            (self.start != 0 && self.end != 0 && self.start <= self.end)
        );
    }
}
