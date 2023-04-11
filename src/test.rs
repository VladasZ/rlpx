#![cfg(test)]

use crate::{error::Result, handshake};

#[tokio::test]
async fn test() -> Result<()> {
    handshake(None).await
}
