use derive_more::derive::{AddAssign, MulAssign};
use derive_more::{Add, Deref, DerefMut, From, Mul, Sub};
use nalgebra::ComplexField;

use super::Vector3;

#[derive(
    Copy,
    Clone,
    From,
    Deref,
    DerefMut,
    Debug,
    Default,
    PartialEq,
    Add,
    Sub,
    Mul,
    AddAssign,
    MulAssign,
)]
pub struct Color(pub Vector3);

impl Color {
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

    pub fn gradient_white_to_blue(point: f64) -> Color {
        let a = 0.5 * (point + 1.0);
        (1.0 - a) * Color::WHITE + a * Color::new(0.5, 0.7, 1.0)
    }

    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vector3::new(r, g, b))
    }

    pub fn to_byte(&self) -> [u8; 3] {
        let (r, g, b) = (self.x, self.y, self.z);

        // Translate the 0.0..=1.0 component values to the byte range 0..=255.
        [(r * 256.0) as u8, (g * 256.0) as u8, (b * 256.0) as u8]
    }

    pub fn to_vec(&self) -> Vector3 {
        self.0
    }

    pub fn clamp(&self) -> Self {
        Self::new(
            self.x.clamp(0.0, 1.0),
            self.y.clamp(0.0, 1.0),
            self.z.clamp(0.0, 1.0),
        )
    }

    pub fn to_gamma_2_color(&self) -> Self {
        let r = self.0.x.try_sqrt().unwrap_or(0.0);
        let g = self.0.y.try_sqrt().unwrap_or(0.0);
        let b = self.0.z.try_sqrt().unwrap_or(0.0);

        Self::new(r, g, b)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(self * rhs.0)
    }
}

impl Add<Color> for Vector3 {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color(self + rhs.0)
    }
}
