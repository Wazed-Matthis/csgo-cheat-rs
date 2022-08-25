#[derive(Clone, Debug)]
#[repr(C)]
pub struct CUserCmd {
    pub destructor: *const *const fn(),
    pub command_number: i32,
    pub tick_count: i32,
    pub view_angles: Vec3,
    pub aim_direction: Vec3,
    pub forward_move: f32,
    pub side_move: f32,
    pub up_move: f32,
    pub buttons: i32,
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
