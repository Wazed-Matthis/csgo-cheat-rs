use crate::{feature, EventCreateMove, EventFrameStageNotify, Vec3, INTERFACES};

feature!(ThirdPerson => ThirdPerson::create_move, ThirdPerson::fns);

impl ThirdPerson {
    pub fn create_move(event: &mut EventCreateMove) {}

    pub fn fns(event: &mut EventFrameStageNotify) {
        let interfaces = INTERFACES.get().unwrap();
        let mut input = unsafe { &mut *interfaces.input };
        let local_player = interfaces
            .entity_list
            .entity(interfaces.engine.local_player());
        if let Some(local) = local_player.get() {
            input.m_camera_in_third_person = true;

            input.m_camera_offset = Vec3 {
                x: -89.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }
}
