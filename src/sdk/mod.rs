use crate::lpcstr;
use crate::macros::Interface;
use once_cell::sync::OnceCell;
use std::any::Any;
use std::collections::HashMap;
use std::ffi::{c_char, c_int, c_void};
use std::mem::transmute;
use std::ptr::null_mut;
use std::sync::RwLock;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::winnt::INT;

pub mod classes;
pub mod client_mode;
pub mod engine;
pub mod entity_list;

// function signature of the CreateInterface function in cs:go
type CreateInterfaceFn =
    extern "system" fn(name: *const c_char, return_code: *const c_int) -> *const c_void;

static INTERFACES: OnceCell<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>> = OnceCell::new();

pub fn init_interface<T: Interface>(interface_name: &str, module: &str) {
    INTERFACES.set(RwLock::new(HashMap::new())).unwrap();

    // get the pointer to the create interface function
    let create_interface = unsafe {
        GetProcAddress(
            GetModuleHandleA(format!("{}\0", module).as_ptr() as winapi::um::winnt::LPCSTR),
            "CreateInterface\0".as_ptr() as winapi::um::winnt::LPCSTR,
        )
    };

    let create_interface = unsafe {
        std::mem::transmute::<_, fn(name: *const c_char, return_code: *const c_int) -> *const c_void>(
            create_interface,
        )
    };
    log::debug!("Capturing interface {}...", interface_name);
    let addr = create_interface(lpcstr!(interface_name), null_mut()) as usize;
    if addr != 0 {
        log::debug!("Captured interface {}, addr.: {:x}", interface_name, addr);
        let interface = Box::new(T::new(addr).as_any());
        INTERFACES
            .get()
            .unwrap()
            .write()
            .unwrap()
            .insert(interface_name.into(), interface);
    } else {
        log::error!("Failed to capture interface {}", interface_name);
    }
    // call create interface function and create interface instance from it
}

pub fn get_interface<T: Interface>(interface_name: &str) -> Option<&T> {
    let interfaces = INTERFACES.get().unwrap().read().unwrap();
    if let Some(addr) = interfaces.get(interface_name) {
        Some(addr.downcast_ref::<&T>().unwrap())
    } else {
        log::error!("Failed to retrieve interface {}", interface_name);
        None
    }
}

pub fn get_interface_raw(interface: &str, module: &str) -> *const c_void {
    // get the pointer to the create interface function
    let create_interface = unsafe {
        GetProcAddress(
            GetModuleHandleA(format!("{}\0", module).as_ptr() as winapi::um::winnt::LPCSTR),
            "CreateInterface\0".as_ptr() as winapi::um::winnt::LPCSTR,
        )
    };

    let create_interface = unsafe {
        std::mem::transmute::<_, fn(name: *const c_char, return_code: *const c_int) -> *const c_void>(
            create_interface,
        )
    };
    log::debug!("Capturing interface {}...", interface);
    let addr = create_interface(lpcstr!(interface), null_mut());
    if !addr.is_null() {
        log::debug!("Captured interface {}, addr.: {:x?}", interface, addr);
    } else {
        log::error!("Failed to capture interface {}", interface);
    }
    // call create interface function and create interface instance from it
    addr
}
