/// A type representing the [Oklab] color space.
///
/// [Oklab]: https://bottosson.github.io/posts/oklab/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Oklab {
    /// Lightness. 0 is complete black, 1 is the brightest white.
    pub l: f32,
    /// Green vs red. -1 is green, 1 is red.
    pub a: f32,
    /// Blue vs yellow. -1 is blue, 1 is yellow.
    pub b: f32,
}
