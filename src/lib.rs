#![feature(abi_thiscall)]

extern crate core;

use std::ffi::{c_char, c_float, c_void, CStr};
use std::ptr::addr_of_mut;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::{mem, ptr};

use hook_rs_lib::{function_hook, register_hooks};
use log::debug;
use once_cell::sync::OnceCell;
use vtables::VTable;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::consoleapi::AllocConsole;
use winapi::um::libloaderapi::{FreeLibraryAndExitThread, GetModuleHandleA, GetProcAddress};
use winapi::um::wincon::FreeConsole;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::um::winuser::{GetAsyncKeyState, VK_END};

use crate::interface::Interfaces;
use crate::sdk::classes::CUserCMD;
use crate::sdk::client::Client;
use crate::sdk::engine::EngineClient;
use crate::sdk::entity_list::EntityList;
use crate::sdk::global_vars::GlobalVars;
use crate::sdk::surface::Color;

pub mod interface;
pub mod macros;
pub mod memory;
pub mod sdk;

static INTERFACES: OnceCell<Interfaces> = OnceCell::new();

/// # Safety
/// This is not safe at all, we just need this to not get clippy warnings
pub unsafe fn entry(module: HINSTANCE) {
    AllocConsole();
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    initialize();

    init_hooks();
    loop {
        std::thread::sleep(Duration::from_millis(5));
        if GetAsyncKeyState(VK_END) != 0 {
            uninit_hooks();
            FreeConsole();
            FreeLibraryAndExitThread(module, 0);
            break;
        }
    }
}

pub fn initialize() {
    INTERFACES.set(Interfaces::load()).unwrap();
    sdk::netvars::manager::scan();
}

pub fn get_interfaces<'a>() -> &'a Interfaces {
    INTERFACES.get().unwrap()
}

#[no_mangle]
pub extern "system" fn DllMain(module: HINSTANCE, fdw_reason: DWORD, _: LPVOID) -> BOOL {
    if fdw_reason == DLL_PROCESS_ATTACH {
        let hmodule = module as usize;
        std::thread::spawn(move || unsafe { entry(hmodule as HINSTANCE) });
    }
    1
}

type PtrPtr<T> = *mut *mut T;
type PtrPtrPtr<T> = *mut *mut *mut T;

#[function_hook(
    interface = "VClient018",
    module = "client.dll",
    index = 24,
    init = r#"**(((*((*(interface as PtrPtr<usize>)).add(10))) + 5) as PtrPtrPtr<usize>)"#
)]
pub extern "fastcall" fn create_move(
    ecx: *const c_void,
    edx: *const c_void,
    flt_sampletime: c_float,
    c_user_cmd: *mut CUserCMD,
) -> bool {
    // let a = &mut *c_user_cmd;
    let interfaces = INTERFACES.get().unwrap();

    if c_user_cmd.is_null() || !interfaces.engine.is_in_game() {
        return create_move_original(ecx, edx, flt_sampletime, c_user_cmd);
    }

    let local_player = interfaces.engine.local_player();
    println!("{}", local_player);
    if let Some(local_entity) = interfaces.entity_list.entity(local_player).get() {
        debug!("Is alive: {}", local_entity.is_alive());
    }
    // for i in 0..entity_list.get_highest_entity_index() {
    //     let e = entity_list.get_entity(i);
    //     dbg!(e);
    // }

    // 28AD0560
    //dbg!(CEntity::from_raw(dbg!(entity_list.get_entity(1) as usize)));

    // println!("{:?}", self_player.is_player());

    // let old_yaw = a.view_angles.y;
    // let new_yaw = rng.gen::<f32>() * 360.0 - 180.0;
    // let delta_yaw = (new_yaw - old_yaw).to_radians();
    //
    // a.view_angles.y = new_yaw;
    //
    // let forward = a.forward_move;
    // let strafe = a.side_move;
    // a.forward_move = delta_yaw.cos() * forward - delta_yaw.sin() * strafe;
    // a.side_move = delta_yaw.sin() * forward + delta_yaw.cos() * strafe;
    false
}

#[function_hook(interface = "VClient018", module = "client.dll", index = 37)]
pub extern "fastcall" fn frame_stage_notify(ecx: *const c_void, edx: *const c_void, stage: i32) {
    //     if let Some(entity_list) = get_interface::<EntityList>("VClientEntityList003") {
    //         for i in 0..entity_list.highest_entity_index() {
    //             let entity = entity_list.entity(i);
    //             if !entity.is_null() {
    //                 unsafe {
    //                     if (*entity).is_player() {
    //                         println!("{} is a player", i);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    frame_stage_notify_original(ecx, edx, stage)
}

#[function_hook(interface = "VGUI_Panel009", module = "vgui2.dll", index = 41)]
pub extern "fastcall" fn paint_traverse(
    exc: *const c_void,
    edx: *const c_void,
    panel: u32,
    force_repaint: bool,
    allow_force: bool,
) {
    let interfaces = INTERFACES.get().unwrap();
    let panel_name = interfaces.vgui_panel.get_panel_name(panel);

    let c_str = unsafe { CStr::from_ptr(panel_name) };
    let string = c_str.to_str().unwrap();
    if string.contains("MatSystemTopPanel") {
        // interfaces
        //     .vgui_surface
        //     .draw_outlined_rect(100, 100, 200, 200);
    }

    paint_traverse_original(exc, edx, panel, force_repaint, allow_force);
}

register_hooks!(create_move, frame_stage_notify, paint_traverse);