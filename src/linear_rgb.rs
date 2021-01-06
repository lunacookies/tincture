/// An RGB color without gamma correction.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct LinearRgb {
    /// Red (0 to 1).
    pub r: f32,
    /// Green (0 to 1).
    pub g: f32,
    /// Blue (0 to 1).
    pub b: f32,
}

impl crate::Color for LinearRgb {
    fn from_xyz(xyz: crate::Xyz) -> Self {
        let r = xyz.x * 3.2404542 + xyz.y * -1.5371385 + xyz.z * -0.4985314;
        let g = xyz.x * -0.9692660 + xyz.y * 1.8760108 + xyz.z * 0.0415560;
        let b = xyz.x * 0.0556434 + xyz.y * -0.2040259 + xyz.z * 1.0572252;

        Self { r, g, b }
    }

    fn to_xyz(self) -> crate::Xyz {
        crate::Xyz {
            x: self.r * 0.4124564 + self.g * 0.3575761 + self.b * 0.1804375,
            y: self.r * 0.2126729 + self.g * 0.7151522 + self.b * 0.0721750,
            z: self.r * 0.0193339 + self.g * 0.1191920 + self.b * 0.9503041,
        }
    }
}
