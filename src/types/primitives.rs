use crate::{TlvReader, error};
use std::io::BufRead;

impl<const N: usize> TlvReader for [u8; N] {
    fn read<R: BufRead>(buf: &mut R) -> error::Result<Self> {
        let mut array = [0u8; N];
        buf.read_exact(&mut array)?;
        Ok(array)
    }
}

impl<T: TlvReader> TlvReader for Vec<T> {
    fn read<R: BufRead>(buf: &mut R) -> error::Result<Self> {
        let mut vec = Vec::new();
        loop {
            match T::read(buf) {
                Ok(item) => vec.push(item),
                Err(_) => break,
            }
        }
        Ok(vec)
    }
}

macro_rules! impl_tlv_for_primitive {
    ($($primitive:ty),+ $(,)?) => {
        $(
            impl TlvReader for $primitive {
                fn read<R: BufRead>(buf: &mut R) -> error::Result<Self> {
                    let mut array = [0u8; std::mem::size_of::<Self>()];
                    buf.read_exact(&mut array)?;
                    Ok(<$primitive>::from_le_bytes(array))
                }
            }
        )+
    };
}

impl_tlv_for_primitive!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,
);

macro_rules! impl_tlv_for_primitive_array {
    ($($primitive:ty),+ $(,)?) => {
        $(
            impl<const N: usize> TlvReader for [$primitive; N] {
                fn read<R: BufRead>(buf: &mut R) -> error::Result<Self> {
                    let mut array: [$primitive; N] = [0 as $primitive; N];
                    for i in 0..N {
                        array[i] = <$primitive>::read(buf)?;
                    }
                    Ok(array)
                }
            }
        )+
    };
}

// Exclude u8 from primitive arrays as it is handled separately
impl_tlv_for_primitive_array!(
    u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,
);
