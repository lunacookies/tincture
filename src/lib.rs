//! A crate for converting colors between different color spaces.

#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

mod oklab;
mod rgb;
mod xyz;

pub use oklab::Oklab;
pub use rgb::Rgb;
pub use xyz::Xyz;

/// A trait that describes what behavior a color must have to interoperate with the rest of the system.
pub trait Color {
    /// Converts a color in the XYZ color space to the color space that `Self` represents.
    fn from_xyz(xyz: Xyz) -> Self;

    /// Converts the color of `Self` to the XYZ color space.
    fn to_xyz(self) -> Xyz;
}

/// Converts a color from one color space to another.
pub fn convert<In: Color, Out: Color>(color: In) -> Out {
    let xyz = color.to_xyz();
    Out::from_xyz(xyz)
}
