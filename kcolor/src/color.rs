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

    pub fn black() -> Color {
        Color::new_xyza(0.0, 0.0, 0.0, 1.0)
    }

    pub fn white() -> Color {
        Color::new_xyza(0.950, 1.0, 1.089, 1.0)
    }

    /// Interpolates between two colors in XYZ color space.
    pub fn interpolate(&self, b: &Color, amount: f64) -> Color {
        Color {
            X: (b.X - self.X) * amount + self.X,
            Y: (b.Y - self.Y) * amount + self.Y,
            Z: (b.Z - self.Z) * amount + self.Z,
            a: (b.a - self.a) * amount + self.a,
        }
    }
}
