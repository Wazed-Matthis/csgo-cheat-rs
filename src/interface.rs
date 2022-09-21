use std::ffi::{c_char, c_int, c_void, CStr};
use std::mem::{size_of, transmute, MaybeUninit};
use std::ptr::null_mut;

use log::debug;
use vtables::VTable;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};

use crate::sdk::panel::Panel;
use crate::sdk::surface::Surface;
use crate::{lpcstr, Client, EngineClient, EntityList, GlobalVars};

const CLIENT: &str = "VClient018";
const ENTITY_LIST: &str = "VClientEntityList003";
const ENGINE: &str = "VEngineClient014";
const VGUI_PANEL: &str = "VGUI_Panel009";
const VGUI_SURFACE: &str = "VGUI_Surface031";
const INPUT_SYSTEM: &str = "InputSystemVersion001";
const RENDER_VIEW: &str = "VEngineRenderView014";
const CVAR: &str = "VEngineCvar007";
const ENGINE_TRACE: &str = "EngineTraceClient004";
const ENGINE_SOUND: &str = "IEngineSoundClient003";
const MAT_SYSTEM: &str = "VMaterialSystem080";
const MODEL_RENDER: &str = "VEngineModel016";
const MODEL_INFO: &str = "VModelInfoClient004";
const LOCALIZE: &str = "Localize_001";
const PHYS_SURFACE_PROPS: &str = "VPhysicsSurfaceProps001";
const PREDICTION: &str = "VClientPrediction001";
const GAME_EVENT_MGR: &str = "GAMEEVENTSMANAGER002";

#[derive(Debug)]
pub struct Interfaces {
    pub client: Client,
    pub client_mode: *mut usize,
    pub vgui_surface: Surface,
    pub vgui_panel: Panel,
    pub entity_list: EntityList,
    pub engine: EngineClient,
    pub global_vars: &'static GlobalVars,
}

unsafe impl Send for Interfaces {}
unsafe impl Sync for Interfaces {}

impl Interfaces {
    pub fn init() -> Self {
        unsafe {
            let client_interface = get_interface::<Client>("client.dll", CLIENT);

            Self {
                client_mode: **(((*((*(client_interface.as_ptr() as *mut *mut usize)).offset(10)))
                    + 5) as *mut *mut _),
                global_vars: &*(**(((*((*(client_interface.as_ptr() as *mut *mut usize))
                    .offset(11)))
                    + 10) as *mut *mut usize) as *const GlobalVars),
                client: client_interface,
                engine: get_interface("engine.dll", ENGINE),
                vgui_panel: get_interface("vgui2.dll", VGUI_PANEL),
                entity_list: get_interface("client.dll", ENTITY_LIST),
                vgui_surface: get_interface("vguimatsurface.dll", VGUI_SURFACE),
            }
        }
    }
}

///
/// # Safety
/// This function is safe if the given interface and the module are valid.
/// Otherwise the function will crash and throw an access violation because of accessing a null pointer
pub unsafe fn get_interface<T: VTable>(module: &str, interface: &str) -> T {
    let create_interface = GetProcAddress(
        GetModuleHandleA(lpcstr!(module)),
        lpcstr!("CreateInterface"),
    );

    let create_interface = transmute::<
        _,
        fn(name: *const c_char, return_code: *const c_int) -> *const c_void,
    >(create_interface);

    debug!("Capturing interface {}...", interface);
    let addr = create_interface(lpcstr!(interface), null_mut()) as usize;
    if addr != 0 {
        debug!("Captured interface {}, addr.: {:x}", interface, addr);
        T::new(addr as _)
    } else {
        panic!("Failed to capture interface {}", interface);
    }
}
