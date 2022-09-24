use crate::sdk::structs::entities::CEntity;
use crate::Vec3;
use std::ffi::c_char;
use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

#[has_vtable]
#[derive(VTable, Debug)]
struct PlayerMoveHelper {}

impl PlayerMoveHelper {
    #[virtual_index(0)]
    fn name(&self) -> *const c_char {}

    #[virtual_index(1)]
    fn set_host(&self, host: *const CEntity) {}

    #[virtual_index(2)]
    fn reset_touch_list(&self) {}

    #[virtual_index(4)]
    fn process_impacts(&self) {}

    #[virtual_index(6)]
    fn start_sound(
        &self,
        origin: &Vec3,
        channel: i32,
        sample: *const c_char,
        volume: f32,
        sound_level: i32,
        f_flags: i32,
        pitch: i32,
    ) {
    }

    #[virtual_index(7)]
    fn start_sound_simple(&self, origin: &Vec3, sound_name: *const c_char) {}

    #[virtual_index(8)]
    fn playback_event_full(
        &self,
        flags: i32,
        client_index: i32,
        event_index: i16,
        delay: f32,
        origin: &Vec3,
        angles: &Vec3,
        f_param1: f32,
        f_param2: f32,
        i_param1: i32,
        i_param2: i32,
        b_param1: i32,
        b_param2: i32,
    ) {
    }

    #[virtual_index(9)]
    fn player_falling_damage(&self) -> bool {}
}
