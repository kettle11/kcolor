//! This library is incomplete and undertested. Don't use it yet.
mod color;
mod color_space;
mod math;

pub use color::Color;
pub use color_space::{ColorSpace, XYZ};

#[cfg(test)]
mod tests;
