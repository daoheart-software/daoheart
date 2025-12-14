use cosmic_text::SubpixelBin;
use iced::{
    Background, Border, Color, Rectangle, advanced::graphics::text::Paragraph, widget::text::Rich,
};
use iced_core::{renderer::Quad, text::Paragraph};

use super::EditorWidget;

impl EditorWidget<'_> {
    pub(super) fn render_text_buffer(
        &self,
        renderer: &mut impl iced_core::text::Renderer,
        layout: iced_core::Layout<'_>,
        font_system: &mut cosmic_text::FontSystem,
        swash_cache: &mut cosmic_text::SwashCache,
        theme: &iced::Theme,
    ) {
        Paragraph::new();
        self.buffer.draw(
            font_system,
            swash_cache,
            cosmic_text::Color::rgb(200, 0, 10),
            |x, y, w, h, color| {
                if (y > layout.bounds().height.ceil() as i32) {
                    return;
                }
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
    }
}
