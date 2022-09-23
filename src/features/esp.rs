use crate::font::FontType::Outline;
use crate::{feature, font, Color, EventPaintTraverse, Vec3, INTERFACES};
use std::ffi::CStr;

feature!(ESP => ESP::paint_traverse);

impl ESP {
    pub fn paint_traverse(_: &mut EventPaintTraverse) {
        let interfaces = INTERFACES.get().unwrap();

        for i in 0..interfaces.global_vars.max_clients {
            let entity = interfaces.entity_list.entity(i);
            if let Some(ent) = entity.get() {
                if (ent.health() > 0) {
                    let collidable = ent.collidable();
                    if let Some(col) = collidable.get() {
                        let origin = ent.abs_origin();
                        let mins = col.min().clone();
                        let maxs = col.max().clone();
                        // Define all bounding box edges
                        #[rustfmt::skip]
                    let points = vec![
                        Vec3 {x: mins.x, y: mins.y, z: mins.z},
                        Vec3 {x: mins.x, y: maxs.y, z: mins.z},
                        Vec3 {x: maxs.x, y: maxs.y, z: mins.z},
                        Vec3 {x: maxs.x, y: mins.y, z: mins.z},
                        Vec3 {x: maxs.x, y: maxs.y, z: maxs.z},
                        Vec3 {x: mins.x, y: maxs.y, z: maxs.z},
                        Vec3 {x: mins.x, y: mins.y, z: maxs.z},
                        Vec3 {x: maxs.x, y: mins.y, z: maxs.z}
                    ];

                        // Project all bounding box points to screen
                        let screen_points: Vec<Vec3> = points
                            .iter()
                            .map(|point| {
                                let mut contextualized = point.clone() + origin.clone();
                                let mut projected = contextualized.clone();
                                interfaces
                                    .debug_overlay
                                    .world_to_screen(&mut contextualized, &mut projected);
                                projected
                            })
                            .collect();

                        let mut left = screen_points[0].x;
                        let mut bottom = screen_points[0].y;
                        let mut right = screen_points[0].x;
                        let mut top = screen_points[0].y;

                        // Validate the world_to_screen result
                        for point in screen_points {
                            left = left.min(point.x);
                            bottom = bottom.max(point.y);
                            right = right.max(point.x);
                            top = top.min(point.y);
                        }

                        let x = left;
                        let y = top;
                        let x1 = right;
                        let y1 = bottom;

                        // Draw box outline
                        interfaces
                            .vgui_surface
                            .set_draw_color(Color::new_rgba(0, 0, 0, 255));
                        interfaces.vgui_surface.draw_outlined_rect(
                            (x - 1.0) as i32,
                            (y - 1.0) as i32,
                            (x1 + 1.0) as i32,
                            (y1 + 1.0) as i32,
                        );
                        interfaces.vgui_surface.draw_outlined_rect(
                            (x + 1.0) as i32,
                            (y + 1.0) as i32,
                            (x1 - 1.0) as i32,
                            (y1 - 1.0) as i32,
                        );

                        // Draw box inner
                        interfaces
                            .vgui_surface
                            .set_draw_color(Color::new_rgba(255, 255, 255, 255));
                        interfaces
                            .vgui_surface
                            .draw_outlined_rect(x as i32, y as i32, x1 as i32, y1 as i32);

                        // Draw name
                        unsafe {
                            let mut player_info = core::mem::zeroed();
                            interfaces.engine.player_info(i, &mut player_info);

                            let name = CStr::from_ptr(player_info.name.as_ptr()).to_str().unwrap();
                            let bounds = font::text_bounds(name, Outline);

                            font::text_center(
                                name,
                                x + ((x1 - x) / 2.0),
                                y - bounds.1 as f32,
                                Outline,
                                Color::new_rgba(255, 255, 255, 255),
                            );
                        }
                    }
                }
            }
        }
    }
}
