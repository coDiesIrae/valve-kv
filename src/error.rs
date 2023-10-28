use core::fmt;
use std::{error, fmt::Display, string::FromUtf8Error};

use serde::{de, ser};

use crate::parser::Rule;

#[derive(Debug)]
pub enum Error {
    ReadFileError(std::io::Error),
    ReadUtf8Error(FromUtf8Error),
    ParseKeyValueError(Box<pest::error::Error<Rule>>),
    ExpectedValueError,
    ExpectedUnitError,
    ExpectedCharError,
    ExpectedSectionError,
    ParseBoolError,
    SectionIsNotSequence,

    Custom(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}
