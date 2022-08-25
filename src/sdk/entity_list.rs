use crate::{define_interface, vfunc};

use std::ffi::c_void;

define_interface!(EntityList);

impl EntityList {
    vfunc!(3,fn get_entity(index: i32) => *const c_void);
}
