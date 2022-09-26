use std::collections::BTreeMap;
use std::ffi::{c_void, CStr};
use std::sync::RwLock;

use log::debug;
use once_cell::sync::OnceCell;
use winapi::ctypes::c_char;

use crate::sdk::client::ClientClass;
use crate::INTERFACES;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(i32)]
pub enum PropType {
    Int = 0,
    Float,
    Vec,
    VecXY,
    String,
    Array,
    DataTable,
    Int64,
}

#[repr(C)]
pub union VariantData {
    pub float: f32,
    pub int: i32,
    pub string: *const c_char,
    pub data: *mut c_void,
    pub vector: [f32; 0x3],
    pub int64: i32,
}

#[repr(C)]
pub struct Variant {
    data: VariantData,
    prop_type: PropType,
}

#[repr(C)]
pub struct RecvProxy {
    recv_prop: *const RecvProp,
    value: Variant,
    element_index: i32,
    object_id: i32,
}

#[repr(C)]
pub struct RecvTable {
    pub recv_props: *const RecvProp,
    pub count: i32,
    decoder: *const c_void,
    table_name: *const c_char,
    initialized: bool,
    in_main_list: bool,
}

#[repr(C)]
pub struct RecvProp {
    pub prop_name: *const c_char,
    pub prop_type: PropType,
    prop_flags: i32,
    buffer_size: i32,
    inside_array: i32,
    extra_data_ptr: *const c_void,
    array_prop: *const RecvProp,
    array_length_proxy_fn: fn(struct_ptr: *mut c_void, object_id: i32, current_array_length: i32),
    pub proxy_fn: fn(data: *const RecvProxy, struct_ptr: *mut c_void, out_ptr: *mut c_void),
    data_table_proxy_fn:
        fn(prop: *const RecvProp, out_ptr: *mut *mut c_void, data_ptr: *mut c_void, object_id: i32),
    pub data_table: *mut RecvTable,
    pub offset: i32,
    element_stride: i32,
    elements_count: i32,
    parent_array_prop_name: *const c_char,
}

static NETVARS: OnceCell<RwLock<BTreeMap<String, usize>>> = OnceCell::new();

/// # Safety
pub unsafe fn store_props(group_name: String, table: *mut RecvTable, child_offset: usize) {
    for i in 0..(*table).count {
        let prop = (*table).recv_props.offset(i as isize).read();
        let child = prop.data_table;

        let var_name = CStr::from_ptr(prop.prop_name).to_str().unwrap();

        if var_name.chars().next().unwrap().is_numeric() {
            continue;
        }

        if var_name == "baseclass" {
            continue;
        }

        if !child.is_null() {
            let table_name = CStr::from_ptr(child.read().table_name)
                .to_str()
                .unwrap()
                .to_string();
            if prop.prop_type == PropType::DataTable && table_name.starts_with('D') {
                store_props(
                    group_name.to_string(),
                    child,
                    prop.offset as usize + child_offset,
                );
            }
        }
        let formatted = format!("{}->{}", group_name, var_name);
        debug!(
            "Netvar: {} offset: {}",
            formatted,
            prop.offset as usize + child_offset as usize
        );
        let mut guard = NETVARS.get().unwrap().write().unwrap();
        guard.insert(
            formatted.replacen('C', "DT_", 1),
            prop.offset as usize + child_offset as usize,
        );
    }
}

pub fn scan_netvars() {
    NETVARS.set(RwLock::new(BTreeMap::new())).unwrap();
    let interfaces = INTERFACES.get().unwrap();
    let mut client_class = interfaces.client.get_all_classes();
    while !client_class.is_null() {
        unsafe {
            let recv_table = client_class.read().recv_table;
            let table_name = CStr::from_ptr(client_class.read().network_name)
                .to_str()
                .unwrap()
                .to_string();
            store_props(table_name, recv_table, 0);

            client_class = client_class.read().next as *const ClientClass;
        }
    }
}

pub fn get_offset(table: &str, netvar: &str) -> usize {
    let guard = NETVARS.get().unwrap().read().unwrap();
    *guard.get(&*format!("{}->{}", table, netvar)).unwrap_or(&0)
}

#[macro_export]
macro_rules! netvar {
    ($table:literal, $name:literal, $func_name:ident, $return_type:ident) => {
        netvar!($table, $name, $func_name, $return_type, 0);
    };

    ($table:literal, $name:literal, $func_name:ident, $return_type:ident, $additional_offset:expr) => {
        pub fn $func_name(&self) -> $return_type {
            use crate::netvar::get_offset;
            let offset = get_offset($table, $name);
            let value = self.get_value::<$return_type>(offset + $additional_offset);
            value
        }
    };
}
