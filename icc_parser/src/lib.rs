// Throughout this file when a comment mentions "the specification"
// it's referring to this document:
// http://www.color.org/specification/ICC1v43_2010-12.pdf

#[cfg(test)]
mod tests;

use core::convert::TryInto;
use std::str;

#[derive(Debug)]

pub enum TagData {
    DescriptionString(String),
    MultiLocalizedStrings(Vec<(Locale, String)>),
    XYZ(XYZ),
    ParametricCurve(ParametricCurve),
    Unknown,
}

#[derive(Debug)]

pub struct Locale {
    language: [u8; 2],
    country: [u8; 2],
}

impl Locale {
    pub fn new(language: &[u8], country: &[u8]) -> Self {
        let mut language_bytes = [0; 2];
        let mut country_bytes = [0; 2];

        language_bytes.copy_from_slice(&language[0..2]);
        country_bytes.copy_from_slice(&country[0..2]);
        Locale {
            language: language_bytes,
            country: country_bytes,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct XYZ {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}

/// A 4 byte ASCII string
pub struct ShortString([u8; 4]);

impl ShortString {
    pub fn into_str(&self) -> &str {
        str::from_utf8(&self.0).unwrap()
    }
}

impl std::fmt::Debug for ShortString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into_str())
    }
}

#[derive(Debug)]

pub struct ColorProfile {
    pub header: Header,
    pub tags: Vec<(ShortString, TagData)>,
}

#[derive(Debug)]

pub struct VersionNumber {
    major: u8,
    minor: u8,
}

#[derive(Debug)]
/// This needs documentation.
pub struct Header {
    size: u32,
    preferred_cmm_type: ShortString,
    version_number: VersionNumber,
    class: ProfileClass,
    color_space_type: ColorSpaceType,
    connection_space: ColorSpaceType,
    date_time: DateTime,
    primary_platform: ShortString,
    flags: u32,
    device_manufacturer: ShortString,
    device_model: u32,
    device_attributes: [u8; 8],
    rendering_intent: RenderingIntent,
    connection_space_illuminant: (f64, f64, f64),
    creator: ShortString,
    id: [u8; 16],
}

#[derive(Debug)]
pub enum ParseError {
    UnableToParse,
    UnimplementedInICCParser,
}

#[derive(Debug)]
pub struct DateTime {
    year: u16,
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
}
#[derive(Debug)]

pub enum RenderingIntent {
    Perceptual,
    MediaRelativeColorimetric,
    Saturation,
    ICCAbsoluteColorimetric,
}

// These definitions are from table 65 on page 69 of the specification.
// IMPORTANT: That table has the '<' symbol incorrectly reversed for the second part of the domain.
// That mistake is corrected in the Errata List as item 5:
// http://www.color.org/specification/ICC1-2010_Cumulative_Errata_List_2019-05-29.pdf
#[derive(Debug)]
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
    /// That was fixed in correction 7:
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

// http://www.color.org/specification/ICC1v43_2010-12.pdf
// See table 19 on page 21
#[derive(Debug)]
pub enum ColorSpaceType {
    XYZ,
    CIELAB,
    CIELUV,
    YCbCr,
    CIEYxy,
    RGB,
    Gray,
    HSV,
    HLS,
    CMYK,
    CMY,
    /// A multicomponent space other than the ones above
    MultipleColor(u8),
}

#[derive(Debug)]
pub enum ProfileClass {
    InputDevice,
    DisplayDevice,
    OutputDevice,
    DeviceLink,
    ColorSpace,
    Abstract,
    NamedColor,
}

struct ICCParser<'a> {
    i: usize, // Current position
    bytes: &'a [u8],
}

impl<'a> ICCParser<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { i: 0, bytes }
    }

    fn read_u8(&mut self) -> Result<u8, ParseError> {
        self.i += 1;
        Ok(self.bytes[self.i - 1])
    }

    fn read_u16(&mut self) -> Result<u16, ParseError> {
        let result = Ok(u16::from_be_bytes(
            (&self.bytes[self.i..self.i + 2])
                .try_into()
                .map_err(|_| ParseError::UnableToParse)?,
        ));
        self.i += 2;
        result
    }

    fn read_u32(&mut self) -> Result<u32, ParseError> {
        let result = Ok(u32::from_be_bytes(
            (&self.bytes[self.i..self.i + 4])
                .try_into()
                .map_err(|_| ParseError::UnableToParse)?,
        ));
        self.i += 4;
        result
    }

    fn read_s15_fixed_16_number(&mut self) -> Result<f64, ParseError> {
        Ok(self.read_u32()? as f64 / 65535.0)
    }

    fn read_short_string(&mut self) -> Result<ShortString, ParseError> {
        let mut short_string = [0; 4];
        short_string.copy_from_slice(&self.bytes[self.i..self.i + 4]);
        self.i += 4;
        Ok(ShortString(short_string))
    }

    /// Length is in characters
    fn read_u16_string(&mut self, start: usize, length_bytes: usize) -> Result<String, ParseError> {
        let old_i = self.i;
        self.i = start;
        let length_chars = length_bytes / 2;
        let mut chars = Vec::with_capacity(length_chars);
        for _ in 0..length_chars {
            let c = self.read_u16()?;
            chars.push(c);
        }

        self.i = old_i;
        Ok(String::from_utf16(&chars).map_err(|_| ParseError::UnableToParse)?)
    }

    fn read_utf8_string(&mut self, bytes: usize) -> Result<&'a str, ParseError> {
        let result = Ok(str::from_utf8(&self.bytes[self.i..self.i + bytes])
            .map_err(|_| ParseError::UnableToParse)?);
        self.i += bytes;
        result
    }

    fn read_bytes(&mut self, bytes: usize) -> Result<&'a [u8], ParseError> {
        let result = Ok(&self.bytes[self.i..self.i + bytes]);
        self.i += bytes;
        result
    }

    fn parse_date_time(&mut self) -> Result<DateTime, ParseError> {
        // See section 4.2 on page 6
        let year = self.read_u16()?;
        let month = self.read_u16()?;
        let day = self.read_u16()?;
        let hour = self.read_u16()?;
        let minute = self.read_u16()?;
        let second = self.read_u16()?;

        Ok(DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }

    fn parse_color_space_type(&mut self) -> Result<ColorSpaceType, ParseError> {
        // See table 19 on page 21
        // All strings must be 4 characters, hence the space after 3 letter signatures.

        Ok(match self.read_utf8_string(4)? {
            // Does this need to be split into nCIEXYZ or PCSXYZ? See footnote 'a' on table 19
            "XYZ " => ColorSpaceType::XYZ,
            // Does this need to be split into CIELAB or PCSLAB? See footnote 'b' on table 19
            "Lab " => ColorSpaceType::CIELAB,
            "Luv " => ColorSpaceType::CIELUV,
            "YCbr" => ColorSpaceType::YCbCr,
            "Yxy " => ColorSpaceType::CIEYxy,
            "RGB " => ColorSpaceType::RGB,
            "GRAY" => ColorSpaceType::Gray,
            "HSV " => ColorSpaceType::HSV,
            "CMYK" => ColorSpaceType::CMYK,
            "CMY " => ColorSpaceType::CMY,
            "2CLR" => ColorSpaceType::MultipleColor(2),
            "3CLR" => ColorSpaceType::MultipleColor(3),
            "4CLR" => ColorSpaceType::MultipleColor(4),
            "5CLR" => ColorSpaceType::MultipleColor(5),
            "6CLR" => ColorSpaceType::MultipleColor(6),
            "7CLR" => ColorSpaceType::MultipleColor(7),
            "8CLR" => ColorSpaceType::MultipleColor(8),
            "9CLR" => ColorSpaceType::MultipleColor(9),
            "ACLR" => ColorSpaceType::MultipleColor(10),
            "BCLR" => ColorSpaceType::MultipleColor(11),
            "CCLR" => ColorSpaceType::MultipleColor(12),
            "DCLR" => ColorSpaceType::MultipleColor(13),
            "ECLR" => ColorSpaceType::MultipleColor(14),
            "FCLR" => ColorSpaceType::MultipleColor(15),
            _ => return Err(ParseError::UnableToParse),
        })
    }

    fn parse_header(&mut self) -> Result<Header, ParseError> {
        // See section '7.2 Profile header' in this document
        // http://www.color.org/specification/ICC1v43_2010-12.pdf

        // Size of the entire profile in bytes
        let size = self.read_u32()?;

        // CMM stands for 'Color Management Module'
        // A list of CMM signatures is here: http://www.color.org/registry/signature/TagRegistry-2019-10.pdf
        let preferred_cmm_type = self.read_short_string()?;
        println!("Preferred CMM Type: {}", preferred_cmm_type.into_str());

        // Read profile version number
        let version_number = VersionNumber {
            major: self.read_u8()?,
            minor: self.read_u8()?, // Should this be split into minor version and bug fix version?
        };
        let _reserved = self.read_u8()?;
        let _reserved = self.read_u8()?;

        // Parse profile/device class
        let class = match self.read_utf8_string(4)? {
            "scnr" => ProfileClass::InputDevice,
            "mntr" => ProfileClass::DisplayDevice,
            "prtr" => ProfileClass::OutputDevice,
            "link" => ProfileClass::DeviceLink,
            "space" => ProfileClass::ColorSpace,
            "abst" => ProfileClass::Abstract,
            "nmcl" => ProfileClass::NamedColor,
            _ => return Err(ParseError::UnableToParse),
        };

        let color_space_type = self.parse_color_space_type()?;
        println!("Color space type: {:?}", color_space_type);

        // Parse the PCS (Profile connection space)
        // For all profile classes, other than DeviceLink this will be either PCSXYZ or PCSLAB
        let connection_space = self.parse_color_space_type()?;

        let date_time = self.parse_date_time()?;
        println!("Date Time: {:?}", date_time);

        // ‘acsp’ (61637370h)
        // This appears to just be a value that can be checked for validity.
        let _profile_file_signature = self.read_utf8_string(4)?;

        // Apple, Microsoft, Silicon Graphics, or Sun Microsystems
        // This doesn't seem very important.
        let primary_platform = self.read_short_string()?;

        // Flags that indicate whether a profile is embedded, and other potential options.
        // Needs investigation as to the importance.
        let flags = self.read_u32()?;

        let device_manufacturer = self.read_short_string()?;

        let device_model = self.read_u32()?;

        // Indicate properties of the media
        // like if a printer is loaded with glossy or matte paper.
        // Additionally can contain vendor specific properties.
        // Stored in bit flags
        let mut device_attributes = [0; 8];
        device_attributes.copy_from_slice(self.read_bytes(8)?);

        let rendering_intent = match self.read_u16()? {
            0 => RenderingIntent::Perceptual,
            1 => RenderingIntent::MediaRelativeColorimetric,
            2 => RenderingIntent::Saturation,
            3 => RenderingIntent::ICCAbsoluteColorimetric,
            _ => return Err(ParseError::UnableToParse),
        };

        println!("RENDERING INTENT: {:?}", rendering_intent);

        let _always_zero = self.read_u16()?;

        // Contains nCIEXYZ values.
        // This is required to always be D50.
        // Or X = 0.9642, Y = 1.0, Z = 0.8249
        let connection_space_illuminant = (
            self.read_s15_fixed_16_number()?,
            self.read_s15_fixed_16_number()?,
            self.read_s15_fixed_16_number()?,
        );

        let creator = self.read_short_string()?;

        // A profile ID is a MD5 fingerprint of the entire profile
        // with the following fields set to zeros:
        // * Profile flags field
        // * Rendering intent field
        // * Profile ID field
        // The inclusion of this field is only a suggestion.
        let mut id = [0; 16];
        id.copy_from_slice(self.read_bytes(16)?);

        // Bytes reserved for future expansion
        let _unused_and_reserved = self.read_bytes(28)?;

        Ok(Header {
            size,
            preferred_cmm_type: preferred_cmm_type,
            version_number,
            class,
            color_space_type,
            connection_space,
            date_time,
            primary_platform,
            flags,
            device_manufacturer,
            device_model,
            device_attributes,
            rendering_intent,
            connection_space_illuminant,
            creator,
            id,
        })
    }

    fn parse_multi_localized_unicode(
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
            println!("string: {:?}", string);

            let locale = Locale::new(language_code, country_code);
            strings.push((locale, string))
            // For now nothing is done with this tag
        }
        Ok(strings)
    }

    // INCOMPLETE
    fn parse_desc_data(&mut self) -> Result<String, ParseError> {
        // This is based on the V2 spec:
        // http://www.color.org/ICC_Minor_Revision_for_Web.pdf
        // See page 6.5.17

        // This is a description format defined in the V2 spec that
        // seems to still be used in V4 profiles.
        let ascii_length = self.read_u32()?;
        let ascii_name = self.read_utf8_string(ascii_length as usize)?;
        println!("ASCII NAME: {:?}", ascii_name);
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

    fn parse_tag_data(&mut self, data_start: usize) -> Result<TagData, ParseError> {
        use TagData::*;

        // Reading at a different location, so preserve the old index.
        let old_i = self.i;
        self.i = data_start;
        let type_signature = self.read_short_string()?;
        let type_signature_str = type_signature.into_str();
        println!("TYPE SIGNATURE: {:?}", type_signature_str);
        let _reserved = self.read_u32()?;

        let result = match type_signature_str {
            "desc" => DescriptionString(self.parse_desc_data()?),
            "mluc" => MultiLocalizedStrings(self.parse_multi_localized_unicode(data_start)?),
            "XYZ " => XYZ(self.parse_xyz_data()?),
            "para" => ParametricCurve(self.parse_para_data()?),
            _ => Unknown,
        };
        self.i = old_i;

        Ok(result)
    }

    fn parse_tags(&mut self) -> Result<Vec<(ShortString, TagData)>, ParseError> {
        // See section 7.3 page 24 of the specification.
        // The tag table includes the tag count following by a description
        // of the various tags.
        let tag_count = self.read_u32()?;
        println!("Tag count: {:?}", tag_count);

        let mut tag_data = Vec::with_capacity(tag_count as usize);

        for _ in 0..tag_count {
            let signature = self.read_short_string()?;
            let offset = self.read_u32()?;
            let size = self.read_u32()?;
            println!(
                "Signature: {:?} Offset: {:?} Size: {:?}",
                signature, offset, size
            );

            tag_data.push((signature, self.parse_tag_data(offset as usize)?));
        }
        Ok(tag_data)
    }

    fn parse(&mut self) -> Result<ColorProfile, ParseError> {
        let header = self.parse_header()?;
        println!("HEADER: {:?}", header);
        let tags = self.parse_tags()?;
        let profile = Ok(ColorProfile { header, tags });
        profile
    }
}

pub fn parse_bytes(bytes: &[u8]) -> Result<ColorProfile, ParseError> {
    let mut parser = ICCParser::new(bytes);
    Ok(parser.parse()?)
}
