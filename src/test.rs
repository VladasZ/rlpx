#![cfg(test)]

use crate::handshake;

#[tokio::test]
async fn test() -> Result<(), String> {
    handshake(None).await
}
