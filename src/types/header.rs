use std::io::BufRead;

use crate::{MAGIC, TlvReader, error};

pub struct FrameHeader {
    pub version: u32,
    pub total_packet_len: u32,
    pub platform: u32,
    pub frame_number: u32,
    pub time_cpu_cycles: u32,
    pub num_detected_obj: u32,
    pub num_tlvs: u32,
    pub sub_frame_number: u32,
}

pub struct TlvHeader {
    pub r#type: u32,
    pub length: u32,
}

impl FrameHeader {
    pub const LENGTH: usize = 8 + 4 * 8; // 8 bytes for magic_word + 8 u32 fields
}

impl TlvReader for FrameHeader {
    fn read<R: BufRead>(buf: &mut R) -> error::Result<Self> {
        let magic_word: [u8; 8] = TlvReader::read(buf)?;

        if magic_word != MAGIC {
            return Err(error::TlvError::InvalidMagicWord);
        }

        let version = TlvReader::read(buf)?;
        let total_packet_len = TlvReader::read(buf)?;
        let platform = TlvReader::read(buf)?;
        let frame_number = TlvReader::read(buf)?;
        let time_cpu_cycles = TlvReader::read(buf)?;
        let num_detected_obj = TlvReader::read(buf)?;
        let num_tlvs = TlvReader::read(buf)?;
        let sub_frame_number = TlvReader::read(buf)?;

        Ok(FrameHeader {
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
    fn read<R: BufRead>(buf: &mut R) -> error::Result<Self> {
        let r#type = TlvReader::read(buf)?;
        let length = TlvReader::read(buf)?;

        Ok(TlvHeader { r#type, length })
    }
}
