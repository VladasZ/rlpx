//! This module is used to generate messages required for RLPx v4 handshake
//! Protocol description: [EIP-8](https://eips.ethereum.org/EIPS/eip-8)

use crate::remote_ack::RemoteAck;
use parity_crypto::publickey::{
    ecdh, ecies, sign, Generator, KeyPair, Message, Public, Random, Secret,
};
use rand::Rng;
use rlp::RlpStream;

const PROTOCOL_VERSION: u64 = 4;
const ECIES_OVERHEAD: usize = 113;

/// Generates messages required for RLPx handshake
pub struct Handshake {
    remote_pk: Public,
    static_kp: KeyPair,
    ephemeral_kp: KeyPair,
    nonce: Message,
}

impl Handshake {
    /// Initialize [`Handshake`] with remote node public key and local [`KeyPair`]
    pub fn with_remote_public_key(remote_pk: Public, static_kp: KeyPair) -> Self {
        Self {
            remote_pk,
            static_kp,
            ephemeral_kp: Random.generate(),
            nonce: Message::random(),
        }
    }
}

impl Handshake {
    /// Generate and encode auth message as initiator
    pub fn encode_auth(&self) -> Result<Vec<u8>, String> {
        encode_auth(
            &self.remote_pk,
            &self.static_kp,
            &self.ephemeral_kp,
            &self.nonce,
        )
    }

    /// Decode ack message received from recipient
    pub fn decode_ack(&self, ack: &[u8]) -> Result<RemoteAck, String> {
        decode_ack(self.static_kp.secret(), ack)
    }
}

fn encode_auth(
    remote_pk: &Public,
    local_kp: &KeyPair,
    ephemeral_kp: &KeyPair,
    nonce: &Message,
) -> Result<Vec<u8>, String> {
    let mut rlp = RlpStream::new_list(4);
    let shared = *ecdh::agree(local_kp.secret(), &remote_pk).map_err(|e| e.to_string())?;
    rlp.append(
        &sign(ephemeral_kp.secret(), &(shared ^ *nonce))
            .map_err(|e| e.to_string())?
            .to_vec(),
    );
    rlp.append(local_kp.public());
    rlp.append(nonce);
    rlp.append(&PROTOCOL_VERSION);
    let mut encoded = rlp.out();
    encoded.resize(encoded.len() + rand::thread_rng().gen_range(100..301), 0);
    let len = (encoded.len() + ECIES_OVERHEAD) as u16;
    let prefix = len.to_be_bytes();
    let message = ecies::encrypt(&remote_pk, &prefix, &encoded).map_err(|e| e.to_string())?;

    let mut auth_cipher = Vec::default();

    auth_cipher.extend_from_slice(&prefix);
    auth_cipher.extend_from_slice(&message);

    Ok(auth_cipher)
}

fn decode_ack(secret: &Secret, data: &[u8]) -> Result<RemoteAck, String> {
    let ack = ecies::decrypt(secret, &data[0..2], &data[2..]).map_err(|e| e.to_string())?;
    RemoteAck::try_from(ack.as_slice())
}
