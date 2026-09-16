#[derive(thiserror::Error, Debug)]
pub enum TlvError {
    #[error("Invalid magic word")]
    InvalidMagicWord,
    #[error("Invalid byte-array length")]
    InvalidArrayLength,
    #[error("Invalid primitive value length")]
    InvalidPrimitiveLength,
    #[error("Invalid frame header length")]
    InvalidFrameHeaderLength,
    #[error("Invalid TLV header length")]
    InvalidTlvHeaderLength,
    #[error("TLV payload is missing")]
    MissingTlvPayload,
    #[error("Invalid TLV length")]
    InvalidTlvLength,
    #[error("Unexpected TLV type")]
    UnexpectedTlvType,
    #[error("Invalid point length")]
    InvalidPointLength,
}

pub type Result<T> = std::result::Result<T, TlvError>;
