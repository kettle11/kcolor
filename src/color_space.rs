use crate::math::*;
use crate::Color;

// A color space expressed in relation to the CIE XYZ color space:
// https://en.wikipedia.org/wiki/CIE_1931_color_space
#[derive(Debug, Copy, Clone)]
pub struct ColorSpace {
    to_xyz: Matrix3x3,
    from_xyz: Matrix3x3,
}

#[derive(Debug, Copy, Clone)]
pub struct Chromacity {
    pub x: f64,
    pub y: f64,
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
    ) -> Self {
        // Reference:
        // http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
        // Why does white point need to be specified in XYZ space?
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
        }
    }

    pub fn to_xyz(&self, color: Color) -> XYZ {
        let v = Vector3::new(color.r, color.g, color.b);
        let xyz = self.to_xyz * v;
        XYZ {
            x: xyz.x,
            y: xyz.y,
            z: xyz.z,
        }
    }

    pub fn from_xyz(&self, xyz: XYZ, alpha: f64) -> Color {
        let v = Vector3::new(xyz.x, xyz.y, xyz.z);
        let c = self.from_xyz * v;
        Color {
            r: c.x,
            g: c.y,
            b: c.z,
            a: alpha,
        }
    }

    pub fn srgb() -> ColorSpace {
        ColorSpace::new(
            Chromacity { x: 0.64, y: 0.33 },
            Chromacity { x: 0.3, y: 0.6 },
            Chromacity { x: 0.15, y: 0.06 },
            XYZ {
                x: 0.95047,
                y: 1.0,
                z: 1.08883,
            },
        )
    }

    pub fn srgb_debug() -> ColorSpace {
        let to_xyz = Matrix3x3::from_columns(
            Vector3::new(0.4124564, 0.2126729, 0.0193339),
            Vector3::new(0.3575761, 0.7151522, 0.1191920),
            Vector3::new(0.1804375, 0.0721750, 0.9503041),
        );
        let from_xyz = to_xyz.inverse();
        ColorSpace { to_xyz, from_xyz }
    }
}
