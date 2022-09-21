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
use rand::rngs::OsRng;
use rand::Rng;
use vtables::VTable;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::consoleapi::AllocConsole;
use winapi::um::libloaderapi::{FreeLibraryAndExitThread, GetModuleHandleA, GetProcAddress};
use winapi::um::wincon::FreeConsole;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::um::winuser::{GetAsyncKeyState, VK_END};

use crate::interface::Interfaces;
use crate::sdk::classes::{CUserCMD, EButtons, Vec3};
use crate::sdk::client::Client;
use crate::sdk::engine::EngineClient;
use crate::sdk::entity_list::EntityList;
use crate::sdk::global_vars::GlobalVars;
use crate::sdk::surface::Color;

pub mod interface;
pub mod macros;
pub mod memory;
pub mod sdk;
mod source_api;

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
    INTERFACES.set(Interfaces::init()).unwrap();
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

    unsafe {
        let a = &mut *c_user_cmd;
        let mut rng = OsRng::default();
        let old_yaw = a.view_angles.y;
        let new_yaw = rng.gen::<f32>() * 360.0 - 180.0;
        let delta_yaw = (new_yaw - old_yaw).to_radians();

        match a.buttons {
            EButtons::InAttack => {}
            _ => {
                a.view_angles.y = new_yaw;
                a.view_angles.x = 89.0;
            }
        }

        let forward = a.forward_move;
        let strafe = a.side_move;
        a.forward_move = delta_yaw.cos() * forward - delta_yaw.sin() * strafe;
        a.side_move = delta_yaw.sin() * forward + delta_yaw.cos() * strafe;
    }
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
    paint_traverse_original(exc, edx, panel, force_repaint, allow_force);

    let interfaces = INTERFACES.get().unwrap();
    let panel_name = interfaces.vgui_panel.get_panel_name(panel);

    let c_str = unsafe { CStr::from_ptr(panel_name) };
    let string = c_str.to_str().unwrap();
    if string == "MatSystemTopPanel" {
        interfaces
            .vgui_surface
            .set_draw_color(Color::new_rgba(0, 255, 255, 255));
        interfaces.vgui_surface.draw_filled_rect(100, 100, 200, 200);

        for i in 0..interfaces.global_vars.max_clients {
            let entity = interfaces.entity_list.entity(i);
            if let Some(ent) = entity.get() {
                let collidable = ent.collidable();
                if let Some(col) = collidable.get() {
                    let origin = ent.abs_origin();
                    let obb_mins = col.min().clone();
                    let obb_maxs = col.max().clone();
                    #[rustfmt::skip]
                    let points = vec![
                        Vec3 {x: obb_mins.x, y: obb_mins.y, z: obb_mins.z},
                        Vec3 {x: obb_mins.x, y: obb_maxs.y, z: obb_mins.z},
                        Vec3 {x: obb_maxs.x, y: obb_maxs.y, z: obb_mins.z},
                        Vec3 {x: obb_maxs.x, y: obb_mins.y, z: obb_mins.z},
                        Vec3 {x: obb_maxs.x, y: obb_maxs.y, z: obb_maxs.z},
                        Vec3 {x: obb_mins.x, y: obb_maxs.y, z: obb_maxs.z},
                        Vec3 {x: obb_mins.x, y: obb_mins.y, z: obb_maxs.z},
                        Vec3 {x: obb_maxs.x, y: obb_mins.y, z: obb_maxs.z}
                    ];

                    let projected_points: Vec<Vec3> = points
                        .iter()
                        .map(|point| {
                            let mut contextualized = point.clone() + origin.clone();
                            let mut projected = contextualized.clone();
                            interfaces
                                .debug_overlay
                                .world_to_screen(&mut contextualized, &mut projected);
                            projected
                        })
                        .collect();

                    let mut left = projected_points[0].x;
                    let mut bottom = projected_points[0].y;
                    let mut right = projected_points[0].x;
                    let mut top = projected_points[0].y;

                    for point in projected_points {
                        left = left.min(point.x);
                        bottom = bottom.max(point.y);
                        right = right.max(point.x);
                        top = top.min(point.y);
                    }

                    let x = left as i32;
                    let y = top as i32;
                    let w = (right - left) as i32;
                    let h = (bottom - top) as i32;

                    interfaces
                        .vgui_surface
                        .set_draw_color(Color::new_rgba(255, 255, 255, 255));
                    interfaces
                        .vgui_surface
                        .draw_outlined_rect(x, y, right as i32, bottom as i32);
                }
            }
        }
    }
}

register_hooks!(create_move, frame_stage_notify, paint_traverse);
