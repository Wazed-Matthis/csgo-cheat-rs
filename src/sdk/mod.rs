use std::collections::HashMap;
use std::ffi::{c_char, c_int, c_void};
use std::mem::transmute;
use std::ptr::null_mut;
use std::sync::RwLock;

use once_cell::sync::OnceCell;
use vtables::VTable;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::winnt::INT;

use crate::lpcstr;
use crate::macros::Interface;

pub mod classes;
pub mod client;
pub mod client_mode;
pub mod engine;
pub mod entity_list;
pub mod global_vars;
pub mod glow_object;
pub mod netvars;
pub mod panel;
pub mod recv_props;
pub mod surface;
