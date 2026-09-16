mod decoder;
mod error;

pub const MAGIC: [u8; 8] = [0x02, 0x01, 0x04, 0x03, 0x06, 0x05, 0x08, 0x07];

pub struct Header {
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

/// A generic packet structure containing a header and a payload of type `T`.
/// Each packet start with a magic word defined in the `Header`.
pub struct Packet<T>
where
    T: Tlv,
{
    pub header: Header,
    pub payload: T,
}

pub trait Tlv: Sized {
    const TYPE: Option<u32> = None;
    const LENGTH: usize;

    fn decode(bytes: &[u8]) -> error::Result<Self>;
}

impl<const N: usize> Tlv for [u8; N] {
    const LENGTH: usize = N;

    fn decode(bytes: &[u8]) -> error::Result<Self> {
        if bytes.len() != N {
            return Err(error::TlvError::DecodeError);
        }
        let mut array = [0u8; N];
        array.copy_from_slice(&bytes[..N]);
        Ok(array)
    }
}

impl Tlv for Header {
    const LENGTH: usize = 8 + 4 * 8; // 8 bytes for magic_word + 8 u32 fields

    fn decode(bytes: &[u8]) -> error::Result<Self> {
        if bytes.len() != Self::LENGTH {
            return Err(error::TlvError::DecodeError);
        }
        let magic_word = Tlv::decode(&bytes[0..8])?;
        let version = Tlv::decode(&bytes[8..12])?;
        let total_packet_len = Tlv::decode(&bytes[12..16])?;
        let platform = Tlv::decode(&bytes[16..20])?;
        let frame_number = Tlv::decode(&bytes[20..24])?;
        let time_cpu_cycles = Tlv::decode(&bytes[24..28])?;
        let num_detected_obj = Tlv::decode(&bytes[28..32])?;
        let num_tlvs = Tlv::decode(&bytes[32..36])?;
        let sub_frame_number = Tlv::decode(&bytes[36..40])?;

        Ok(Header {
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

macro_rules! impl_tlv_for_primitive {
    ($($primitive:ty),+ $(,)?) => {
        $(
            impl Tlv for $primitive {
                const LENGTH: usize = std::mem::size_of::<Self>();

                fn decode(bytes: &[u8]) -> error::Result<Self> {
                    if bytes.len() != Self::LENGTH {
                        return Err(error::TlvError::DecodeError);
                    }
                    let mut array = [0u8; Self::LENGTH];
                    array.copy_from_slice(&bytes[..Self::LENGTH]);
                    Ok(<$primitive>::from_le_bytes(array))
                }
            }
        )+
    };
}

impl_tlv_for_primitive!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,
);
