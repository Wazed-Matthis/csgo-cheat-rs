use rand::Rng;

use crate::sdk::classes::EButtons;
use crate::{feature, EventCreateMove, OsRng, CONFIG};

feature!(AntiAim => AntiAim::on_create_move);

impl AntiAim {
    fn on_create_move(event: &mut EventCreateMove) {
        unsafe {
            let a = &mut *event.user_cmd;
            let mut rng = OsRng::default();
            let new_yaw = rng.gen::<f32>() * 360.0 - 180.0;

            // Check if the in_attack button is currently being pressed, if not, set the antiAim yaw
            if !a.buttons.contains(EButtons::ATTACK) {
                let guard = CONFIG.get().unwrap();
                a.view_angles.x = guard.features.AntiAim.pitch;
                a.view_angles.y = new_yaw;
            }
        }
    }
}
