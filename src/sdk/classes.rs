use std::f32::EPSILON;
use std::ops::{Add, Div, DivAssign, Index, Mul, Sub};

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
        const IN_WALK = 1 << 18;
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

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Matrix4x3 {
    pub body: [[f32; 4]; 3],
    // float[3][4]
}

impl Matrix4x3 {
    pub fn origin(&self) -> Vec3 {
        Vec3 {
            x: self.body[0][3],
            y: self.body[1][3],
            z: self.body[2][3],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Matrix3x4 {
    pub body: [[f32; 3]; 4],
    // float[4][3]
}

impl Matrix3x4 {
    pub fn origin(&self) -> Vec3 {
        Vec3 {
            x: self.body[0][3],
            y: self.body[1][3],
            z: self.body[2][3],
        }
    }
}

impl Vec3 {
    pub fn mag_sqrt(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn mag(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, b: Vec3) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn normalized(&self) -> Vec3 {
        let mag = self.mag();
        Vec3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
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

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
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

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self = &mut Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
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
