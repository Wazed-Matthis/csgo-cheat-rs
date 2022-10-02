use crate::{feature, EventFrameStageNotify, EventOverrideView, Vec3, INTERFACES};
use std::mem;

feature!(ThirdPerson => ThirdPerson::override_view);

impl ThirdPerson {
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

        if let Some(local) = local_player.get() {
            input.m_camera_in_third_person = false;
            view.fov = 120.0;
            view.fov_viewmodel = 120.0;
            let mut angels = unsafe { mem::zeroed() };
            interfaces.engine.get_view_angles(&mut angels);
            angels.z = 100.0;
            view.aspect_ratio = 0.5;
            input.m_camera_offset = angels;
        }
    }
}
