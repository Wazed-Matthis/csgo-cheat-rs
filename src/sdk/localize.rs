use std::ffi::c_char;

use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};
use winapi::ctypes::wchar_t;

#[has_vtable]
#[derive(VTable, Debug)]
pub struct Localize {}
impl Localize {
    #[virtual_index(12)]
    pub fn find(&self, token_name: *const c_char) -> *const wchar_t {}
}
