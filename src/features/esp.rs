use crate::font::FontType::{Items, OutlineBold, Shadow, ShadowBold, Small};
use crate::sdk::classes::Vec2;
use crate::sdk::structs::entities::CEntity;
use crate::sdk::surface::Vertex;
use crate::util::math;
use crate::{feature, font, Color, EventPaintTraverse, Vec3, WeaponType, INTERFACES, WEAPON_MAP};
use color_space::{Hsv, Rgb};
use std::f32::consts::PI;
use std::ffi::CStr;
use std::mem;
use std::mem::zeroed;
use widestring::WideCString;

feature!(ESP => ESP::paint_traverse);

impl ESP {
    pub fn paint_traverse(_: &mut EventPaintTraverse) {
        let interfaces = INTERFACES.get().unwrap();
        let option = interfaces
            .entity_list
            .entity(interfaces.engine.local_player())
            .get();

        let local_player = if let Some(ent) = option {
            ent
        } else {
            return;
        };

        for i in 0..interfaces.global_vars.max_clients {
            let entity = interfaces.entity_list.entity(i);
            if let Some(ent) = entity.get() {
                if ent.is_player()
                    && ent.is_alive()
                    && i != interfaces.engine.local_player()
                    && ent.team() != local_player.team()
                {
                    Self::off_screen_esp(&ent);
                    Self::bone_esp(&ent);
                    let collidable = ent.collidable();
                    if let Some(col) = collidable.get() {
                        let origin = ent.abs_origin();
                        let mins = *col.min();
                        let maxs = *col.max();
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
                                let mut contextualized = *point + *origin;
                                let mut projected = contextualized;
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
                            if let Ok(Some(&weapon_str)) = weapon
                                .get_id()
                                .try_into()
                                .map(|ty: WeaponType| weapon_map.get(&ty))
                            {
                                font::text_center(
                                    weapon_str,
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

    pub fn off_screen_esp(entity: &CEntity) {
        let interfaces = INTERFACES.get().unwrap();
        let local_player = interfaces
            .entity_list
            .entity(interfaces.engine.local_player())
            .get()
            .unwrap();
        let mut target_pos = entity.origin();
        let mut screen_pos = target_pos;
        interfaces
            .debug_overlay
            .world_to_screen(&mut target_pos, &mut screen_pos);
        let mut screen_w = 0;
        let mut screen_h = 0;
        interfaces.engine.screen_size(&mut screen_w, &mut screen_h);
        let tolerance_x = screen_w as f32 / 18f32;
        let tolerance_y = screen_h as f32 / 18f32;

        if screen_pos.x < -tolerance_x
            || screen_pos.x > (screen_w as f32 + tolerance_x)
            || screen_pos.y < -tolerance_y
            || screen_pos.y > (screen_h as f32 + tolerance_y)
        {
            let pos_data =
                Self::get_offscreen_pos((target_pos - local_player.origin()).normalized());

            let position = pos_data.0;
            let rotation = (-pos_data.1.to_radians()).to_degrees();
            let base = math::rotate_vertex(
                position,
                Vertex::pos(Vec2 {
                    x: position.x,
                    y: position.y,
                }),
                rotation,
            );
            let left = math::rotate_vertex(
                position,
                Vertex::pos(Vec2 {
                    x: position.x - 12.0,
                    y: position.y + 24.0,
                }),
                rotation,
            );
            let right = math::rotate_vertex(
                position,
                Vertex::pos(Vec2 {
                    x: position.x + 12.0,
                    y: position.y + 24.0,
                }),
                rotation,
            );
            let mut vec = [base, left, right];

            unsafe {
                let vertex_ptr = vec.as_mut_ptr();
                interfaces
                    .vgui_surface
                    .set_draw_color(Color::new_hex(0xffffffff));
                interfaces.vgui_surface.draw_polygon(3, vertex_ptr, true);
            }
        }
    }

    pub fn get_offscreen_pos(delta: Vec3) -> (Vec2, f32) {
        let interfaces = INTERFACES.get().unwrap();
        let mut screen_w = 0;
        let mut screen_h = 0;
        interfaces.engine.screen_size(&mut screen_w, &mut screen_h);
        let radius: f32 = 150.0 * (screen_h as f32 / 480.0);
        let mut view_angles = unsafe { zeroed::<Vec3>() };
        interfaces.engine.get_view_angles(&mut view_angles);
        let up = Vec3 {
            x: 0f32,
            y: 0f32,
            z: 1f32,
        };
        let mut fwd = math::heading(view_angles);
        fwd.z = 0f32;
        fwd = fwd.normalized();
        let right = up.crossed(fwd);
        let front = delta.dot(fwd);
        let side = delta.dot(right);
        let mut output = Vec2 {
            x: radius * -side,
            y: radius * -front,
        };
        // Calculate the offsets used for offsetting from screen center
        let out_rotation = (output.x.atan2(output.y) + PI).to_degrees();
        let yaw_rad = -out_rotation.to_radians();
        let sa = yaw_rad.sin();
        let ca = yaw_rad.cos();
        // Offset from the screen center
        output.x = (screen_w as f32 / 2f32) + (radius * sa);
        output.y = (screen_h as f32 / 2f32) - (radius * ca);
        // Return the results
        (output, out_rotation)
    }

    pub fn bone_esp(entity: &CEntity) {
        let interfaces = INTERFACES.get().unwrap();
        let mut first_bones = unsafe { mem::zeroed() };
        unsafe {
            entity.setup_bones(&mut first_bones, 128, 0x0007FF00, 0.0);
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
                            let mut bone_screen = bone_pos;
                            let mut parent_bone_screen = parent_bone_pos;
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
