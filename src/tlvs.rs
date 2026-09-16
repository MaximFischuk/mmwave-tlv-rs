use crate::TlvDecode;

pub struct PointCloud {
    pub points: Vec<Point>,
}

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub doppler: f32,
}

impl TlvDecode for Point {
    fn decode(bytes: &[u8]) -> crate::error::Result<Self> {
        if bytes.len() != 16 {
            return Err(crate::error::TlvError::DecodeError);
        }
        let x = TlvDecode::decode(&bytes[0..4])?;
        let y = TlvDecode::decode(&bytes[4..8])?;
        let z = TlvDecode::decode(&bytes[8..12])?;
        let doppler = TlvDecode::decode(&bytes[12..16])?;

        Ok(Point { x, y, z, doppler })
    }
}
