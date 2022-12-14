use event_bus::Event;

use crate::sdk::classes::Stage;
use crate::{CUserCMD, ViewSetup};

pub struct EventCreateMove {
    pub user_cmd: *mut CUserCMD,
}

impl Event for EventCreateMove {}

pub struct EventPaintTraverse {}

impl Event for EventPaintTraverse {}

pub struct EventFrameStageNotify {
    pub stage: Stage,
}

impl Event for EventFrameStageNotify {}

pub struct EventOverrideView {
    pub setup: *mut ViewSetup,
}

impl Event for EventOverrideView {}
