use crate::{feature, Color, EventPaintTraverse, Vec3, INTERFACES};

feature!(ESP => ESP::paint_traverse);

impl ESP {
    pub fn paint_traverse(_: &mut EventPaintTraverse) {
        let interfaces = INTERFACES.get().unwrap();

        for i in 0..interfaces.global_vars.max_clients {
            let entity = interfaces.entity_list.entity(i);
            if let Some(ent) = entity.get() {
                let collidable = ent.collidable();
                if let Some(col) = collidable.get() {
                    let origin = ent.abs_origin();
                    let mins = col.min().clone();
                    let maxs = col.max().clone();
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

                    let projected_points: Vec<Vec3> = points
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

                    let mut left = projected_points[0].x;
                    let mut bottom = projected_points[0].y;
                    let mut right = projected_points[0].x;
                    let mut top = projected_points[0].y;

                    for point in projected_points {
                        left = left.min(point.x);
                        bottom = bottom.max(point.y);
                        right = right.max(point.x);
                        top = top.min(point.y);
                    }

                    let x = left as i32;
                    let y = top as i32;
                    let x1 = right as i32;
                    let y1 = bottom as i32;

                    interfaces
                        .vgui_surface
                        .set_draw_color(Color::new_rgba(255, 255, 255, 255));
                    interfaces.vgui_surface.draw_outlined_rect(x, y, x1, y1);
                }
            }
        }
    }
}
