use crate::internal_state::InternalState;
use crate::paragraph_cache::TextConfig;
use crate::{message::EditorMessage, state::EditorState};
use iced::advanced::graphics::text::Paragraph;
use iced::{Element, Length};
use iced_core::alignment::Vertical;
use iced_core::mouse::Cursor;
use iced_core::renderer::Quad;
use iced_core::text::{Alignment, LineHeight, Paragraph as _, Shaping, Wrapping};
use iced_core::widget::Tree;
use iced_core::widget::tree::{State, Tag};
use iced_core::{Border, Clipboard, Layout, Point, Rectangle, Shadow, Size, Widget, layout::Node};
use std::ops::Add;

pub mod message;

pub mod internal_state;
mod paragraph_cache;
pub mod state;

pub struct EditorWidget<'a> {
    state: &'a EditorState,
}
impl<'a> EditorWidget<'a> {
    pub fn new(state: &'a EditorState) -> Self {
        Self { state }
    }
}

impl<'a> EditorWidget<'a> {
    pub(crate) fn text_config(&self, width: f32) -> TextConfig {
        TextConfig {
            bounds: Size {
                width,
                height: f32::INFINITY,
            },
            size: 16.into(),
            align_x: Alignment::Left,
            wrapping: Wrapping::Word,
            shaping: Shaping::Advanced,
            align_y: Vertical::Top,
            font: Default::default(),
            line_height: LineHeight::Relative(1.0),
        }
    }
}

impl<'a, Renderer: iced_core::text::Renderer<Paragraph = Paragraph>>
    Widget<EditorMessage, iced::Theme, Renderer> for EditorWidget<'a>
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn state(&self) -> State {
        let state = self
            .state
            .path
            .as_ref()
            .map(|path| {
                InternalState::new(path)
                    .inspect_err(|err| eprintln!("Error opening file: {:?}", err))
                    .ok()
            })
            .flatten()
            .unwrap_or(Default::default());

        State::new(state)
    }

    fn tag(&self) -> Tag {
        Tag::of::<InternalState>()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        _renderer: &Renderer,
        limits: &iced_core::layout::Limits,
    ) -> Node {
        let state = tree.state.downcast_ref::<InternalState>();
        let mut paragraph_cache = state.paragraph_cache.borrow_mut();

        paragraph_cache.layout_everything(&state.buffer, self.text_config(limits.max().width));

        paragraph_cache.scroll(
            self.state.scroll_px,
            limits.max().height,
            &state.buffer,
            self.text_config(limits.max().width),
        );

        Node::new(limits.max())
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &iced::Theme,
        _style: &iced_core::renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<InternalState>();
        let mut paragraph_cache = state.paragraph_cache.borrow_mut();

        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border::default(),
                shadow: Shadow::default(),
                snap: true,
            },
            theme.palette().background,
        );

        for paragraph in paragraph_cache.paragraphs() {
            renderer.fill_paragraph(
                paragraph.paragraph,
                Point {
                    x: 0.0,
                    y: paragraph.bounds.top,
                },
                theme.palette().text,
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: paragraph.paragraph.bounds().width,
                    height: paragraph.paragraph.bounds().height,
                },
            )
        }
    }
}

impl<'a, Renderer: iced_core::text::Renderer<Paragraph = Paragraph>> From<EditorWidget<'a>>
    for Element<'a, EditorMessage, iced::Theme, Renderer>
{
    fn from(value: EditorWidget<'a>) -> Self {
        Element::new(value)
    }
}
