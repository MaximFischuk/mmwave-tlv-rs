use std::io::BufRead;

use crate::{
    Tlv, TlvReader, error,
    types::{FrameHeader, TlvHeader, TlvPayload},
};

pub struct Frame<T> {
    pub header: FrameHeader,
    pub payload: Vec<TlvPayload<T>>,
}

impl<T> TlvReader for Frame<T>
where
    T: Tlv + TlvReader,
{
    fn read<R: BufRead>(buf: &mut R) -> error::Result<Self> {
        let header = FrameHeader::read(buf)?;

        let mut payload = Vec::with_capacity(header.num_tlvs as usize);

        for _ in 0..header.num_tlvs {
            let tlv_header = TlvHeader::read(buf)?;
            let length = tlv_header.length;
            // let tlv = TlvPayload::<T>::read_with_header(buf, tlv_header)?;
            // payload.push(tlv);
        }

        Ok(Frame { header, payload })
    }
}

// TODO: Stream reader
