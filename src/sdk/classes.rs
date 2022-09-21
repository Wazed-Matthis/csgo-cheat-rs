use std::ffi::c_float;
use std::ops::Add;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(i32)]
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

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
