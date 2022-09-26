use std::ops::{Add, Mul, Sub};

use bitflags::bitflags;

bitflags! {
    #[repr(C)]
    pub struct EButtons: i32 {
        const ATTACK = 1 << 0;
        const JUMP = 1 << 1;
        const DUCK = 1 << 2;
        const FORWARD = 1 << 3;
        const BACK = 1 << 4;
        const USE = 1 << 5;
        const MOVE_LEFT = 1 << 9;
        const MOVE_RIGHT = 1 << 10;
        const ATTACK2= 1 << 11;
        const SCORE = 1 << 16;
        const BULL_RUSH = 1 << 22;
    }
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn len_sqr(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
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

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Clone, Debug, Copy)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
