// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

//! Url

use core::fmt;
use core::str::FromStr;

use serde::{Deserialize, Serialize};

use super::uri::{self, Uri};

/// Url Error
#[derive(Debug)]
pub enum Error {
    /// Url error
    Url(uri::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Url(e) => write!(f, "Url: {e}"),
        }
    }
}

impl From<uri::Error> for Error {
    fn from(e: uri::Error) -> Self {
        Self::Url(e)
    }
}

/// Unchecked Url
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UncheckedUrl(String);

impl UncheckedUrl {
    /// New unchecked url
    pub fn new<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        Self(url.into())
    }

    /// Empty unchecked url
    pub fn empty() -> Self {
        Self(String::new())
    }
}

impl<S> From<S> for UncheckedUrl
where
    S: Into<String>,
{
    fn from(url: S) -> Self {
        Self(url.into())
    }
}

impl FromStr for UncheckedUrl {
    type Err = Error;

    fn from_str(url: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(url))
    }
}

impl TryFrom<UncheckedUrl> for Uri {
    type Error = Error;

    fn try_from(unchecked_url: UncheckedUrl) -> Result<Self, Self::Error> {
        Self::from_str(&unchecked_url.0).map_err(|e| Error::Url(uri::Error::Uri(e)))
    }
}

impl fmt::Display for UncheckedUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn test_unchecked_relay_url() -> Result<()> {
        let relay = "wss://relay.damus.io/";
        let relay_url = Uri::from_str(relay)?;

        println!("{}", relay_url.to_string());

        let unchecked_relay_url = UncheckedUrl::from(relay_url.to_string());

        assert_eq!(unchecked_relay_url, UncheckedUrl::from(relay));

        assert_eq!(Uri::try_from(unchecked_relay_url.clone())?, relay_url);

        assert_eq!(relay, unchecked_relay_url.to_string());

        Ok(())
    }
}
