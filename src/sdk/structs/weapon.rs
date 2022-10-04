use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::ffi::c_char;
use std::mem::transmute;

use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

use crate::WeaponType::Elite;
use crate::{netvar, Vec3};
use num_enum::TryFromPrimitive;
use WeaponType::*;

pub static WEAPON_MAP: OnceCell<HashMap<WeaponType, &str>> = OnceCell::new();

#[repr(i16)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, TryFromPrimitive)]
pub enum WeaponType {
    Deagle = 1,
    Elite = 2,
    FiveSeven = 3,
    Glock = 4,
    AK47 = 7,
    AUG = 8,
    AWP = 9,
    Famas = 10,
    G3SG1 = 11,
    Galil = 13,
    M249 = 14,
    M4A4 = 16,
    MAC10 = 17,
    P90 = 19,
    UMP45 = 24,
    XM1014 = 25,
    Bizon = 26,
    MAG7 = 27,
    Negev = 28,
    SawedOff = 29,
    TEC9 = 30,
    Zeus = 31,
    P2000 = 32,
    MP7 = 33,
    MP9 = 34,
    Nova = 35,
    P250 = 36,
    SCAR20 = 38,
    SG553 = 39,
    SSG08 = 40,
    KniveT = 42,
    FlashBang = 43,
    HEGrenade = 44,
    Smoke = 45,
    Molotov = 46,
    Decoy = 47,
    FireBomb = 48,
    C4 = 49,
    MusicKit = 58,
    KniveCT = 59,
    M4A1S = 60,
    USPS = 61,
    TradeUpContract = 62,
    CZ75A = 63,
    Revolver = 64,
    KniveBayonet = 500,
    KniveFlip = 505,
    KniveGut = 506,
    KniveKarambit = 507,
    KniveM9Bayonet = 508,
    KniveHuntsman = 509,
    KniveFalchion = 512,
    KniveBowie = 514,
    KniveButterfly = 515,
    KniveShadowDaggers = 516,
}

pub fn init_weapon_map() {
    macro_rules! define_weapon_map {
        ($($key:expr => $val:literal),*) => {
            {
                let mut map = HashMap::<WeaponType, &str>::new();
            $(
                map.insert($key, $val);
            )*
            map
            }
        };
    }

    let weapon_map = define_weapon_map! {
        Elite => "S",
        FiveSeven => "U",
        Glock => "C",
        AK47 => "B",
        AUG => "E",
        AWP => "R",
        Famas => "T",
        G3SG1 => "I",
        Galil => "V",
        M249 => "Z",
        M4A4 => "W",
        MAC10 => "L",
        P90 => "M",
        UMP45 => "Q",
        XM1014 => "]",
        Bizon => "D",
        MAG7 => "K",
        Negev => "Z",
        SawedOff => "K",
        TEC9 => "C",
        Zeus => "Y",
        P2000 => "Y",
        MP7 => "X",
        MP9 => "D",
        Nova => "K",
        P250 => "Y",
        SCAR20 => "I",
        SG553 => "[",
        SSG08 => "N",
        KniveCT => "J",
        FlashBang => "G",
        HEGrenade => "H",
        Smoke => "P",
        Molotov => "H",
        Decoy => "G",
        FireBomb => "H",
        C4 => "\\",
        KniveT => "J",
        M4A1S => "W",
        USPS => "Y",
        CZ75A => "Y",
        Revolver => "F",
        KniveBayonet => "J",
        KniveFlip => "J",
        KniveGut => "J",
        KniveKarambit => "J",
        KniveM9Bayonet => "J",
        KniveHuntsman => "J",
        KniveFalchion => "J",
        KniveBowie => "J",
        KniveButterfly => "J",
        KniveShadowDaggers => "J"
    };
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
