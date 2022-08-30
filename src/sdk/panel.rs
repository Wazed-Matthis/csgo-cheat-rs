use std::ffi::c_char;
use vtables::VTable;
use vtables_derive::*;

#[has_vtable]
#[derive(VTable, Debug)]
pub struct Panel {}

impl Panel {
    #[virtual_index(36)]
    pub fn get_panel_name(&self, panel_id: u32) -> *const c_char {}
}
