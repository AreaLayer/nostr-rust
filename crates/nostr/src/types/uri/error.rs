use core::fmt;

use super::{InvalidUri, InvalidUriParts};

#[derive(Debug)]
pub enum Error {
    Uri(InvalidUri),
    UriParts(InvalidUriParts),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uri(e) => write!(f, "Uri: {e}"),
            Self::UriParts(e) => write!(f, "Uri parts: {e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<InvalidUri> for Error {
    fn from(e: InvalidUri) -> Self {
        Self::Uri(e)
    }
}

impl From<InvalidUriParts> for Error {
    fn from(e: InvalidUriParts) -> Self {
        Self::UriParts(e)
    }
}

impl From<core::convert::Infallible> for Error {
    fn from(e: core::convert::Infallible) -> Self {
        match e {}
    }
}
