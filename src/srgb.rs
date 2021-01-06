/// An sRGB color.
///
/// To convert sRGB colors to other color spaces they must first be made linear.
/// This can be done with [`Self::from_linear`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Srgb {
    /// Red (0 to 1).
    pub r: f32,
    /// Green (0 to 1).
    pub g: f32,
    /// Blue (0 to 1).
    pub b: f32,
}

impl Srgb {
    /// Converts a linear RGB value to an sRGB one.
    pub fn from_linear(linear_rgb: crate::LinearRgb) -> Self {
        let from_linear = |n: f32| {
            if n <= 0.0031308 {
                n * 12.92
            } else {
                n.powf(1.0 / 2.4) * 1.055 - 0.055
            }
        };

        Self {
            r: from_linear(linear_rgb.r),
            g: from_linear(linear_rgb.g),
            b: from_linear(linear_rgb.b),
        }
    }

    /// Converts this sRGB color to one in the linear RGB color space.
    pub fn to_linear(self) -> crate::LinearRgb {
        let to_linear = |n: f32| {
            if n <= 0.04045 {
                n / 12.92
            } else {
                ((n + 0.055) / 1.055).powf(2.4)
            }
        };

        crate::LinearRgb {
            r: to_linear(self.r),
            g: to_linear(self.g),
            b: to_linear(self.b),
        }
    }
}
