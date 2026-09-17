use crate::TlvReader;

// #[derive(Decode, Type)]
// #[tlv(type = 1)] # Add implementation of Tlv trait to allow use this struct as a TLV type in TlvPayload
pub struct PointCloud {
    pub points: Vec<Point>,
}

impl crate::Tlv for PointCloud {
    const TYPE: u32 = 1;
}

impl TlvReader for PointCloud {
    fn read(bytes: &[u8]) -> crate::error::Result<Self> {
        let points = Vec::<Point>::read(bytes)?;
        Ok(PointCloud { points })
    }
}

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub doppler: f32,
}

impl TlvReader for Point {
    fn read(bytes: &[u8]) -> crate::error::Result<Self> {
        if bytes.len() != 16 {
            return Err(crate::error::TlvError::InvalidPointLength);
        }
        let x = TlvReader::read(&bytes[0..4])?;
        let y = TlvReader::read(&bytes[4..8])?;
        let z = TlvReader::read(&bytes[8..12])?;
        let doppler = TlvReader::read(&bytes[12..16])?;

        Ok(Point { x, y, z, doppler })
    }
}
