pub use self::color::*;

pub mod color;

pub type Vector3 = nalgebra::Vector3<f64>;
pub type Point3 = Vector3;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Basis {
    pub u: Vector3,
    pub v: Vector3,
    pub w: Vector3,
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

pub fn random_vector_in_cube() -> Vector3 {
    2.0 * Vector3::new(
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
    ) - Vector3::new(1.0, 1.0, 1.0)
}

pub fn random_vector_in_sphere() -> Vector3 {
    loop {
        let vector = random_vector_in_cube();
        let lensq = vector.norm_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return vector;
        }
    }
}

pub fn random_unit_vector_on_sphere() -> Vector3 {
    random_vector_in_sphere().normalize()
}

pub fn random_unit_vector_on_hemisphere(normal: &Vector3) -> Vector3 {
    let vector = random_unit_vector_on_sphere();
    // In the same hemisphere as the normal
    if vector.dot(normal) > 0.0 {
        vector
    } else {
        -vector
    }
}

/// Return true if the vector is close to zero in all dimensions.
pub fn near_zero(vector: &Vector3) -> bool {
    const SMALLEST: f64 = 1e-8;

    (vector.x.abs() < SMALLEST) && (vector.y.abs() < SMALLEST) && (vector.z.abs() < SMALLEST)
}
