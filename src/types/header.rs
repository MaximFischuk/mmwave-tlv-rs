use std::io::BufRead;

use crate::{TlvReader, error, types::Tag};

/// Metadata at the start of a TI radar frame, after the magic word.
pub struct FrameHeader {
    /// Firmware version reported by the device.
    pub version: u32,
    /// Total frame length reported by the device.
    pub total_packet_len: u32,
    /// Platform identifier reported by the device.
    pub platform: u32,
    /// Device-assigned sequence number for this frame.
    pub frame_number: u32,
    /// CPU timestamp in device cycles.
    pub time_cpu_cycles: u32,
    /// Number of detected objects reported by the device.
    pub num_detected_obj: u32,
    /// Number of TLVs following this header.
    pub num_tlvs: u32,
    /// Sub-frame number reported by the device.
    pub sub_frame_number: u32,
}

/// Header that precedes one TLV payload.
pub struct TlvHeader {
    /// TLV type identifier.
    pub r#type: Tag,
    /// Length value reported in the TLV header.
    pub length: u32,
}

impl FrameHeader {
    pub const LENGTH: usize = 8 + 4 * 8; // 8 bytes for magic_word + 8 u32 fields
}

impl TlvReader for FrameHeader {
    fn read<R: BufRead>(buf: &mut R) -> error::Result<Self> {
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
        let r#type = Tag::read(buf)?;
        let length = TlvReader::read(buf)?;

        Ok(TlvHeader { r#type, length })
    }
}
