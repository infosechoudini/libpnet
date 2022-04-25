



/// Packet Results - {I = Packet Bytes Input, O = Output Field, E = Error}}, 
pub type PResult<I, O, E = error::Error<I>> = Result<(I, O), Err<E>>;


pub trait Finish<I, O, E> {
    fn finish(self) -> Result<(I, O), E>;
}


impl<I, O, E> Finish<I, O, E> for IResult<I, O, E> {
    fn finish(self) -> Result<(I, O), E> {
      match self {
        Ok(res) => Ok(res),
        Err(Err::Error(e)) | Err(Err::Failure(e)) => Err(e),
        Err(Err::Incomplete(_)) => {
          panic!("Cannot call `finish()` on `Err(Err::Incomplete(_))`: this result means that the parser does not have enough data to decide, you should gather more data and try to reapply  the parser instead")
        }
      }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Err<E> {
  /// There was not enough data
  Incomplete(Needed),
  /// The parser had an error (recoverable)
  Error(E),
  /// The parser had an unrecoverable error: we got to the right
  /// branch and we know other branches won't work, so backtrack
  /// as fast as possible
  Failure(E),
}