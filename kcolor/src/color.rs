use crate::color_space::ColorSpace;
/// Colors are stored internally in XYZ CIE 1931 space.
/// Alpha is provided purely for convenience, but is not adjusted by any of
/// the code in this library.
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub(crate) X: f64,
    pub(crate) Y: f64,
    pub(crate) Z: f64,
    pub(crate) a: f64,
}

impl Color {
    pub fn new_srgb(r: f64, g: f64, b: f64, a: f64) -> Self {
        ColorSpace::SRGB.new_color(r, g, b, a)
    }

    /// A new XYZ color in relation to white point D50 2 degrees.
    /// XYZ values must be positive
    pub fn new_xyza(X: f64, Y: f64, Z: f64, a: f64) -> Self {
        Color { X, Y, Z, a }
    }

    pub fn from_hex_srgb(hex: u32, alpha: f64) -> Self {
        ColorSpace::SRGB.new_color_from_hex(hex, alpha)
    }

    pub fn from_bytes_srgb(r: u8, g: u8, b: u8, alpha: u8) -> Self {
        ColorSpace::SRGB.new_color_from_bytes(r, g, b, alpha)
    }

    pub fn from_f32_srgb(r: f64, g: f64, b: f64, alpha: f64) -> Self {
        ColorSpace::SRGB.new_color(r, g, b, alpha)
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

    pub fn from_f32_linear_srgb(r: f64, g: f64, b: f64, alpha: f64) -> Self {
        ColorSpace::SRGB_LINEAR.new_color(r, g, b, alpha)
    }

    pub fn to_srgb(&self) -> (f64, f64, f64, f64) {
        ColorSpace::SRGB.color_to_rgba(self)
    }

    pub fn to_srgb_unclipped(&self) -> (f64, f64, f64, f64) {
        ColorSpace::SRGB.color_to_rgba_unclipped(self)
    }

    pub fn to_linear_srgb(&self) -> (f64, f64, f64, f64) {
        ColorSpace::SRGB_LINEAR.color_to_rgba(self)
    }

    pub fn to_linear_srgb_unclipped(&self) -> (f64, f64, f64, f64) {
        ColorSpace::SRGB_LINEAR.color_to_rgba_unclipped(self)
    }

    pub fn to_color_space(&self, color_space: &ColorSpace) -> (f64, f64, f64, f64) {
        color_space.color_to_rgba(self)
    }

    pub fn to_color_space_unclipped(&self, color_space: &ColorSpace) -> (f64, f64, f64, f64) {
        color_space.color_to_rgba_unclipped(self)
    }

    // A few constant colors
    // Note that these colors are in xyz color space.
    // It'd be nicer to specify them in sRGB, but that requires calling
    // a conversion function and that's not OK for Rust constants.
    pub const RED: Color = Color {
        X: 0.41245643908969215,
        Y: 0.2126728514056225,
        Z: 0.019333895582329296,
        a: 1.0,
    };
    pub const GREEN: Color = Color {
        X: 0.3575760776439089,
        Y: 0.7151521552878178,
        Z: 0.11919202588130295,
        a: 1.0,
    };
    pub const BLUE: Color = Color {
        X: 0.1804374832663989,
        Y: 0.07217499330655956,
        Z: 0.9503040785363678,
        a: 1.0,
    };
    pub const BLACK: Color = Color {
        X: 0.0,
        Y: 0.0,
        Z: 0.0,
        a: 0.0,
    };
    pub const WHITE: Color = Color {
        X: 0.9504699999999999,
        Y: 0.9999999999999999,
        Z: 1.08883,
        a: 1.0,
    };
}
