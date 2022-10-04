use std::ffi::{c_char, c_short};
use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

#[derive(Clone, Debug, Copy)]
pub struct SurfaceData {
    pad: [c_char; 80],
    max_speed_factor: f32,
    jump_factor: f32,
    pub penetration_modifier: f32,
    damage_modifier: f32,
    pub material: c_short,
    climbable: bool,
}

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SurfaceProps {}

impl SurfaceProps {
    #[virtual_index(5)]
    pub fn surface_data(&self, index: i32) -> &'static SurfaceData {}
}
