#![feature(abi_thiscall)]

extern crate core;

use std::ffi::{c_char, c_float, c_void, CStr};
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::{mem, ptr};

use event_bus::{dispatch_event, EventBus};
use hook_rs_lib::{function_hook, register_hooks};
use log::error;
use once_cell::sync::OnceCell;
use rand::rngs::OsRng;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::consoleapi::AllocConsole;
use winapi::um::libloaderapi::{FreeLibraryAndExitThread, GetModuleHandleA, GetProcAddress};
use winapi::um::wincon::FreeConsole;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::um::winuser::{GetAsyncKeyState, VK_END};

use crate::config::Configuration;
use crate::events::{EventCreateMove, EventPaintTraverse};
use crate::features::aimbot::Aimbot;
use crate::features::esp::ESP;
use crate::features::watermark::Watermark;
use crate::features::Feature;
use crate::interface::Interfaces;
use crate::sdk::classes::{CUserCMD, Vec3};
use crate::sdk::client::Client;
use crate::sdk::engine::EngineClient;
use crate::sdk::entity_list::EntityList;
use crate::sdk::global_vars::GlobalVars;
use crate::sdk::surface::Color;

pub mod config;
pub mod events;
pub mod features;
pub mod font;
pub mod interface;
pub mod macros;
pub mod memory;
pub mod netvar;
pub mod sdk;
pub mod source_api;

static INTERFACES: OnceCell<Interfaces> = OnceCell::new();
static MAIN_BUS: OnceCell<EventBus> = OnceCell::new();

static CONFIG: OnceCell<Configuration> = OnceCell::new();

/// # Safety
/// This is not safe at all, we just need this to not get clippy warnings
pub unsafe fn entry(module: HINSTANCE) {
    AllocConsole();
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    initialize();
    init_hooks();
    font::setup_fonts();
    netvar::scan_netvars();
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
register_features!(
    AimbotSettings => Aimbot {
        min_damage: f32,
        fov: f32
    },
    ESPSettings => ESP {},
    WatermarkSettings => Watermark {}
);

pub fn initialize() {
    INTERFACES.set(Interfaces::init()).unwrap();
    let mut config_string = String::new();
    File::open("C:/Users/matth/CLionProjects/csgo-cheat-rs/config.json")
        .unwrap()
        .read_to_string(&mut config_string)
        .unwrap();
    let config =
        serde_json::from_str::<Configuration>(&config_string).expect("Failed to parse config_file");
    CONFIG.set(config).unwrap();
    let is_err = MAIN_BUS.set(EventBus::new("main")).is_err();
    if is_err {
        error!("Failed to initialize main event bus");
    }

    init_features();
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
    user_cmd: *mut CUserCMD,
) -> bool {
    let interfaces = INTERFACES.get().unwrap();
    if user_cmd.is_null() || !interfaces.engine.is_in_game() {
        return create_move_original(ecx, edx, flt_sampletime, user_cmd);
    }

    let a = unsafe { &mut *user_cmd };
    let old_view_angle = a.view_angles.clone();

    dispatch_event("main", &mut EventCreateMove { user_cmd });

    let delta_yaw = (a.view_angles.y - old_view_angle.y).to_radians();
    let forward = a.forward_move;
    let strafe = a.side_move;
    a.forward_move = delta_yaw.cos() * forward - delta_yaw.sin() * strafe;
    a.side_move = delta_yaw.sin() * forward + delta_yaw.cos() * strafe;

    false
}

#[function_hook(interface = "VClient018", module = "client.dll", index = 37)]
pub extern "fastcall" fn frame_stage_notify(ecx: *const c_void, edx: *const c_void, stage: i32) {
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
        dispatch_event("main", &mut EventPaintTraverse {});
    }
}

register_hooks!(create_move, frame_stage_notify, paint_traverse);
