use snafu::Snafu;

pub type Result<T> = std::result::Result<T, RlpxError>;

#[derive(Debug, Snafu)]
pub enum RlpxError {
    #[snafu(display("IO error: {source}"), context(false))]
    Io { source: std::io::Error },

    #[snafu(display("Parity crypto error: {source}"), context(false))]
    ParityCrypto {
        source: parity_crypto::publickey::Error,
    },

    #[snafu(display("RLP decoder error: {source}"), context(false))]
    RlpDecoder { source: rlp::DecoderError },

    #[snafu(display("URL parse error: {source}"), context(false))]
    UrlParse { source: url::ParseError },

    #[snafu(display("Failed to start geth: {message}"))]
    Geth { message: &'static str },

    #[snafu(display("Failed to parse NodeConnection: {message}"))]
    NodeConnection { message: &'static str },

    #[snafu(display("Failed to parse key from HEX"))]
    FromHex,
}
