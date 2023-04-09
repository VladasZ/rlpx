#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

extern crate core;

use std::mem::{transmute, zeroed};

use concat_arrays::concat_arrays;
use hex::decode;
use secp256k1::hashes::sha256;
use rand::{rngs::OsRng, Rng};
// use rlp::RlpStream;
use secp256k1::{ecdh::SharedSecret, Message, PublicKey, Secp256k1, SecretKey};

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
    Version: u32, /* // Ignore additional fields (forward-compatibility)
                   * Rest []rlp.RawValue `rlp:"tail"` */
}

// fn new_auth(remote_pk: &PublicKey) {
//     let mut rng = rand::thread_rng();
//
//     let secp = Secp256k1::new();
//
//     let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
//
//     let shared_secret = SharedSecret::new(remote_pk, &secret_key);
//
//     let auth_vsn: u32 = 4;
//     let sig = shared_secret.secret_bytes();
//     let initiator_pubk: [u8; 64] = unsafe { transmute(public_key) };
//     let initiator_nonce: [u8; shaLen] = rng.gen();
//
//     let mut stream = RlpStream::new();
//
//     stream
//         .append(&sig.as_slice())
//         .append(&initiator_pubk.as_slice())
//         .append(&initiator_nonce.as_slice())
//         .append(&auth_vsn);
//
//
//     let stream_out = stream.out().freeze();
//
//
//     let auth_body = stream_out.as_ref();
//
//
//     dbg!(auth_body.len());
//
// }

fn make_auth(remote_pk: &PublicKey) -> authMsgV4 {
    let mut rng = rand::thread_rng();

    //secure random?
    let Nonce: [u8; shaLen] = rng.gen();

    let secp = Secp256k1::new();
    let (random_secret_key, random_public_key) = secp.generate_keypair(&mut OsRng);
    let (secret_key, public_key) = prepared_keypair();//secp.generate_keypair(&mut OsRng);


    let remote_pk_bytes: [u8; 64] = unsafe { transmute(*remote_pk) };

    println!("remote_pk_bytes:");
    println!("{:?}", remote_pk_bytes);

    let shared_secret = SharedSecret::new(remote_pk, &secret_key);

    let shared_secret_bytes = shared_secret.secret_bytes();

    println!("Shared secret bytes:");
    println!("{:?}", shared_secret_bytes);

    let mut signed: [u8; 32] = Default::default();

    for (index, (shared, nonce)) in shared_secret_bytes.iter().zip(Nonce.iter()).enumerate() {
        signed[index] = shared ^ nonce;
    }

    let (recovery_id, Signature) = secp
        .sign_ecdsa_recoverable(&Message::from_slice(&signed).unwrap(), &random_secret_key)
        .serialize_compact();

    let Signature = concat_arrays!(Signature, [recovery_id.to_i32() as u8]);

    //secp.sig()

    let InitiatorPubkey: [u8; 64] = unsafe { transmute(public_key) };

    authMsgV4 {
        Signature,
        InitiatorPubkey,
        Nonce,
        Version: 4,
        Rest: Default::default(),
    }
}

fn prepared_keypair() -> (SecretKey, PublicKey) {

    let secret_data: [u8; 32] = [5, 184, 128, 235, 18, 21, 243, 158, 13, 233, 198, 243, 68, 185, 239, 197, 134, 188, 182, 0, 41, 146, 254, 4, 202, 64, 156, 68, 28, 116, 179, 21];

    let public_data: [u8; 65] = [4, 137, 55, 58, 227, 77, 146, 51, 29, 240, 76, 94, 204, 147, 233, 247, 152, 144, 50, 49, 41, 83, 16, 102, 28, 163, 196, 181, 145, 30, 31, 122, 133, 254, 38, 123, 125, 143, 254, 211, 248, 165, 66, 200, 253, 253, 172, 45, 236, 99, 229, 79, 165, 75, 168, 180, 204, 131, 134, 128, 210, 3, 124, 198, 31];


    // let secret_key = SecretKey::from_slice(&secret_data).unwrap();
    let public_key = PublicKey::from_slice(&public_data).unwrap();

    let secret_key: SecretKey = unsafe { transmute(secret_data) };
    //let public_key: PublicKey = unsafe { transmute(public_data) };

    let secp = Secp256k1::new();

    let message = Message::from_hashed_data::<sha256::Hash>("Hello World!".as_bytes());

    let sig = secp.sign_ecdsa(&message, &secret_key);
    assert!(secp.verify_ecdsa(&message, &sig, &public_key).is_ok());

    (secret_key, public_key)
}

fn main() {
    let enode_url = "enode://cc3a313d9894d23fac7decfd268bb052887c415dea339c301c053548ac30243be32d78898c2055f2a2a934638396f6a5906e732da67ab4116a8b13f0c85cc63e@127.0.0.1:30303";
    let public_key_hex = &enode_url[8..136]; // Extract public key from ENode URL

    let public_key_bytes_vec = decode(public_key_hex).unwrap();

    let mut public_key_bytes: [u8; 64] = unsafe { zeroed() };

    for i in 0..64 {
        public_key_bytes[i] = public_key_bytes_vec[i];
    }

    let new_slice = [4].into_iter().chain(public_key_bytes.into_iter()).collect::<Vec<_>>();

    let public_key = PublicKey::from_slice(&new_slice).unwrap();

    let public_key: PublicKey = unsafe { transmute(public_key_bytes) };

    let auth = make_auth(&public_key);


    // println!("Auth Signature {:?}", &auth.Signature);
    //
    // dbg!(size_of::<authMsgV4>());
}
