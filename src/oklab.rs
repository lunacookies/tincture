/// A color from the [Oklab] color space.
///
/// [Oklab]: https://bottosson.github.io/posts/oklab/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Oklab {
    /// Lightness.
    /// 0 is complete black, 1 is the brightest white.
    pub l: f32,
    /// Green vs red.
    /// -1 is green, 1 is red.
    pub a: f32,
    /// Blue vs yellow.
    /// -1 is blue, 1 is yellow.
    pub b: f32,
}

const M1: [[f32; 3]; 3] = [
    [0.8189330101, 0.3618667424, -0.1288597137],
    [0.0329845436, 0.9293118715, 0.0361456387],
    [0.0482003018, 0.2643662691, 0.6338517070],
];

const M1_INV: [[f32; 3]; 3] = [
    [1.2270138511, -0.5577999807, 0.2812561490],
    [-0.0405801784, 1.1122568696, -0.0716766787],
    [-0.0763812845, -0.4214819784, 1.5861632204],
];

const M2: [[f32; 3]; 3] = [
    [0.2104542553, 0.7936177850, -0.0040720468],
    [1.9779984951, -2.4285922050, 0.4505937099],
    [0.0259040371, 0.7827717662, -0.8086757660],
];

const M2_INV: [[f32; 3]; 3] = [
    [0.9999999985, 0.3963377922, 0.2158037581],
    [1.0000000089, -0.1055613423, -0.0638541748],
    [1.0000000547, -0.0894841821, -1.2914855379],
];

impl From<crate::Oklch> for Oklab {
    fn from(oklch: crate::Oklch) -> Self {
        Self {
            l: oklch.l,
            a: oklch.c * oklch.h.unnormalized_degrees.cos(),
            b: oklch.c * oklch.h.unnormalized_degrees.sin(),
        }
    }
}

#[allow(clippy::many_single_char_names)]
impl crate::CoreColorSpace for Oklab {
    fn from_xyz(xyz: crate::Xyz) -> Self {
        let l = M1[0][0] * xyz.x + M1[0][1] * xyz.y + M1[0][2] * xyz.z;
        let m = M1[1][0] * xyz.x + M1[1][1] * xyz.y + M1[1][2] * xyz.z;
        let s = M1[2][0] * xyz.x + M1[2][1] * xyz.y + M1[2][2] * xyz.z;

        let l_ = l.cbrt();
        let m_ = m.cbrt();
        let s_ = s.cbrt();

        let l = M2[0][0] * l_ + M2[0][1] * m_ + M2[0][2] * s_;
        let a = M2[1][0] * l_ + M2[1][1] * m_ + M2[1][2] * s_;
        let b = M2[2][0] * l_ + M2[2][1] * m_ + M2[2][2] * s_;

        Self { l, a, b }
    }

    fn to_xyz(self) -> crate::Xyz {
        let l_ = M2_INV[0][0] * self.l + M2_INV[0][1] * self.a + M2_INV[0][2] * self.b;
        let m_ = M2_INV[1][0] * self.l + M2_INV[1][1] * self.a + M2_INV[1][2] * self.b;
        let s_ = M2_INV[2][0] * self.l + M2_INV[2][1] * self.a + M2_INV[2][2] * self.b;

        let l = l_.powi(3);
        let m = m_.powi(3);
        let s = s_.powi(3);

        let x = M1_INV[0][0] * l + M1_INV[0][1] * m + M1_INV[0][2] * s;
        let y = M1_INV[1][0] * l + M1_INV[1][1] * m + M1_INV[1][2] * s;
        let z = M1_INV[2][0] * l + M1_INV[2][1] * m + M1_INV[2][2] * s;

        crate::Xyz { x, y, z }
    }
}
