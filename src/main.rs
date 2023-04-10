mod geth;
mod handshake;
mod node_connection_info;
mod remote_ack;
mod test;

use parity_crypto::publickey::{Generator, KeyPair, Public, Random};

use crate::{geth::run_geth, handshake::Handshake};
use std::str::FromStr;
use tokio::{io::AsyncReadExt, net::TcpStream};

#[tokio::main]
async fn main() -> Result<(), String> {
    let (mut geth, connection) = run_geth().await?;

    println!("{connection:#?}");

    let node_pk = Public::from_str(&connection.public_key).unwrap();
    let local_kp: KeyPair = Random.generate();

    let handshake = Handshake::with_remote_public_key(node_pk, local_kp);

    let mut stream = TcpStream::connect(connection.socket())
        .await
        .map_err(|e| e.to_string())?;

    let auth = handshake.encode_auth().map_err(|e| e.to_string())?;

    stream.try_write(&auth).map_err(|e| e.to_string())?;

    let mut buf = [0; 1024];

    let bytes_read = stream.read(&mut buf).await.map_err(|e| e.to_string())?;

    let mut ack = Vec::with_capacity(bytes_read);

    for i in 0..bytes_read {
        ack.push(buf[i]);
    }

    let remote_ack = handshake
        .decode_ack(ack.as_slice())
        .map_err(|e| e.to_string())?;

    println!("Handshake successful:");
    println!("{remote_ack:#?}");

    geth.kill().await.map_err(|e| e.to_string())?;

    Ok(())
}
