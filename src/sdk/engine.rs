use crate::{define_interface, vfunc};

define_interface!(Engine);

impl Engine {
    vfunc!(26, fn is_ingame() => bool);
}
