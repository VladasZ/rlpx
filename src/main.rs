mod geth;
mod handshake;
mod node_connection;
mod remote_ack;
mod test;

use parity_crypto::publickey::{Generator, KeyPair, Public, Random};

use crate::{geth::run_geth, handshake::Handshake, node_connection::NodeConnection};
use std::{env, str::FromStr};
use tokio::{io::AsyncReadExt, net::TcpStream};
use url::Url;

/// Perform handshake with remote node.
/// If [`NodeConnection`] is not specified
/// it will start geth and use it
async fn handshake(conn: Option<NodeConnection>) -> Result<(), String> {
    let mut connection = match conn {
        Some(conn) => conn,
        None => run_geth().await?,
    };

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

    connection.kill().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    // parse enode URL
    let connection = match args.get(1) {
        Some(enode) => Some(NodeConnection::try_from(
            Url::parse(enode).map_err(|e| e.to_string())?,
        )?),
        None => None,
    };

    handshake(connection).await
}
