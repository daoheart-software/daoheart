use iced::Rectangle;

// A constant array of common page sizes.
// The Rectangle uses the standard 72 DPI logical pixels (width, height)
// that scale automatically in Iced applications.
#[rustfmt::skip]
pub const COMMON_PAGE_SIZES: [(&'static str, Rectangle); 10] = [
    // ISO A Series (Portrait)
    ("A4", Rectangle { x: 0.0, y: 0.0, width: 595.0, height: 842.0 }),
    ("A5", Rectangle { x: 0.0, y: 0.0, width: 420.0, height: 595.0 }),
    ("A3", Rectangle { x: 0.0, y: 0.0, width: 842.0, height: 1190.0 }),
    ("A2", Rectangle { x: 0.0, y: 0.0, width: 1190.0, height: 1684.0 }),
    ("A1", Rectangle { x: 0.0, y: 0.0, width: 1684.0, height: 2384.0 }),
    ("A0", Rectangle { x: 0.0, y: 0.0, width: 2384.0, height: 3370.0 }),
    
    // US Standard Sizes (Portrait)
    ("US Letter", Rectangle { x: 0.0, y: 0.0, width: 612.0, height: 792.0 }),
    ("US Legal", Rectangle { x: 0.0, y: 0.0, width: 612.0, height: 1008.0 }),
    ("US Tabloid", Rectangle { x: 0.0, y: 0.0, width: 792.0, height: 1224.0 }),
    ("US Executive", Rectangle { x: 0.0, y: 0.0, width: 522.0, height: 756.0 }),
];
