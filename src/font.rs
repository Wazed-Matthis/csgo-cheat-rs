use crate::sdk::surface::EFontFlags::FontflagOutline;
use crate::sdk::surface::HFONT;
use crate::{lpcstr, Color, INTERFACES};
use std::ffi::{CStr, OsStr};
use std::os::windows::ffi::OsStrExt;
use std::str::FromStr;
use winapi::ctypes::wchar_t;

static mut BASE: u64 = 0;

pub fn setup_fonts() {
    let interfaces = INTERFACES.get().unwrap();

    unsafe {
        BASE = interfaces.vgui_surface.create_font();
        interfaces.vgui_surface.font_glyph(
            BASE as HFONT,
            lpcstr!("Tahoma"),
            12,
            500,
            0,
            0,
            0x200,
            0,
            0,
        );
    }
}

fn to_vec(str: &str) -> Vec<u16> {
    return OsStr::new(str)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
}

pub fn text(x: i32, y: i32, text: &str, color: Color) {
    let interfaces = INTERFACES.get().unwrap();
    unsafe {
        interfaces.vgui_surface.text_font(BASE as HFONT);
        interfaces.vgui_surface.text_color(color);
        interfaces.vgui_surface.text_pos(x, y);
        let i = to_vec(text);
        interfaces
            .vgui_surface
            .render_text(i.as_ptr(), i.len() as i32, 0)
    }
}
