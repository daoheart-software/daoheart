use std::cell::RefCell;
use xilem::masonry::core::BrushIndex;
use xilem::masonry::parley::FontContext;
use xilem::masonry::parley::LayoutContext;

thread_local! {
    pub static FONT_CTX: RefCell<FontContext> = RefCell::new(FontContext::new());
    pub static LAYOUT_CTX: RefCell<LayoutContext<BrushIndex>> = RefCell::new(LayoutContext::new());
}
