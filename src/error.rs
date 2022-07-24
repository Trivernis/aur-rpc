use thiserror::Error;

pub type RPCResult<T> = std::result::Result<T, RPCError>;

#[derive(Debug, Error)]
pub enum RPCError {
    #[error("HTTP Error {0}")]
    HTTP(#[from] reqwest::Error),

    #[error("RPC Error {0}")]
    RPC(String),
}
