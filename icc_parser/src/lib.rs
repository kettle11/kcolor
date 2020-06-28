// Throughout this file when a comment mentions "the specification"
// it's referring to this document:
// http://www.color.org/specification/ICC1v43_2010-12.pdf

#[cfg(test)]
mod tests;

mod tags;
pub use tags::*;

use core::convert::TryInto;
use kcolor_types::*;
use std::str;

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

/// A 4 byte ASCII string
#[derive(PartialEq, Clone)]
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
    NoMoreTags,
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

pub struct ICCParser<'a> {
    i: usize, // Current position
    current_tag: u32,
    tag_count: u32,
    bytes: &'a [u8],
}

impl<'a> ICCParser<'a> {
    pub fn new(bytes: &'a [u8]) -> Result<Self, ParseError> {
        let mut parser = Self {
            i: 0,
            current_tag: 0,
            bytes,
            tag_count: 0,
        };

        // Skip ahead and read the tag count, error if it can't be found
        parser.i = 128;
        parser.tag_count = parser.read_u32()?;
        parser.i = 0;
        Ok(parser)
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

    fn read_i32(&mut self) -> Result<i32, ParseError> {
        let result = Ok(i32::from_be_bytes(
            (&self.bytes[self.i..self.i + 4])
                .try_into()
                .map_err(|_| ParseError::UnableToParse)?,
        ));
        self.i += 4;
        result
    }
    fn read_s15_fixed_16_number(&mut self) -> Result<f64, ParseError> {
        let u = self.read_i32()?;
        Ok(u as f64 / 65535.0)
    }

    /// Often 3x3 matrices will be parsed, so this is a special case for just those
    fn read_s15_fixed_16_array_length_9(&mut self) -> Result<[f64; 9], ParseError> {
        Ok([
            self.read_s15_fixed_16_number()?,
            self.read_s15_fixed_16_number()?,
            self.read_s15_fixed_16_number()?,
            self.read_s15_fixed_16_number()?,
            self.read_s15_fixed_16_number()?,
            self.read_s15_fixed_16_number()?,
            self.read_s15_fixed_16_number()?,
            self.read_s15_fixed_16_number()?,
            self.read_s15_fixed_16_number()?,
        ])
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

    pub fn header(&mut self) -> Result<Header, ParseError> {
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
}
