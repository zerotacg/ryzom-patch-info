use std;
use std::fmt::{self, Display};

use serde::{de, ser};

#[derive(Debug)]
pub enum Error {
    InvalidFormat,
    Message(String),
    NoMoreTokens,
    NoMoreArgs,
    ExpectedString,
    ExpectedUintToken,
    ExpectedSintToken,
    ExpectedEnum,
    ExpectedTokenWithName(String),
    ExpectedBeginToken,
    ExpectedEndToken,
}

pub type Result<T> = std::result::Result<T, Error>;

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::InvalidFormat => formatter.write_str("invalid format"),
            Error::NoMoreTokens => formatter.write_str("no more tokens"),
            Error::NoMoreArgs => formatter.write_str("no more args"),
            Error::ExpectedString => formatter.write_str("expected string"),
            Error::ExpectedBeginToken => formatter.write_str("expected begin token"),
            Error::ExpectedEndToken => formatter.write_str("expected end token"),
            Error::ExpectedSintToken => formatter.write_str("expected sint token"),
            Error::ExpectedUintToken => formatter.write_str("expected uint token"),
            Error::ExpectedEnum => formatter.write_str("expected enum"),
            Error::ExpectedTokenWithName(name) => {
                write!(formatter, "expected token with name({})", name)
            }
        }
    }
}

impl std::error::Error for Error {}
