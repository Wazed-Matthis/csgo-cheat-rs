use crate::{feature, EButtons, EventCreateMove, OsRng};
use rand::Rng;

feature!(AntiAim => AntiAim::on_create_move);

impl AntiAim {
    fn on_create_move(event: &mut EventCreateMove) {
        unsafe {
            let a = &mut *event.user_cmd;
            let mut rng = OsRng::default();
            let old_yaw = a.view_angles.y;
            let new_yaw = rng.gen::<f32>() * 360.0 - 180.0;
            let delta_yaw = (new_yaw - old_yaw).to_radians();

            match a.buttons {
                EButtons::InAttack => {}
                _ => {
                    a.view_angles.y = new_yaw;
                    a.view_angles.x = 89.0;
                }
            }

            let forward = a.forward_move;
            let strafe = a.side_move;
            a.forward_move = delta_yaw.cos() * forward - delta_yaw.sin() * strafe;
            a.side_move = delta_yaw.sin() * forward + delta_yaw.cos() * strafe;
        }
    }
}