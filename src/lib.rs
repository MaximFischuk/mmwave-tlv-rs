extern crate self as titlv;

pub mod error;
pub mod reader;
pub mod tlvs;
pub mod types;

pub mod prelude;

pub use titlv_derive::{Tlv, TlvReader};

use std::io::BufRead;

use crate::types::TlvPacket;

pub const MAGIC: [u8; 8] = [0x02, 0x01, 0x04, 0x03, 0x06, 0x05, 0x08, 0x07];

pub trait Tlv: Sized {
    fn from_packet(packet: TlvPacket<'_>) -> error::Result<Self>;
}

pub trait TlvReader: Sized {
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
