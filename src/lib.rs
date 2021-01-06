//! A crate for converting colors between different color spaces.
//!
//! Color spaces can be converted between one another using [`convert`]:
//!
//! ```
//! use tincture::{LinearRgb, Oklab};
//!
//! let rebeccapurple = LinearRgb {
//!     r: 0.4,
//!     g: 0.2,
//!     b: 0.6,
//! };
//!
//! let oklab: Oklab = tincture::convert(rebeccapurple);
//!
//! assert_eq!(
//!     oklab,
//!     Oklab {
//!         l: 0.66066486,
//!         a: 0.079970956,
//!         b: -0.095915854,
//!     },
//! );
//! ```
//!
//! Variations on the core color spaces do not implement the necessary trait to use [`convert`] directly.
//! Instead, they implement `From<ACoreColorSpace>`, allowing you to convert this variation to its corresponding core color space and call [`convert`]:
//!
//! ```
//! use tincture::{Hue, LinearRgb, Oklab, Oklch, Srgb};
//!
//! // `Oklch` is a variation on `Oklab` (`Oklch` uses polar coordinates).
//! let peach = Oklch {
//!     l: 0.8,
//!     c: 0.25,
//!     h: Hue::from_degrees(40.0),
//! };
//!
//! // This means we can create an `Oklab` using `From`.
//! let oklab = Oklab::from(peach);
//!
//! // We can now convert `oklab` to any other core color space, such as `LinearRgb`.
//! let linear_rgb: LinearRgb = tincture::convert(oklab);
//!
//! // `Srgb` is a variant of `LinearRgb`, so we again create one using `From`.
//! let srgb = Srgb::from(linear_rgb);
//! ```

#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![allow(clippy::excessive_precision)]

mod hex;
mod hue;
mod linear_rgb;
mod oklab;
mod oklch;
mod srgb;
mod xyz;

pub use hex::Hex;
pub use hue::Hue;
pub use linear_rgb::LinearRgb;
pub use oklab::Oklab;
pub use oklch::Oklch;
pub use srgb::Srgb;
pub use xyz::Xyz;

/// A trait that describes what behavior a color must have to interoperate with the rest of the system.
pub trait Color {
    /// Convert a color in the XYZ color space to the color space that `Self` represents.
    fn from_xyz(xyz: Xyz) -> Self;

    /// Convert the color of `Self` to the XYZ color space.
    fn to_xyz(self) -> Xyz;
}

/// Convert a color from one color space to another.
pub fn convert<In: Color, Out: Color>(color: In) -> Out {
    let xyz = color.to_xyz();
    Out::from_xyz(xyz)
}
