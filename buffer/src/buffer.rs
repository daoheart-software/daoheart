use std::io::{self, Read};

use ropey::Rope;

#[derive(Clone, Debug, Default)]
pub struct Buffer {
    buffer: Rope,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            buffer: Rope::new(),
        }
    }

    pub fn from_reader(reader: impl Read) -> Result<Self, io::Error> {
        Ok(Buffer {
            buffer: Rope::from_reader(reader)?,
        })
    }

    pub fn lines_at(&self, line_idx: usize) -> ropey::iter::Lines<'_> {
        self.buffer.lines_at(line_idx)
    }
}
