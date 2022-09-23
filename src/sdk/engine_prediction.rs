use vtables::VTable;
use vtables_derive::{has_vtable, VTable};

#[has_vtable]
#[derive(VTable, Debug)]
pub struct Prediction {}

impl Prediction {}
