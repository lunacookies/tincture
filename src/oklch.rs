/// A color from the polar variant of the [Oklab] color space.
///
/// [Oklab]: https://bottosson.github.io/posts/oklab/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Oklch {
    /// Lightness.
    /// 0 is complete black, 1 is the brightest white.
    pub l: f32,
    /// Chroma, which is similar to ([but not exactly the same as][chroma_vs_sat]) saturation.
    /// 0 is completely colorless, 1 is the most vivid color.
    ///
    /// [chroma_vs_sat]: https://munsell.com/color-blog/difference-chroma-saturation/
    pub c: f32,
    /// Hue.
    pub h: crate::Hue,
}

impl From<crate::Oklab> for Oklch {
    fn from(oklab: crate::Oklab) -> Self {
        let geometric_mean = |x: f32, y: f32| (x.powi(2) + y.powi(2)).sqrt();

        Self {
            l: oklab.l,
            c: geometric_mean(oklab.a, oklab.b),
            h: crate::Hue {
                unnormalized_degrees: oklab.b.atan2(oklab.a),
            },
        }
    }
}

impl crate::ColorSpace for Oklch {
    const BLACK: Self = Self {
        l: 0.0,
        c: 0.0,
        h: crate::Hue {
            unnormalized_degrees: 0.0,
        },
    };

    const WHITE: Self = Self {
        l: 1.0,
        c: 0.0,
        h: crate::Hue {
            unnormalized_degrees: 0.0,
        },
    };
}
