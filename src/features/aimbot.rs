use std::cmp::Ordering;
use std::ffi::CStr;
use std::mem;
use std::mem::zeroed;
use std::process::exit;
use std::ptr::null_mut;

use log::debug;
use vtables::VTable;
use winapi::um::winbase::BuildCommDCBAndTimeoutsA;
use winapi::um::winnt::INT;

use crate::sdk::classes::{EButtons, Matrix3x4, Matrix4x3};
use crate::sdk::structs::entities::CEntity;
use crate::sdk::structs::weapon::{Weapon, WeaponInfo};
use crate::sdk::surface::Surface;
use crate::sdk::surface_props::SurfaceData;
use crate::sdk::trace::hit_group::HitGroup::Invalid;
use crate::sdk::trace::hit_group::{get_damage_multiplier, HitGroup};
use crate::sdk::trace::{
    CSurface, Ray, Trace, TraceFilterGeneric, TraceFilterTrait, CHAR_TEX_CARDBOARD, CHAR_TEX_GLASS,
    CHAR_TEX_GRATE, CHAR_TEX_PLASTIC, CHAR_TEX_WOOD, CONTENTS_GRATE, CONTENTS_HITBOX, MASK_SHOT,
    MASK_SHOT_HULL, SURF_HITBOX, SURF_NODRAW,
};
use crate::{feature, EventCreateMove, FeatureSettings, Vec3, CONFIG, INTERFACES};

feature!(Aimbot => Aimbot::create_move);

impl Aimbot {
    pub fn create_move(event: &mut EventCreateMove) {
        let interfaces = INTERFACES.get().unwrap();
        let cmd = unsafe { &mut *event.user_cmd };

        let local_player = interfaces
            .entity_list
            .entity(interfaces.engine.local_player())
            .get()
            .unwrap();
        let mut local_eye_pos = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        local_player.eye_pos(&mut local_eye_pos);

        let mut possible_targets = (0..interfaces.global_vars.max_clients)
            .flat_map(|i| {
                let entity = interfaces.entity_list.entity(i);
                (entity.get())
            })
            .filter(|(entity)| {
                entity.is_player()
                    && entity.is_alive()
                    && entity.as_ptr() != local_player.as_ptr()
                    && entity.team() != local_player.team()
            })
            .collect::<Vec<CEntity>>();

        let mut view_angles = unsafe { zeroed::<Vec3>() };

        interfaces.engine.get_view_angles(&mut view_angles);

        possible_targets.sort_by(|entity, entity1| {
            let mut first_bones = unsafe { mem::zeroed() };
            unsafe {
                entity.setup(&mut first_bones, 255, 255, 0.0);
            }
            let first_head_bone = first_bones[8].origin();

            let mut second_bones = unsafe { mem::zeroed() };
            unsafe {
                entity1.setup(&mut second_bones, 255, 255, 0.0);
            }
            let second_head_bone = second_bones[8].origin();

            let (first_yaw, first_pitch) =
                Aimbot::calculate_angle_to_entity(first_head_bone, local_eye_pos);
            let (second_yaw, second_pitch) =
                Aimbot::calculate_angle_to_entity(second_head_bone, local_eye_pos);

            let current_yaw = view_angles.y;
            let current_pitch = view_angles.x;
            let (first_diff_yaw, first_diff_pitch) = (
                (first_yaw - current_yaw).abs(),
                (first_pitch - current_pitch).abs(),
            );

            let (second_diff_yaw, second_diff_pitch) = (
                (second_yaw - current_yaw).abs(),
                (second_pitch - current_pitch).abs(),
            );

            let first_diff = first_diff_yaw.hypot(first_diff_pitch);
            let second_diff = second_diff_yaw.hypot(second_diff_pitch);

            if first_diff < second_diff {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let closest_target = possible_targets.first();

        let weapon_data = match local_player.weapon() {
            None => return,
            Some(weapon) => weapon.get_weapon_data(),
        };

        if let Some(closest) = closest_target {
            let mut mat = unsafe { mem::zeroed() };
            unsafe {
                let a = closest.setup(&mut mat, 255, 255, 0.0);
            }
            let closest_eye_pos = mat[8].origin();

            let (damage, hitgroup, lethal) =
                Self::handle_wall(&local_player, closest_eye_pos, weapon_data, closest, 0);

            let guard = CONFIG.get().unwrap();
            let aimbot_settings = guard.features.Aimbot.clone();

            if damage > aimbot_settings.min_damage
                || lethal
                    && local_player.weapon().unwrap().next_attack() + 0.2
                        < interfaces.global_vars.cur_time
            {
                let (yaw, pitch) =
                    Aimbot::calculate_angle_to_entity(closest_eye_pos, local_eye_pos);
                cmd.view_angles.x = pitch;
                cmd.view_angles.y = yaw;
                debug!(
                    "Damage: {}, hitgroup: {:?}, is lethal: {}",
                    damage, hitgroup, lethal
                );

                let mut velocity = local_player.velocity() * -1.0;

                velocity.z = 0.0;

                let speed = velocity.mag_sqrt();
                let yaw =
                    (cmd.view_angles.y - velocity.y.atan2(velocity.x).to_degrees()).to_radians();

                cmd.forward_move = yaw.cos() * speed - yaw.sin() * speed;
                cmd.side_move = yaw.sin() * speed + yaw.cos() * speed;

                if speed < weapon_data.max_speed / 5.0 {
                    cmd.buttons |= EButtons::ATTACK;
                }
            }
        }
    }

    fn calculate_angle_to_entity(entity: Vec3, local_origin: Vec3) -> (f32, f32) {
        let delta = entity - local_origin;
        (
            delta.y.atan2(delta.x).to_degrees(),
            (-delta.z).atan2(delta.x.hypot(delta.y)).to_degrees(),
        )
    }
    fn scale_damage(target: &CEntity, hit_group: i32, hs_multiplier: f32, armor_ratio: f32) -> f32 {
        let group = HitGroup::try_from(hit_group).unwrap_or(Invalid);
        let mut damage = match group {
            HitGroup::Head => hs_multiplier,
            HitGroup::Stomach => 1.25,
            HitGroup::LeftLeg | HitGroup::RightLeg => 0.75,
            _ => 1.0,
        };

        let armor = target.armor();
        if armor > 0 || (group == HitGroup::Head && target.has_helmet()) {
            let heavy_ratio = 1f32;
            let bonus_ratio = 0.5f32;
            let ratio = armor_ratio * 0.05;

            damage -= if (armor as f32) < damage * armor_ratio {
                armor as f32 * 4.0
            } else {
                damage * (1.0 - armor_ratio)
            };
        }

        damage
    }

    fn handle_wall(
        local_player: &CEntity,
        target_eye_pos: Vec3,
        weapon_data: &WeaponInfo,
        target: &CEntity,
        target_index: i32,
    ) -> (f32, HitGroup, bool) {
        let interfaces = INTERFACES.get().unwrap();
        let mut start = unsafe { zeroed::<Vec3>() };
        local_player.eye_pos(&mut start);

        let mut direction = (target_eye_pos - start).normalized();
        // Get weapon data
        let mut damage = weapon_data.damage as f32;
        let mut penetration = weapon_data.penetration;

        let mut distance = 0.0;
        let mut pen = 4;

        while damage >= 0.0 && pen > 0 {
            // Calculate remaining length
            let remaining = weapon_data.range - distance;

            // Set trace end
            let end = start + (direction * remaining);

            // Setup the ray and trace
            let ray = Ray::new(start, end);
            let mut trace = unsafe { zeroed::<Trace>() };
            let mut filter = TraceFilterGeneric::new(local_player.as_ptr());
            interfaces.trace.trace_ray_virtual(
                &ray,
                0x4600400B,
                &mut filter as *mut TraceFilterGeneric as *mut usize,
                &mut trace,
            );

            // If not hit
            if trace.fraction == 1.0 {
                break;
            }

            // Update damage based on the distance traveled
            distance += trace.fraction * remaining;
            let scale = Self::scale_damage(
                target,
                trace.hit_group,
                weapon_data.headshot_mult,
                weapon_data.armor_ratio / 2.0,
            );
            damage *= scale * weapon_data.range_modifier.powf(distance / 500f32);

            let group = HitGroup::try_from(trace.hit_group).unwrap_or(HitGroup::Invalid);

            if trace.ptr_entity == target.as_ptr() as _ {
                return (damage, group, target.health() <= damage as _);
            }

            let surface_data = interfaces
                .surface_props
                .surface_data(trace.surface.surface_props as i32);

            if surface_data.penetration_modifier < 0.1 {
                break;
            }

            if !Self::handle_bullet_penetration(
                *surface_data,
                &mut trace,
                &mut direction,
                &mut start,
                weapon_data.penetration,
                &mut damage,
            ) {
                break;
            }

            pen -= 1;
        }

        (0f32, HitGroup::Invalid, false)
    }

    fn trace_to_exit(
        end: &mut Vec3,
        enter_trace: &mut Trace,
        start: &mut Vec3,
        dir: &mut Vec3,
        exit_trace: &mut Trace,
    ) -> bool {
        let interfaces = INTERFACES.get().unwrap();
        for distance in (4..90).step_by(4) {
            *end = *start + *dir * distance as f32;
            let point_contents = interfaces
                .trace
                .get_point_contents(end, MASK_SHOT, null_mut());

            if point_contents & MASK_SHOT_HULL != 0 && point_contents & CONTENTS_HITBOX == 0 {
                continue;
            }

            let new_end = *end - (*dir * 4.0);
            let ray = Ray::new(*end, new_end);
            interfaces
                .trace
                .trace_ray_virtual(&ray, MASK_SHOT as u32, 0 as _, exit_trace);

            if exit_trace.start_solid && exit_trace.surface.flags & SURF_HITBOX != 0 {
                let ray = Ray::new(*end, *start);
                let mut filter = TraceFilterGeneric::new(exit_trace.ptr_entity as _);
                interfaces.trace.trace_ray_virtual(
                    &ray,
                    MASK_SHOT_HULL as u32,
                    &mut filter as *const TraceFilterGeneric as _,
                    exit_trace,
                );
                if (exit_trace.fraction < 1.0 || exit_trace.all_solid) && !exit_trace.start_solid {
                    *end = exit_trace.end;
                    return true;
                }
                continue;
            }

            if exit_trace.start_solid || exit_trace.fraction > 1.0 && !exit_trace.all_solid {
                if !exit_trace.ptr_entity.is_null()
                    && !enter_trace.ptr_entity.is_null()
                    && enter_trace.ptr_entity
                        == interfaces.entity_list.entity(0).get().unwrap().as_ptr() as _
                {
                    *exit_trace = *enter_trace;
                    exit_trace.end = *start + *dir;
                    return true;
                }

                continue;
            }

            if exit_trace.surface.flags & SURF_NODRAW != 0 {
                return true;
            }
            if exit_trace.plane.normal.dot(*dir) <= 1.0 {
                let fraction = exit_trace.fraction * 4.0;
                *end = *end - (*dir * fraction);
                return true;
            }
        }

        false
    }

    pub fn handle_bullet_penetration(
        enter_surface_data: SurfaceData,
        enter_trace: &mut Trace,
        direction: &mut Vec3,
        start: &mut Vec3,
        penetration: f32,
        damage: &mut f32,
    ) -> bool {
        let interfaces = INTERFACES.get().unwrap();
        let mut exit_trace = unsafe { zeroed() };
        let mut dummy = unsafe { zeroed() };

        if !Self::trace_to_exit(
            &mut dummy,
            enter_trace,
            &mut enter_trace.end.clone(),
            direction,
            &mut exit_trace,
        ) {
            return false;
        }
        let exit_surface_data = interfaces
            .surface_props
            .surface_data(exit_trace.surface.surface_props as i32);

        let exit_surface_data = unsafe { *exit_surface_data };
        let mut damage_modifier = 0.16f32;
        let mut penetration_modifier = 0.0;

        // used later in calculations.
        let penetration_mod = 0.0f32.max((3.0 / penetration) * 1.25);
        let nodraw = enter_trace.surface.flags & SURF_NODRAW;
        let grate = enter_trace.contents & CONTENTS_GRATE;

        if enter_surface_data.material as u8 as char == CHAR_TEX_GRATE
            || enter_surface_data.material as u8 as char == CHAR_TEX_GLASS
        {
            penetration_modifier = 3.0;
            damage_modifier = 0.05;
        } else if nodraw != 0 || grate != 0 {
            penetration_modifier = 1.0;
        } else {
            penetration_modifier = (enter_surface_data.penetration_modifier
                + exit_surface_data.penetration_modifier)
                / 2.0;
            damage_modifier = 0.16;
        }

        if enter_surface_data.material == exit_surface_data.material {
            if exit_surface_data.material as u8 as char == CHAR_TEX_CARDBOARD
                || exit_surface_data.material as u8 as char == CHAR_TEX_WOOD
            {
                penetration_modifier = 3.0;
            } else if exit_surface_data.material as u8 as char == CHAR_TEX_PLASTIC {
                penetration_modifier = 2.0;
            }
        }

        let trace_len = (exit_trace.end - enter_trace.end).mag();
        let modifier = 0.0f32.max(1.0 / penetration_modifier);
        let damage_lost = ((modifier * 3.0) * penetration_mod + (*damage * damage_modifier))
            + (((trace_len * trace_len) * modifier) / 24.0);
        *damage -= damage_lost;
        *start = exit_trace.end;

        if *damage < 1.0 {
            return false;
        }

        true
    }
}
