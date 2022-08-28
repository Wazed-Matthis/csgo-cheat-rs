#![feature(abi_thiscall)]

use hook_rs_lib::{function_hook, register_hooks};
use std::ffi::{c_char, c_float, c_void};
use std::time::Duration;
use std::{mem, ptr};

use crate::sdk::classes::CUserCmd;
use crate::sdk::entity_list::{CEntity, EntityList};
use crate::sdk::get_interface;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::consoleapi::AllocConsole;
use winapi::um::libloaderapi::{FreeLibraryAndExitThread, GetModuleHandleA, GetProcAddress};
use winapi::um::wincon::FreeConsole;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::um::winuser::{GetAsyncKeyState, VK_END};

mod macros;
mod sdk;

/// # Safety
/// This is not safe at all, we just need this to not get clippy warnings
pub unsafe fn entry(module: HINSTANCE) {
    AllocConsole();

    init_hooks();

    loop {
        std::thread::sleep(Duration::from_millis(5));

        unsafe {
            if GetAsyncKeyState(VK_END) != 0 {
                uninit_hooks();
                FreeConsole();
                FreeLibraryAndExitThread(module, 0);
                break;
            }
        }
    }
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
    c_user_cmd: *mut CUserCmd,
) -> bool {
    if c_user_cmd.is_null() {
        return create_move_original(ecx, edx, flt_sampletime, c_user_cmd);
    }
    // let a = &mut *c_user_cmd;

    let entity_list = get_interface::<EntityList>("VClientEntityList003", "client.dll");
    let self_player = dbg!(CEntity::from_raw(dbg!(entity_list.get_entity(0) as usize)));

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
    paint_traverse_original(exc, edx, panel, force_repaint, allow_force)
}

register_hooks!(create_move);
