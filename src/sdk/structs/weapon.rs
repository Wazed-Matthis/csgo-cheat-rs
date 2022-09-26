use std::ffi::c_char;

use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

use crate::{netvar, Vec3};

#[has_vtable]
#[derive(VTable, Debug, Clone)]
pub struct Weapon {}

impl Weapon {
    #[virtual_index(479)]
    pub fn inaccuracy(&self) -> f32 {}

    #[virtual_index(449)]
    pub fn spread(&self) -> f32 {}

    #[virtual_index(480)]
    pub fn update_accuracy_penalty(&self) {}

    #[virtual_index(461)]
    pub fn get_weapon_data(&self) -> &'static WeaponInfo {}

    netvar!(
        "DT_BaseCombatWeapon",
        "m_flNextPrimaryAttack",
        next_attack,
        f32
    );
    netvar!(
        "DT_BaseAttributableItem",
        "m_nFallbackPaintKit",
        get_fallback_paint_kit,
        u32
    );
    netvar!(
        "DT_BaseAttributableItem",
        "m_iEntityQuality",
        get_entity_quality,
        i32
    );
    netvar!(
        "DT_BaseAttributableItem",
        "m_nFallbackSeed",
        get_fallback_seed,
        u32
    );
    netvar!(
        "DT_BaseAttributableItem",
        "m_nFallbackStatTrak",
        get_fallback_stat_track,
        i32
    );
    netvar!(
        "DT_BaseAttributableItem",
        "m_flFallbackWear",
        get_fallback_wear,
        f32
    );
    netvar!(
        "DT_BaseCombatWeapon",
        "m_hWeaponWorldModel",
        get_weapon_world_model,
        i32
    );
    netvar!("DT_BaseAttributableItem", "m_iItemIDHigh", get_id_high, i32);
    netvar!(
        "DT_BaseAttributableItem",
        "m_iItemDefinitionIndex",
        get_id,
        i16
    );
    netvar!("DT_BaseEntity", "m_nModelIndex", get_model_index, u32);
    netvar!(
        "DT_BaseAttributableItem",
        "m_iAccountID",
        get_account_id,
        u32
    );
    netvar!("DT_CSPlayer", "m_hOwnerEntity", get_owner_entity, i32);
    netvar!("DT_BaseEntity", "m_vecOrigin", get_origin, Vec3);
    netvar!("DT_BaseCombatWeapon", "m_iClip1", get_clip, i32);
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WeaponInfo {
    _pad0: [c_char; 0x14],
    pub max_clip: i32,
    _pad1: [c_char; 0x70],
    pub name: *const c_char,
    _pad2: [c_char; 0x3C],
    weapon_type: i32,
    _pad3: [c_char; 0x4],
    price: i32,
    _pad4: [c_char; 0x8],
    cycle_time: f32,
    _pad5: [c_char; 0xC],
    full_auto: bool,
    _pad6: [c_char; 0x3],
    pub damage: i32,
    pub headshot_mult: f32,
    pub armor_ratio: f32,
    bullets: i32,
    pub penetration: f32,
    _pad7: [c_char; 0x8],
    pub range: f32,
    pub range_modifier: f32,
    _pad8: [c_char; 0x10],
    silencer: bool,
    _pad9: [c_char; 0xF],
    max_speed: f32,
    max_speed_alt: f32,
    _pad10: [c_char; 0x64],
    recoil_magnitude: f32,
    recoil_magnitude_alt: f32,
    _pad11: [c_char; 0x10],
    recovery_time_stand: f32,
}
