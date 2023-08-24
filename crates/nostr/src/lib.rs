// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

#![warn(missing_docs)]
#![warn(rustdoc::bare_urls)]

//! Rust implementation of the Nostr protocol.

#![cfg_attr(
    feature = "default",
    doc = include_str!("../README.md")
)]

#[cfg(feature = "nip19")]
pub use bech32;
#[cfg(feature = "nip06")]
pub use bip39;
#[cfg(feature = "nip06")]
pub use bitcoin;
pub use bitcoin_hashes as hashes;
pub use secp256k1::{self, SECP256K1};
pub use serde_json;

pub mod event;
pub mod key;
pub mod message;
pub mod nips;
pub mod prelude;
pub mod types;
pub mod util;

pub use self::event::tag::{
    ExternalIdentity, HttpMethod, Identity, ImageDimensions, Marker, RelayMetadata, Report, Tag,
    TagKind,
};
pub use self::event::{Event, EventBuilder, EventId, Kind, UnsignedEvent};
pub use self::key::Keys;
pub use self::message::{Alphabet, ClientMessage, Filter, RelayMessage, SubscriptionId};
pub use self::types::{ChannelId, Contact, Entity, Metadata, Profile, Timestamp, UncheckedUrl};

/// Result
pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;
