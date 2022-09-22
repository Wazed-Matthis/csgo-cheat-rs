use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

use crate::sdk::classes::Vec3;

#[has_vtable]
#[derive(VTable, Debug)]
pub struct Collidable {}

impl Collidable {
    #[virtual_index(1)]
    pub fn min(&self) -> &'static Vec3 {}
    #[virtual_index(2)]
    pub fn max(&self) -> &'static Vec3 {}
}
