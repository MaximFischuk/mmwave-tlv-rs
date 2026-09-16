#[derive(thiserror::Error, Debug)]
pub enum TlvError {
    #[error("Invalid magic word")]
    InvalidMagicWord,
    #[error("Failed to decode TLV")]
    DecodeError,
}

pub type Result<T> = std::result::Result<T, TlvError>;
