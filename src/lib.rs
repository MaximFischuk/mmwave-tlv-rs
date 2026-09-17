use std::io::BufRead;

pub mod error;
pub mod reader;
pub mod tlvs;
pub mod types;

pub mod prelude;

pub const MAGIC: [u8; 8] = [0x02, 0x01, 0x04, 0x03, 0x06, 0x05, 0x08, 0x07];

pub trait Tlv: Sized {}

pub trait TlvReader: Sized {
    fn read<R: BufRead>(buf: &mut R) -> error::Result<Self>;
}
