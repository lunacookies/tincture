/// The hue of a color.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Hue {
    pub(crate) unnormalized_degrees: f32,
}

impl Hue {
    /// The hue in normalized degrees (from 0 to 360).
    pub fn normalized_degrees(self) -> f32 {
        if self.unnormalized_degrees < 0.0 {
            self.unnormalized_degrees + 360.0
        } else {
            self.unnormalized_degrees
        }
    }

    /// The hue in unnormalized degrees (from -180 to 180).
    pub fn unnormalized_degrees(self) -> f32 {
        self.unnormalized_degrees
    }
}
