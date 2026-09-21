#![forbid(unsafe_code, unused_crate_dependencies, unused_imports, dead_code)]
//! Decode Texas Instruments mmWave radar packets encoded as TLVs.
//!
//! [`reader::FrameStreamReader`] scans a byte stream for the TI magic word and
//! decodes complete frames. Use [`tlvs::StandardTlv`] to decode the TLV types
//! supported by this crate.
//!
//! # Example
//!
//! ```no_run
//! use std::{fs::File, io::BufReader};
//!
//! use titlv::prelude::*;
//!
//! fn main() -> Result<()> {
//!     let file = File::open("radar.bin")?;
//!     let mut reader = FrameStreamReader::new(BufReader::new(file));
//!
//!     while let Some(frame) = reader.read_frame::<Vec<StandardTlv>>()? {
//!         println!("frame {} contains {} supported TLVs", frame.header.frame_number, frame.payload.len());
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # Custom TLVs
//!
//! Use the prelude to derive decoders for TLVs emitted by your radar firmware.
//!
//! ```no_run
//! use titlv::prelude::*;
//!
//! #[derive(TlvReader)]
//! struct Temperature {
//!     degrees_celsius: f32,
//! }
//!
//! #[derive(Tlv)]
//! #[tlv(type = 42)]
//! struct TemperatureTlv {
//!     degrees_celsius: f32,
//! }
//! ```

extern crate self as titlv;

pub mod error;
pub mod reader;
pub mod tlvs;
pub mod types;

pub mod prelude;

pub use titlv_derive::{Tlv, TlvReader};

use std::io::BufRead;

use crate::types::{Tag, TlvPacket};

/// Magic word that marks the beginning of a TI radar frame.
pub const MAGIC: [u8; 8] = [0x02, 0x01, 0x04, 0x03, 0x06, 0x05, 0x08, 0x07];

/// Decodes a typed value from a complete TLV packet.
///
/// Implement this trait with `#[derive(Tlv)]` for a tagged struct or an enum
/// that dispatches among several tagged structs.
pub trait Tlv: Sized {
    const TYPE: Tag;

    /// Decodes this value from a TLV header and payload.
    fn from_packet(packet: TlvPacket<'_>) -> error::Result<Self>;
}

/// Decodes a value from little-endian bytes.
///
/// Implement this trait with `#[derive(TlvReader)]` for structs whose fields
/// are laid out consecutively in the payload.
pub trait TlvReader: Sized {
    /// Reads one value from the supplied buffered byte stream.
    fn read<R: BufRead>(buf: &mut R) -> error::Result<Self>;
}

// Byte offset
// 0                                                                      totalPacketLen
// │                                                                              │
// ▼                                                                              ▼
// ┌────────────────────────────── Frame header: 40 bytes ─────────────────────────────┐
// │                                                                                   │
// │  Magic word       8 bytes                                                         │
// │  Version          4 bytes                                                         │
// │  Total length     4 bytes                                                         │
// │  Platform         4 bytes                                                         │
// │  Frame number     4 bytes                                                         │
// │  CPU timestamp    4 bytes                                                         │
// │  Num objects      4 bytes                                                         │
// │  Num TLVs         4 bytes                                                         │
// │  Sub-frame number 4 bytes                                                         │
// │                                                                                   │
// └───────────────────────────────────────────────────────────────────────────────────┘
// ┌────────────────────────────── TLV 0 ───────────────────────────────────────────────┐
// │ Type       4 bytes                                                                 │
// │ Length     4 bytes   ← includes these 8 TLV-header bytes                           │
// │ Payload    Length - 8 bytes                                                        │
// └────────────────────────────────────────────────────────────────────────────────────┘
// ┌────────────────────────────── TLV 1 ───────────────────────────────────────────────┐
// │ Type       4 bytes                                                                 │
// │ Length     4 bytes                                                                 │
// │ Payload    Length - 8 bytes                                                        │
// └────────────────────────────────────────────────────────────────────────────────────┘
//                                       ...
// ┌────────────────────────────── TLV N ───────────────────────────────────────────────┐
// │ Type       4 bytes                                                                 │
// │ Length     4 bytes                                                                 │
// │ Payload    Length - 8 bytes                                                        │
// └────────────────────────────────────────────────────────────────────────────────────┘
// ┌────────────────────────────── Padding ─────────────────────────────────────────────┐
// │ Optional zero bytes; packet is normally padded to a 32-byte boundary               │
// └────────────────────────────────────────────────────────────────────────────────────┘
