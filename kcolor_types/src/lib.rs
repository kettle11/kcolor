//! An internal library used to define common data structures between `icc_parser` and `kcolor`

mod math;
pub use math::*;

/// A coordinate in the 1931 CIE XYZ color space.
/// Y corresponds to luminance, X and Y are hue.
// Snake case name is allowed because upper and lowercase are used to mean different things
// in different color spaces.
#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct XYZ {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}

impl XYZ {
    #[allow(non_snake_case)]
    pub fn new(X: f64, Y: f64, Z: f64) -> Self {
        Self { X, Y, Z }
    }

    pub fn to_chromaticity(&self) -> Chromaticity {
        Chromaticity {
            x: self.X / (self.X + self.Y + self.Z),
            y: self.Y / (self.X + self.Y + self.Z),
        }
    }

    pub fn to_vector3(&self) -> Vector3 {
        Vector3 {
            x: self.X,
            y: self.Y,
            z: self.Z,
        }
    }
}

/// Chromaticity values represent the hue of a color, irrespective of brightness
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Chromaticity {
    pub x: f64,
    pub y: f64,
}

impl Chromaticity {
    pub fn new(x: f64, y: f64) -> Self {
        Chromaticity { x, y }
    }

    #[allow(non_snake_case)]
    pub fn to_XYZ(&self) -> XYZ {
        XYZ::new(self.x / self.y, 1.0, (1.0 - self.x - self.y) / self.y)
    }
}

/// A transfer function describes converting between a linear and non-linear color space.
#[derive(Debug, Clone, PartialEq)]
pub enum TransferFunction {
    ParametricCurve(ParametricCurve),
    None,
}

// These definitions are from table 65 on page 69 of the specification.
// IMPORTANT: That table has the '<' symbol incorrectly reversed for the second part of the domain.
// That mistake is corrected in errata 5:
// http://www.color.org/specification/ICC1-2010_Cumulative_Errata_List_2019-05-29.pdf
#[derive(Debug, Clone, PartialEq)]
pub enum ParametricCurve {
    /// X is the input value and Y is the returned value:
    ///
    /// Y = X^gamma
    Function0 { gamma: f64 },
    /// X is the input value and Y is the returned value:
    ///
    /// if X >= -b / a {
    ///     Y = (a * X + b) ^ gamma
    /// } else {
    ///     Y = 0.
    /// }
    Function1 { gamma: f64, a: f64, b: f64 },
    /// X is the input value and Y is the returned value:
    ///
    /// if X >= -b / a {
    ///     Y = (a * X + b) ^ gamma + c
    /// } else {
    ///     Y = c
    /// }
    Function2 { gamma: f64, a: f64, b: f64, c: f64 },
    /// Used by sRGB
    /// X is the input value and Y is the returned value:
    ///
    /// if X >= d {
    ///     Y = (a * X + b) ^ gamma
    /// } else {
    ///     Y = (c * x)
    /// }
    Function3 {
        gamma: f64,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
    },
    /// This could be used by Rec. 2020
    /// X is the input value and Y is the returned value:
    ///
    /// if X >= d {
    ///     Y = (a * X + b) ^ gamma + e
    /// } else {
    ///     Y = c * X + f
    /// }
    ///
    /// IMPORTANT: In the original specification the 'e' is mistakingly written as 'c'
    /// That was fixed in errata 7:
    /// http://www.color.org/specification/ICC1-2010_Cumulative_Errata_List_2019-05-29.pdf
    Function4 {
        gamma: f64,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    },
}
