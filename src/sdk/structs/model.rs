use crate::Vec3;
use log::{debug, error};
use std::ffi::{c_char, c_void};
use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

#[repr(C)]
#[derive(Debug)]
pub struct Model {
    pub handle: *const c_void,
    pub name: [c_char; 0x104],
    pub load_flags: i32,
    pub server_count: i32,
    pub model_type: i32,
    pub flags: i32,
    pub mins: Vec3,
    pub maxs: Vec3,
    pub radius: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct StudioModel {
    pub id: i32,
    pub version: i32,
    pub checksum: i32,
    pub name: [c_char; 0x40],
    pub length: i32,
    pub eye_pos: Vec3,
    pub illum_pos: Vec3,
    pub hull_mins: Vec3,
    pub hull_maxs: Vec3,
    pub view_mins: Vec3,
    pub view_maxs: Vec3,
    pub flags: i32,
    pub bone_count: i32,
    pub bone_id: i32,
    pub controller_count: i32,
    pub controller_id: i32,
    pub set_count: i32,
    pub set_id: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StudioBone {
    pub bone_name_index: i32,
    pub parent: i32,
    pub _pad1: [c_char; 0x98],
    pub flags: i32,
    pub _pad2: [c_char; 0x34],
}

impl StudioModel {
    pub fn bone(&self, index: i32) -> Option<StudioBone> {
        unsafe {
            let self_ptr = self as *const Self as *const u8;
            let bone_ptr = self_ptr.offset(self.bone_id as isize) as *const StudioBone;
            let model_ptr = bone_ptr.offset(index as isize);
            if model_ptr.is_null() {
                return None;
            }
            Some(*(model_ptr))
        }
    }
}

#[has_vtable]
#[derive(VTable, Debug)]
pub struct ModelInfo {}

impl ModelInfo {
    #[virtual_index(32)]
    pub fn studio_model(&self, model: *const Model) -> *const StudioModel {}
}
