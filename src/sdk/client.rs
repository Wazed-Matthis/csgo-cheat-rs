use std::ffi::c_char;

use crate::netvar::RecvTable;
use vtables::VTable;
use vtables_derive::has_vtable;
use vtables_derive::virtual_index;
use vtables_derive::VTable;

#[repr(C)]
pub struct ClientClass {
    create_client_class: extern "system" fn(ent: i32, serial: i32),
    create_event: extern "system" fn(),
    pub network_name: *mut c_char,
    pub recv_table: *mut RecvTable,
    pub next: *mut usize,
    pub class_id: i32,
}

#[has_vtable]
#[derive(VTable, Debug, Clone)]
pub struct Client {}

impl Client {
    #[virtual_index(8)]
    pub fn get_all_classes(&self) -> *const ClientClass {}
}
