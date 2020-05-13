use crate::color_space::ColorSpace;
/// Colors are stored internally in XYZ CIE 1931 space.
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
    pub(crate) a: f64,
}

impl Color {
    pub fn new_srgb(r: f64, g: f64, b: f64, a: f64) -> Self {
        ColorSpace::SRGB.new_color(r, g, b, a)
    }

    pub fn from_hex_srgb(hex: u32, alpha: f64) -> Self {
        ColorSpace::SRGB.new_color_from_hex(hex, alpha)
    }

    pub fn from_bytes_srgb(r: u8, g: u8, b: u8, alpha: u8) -> Self {
        ColorSpace::SRGB.new_color_from_bytes(r, g, b, alpha)
    }

    pub fn new_linear_srgb(r: f64, g: f64, b: f64, a: f64) -> Self {
        ColorSpace::SRGB_LINEAR.new_color(r, g, b, a)
    }

    pub fn from_hex_linear_srgb(hex: u32, alpha: f64) -> Self {
        ColorSpace::SRGB_LINEAR.new_color_from_hex(hex, alpha)
    }

    pub fn from_bytes_linear_srgb(r: u8, g: u8, b: u8, alpha: u8) -> Self {
        ColorSpace::SRGB_LINEAR.new_color_from_bytes(r, g, b, alpha)
    }

    pub fn to_srgb(&self) -> (f64, f64, f64, f64) {
        ColorSpace::SRGB.to_rgba(*self)
    }

    pub fn to_srgb_linear(&self) -> (f64, f64, f64, f64) {
        ColorSpace::SRGB_LINEAR.to_rgba(*self)
    }
}
