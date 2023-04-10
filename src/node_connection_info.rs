use url::Url;

#[derive(Debug)]
pub struct NodeConnectionInfo {
    pub public_key: String,
    pub ip: String,
    pub port: u16,
}

impl NodeConnectionInfo {
    pub fn socket(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

impl TryFrom<Url> for NodeConnectionInfo {
    type Error = String;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        Ok(NodeConnectionInfo {
            public_key: url.username().to_string(),
            ip: url.host().ok_or("Failed to get geth IP")?.to_string(),
            port: url.port().ok_or("Failed to get geth port")?,
        })
    }
}
