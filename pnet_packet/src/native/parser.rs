
use std::error::Error;
use core::slice::Iter;
use core::iter::Copied;
use core::iter::Enumerate;

pub trait Parser {
    type T;
    fn parse(i: &[u8]) -> Result<Self::T, &dyn Error>;
}

  /// Abstract method to calculate the input length
pub trait InputLength {
    /// Calculates the input length, as indicated by its name,
    /// and the name of the trait itself
    fn input_len(&self) -> usize;
  }

impl<'a, T> InputLength for &'a [T] {
    #[inline]
    fn input_len(&self) -> usize {
        self.len()
    }
}  

/// Abstracts slicing operations
pub trait InputTake: Sized {
    /// Returns a slice of `count` bytes. panics if count > length
    fn take(&self, count: usize) -> Self;
    /// Split the stream at the `count` byte offset. panics if count > length
    fn take_split(&self, count: usize) -> (Self, Self);

    fn take_offsets(&self, offsets: usize, pos: usize) -> Self;

}

impl<'a> InputTake for &'a [u8] {
    #[inline]
    fn take(&self, count: usize) -> Self {
      &self[0..count]
    }

    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
      let (prefix, suffix) = self.split_at(count);
      (suffix, prefix)
    }

    #[inline]
    fn take_offsets(&self, offset: usize, pos: usize) -> Self{
        &self[pos..offset]
    }

}


pub trait InputTakeIter: Sized {
    /// Returns an iterator of `count` bytes. panics if count > length
    fn take_iter(&self, count: usize) -> Self;

}

impl<'a> InputTakeIter for &'a [u8] {
    #[inline]
    fn take_iter(&self, count: usize) -> Self {
      &self
    }
}
