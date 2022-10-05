use crate::sdk::structs::entities::CEntity;
use crate::util::math;
use crate::{Aimbot, Vec3, INTERFACES};
use std::cmp::Ordering;
use std::mem::zeroed;
use vtables::VTable;

pub fn closest_target<F: FnOnce(&CEntity) -> bool + Copy>(filter: F) -> Option<CEntity> {
    let interfaces = INTERFACES.get().unwrap();
    let local_player = interfaces
        .entity_list
        .entity(interfaces.engine.local_player())
        .get()
        .unwrap();
    let mut player_eye_pos = unsafe { zeroed() };
    local_player.eye_pos(&mut player_eye_pos);

    let mut possible_targets = (0..interfaces.global_vars.max_clients)
        .filter_map(|i| interfaces.entity_list.entity(i).get())
        .filter(|entity| {
            entity.is_player()
                && entity.is_alive()
                && entity.as_ptr() != local_player.as_ptr()
                && entity.team() != local_player.team()
                && filter(entity)
        })
        .collect::<Vec<CEntity>>();

    let mut view_angles = unsafe { zeroed::<Vec3>() };

    interfaces.engine.get_view_angles(&mut view_angles);

    possible_targets.sort_by(|entity, entity1| {
        let mut first_bones = unsafe { zeroed() };
        unsafe {
            entity.setup_bones(&mut first_bones, 255, 255, 0.0);
        }
        let first_head_bone = first_bones[8].origin();

        let mut second_bones = unsafe { zeroed() };
        unsafe {
            entity1.setup_bones(&mut second_bones, 255, 255, 0.0);
        }
        let second_head_bone = second_bones[8].origin();

        let (first_yaw, first_pitch) =
            math::calculate_angle_to_entity(first_head_bone, player_eye_pos);
        let (second_yaw, second_pitch) =
            math::calculate_angle_to_entity(second_head_bone, player_eye_pos);

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

    possible_targets.first().cloned()
}
