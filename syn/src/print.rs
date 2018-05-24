use std::fmt;


pub trait Print {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result;

    fn display_wrapper<'a>(&'a self) -> DisplayWrapper<'a, Self>  {
        DisplayWrapper(self)
    }
}

#[derive(Debug)]
pub struct DisplayWrapper<'a, T: ?Sized + 'a>(&'a T);

impl<'a, T: Print + ?Sized + 'a> fmt::Display for DisplayWrapper<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.print(f)
    }
}


impl<'a, T: Print> Print for &'a T {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (**self).print(f)
    }
}

impl<T: Print> Print for Box<T> {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (**self).print(f)
    }
}

impl<T: Print> Print for Option<T> {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Some(ref t) => t.print(f),
            None => Ok(()),
        }
    }
}

pub(crate) fn print_slice_with_separator<T>(
    slice: &[T],
    separator: &str,
    f: &mut fmt::Formatter,
) -> Result<Option<()>, fmt::Error>
where
    T: Print,
{
    match slice.split_first() {
        None => Ok(None),
        Some((first, rest)) => {
            first.print(f)?;

            for other in rest {
                f.write_str(separator)?;
                other.print(f)?;
            }

            Ok(Some(()))
        },
    }
}
