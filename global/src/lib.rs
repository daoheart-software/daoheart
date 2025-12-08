use std::cell::RefCell;

use cosmic_text::{FontSystem, SwashCache};

thread_local! {
    pub static FONT_SYSTEM: RefCell<FontSystem> = RefCell::new(FontSystem::new());
    pub static SWASH_CACHE: RefCell<SwashCache> = RefCell::new(SwashCache::new());
}
