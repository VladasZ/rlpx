use crate::error::RlpxError;
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
    type Error = RlpxError;
    fn try_from(ack: &[u8]) -> Result<Self, Self::Error> {
        let rlp = Rlp::new(&ack);

        Ok(RemoteAck {
            ephemeral: rlp.val_at(0)?,
            nonce: rlp.val_at(1)?,
            protocol_version: rlp.val_at(2)?,
        })
    }
}
