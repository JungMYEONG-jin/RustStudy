use std::{error, fmt};
use std::error::Error;
use std::fmt::{Formatter, Pointer};
use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum CustomError{
    EmptyVec,
    Parse(ParseIntError),
}

impl From<ParseIntError> for CustomError{
    fn from(value: ParseIntError) -> CustomError {
        CustomError::Parse(value)
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self{
            CustomError::EmptyVec =>
            write!(f, "please use a vector at least one element"),
            CustomError::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for CustomError{
    fn description(&self) -> &str {
        match *self{
            CustomError::EmptyVec => "empty vectors not allowed",
            CustomError::Parse(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            CustomError::EmptyVec => None,
            CustomError::Parse(ref e) => Some(e),
        }
    }
}