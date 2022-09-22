use crate::memory::NotNull;
use crate::sdk::structs::entities::CEntity;
use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

#[has_vtable]
#[derive(VTable, Debug)]
pub struct EntityList {}

impl EntityList {
    #[virtual_index(3)]
    pub fn entity(&self, index: i32) -> NotNull<CEntity> {}

    #[virtual_index(5)]
    pub fn number_of_entities(&self, include_networkable: bool) -> i32 {}

    #[virtual_index(6)]
    pub fn highest_entity_index(&self) -> i32 {}

    #[virtual_index(8)]
    pub fn get_max_entities(&self) -> i32 {}
}
