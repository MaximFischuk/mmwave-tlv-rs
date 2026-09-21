use std::io::BufRead;

use crate::{
    Tlv, TlvReader, error,
    types::{FrameHeader, TlvHeader},
};

/// A decoded TI radar frame.
pub struct Frame<T> {
    pub header: FrameHeader,
    pub payload: T,
}

/// Collects decoded TLVs into a frame payload.
pub trait FramePayload: Sized {
    fn with_capacity(capacity: usize) -> Self;
    fn push(&mut self, packet: TlvPacket<'_>);
}

impl<T: Tlv> FramePayload for Vec<T> {
    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }

    fn push(&mut self, packet: TlvPacket<'_>) {
        if let Ok(tlv) = T::from_packet(packet) {
            self.push(tlv);
        }
    }
}

impl<T> TlvReader for Frame<T>
where
    T: FramePayload,
{
    fn read<R: BufRead>(buf: &mut R) -> error::Result<Self> {
        let header = FrameHeader::read(buf)?;

        let mut payload = T::with_capacity(header.num_tlvs as usize);

        for _ in 0..header.num_tlvs {
            let tlv_header = TlvHeader::read(buf)?;
            let length = tlv_header.length;

            let mut payload_buf = vec![0u8; length as usize];
            buf.read_exact(&mut payload_buf)?;

            let packet = TlvPacket {
                header: &tlv_header,
                payload: &payload_buf,
            };

            payload.push(packet);
        }

        Ok(Frame { header, payload })
    }
}

/// Borrowed TLV header and payload passed to [`Tlv::from_packet`].
pub struct TlvPacket<'a> {
    pub header: &'a TlvHeader,
    pub payload: &'a [u8],
}
