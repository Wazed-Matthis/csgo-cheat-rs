use std::ffi::{c_char, c_void};

use vtables::VTable;
use vtables_derive::*;
use winapi::ctypes::wchar_t;

/**
    Taken from ION lolz I am big skid :feelsbadman:
*/
pub type HFONT = *mut c_void;

pub enum EFontFlags {
    FontflagNone,
    FontflagItalic = 0x001,
    FontflagUnderline = 0x002,
    FontflagStrikeout = 0x004,
    FontflagSymbol = 0x008,
    FontflagAntialias = 0x010,
    FontflagGaussianblur = 0x020,
    FontflagRotary = 0x040,
    FontflagDropshadow = 0x080,
    FontflagAdditive = 0x100,
    FontflagOutline = 0x200,
    FontflagCustom = 0x400,
    FontflagBitmap = 0x800,
}

#[derive(Clone, Copy)]
pub enum GradientType {
    GradientHorizontal = 0,
    GradientVertical,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub a: i32,
}

impl Color {
    pub fn new_rgb(r: i32, g: i32, b: i32) -> Self {
        Self::new_rgba(r, g, b, 255)
    }

    pub fn new_rgba(r: i32, g: i32, b: i32, a: i32) -> Self {
        Self { r, g, b, a }
    }

    pub fn blend(&self, other: Color, t: f32) -> Self {
        Self {
            r: (self.r as f32 + t * (other.r - self.r) as f32) as i32,
            g: (self.g as f32 + t * (other.g - self.g) as f32) as i32,
            b: (self.b as f32 + t * (other.b - self.b) as f32) as i32,
            a: (self.a as f32 + t * (other.a - self.a) as f32) as i32,
        }
    }
}

#[has_vtable]
#[derive(VTable, Debug)]
pub struct Surface {}

impl Surface {
    #[virtual_index(15)]
    pub fn set_draw_color(&self, color: Color) {}

    #[virtual_index(16)]
    pub fn draw_filled_rect(&self, x: i32, y: i32, width: i32, height: i32) {}

    #[virtual_index(18)]
    pub fn draw_outlined_rect(&self, x: i32, y: i32, width: i32, height: i32) {}

    #[virtual_index(19)]
    pub fn draw_line(&self, x: i32, y: i32, width: i32, height: i32) {}

    #[virtual_index(79)]
    pub fn text_size(&self, font: HFONT, text: *const wchar_t, wide: &mut i32, tall: &mut i32) {}

    #[virtual_index(72)]
    #[warn(clippy::too_many_arguments)]
    pub fn font_glyph(
        &self,
        font: HFONT,
        windows_font_name: *const c_char,
        tall: i32,
        weight: i32,
        blur: i32,
        scanlines: i32,
        flags: i32,
        what: i32,
        what_1: i32,
    ) -> bool {
    }

    #[virtual_index(71)]
    pub fn create_font(&self) -> u64 {}

    #[virtual_index(28)]
    pub fn render_text(&self, text: *const wchar_t, text_len: i32, font_draw_type: i32) {}

    #[virtual_index(26)]
    pub fn text_pos(&self, x: i32, y: i32) {}

    #[virtual_index(23)]
    pub fn text_font(&self, font: HFONT) {}

    #[virtual_index(25)]
    pub fn text_color(&self, color: Color) {}
}
