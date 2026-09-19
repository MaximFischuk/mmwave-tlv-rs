#[derive(thiserror::Error, Debug)]
pub enum TlvError {
    #[error("Unexpected TLV type")]
    UnexpectedTlvType,

    #[error("incomplete frame header")]
    IncompleteFrameHeader,

    #[error("invalid frame header")]
    InvalidFrameHeader,

    #[error("frame length is smaller than its header")]
    FrameLengthSmallerThanHeader,

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, TlvError>;
