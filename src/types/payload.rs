use crate::{Tlv, TlvReader, error, types::TlvHeader};

const TLV_HEADER_LENGTH: usize = std::mem::size_of::<TlvHeader>();

// pub struct TlvPayload<T>
// where
//     T: Sized,
// {
//     pub header: TlvHeader,
//     pub value: T,
// }

// impl<T> TlvPayload<T>
// where
//     T: Sized,
// {
//     pub fn new(header: TlvHeader, value: T) -> Self {
//         Self { header, value }
//     }
// }

// impl<T> TryFrom<TlvPacket<'_>> for TlvPayload<T>
// where
//     T: Tlv + Sized,
// {
//     type Error = error::TlvError;

//     fn try_from(packet: TlvPacket<'_>) -> Result<Self, Self::Error> {
//         let payload = T::from_packet(packet)?;
//         Ok(TlvPayload {
//             header: packet.header,
//             value: payload,
//         })
//     }
// }

// impl<T> TlvReader for TlvPayload<T>
// where
//     T: Tlv + TlvReader,
// {
//     fn read<R: std::io::BufRead>(buf: &mut R) -> error::Result<Self> {
//         let header = TlvHeader::read(buf)?;

//         if T::TYPE != header.r#type {
//             return Err(error::TlvError::UnexpectedTlvType);
//         }

//         let value = TlvReader::read(buf)?;
//         Ok(TlvPayload { header, value })
//     }
// }
