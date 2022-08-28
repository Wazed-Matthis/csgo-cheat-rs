use crate::lpcstr;
use crate::macros::Interface;
use once_cell::sync::OnceCell;
use std::ffi::{c_char, c_int, c_void};
use std::mem::transmute;
use std::ptr::null_mut;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};

pub mod classes;
pub mod client_mode;
pub mod entity_list;

// function signature of the CreateInterface function in cs:go
type CreateInterfaceFn = fn(name: *const c_char, return_code: *const c_int) -> *const c_void;

static CREATE_INTERFACE: OnceCell<CreateInterfaceFn> = OnceCell::new();

pub fn get_interface<T: Interface>(interface: &str, module: &str) -> T {
    // get the pointer to the create interface function
    let create_interface = CREATE_INTERFACE.get_or_init(|| unsafe {
        // address to function
        let create_interface_ptr = GetProcAddress(
            GetModuleHandleA(lpcstr!(module)),
            lpcstr!("CreateInterface"),
        );

        // transmute to correct rust type
        transmute::<_, CreateInterfaceFn>(create_interface_ptr)
    });

    // call create interface function and create interface instance from it
    T::new(create_interface(lpcstr!(interface), null_mut()) as usize)
}
