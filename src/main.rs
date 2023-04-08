#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

extern crate core;

use concat_arrays::concat_arrays;
use hex::decode;
use rand::rngs::OsRng;
use rand::Rng;
use secp256k1::ecdh::SharedSecret;
use secp256k1::{Message, PublicKey, Secp256k1};
use std::mem::transmute;
use std::mem::size_of;

// Constants for the handshake.
mod consts {
    pub const SignatureLength: usize = 64 + 1; // 64 bytes ECDSA signature + 1 byte recovery id

    pub const sskLen: u32 = 16;
    // ecies.MaxSharedKeyLength(pubKey) / 2
    pub const sigLen: usize = SignatureLength; // elliptic S256
pub const pubLen: usize = 64; // 512 bit pubkey in uncompressed representation without format byte
pub const shaLen: usize = 32; // hash length (for nonce etc)

    pub const eciesOverhead: u32 = 65 /* pubkey */ + 16 /* IV */ + 32 /* MAC */;
}

use crate::consts::*;

// RLPx v4 handshake auth (defined in EIP-8).
#[derive(Debug)]
struct authMsgV4 {
    Signature: [u8; sigLen],
    InitiatorPubkey: [u8; pubLen],
    Nonce: [u8; shaLen],
    Version: u32,
    // Ignore additional fields (forward-compatibility)
    //Rest []rlp.RawValue `rlp:"tail"`
    Rest: [u8; 32],
}

// RLPx v4 handshake response (defined in EIP-8).
#[derive(Debug)]
struct authRespV4 {
    RandomPubkey: [u8; pubLen],
    Nonce: [u8; shaLen],
    Version: u32, // // Ignore additional fields (forward-compatibility)
    // Rest []rlp.RawValue `rlp:"tail"`
}

fn make_auth(remote_pk: &PublicKey) -> authMsgV4 {
    let mut rng = rand::thread_rng();

    let Nonce: [u8; shaLen] = rng.gen();

    let secp = Secp256k1::new();
    let (random_secret_key, random_public_key) = secp.generate_keypair(&mut OsRng);
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

    let shared_secret = SharedSecret::new(remote_pk, &secret_key);

    let shared_secret_bytes = shared_secret.secret_bytes();

    let mut signed: [u8; 32] = Default::default();

    for (index, (shared, nonce)) in shared_secret_bytes.iter().zip(Nonce.iter()).enumerate() {
        signed[index] = shared ^ nonce;
    }

    let (recovery_id, Signature) = secp
        .sign_ecdsa_recoverable(&Message::from_slice(&signed).unwrap(), &random_secret_key)
        .serialize_compact();

    let Signature = concat_arrays!(Signature, [recovery_id.to_i32() as u8]);

    let InitiatorPubkey: [u8; 64] = unsafe { transmute(public_key) };

    authMsgV4 {
        Signature,
        InitiatorPubkey,
        Nonce,
        Version: 4,
        Rest: Default::default(),
    }
}

fn main() {
    let enode_url = "enode://cc3a313d9894d23fac7decfd268bb052887c415dea339c301c053548ac30243be32d78898c2055f2a2a934638396f6a5906e732da67ab4116a8b13f0c85cc63e@127.0.0.1:30303";
    let public_key_hex = &enode_url[8..136]; // Extract public key from ENode URL

    let dota = decode(public_key_hex).unwrap();

    assert_eq!(dota.len(), 64);

    let new_slice = [4].into_iter().chain(dota.into_iter()).collect::<Vec<_>>();

    let public_key = PublicKey::from_slice(&new_slice).unwrap();

    dbg!(&public_key);

    let auth = make_auth(&public_key);

    dbg!(auth);

    dbg!(size_of::<authMsgV4>());

}