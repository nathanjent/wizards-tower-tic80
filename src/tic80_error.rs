use std::error::Error;
use std::ffi::NulError;
use std::fmt::Display;
use std::num::TryFromIntError;

#[derive(Debug)]
pub enum Tic80Error {
    TryFromIntError(TryFromIntError),
    NulCStringError(NulError),
}

impl Error for Tic80Error {}

impl From<NulError> for Tic80Error {
    fn from(e: NulError) -> Self {
        Tic80Error::NulCStringError(e)
    }
}

impl From<TryFromIntError> for Tic80Error {
    fn from(e: TryFromIntError) -> Self {
        Tic80Error::TryFromIntError(e)
    }
}

impl Display for Tic80Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tic80Error::TryFromIntError(e) => write!(f, "{}", e),
            Tic80Error::NulCStringError(e) => write!(f, "{}", e),
        }
    }
}
