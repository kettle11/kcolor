//! This module handles parsing the various tags within an ICC profile.

use crate::*;

#[derive(Clone, PartialEq, Debug)]

pub struct Tag {
    pub tag_type: TagType,
    offset: u32,
    size: u32,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TagType {
    Description,
    WhitePoint,
    BlackPoint,
    RedPrimary,
    GreenPrimary,
    BluePrimary,
    RedToneReproductionCurve,
    GreenToneReproductionCurve,
    BlueToneReproductionCurve,
    ChromaticAdaptationMatrix,
    Other(ShortString),
}

#[derive(Debug)]
pub enum TagData {
    DescriptionString(String),
    MultiLocalizedStrings(Vec<(Locale, String)>),
    XYZ(XYZ),
    ParametricCurve(ParametricCurve),
    Array9([f64; 9]), // This does not directly correspond to an item in the spec. It's just for convenience.
    Unknown,
}

impl<'a> ICCParser<'a> {
    pub fn next_tag(&mut self) -> Result<Tag, ParseError> {
        if self.current_tag < self.tag_count {
            self.current_tag += 1;
            let (signature, offset, size) = self.parse_tag_info(self.current_tag - 1)?;
            let tag_type = match signature.into_str() {
                "desc" => TagType::Description,
                "wtpt" => TagType::WhitePoint,
                "bkpt" => TagType::BlackPoint,
                "rXYZ" => TagType::RedPrimary,
                "gXYZ" => TagType::GreenPrimary,
                "bXYZ" => TagType::BluePrimary,
                // aarg, aagg, aabg were observedin Apple profiles. But where are they specified?
                // Is there any issue with supporting them this way?
                "rTRC" | "aarg" => TagType::RedToneReproductionCurve,
                "gTRC" | "aagg" => TagType::GreenToneReproductionCurve,
                "bTRC" | "aabg" => TagType::BlueToneReproductionCurve,
                "chad" => TagType::ChromaticAdaptationMatrix,
                _ => TagType::Other(signature),
            };
            Ok(Tag {
                tag_type,
                offset,
                size,
            })
        } else {
            Err(ParseError::NoMoreTags) // Not an error
        }
    }

    pub fn tag_data(&mut self, tag: Tag) -> Result<TagData, ParseError> {
        Ok(self.parse_tag_data(tag.offset as usize, tag.size as usize)?)
    }

    pub(crate) fn parse_tag_info(
        &mut self,
        index: u32,
    ) -> Result<(ShortString, u32, u32), ParseError> {
        self.i = (132 + index * 12) as usize; // Skip header + 4 bytes for tag
        let signature = self.read_short_string()?;
        let offset = self.read_u32()?;
        let size = self.read_u32()?;
        Ok((signature, offset, size))
    }

    pub(crate) fn parse_tag_data(
        &mut self,
        data_start: usize,
        size: usize,
    ) -> Result<TagData, ParseError> {
        use TagData::*;

        // Reading at a different location, so preserve the old index.
        let old_i = self.i;
        self.i = data_start;
        let type_signature = self.read_short_string()?;
        let type_signature_str = type_signature.into_str();
        let _reserved = self.read_u32()?;

        let result = match type_signature_str {
            "desc" => DescriptionString(self.parse_desc_data()?),
            "mluc" => MultiLocalizedStrings(self.parse_multi_localized_unicode(data_start)?),
            "XYZ " => XYZ(self.parse_xyz_data()?),
            "para" => ParametricCurve(self.parse_para_data()?),
            "sf32" => self.parse_sf32(size)?,
            _ => Unknown,
        };
        self.i = old_i;

        Ok(result)
    }

    /// Super incorrect right now
    pub fn parse_sf32(&mut self, tag_length: usize) -> Result<TagData, ParseError> {
        // Each tag has 8 bytes for the signature and reserved area.
        // If this is exactly 9 (as will be common for 3x3 matrices)
        // then parse / store the data in a convenient way.
        if tag_length - 8 == 36 {
            Ok(TagData::Array9(self.read_s15_fixed_16_array_length_9()?))
        } else {
            Err(ParseError::UnimplementedInICCParser)
        }
    }

    pub(crate) fn parse_multi_localized_unicode(
        &mut self,
        tag_start: usize,
    ) -> Result<Vec<(Locale, String)>, ParseError> {
        // multilocalizedUnicodeType
        // See section 10.13 page 61 of the specification

        // A multi-localized string is a table of strings with language and country codes.

        let number_of_records = self.read_u32()?;
        // record_size is always equal to 12
        let _record_size = self.read_u32()?;

        let mut strings = Vec::with_capacity(number_of_records as usize);

        for _ in 0..number_of_records {
            let language_code = self.read_bytes(2)?;
            let country_code = self.read_bytes(2)?;
            // string_length is in bytes
            let string_length = self.read_u32()?;
            // Offset is from the start of the tag
            let string_offset = self.read_u32()?;

            let string =
                self.read_u16_string(string_offset as usize + tag_start, string_length as usize)?;

            let locale = Locale::new(language_code, country_code);
            strings.push((locale, string))
            // For now nothing is done with this tag
        }
        Ok(strings)
    }

    // INCOMPLETE
    pub(crate) fn parse_desc_data(&mut self) -> Result<String, ParseError> {
        // This is based on the V2 spec:
        // http://www.color.org/ICC_Minor_Revision_for_Web.pdf
        // See page 6.5.17

        // This is a description format defined in the V2 spec that
        // seems to still be used in V4 profiles.
        let ascii_length = self.read_u32()?;
        let ascii_name = self.read_utf8_string(ascii_length as usize)?;
        let _unicode_language_code = self.read_u32()?;
        let _unicode_length = self.read_u32()?;
        // To-do unicode description goes here
        let _script_code_code = self.read_u16()?;
        let _mac_description_count = self.read_u8()?;
        // To-do Read mac description here

        Ok(ascii_name.to_owned())
    }

    fn parse_xyz_data(&mut self) -> Result<XYZ, ParseError> {
        Ok(XYZ {
            X: self.read_s15_fixed_16_number()?,
            Y: self.read_s15_fixed_16_number()?,
            Z: self.read_s15_fixed_16_number()?,
        })
    }

    /// Parse parametric curve data.
    /// See table 65 on page 69 of the specification.
    fn parse_para_data(&mut self) -> Result<ParametricCurve, ParseError> {
        let function_type = self.read_u16()?;
        let _reserved = self.read_u16();
        match function_type {
            0 => Ok(ParametricCurve::Function0 {
                gamma: self.read_s15_fixed_16_number()?,
            }),

            1 => Ok(ParametricCurve::Function1 {
                gamma: self.read_s15_fixed_16_number()?,
                a: self.read_s15_fixed_16_number()?,
                b: self.read_s15_fixed_16_number()?,
            }),

            2 => Ok(ParametricCurve::Function2 {
                gamma: self.read_s15_fixed_16_number()?,
                a: self.read_s15_fixed_16_number()?,
                b: self.read_s15_fixed_16_number()?,
                c: self.read_s15_fixed_16_number()?,
            }),
            3 => Ok(ParametricCurve::Function3 {
                gamma: self.read_s15_fixed_16_number()?,
                a: self.read_s15_fixed_16_number()?,
                b: self.read_s15_fixed_16_number()?,
                c: self.read_s15_fixed_16_number()?,
                d: self.read_s15_fixed_16_number()?,
            }),
            4 => Ok(ParametricCurve::Function4 {
                gamma: self.read_s15_fixed_16_number()?,
                a: self.read_s15_fixed_16_number()?,
                b: self.read_s15_fixed_16_number()?,
                c: self.read_s15_fixed_16_number()?,
                d: self.read_s15_fixed_16_number()?,
                e: self.read_s15_fixed_16_number()?,
                f: self.read_s15_fixed_16_number()?,
            }),
            _ => Err(ParseError::UnableToParse),
        }
    }
}
