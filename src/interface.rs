use std::ffi::{c_char, c_int, c_void, CStr};
use std::mem::{size_of, transmute, MaybeUninit};
use std::ptr::null_mut;

use log::debug;
use vtables::VTable;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};

use crate::sdk::glow_object::GlowObjectManager;
use crate::sdk::panel::Panel;
use crate::sdk::surface::Surface;
use crate::{lpcstr, memory, Client, EngineClient, EntityList, GlobalVars};

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
    //pub glow_object_manager: &'static mut GlowObjectManager,
    //pub input_system: input_system::IInputSystem,
    //pub input: input::IInput,
    pub global_vars: &'static GlobalVars,
    //pub render_view: render_view::IRenderView,
    //pub cvar: convar::ICVar,
    //pub engine_trace: engine_trace::IEngineTrace,
    //pub engine_sound: engine_sound::IEngineSound,
    //pub material_system: material_system::IMaterialSystem,
    //pub model_render: model_render::IModelRender,
    //pub model_info: model_info::IModelInfo,
    //pub localize: localize::ILocalize,
    //pub physics_surface_props: physics_surface_props::IPhysicsSurfaceProps,
    //pub prediction: prediction::IPrediction,
    //pub view_render_beams: view_render_beams::IViewRenderBeams,
    //pub game_event: game_events::IGameEventManager,
    //pub key_values_system: key_values_system::IKeyValuesSystem,
}

unsafe impl Send for Interfaces {}
unsafe impl Sync for Interfaces {}

impl Interfaces {
    pub(crate) fn load() -> Self {
        unsafe {
            let client_interface = get_interface::<Client>(lpcstr!("client.dll"), CLIENT);

            Self {
                client_mode: **(((*((*(client_interface.as_ptr() as *mut *mut usize)).offset(10)))
                    + 5) as *mut *mut _),
                global_vars: &*(**(((*((*(client_interface.as_ptr() as *mut *mut usize))
                    .offset(11)))
                    + 10) as *mut *mut usize) as *const GlobalVars),
                client: client_interface,
                engine: get_interface(lpcstr!("engine.dll"), ENGINE),
                // glow_object_manager: core::mem::transmute::<_, &mut glow::IGlowObjectManager>(
                //     memory::read_mut::<usize>(
                //         some_or_ret!(
                //             pattern_scan(modules::CLIENT, patterns::GLOW_MANAGER),
                //             Err(NotFound {
                //                 item: obfstr!("IGlowMgr").into()
                //             })
                //         ) as usize
                //             + 0x3,
                //     ),
                // ),
                vgui_panel: get_interface(lpcstr!("vgui2.dll"), VGUI_PANEL),
                entity_list: get_interface(lpcstr!("client.dll"), ENTITY_LIST),
                vgui_surface: get_interface(lpcstr!("vguimatsurface.dll"), VGUI_SURFACE),
                // input_system: get_interface(modules::INPUT_SYSTEM, INPUT_SYSTEM)?,
                // input: input::IInput::from_raw_unchecked(memory::read::<*mut usize>(
                //     some_or_ret!(
                //         pattern_scan(modules::CLIENT, patterns::INPUT_INTERFACE),
                //         Err(NotFound {
                //             item: obfstr!("IInput").into()
                //         })
                //     ) as usize
                //         + 0x1,
                // )),
                // render_view: get_interface(modules::ENGINE, RENDER_VIEW)?,
                // cvar: get_interface(modules::VSTD_LIB, CVAR)?,
                // engine_trace: get_interface(modules::ENGINE, ENGINE_TRACE)?,
                // engine_sound: get_interface(modules::ENGINE, ENGINE_SOUND)?,
                // material_system: get_interface(modules::MATERIAL_SYSTEM, MAT_SYSTEM)?,
                // model_render: get_interface(modules::ENGINE, MODEL_RENDER)?,
                // model_info: get_interface(modules::ENGINE, MODEL_INFO)?,
                // localize: get_interface(modules::LOCALIZE, LOCALIZE)?,
                // physics_surface_props: get_interface(modules::PHYSICS, PHYS_SURFACE_PROPS)?,
                // prediction: get_interface(modules::CLIENT, PREDICTION)?,
                // view_render_beams: view_render_beams::IViewRenderBeams::from_raw_unchecked(
                //     *((some_or_ret!(
                //         pattern_scan(modules::CLIENT, patterns::VIEW_RENDER_BEAMS),
                //         Err(NotFound {
                //             item: obfstr!("IViewRenderBeams").into()
                //         })
                //     ) as usize
                //         + 0x1) as *mut usize) as *mut usize,
                // ),
                // game_event: get_interface(modules::ENGINE, GAME_EVENT_MGR)?,
                // key_values_system: key_values_system::IKeyValuesSystem::from_raw_unchecked(
                //     some_or_ret!(
                //         get_proc_address(
                //             get_module_handle(modules::VSTD_LIB),
                //             cstr!("KeyValuesSystem")
                //         ),
                //         Err(NotFound {
                //             item: obfstr!("KeyValuesSystem").into()
                //         })
                //     ) as *const usize,
                // ),
            }
        }
    }
}

impl Default for Interfaces {
    #[allow(invalid_value)]
    fn default() -> Self {
        unsafe { MaybeUninit::uninit().assume_init() }
    }
}

pub(crate) unsafe fn get_interface<T: VTable>(module: *const i8, interface: &str) -> T {
    let create_interface = GetProcAddress(GetModuleHandleA(module), lpcstr!("CreateInterface"));

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

#[repr(C)]
struct InterfaceLinkedList {
    func: fn() -> *const c_void,
    name: *const c_char,
    next: *mut InterfaceLinkedList,
}
