use crate::font::FontType::Shadow;
use crate::{feature, font, Color, EventPaintTraverse};

feature!(Watermark => Watermark::paint_traverse);

impl Watermark {
    pub fn paint_traverse(_: &mut EventPaintTraverse) {
        font::text(
            "schiller-hook-rs (tasty)",
            2.0,
            2.0,
            Shadow,
            Color::new_rgb(255, 255, 255),
        );
    }
}
