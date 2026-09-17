use crate::{TlvReader, error};

impl<const N: usize> TlvReader for [u8; N] {
    fn read(bytes: &[u8]) -> error::Result<Self> {
        if bytes.len() != N {
            return Err(error::TlvError::InvalidArrayLength);
        }
        let mut array = [0u8; N];
        array.copy_from_slice(&bytes[..N]);
        Ok(array)
    }
}

impl<T: TlvReader> TlvReader for Vec<T> {
    fn read(bytes: &[u8]) -> error::Result<Self> {
        let mut vec = Vec::new();
        if bytes.len() % std::mem::size_of::<T>() != 0 {
            return Err(error::TlvError::InvalidArrayLength);
        }

        for chunk in bytes.chunks(std::mem::size_of::<T>()) {
            let item = T::read(chunk)?;
            vec.push(item);
        }
        Ok(vec)
    }
}

macro_rules! impl_tlv_for_primitive {
    ($($primitive:ty),+ $(,)?) => {
        $(
            impl TlvReader for $primitive {
                fn read(bytes: &[u8]) -> error::Result<Self> {
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
