use crate::font::FontType::Shadow;
use crate::{feature, font, Color, EventPaintTraverse, INTERFACES};

feature!(Watermark => Watermark::paint_traverse);

impl Watermark {
    pub fn paint_traverse(_: &mut EventPaintTraverse) {
        let interfaces = INTERFACES.get().unwrap();

        font::text(
            &format!(
                "powered by rust @ {}fps",
                (1f32 / interfaces.global_vars.absolute_frame_time) as i32
            ),
            2.0,
            2.0,
            Shadow,
            Color::new_rgb(255, 255, 255),
        );
    }
}
