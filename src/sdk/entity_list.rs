use crate::{define_interface, vfunc};

define_interface!(EntityList);

#[repr(C)]
pub struct CEntity {}

impl EntityList {
    // vfunc!(3, fn get_entity(index: i32) => *const c_void);
    vfunc!(6, fn get_highest_entity_index() => i32);
}
