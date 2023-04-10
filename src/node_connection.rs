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

    pub async fn kill(&mut self) -> Result<(), String> {
        if let Some(child) = &mut self.child {
            child.kill().await.map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub(crate) fn set_child(mut self, child: Child) -> Self {
        self.child = child.into();
        self
    }
}

impl TryFrom<Url> for NodeConnection {
    type Error = String;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        Ok(NodeConnection {
            public_key: url.username().to_string(),
            ip: url.host().ok_or("Failed to get geth IP")?.to_string(),
            port: url.port().ok_or("Failed to get geth port")?,
            child: None,
        })
    }
}
