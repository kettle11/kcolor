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
        let mut white_point = None;

        let mut red_tone_reproduction_curve = None;
        let mut blue_tone_reproduction_curve = None;
        let mut green_tone_reproduction_curve = None;

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
                    TagData::XYZ(x) => white_point = Some(x),
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
                _ => {}
            }
        }

        let red_primary = red_primary.map_or(Err(UnsupportedICCProfile), |p| Ok(p))?;
        let green_primary = green_primary.map_or(Err(UnsupportedICCProfile), |p| Ok(p))?;
        let blue_primary = blue_primary.map_or(Err(UnsupportedICCProfile), |p| Ok(p))?;
        let white_point = white_point.map_or(Err(UnsupportedICCProfile), |p| Ok(p))?;
        let red_tone_reproduction_curve =
            red_tone_reproduction_curve.map_or(Err(UnsupportedICCProfile), |p| Ok(p))?;
        let _green_tone_reproduction_curve =
            green_tone_reproduction_curve.map_or(Err(UnsupportedICCProfile), |p| Ok(p))?;
        let _blue_tone_reproduction_curve =
            blue_tone_reproduction_curve.map_or(Err(UnsupportedICCProfile), |p| Ok(p))?;

        // It's incorrect to use only one tone reproduction curve, but for now it's ok
        // Most of the time tone reproduction curves are shared between red, green, and blue.
        Ok(ColorSpace::new(
            red_primary.to_chromaticity(),
            green_primary.to_chromaticity(),
            blue_primary.to_chromaticity(),
            white_point.to_chromaticity(),
            TransferFunction::ParametricCurve(red_tone_reproduction_curve),
        ))
    }
}
