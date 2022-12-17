// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use anyhow::Result;
use bitcoin::hashes::Hash;
use bitcoin::secp256k1::schnorr::Signature;
use bitcoin::secp256k1::{KeyPair, Message, Secp256k1, XOnlyPublicKey};

use crate::key::Keys;
use crate::Sha256Hash;

pub fn sign_delegation(
    keys: &Keys,
    delegatee_pk: XOnlyPublicKey,
    conditions: String,
) -> Result<Signature> {
    let secp = Secp256k1::new();
    let keypair: &KeyPair = &keys.key_pair()?;
    let unhashed_token: String = format!("nostr:delegation:{}:{}", delegatee_pk, conditions);
    let hashed_token = Sha256Hash::hash(unhashed_token.as_bytes());
    let message = Message::from_slice(&hashed_token)?;
    Ok(secp.sign_schnorr(&message, keypair))
}
