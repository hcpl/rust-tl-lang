use std::ops;
use std::str;

use nom::{self, Offset};

use span::Span;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cursor<'a> {
    offset: usize,
    remaining: &'a str,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor { offset: 1, remaining: input }
    }

    pub fn span(self) -> Span {
        let start = self.offset as u32;
        let end = (self.offset + self.remaining.len()) as u32;

        Span::new(start, end)
    }

    pub fn offset(self) -> usize {
        self.offset
    }

    pub fn to_str(self) -> &'a str {
        self.remaining
    }

    // ===== Common implementation details ===== //

    fn impl_take_split(&self, count: usize) -> (Cursor<'a>, Cursor<'a>) {
        let (before, after) = self.remaining.split_at(count);

        let cursor_before = Cursor {
            offset: self.offset,
            remaining: before,
        };

        let cursor_after = Cursor {
            offset: self.offset + count,
            remaining: after,
        };

        (cursor_after, cursor_before)  // Notice the swapped order here
    }

    fn split_at_position_check_count<P, C>(
        &self,
        predicate: P,
        error_kind: Option<nom::ErrorKind<u32>>,
        check_count: C,
    ) -> nom::IResult<Cursor<'a>, Cursor<'a>, u32>
    where
        P: Fn(char) -> bool,
        C: Fn(usize, nom::ErrorKind<u32>) -> Result<(), nom::Err<Cursor<'a>, u32>>,
    {
        let match_pos = match self.remaining.char_indices().find(|&(_, c)| predicate(c)) {
            Some((i, _)) => i,
            None => return Err(nom::Err::Incomplete(nom::Needed::Size(1))),
        };

        if let Some(error_kind) = error_kind {
            check_count(match_pos, error_kind)?;
        }

        Ok(self.impl_take_split(match_pos))
    }
}


impl<'a> nom::AtEof for Cursor<'a> {
    fn at_eof(&self) -> bool {
        self.remaining.is_empty()
    }
}

impl<'a, 'b> nom::Compare<&'b str> for Cursor<'a> {
    fn compare(&self, t: &'b str) -> nom::CompareResult {
        self.remaining.compare(t)
    }

    fn compare_no_case(&self, t: &'b str) -> nom::CompareResult {
        self.remaining.compare_no_case(t)
    }
}

impl<'a, 'b> nom::FindSubstring<&'b str> for Cursor<'a> {
    fn find_substring(&self, substr: &'b str) -> Option<usize> {
        self.remaining.find_substring(substr)
    }
}

impl<'a> nom::InputIter for Cursor<'a> {
    type Item = char;
    type RawItem = char;
    type Iter = str::CharIndices<'a>;
    type IterElem = str::Chars<'a>;

    fn iter_indices(&self) -> Self::Iter {
        self.remaining.iter_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.remaining.iter_elements()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::RawItem) -> bool,
    {
        self.remaining.position(predicate)
    }

    fn slice_index(&self, count: usize) -> Option<usize> {
        self.remaining.slice_index(count)
    }
}

impl<'a> nom::InputLength for Cursor<'a> {
    fn input_len(&self) -> usize {
        self.remaining.input_len()
    }
}

impl<'a> nom::InputTake for Cursor<'a> {
    fn take(&self, count: usize) -> Self {
        Cursor {
            offset: self.offset,
            remaining: &self.remaining[0..count],
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        self.impl_take_split(count)
    }
}

impl<'a> nom::InputTakeAtPosition for Cursor<'a> {
    type Item = char;

    fn split_at_position<P>(&self, predicate: P) -> nom::IResult<Self, Self, u32>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.split_at_position_check_count(predicate, None, |_, _| Ok(()))
    }

    fn split_at_position1<P>(&self, predicate: P, e: nom::ErrorKind<u32>) -> nom::IResult<Self, Self, u32>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.split_at_position_check_count(predicate, Some(e), |match_pos, e| {
            match match_pos {
                0 => Err(nom::Err::Error(nom::Context::Code(*self, e))),
                _ => Ok(()),
            }
        })
    }
}

macro_rules! slice_range_impl {
    ($range_type:ty) => {
        impl<'a> nom::Slice<$range_type> for Cursor<'a> {
            fn slice(&self, range: $range_type) -> Cursor<'a> {
                let sliced_remaining = self.remaining.slice(range);
                let slice_offset = self.offset + self.remaining.offset(&sliced_remaining);

                Cursor {
                    offset: slice_offset,
                    remaining: sliced_remaining,
                }
            }
        }
    };
}

slice_range_impl!(ops::Range<usize>);
slice_range_impl!(ops::RangeTo<usize>);
slice_range_impl!(ops::RangeFrom<usize>);
//slice_range_impl!(ops::RangeFull);
