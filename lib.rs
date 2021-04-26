//! Print all items from an iterator.
//!
//! **Example:** Print all items from an array slice
//!
//! ```no_run
//! use fmt_iter::FmtIter;
//! println!("{}", FmtIter::from(&[0, 12, 345]));
//! ```
//!
//! _Expected Output:_
//!
//! ```txt
//! 012345
//! ```
//!
//! **Example:** Repeat a certain character multiple times
//!
//! ```no_run
//! use fmt_iter::repeat;
//! println!("{}", repeat('x', 5));
//! ```
//!
//! _Expected Output:_
//!
//! ```txt
//! xxxxx
//! ```

#![no_std]
use core::fmt::{Display, Error, Formatter};
use derive_more::{AsMut, AsRef, Deref, DerefMut, From};
use itertools::repeat_n;

/// Wrap around an [`Iterator`] to print all items.
///
/// **Example:** From an [`Iterator`]
///
/// ```
/// # use fmt_iter::FmtIter;
/// let iter = [0, 1, 2, 3].iter();
/// let fmt_iter = FmtIter::from(iter);
/// assert_eq!(fmt_iter.to_string(), "0123");
/// ```
///
/// **Example:** From an array slice
///
/// ```
/// # use fmt_iter::FmtIter;
/// let fmt_iter = FmtIter::from(&[0, 1, 2, 3] as &[_]);
/// assert_eq!(fmt_iter.to_string(), "0123");
/// ```
///
/// **Example:** From an array reference
///
/// ```
/// # use fmt_iter::FmtIter;
/// let fmt_iter = FmtIter::from(&[0, 1, 2, 3]);
/// assert_eq!(fmt_iter.to_string(), "0123");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsMut, AsRef, Deref, DerefMut, From)]
pub struct FmtIter<Inner>(Inner)
where
    Inner: Iterator + Clone,
    <Inner as Iterator>::Item: Display;

impl<'a, Item> From<&'a [Item]> for FmtIter<core::slice::Iter<'a, Item>>
where
    Item: Display + Clone,
{
    fn from(inner: &'a [Item]) -> Self {
        FmtIter(inner.iter())
    }
}

impl<'a, Item, const LEN: usize> From<&'a [Item; LEN]> for FmtIter<core::slice::Iter<'a, Item>>
where
    Item: Display + Clone,
{
    fn from(inner: &'a [Item; LEN]) -> Self {
        FmtIter::from(inner as &[Item])
    }
}

impl<Inner> Display for FmtIter<Inner>
where
    Inner: Iterator + Clone,
    Inner::Item: Display,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        for item in self.clone() {
            write!(formatter, "{}", item)?;
        }
        Ok(())
    }
}

impl<Inner> Iterator for FmtIter<Inner>
where
    Inner: Iterator + Clone,
    Inner::Item: Display,
{
    type Item = Inner::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<Inner> ExactSizeIterator for FmtIter<Inner>
where
    Inner: ExactSizeIterator + Clone,
    Inner::Item: Display,
{
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// Print a certain value multiple times.
///
/// **Example:**
///
/// ```
/// # use fmt_iter::repeat;
/// let fmt_iter = repeat(123, 5);
/// assert_eq!(fmt_iter.to_string(), "123123123123123");
/// ```
pub fn repeat<Value: Display + Clone>(
    value: Value,
    times: usize,
) -> FmtIter<impl Iterator<Item = Value> + Clone> {
    FmtIter::from(repeat_n(value, times))
}
