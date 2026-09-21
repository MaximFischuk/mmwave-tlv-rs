#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unexpected TLV type")]
    UnexpectedTlvType,

    #[error("incomplete frame header")]
    IncompleteFrameHeader,

    #[error("invalid frame header")]
    InvalidFrameHeader,

    #[error("frame length is smaller than its header")]
    FrameLengthSmallerThanHeader,

    #[error("tag value must be non-zero")]
    NonZeroTagValue,

    #[error("required frame TLV is missing")]
    MissingRequiredTlv,

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
