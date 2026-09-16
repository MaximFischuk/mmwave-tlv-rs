use crate::{TlvDecode, error};

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
