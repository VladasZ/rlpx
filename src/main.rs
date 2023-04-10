#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

mod geth;
mod handshake;
mod remote_ack;
mod test;

use parity_crypto::publickey::{Generator, KeyPair, Public, Random};
use std::io::{Read, Write};

use crate::{geth::run_geth, handshake::Handshake};
use std::str::FromStr;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), String> {
    let (mut geth, connection) = run_geth().await?;

    dbg!(&connection);

    let status = geth.wait().await.map_err(|e| e.to_string())?;
    println!("Exited with status: {}", status);

    // let node_pk = Public::from_str("cc3a313d9894d23fac7decfd268bb052887c415dea339c301c053548ac30243be32d78898c2055f2a2a934638396f6a5906e732da67ab4116a8b13f0c85cc63e").unwrap();
    // let local_kp: KeyPair = Random.generate();
    //
    // let handshake = Handshake::with_remote_public_key(node_pk, local_kp);
    //
    // let mut stream = TcpStream::connect("127.0.0.1:30303")
    //     .await
    //     .map_err(|e| e.to_string())?;

    // let auth = handshake.encode_auth().map_err(|e| e.to_string())?;
    //
    // stream.try_write(&auth).map_err(|e| e.to_string())?;
    //
    // let mut buf = [0; 1024];
    //
    // let bytes_read = stream.read(&mut buf).map_err(|e| e.to_string())?;
    //
    // let mut ack = Vec::with_capacity(bytes_read);
    //
    // for i in 0..bytes_read {
    //     ack.push(buf[i]);
    // }
    //
    // let remote_ack = handshake
    //     .decode_ack(ack.as_slice())
    //     .map_err(|e| e.to_string())?;
    //
    // println!("{remote_ack:#?}");

    Ok(())
}
