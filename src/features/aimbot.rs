use crate::{feature, EventCreateMove};
feature!(Aimbot => Aimbot::create_move);

impl Aimbot {
    pub fn create_move(_: &mut EventCreateMove) {}
}
