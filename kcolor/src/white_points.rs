use crate::Chromaticity;

/// "Horizon light". A commonly used white point.
/// https://en.wikipedia.org/wiki/Standard_illuminant
// Chromaticity values from here:
// https://en.wikipedia.org/wiki/Standard_illuminant
pub const D50_WHITE_POINT_2DEGREES: Chromaticity = Chromaticity {
    x: 0.34567,
    y: 0.35850,
};

/// A white point that corresponds to average midday light in Western / Northern Europe:
/// https://en.wikipedia.org/wiki/Illuminant_D65
// Chromaticity values from here:
// https://en.wikipedia.org/wiki/Standard_illuminant
pub const D65_WHITE_POINT_2DEGREES: Chromaticity = Chromaticity {
    x: 0.31271,
    y: 0.32902,
};
