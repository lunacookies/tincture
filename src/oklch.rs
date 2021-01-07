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
        Self {
            l: oklab.l,
            c: (oklab.a.powi(2) + oklab.b.powi(2)).sqrt(),
            h: crate::Hue {
                unnormalized_radians: oklab.b.atan2(oklab.a),
            },
        }
    }
}

impl crate::ColorSpace for Oklch {
    const BLACK: Self = Self {
        l: 0.0,
        c: 0.0,
        h: crate::Hue {
            unnormalized_radians: 0.0,
        },
    };

    const WHITE: Self = Self {
        l: 1.0,
        c: 0.0,
        h: crate::Hue {
            unnormalized_radians: 0.0,
        },
    };

    fn in_bounds(self) -> bool {
        crate::approx_in_range(self.l, 0.0..1.0) && crate::approx_in_range(self.c, 0.0..1.0)
    }
}
