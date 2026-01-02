use crate::paragraph_cache::ParagraphCache;
use buffer::Buffer;
use ropey::Rope;
use std::cell::RefCell;
use std::fs::File;
use std::path::Path;

pub(super) struct InternalState {
    pub paragraph_cache: RefCell<ParagraphCache>,
    pub buffer: Buffer,
}

impl InternalState {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let rope = Rope::from_reader(&mut file)?;

        let buffer = Buffer::new(rope);

        Ok(Self {
            paragraph_cache: ParagraphCache::new_with_line_capacity(buffer.lines_count()).into(),
            buffer,
        })
    }
}

impl Default for InternalState {
    fn default() -> Self {
        Self {
            paragraph_cache: ParagraphCache::new_with_line_capacity(0).into(),
            buffer: Buffer::new(Rope::default()),
        }
    }
}
