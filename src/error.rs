#[derive(thiserror::Error, Debug)]
pub enum TlvError {
    #[error("Unexpected TLV type")]
    UnexpectedTlvType,

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, TlvError>;
