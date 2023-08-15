// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

//! NIP57
//!
//! <https://github.com/nostr-protocol/nips/blob/master/57.md>

use core::fmt;

use aes::cipher::block_padding::Pkcs7;
use aes::cipher::{BlockEncryptMut, KeyIvInit};
use bech32::{self, ToBase32, Variant};
use bitcoin_hashes::sha256::Hash as Sha256Hash;
use bitcoin_hashes::Hash;
use secp256k1::{SecretKey, XOnlyPublicKey};

use super::nip04::{self, Aes256CbcEnc};
use super::nip33::ParameterizedReplaceableEvent;
use crate::event::builder::Error as BuilderError;
use crate::event::unsigned::Error as UnsignedError;
use crate::key::Error as KeyError;
use crate::{Event, EventBuilder, EventId, Keys, Tag, Timestamp, UncheckedUrl, UnsignedEvent};

#[allow(missing_docs)]
#[derive(Debug)]
pub enum Error {
    Key(KeyError),
    Builder(BuilderError),
    NIP04(nip04::Error),
    Bech32(bech32::Error),
    Secp256k1(secp256k1::Error),
    Unsigned(UnsignedError),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Key(e) => write!(f, "{e}"),
            Self::Builder(e) => write!(f, "{e}"),
            Self::NIP04(e) => write!(f, "{e}"),
            Self::Bech32(e) => write!(f, "{e}"),
            Self::Secp256k1(e) => write!(f, "{e}"),
            Self::Unsigned(e) => write!(f, "{e}"),
        }
    }
}

impl From<KeyError> for Error {
    fn from(e: KeyError) -> Self {
        Self::Key(e)
    }
}

impl From<BuilderError> for Error {
    fn from(e: BuilderError) -> Self {
        Self::Builder(e)
    }
}

impl From<nip04::Error> for Error {
    fn from(e: nip04::Error) -> Self {
        Self::NIP04(e)
    }
}

impl From<bech32::Error> for Error {
    fn from(e: bech32::Error) -> Self {
        Self::Bech32(e)
    }
}

impl From<secp256k1::Error> for Error {
    fn from(e: secp256k1::Error) -> Self {
        Self::Secp256k1(e)
    }
}

impl From<UnsignedError> for Error {
    fn from(e: UnsignedError) -> Self {
        Self::Unsigned(e)
    }
}

/// Zap Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ZapType {
    /// Public
    Public,
    /// Private
    Private,
    /// Anonymous
    Anonymous,
}

/// Zap Request Data
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZapRequestData {
    /// Public key of the recipient
    pub public_key: XOnlyPublicKey,
    /// List of relays the recipient's wallet should publish its zap receipt to
    pub relays: Vec<UncheckedUrl>,
    /// Message
    pub message: String,
    /// Amount in `millisats` the sender intends to pay
    pub amount: Option<u64>,
    /// Lnurl pay url of the recipient, encoded using bech32 with the prefix lnurl.
    pub lnurl: Option<String>,
    /// Event ID
    pub event_id: Option<EventId>,
    /// NIP-33 event coordinate that allows tipping parameterized replaceable events such as NIP-23 long-form notes.
    pub event_coordinate: Option<ParameterizedReplaceableEvent>,
}

impl ZapRequestData {
    /// New Zap Request Data
    pub fn new(public_key: XOnlyPublicKey, relays: Vec<UncheckedUrl>) -> Self {
        Self {
            public_key,
            relays,
            message: String::new(),
            amount: None,
            lnurl: None,
            event_id: None,
            event_coordinate: None,
        }
    }

    /// Message
    pub fn message<S>(self, message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            message: message.into(),
            ..self
        }
    }

    /// Amount in `millisats` the sender intends to pay
    pub fn amount(self, amount: u64) -> Self {
        Self {
            amount: Some(amount),
            ..self
        }
    }

    /// Lnurl pay url of the recipient, encoded using bech32 with the prefix lnurl.
    pub fn lnurl<S>(self, lnurl: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            lnurl: Some(lnurl.into()),
            ..self
        }
    }

    /// Event ID
    pub fn event_id(self, event_id: EventId) -> Self {
        Self {
            event_id: Some(event_id),
            ..self
        }
    }

    /// NIP-33 event coordinate that allows tipping parameterized replaceable events such as NIP-23 long-form notes.
    pub fn event_coordinate(self, event_coordinate: ParameterizedReplaceableEvent) -> Self {
        Self {
            event_coordinate: Some(event_coordinate),
            ..self
        }
    }
}

/// Create anonymous zap request
pub fn anonymous_zap_request(data: ZapRequestData) -> Result<Event, Error> {
    let mut builder = EventBuilder::new_zap_request(data);
    builder.tags.push(Tag::Anon { msg: None });
    let keys = Keys::generate();
    Ok(builder.to_event(&keys)?)
}

/// Create private zap request
pub fn private_zap_request(data: ZapRequestData, keys: &Keys) -> Result<Event, Error> {
    let created_at: Timestamp = Timestamp::now();
    let secret_key = create_encryption_key(keys.secret_key()?, data.public_key, created_at)?;

    let msg = encrypt_private_zap_message(secret_key, data.public_key, &data.message)?;
    let mut builder = EventBuilder::new_zap_request(data);
    builder.content.clear();
    builder.tags.push(Tag::Anon { msg: Some(msg) });

    let keys = Keys::new(secret_key);
    let id = EventId::new(
        &keys.public_key(),
        created_at,
        &builder.kind,
        &builder.tags,
        &builder.content,
    );
    let unsigned = UnsignedEvent {
        id,
        pubkey: keys.public_key(),
        created_at,
        kind: builder.kind,
        tags: builder.tags,
        content: builder.content,
    };

    Ok(unsigned.sign(&keys)?)
}

fn create_encryption_key(
    secret_key: SecretKey,
    public_key: XOnlyPublicKey,
    created_at: Timestamp,
) -> Result<SecretKey, Error> {
    let mut unhashed: String = secret_key.display_secret().to_string();
    unhashed.push_str(&public_key.to_string());
    unhashed.push_str(&created_at.to_string());
    let hash = Sha256Hash::hash(unhashed.as_bytes());
    Ok(SecretKey::from_slice(hash.as_byte_array())?)
}

fn encrypt_private_zap_message<T>(
    secret_key: SecretKey,
    public_key: XOnlyPublicKey,
    msg: T,
) -> Result<String, Error>
where
    T: AsRef<[u8]>,
{
    let key: [u8; 32] = nip04::generate_shared_key(&secret_key, &public_key)?;
    let iv: [u8; 16] = secp256k1::rand::random();

    let cipher = Aes256CbcEnc::new(&key.into(), &iv.into());
    let result: Vec<u8> = cipher.encrypt_padded_vec_mut::<Pkcs7>(msg.as_ref());

    // Bech32 msg
    let data = result.to_base32();
    let encrypted_bech32_msg = bech32::encode("pzap", data, Variant::Bech32)?;

    // Bech32 IV
    let data = iv.to_base32();
    let iv_bech32 = bech32::encode("iv", data, Variant::Bech32)?;

    Ok(format!("{encrypted_bech32_msg}_{iv_bech32}"))
}
