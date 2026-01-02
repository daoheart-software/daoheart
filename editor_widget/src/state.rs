use std::{path::Path, sync::Arc};

#[derive(Clone, Debug)]
pub struct EditorState {
    /// The path of the file being read/written
    ///
    /// `None` if the file is in memory only.
    pub(super) path: Option<Arc<Path>>,
    pub(super) scroll_px: f32,
}

pub struct EditorStateInit {
    pub path: Option<Arc<Path>>,
    pub scroll_px: f32,
}

impl EditorState {
    pub fn new(EditorStateInit { path, scroll_px }: EditorStateInit) -> Self {
        Self { path, scroll_px }
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
}
