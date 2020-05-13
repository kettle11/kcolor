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

// This first set of tests tests conversions to and from sRGB space
#[test]
fn srgb_red() {
    let red_f64 = (1.0, 0.0, 0.0, 1.0);
    let red = Color::new_srgb(red_f64.0, red_f64.1, red_f64.2, red_f64.3);
    let rgba = red.to_srgb();
    assert!(approx_equal(red_f64, rgba));
}

#[test]
fn srgb_half_red() {
    let red_f64 = (0.5, 0.0, 0.0, 1.0);
    let red = Color::new_srgb(red_f64.0, red_f64.1, red_f64.2, red_f64.3);
    let rgba = red.to_srgb();
    assert!(approx_equal(red_f64, rgba));
}

#[test]
fn srgb_green() {
    let color_f64 = (0.0, 1.0, 0.0, 1.0);
    let color = Color::new_srgb(color_f64.0, color_f64.1, color_f64.2, color_f64.3);
    let color_rgba = color.to_srgb();
    assert!(approx_equal(color_f64, color_rgba));
}

#[test]
fn srgb_blue() {
    let color_f64 = (0.0, 0.0, 1.0, 1.0);
    let color = Color::new_srgb(color_f64.0, color_f64.1, color_f64.2, color_f64.3);
    let color_rgba = color.to_srgb();
    assert!(approx_equal(color_f64, color_rgba));
}

#[test]
fn srgb_white() {
    let color_f64 = (1.0, 1.0, 1.0, 1.0);
    let color = Color::new_srgb(color_f64.0, color_f64.1, color_f64.2, color_f64.3);
    let color_rgba = color.to_srgb();
    assert!(approx_equal(color_f64, color_rgba));
}

#[test]
fn srgb_black() {
    let color_f64 = (0.0, 0.0, 0.0, 0.0);
    let color = Color::new_srgb(color_f64.0, color_f64.1, color_f64.2, color_f64.3);
    let color_rgba = color.to_srgb();
    assert!(approx_equal(color_f64, color_rgba));
}

#[test]
fn srgb_negative() {
    let color_f64 = (-0.5, 0.0, 0.0, 0.0);
    let color = Color::new_srgb(color_f64.0, color_f64.1, color_f64.2, color_f64.3);
    let color_rgba = color.to_srgb();
    assert!(approx_equal(color_f64, color_rgba));
}
