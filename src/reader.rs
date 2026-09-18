use std::io::{BufRead, Read};

use bytes::{Buf, BufMut, BytesMut};

use crate::{MAGIC, Tlv, TlvReader, error, types::Frame};

const BUFFER_SIZE: usize = 64 * 1024;

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
            if let Some(offset) = self
                .buffer
                .windows(MAGIC.len())
                .position(|window| window == MAGIC)
            {
                self.buffer.advance(offset);
                self.buffer.advance(MAGIC.len());
                return Frame::read(self).map(Some);
            }

            let retained = self.buffer.len().min(MAGIC.len() - 1);
            if self.buffer.len() > retained {
                self.buffer.advance(self.buffer.len() - retained);
            }

            let mut chunk = [0; 4096];
            let bytes_read = self.reader.read(&mut chunk)?;
            if bytes_read == 0 {
                self.buffer.clear();
                return Ok(None);
            }

            self.buffer.put_slice(&chunk[..bytes_read]);
        }
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
