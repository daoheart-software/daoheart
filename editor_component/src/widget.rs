use std::io::{self, Read};

use buffer::Buffer;
use global::{FONT_CTX, LAYOUT_CTX};
use xilem::masonry::{
    accesskit,
    core::{
        AccessCtx, BoxConstraints, LayoutCtx, NoAction, PaintCtx, PropertiesMut, PropertiesRef,
        Widget,
    },
    kurbo::Size,
    vello::Scene,
};

mod draw_widget;

#[derive(Clone, Debug, Default)]
pub struct EditorWidget {
    buffer: Buffer,
}

impl EditorWidget {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
        }
    }

    pub fn from_reader(reader: impl Read) -> Result<Self, io::Error> {
        Ok(Self {
            buffer: Buffer::from_reader(reader)?,
        })
    }
}

impl Widget for EditorWidget {
    type Action
        = NoAction
    where
        Self: Sized;

    fn paint(&mut self, ctx: &mut PaintCtx<'_>, _props: &PropertiesRef<'_>, scene: &mut Scene) {
        LAYOUT_CTX.with_borrow_mut(|layout_cx| {
            FONT_CTX.with_borrow_mut(|font_cx| {
                draw_widget::draw_widget(&mut self.buffer, ctx, scene, layout_cx, font_cx)
            })
        })
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx<'_>,
        _props: &mut PropertiesMut<'_>,
        bc: &BoxConstraints,
    ) -> Size {
        bc.max()
    }

    fn accessibility_role(&self) -> accesskit::Role {
        accesskit::Role::MultilineTextInput
    }

    fn accessibility(
        &mut self,
        _ctx: &mut AccessCtx<'_>,
        _props: &PropertiesRef<'_>,
        _node: &mut accesskit::Node,
    ) {
        // todo
    }

    fn register_children(&mut self, _: &mut xilem::masonry::core::RegisterCtx<'_>) {}
    fn children_ids(&self) -> xilem::masonry::core::ChildrenIds {
        [].into_iter().collect()
    }
}
