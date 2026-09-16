mod decoder;
mod error;
mod tlvs;

pub const MAGIC: [u8; 8] = [0x02, 0x01, 0x04, 0x03, 0x06, 0x05, 0x08, 0x07];

pub struct FrameHeader {
    pub magic_word: [u8; 8],   // Offset 0, 8 bytes
    pub version: u32,          // Offset 8, 4 bytes
    pub total_packet_len: u32, // Offset 12, 4 bytes
    pub platform: u32,         // Offset 16, 4 bytes
    pub frame_number: u32,     // Offset 20, 4 bytes
    pub time_cpu_cycles: u32,  // Offset 24, 4 bytes
    pub num_detected_obj: u32, // Offset 28, 4 bytes
    pub num_tlvs: u32,         // Offset 32, 4 bytes
    pub sub_frame_number: u32, // Offset 36, 4 bytes
}

pub struct Frame<T>
where
    T: Tlv + TlvDecode,
{
    pub header: FrameHeader,
    pub payload: TlvPayload<T>,
}

pub struct TlvPayload<T>
where
    T: Tlv + TlvDecode,
{
    pub header: TlvHeader,
    pub value: T,
}

pub struct TlvHeader {
    pub r#type: u32, // Offset 0, 4 bytes
    pub length: u32, // Offset 4, 4 bytes
}

pub trait Tlv: Sized {
    const TYPE: u32;
    const LENGTH: usize;
}

pub trait TlvDecode: Sized {
    fn decode(bytes: &[u8]) -> error::Result<Self>;
}

impl<const N: usize> TlvDecode for [u8; N] {
    fn decode(bytes: &[u8]) -> error::Result<Self> {
        if bytes.len() != N {
            return Err(error::TlvError::InvalidArrayLength);
        }
        let mut array = [0u8; N];
        array.copy_from_slice(&bytes[..N]);
        Ok(array)
    }
}

impl FrameHeader {
    const LENGTH: usize = 8 + 4 * 8; // 8 bytes for magic_word + 8 u32 fields
}

impl TlvDecode for FrameHeader {
    fn decode(bytes: &[u8]) -> error::Result<Self> {
        if bytes.len() != Self::LENGTH {
            return Err(error::TlvError::InvalidFrameHeaderLength);
        }
        let magic_word = TlvDecode::decode(&bytes[0..8])?;
        let version = TlvDecode::decode(&bytes[8..12])?;
        let total_packet_len = TlvDecode::decode(&bytes[12..16])?;
        let platform = TlvDecode::decode(&bytes[16..20])?;
        let frame_number = TlvDecode::decode(&bytes[20..24])?;
        let time_cpu_cycles = TlvDecode::decode(&bytes[24..28])?;
        let num_detected_obj = TlvDecode::decode(&bytes[28..32])?;
        let num_tlvs = TlvDecode::decode(&bytes[32..36])?;
        let sub_frame_number = TlvDecode::decode(&bytes[36..40])?;

        Ok(FrameHeader {
            magic_word,
            version,
            total_packet_len,
            platform,
            frame_number,
            time_cpu_cycles,
            num_detected_obj,
            num_tlvs,
            sub_frame_number,
        })
    }
}

impl TlvDecode for TlvHeader {
    fn decode(bytes: &[u8]) -> error::Result<Self> {
        if bytes.len() != 8 {
            return Err(error::TlvError::InvalidTlvHeaderLength);
        }
        let r#type = TlvDecode::decode(&bytes[0..4])?;
        let length = TlvDecode::decode(&bytes[4..8])?;

        Ok(TlvHeader { r#type, length })
    }
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
        if T::LENGTH != 0 && bytes.len() != T::LENGTH {
            return Err(error::TlvError::InvalidTlvLength);
        }
        if T::TYPE != header.r#type {
            return Err(error::TlvError::UnexpectedTlvType);
        }
        let value = TlvDecode::decode(&bytes[8..])?;
        Ok(TlvPayload { header, value })
    }
}

macro_rules! impl_tlv_for_primitive {
    ($($primitive:ty),+ $(,)?) => {
        $(
            impl TlvDecode for $primitive {
                fn decode(bytes: &[u8]) -> error::Result<Self> {
                    if bytes.len() != std::mem::size_of::<Self>() {
                        return Err(error::TlvError::InvalidPrimitiveLength);
                    }
                    let mut array = [0u8; std::mem::size_of::<Self>()];
                    array.copy_from_slice(&bytes[..std::mem::size_of::<Self>()]);
                    Ok(<$primitive>::from_le_bytes(array))
                }
            }
        )+
    };
}

impl_tlv_for_primitive!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,
);
