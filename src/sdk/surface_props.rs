use std::ffi::c_char;
use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

#[derive(Clone, Debug, Copy)]
pub struct SurfaceData {
    pad: [c_char; 80],
    max_speed_factor: f32,
    jump_factor: f32,
    pub(crate) penetration_modifier: f32,
    damage_modifier: f32,
    pub(crate) material: i16,
    climbable: bool,
}

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SurfaceProps {}

impl SurfaceProps {
    #[virtual_index(5)]
    pub fn surface_data(&self, index: i32) -> *const SurfaceData {}
}
