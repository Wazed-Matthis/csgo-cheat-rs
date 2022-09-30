use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::ffi::c_char;

use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

use crate::{netvar, Vec3};

pub static WEAPON_MAP: OnceCell<HashMap<i32, &str>> = OnceCell::new();

#[non_exhaustive]
pub struct WeaponType;

impl WeaponType {
    pub const DEAGLE: i32 = 1;
    pub const ELITE: i32 = 2;
    pub const FIVESEVEN: i32 = 3;
    pub const GLOCK: i32 = 4;
    pub const AK47: i32 = 7;
    pub const AUG: i32 = 8;
    pub const AWP: i32 = 9;
    pub const FAMAS: i32 = 10;
    pub const G3SG1: i32 = 11;
    pub const GALIL: i32 = 13;
    pub const M249: i32 = 14;
    pub const M4A4: i32 = 16;
    pub const MAC10: i32 = 17;
    pub const P90: i32 = 19;
    pub const UMP45: i32 = 24;
    pub const XM1014: i32 = 25;
    pub const BIZON: i32 = 26;
    pub const MAG7: i32 = 27;
    pub const NEGEV: i32 = 28;
    pub const SAWEDOFF: i32 = 29;
    pub const TEC9: i32 = 30;
    pub const ZEUS: i32 = 31;
    pub const P2000: i32 = 32;
    pub const MP7: i32 = 33;
    pub const MP9: i32 = 34;
    pub const NOVA: i32 = 35;
    pub const P250: i32 = 36;
    pub const SCAR20: i32 = 38;
    pub const SG553: i32 = 39;
    pub const SSG08: i32 = 40;
    pub const KNIFE_T: i32 = 42;
    pub const FLASHBANG: i32 = 43;
    pub const HEGRENADE: i32 = 44;
    pub const SMOKE: i32 = 45;
    pub const MOLOTOV: i32 = 46;
    pub const DECOY: i32 = 47;
    pub const FIREBOMB: i32 = 48;
    pub const C4: i32 = 49;
    pub const MUSICKIT: i32 = 58;
    pub const KNIFE_CT: i32 = 59;
    pub const M4A1S: i32 = 60;
    pub const USPS: i32 = 61;
    pub const TRADEUPCONTRACT: i32 = 62;
    pub const CZ75A: i32 = 63;
    pub const REVOLVER: i32 = 64;
    pub const KNIFE_BAYONET: i32 = 500;
    pub const KNIFE_FLIP: i32 = 505;
    pub const KNIFE_GUT: i32 = 506;
    pub const KNIFE_KARAMBIT: i32 = 507;
    pub const KNIFE_M9_BAYONET: i32 = 508;
    pub const KNIFE_HUNTSMAN: i32 = 509;
    pub const KNIFE_FALCHION: i32 = 512;
    pub const KNIFE_BOWIE: i32 = 514;
    pub const KNIFE_BUTTERFLY: i32 = 515;
    pub const KNIFE_SHADOW_DAGGERS: i32 = 516;
}

pub fn init_weapon_map() {
    let mut weapon_map = HashMap::<i32, &str>::new();
    weapon_map.insert(WeaponType::DEAGLE, "F");
    weapon_map.insert(WeaponType::ELITE, "S");
    weapon_map.insert(WeaponType::FIVESEVEN, "U");
    weapon_map.insert(WeaponType::GLOCK, "C");
    weapon_map.insert(WeaponType::AK47, "B");
    weapon_map.insert(WeaponType::AUG, "E");
    weapon_map.insert(WeaponType::AWP, "R");
    weapon_map.insert(WeaponType::FAMAS, "T");
    weapon_map.insert(WeaponType::G3SG1, "I");
    weapon_map.insert(WeaponType::GALIL, "V");
    weapon_map.insert(WeaponType::M249, "Z");
    weapon_map.insert(WeaponType::M4A4, "W");
    weapon_map.insert(WeaponType::MAC10, "L");
    weapon_map.insert(WeaponType::P90, "M");
    weapon_map.insert(WeaponType::UMP45, "Q");
    weapon_map.insert(WeaponType::XM1014, "]");
    weapon_map.insert(WeaponType::BIZON, "D");
    weapon_map.insert(WeaponType::MAG7, "K");
    weapon_map.insert(WeaponType::NEGEV, "Z");
    weapon_map.insert(WeaponType::SAWEDOFF, "K");
    weapon_map.insert(WeaponType::TEC9, "C");
    weapon_map.insert(WeaponType::ZEUS, "Y");
    weapon_map.insert(WeaponType::P2000, "Y");
    weapon_map.insert(WeaponType::MP7, "X");
    weapon_map.insert(WeaponType::MP9, "D");
    weapon_map.insert(WeaponType::NOVA, "K");
    weapon_map.insert(WeaponType::P250, "Y");
    weapon_map.insert(WeaponType::SCAR20, "I");
    weapon_map.insert(WeaponType::SG553, "[");
    weapon_map.insert(WeaponType::SSG08, "N");
    weapon_map.insert(WeaponType::KNIFE_CT, "J");
    weapon_map.insert(WeaponType::FLASHBANG, "G");
    weapon_map.insert(WeaponType::HEGRENADE, "H");
    weapon_map.insert(WeaponType::SMOKE, "P");
    weapon_map.insert(WeaponType::MOLOTOV, "H");
    weapon_map.insert(WeaponType::DECOY, "G");
    weapon_map.insert(WeaponType::FIREBOMB, "H");
    weapon_map.insert(WeaponType::C4, "\\");
    weapon_map.insert(WeaponType::KNIFE_T, "J");
    weapon_map.insert(WeaponType::M4A1S, "W");
    weapon_map.insert(WeaponType::USPS, "Y");
    weapon_map.insert(WeaponType::CZ75A, "Y");
    weapon_map.insert(WeaponType::REVOLVER, "F");
    weapon_map.insert(WeaponType::KNIFE_BAYONET, "J");
    weapon_map.insert(WeaponType::KNIFE_FLIP, "J");
    weapon_map.insert(WeaponType::KNIFE_GUT, "J");
    weapon_map.insert(WeaponType::KNIFE_KARAMBIT, "J");
    weapon_map.insert(WeaponType::KNIFE_M9_BAYONET, "J");
    weapon_map.insert(WeaponType::KNIFE_HUNTSMAN, "J");
    weapon_map.insert(WeaponType::KNIFE_FALCHION, "J");
    weapon_map.insert(WeaponType::KNIFE_BOWIE, "J");
    weapon_map.insert(WeaponType::KNIFE_BUTTERFLY, "J");
    weapon_map.insert(WeaponType::KNIFE_SHADOW_DAGGERS, "J");
    WEAPON_MAP
        .set(weapon_map)
        .expect("Weapon Map has already been defined");
}

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
    pub max_speed: f32,
    max_speed_alt: f32,
    _pad10: [c_char; 0x64],
    recoil_magnitude: f32,
    recoil_magnitude_alt: f32,
    _pad11: [c_char; 0x10],
    recovery_time_stand: f32,
}
