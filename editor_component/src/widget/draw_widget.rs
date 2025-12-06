use xilem::{
    Affine, Color, FontWeight,
    masonry::{
        core::{BrushIndex, render_text},
        parley::{Alignment, AlignmentOptions, StyleProperty},
    },
};

pub fn draw_widget(
    _buffer: &mut buffer::Buffer,
    _ctx: &mut xilem::masonry::core::PaintCtx<'_>,
    _scene: &mut xilem::masonry::vello::Scene,
    _layout_cx: &mut xilem::masonry::parley::LayoutContext<BrushIndex>,
    _font_cx: &mut xilem::masonry::parley::FontContext,
) {
    let txt = _buffer
        .lines_at(0)
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    let mut builder = _layout_cx.ranged_builder(_font_cx, txt.as_str(), 1.0, true);
    builder.push_default(StyleProperty::FontSize(16.0));
    builder.push(StyleProperty::FontWeight(FontWeight::new(600.0)), 0..4);

    let mut layout = builder.build(txt.as_str());

    let width = Some(_ctx.bounding_rect().width() as f32);
    layout.break_all_lines(width);
    layout.align(width, Alignment::Start, AlignmentOptions::default());

    render_text(
        _scene,
        Affine::IDENTITY,
        &layout,
        &[Color::WHITE.into()],
        true,
    );
}
