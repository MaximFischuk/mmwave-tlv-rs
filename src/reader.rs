use std::io::{BufRead, Read};

use crate::{MAGIC, Tlv, TlvReader, error, types::Frame};

const BUFFER_SIZE: usize = 64 * 1024;

pub struct FrameStreamReader<R> {
    reader: R,
    buffer: Vec<u8>,
}

impl<R: BufRead> FrameStreamReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::with_capacity(BUFFER_SIZE),
        }
    }

    pub fn read_frame<T>(&mut self) -> error::Result<Option<Frame<T>>>
    where
        T: Tlv + TlvReader,
    {
        loop {
            if let Some(offset) = self
                .buffer
                .windows(MAGIC.len())
                .position(|window| window == MAGIC)
            {
                self.buffer.drain(..offset);
                return Frame::read(self).map(Some);
            }

            let retained = self.buffer.len().min(MAGIC.len() - 1);
            if self.buffer.len() > retained {
                self.buffer.drain(..self.buffer.len() - retained);
            }

            let mut chunk = [0; 4096];
            let bytes_read = self.reader.read(&mut chunk)?;
            if bytes_read == 0 {
                self.buffer.clear();
                return Ok(None);
            }

            self.buffer.extend_from_slice(&chunk[..bytes_read]);
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
        self.buffer.drain(..bytes_read);
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
            self.buffer.drain(..amount.min(self.buffer.len()));
        }
    }
}
