// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use nostr::nips::nip19::{FromBech32, ToBech32};
use nostr::nips::nip21::NostrURI;
use nostr::secp256k1::XOnlyPublicKey;
use uniffi::Object;

use crate::error::Result;

#[derive(Debug, Object)]
pub struct PublicKey {
    inner: XOnlyPublicKey,
}

impl From<XOnlyPublicKey> for PublicKey {
    fn from(inner: XOnlyPublicKey) -> Self {
        Self { inner }
    }
}

impl From<&PublicKey> for XOnlyPublicKey {
    fn from(pk: &PublicKey) -> Self {
        pk.inner
    }
}

impl Deref for PublicKey {
    type Target = XOnlyPublicKey;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[uniffi::export]
impl PublicKey {
    #[uniffi::constructor]
    pub fn from_hex(hex: String) -> Result<Arc<Self>> {
        Ok(Arc::new(Self {
            inner: XOnlyPublicKey::from_str(&hex)?,
        }))
    }

    #[uniffi::constructor]
    pub fn from_bech32(pk: String) -> Result<Arc<Self>> {
        Ok(Arc::new(Self {
            inner: XOnlyPublicKey::from_bech32(pk)?,
        }))
    }

    #[uniffi::constructor]
    pub fn from_nostr_uri(uri: String) -> Result<Arc<Self>> {
        Ok(Arc::new(Self {
            inner: XOnlyPublicKey::from_nostr_uri(uri)?,
        }))
    }

    pub fn to_hex(&self) -> String {
        self.inner.to_string()
    }

    pub fn to_bech32(&self) -> Result<String> {
        Ok(self.inner.to_bech32()?)
    }

    pub fn to_nostr_uri(&self) -> Result<String> {
        Ok(self.inner.to_nostr_uri()?)
    }
}
