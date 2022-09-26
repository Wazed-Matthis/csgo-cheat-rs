use crate::sdk::structs::weapon::WeaponInfo;
use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

#[has_vtable]
#[derive(VTable, Debug)]
pub struct WeaponSystem {}

impl WeaponSystem {
    #[virtual_index(3)]
    pub fn get_weapon_data(&self, index: u32) -> *const WeaponInfo {}
}
