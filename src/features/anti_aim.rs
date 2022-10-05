use rand::Rng;
use std::mem::zeroed;

use crate::sdk::classes::EButtons;
use crate::{feature, util, EventCreateMove, OsRng, Vec3, CONFIG, INTERFACES, TARGET};

feature!(AntiAim => AntiAim::on_create_move);

impl AntiAim {
    fn on_create_move(event: &mut EventCreateMove) {
        unsafe {
            let interfaces = INTERFACES.get().unwrap();
            let a = &mut *event.user_cmd;
            let mut rng = OsRng::default();
            let new_yaw = rng.gen::<f32>() * 360.0 - 180.0;

            let option_target = TARGET.get().unwrap().read().unwrap();
            let local_player = interfaces
                .entity_list
                .entity(interfaces.engine.local_player())
                .get()
                .unwrap();

            let mut view_angles = unsafe { zeroed::<Vec3>() };
            interfaces.engine.get_view_angles(&mut view_angles);

            let mut base_rotation = view_angles.y - 180.0;
            if let Some(target) = util::entity_util::closest_target(|_| true) {
                let pos_diff = local_player.origin() - target.origin();
                let rotation_vec = pos_diff.normalized();
                let rotation = (
                    rotation_vec.y.atan2(rotation_vec.x).to_degrees(),
                    (-rotation_vec.z)
                        .atan2(rotation_vec.x.hypot(rotation_vec.y))
                        .to_degrees(),
                );
                base_rotation = rotation.0;
            }

            let random_jitter = rng.gen::<f32>() * 25.0;

            let finalized_rot = base_rotation + random_jitter;

            // Check if the in_attack button is currently being pressed, if not, set the antiAim yaw
            if !a.buttons.contains(EButtons::ATTACK) {
                let guard = CONFIG.get().unwrap();
                a.view_angles.x = guard.features.AntiAim.pitch;
                a.view_angles.y = finalized_rot;
            }
        }
    }
}
