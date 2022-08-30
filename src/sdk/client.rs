use std::ffi::c_char;

use crate::sdk::recv_props::CRecvTable;
use vtables::VTable;
use vtables_derive::has_vtable;
use vtables_derive::virtual_index;
use vtables_derive::VTable;

type CreateClientClassFn = extern "system" fn(ent: i32, serial: i32);
type CreateEventFn = extern "system" fn();

#[repr(C)]
pub struct ClientClass {
    create_client_class: CreateClientClassFn,
    create_event: CreateEventFn,
    pub network_name: *mut c_char,
    pub recv_table: *mut CRecvTable,
    pub next: *mut usize,
    pub class_id: i32,
}

#[has_vtable]
#[derive(VTable, Debug)]
pub struct Client {}

impl Client {
    #[virtual_index(8)]
    pub fn get_all_classes(&self) -> *const ClientClass {}
}
