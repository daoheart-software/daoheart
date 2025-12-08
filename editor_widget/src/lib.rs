use std::io::Read;

use buffer::Buffer;
use cosmic_text::{Attrs, FontSystem, Metrics};
use global::{FONT_SYSTEM, SWASH_CACHE};
use iced::{Background, Color, Element, Event, Length, Rectangle, Size};
use iced_core::{
    Clipboard, Layout, Shell, Widget,
    layout::{Limits, Node},
    mouse::Cursor,
    renderer::{Quad, Style},
    widget::Tree,
};

pub struct EditorWidget {
    buffer: Buffer,
    cosmic_buffer: cosmic_text::Buffer,
}

impl EditorWidget {
    pub fn new(reader: impl Read) -> Self {
        Self {
            buffer: Buffer::from_reader(reader).unwrap(),
            cosmic_buffer: cosmic_text::Buffer::new_empty(Metrics::new(24.0, 24.0)),
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for EditorWidget
where
    Renderer: iced_core::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(&mut self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> Node {
        self.build_buffer(limits.max());
        Node::new(limits.max())
    }

    fn draw(
        &self,
        _tree: &Tree,
        _renderer: &mut Renderer,
        _theme: &Theme,
        _style: &Style,
        _layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        FONT_SYSTEM.with_borrow_mut(|font_system| {
            SWASH_CACHE.with_borrow_mut(|cache| {
                self.cosmic_buffer.draw(
                    font_system,
                    cache,
                    cosmic_text::Color::rgb(255, 255, 255),
                    |x, y, w, h, color| {
                        _renderer.fill_quad(
                            Quad {
                                bounds: Rectangle {
                                    x: x as f32,
                                    y: y as f32,
                                    width: w as f32,
                                    height: h as f32,
                                },
                                ..Default::default()
                            },
                            Background::Color(Color {
                                r: color.r() as f32,
                                g: color.g() as f32,
                                b: color.b() as f32,
                                a: color.a() as f32,
                            }),
                        );
                    },
                );
            })
        });
    }

    fn update(
        &mut self,
        _tree: &mut Tree,
        _event: &Event,
        _layout: Layout<'_>,
        _cursor: Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
    }
}

impl EditorWidget {
    fn build_buffer(&mut self, size: Size<f32>) {
        FONT_SYSTEM.with_borrow_mut(|font_system| {
            self.do_build_buffer(font_system, size);
        })
    }

    fn do_build_buffer(&mut self, font_system: &mut FontSystem, size: Size<f32>) {
        let mut text = String::new();
        for line in self.buffer.lines_at(0) {
            text.push_str(line.as_str().unwrap_or(""));
            text.push('\n');
        }

        self.cosmic_buffer
            .set_size(font_system, Some(size.width), None);

        self.cosmic_buffer.set_text(
            font_system,
            &text,
            &Attrs::new(),
            cosmic_text::Shaping::Advanced,
            None,
        );

        self.cosmic_buffer.shape_until_scroll(font_system, true);
    }
}

impl<'a, Message, Theme, Renderer> From<EditorWidget> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced_core::Renderer,
{
    fn from(widget: EditorWidget) -> Self {
        Element::new(widget)
    }
}
