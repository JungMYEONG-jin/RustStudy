use std::fmt;
use std::fmt::{Formatter, Pointer};
use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, CustomError>;

#[derive(Debug)]
pub enum CustomError{
    EmptyVec,
    Parse(ParseIntError),
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

