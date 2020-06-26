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

#[cfg(test)]
mod tests;
