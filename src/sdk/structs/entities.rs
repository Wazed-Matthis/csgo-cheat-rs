use std::cmp::max;
use std::ffi::{c_int, c_void};
use std::mem;
use std::mem::size_of;

use log::{debug, error};
use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

use crate::get_interfaces;
use crate::memory::NotNull;
use crate::sdk::classes::{Matrix3x4, Matrix4x3, Vec2, Vec3};
use crate::sdk::structs::collidable::Collidable;
use crate::sdk::structs::weapon::Weapon;
use crate::{netvar, Client};

type OriginalFn = unsafe extern "thiscall" fn(*mut c_void, *mut Matrix4x3, i32, i32, f32) -> bool;

#[has_vtable]
#[derive(VTable, Debug)]
pub struct CEntity {}

impl CEntity {
    // #[virtual_index(122)]
    // pub fn health(&self) -> i32 {}

    pub fn is_alive(&self) -> bool {
        self.health() > 0
    }

    #[virtual_index(158)]
    pub fn is_player(&self) -> bool {}

    #[virtual_index(10)]
    pub fn abs_origin(&self) -> &'static Vec3 {}

    #[virtual_index(88)]
    pub fn team(&self) -> i32 {}

    #[virtual_index(166)]
    pub fn is_weapon(&self) -> bool {}

    #[virtual_index(285)]
    pub fn eye_pos(&self, ret: &mut Vec3) {}

    #[virtual_index(3)]
    pub fn collidable(&self) -> NotNull<Collidable> {}

    netvar!("DT_BasePlayer", "m_iHealth", health, i32);
    netvar!("DT_CSPlayer", "m_ArmorValue", armor, i32);
    netvar!("DT_CSPlayer", "m_bIsScoped", is_scoped, bool);
    netvar!("DT_CSPlayer", "m_bIsDefusing", is_defusing, bool);
    netvar!("DT_BasePlayer", "m_fFlags", flags, i32);
    netvar!("DT_CSPlayer", "m_flFlashDuration", flash_duration, i32);
    netvar!("DT_BaseEntity", "m_bSpotted", is_spotted, bool);
    netvar!("DT_BaseEntity", "m_vecOrigin", origin, Vec3);
    netvar!("DT_BasePlayer", "m_vecViewOffset", view_offset, Vec3);
    netvar!("DT_BasePlayer", "m_vecVelocity[0]", velocity, Vec3);
    netvar!("DT_BasePlayer", "m_hViewModel[0]", view_model, i32);
    netvar!("DT_BasePlayer", "m_iObserverMode", observer_mode, i32);
    netvar!(
        "DT_BaseCombatWeapon",
        "m_flNextPrimaryAttack",
        weapon_cooldown,
        i32
    );
    netvar!(
        "DT_BaseCombatCharacter",
        "m_hActiveWeapon",
        active_weapon,
        i32
    );
    netvar!("DT_BasePlayer", "m_nTickBase", tickbase, i32);
    netvar!("DT_BaseCombatCharacter", "m_flNextAttack", next_attack, f32);
    netvar!("DT_CSPlayer", "m_iShotsFired", shots_fired, i32);
    netvar!("DT_BasePlayer", "m_aimPunchAngle", aim_punch, Vec2);
    netvar!("DT_BasePlayer", "m_lifeState", life_state, i32);
    netvar!("DT_CSPlayer", "m_bGunGameImmunity", is_immune, bool);
    netvar!("DT_CSPlayer", "m_bHasHelmet", has_helmet, bool);
    netvar!("DT_CSPlayer", "m_iAccount", money, i32);

    pub fn weapons(&self) -> [u32; 48] {
        self.get_value::<[u32; 48]>(netvar::offset("DT_BaseCombatCharacter", "m_hMyWeapons"))
    }

    pub fn weapon(&self) -> Option<Weapon> {
        get_interfaces()
            .entity_list
            .entity_from_handle::<Weapon>(self.active_weapon())
    }

    /// # Safety
    /// this crashes :shrug:
    pub unsafe fn setup(
        &self,
        out: &mut [Matrix4x3; 256],
        max_bones: c_int,
        mask: c_int,
        time: f32,
    ) -> bool {
        let this = self.as_ptr() as *mut c_void;
        if this.is_null() {
            return false;
        }
        let vtable = this.cast::<*const *const ()>().add(1);
        if vtable.is_null() {
            return false;
        }
        let offset = (*vtable).add(13);
        if offset.is_null() {
            return false;
        }
        let f = *(offset).cast::<OriginalFn>();

        let arg = vtable.cast::<c_void>();

        f(arg, out.as_mut_ptr(), max_bones, mask, time)
    }
}

// #[has_vtable]
// #[derive(VTable, Debug)]
// pub struct ClientRenderable{
//
// }
//
// impl ClientRenderable{
//     #[virtual_index(13)]
//     pub fn setup_bones(&self, mat: *const Matrix4x3, ) -> bool {}
// }
