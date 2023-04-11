mod error;
mod geth;
mod handshake;
mod node_connection;
mod remote_ack;
mod test;

use parity_crypto::publickey::{Generator, KeyPair, Public, Random};

use crate::{
    error::{Result, RlpxError},
    geth::run_geth,
    handshake::Handshake,
    node_connection::NodeConnection,
};
use std::{env, str::FromStr};
use tokio::{io::AsyncReadExt, net::TcpStream};
use url::Url;

/// Perform handshake with remote node.
/// If [`NodeConnection`] is not specified
/// it will start geth and use it
async fn handshake(conn: Option<NodeConnection>) -> Result<()> {
    let mut connection = match conn {
        Some(conn) => conn,
        None => run_geth().await?,
    };

    println!("{connection:#?}");

    let node_public_key =
        Public::from_str(&connection.public_key).map_err(|_| RlpxError::FromHex)?;
    let local_keypair: KeyPair = Random.generate();

    let handshake = Handshake::with_remote_public_key(node_public_key, local_keypair);

    let auth = handshake.encode_auth()?;

    // Send auth packet
    let mut stream = TcpStream::connect(connection.socket()).await?;
    stream.try_write(&auth)?;

    // Receive ack packet
    let mut ack_buf = [0; 1024];
    let bytes_read = stream.read(&mut ack_buf).await?;
    let remote_ack = handshake.decode_ack(&ack_buf[0..bytes_read])?;

    println!("Handshake successful:");
    println!("{remote_ack:#?}");

    connection.kill().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // parse enode URL
    let connection = match args.get(1) {
        Some(enode) => Some(NodeConnection::try_from(Url::parse(enode)?)?),
        None => None,
    };

    handshake(connection).await
}
