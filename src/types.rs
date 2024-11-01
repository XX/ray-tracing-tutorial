use std::io;

use derive_more::{Deref, DerefMut, From};

pub type Vector3 = nalgebra::Vector3<f64>;

#[derive(Copy, Clone, From, Deref, DerefMut, Debug, Default, PartialEq)]
pub struct Point3(pub Vector3);

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vector3::new(x, y, z))
    }

    pub fn to_vec(&self) -> Vector3 {
        self.0
    }
}

#[derive(Copy, Clone, From, Deref, DerefMut, Debug, Default, PartialEq)]
pub struct Color(pub Vector3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vector3::new(r, g, b))
    }

    pub fn write(&self, mut out: impl io::Write) {
        let (r, g, b) = (self.x, self.y, self.z);

        // Translate the [0,1] component values to the byte range [0,255].
        let r_byte = (r * 255.999) as u8;
        let g_byte = (g * 255.999) as u8;
        let b_byte = (b * 255.999) as u8;

        writeln!(out, "{r_byte} {g_byte} {b_byte}").expect("Failed to write color");
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
        (self.origin.to_vec() + t * self.direction).into()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Hit {
    pub t: f64,
    pub point: Point3,
    pub normal: Vector3,
}
