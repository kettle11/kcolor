use crate::math::*;
use crate::Color;

// An RGB color space expressed in relation to the CIE XYZ color space:
// https://en.wikipedia.org/wiki/CIE_1931_color_space
#[derive(Debug, Copy, Clone)]
pub struct ColorSpace {
    to_xyz: Matrix3x3,
    from_xyz: Matrix3x3,
    transfer_function: TransferFunction,
}

#[derive(Debug, Copy, Clone)]
pub struct Chromacity {
    pub x: f64,
    pub y: f64,
}

/// If the color space stores RGB values nonlinearly this specifies how to make them linear.
/// This should be possible to express numerically.
#[derive(Debug, Copy, Clone)]
pub enum TransferFunction {
    /// Use the sRGB transfer function
    SRGB,
    /// The values are already linearly
    None,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct XYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ColorSpace {
    /// Primaries are specified as xyY.
    /// x and y measure chromaticity.
    /// Y measures luminance.
    /// Luminance: https://en.wikipedia.org/wiki/Luminance
    /// More info:
    /// https://en.wikipedia.org/wiki/CIE_1931_color_space#CIE_xy_chromaticity_diagram_and_the_CIE_xyY_color_space
    /// White point is specified in XYZ space
    pub fn new(
        red_primary: Chromacity,
        green_primary: Chromacity,
        blue_primary: Chromacity,
        white_point: XYZ,
        transfer_function: TransferFunction,
    ) -> Self {
        // Reference:
        // http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
        // Why does white point need to be specified in XYZ space?
        // Shouldn't it be possible to specify the Chromacity of the whitepoint instead?
        // If the white point is already known can extra calculations be avoided?
        let r = Vector3::new(
            red_primary.x / red_primary.y,
            1.0,
            (1.0 - red_primary.x - red_primary.y) / red_primary.y,
        );

        let g = Vector3::new(
            green_primary.x / green_primary.y,
            1.0,
            (1.0 - green_primary.x - green_primary.y) / green_primary.y,
        );

        let b = Vector3::new(
            blue_primary.x / blue_primary.y,
            1.0,
            (1.0 - blue_primary.x - blue_primary.y) / blue_primary.y,
        );

        let inverse = Matrix3x3::from_columns(r, g, b).inverse();
        let s = inverse * Vector3::new(white_point.x, white_point.y, white_point.z);

        let sr = r * s.x;
        let sg = g * s.y;
        let sb = b * s.z;

        let to_xyz = Matrix3x3::from_columns(sr, sg, sb);
        println!("TO XYZ: {:?}", to_xyz);
        println!("FROM XYZ: {:?}", to_xyz.inverse());
        Self {
            to_xyz,
            from_xyz: to_xyz.inverse(),
            transfer_function,
        }
    }

    /// Creates a color with the specified RGB values for the color space
    pub fn new_color(&self, r: f64, g: f64, b: f64, a: f64) -> Color {
        let rgb = Vector3::new(r, g, b);
        let rgb = match self.transfer_function {
            TransferFunction::SRGB => Vector3::new(
                srgb_to_linear(rgb.x),
                srgb_to_linear(rgb.y),
                srgb_to_linear(rgb.z),
            ),
            TransferFunction::None => rgb,
        };
        let xyz = self.to_xyz * rgb;
        Color {
            x: xyz.x,
            y: xyz.y,
            z: xyz.z,
            a,
        }
    }

    /// Creates a new color from the hex values of a number.
    pub fn new_color_from_hex(&self, hex: u32, alpha: f64) -> Color {
        let r = ((hex >> 16) & 0xFF) as f64 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f64 / 255.0;
        let b = ((hex) & 0xFF) as f64 / 255.0;
        self.new_color(r, g, b, alpha)
    }

    /// Creates a new color from the hex values of a number.
    /// Alpha is transparency
    pub fn new_color_from_bytes(&self, r: u8, b: u8, g: u8, alpha: u8) -> Color {
        let r = r as f64 / 255.0;
        let g = g as f64 / 255.0;
        let b = b as f64 / 255.0;
        let a = alpha as f64 / 255.0;
        self.new_color(r, g, b, a)
    }

    /// Gets the RGBA values for the color as expressed in this color space
    pub fn to_rgba(&self, color: Color) -> (f64, f64, f64, f64) {
        let xyz = Vector3::new(color.x, color.y, color.z);
        let rgb = self.from_xyz * xyz;
        let rgb = match self.transfer_function {
            TransferFunction::SRGB => Vector3::new(
                linear_to_srgb(rgb.x),
                linear_to_srgb(rgb.y),
                linear_to_srgb(rgb.z),
            ),
            TransferFunction::None => rgb,
        };
        (rgb.x, rgb.y, rgb.z, color.a)
    }

    /// The popular sRGB color space
    /// https://en.wikipedia.org/wiki/SRGB
    /// Conversion values in table below were calculated with this library.
    /// Chromacity of primaries as expressed in CIE XYZ 1931
    /// Red primary x: 0.64 y: 0.33
    /// Green primary x: 0.3 y: 0.6
    /// Blue primary x: 0.15 y: 0.06
    /// White point (D65) as expressed in CIE XYZ 1931
    /// x: 0.96047
    /// y: 1.0
    /// z: 1.08883
    pub const SRGB: ColorSpace = ColorSpace {
        to_xyz: Matrix3x3 {
            c0: Vector3 {
                x: 0.4124564390896922,
                y: 0.21267285140562253,
                z: 0.0193338955823293,
            },
            c1: Vector3 {
                x: 0.357576077643909,
                y: 0.715152155287818,
                z: 0.11919202588130297,
            },
            c2: Vector3 {
                x: 0.18043748326639894,
                y: 0.07217499330655958,
                z: 0.9503040785363679,
            },
        },
        from_xyz: Matrix3x3 {
            c0: Vector3 {
                x: 3.240454162114104,
                y: -0.9692660305051866,
                z: 0.05564343095911472,
            },
            c1: Vector3 {
                x: -1.5371385127977162,
                y: 1.8760108454466937,
                z: -0.20402591351675378,
            },
            c2: Vector3 {
                x: -0.4985314095560159,
                y: 0.04155601753034983,
                z: 1.057225188223179,
            },
        },
        transfer_function: TransferFunction::SRGB,
    };

    /// Exact same as the above SRGB space, except with a linear transfer function.
    pub const SRGB_LINEAR: ColorSpace = ColorSpace {
        to_xyz: Matrix3x3 {
            c0: Vector3 {
                x: 0.4124564390896922,
                y: 0.21267285140562253,
                z: 0.0193338955823293,
            },
            c1: Vector3 {
                x: 0.357576077643909,
                y: 0.715152155287818,
                z: 0.11919202588130297,
            },
            c2: Vector3 {
                x: 0.18043748326639894,
                y: 0.07217499330655958,
                z: 0.9503040785363679,
            },
        },
        from_xyz: Matrix3x3 {
            c0: Vector3 {
                x: 3.240454162114104,
                y: -0.9692660305051866,
                z: 0.05564343095911472,
            },
            c1: Vector3 {
                x: -1.5371385127977162,
                y: 1.8760108454466937,
                z: -0.20402591351675378,
            },
            c2: Vector3 {
                x: -0.4985314095560159,
                y: 0.04155601753034983,
                z: 1.057225188223179,
            },
        },
        transfer_function: TransferFunction::None,
    };
}

fn srgb_to_linear(v: f64) -> f64 {
    if v <= 0.0031308 {
        v * 12.92
    } else {
        (1.055 * f64::powf(v, 1.0 / 2.4)) - 0.055
    }
}

fn linear_to_srgb(v: f64) -> f64 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        f64::powf((v + 0.055) / 1.055, 2.4)
    }
}
