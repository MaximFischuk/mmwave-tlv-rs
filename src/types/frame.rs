use std::io::BufReader;

use crate::{
    Tlv, TlvReader, error,
    types::{FrameHeader, TlvPayload},
};

pub struct Frame<T> {
    pub header: FrameHeader,
    pub payload: Vec<TlvPayload<T>>,
}

impl<T> TlvReader for Frame<T>
where
    T: Tlv + TlvReader,
{
    fn read(bytes: &[u8]) -> error::Result<Self> {
        let header = FrameHeader::read(&bytes[..FrameHeader::LENGTH])?;

        let mut payload = Vec::with_capacity(header.num_tlvs as usize);
        let buffer = BufReader::new(&bytes[FrameHeader::LENGTH..]);

        Ok(Frame { header, payload })
    }
}
