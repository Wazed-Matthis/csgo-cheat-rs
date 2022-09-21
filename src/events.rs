use event_bus::Event;

use crate::CUserCMD;

pub struct EventCreateMove {
    pub(crate) user_cmd: *mut CUserCMD,
}

impl Event for EventCreateMove {}

pub struct EventPaintTraverse {}

impl Event for EventPaintTraverse {}
