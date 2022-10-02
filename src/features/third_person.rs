use std::mem;
use std::sync::RwLock;

use crate::sdk::classes::Stage;
use crate::{feature, EventFrameStageNotify, EventOverrideView, Vec3, INTERFACES};

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
                    let mut angles = ANGLES.read().unwrap().clone();
                    let dead_flag = crate::netvar::offset("DT_BasePlayer", "deadflag");

                    let angle_ptr = ((local.vtable as usize + dead_flag + 0x4) as *mut Vec3);
                    dbg!(angle_ptr);
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
        dbg!(interfaces.input);
        let mut input = unsafe { &mut *interfaces.input };
        let local_player = interfaces
            .entity_list
            .entity(interfaces.engine.local_player());
        if !interfaces.engine.is_in_game() {
            return;
        }

        unsafe {
            if let Some(local) = local_player.get() {
                input.m_camera_in_third_person = true;
                view.fov = 120.0;
                view.fov_viewmodel = 120.0;
                let mut view_angles = unsafe { mem::zeroed() };
                interfaces.engine.get_view_angles(&mut view_angles);
                view_angles.z = 100.0;

                view.aspect_ratio = 0.5;
                input.m_camera_moving_with_mouse = false;
                input.m_camera_offset = view_angles;
            }
        }
    }
}
