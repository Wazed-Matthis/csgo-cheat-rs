use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

use crate::{define_interface, vfunc};
use crate::memory::NotNull;
use crate::sdk::classes::Vec3;

#[has_vtable]
#[derive(VTable, Debug)]
pub struct CEntity {}

impl CEntity {

    #[virtual_index(122)]
    pub fn health(&self) -> i32 {}

    #[virtual_index(156)]
    pub fn is_alive(&self) -> bool {}

    #[virtual_index(158)]
    pub fn is_player(&self) -> bool {}

    #[virtual_index(10)]
    pub fn get_abs_origin(&self) -> &'static Vec3 {}

    #[virtual_index(88)]
    pub fn get_team(&self) -> i32{}
    
    #[virtual_index(166)]
    pub fn is_weapon(&self) -> bool{}
}

#[has_vtable]
#[derive(VTable, Debug)]
pub struct EntityList {}

impl EntityList {
    #[virtual_index(3)]
    pub fn entity(&self, index: i32) -> NotNull<CEntity> {}
    #[virtual_index(6)]
    pub fn highest_entity_index(&self) -> i32 {}
}
