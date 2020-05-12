use crate::*;

#[test]
fn exploration() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn srgb_to_xyz() {
    let srgb_space = ColorSpace::srgb();
    let color = Color::new(0.5, 0.0, 0.0, 0.0);
    let xyz = srgb_space.to_xyz(color);
    println!("XYZ: {:?}", xyz);

    let expected = XYZ {
        x: 0.4124564,
        y: 0.2126729,
        z: 0.0193339,
    };

    // assert_eq!(xyz, expected);

    let back_to_srgb = srgb_space.from_xyz(xyz, 1.0);
    println!("BACK TO SRGB: {:?}", back_to_srgb);
}
