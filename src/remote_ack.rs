use parity_crypto::publickey::{Message, Public};
use rlp::Rlp;

/// Data returned by remote in Ack message
#[derive(Debug)]
pub struct RemoteAck {
    pub ephemeral: Public,
    pub nonce: Message,
    pub protocol_version: u64,
}

/// Parse from RLP encoded buffer
impl TryFrom<&[u8]> for RemoteAck {
    type Error = String;
    fn try_from(ack: &[u8]) -> Result<Self, Self::Error> {
        let rlp = Rlp::new(&ack);

        Ok(RemoteAck {
            ephemeral: rlp.val_at(0).map_err(|e| e.to_string())?,
            nonce: rlp.val_at(1).map_err(|e| e.to_string())?,
            protocol_version: rlp.val_at(2).map_err(|e| e.to_string())?,
        })
    }
}
