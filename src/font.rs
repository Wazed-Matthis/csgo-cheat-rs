use crate::{lpcstr, Color, INTERFACES};
use bitflags::bitflags;
use once_cell::sync::OnceCell;
use std::ffi::{c_void, OsStr};
use std::os::windows::ffi::OsStrExt;

pub type HFONT = *mut c_void;

bitflags! {
    pub struct FontFlags: i32 {
        const NONE = 0x000;
        const ITALIC = 0x001;
        const UNDERLINE = 0x002;
        const STRIKEOUT = 0x004;
        const SYMBOL = 0x008;
        const ANTI_ALIAS = 0x010;
        const GAUSSIAN_BLUR = 0x020;
        const ROTARY = 0x040;
        const DROP_SHADOW = 0x080;
        const ADDITIVE = 0x100;
        const OUTLINE = 0x200;
        const CUSTOM = 0x400;
        const BITMAP = 0x800;
    }
}

/// # Safety
/// This is safe because setup fonts will always be called before using fonts
static OUTLINE: OnceCell<u64> = OnceCell::new();
static SHADOW: OnceCell<u64> = OnceCell::new();

pub fn setup_fonts() {
    let interfaces = INTERFACES.get().unwrap();

    // Set the default outlined font
    let outline = interfaces.vgui_surface.create_font();
    OUTLINE.set(outline).expect("Outline font already set.");
    interfaces.vgui_surface.font_glyph(
        outline as HFONT,
        lpcstr!("Tahoma"),
        12,
        500,
        0,
        0,
        FontFlags::OUTLINE.bits(),
        0,
        0,
    );

    // Set the default shadowed font
    let shadow = interfaces.vgui_surface.create_font();
    SHADOW.set(shadow).expect("Shadow font already set.");
    interfaces.vgui_surface.font_glyph(
        shadow as HFONT,
        lpcstr!("Tahoma"),
        12,
        500,
        0,
        0,
        (FontFlags::ANTI_ALIAS | FontFlags::DROP_SHADOW).bits(),
        0,
        0,
    );
}

/// The different available font types
pub enum FontType {
    Outline,
    Shadow,
}

/// Converts a str to a Utf16 equivilant as bytes
fn convert_to_utf16(str: &str) -> Vec<u16> {
    return OsStr::new(str)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
}

/// Renders text at a given position
pub fn text(text: &str, x: f32, y: f32, font_type: FontType, color: Color) {
    let interfaces = INTERFACES.get().unwrap();
    let font_id = font_by_type(font_type);
    let i = convert_to_utf16(text);

    // Set drawing properties
    interfaces.vgui_surface.text_font(font_id as HFONT);
    interfaces.vgui_surface.text_color(color);
    interfaces.vgui_surface.text_pos(x as i32, y as i32);

    // Draw text
    interfaces
        .vgui_surface
        .render_text(i.as_ptr(), i.len() as i32, 0)
}

/// Renders centered text at a given position
pub fn text_center(text: &str, x: f32, y: f32, font_type: FontType, color: Color) {
    let interfaces = INTERFACES.get().unwrap();
    let font_id = font_by_type(font_type);
    let mut width = 0;
    let mut height = 0;
    let i = convert_to_utf16(text);
    let ptr = i.as_ptr();

    // Set drawing properties
    interfaces
        .vgui_surface
        .text_size(font_id as HFONT, ptr, &mut width, &mut height);
    interfaces.vgui_surface.text_font(font_id as HFONT);
    interfaces.vgui_surface.text_color(color);
    interfaces
        .vgui_surface
        .text_pos((x - (width as f32) / 2.0) as i32, y as i32);

    // Draw text
    interfaces.vgui_surface.render_text(ptr, i.len() as i32, 0)
}

pub fn text_bounds(text: &str, font_type: FontType) -> (i32, i32) {
    let interfaces = INTERFACES.get().unwrap();
    let mut width = 0;
    let mut height = 0;
    let font_id = font_by_type(font_type);
    let i = convert_to_utf16(text);
    let ptr = i.as_ptr();

    // Get bounds of text
    interfaces
        .vgui_surface
        .text_size(font_id as HFONT, ptr, &mut width, &mut height);
    (width, height)
}

fn font_by_type(font_type: FontType) -> u64 {
    *match font_type {
        FontType::Outline => OUTLINE.get().unwrap(),
        FontType::Shadow => SHADOW.get().unwrap(),
    }
}
