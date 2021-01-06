/// An sRGB color.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Srgb {
    /// Red (0 to 1).
    pub r: f32,
    /// Green (0 to 1).
    pub g: f32,
    /// Blue (0 to 1).
    pub b: f32,
}

impl From<crate::LinearRgb> for Srgb {
    fn from(linear: crate::LinearRgb) -> Self {
        let from_linear = |n: f32| {
            if n <= 0.0031308 {
                n * 12.92
            } else {
                n.powf(1.0 / 2.4) * 1.055 - 0.055
            }
        };

        Self {
            r: from_linear(linear.r),
            g: from_linear(linear.g),
            b: from_linear(linear.b),
        }
    }
}
