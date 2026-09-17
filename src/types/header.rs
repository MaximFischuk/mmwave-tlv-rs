use crate::{TlvReader, error};

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

pub struct TlvHeader {
    pub r#type: u32, // Offset 0, 4 bytes
    pub length: u32, // Offset 4, 4 bytes
}

impl FrameHeader {
    pub const LENGTH: usize = 8 + 4 * 8; // 8 bytes for magic_word + 8 u32 fields
}

impl TlvReader for FrameHeader {
    fn read(bytes: &[u8]) -> error::Result<Self> {
        if bytes.len() != Self::LENGTH {
            return Err(error::TlvError::InvalidFrameHeaderLength);
        }
        let magic_word = TlvReader::read(&bytes[0..8])?;
        let version = TlvReader::read(&bytes[8..12])?;
        let total_packet_len = TlvReader::read(&bytes[12..16])?;
        let platform = TlvReader::read(&bytes[16..20])?;
        let frame_number = TlvReader::read(&bytes[20..24])?;
        let time_cpu_cycles = TlvReader::read(&bytes[24..28])?;
        let num_detected_obj = TlvReader::read(&bytes[28..32])?;
        let num_tlvs = TlvReader::read(&bytes[32..36])?;
        let sub_frame_number = TlvReader::read(&bytes[36..40])?;

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

impl TlvReader for TlvHeader {
    fn read(bytes: &[u8]) -> error::Result<Self> {
        if bytes.len() != 8 {
            return Err(error::TlvError::InvalidTlvHeaderLength);
        }
        let r#type = TlvReader::read(&bytes[0..4])?;
        let length = TlvReader::read(&bytes[4..8])?;

        Ok(TlvHeader { r#type, length })
    }
}
