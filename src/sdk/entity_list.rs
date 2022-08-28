use crate::{define_interface, vfunc};

define_interface!(EntityList);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CEntity {
    base_address: usize,
}

impl CEntity {
    pub fn from_raw(base_address: usize) -> Self {
        Self { base_address }
    }
}

impl CEntity {
    vfunc!(155, fn is_player() => bool);
}

impl EntityList {
    vfunc!(3, fn get_entity(index: i32) => *const CEntity);
    vfunc!(6, fn get_highest_entity_index() => i32);
}
