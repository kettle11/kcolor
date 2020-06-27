use crate::white_points::*;
use crate::Color;
use kcolor_types::*;

// An RGB color space expressed in relation to the CIE XYZ color space:
// https://en.wikipedia.org/wiki/CIE_1931_color_space
#[derive(Debug, Clone, PartialEq)]
pub struct ColorSpace {
    pub(crate) to_XYZ: Matrix3x3,
    pub(crate) from_XYZ: Matrix3x3,
    pub(crate) transfer_function: TransferFunction,
}

impl ColorSpace {
    /// 'Primaries' are the color that represents the reddest red, the greenest green, and the bluest blue in the color space.
    /// The 'White point' represents the 'whitest white' and also the brightest color.
    /// The white point is the color when all other primaries are set to their max value.
    ///
    /// The primaries and the white point are specified with 'chromaticity' values.
    /// A chromaticity is an x, y coordinate that specifies the hue of a color, irrespective of brightness.
    ///
    /// A 'transfer function' modifies the way colors are stored numerically to allow storing more colors within
    /// the ranges of colors that humans are most sensitive to color changes.
    /// Color spaces have different transfer functions.
    /// More info:
    /// https://en.wikipedia.org/wiki/CIE_1931_color_space#CIE_xy_chromaticity_diagram_and_the_CIE_xyY_color_space
    pub fn new(
        red_primary: Chromaticity,
        green_primary: Chromaticity,
        blue_primary: Chromaticity,
        white_point: Chromaticity,
        transfer_function: TransferFunction,
    ) -> Self {
        // Reference:
        // http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html

        // First convert the chromaticities into XYZ values.
        let r = red_primary.to_XYZ().to_vector3();
        let g = green_primary.to_XYZ().to_vector3();
        let b = blue_primary.to_XYZ().to_vector3();

        let inverse = Matrix3x3::from_columns(r, g, b).inverse();
        let s = inverse * white_point.to_XYZ().to_vector3();

        // The three primaries in XYZ space relative to the white point passed in.
        let sr = r * s.x;
        let sg = g * s.y;
        let sb = b * s.z;

        // The 2 degrees D50 white point is used to store colors internally
        // If the color space being declared is not relative to the D50 white point then the primaries must
        // be converted to be relative to D50.
        // 2 degrees D50 is used because ICC profiles are always specified with the 2 degrees D50 white point.
        let (to_XYZ, from_XYZ) = if white_point != D50_WHITE_POINT_2DEGREES {
            let white_point_adaptation =
                ChromaticAdaptation::new(white_point, D50_WHITE_POINT_2DEGREES);
            let white_point_adaptation_inverse =
                ChromaticAdaptation::new(D50_WHITE_POINT_2DEGREES, white_point);
            (
                white_point_adaptation.inner_matrix * Matrix3x3::from_columns(sr, sg, sb),
                Matrix3x3::from_columns(sr, sg, sb).inverse()
                    * white_point_adaptation_inverse.inner_matrix,
            )
        } else {
            let to_XYZ = Matrix3x3::from_columns(sr, sg, sb);
            (to_XYZ, to_XYZ.inverse())
        };

        Self {
            to_XYZ,
            from_XYZ,
            transfer_function,
        }
    }

    /// Creates a color with the specified RGB values for the color space
    pub fn new_color(&self, r: f64, g: f64, b: f64, a: f64) -> Color {
        let rgb = Vector3::new(r, g, b);
        let rgb = Vector3::new(
            self.transfer_function.to_linear(rgb.x),
            self.transfer_function.to_linear(rgb.y),
            self.transfer_function.to_linear(rgb.z),
        );
        let XYZ = self.to_XYZ * rgb;
        Color {
            X: XYZ.x,
            Y: XYZ.y,
            Z: XYZ.z,
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
    /// RGB values outside of 0.0 to 1.0 will be clipped.
    pub fn color_to_rgba(&self, color: &Color) -> (f64, f64, f64, f64) {
        let (r, g, b, a) = self.color_to_rgba_unclipped(color);
        (
            r.max(0.0).min(1.0),
            g.max(0.0).min(1.0),
            b.max(0.0).min(1.0),
            a,
        )
    }

    /// Gets the RGBA values for the color as expressed in this color space
    /// RGB values are allowed to go outside the 0.0 to 1.0 range.
    /// The transfer function (if not None) is mirrored for values less than 0.0
    pub fn color_to_rgba_unclipped(&self, color: &Color) -> (f64, f64, f64, f64) {
        let XYZ = Vector3::new(color.X, color.Y, color.Z);
        let rgb = self.from_XYZ * XYZ;
        let rgb = Vector3::new(
            self.transfer_function.from_linear(rgb.x),
            self.transfer_function.from_linear(rgb.y),
            self.transfer_function.from_linear(rgb.z),
        );
        (rgb.x, rgb.y, rgb.z, color.a)
    }
}

/// If frequent color space conversions are to be performed, use this.
pub struct ColorSpaceConverter {
    conversion_matrix: Matrix3x3,
}

impl ColorSpaceConverter {
    pub fn new(from: &ColorSpace, to: &ColorSpace) -> Self {
        Self {
            conversion_matrix: to.from_XYZ * from.to_XYZ,
        }
    }

    pub fn convert_color(&self, color: &(f64, f64, f64)) -> (f64, f64, f64) {
        let color = Vector3::new(color.0, color.1, color.2);
        let color = self.conversion_matrix * color;
        (color.x, color.y, color.z)
    }
}

/// Convert between XYZ color spaces with different white points.
/// Wavelengths are perceived as one color in one lighting condition and a
/// different color under a different lighting condition.
/// Our eyes adjust to lighting and if a room has yellow-ish lighting
/// (it has a yellow-ish whitepoint) then what appears white is produced
/// with yellow-ish wavelenghts.
///
/// This function first converts to an intermediate space (LMS) that represents our eyes'
/// cone responses using a Bradford transform.
///
/// Then a conversion is performed from the LMS intermediate space back into XYZ.
#[derive(Debug, Clone, PartialEq)]
pub struct ChromaticAdaptation {
    pub(crate) inner_matrix: Matrix3x3,
}

impl ChromaticAdaptation {
    pub fn new(source_white_point: Chromaticity, destination_white_point: Chromaticity) -> Self {
        // Implemented using the techniques described here:
        // http://www.brucelindbloom.com/index.html?Eqn_ChromAdapt.html

        // To do math with the XYZ values convert them to Vector3s.
        let source_white_point = source_white_point.to_XYZ().to_vector3();
        let destination_white_point = destination_white_point.to_XYZ().to_vector3();

        // The Bradford matrix constants are found at the above link.
        // The matrix is also available here: https://en.wikipedia.org/wiki/LMS_color_space
        // These matrices convert XYZ values to LMS (Long Medium Short) values measuring the response of cones.
        let bradford_matrix = Matrix3x3 {
            c0: Vector3 {
                x: 0.8951000,
                y: -0.7502000,
                z: 0.0389000,
            },
            c1: Vector3 {
                x: 0.2664000,
                y: 1.7135000,
                z: -0.0685000,
            },
            c2: Vector3 {
                x: -0.1614000,
                y: 0.0367000,
                z: 1.0296000,
            },
        };

        let bradford_matrix_inverse = Matrix3x3 {
            c0: Vector3 {
                x: 0.9869929,
                y: 0.4323053,
                z: -0.0085287,
            },
            c1: Vector3 {
                x: -0.1470543,
                y: 0.5183603,
                z: 0.0400428,
            },
            c2: Vector3 {
                x: 0.1599627,
                y: 0.0492912,
                z: 0.9684867,
            },
        };

        // "crs" stands for "Cone response of source white point"
        // "crd" stands for "Cone response of destination white point"
        // The xyz values correspond to the response of the three cones.
        // These three responses are the "LMS" color space.
        // "LMS" stands for "Long", "Medium", "Short" based on the wavelengths
        // the three types of cones respond to.
        let crs = bradford_matrix * source_white_point;
        let crd = bradford_matrix * destination_white_point;

        let intermediate_matrix = Matrix3x3::from_columns(
            Vector3::new(crd.x / crs.x, 0., 0.),
            Vector3::new(0., crd.y / crs.y, 0.),
            Vector3::new(0., 0., crd.z / crs.z),
        );

        let inner_matrix = bradford_matrix_inverse * intermediate_matrix * bradford_matrix;

        Self { inner_matrix }
    }

    pub fn convert(&self, xyz: XYZ) -> XYZ {
        let v = Vector3::new(xyz.X, xyz.Y, xyz.Z);
        let v = self.inner_matrix * v;
        XYZ {
            X: v.x,
            Y: v.y,
            Z: v.z,
        }
    }
}
