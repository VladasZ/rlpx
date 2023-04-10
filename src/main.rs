#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use parity_crypto::publickey::{
    ecdh, ecies, sign, Generator, KeyPair, Message, Public, Random, Secret,
};
use rand::Rng;
use rlp::{Rlp, RlpStream};
use std::io::{Read, Write};

use std::{net::TcpStream, str::FromStr};

const PROTOCOL_VERSION: u64 = 4;
const ECIES_OVERHEAD: usize = 113;

fn make_auth(node_pk: &Public, kp: &KeyPair, ephemeral: &KeyPair) -> Result<Vec<u8>, String> {
    let nonce = Message::random();

    let mut rlp = RlpStream::new_list(4);
    let shared = *ecdh::agree(kp.secret(), &node_pk).map_err(|e| e.to_string())?;
    rlp.append(
        &sign(ephemeral.secret(), &(shared ^ nonce))
            .map_err(|e| e.to_string())?
            .to_vec(),
    );
    rlp.append(kp.public());
    rlp.append(&nonce);
    rlp.append(&PROTOCOL_VERSION);
    let mut encoded = rlp.out();
    encoded.resize(encoded.len() + rand::thread_rng().gen_range(100..301), 0);
    let len = (encoded.len() + ECIES_OVERHEAD) as u16;
    let prefix = len.to_be_bytes();
    let message = ecies::encrypt(&node_pk, &prefix, &encoded).map_err(|e| e.to_string())?;

    let mut auth_cipher = Vec::default();

    auth_cipher.extend_from_slice(&prefix);
    auth_cipher.extend_from_slice(&message);

    Ok(auth_cipher)
}

fn read_ack_eip8(secret: &Secret, data: &[u8]) -> Result<(), String> {
    let mut ack_cipher = Vec::new();
    ack_cipher.extend_from_slice(data);
    let ack =
        ecies::decrypt(secret, &ack_cipher[0..2], &ack_cipher[2..]).map_err(|e| e.to_string())?;

    println!("ACK!!: {:?}", ack);

    let rlp = Rlp::new(&ack);
    let remote_ephemeral: Public = rlp.val_at(0).map_err(|e| e.to_string())?;
    let remote_nonce: Message = rlp.val_at(1).map_err(|e| e.to_string())?;
    let remote_version: u64 = rlp.val_at(2).map_err(|e| e.to_string())?;

    dbg!(&remote_version);
    dbg!(&remote_ephemeral);
    dbg!(&remote_nonce);

    Ok(())
}

fn main() -> Result<(), String> {
    let node_pk = Public::from_str("cc3a313d9894d23fac7decfd268bb052887c415dea339c301c053548ac30243be32d78898c2055f2a2a934638396f6a5906e732da67ab4116a8b13f0c85cc63e").unwrap();

    let kp: KeyPair = Random.generate();

    let ephemeral: KeyPair = Random.generate();

    let auth = make_auth(&node_pk, &kp, &ephemeral)?;

    println!("{:?}", auth);

    let mut stream = TcpStream::connect("127.0.0.1:30303").map_err(|e| e.to_string())?;

    stream.write(&auth).map_err(|e| e.to_string())?;

    let mut buf = [0; 1024];

    let bytes_read = stream.read(&mut buf).map_err(|e| e.to_string())?;

    dbg!(&bytes_read);

    let mut ack = Vec::with_capacity(bytes_read);

    for i in 0..bytes_read {
        ack.push(buf[i]);
    }

    println!("{}", String::from_utf8_lossy(&buf));

    read_ack_eip8(kp.secret(), &ack)?;

    Ok(())
}
