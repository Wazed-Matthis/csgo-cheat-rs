use std::ffi::{c_short, c_void};

use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};
use winapi::ctypes::c_char;

use crate::sdk::structs::entities::CEntity;
use crate::Vec3;

pub mod hit_group {
    use core::convert::TryFrom;

    #[repr(i32)]
    #[derive(Debug, Eq, PartialOrd, PartialEq)]
    pub enum HitGroup {
        Invalid = -1,
        Generic = 0,
        Head = 1,
        Chest = 2,
        Stomach = 3,
        LeftArm = 4,
        RightArm = 5,
        LeftLeg = 6,
        RightLeg = 7,
        Gear = 10,
    }

    impl TryFrom<i32> for HitGroup {
        type Error = ();

        fn try_from(v: i32) -> Result<Self, Self::Error> {
            match v {
                x if x == HitGroup::Head as i32 => Ok(HitGroup::Head),
                x if x == HitGroup::Chest as i32 => Ok(HitGroup::Chest),
                x if x == HitGroup::Stomach as i32 => Ok(HitGroup::Stomach),
                x if x == HitGroup::LeftArm as i32 => Ok(HitGroup::LeftArm),
                x if x == HitGroup::RightArm as i32 => Ok(HitGroup::RightArm),
                x if x == HitGroup::RightLeg as i32 => Ok(HitGroup::RightLeg),
                _ => Err(()),
            }
        }
    }

    pub fn get_damage_multiplier(hit_group: i32, headshot_mul: f32) -> f32 {
        if let Ok(hit_group) = HitGroup::try_from(hit_group) {
            return match hit_group {
                HitGroup::Head => headshot_mul,
                HitGroup::Stomach => 1.25,
                HitGroup::LeftLeg | HitGroup::RightLeg => 0.75,
                _ => 1.0,
            };
        }
        1.0
    }

    pub fn is_armored(hit_group: i32, helmet: bool) -> bool {
        if let Ok(hit_group) = HitGroup::try_from(hit_group) {
            return match hit_group {
                HitGroup::Head => helmet,
                HitGroup::Chest => true,
                HitGroup::Stomach => true,
                HitGroup::LeftArm => true,
                HitGroup::RightArm => true,
                _ => false,
            };
        }

        false
    }
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Default)]
struct VectorAligned {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl From<Vec3> for VectorAligned {
    fn from(vec: Vec3) -> Self {
        Self {
            x: vec.x as f32,
            y: vec.y as f32,
            z: vec.z as f32,
            w: 0e0,
        }
    }
}

impl From<VectorAligned> for Vec3 {
    fn from(vec_aligned: VectorAligned) -> Self {
        Self {
            x: vec_aligned.x,
            y: vec_aligned.y,
            z: vec_aligned.z,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Entity {
    pub vtable: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ray {
    start: VectorAligned,
    delta: VectorAligned,
    start_offset: VectorAligned,
    extents: VectorAligned,
    pub p_world_axis_transform_matrix: *const c_void,
    pub is_ray: bool,
    pub is_swept: bool,
}

impl Ray {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        let mut instance = unsafe { core::mem::zeroed::<Self>() };
        instance.delta = VectorAligned::from(end - start);
        instance.start = VectorAligned::from(start);
        instance.is_swept = Vec3::from(instance.delta).mag() != 0e0;
        instance.is_ray = true;
        instance
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CSurface {
    pub name: *const c_char,
    pub surface_props: c_short,
    pub flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CPlane {
    pub normal: Vec3,
    pub dist: f32,
    pub r#type: c_char,
    pub sign_bit: c_char,
    pad: [c_char; 0x2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Trace {
    pub start: Vec3,
    pub end: Vec3,
    pub plane: CPlane,
    pub fraction: f32,
    pub contents: i32,
    pub disp_flags: u16,
    pub all_solid: bool,
    pub start_solid: bool,
    pub fraction_solid_left: f32,
    pub surface: CSurface,
    pub hit_group: i32,
    pub physics_bone: c_short,
    pub worldSurfaceIndex: u16,
    pub ptr_entity: *const CEntity,
    pub hitbox: i32,
}

#[repr(C)]
pub enum TraceType {
    EVERYTHING = 0,
    WorldOnly = 1,
    EntitiesOnly = 2,
    EverythingFilterProps = 3,
}

pub trait TraceFilterTrait {
    fn should_hit_entity(&self, entity: &Entity, contents_mask: u32) -> bool;
    fn get_trace_type(&self) -> TraceType;
    fn new(skip: *const usize) -> Self;
}

#[repr(C)]
pub struct TraceFilterGeneric {
    vtable: usize,
    skip: *const Entity,
    vec_vtable: Vec<usize>,
}

impl TraceFilterTrait for TraceFilterGeneric {
    fn should_hit_entity(&self, entity: &Entity, _: u32) -> bool {
        entity as *const _ as usize != self.skip as *const _ as usize
    }

    fn get_trace_type(&self) -> TraceType {
        TraceType::EVERYTHING
    }

    fn new(skip: *const usize) -> Self {
        extern "thiscall" fn should_hit_entity_wrapper(
            this: &TraceFilterGeneric,
            entity: &Entity,
            contents_mask: u32,
        ) -> bool {
            this.should_hit_entity(entity, contents_mask)
        }

        extern "thiscall" fn get_trace_type_wrapper(this: &TraceFilterGeneric) -> TraceType {
            this.get_trace_type()
        }

        let mut vec = Vec::<usize>::new();

        vec.push(should_hit_entity_wrapper as usize);
        vec.push(get_trace_type_wrapper as usize);

        Self {
            vtable: vec.as_ptr() as _,
            vec_vtable: vec,
            skip: unsafe { &*(skip as *const Entity) },
        }
    }
}

pub const MASK_SHOT: i32 = 0x1 | 0x4000 | 0x2000000 | 0x2 | 0x4000000 | 0x40000000;
pub const MASK_SHOT_HULL: i32 = 0x1 | 0x4000 | 0x2000000 | 0x2 | 0x4000000 | 0x8;
pub const CONTENTS_GRATE: i32 = 0x8;
pub const CONTENTS_HITBOX: i32 = 0x40000000;
pub const SURF_HITBOX: u16 = 0x8000;
pub const SURF_LIGHT: u16 = 0x0001;
pub const SURF_NODRAW: u16 = 0x0080;

pub const CHAR_TEX_ANTLION: char = 'A';
pub const CHAR_TEX_BLOODYFLESH: char = 'B';
pub const CHAR_TEX_CONCRETE: char = 'C';
pub const CHAR_TEX_DIRT: char = 'D';
pub const CHAR_TEX_EGGSHELL: char = 'E';
pub const CHAR_TEX_FLESH: char = 'F';
pub const CHAR_TEX_GRATE: char = 'G';
pub const CHAR_TEX_ALIENFLESH: char = 'H';
pub const CHAR_TEX_CLIP: char = 'I';
pub const CHAR_TEX_PLASTIC: char = 'L';
pub const CHAR_TEX_METAL: char = 'M';
pub const CHAR_TEX_SAND: char = 'N';
pub const CHAR_TEX_FOLIAGE: char = 'O';
pub const CHAR_TEX_COMPUTER: char = 'P';
pub const CHAR_TEX_SLOSH: char = 'S';
pub const CHAR_TEX_TILE: char = 'T';
pub const CHAR_TEX_CARDBOARD: char = 'U';
pub const CHAR_TEX_VENT: char = 'V';
pub const CHAR_TEX_WOOD: char = 'W';
pub const CHAR_TEX_GLASS: char = 'Y';
pub const CHAR_TEX_WARPSHIELD: char = 'Z';

#[has_vtable]
#[derive(VTable, Debug)]
pub struct EngineTrace {}

impl EngineTrace {
    #[virtual_index(0)]
    pub fn get_point_contents(
        &self,
        abs_pos: &Vec3,
        contents_mask: i32,
        entity: *mut usize,
    ) -> i32 {
    }

    #[virtual_index(4)]
    pub fn clip_ray_to_entity(&self, ray: &Ray, mask: u32, ent: &mut Entity, trace: &mut Trace) {}

    #[virtual_index(5)]
    pub fn trace_ray_virtual(&self, ray: &Ray, mask: u32, filter: *mut usize, trace: &mut Trace) {}
}
