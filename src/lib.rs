use hook_rs_lib::{function_hook, register_hooks};
use std::ffi::{c_char, c_float, c_void};
use std::time::Duration;
use std::{mem, ptr};

use crate::sdk::classes::CUserCmd;
use hook_rs_lib::hooks::vtable::VMT;
use rand::Rng;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::consoleapi::AllocConsole;
use winapi::um::libloaderapi::{FreeLibraryAndExitThread, GetModuleHandleA, GetProcAddress};
use winapi::um::wincon::FreeConsole;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::um::winuser::{GetAsyncKeyState, HARDWAREHOOKSTRUCT, VK_END};

mod macros;
mod sdk;

const DEG_TO_RAD: f32 = 0.017453292519943295;
const RAD_TO_DEG: f32 = 57.29577951308232;

pub fn thing(module: HINSTANCE) {
    unsafe {
        AllocConsole();
    }

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
static mut test: u64 = 0;

#[no_mangle]
pub extern "system" fn DllMain(module: HINSTANCE, fdw_reason: DWORD, _: LPVOID) -> BOOL {
    if fdw_reason == DLL_PROCESS_ATTACH {
        let hmodule = module as usize;
        std::thread::spawn(move || thing(hmodule as HINSTANCE));
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
    let mut rng = rand::rngs::OsRng::default();

    unsafe {
        if c_user_cmd.is_null() {
            return create_move_original(ecx, edx, flt_sampletime, c_user_cmd);
        }
        let a = &mut *c_user_cmd;

        let old_yaw = a.view_angles.y;
        let new_yaw = rng.gen::<f32>() * 360.0 - 180.0;
        let delta_yaw = new_yaw - old_yaw;

        a.view_angles.y = new_yaw;

        let prev_forward = a.forward_move;
        let prev_sidemove = a.side_move;
        a.forward_move = (delta_yaw * DEG_TO_RAD).cos() * prev_forward
            + ((delta_yaw + 90.0) * DEG_TO_RAD).cos() * prev_sidemove;
        a.side_move = (delta_yaw * DEG_TO_RAD).sin() * prev_forward
            + ((delta_yaw + 90.0) * DEG_TO_RAD).sin() * prev_sidemove;
    }

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
