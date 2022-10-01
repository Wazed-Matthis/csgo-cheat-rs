use crate::font::FontType::{Items, Outline, OutlineBold, Shadow, ShadowBold, Small};
use crate::sdk::structs::entities::CEntity;
use crate::{feature, font, sdk, Color, EventPaintTraverse, Vec3, INTERFACES, WEAPON_MAP};
use color_space::{Hsv, Rgb};
use std::ffi::{CStr, OsStr};
use std::mem;
use widestring::WideCString;

feature!(ESP => ESP::paint_traverse);

impl ESP {
    pub fn paint_traverse(_: &mut EventPaintTraverse) {
        let interfaces = INTERFACES.get().unwrap();

        for i in 0..interfaces.global_vars.max_clients {
            let entity = interfaces.entity_list.entity(i);
            if let Some(ent) = entity.get() {
                if ent.health() > 0 && interfaces.engine.local_player() != i {
                    Self::bone_esp(&ent);
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
                            let bounds = font::text_bounds(name, Shadow);

                            font::text_center(
                                name,
                                x + ((x1 - x) / 2.0),
                                y - bounds.1 as f32,
                                Shadow,
                                Color::new_rgba(255, 255, 255, 255),
                            );
                        }

                        // Health bar
                        interfaces
                            .vgui_surface
                            .set_draw_color(Color::new_hex(0xff000000));
                        interfaces.vgui_surface.draw_outlined_rect(
                            (x - 6f32) as i32,
                            (y - 1f32) as i32,
                            (x - 2f32) as i32,
                            (y1 + 1f32) as i32,
                        );
                        interfaces
                            .vgui_surface
                            .set_draw_color(Color::new_hex(0x90000000));
                        interfaces.vgui_surface.draw_filled_rect(
                            (x - 5f32) as i32,
                            (y) as i32,
                            (x - 2f32) as i32,
                            (y1) as i32,
                        );
                        let health = ent.health();
                        let modifier = health as f32 / 100f32;
                        let bar_height = (y1 - y) * modifier;
                        let hsv = Hsv::new((modifier as f64) / 3f64 * 360f64, 1f64, 1f64);
                        let rgb = Rgb::from(hsv);
                        let bar_y = y + ((y1 - y) - bar_height);
                        interfaces.vgui_surface.set_draw_color(Color::new_rgba(
                            rgb.r as i32,
                            rgb.g as i32,
                            rgb.b as i32,
                            255,
                        ));
                        interfaces.vgui_surface.draw_filled_rect(
                            (x - 5f32) as i32,
                            bar_y as i32,
                            (x - 3f32) as i32,
                            (bar_y + bar_height) as i32,
                        );
                        // Draw health text
                        if health < 100 {
                            let bounds = font::text_bounds(health.to_string().as_str(), ShadowBold);
                            font::text_center(
                                health.to_string().as_str(),
                                x - 4f32,
                                bar_y - bounds.1 as f32 / 2f32,
                                OutlineBold,
                                Color::new_rgba(255, 255, 255, 255),
                            );
                        }
                        if let Some(weapon) = ent.weapon() {
                            // Weapon text
                            let display_name_find =
                                interfaces.localize.find(weapon.get_weapon_data().name);
                            let c_str = unsafe { WideCString::from_ptr_str(display_name_find) };
                            let weapon_name = c_str.to_string_lossy();
                            let text_bounds = font::text_bounds(&weapon_name.to_uppercase(), Small);
                            font::text_center(
                                &weapon_name.to_uppercase(),
                                x + ((x1 - x) / 2.0),
                                y1 + 1f32,
                                Small,
                                Color::new_hex(0xffffffff),
                            );
                            // Weapon icon
                            let weapon_map = WEAPON_MAP
                                .get()
                                .expect("Could not get weapon map for Item Rendering");
                            if let Some(weapon_id) = weapon_map.get(&(weapon.get_id() as i32)) {
                                font::text_center(
                                    weapon_id,
                                    x + ((x1 - x) / 2.0),
                                    y1 + text_bounds.1 as f32 - 4f32,
                                    Items,
                                    Color::new_hex(0xffffffff),
                                )
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn bone_esp(entity: &CEntity) {
        let interfaces = INTERFACES.get().unwrap();
        let mut first_bones = unsafe { mem::zeroed() };
        unsafe {
            entity.setup(&mut first_bones, 128, 0x0007FF00, 0.0);
        }
        let model_ptr = entity.model();
        if !model_ptr.is_null() {
            let studio_model_ptr = interfaces.model_info.studio_model(model_ptr);
            if !studio_model_ptr.is_null() {
                unsafe {
                    let studio_model = &*studio_model_ptr;
                    for i in 0..studio_model.bone_count {
                        if let Some(bone) = studio_model.bone(i) {
                            let parent = bone.parent;
                            if parent == -1 || bone.flags & 0x00000100 == 0 {
                                continue;
                            }
                            let mut bone_pos = first_bones[i as usize].origin();
                            let mut parent_bone_pos = first_bones[parent as usize].origin();
                            let mut bone_screen = bone_pos.clone();
                            let mut parent_bone_screen = parent_bone_pos.clone();
                            interfaces
                                .debug_overlay
                                .world_to_screen(&mut bone_pos, &mut bone_screen);
                            interfaces
                                .debug_overlay
                                .world_to_screen(&mut parent_bone_pos, &mut parent_bone_screen);
                            interfaces
                                .vgui_surface
                                .set_draw_color(Color::new_hex(0xffffffff));
                            interfaces.vgui_surface.draw_line(
                                bone_screen.x as i32,
                                bone_screen.y as i32,
                                parent_bone_screen.x as i32,
                                parent_bone_screen.y as i32,
                            );
                        }
                    }
                }
            }
        }
    }
}
