/// The hue of a color.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Hue {
    pub(crate) unnormalized_degrees: f32,
}

impl Hue {
    /// Creates a new `Hue` from a hue in degrees (from 0 to 360).
    pub fn from_degrees(degrees: f32) -> Self {
        let unnormalized_degrees = if degrees > 180.0 {
            degrees - 360.0
        } else {
            degrees
        };

        Self {
            unnormalized_degrees,
        }
    }

    /// The hue in degrees (from 0 to 360).
    pub fn degrees(self) -> f32 {
        if self.unnormalized_degrees < 0.0 {
            self.unnormalized_degrees + 360.0
        } else {
            self.unnormalized_degrees
        }
    }
}
