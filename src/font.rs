use crate::sdk::surface::EFontFlags::FontflagOutline;
use crate::sdk::surface::HFONT;
use crate::{lpcstr, Color, INTERFACES};
use std::ffi::{CStr, OsStr};
use std::os::windows::ffi::OsStrExt;
use std::str::FromStr;
use winapi::ctypes::wchar_t;

/// # Safety
/// This is safe because setup fonts will always be called before using fonts
static mut OUTLINE: u64 = 0;
static mut SHADOW: u64 = 0;

pub fn setup_fonts() {
    let interfaces = INTERFACES.get().unwrap();

    unsafe {
        OUTLINE = interfaces.vgui_surface.create_font();
        interfaces.vgui_surface.font_glyph(
            OUTLINE as HFONT,
            lpcstr!("Tahoma"),
            12,
            500,
            0,
            0,
            0x200,
            0,
            0,
        );

        SHADOW = interfaces.vgui_surface.create_font();
        interfaces.vgui_surface.font_glyph(
            SHADOW as HFONT,
            lpcstr!("Tahoma"),
            12,
            500,
            0,
            0,
            0x080 | 0x010,
            0,
            0,
        );
    }
}

pub enum FontType {
    Outline,
    Shadow,
}

fn to_vec(str: &str) -> Vec<u16> {
    return OsStr::new(str)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
}

pub fn text(text: &str, x: i32, y: i32, font_type: FontType, color: Color) {
    let interfaces = INTERFACES.get().unwrap();
    unsafe {
        let font_id = match font_type {
            FontType::Outline => OUTLINE,
            FontType::Shadow => SHADOW,
        };
        interfaces.vgui_surface.text_font(font_id as HFONT);
        interfaces.vgui_surface.text_color(color);
        interfaces.vgui_surface.text_pos(x, y);
        let i = to_vec(text);
        interfaces
            .vgui_surface
            .render_text(i.as_ptr(), i.len() as i32, 0)
    }
}

pub fn text_center(text: &str, x: f32, y: f32, font_type: FontType, color: Color) {
    let interfaces = INTERFACES.get().unwrap();
    unsafe {
        let font_id = match font_type {
            FontType::Outline => OUTLINE,
            FontType::Shadow => SHADOW,
        };

        let mut width = 0;
        let mut height = 0;
        let i = to_vec(text);

        let ptr = i.as_ptr();
        interfaces
            .vgui_surface
            .text_size(font_id as HFONT, ptr, &mut width, &mut height);

        interfaces.vgui_surface.text_font(font_id as HFONT);
        interfaces.vgui_surface.text_color(color);
        interfaces
            .vgui_surface
            .text_pos((x - (width as f32) / 2.0) as i32, y as i32);

        interfaces.vgui_surface.render_text(ptr, i.len() as i32, 0)
    }
}

pub fn text_bounds(text: &str, font_type: FontType) -> (i32, i32) {
    let interfaces = INTERFACES.get().unwrap();
    let mut width = 0;
    let mut height = 0;
    unsafe {
        let font_id = match font_type {
            FontType::Outline => OUTLINE,
            FontType::Shadow => SHADOW,
        };
        let i = to_vec(text);
        let ptr = i.as_ptr();
        interfaces
            .vgui_surface
            .text_size(font_id as HFONT, ptr, &mut width, &mut height);
        return (width, height);
    }
}
