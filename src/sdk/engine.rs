use std::ffi::c_char;

use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

#[has_vtable]
#[derive(VTable, Debug)]
pub struct EngineClient {}

impl EngineClient {
    #[virtual_index(5)]
    pub fn screen_size(&self, width: *mut i32, height: *mut i32) {}

    #[virtual_index(26)]
    pub fn is_in_game(&self) -> bool {}

    #[virtual_index(12)]
    pub fn local_player(&self) -> i32 {}

    #[virtual_index(53)]
    pub fn get_level_name(&self) -> *const c_char {}

    #[virtual_index(108)]
    pub fn execute_client_command(&self, command: *const c_char) {}

    #[virtual_index(113)]
    pub fn execute_client_command_unrestricted(&self, command: *const c_char) {}
}
