use std::ffi::c_char;

use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};
use crate::Vec3;

#[repr(C)]
pub union PlayerInfoUnion {
    xuid: i64,
    low_high: (i32, i32),
}

#[repr(C)]
pub struct PlayerInfo {
    version: u32,
    xuid: u32,
    pub xuid_low: u32,
    xuid_high: u32,
    pub name: [c_char; 0x80],
    pub user_id: i32,
    _guid: [c_char; 0x21],
    _friends_id: u32,
    _friends_name: [i32; 0x80],
    pub fake_player: bool,
    hltv: bool,
    _customfiles: [i32; 0x4],
    _files_downloaded: u8,
}

#[has_vtable]
#[derive(VTable, Debug)]
pub struct EngineClient {}

impl EngineClient {
    #[virtual_index(5)]
    pub fn screen_size(&self, width: *mut i32, height: *mut i32) {}

    #[virtual_index(26)]
    pub fn is_in_game(&self) -> bool {}

    #[virtual_index(12)]
    pub fn local_player(&self) -> i32 {}

    #[virtual_index(53)]
    pub fn get_level_name(&self) -> *const c_char {}

    #[virtual_index(108)]
    pub fn execute_client_command(&self, command: *const c_char) {}

    #[virtual_index(113)]
    pub fn execute_client_command_unrestricted(&self, command: *const c_char) {}

    #[virtual_index(8)]
    pub fn player_info(&self, index: i32, player_info: &mut PlayerInfo) {}

    #[virtual_index(18)]
    pub fn get_view_angles(&self, angles: &mut Vec3) {}
}
