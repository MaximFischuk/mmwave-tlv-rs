use crate::{
    Tlv, TlvDecode, error,
    types::{FrameHeader, TlvPayload},
};

pub struct Frame<T> {
    pub header: FrameHeader,
    pub payload: Vec<TlvPayload<T>>,
}

impl<T> TlvDecode for Frame<T>
where
    T: Tlv + TlvDecode,
{
    fn decode(bytes: &[u8]) -> error::Result<Self> {
        let header = FrameHeader::decode(&bytes[..FrameHeader::LENGTH])?;
        // Implement the decoding logic for the payloads here
        unimplemented!()
    }
}
