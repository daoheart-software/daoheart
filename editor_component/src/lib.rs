use crate::view::EditorView;

mod view;
mod widget;

pub fn editor(init_file_name: String) -> EditorView {
    EditorView::new_with_init_file_name(init_file_name)
}
