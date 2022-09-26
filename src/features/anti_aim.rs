use crate::sdk::classes::EButtons;
use crate::{feature, EventCreateMove, OsRng};
use rand::Rng;

feature!(AntiAim => AntiAim::on_create_move);

impl AntiAim {
    fn on_create_move(event: &mut EventCreateMove) {
        unsafe {
            let a = &mut *event.user_cmd;
            let mut rng = OsRng::default();
            let new_yaw = rng.gen::<f32>() * 360.0 - 180.0;

            // Check if the in_attack button is currently being pressed, if not, set the antiAim yaw
            if a.buttons.contains(EButtons::ATTACK) {
                a.view_angles.x = 89f32;
                a.view_angles.y = new_yaw;
            }
        }
    }
}
