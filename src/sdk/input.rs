use std::ffi::{c_char, c_void};

use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

use crate::{CUserCMD, Vec3};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CInput {
    _pad3: [c_char; 0x8],
    pub m_trackir: bool,
    pub m_mouse_init: bool,
    pub m_mouse_active: bool,
    pub m_joystick_adv_init: bool,
    pub _pad0: [c_char; 0x2C],
    pub m_keys: *const c_void,
    pub _pad1: [c_char; 0x6C],
    pub m_camera_intercepting_mouse: bool,
    pub m_camera_in_third_person: bool,
    pub m_camera_moving_with_mouse: bool,
    pub m_camera_offset: Vec3,
    pub m_camera_distance_move: bool,
    pub m_camera_old_x: i32,
    pub m_camera_old_y: i32,
    pub m_camera_x: i32,
    pub m_camera_y: i32,
    pub m_camera_is_orthographic: bool,
    pub m_previous_view_angles: Vec3,
    pub m_previous_view_angles_tilt: Vec3,
    pub m_last_forward_move: f32,
    pub m_clear_input_state: i32,
    pub m_commands: *const CUserCMD,
    pub _pad2: [c_char; 0x1],
}

#[has_vtable]
#[derive(VTable, Debug)]
pub struct Input {}

impl Input {
    #[virtual_index(35)]
    pub fn enable_third_person(&self) {}

    #[virtual_index(36)]
    pub fn enable_first_person(&self) {}

    pub fn is_third_person(&self) -> bool {
        self.get_value::<bool>(self.vtable as usize + 0xAD)
    }
}
