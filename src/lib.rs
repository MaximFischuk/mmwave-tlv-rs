pub mod decoder;
pub mod error;
pub mod tlvs;
pub mod types;

pub mod prelude;

pub const MAGIC: [u8; 8] = [0x02, 0x01, 0x04, 0x03, 0x06, 0x05, 0x08, 0x07];

pub trait Tlv: Sized {
    const TYPE: u32;
}

pub trait TlvReader: Sized {
    fn read(bytes: &[u8]) -> error::Result<Self>;
}
