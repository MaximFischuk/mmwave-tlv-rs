use crate::{Tlv, TlvReader, error, types::TlvHeader};

const TLV_HEADER_LENGTH: usize = std::mem::size_of::<TlvHeader>();

pub struct TlvPayload<T>
where
    T: Sized,
{
    pub header: TlvHeader,
    pub value: T,
}

impl<T> TlvReader for TlvPayload<T>
where
    T: Tlv + TlvReader,
{
    fn read(bytes: &[u8]) -> error::Result<Self> {
        if bytes.is_empty() {
            return Err(error::TlvError::MissingTlvPayload);
        }
        let header = TlvHeader::read(
            bytes
                .get(..TLV_HEADER_LENGTH)
                .ok_or(error::TlvError::InvalidTlvHeaderLength)?,
        )?;
        if bytes.len() != std::mem::size_of::<T>() + TLV_HEADER_LENGTH {
            return Err(error::TlvError::InvalidTlvLength);
        }
        if T::TYPE != header.r#type {
            return Err(error::TlvError::UnexpectedTlvType);
        }
        let value = TlvReader::read(&bytes[TLV_HEADER_LENGTH..])?;
        Ok(TlvPayload { header, value })
    }
}
