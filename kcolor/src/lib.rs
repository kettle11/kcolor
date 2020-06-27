//! This library is incomplete and undertested. Don't use it yet.
//
// Non-snake case is allowed because XYZ and xyY are traditionally
// capitalized a specific way.
#[allow(non_snake_case)]
mod color;
#[allow(non_snake_case)]
pub mod color_space;

pub use color::Color;
pub use color_space::*;

pub use kcolor_types::*;

mod constant_color_spaces;
pub mod white_points;

pub use constant_color_spaces::*;

mod icc;

#[cfg(test)]
mod tests;
