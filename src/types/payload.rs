use crate::{Tlv, TlvDecode, error, types::TlvHeader};

pub struct TlvPayload<T> {
    pub header: TlvHeader,
    pub value: T,
}

impl<T> TlvDecode for TlvPayload<T>
where
    T: Tlv + TlvDecode,
{
    fn decode(bytes: &[u8]) -> error::Result<Self> {
        if bytes.is_empty() {
            return Err(error::TlvError::MissingTlvPayload);
        }
        let header = TlvHeader::decode(
            bytes
                .get(..8)
                .ok_or(error::TlvError::InvalidTlvHeaderLength)?,
        )?;
        if bytes.len() != std::mem::size_of::<T>() + 8 {
            return Err(error::TlvError::InvalidTlvLength);
        }
        if T::TYPE != header.r#type {
            return Err(error::TlvError::UnexpectedTlvType);
        }
        let value = TlvDecode::decode(&bytes[8..])?;
        Ok(TlvPayload { header, value })
    }
}
