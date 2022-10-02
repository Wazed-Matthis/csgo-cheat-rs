use crate::sdk::classes::Matrix3x4;
use crate::Vec3;
use std::ffi::c_char;

#[repr(C)]
#[derive(Debug)]
pub struct ViewRender {
    _pad1: [c_char; 0x4],
    pub view: ViewSetup,
}

#[repr(C)]
#[derive(Debug)]
pub struct ViewSetup {
    pub x: i32,
    pub old_x: i32,
    pub y: i32,
    pub old_y: i32,
    pub width: i32,
    pub old_width: i32,
    pub height: i32,
    pub old_height: i32,
    pub ortho: bool,
    pub ortho_left: f32,
    pub ortho_top: f32,
    pub ortho_right: f32,
    pub ortho_bottom: f32,
    pub custom_view_matrix: bool,
    pub custom_matrix: Matrix3x4,
    pub _pad: [c_char; 0x48],
    pub fov: f32,
    pub view_model_fov: f32,
    pub origin: Vec3,
    pub angles: Vec3,
}
