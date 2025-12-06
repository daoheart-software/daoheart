use std::{
    fs::File,
    io::{BufReader, Read},
};

use xilem::{
    Pod, ViewCtx,
    core::{MessageResult, View, ViewMarker},
};

use crate::widget::EditorWidget;

pub struct EditorView {
    init_file_name: Option<String>,
}

impl EditorView {
    pub fn new_with_init_file_name(init_file_name: String) -> Self {
        Self {
            init_file_name: Some(init_file_name),
        }
    }
}

pub struct EditorViewState {}

impl ViewMarker for EditorView {}
impl<State, Action> View<State, Action, ViewCtx> for EditorView {
    type Element = Pod<EditorWidget>;
    type ViewState = EditorViewState;

    fn build(
        &self,
        _ctx: &mut ViewCtx,
        _app_state: &mut State,
    ) -> (Self::Element, Self::ViewState) {
        let pod = Pod::new(match self.init_file_name {
            Some(ref file_name) => {
                let reader = BufReader::new(File::open(file_name).unwrap());
                EditorWidget::from_reader(reader).unwrap()
            }
            None => EditorWidget::new(),
        });

        let view_state = EditorViewState {};
        (pod, view_state)
    }

    fn rebuild(
        &self,
        _prev: &Self,
        _view_state: &mut Self::ViewState,
        _ctx: &mut ViewCtx,
        _element: xilem::core::Mut<'_, Self::Element>,
        _app_state: &mut State,
    ) {
    }

    fn teardown(
        &self,
        _view_state: &mut Self::ViewState,
        _ctx: &mut ViewCtx,
        _element: xilem::core::Mut<'_, Self::Element>,
    ) {
    }

    fn message(
        &self,
        _view_state: &mut Self::ViewState,
        _message: &mut xilem::core::MessageContext,
        _element: xilem::core::Mut<'_, Self::Element>,
        _app_state: &mut State,
    ) -> xilem::core::MessageResult<Action> {
        MessageResult::Nop
    }
}
