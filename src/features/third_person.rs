use std::mem;
use std::sync::RwLock;

use crate::sdk::classes::Stage;
use crate::sdk::trace::{Ray, TraceFilterGeneric, TraceFilterTrait, CONTENTS_GRATE, MASK_SHOT};
use crate::util::math;
use crate::util::math::heading;
use crate::{feature, EventFrameStageNotify, EventOverrideView, Vec3, INTERFACES};
use vtables::VTable;

pub static ANGLES: RwLock<Vec3> = RwLock::new(Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
});

feature!(ThirdPerson => ThirdPerson::override_view, ThirdPerson::fns);

impl ThirdPerson {
    pub fn fns(event: &mut EventFrameStageNotify) {
        let interfaces = INTERFACES.get().unwrap();
        if event.stage == Stage::FrameRenderStart {
            let local_player = interfaces
                .entity_list
                .entity(interfaces.engine.local_player());
            if !interfaces.engine.is_in_game() {
                return;
            }

            unsafe {
                if let Some(local) = local_player.get() {
                    if !local.is_alive() {
                        return;
                    }

                    let angles = ANGLES.read().unwrap().clone();
                    let dead_flag = crate::netvar::offset("DT_BasePlayer", "deadflag");

                    let angle_ptr = (local.vtable as usize + dead_flag + 0x4) as *mut Vec3;
                    if angle_ptr.is_null() {
                        return;
                    }
                    let v = &mut *angle_ptr;
                    v.x = angles.x;
                    v.y = angles.y;
                    v.z = angles.z;
                }
            }
        }
    }

    pub fn override_view(event: &mut EventOverrideView) {
        let interfaces = INTERFACES.get().unwrap();
        if event.setup.is_null() {
            return;
        }
        let view = unsafe { &mut *event.setup };
        let mut input = unsafe { &mut *interfaces.input };
        let local_player = interfaces
            .entity_list
            .entity(interfaces.engine.local_player());
        if !interfaces.engine.is_in_game() {
            return;
        }

        unsafe {
            if let Some(local) = local_player.get() {
                if !local.is_alive() {
                    return;
                }
                let mut view_angles = unsafe { mem::zeroed() };
                interfaces.engine.get_view_angles(&mut view_angles);

                let mut start = unsafe { mem::zeroed() };
                local.eye_pos(&mut start);

                let yaw = view_angles.y.to_radians();
                let pitch = view_angles.x.to_radians();

                let direction = Vec3 {
                    x: yaw.cos() * pitch.cos(),
                    y: yaw.sin() * pitch.cos(),
                    z: -pitch.sin(),
                };

                let ray = Ray::new(start, start - direction * 150.0);
                let mut trace = unsafe { mem::zeroed() };
                let mut filter = TraceFilterGeneric::new(local.as_ptr());
                interfaces.trace.trace_ray_virtual(
                    &ray,
                    (MASK_SHOT | CONTENTS_GRATE) as u32,
                    &mut filter as *mut TraceFilterGeneric as _,
                    &mut trace,
                );

                let dist = 100.0 * trace.fraction.min(1.0) * 0.8;

                view_angles.z = dist;
                input.m_camera_in_third_person = true;
                input.m_camera_offset = view_angles;

                // view.fov = 120.0;
                // view.fov_viewmodel = 120.0;
                view.aspect_ratio = 0.25;
            }
        }
    }
}
