use derive_more::{Deref, DerefMut, From};

pub type Vector3 = nalgebra::Vector3<f64>;
pub type Point3 = Vector3;

#[derive(Copy, Clone, From, Deref, DerefMut, Debug, Default, PartialEq)]
pub struct Color(pub Vector3);

impl Color {
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

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
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        (self.origin + t * self.direction).into()
    }
}
