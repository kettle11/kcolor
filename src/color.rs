/// Colors are stored in an unspecified color space.
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    /// Creates a new color.
    /// Note that the color space is unspecified.
    /// Alpha is transparency
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Color {
        Color { r, g, b, a }
    }

    /// Creates a new color from the hex values of a number.
    /// Note that the color space is unspecified.
    pub fn from_hex(hex: u32, alpha: f64) -> Color {
        let r = ((hex >> 16) & 0xFF) as f64 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f64 / 255.0;
        let b = ((hex) & 0xFF) as f64 / 255.0;

        Self::new(r, g, b, alpha)
    }

    /// Creates a new color from the hex values of a number.
    /// Note that the color space is unspecified.
    /// Alpha is transparency
    pub fn from_byte(r: u8, b: u8, g: u8, alpha: u8) -> Color {
        let r = r as f64 / 255.0;
        let g = g as f64 / 255.0;
        let b = b as f64 / 255.0;
        let a = alpha as f64 / 255.0;

        Self::new(r, g, b, a)
    }

    pub const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const TRANSPARENT: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    pub const YELLOW: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const MAGENTA: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const CYAN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
}

/*
fn srgb_to_linear(v: f64) -> f64 {
    // See transformation function here:
    // https://en.wikipedia.org/wiki/SRGB
    if v <= 0.04045 {
        v / 12.92
    } else {
        f64::powf((v + 0.055) / 1.055, 2.4)
    }
}
*/
