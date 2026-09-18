use std::io::BufRead;

use crate::{
    Tlv, TlvReader, error,
    types::{FrameHeader, TlvHeader},
};

pub struct Frame<T> {
    pub header: FrameHeader,
    pub payload: Vec<T>,
}

impl<T> TlvReader for Frame<T>
where
    T: Tlv,
{
    fn read<R: BufRead>(buf: &mut R) -> error::Result<Self> {
        let header = FrameHeader::read(buf)?;

        let mut payload = Vec::with_capacity(header.num_tlvs as usize);

        for _ in 0..header.num_tlvs {
            let tlv_header = TlvHeader::read(buf)?;
            let length = tlv_header.length;

            let mut payload_buf = vec![0u8; length as usize];
            buf.read_exact(&mut payload_buf)?;

            let packet = TlvPacket {
                header: &tlv_header,
                payload: &payload_buf,
            };

            if let Ok(tlv) = T::from_packet(packet) {
                payload.push(tlv);
            }
        }

        Ok(Frame { header, payload })
    }
}

pub struct TlvPacket<'a> {
    pub header: &'a TlvHeader,
    pub payload: &'a [u8],
}
