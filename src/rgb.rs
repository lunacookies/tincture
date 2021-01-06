/// An RGB color.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Rgb {
    /// Red (0 to 1).
    r: f32,
    /// Green (0 to 1).
    g: f32,
    /// Blue (0 to 1).
    b: f32,
}
