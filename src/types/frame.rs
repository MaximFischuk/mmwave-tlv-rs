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
    type Builder: FramePayloadBuilder<Payload = Self>;

    fn builder(capacity: usize) -> Self::Builder;
}

/// Builds a decoded frame payload from its TLV packets.
pub trait FramePayloadBuilder {
    type Payload;

    fn push(&mut self, packet: TlvPacket<'_>);
    fn finish(self) -> error::Result<Self::Payload>;
}

impl<T: Tlv> FramePayload for Vec<T> {
    type Builder = Self;

    fn builder(capacity: usize) -> Self::Builder {
        Vec::with_capacity(capacity)
    }
}

impl<T: Tlv> FramePayloadBuilder for Vec<T> {
    type Payload = Self;

    fn push(&mut self, packet: TlvPacket<'_>) {
        if let Ok(tlv) = T::from_packet(packet) {
            self.push(tlv);
        }
    }

    fn finish(self) -> error::Result<Self::Payload> {
        Ok(self)
    }
}

impl<T> TlvReader for Frame<T>
where
    T: FramePayload,
{
    fn read<R: BufRead>(buf: &mut R) -> error::Result<Self> {
        let header = FrameHeader::read(buf)?;

        let mut payload = T::builder(header.num_tlvs as usize);

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

        Ok(Frame {
            header,
            payload: payload.finish()?,
        })
    }
}

/// Borrowed TLV header and payload passed to [`Tlv::from_packet`].
pub struct TlvPacket<'a> {
    pub header: &'a TlvHeader,
    pub payload: &'a [u8],
}
