use std::ffi::{c_char, c_float};

#[repr(C)]
pub struct CUtlVec<T> {
    pub memory: *mut T,
    pub allocation_count: i32,
    pub grow_size: i32,
    pub size: i32,
    pub elements: *mut T,
}

impl<T> CUtlVec<T> {
    pub fn get_mut(&mut self, index: i32) -> Option<&mut T> {
        unsafe {
            core::slice::from_raw_parts_mut(self.memory, self.size as usize).get_mut(index as usize)
        }
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        unsafe { core::slice::from_raw_parts(self.memory, self.size as usize).get(index as usize) }
    }

    pub fn size(&self) -> i32 {
        self.size
    }
}

#[repr(i32)]
#[derive(Debug)]
pub enum EButtons {
    InAttack = 1 << 0,
    InJump = 1 << 1,
    InDuck = 1 << 2,
    InForward = 1 << 3,
    InBack = 1 << 4,
    InUse = 1 << 5,
    InMoveleft = 1 << 9,
    InMoveright = 1 << 10,
    InAttack2 = 1 << 11,
    InScore = 1 << 16,
    InBullrush = 1 << 22,
}

#[repr(C)]
#[derive(Debug)]
pub struct CUserCMD {
    pub destructor: *const *const fn(),
    pub command_number: i32,
    pub tick_count: i32,
    pub view_angles: Vec3,
    pub aim_direction: Vec2,
    pub forward_move: f32,
    pub side_move: f32,
    pub up_move: f32,
    pub buttons: EButtons,
    pub impulse: u8,
    pub weapon_select: i32,
    pub weapon_subtype: i32,
    pub random_seed: i32,
    pub mouse_dx: i16,
    pub mouse_dy: i16,
    pub has_been_predicted: bool,
    pub pad: [u8; 0x18],
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
