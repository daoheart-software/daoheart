use cosmic_text::{Align, Attrs, Buffer, Metrics, Shaping};
use global::{FONT_SYSTEM, SWASH_CACHE};
use iced::{Background, Border, Color, Element, Length, Rectangle};
use iced_core::{Widget, layout::Node, renderer::Quad};

use crate::{message::EditorMessage, state::EditorState};

pub mod message;
pub mod state;

pub struct EditorWidget<'a> {
    _state: &'a EditorState,
    buffer: Buffer,
}
impl<'a> EditorWidget<'a> {
    pub fn new(state: &'a EditorState) -> Self {
        FONT_SYSTEM.with_borrow_mut(|fs| {
            let mut buffer = Buffer::new(fs, Metrics::new(24., 24.));
            buffer.set_text(
                fs,
                &state.buffer().to_string(),
                &Attrs::new(),
                Shaping::Advanced,
                Some(Align::Left),
            );
            Self {
                _state: state,
                buffer,
            }
        })
    }
}

impl<'a, Theme, Renderer: iced_core::Renderer> Widget<EditorMessage, Theme, Renderer>
    for EditorWidget<'a>
{
    fn size(&self) -> iced::Size<iced::Length> {
        iced::Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &mut self,
        _tree: &mut iced_core::widget::Tree,
        _renderer: &Renderer,
        limits: &iced_core::layout::Limits,
    ) -> iced_core::layout::Node {
        FONT_SYSTEM.with_borrow_mut(|fs| {
            self.buffer
                .set_size(fs, Some(limits.max().width), Some(limits.max().height));
            self.buffer.shape_until_scroll(fs, true);
        });
        Node::new(limits.max())
    }

    fn draw(
        &self,
        _tree: &iced_core::widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced_core::renderer::Style,
        _layout: iced_core::Layout<'_>,
        _cursor: iced_core::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        FONT_SYSTEM.with_borrow_mut(|fs| {
            SWASH_CACHE.with_borrow_mut(|sc| {
                self.buffer.draw(
                    fs,
                    sc,
                    cosmic_text::Color::rgb(200, 0, 10),
                    |x, y, w, h, color| {
                        renderer.fill_quad(
                            Quad {
                                bounds: Rectangle {
                                    x: x as f32,
                                    y: y as f32,
                                    width: w as f32,
                                    height: h as f32,
                                },
                                border: Border::default(),
                                ..Default::default()
                            },
                            Background::Color(Color::from_rgba8(
                                color.r(),
                                color.g(),
                                color.b(),
                                color.a() as f32,
                            )),
                        );
                    },
                );
            })
        })
    }
}

impl<'a, Theme, Renderer: iced_core::Renderer> From<EditorWidget<'a>>
    for Element<'a, EditorMessage, Theme, Renderer>
{
    fn from(value: EditorWidget<'a>) -> Self {
        Element::new(value)
    }
}
