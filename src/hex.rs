/// Allows converting colors into single `u32` hex values.
pub trait Hex: Sized {
    /// The components of the color.
    ///
    /// Invariant: each component must have a minimum of 0 and a maximum of 1.
    fn components(self) -> (f32, f32, f32);

    /// Converts the color to a hex value.
    fn hex(self) -> u32 {
        let (c0, c1, c2) = self.components();

        let scale = |n: f32| (n * 255.0).round() as u8;
        let (c0, c1, c2) = (scale(c0), scale(c1), scale(c2));

        (u32::from(c0) << 16) | (u32::from(c1) << 8) | u32::from(c2)
    }
}
