use crate::sdk::structs::entities::CEntity;
use crate::Vec3;
use std::ffi::c_char;
use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

#[derive(Debug)]
#[repr(C)]
struct PlayerMoveData {
    first_run_of_instructions: bool,
    game_code_moved_player: bool,
    player_handle: i32,
    impulse_command: i32,
    view_angles: Vec3,
    abs_view_angles: Vec3,
    buttons: i32,
    old_buttons: i32,
    fw_move: f32,
    sd_move: f32,
    up_move: f32,
    max_speed: f32,
    client_max_speed: f32,
    velocity: Vec3,
    angles: Vec3,
    old_angles: Vec3,
    step_height: f32,
    wish_velocity: Vec3,
    jump_velocity: Vec3,
    constraint_center: Vec3,
    constraint_radius: f32,
    constraint_width: f32,
    constraint_speed_factor: f32,
    u0: [f32; 5],
    abs_origin: Vec3,
}

#[has_vtable]
#[derive(VTable, Debug)]
struct PlayerGameMovement {}

impl PlayerGameMovement {
    #[virtual_index(2)]
    pub fn process_movement(
        &self,
        player: *const CEntity,
        mve: *const PlayerMoveData,
    ) -> *const c_char {
    }

    #[virtual_index(3)]
    pub fn reset(&self) {}

    #[virtual_index(4)]
    pub fn start_track_prediction_errors(&self, player: *const CEntity) {}

    #[virtual_index(5)]
    pub fn finish_track_prediction_errors(&self, player: *const CEntity) {}

    #[virtual_index(7)]
    pub fn player_mins(&self, ducked: bool) -> Vec3 {}

    #[virtual_index(8)]
    pub fn player_maxs(&self, ducked: bool) -> Vec3 {}

    #[virtual_index(9)]
    pub fn player_view_offset(&self, ducked: bool) -> Vec3 {}

    #[virtual_index(10)]
    pub fn moving_player_stuck(&self) -> bool {}

    #[virtual_index(11)]
    pub fn get_moving_player(&self) -> CEntity {}

    #[virtual_index(12)]
    pub fn unblock_pusher(&self, player: &CEntity, pusher: &CEntity) {}

    #[virtual_index(13)]
    pub fn setup_movement_bounds(&self, mve: *const PlayerMoveData) {}
}
