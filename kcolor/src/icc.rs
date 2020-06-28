use crate::*;
use icc_parser::*;

#[derive(Debug)]

pub enum ICCProfileError {
    ParseError(ParseError),
    UnsupportedICCProfile,
}
impl ColorSpace {
    /// This function does not handle all ICC profiles, or even most.
    /// Presently it only handles RGB profiles that define
    /// primaries, a white point, and a transfer function
    pub fn from_icc_profile(bytes: &[u8]) -> Result<Self, ICCProfileError> {
        use ICCProfileError::*;

        let mut parser = ICCParser::new(bytes).unwrap();

        let mut red_primary = None;
        let mut blue_primary = None;
        let mut green_primary = None;

        // Why is white point unnecessary?
        let mut _white_point = None;

        let mut red_tone_reproduction_curve = None;
        let mut blue_tone_reproduction_curve = None;
        let mut green_tone_reproduction_curve = None;

        let mut chromatic_adaptation = None;
        while let Ok(tag) = parser.next_tag() {
            match tag.tag_type {
                TagType::RedPrimary => match parser.tag_data(tag).map_err(|p| ParseError(p))? {
                    TagData::XYZ(x) => red_primary = Some(x),
                    _ => {}
                },
                TagType::GreenPrimary => match parser.tag_data(tag).map_err(|p| ParseError(p))? {
                    TagData::XYZ(x) => green_primary = Some(x),
                    _ => {}
                },
                TagType::BluePrimary => match parser.tag_data(tag).map_err(|p| ParseError(p))? {
                    TagData::XYZ(x) => blue_primary = Some(x),
                    _ => {}
                },
                TagType::WhitePoint => match parser.tag_data(tag).map_err(|p| ParseError(p))? {
                    TagData::XYZ(x) => _white_point = Some(x),
                    _ => {}
                },
                TagType::RedToneReproductionCurve => {
                    match parser.tag_data(tag).map_err(|p| ParseError(p))? {
                        TagData::ParametricCurve(x) => red_tone_reproduction_curve = Some(x),
                        _ => {}
                    }
                }
                TagType::GreenToneReproductionCurve => {
                    match parser.tag_data(tag).map_err(|p| ParseError(p))? {
                        TagData::ParametricCurve(x) => green_tone_reproduction_curve = Some(x),
                        _ => {}
                    }
                }
                TagType::BlueToneReproductionCurve => {
                    match parser.tag_data(tag).map_err(|p| ParseError(p))? {
                        TagData::ParametricCurve(x) => blue_tone_reproduction_curve = Some(x),
                        _ => {}
                    }
                }
                TagType::ChromaticAdaptationMatrix => {
                    match parser.tag_data(tag).map_err(|p| ParseError(p))? {
                        TagData::Array9(x) => chromatic_adaptation = Some(x),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        let red_primary = red_primary.map_or(Err(UnsupportedICCProfile), |p| Ok(p))?;
        let green_primary = green_primary.map_or(Err(UnsupportedICCProfile), |p| Ok(p))?;
        let blue_primary = blue_primary.map_or(Err(UnsupportedICCProfile), |p| Ok(p))?;

        //let white_point = white_point.map_or(Err(UnsupportedICCProfile), |p| Ok(p))?;

        let red_tone_reproduction_curve =
            red_tone_reproduction_curve.map_or(Err(UnsupportedICCProfile), |p| Ok(p))?;
        let _green_tone_reproduction_curve =
            green_tone_reproduction_curve.map_or(Err(UnsupportedICCProfile), |p| Ok(p))?;
        let _blue_tone_reproduction_curve =
            blue_tone_reproduction_curve.map_or(Err(UnsupportedICCProfile), |p| Ok(p))?;
        let _c = chromatic_adaptation.map_or(Err(UnsupportedICCProfile), |p| Ok(p))?;

        // The chromatic adaptation matrix describes how to convert an XYZ
        // color from the white point to the native (D50) color space.

        /*
        let chromatic_adaptation = Matrix3x3::from_columns(
            Vector3::new(c[0], c[3], c[6]),
            Vector3::new(c[1], c[4], c[7]),
            Vector3::new(c[2], c[5], c[8]),
        )
        .inverse();
        */

        // It's incorrect to use only one tone reproduction curve, but for now it's ok
        // Most of the time tone reproduction curves are shared between red, green, and blue.
        Ok(ColorSpace::new_xyz_d50(
            red_primary,
            green_primary,
            blue_primary,
            TransferFunction::ParametricCurve(red_tone_reproduction_curve),
        ))
    }
}
