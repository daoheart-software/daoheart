use std::{path::Path, sync::Arc};

use ropey::Rope;

#[derive(Clone, Debug)]
pub struct EditorState {
    /// The path of the file being read/written
    ///
    /// `None` if the file is in memory only.
    path: Option<Arc<Path>>,
    buffer: Rope,
}

impl EditorState {
    pub fn new(path: Option<Arc<Path>>, buffer: Rope) -> Self {
        Self { path, buffer }
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn buffer(&self) -> &Rope {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Rope {
        &mut self.buffer
    }
}
