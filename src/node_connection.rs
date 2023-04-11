use crate::error::{Result, RlpxError};
use derivative::Derivative;
use tokio::process::Child;
use url::Url;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct NodeConnection {
    pub public_key: String,
    pub ip: String,
    pub port: u16,
    // Hold process handle if we run node automatically
    #[derivative(Debug = "ignore")]
    child: Option<Child>,
}

impl NodeConnection {
    pub fn socket(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    pub async fn kill(&mut self) -> Result<()> {
        if let Some(child) = &mut self.child {
            child.kill().await?;
        }
        Ok(())
    }

    pub(crate) fn set_child(mut self, child: Child) -> Self {
        self.child = child.into();
        self
    }
}

impl TryFrom<Url> for NodeConnection {
    type Error = RlpxError;

    fn try_from(url: Url) -> Result<Self> {
        Ok(NodeConnection {
            public_key: url.username().to_string(),
            ip: url
                .host()
                .ok_or(RlpxError::NodeConnection {
                    message: "Failed to get geth IP",
                })?
                .to_string(),
            port: url.port().ok_or(RlpxError::NodeConnection {
                message: "Failed to get geth port",
            })?,
            child: None,
        })
    }
}
