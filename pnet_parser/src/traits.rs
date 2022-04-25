


/// Method to calculate the input length of a packet
pub trait InputLength {
    fn input_len(&self) -> usize;
}

impl<'p, T> InputLength for &'p [T] {
    #[inline]
    fn input_len(&self) -> usize {
        self.len()
    }
}

// Implementation for InputLength for slices
impl<'p> InputLength for (&'p [u8], usize) {
    #[inline]
    fn input_len(&self) -> usize {
        self.0.len() * 8 - self.1
    }
}

/// Allows types to be viewed as bytes
pub trait AsBytes {
    /// Takes input and casts it to a slice of bytes
    fn as_bytes(&self) -> &[u8];
}

impl<'p> AsBytes for &'p [u8] {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        *self
    }
}

impl AsBytes for [u8] {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        self
    }
}  

/// Abstracts common iteration operations on the input type
pub trait InputIter {
    /// The current input type is a sequence of that `Item` type.
    ///
    /// Example: `u8` for `&[u8]` and `u16` for `&[u16]`
    type Item;
    /// An iterator over the input type, producing the item and its position
    /// for use with [Slice]. If we're iterating over `&str`, the position
    /// corresponds to the byte index of the character
    type Iter: Iterator<Item = (usize, Self::Item)>;
  
    /// An iterator over the input type, producing the item
    type IterElem: Iterator<Item = Self::Item>;
  
    /// Returns an iterator over the elements and their byte offsets
    fn iter_indices(&self) -> Self::Iter;
    /// Returns an iterator over the elements
    fn iter_elements(&self) -> Self::IterElem;
    /// Finds the byte position of the element
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
      P: Fn(Self::Item) -> bool;
    /// Get the byte offset from the element's position in the stream
    fn slice_index(&self, count: usize) -> Result<usize, Needed>;
}

/// Abstracts slicing operations
pub trait InputTake: Sized {
    /// Returns a slice of `count` bytes. panics if count > length
    fn take(&self, count: usize) -> Self;
    /// Split the stream at the `count` byte offset. panics if count > length
    fn take_split(&self, count: usize) -> (Self, Self);
}
  
 impl<'a> InputIter for &'a [u8] {
    type Item = u8;
    type Iter = Enumerate<Self::IterElem>;
    type IterElem = Copied<Iter<'a, u8>>;
  
    #[inline]
    fn iter_indices(&self) -> Self::Iter {
      self.iter_elements().enumerate()
    }
    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
      self.iter().copied()
    }
    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
      P: Fn(Self::Item) -> bool,
    {
      self.iter().position(|b| predicate(*b))
    }
    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
      if self.len() >= count {
        Ok(count)
      } else {
        Err(Needed::new(count - self.len()))
      }
    }
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

}

pub trait InputTakeAtPosition: Sized {
    /// The current input type is a sequence of that `Item` type.
    ///
    /// Example: `u8` for `&[u8]` or `char` for `&str`
    type Item;
  
    /// Looks for the first element of the input type for which the condition returns true,
    /// and returns the input up to this position.
    ///
    /// *streaming version*: If no element is found matching the condition, this will return `Incomplete`
    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
    where
      P: Fn(Self::Item) -> bool;
  
    /// Looks for the first element of the input type for which the condition returns true
    /// and returns the input up to this position.
    ///
    /// Fails if the produced slice is empty.
    ///
    /// *streaming version*: If no element is found matching the condition, this will return `Incomplete`
    fn split_at_position1<P, E: ParseError<Self>>(
      &self,
      predicate: P,
      e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
      P: Fn(Self::Item) -> bool;
}

impl<'p> InputTakeAtPosition for &'p [u8] {
    type Item = u8;
  
    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
    where
      P: Fn(Self::Item) -> bool,
    {
      match self.iter().position(|c| predicate(*c)) {
        Some(i) => Ok(self.take_split(i)),
        None => Err(Err::Incomplete(Needed::new(1))),
      }
    }
  
    fn split_at_position1<P, E: ParseError<Self>>(
      &self,
      predicate: P,
      e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
      P: Fn(Self::Item) -> bool,
    {
      match self.iter().position(|c| predicate(*c)) {
        Some(0) => Err(Err::Error(E::from_error_kind(self, e))),
        Some(i) => Ok(self.take_split(i)),
        None => Err(Err::Incomplete(Needed::new(1))),
      }
    }
}

/// Look for a token in self
pub trait FindToken<T> {
    /// Returns true if self contains the token
    fn find_token(&self, token: T) -> bool;
  }
  
 impl<'a> FindToken<u8> for &'a [u8] {
    fn find_token(&self, token: u8) -> bool {
      memchr::memchr(token, self).is_some()
    }
 }

 impl<'a, 'b> FindToken<&'a u8> for &'b [u8] {
    fn find_token(&self, token: &u8) -> bool {
      self.find_token(*token)
    }
}

/// Slicing operations using ranges.
///
/// This trait is loosely based on
/// `Index`, but can actually return
/// something else than a `&[T]` or `&str`
pub trait Slice<R> {
    /// Slices self according to the range argument
    fn slice(&self, range: R) -> Self;
  }
  
  macro_rules! impl_fn_slice {
    ( $ty:ty ) => {
      fn slice(&self, range: $ty) -> Self {
        &self[range]
      }
    };
  }
  
 macro_rules! slice_range_impl {
    ( [ $for_type:ident ], $ty:ty ) => {
      impl<'a, $for_type> Slice<$ty> for &'a [$for_type] {
        impl_fn_slice!($ty);
      }
    };
    ( $for_type:ty, $ty:ty ) => {
      impl<'a> Slice<$ty> for &'a $for_type {
        impl_fn_slice!($ty);
      }
    };
}


macro_rules! slice_ranges_impl {
    ( [ $for_type:ident ] ) => {
      slice_range_impl! {[$for_type], Range<usize>}
      slice_range_impl! {[$for_type], RangeTo<usize>}
      slice_range_impl! {[$for_type], RangeFrom<usize>}
      slice_range_impl! {[$for_type], RangeFull}
    };
    ( $for_type:ty ) => {
      slice_range_impl! {$for_type, Range<usize>}
      slice_range_impl! {$for_type, RangeTo<usize>}
      slice_range_impl! {$for_type, RangeFrom<usize>}
      slice_range_impl! {$for_type, RangeFull}
    };
  }
  
slice_ranges_impl! {[T]}