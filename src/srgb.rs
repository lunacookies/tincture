/// An sRGB color.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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

impl crate::ColorSpace for Srgb {
    const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };

    const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    fn in_bounds(self) -> bool {
        crate::approx_in_range(self.r, 0.0..1.0)
            && crate::approx_in_range(self.g, 0.0..1.0)
            && crate::approx_in_range(self.b, 0.0..1.0)
    }
}

impl crate::Hex for Srgb {
    fn components(self) -> (f32, f32, f32) {
        (self.r, self.g, self.b)
    }
}

#[cfg(test)]
#[test]
fn hex() {
    use crate::Hex;

    let rgb = Srgb {
        r: 1.0,
        g: 0.25,
        b: 1.0,
    };

    assert_eq!(rgb.hex(), 0xff40ff);
}
