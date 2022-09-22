use crate::Vec3;
use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

#[has_vtable]
#[derive(VTable, Debug)]
pub struct DebugOverlay {}

impl DebugOverlay {
    #[virtual_index(13)]
    pub fn world_to_screen(&self, input: &mut Vec3, output: &mut Vec3) -> i32 {}
}
