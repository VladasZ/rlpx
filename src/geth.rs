//! Run `geth` as a test node

use log::error;
use std::process::Stdio;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
    spawn,
    sync::mpsc::channel,
};
use url::Url;

#[derive(Debug)]
pub struct NodeConnectionInfo {
    pub public_key: String,
    pub ip: String,
    pub port: u16,
}

/// Start `geth` P2P node and parse its output
/// Returns [`Child`] process and P2P [`NodeConnectionInfo`]
pub async fn run_geth() -> Result<(Child, NodeConnectionInfo), String> {
    let mut child = Command::new("make")
        .arg("run-geth")
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    let stderr = child.stderr.take().expect("stdout not captured");
    let mut reader = BufReader::new(stderr).lines();

    let (sender, mut receiver) = channel::<String>(1);

    // Waiting for enode URL
    spawn(async move {
        while let Ok(Some(line)) = reader.next_line().await {
            if line.contains("self=enode") {
                if let Err(err) = sender.send(line).await {
                    error!("Failed to send enode URL: {err}");
                }
            }
        }
    });

    let enode = receiver.recv().await.ok_or("Failed to receive enode URL")?;

    // Output from geth looks something like this:
    // INFO [04-10|22:59:10.860] Started P2P networking self=enode://cc3a313d9894d23fac7decfd268bb052887c415dea339c301c053548ac30243be32d78898c2055f2a2a934638396f6a5906e732da67ab4116a8b13f0c85cc63e@127.0.0.1:30303
    // Here we etract enode URL from this line
    let parts = enode.split("self=").collect::<Vec<_>>();

    let enode = parts
        .get(1)
        .ok_or("Failed to parse geth output line with enode")?;

    let enode = Url::parse(enode).map_err(|e| e.to_string())?;

    let connection = NodeConnectionInfo {
        public_key: enode.username().to_string(),
        ip: enode.host().ok_or("Failed to get geth IP")?.to_string(),
        port: enode.port().ok_or("Failed to get geth port")?,
    };

    Ok((child, connection))
}
