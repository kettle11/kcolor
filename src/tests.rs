use crate::math::*;
use crate::*;

fn approx_equal_f64(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.00001
}

fn approx_equal(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
    approx_equal_f64(a.0, b.0)
        && approx_equal_f64(a.1, b.1)
        && approx_equal_f64(a.2, b.2)
        && approx_equal_f64(a.3, b.3)
}

// This test tests that the sRGB color space constant is equivalent to the one calculated.
#[test]
fn srgb_constant() {
    let srgb_color_space = ColorSpace::new(
        Chromaticity::new(0.64, 0.33),
        Chromaticity::new(0.3, 0.6),
        Chromaticity::new(0.15, 0.06),
        ColorSpace::D65_WHITE_POINT,
        SRGBTransferFunction,
    );

    assert!(srgb_color_space == ColorSpace::SRGB);
}

// This test tests that the sRGB linear color space constant is equivalent to the one calculated.
#[test]
fn srgb_linear_constant() {
    let srgb_linear_color_space = ColorSpace::new(
        Chromaticity::new(0.64, 0.33),
        Chromaticity::new(0.3, 0.6),
        Chromaticity::new(0.15, 0.06),
        ColorSpace::D65_WHITE_POINT,
        TransferFunction::None,
    );

    assert!(srgb_linear_color_space == ColorSpace::SRGB_LINEAR);
}

// Tests that sRGB values converted to and from a Color remain the same.
// Note that if the transformations are wrong, but opposite, this test will still pass.
#[test]
fn srgb_half_red() {
    let color_srgb0 = (0.5, 0.0, 0.0, 1.0);
    let color = Color::new_srgb(color_srgb0.0, color_srgb0.1, color_srgb0.2, color_srgb0.3);
    let color_srgb1 = color.to_srgb();
    assert!(approx_equal(color_srgb0, color_srgb1));
}

// Tests that an sRGB value with a negative remains the same when converted to and from a color.
// Note that if the transformations are wrong, but opposite, this test will still pass.
#[test]
fn srgb_negative() {
    let color_f64 = (-0.5, 0.0, 0.0, 0.0);
    let color = Color::new_srgb(color_f64.0, color_f64.1, color_f64.2, color_f64.3);
    let color_rgba = color.to_srgb_unclipped();

    println!("Color: {:?}", color);
    println!("RGBA: {:?}", color_rgba);
    assert!(approx_equal(color_f64, color_rgba));
}

// Tests conversion from Display P3 colorspace to sRGB
#[test]
fn display_p3_to_srgb() {
    // Display P3 and DCI P3 use color primaries that when adapted relative to D50 are negative.
    // Technically XYZ values are not allowed to be negative, however the profile is wrong
    // without the adaptation.
    // More information here: http://www.color.org/chardata/rgb/DCIP3.xalter
    let display_p3 = ColorSpace::new(
        Chromaticity { x: 0.68, y: 0.32 },
        Chromaticity { x: 0.265, y: 0.69 },
        Chromaticity { x: 0.15, y: 0.06 },
        ColorSpace::D65_WHITE_POINT,
        SRGBTransferFunction,
    );

    let color_p3 = (1.0, 0.1, 0.1, 1.0);
    let color = display_p3.new_color(color_p3.0, color_p3.1, color_p3.2, color_p3.3);
    let color_srgb_clipped = color.to_srgb();

    assert!(color_srgb_clipped == (1., 0., 0., 1.));
    let color_srgb_unclipped = color.to_srgb_unclipped();

    assert!(
        color_srgb_unclipped
            == (
                1.0921700914213934,
                -0.19516672953602265,
                -0.09607923656141225,
                1.0
            )
    );
}

// Tests conversion from sRGB to Display P3
#[test]
fn srgb_to_display_p3() {
    let display_p3 = ColorSpace::new(
        Chromaticity { x: 0.68, y: 0.32 },
        Chromaticity { x: 0.265, y: 0.69 },
        Chromaticity { x: 0.15, y: 0.06 },
        ColorSpace::D65_WHITE_POINT,
        SRGBTransferFunction,
    );

    let color = Color::new_srgb(1.0, 0.1, 0.1, 1.0);
    let color_p3 = color.to_color_space(&display_p3);

    println!("color_p3: {:?}", color_p3);
    assert!(
        color_p3
            == (
                0.9183766263231892,
                0.22905441360408713,
                0.1790212430236189,
                1.0
            )
    );
}

/// Tests chromatic adaptation
/// Chromatic adaptation finds XYZ values that are perceptually
/// similar to different XYZ values under another white point.
#[test]
fn chromatic_adaptation() {
    let chromatic_adaptation =
        ChromaticAdaptation::new(ColorSpace::D65_WHITE_POINT, ColorSpace::D50_WHITE_POINT);

    let expected = ChromaticAdaptation {
        inner_matrix: Matrix3x3 {
            c0: Vector3 {
                x: 1.0478112719598691,
                y: 0.029542405202826368,
                z: -0.009234507223803406,
            },
            c1: Vector3 {
                x: 0.022886525214775758,
                y: 0.9904844613128458,
                z: 0.015043570198253024,
            },
            c2: Vector3 {
                x: -0.05012693920986061,
                y: -0.017049121601636838,
                z: 0.7521316440046968,
            },
        },
    };

    assert!(chromatic_adaptation == expected);
}
