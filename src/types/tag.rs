use crate::TlvReader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tag(u32);

impl Tag {
    /// Represents an unknown or uninitialized tag.
    /// It used as a placeholder for enum variants that also implements the `Tlv` trait.
    pub const UNKNOWN: Tag = Tag(0);

    /// Creates a new `Tag` with a compile-time constant value.
    ///
    /// # Panics
    ///
    /// Panics if `VALUE` is zero.
    pub const fn const_new<const VALUE: u32>() -> Self {
        assert!(VALUE != 0, "Tag value must be non-zero");
        Tag(VALUE)
    }
}

impl TryFrom<u32> for Tag {
    type Error = crate::error::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Err(crate::error::Error::NonZeroTagValue),
            _ => Ok(Tag(value)),
        }
    }
}

impl TlvReader for Tag {
    fn read<R: std::io::prelude::BufRead>(buf: &mut R) -> crate::error::Result<Self> {
        let value = u32::read(buf)?;
        Tag::try_from(value)
    }
}
