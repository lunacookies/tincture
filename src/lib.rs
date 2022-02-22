#![allow(clippy::excessive_precision)]

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Oklab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Oklch {
    pub l: f32,
    pub c: f32,
    pub h: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct LinearSrgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Srgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub fn linear_srgb_to_srgb(c: LinearSrgb) -> Srgb {
    let nonlinearize = |x: f32| {
        if x >= 0.0031308 {
            x.powf(1.0 / 2.4) * 1.055 - 0.055
        } else {
            x * 12.92
        }
    };

    Srgb { r: nonlinearize(c.r), g: nonlinearize(c.g), b: nonlinearize(c.b) }
}

pub fn srgb_to_linear_srgb(c: Srgb) -> LinearSrgb {
    let linearize = |x: f32| {
        if x >= 0.04045 {
            ((x + 0.055) / 1.055).powf(2.4)
        } else {
            x / 12.92
        }
    };

    LinearSrgb { r: linearize(c.r), g: linearize(c.g), b: linearize(c.b) }
}

pub fn linear_srgb_to_oklab(c: LinearSrgb) -> Oklab {
    let l = 0.4122214708 * c.r + 0.5363325363 * c.g + 0.0514459929 * c.b;
    let m = 0.2119034982 * c.r + 0.6806995451 * c.g + 0.1073969566 * c.b;
    let s = 0.0883024619 * c.r + 0.2817188376 * c.g + 0.6299787005 * c.b;

    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();

    Oklab {
        l: 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
        a: 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
        b: 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
    }
}

pub fn oklab_to_linear_srgb(c: Oklab) -> LinearSrgb {
    let l_ = c.l + 0.3963377774 * c.a + 0.2158037573 * c.b;
    let m_ = c.l - 0.1055613458 * c.a - 0.0638541728 * c.b;
    let s_ = c.l - 0.0894841775 * c.a - 1.2914855480 * c.b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    LinearSrgb {
        r: 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        g: -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        b: -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    }
}

pub fn oklab_to_oklch(c: Oklab) -> Oklch {
    Oklch { l: c.l, c: c.a.hypot(c.b), h: c.b.atan2(c.a) }
}

pub fn oklch_to_oklab(c: Oklch) -> Oklab {
    Oklab { l: c.l, a: c.c * c.h.cos(), b: c.c * c.h.sin() }
}

impl Srgb {
    pub fn hex(self) -> u32 {
        let (r, g, b) = self.components();
        (r as u32) << 16 | (g as u32) << 8 | b as u32
    }

    pub fn components(self) -> (u8, u8, u8) {
        let scale = |n: f32| (n * 255.0).round() as u8;
        (scale(self.r), scale(self.g), scale(self.b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex() {
        assert_eq!(Srgb { r: 0.0, g: 0.0, b: 0.0 }.hex(), 0x000000);
        assert_eq!(Srgb { r: 1.0, g: 0.0, b: 0.0 }.hex(), 0xFF0000);
        assert_eq!(Srgb { r: 0.0, g: 1.0, b: 0.0 }.hex(), 0x00FF00);
        assert_eq!(Srgb { r: 0.0, g: 0.0, b: 1.0 }.hex(), 0x0000FF);
        assert_eq!(Srgb { r: 1.0, g: 1.0, b: 1.0 }.hex(), 0xFFFFFF);
        assert_eq!(Srgb { r: 0.5, g: 0.3, b: 0.8 }.hex(), 0x804DCC);
    }
}
