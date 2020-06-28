use crate::*;

impl ColorSpace {
    /// The popular sRGB color space
    /// https://en.wikipedia.org/wiki/SRGB
    /// Conversion values in table below were calculated with this library.
    /// Chromaticity of primaries as expressed in CIE XYZ 1931
    /// Red primary x: 0.64 y: 0.33
    /// Green primary x: 0.3 y: 0.6
    /// Blue primary x: 0.15 y: 0.06
    /// White point: D65
    pub const SRGB: ColorSpace = ColorSpace {
        to_XYZ: Matrix3x3 {
            c0: Vector3 {
                x: 0.4360219083775758,
                y: 0.2224751872467074,
                z: 0.013928117106761706,
            },
            c1: Vector3 {
                x: 0.3851088006156898,
                y: 0.7169066518920372,
                z: 0.09710152837405213,
            },
            c2: Vector3 {
                x: 0.14308127508123153,
                y: 0.06061819697439862,
                z: 0.7141585850968147,
            },
        },
        from_XYZ: Matrix3x3 {
            c0: Vector3 {
                x: 3.1343112068079124,
                y: -0.9787436666690797,
                z: 0.07194823874527759,
            },
            c1: Vector3 {
                x: -1.617232140026793,
                y: 1.9161138492090442,
                z: -0.22898637371922786,
            },
            c2: Vector3 {
                x: -0.4906858992761275,
                y: 0.03344988853522833,
                z: 1.4052709248478632,
            },
        },
        transfer_function: SRGB_TRANSFER_FUNCTION,
    };

    /// Exact same as the above SRGB space, except with a linear transfer function.
    pub const SRGB_LINEAR: ColorSpace = ColorSpace {
        to_XYZ: Matrix3x3 {
            c0: Vector3 {
                x: 0.4360219083775758,
                y: 0.2224751872467074,
                z: 0.013928117106761706,
            },
            c1: Vector3 {
                x: 0.3851088006156898,
                y: 0.7169066518920372,
                z: 0.09710152837405213,
            },
            c2: Vector3 {
                x: 0.14308127508123153,
                y: 0.06061819697439862,
                z: 0.7141585850968147,
            },
        },
        from_XYZ: Matrix3x3 {
            c0: Vector3 {
                x: 3.1343112068079124,
                y: -0.9787436666690797,
                z: 0.07194823874527759,
            },
            c1: Vector3 {
                x: -1.617232140026793,
                y: 1.9161138492090442,
                z: -0.22898637371922786,
            },
            c2: Vector3 {
                x: -0.4906858992761275,
                y: 0.03344988853522833,
                z: 1.4052709248478632,
            },
        },
        transfer_function: TransferFunction::None,
    };
}

pub const SRGB_TRANSFER_FUNCTION: TransferFunction =
    TransferFunction::ParametricCurve(ParametricCurve::Function3 {
        gamma: 2.4,
        a: 0.94786729857,
        b: 0.05213270142,
        c: 1.0 / 12.0, // 1.0 / 12.0
        d: 0.04045,
    });
