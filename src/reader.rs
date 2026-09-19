use std::io::{BufRead, Read};

use bytes::{Buf, BufMut, BytesMut};
#[cfg(feature = "async")]
use tokio::io::{AsyncBufRead, AsyncReadExt};

use crate::{
    MAGIC, Tlv, TlvReader, error,
    types::{Frame, FrameHeader},
};

const BUFFER_SIZE: usize = 64 * 1024;

/// Byte range containing [`FrameHeader::total_packet_len`].
const FRAME_LEN_RANGE: std::ops::Range<usize> = 4..8;

/// Reads complete TI radar frames from a buffered byte stream.
///
/// The reader discards bytes before the next [`MAGIC`] sequence and retains
/// unread bytes between calls to [`Self::read_frame`].
pub struct FrameStreamReader<R> {
    reader: R,
    buffer: BytesMut,
}

impl<R: BufRead> FrameStreamReader<R> {
    /// Creates a frame reader over a buffered byte stream.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: BytesMut::with_capacity(BUFFER_SIZE),
        }
    }

    /// Reads the next frame and decodes TLVs as `T`.
    ///
    /// Returns `Ok(None)` after the underlying stream reaches end of input.
    /// TLVs rejected by `T::from_packet` are omitted from the frame payload.
    pub fn read_frame<T>(&mut self) -> error::Result<Option<Frame<T>>>
    where
        T: Tlv,
    {
        loop {
            if consume_magic(&mut self.buffer) {
                self.read_until(FrameHeader::LENGTH - MAGIC.len())?;
                let frame_len = frame_data_len(&self.buffer)?;
                self.read_until(frame_len)?;
                let mut frame_data = std::io::Cursor::new(self.buffer.split_to(frame_len));
                return Frame::read(&mut frame_data).map(Some);
            }

            if !self.read_more()? {
                self.buffer.clear();
                return Ok(None);
            }
        }
    }

    fn read_until(&mut self, length: usize) -> std::io::Result<()> {
        while self.buffer.len() < length {
            if !self.read_more()? {
                return Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof));
            }
        }
        Ok(())
    }

    fn read_more(&mut self) -> std::io::Result<bool> {
        let mut chunk = [0; 4096];
        let bytes_read = self.reader.read(&mut chunk)?;
        self.buffer.put_slice(&chunk[..bytes_read]);
        Ok(bytes_read != 0)
    }
}

impl<R: BufRead> Read for FrameStreamReader<R> {
    fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
        if self.buffer.is_empty() {
            return self.reader.read(output);
        }

        let bytes_read = output.len().min(self.buffer.len());
        output[..bytes_read].copy_from_slice(&self.buffer[..bytes_read]);
        self.buffer.advance(bytes_read);
        Ok(bytes_read)
    }
}

impl<R: BufRead> BufRead for FrameStreamReader<R> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        if self.buffer.is_empty() {
            self.reader.fill_buf()
        } else {
            Ok(&self.buffer)
        }
    }

    fn consume(&mut self, amount: usize) {
        if self.buffer.is_empty() {
            self.reader.consume(amount);
        } else {
            self.buffer.advance(amount.min(self.buffer.len()));
        }
    }
}

/// Reads complete TI radar frames from an asynchronous buffered byte stream.
///
/// The reader awaits data from the underlying stream and retains unread bytes
/// between calls to [`Self::read_frame`].
#[cfg(feature = "async")]
pub struct AsyncFrameStreamReader<R> {
    reader: R,
    buffer: BytesMut,
}

#[cfg(feature = "async")]
impl<R: AsyncBufRead + Unpin> AsyncFrameStreamReader<R> {
    /// Creates an asynchronous frame reader over a buffered byte stream.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: BytesMut::with_capacity(BUFFER_SIZE),
        }
    }

    /// Reads and decodes the next frame within the async runtime.
    ///
    /// Returns `Ok(None)` after the underlying stream reaches end of input.
    /// TLVs rejected by `T::from_packet` are omitted from the frame payload.
    pub async fn read_frame<T>(&mut self) -> error::Result<Option<Frame<T>>>
    where
        T: Tlv,
    {
        loop {
            if consume_magic(&mut self.buffer) {
                self.read_until(FrameHeader::LENGTH - MAGIC.len()).await?;
                let frame_len = frame_data_len(&self.buffer)?;
                self.read_until(frame_len).await?;
                let mut frame_data = std::io::Cursor::new(self.buffer.split_to(frame_len));
                return Frame::read(&mut frame_data).map(Some);
            }

            if !self.read_more().await? {
                self.buffer.clear();
                return Ok(None);
            }
        }
    }

    async fn read_until(&mut self, length: usize) -> std::io::Result<()> {
        while self.buffer.len() < length {
            if !self.read_more().await? {
                return Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof));
            }
        }
        Ok(())
    }

    async fn read_more(&mut self) -> std::io::Result<bool> {
        let mut chunk = [0; 4096];
        let bytes_read = self.reader.read(&mut chunk).await?;
        self.buffer.put_slice(&chunk[..bytes_read]);
        Ok(bytes_read != 0)
    }
}

fn consume_magic(buffer: &mut BytesMut) -> bool {
    if let Some(offset) = buffer
        .windows(MAGIC.len())
        .position(|window| window == MAGIC)
    {
        buffer.advance(offset + MAGIC.len());
        return true;
    }

    let retained = buffer.len().min(MAGIC.len() - 1);
    buffer.advance(buffer.len() - retained);
    false
}

fn frame_data_len(buffer: &[u8]) -> error::Result<usize> {
    let total_packet_len = u32::from_le_bytes(
        buffer
            .get(FRAME_LEN_RANGE)
            .ok_or(error::TlvError::IncompleteFrameHeader)?
            .try_into()
            .map_err(|_| error::TlvError::InvalidFrameHeader)?,
    ) as usize;

    if total_packet_len < FrameHeader::LENGTH {
        return Err(error::TlvError::FrameLengthSmallerThanHeader);
    }

    Ok(total_packet_len - MAGIC.len())
}

#[cfg(all(test, feature = "async"))]
mod tests {
    use super::*;
    use tokio::io::{AsyncWriteExt, BufReader};

    #[derive(Debug, PartialEq)]
    struct TestTlv(u32);

    impl Tlv for TestTlv {
        fn from_packet(packet: crate::types::TlvPacket<'_>) -> error::Result<Self> {
            if packet.header.r#type != 7 {
                return Err(error::TlvError::UnexpectedTlvType);
            }

            Ok(Self(u32::from_le_bytes(
                packet.payload.try_into().expect("test payload is a u32"),
            )))
        }
    }

    #[tokio::test]
    async fn reads_fragmented_frame_without_blocking() {
        let mut bytes = vec![0, 1, 2];
        bytes.extend_from_slice(&MAGIC);
        for value in [1u32, 52, 0, 9, 0, 1, 1, 0] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes.extend_from_slice(&7u32.to_le_bytes());
        bytes.extend_from_slice(&4u32.to_le_bytes());
        bytes.extend_from_slice(&42u32.to_le_bytes());

        let (mut writer, reader) = tokio::io::duplex(BUFFER_SIZE);
        writer.write_all(&bytes[..11]).await.unwrap();
        writer.write_all(&bytes[11..]).await.unwrap();
        drop(writer);

        let mut reader = AsyncFrameStreamReader::new(BufReader::new(reader));
        let frame = reader.read_frame::<TestTlv>().await.unwrap().unwrap();

        assert_eq!(frame.header.frame_number, 9);
        assert_eq!(frame.payload, vec![TestTlv(42)]);
        assert!(reader.read_frame::<TestTlv>().await.unwrap().is_none());
    }
}
