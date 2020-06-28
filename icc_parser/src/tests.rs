use crate::*;
use std::fs;

#[test]
fn basic_run() {
    let contents = fs::read("examples/sRGB Profile.icc").expect("Could not find file");
    let mut parser = ICCParser::new(&contents).unwrap();
    let header = parser.header().unwrap();

    let mut red_primary = None;
    let mut blue_primary = None;
    let mut green_primary = None;
    let mut white_point = None;

    let mut red_tone_reproduction_curve = None;
    let mut blue_tone_reproduction_curve = None;
    let mut green_tone_reproduction_curve = None;

    while let Ok(tag) = parser.next_tag() {
        println!("Tag: {:?}", tag);
        match tag.tag_type {
            TagType::RedPrimary => match parser.tag_data(tag).unwrap() {
                TagData::XYZ(x) => red_primary = Some(x),
                _ => {}
            },
            TagType::GreenPrimary => match parser.tag_data(tag).unwrap() {
                TagData::XYZ(x) => green_primary = Some(x),
                _ => {}
            },
            TagType::BluePrimary => match parser.tag_data(tag).unwrap() {
                TagData::XYZ(x) => blue_primary = Some(x),
                _ => {}
            },
            TagType::WhitePoint => match parser.tag_data(tag).unwrap() {
                TagData::XYZ(x) => white_point = Some(x),
                _ => {}
            },
            TagType::RedToneReproductionCurve => match parser.tag_data(tag).unwrap() {
                TagData::ParametricCurve(x) => red_tone_reproduction_curve = Some(x),
                _ => {}
            },
            TagType::GreenToneReproductionCurve => match parser.tag_data(tag).unwrap() {
                TagData::ParametricCurve(x) => green_tone_reproduction_curve = Some(x),
                _ => {}
            },
            TagType::BlueToneReproductionCurve => match parser.tag_data(tag).unwrap() {
                TagData::ParametricCurve(x) => blue_tone_reproduction_curve = Some(x),
                _ => {}
            },
            _ => {}
        }
    }

    println!("Red Primary: {:?}", red_primary);
    println!("Blue Primary: {:?}", blue_primary);
    println!("Green Primary: {:?}", green_primary);

    println!("White point: {:?}", white_point);
    println!("Profile: {:?}", header);
}
