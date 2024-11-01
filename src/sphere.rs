use crate::types::{Point3, Ray};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn hit(&self, ray: &Ray) -> Option<f64> {
        let oc = self.center.to_vec() - ray.origin.to_vec();
        let a = ray.direction.dot(&ray.direction);
        let b = -2.0 * ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrt = discriminant.sqrt();
            let mut t = (-b - sqrt) / (2.0 * a);
            if t < 0.0 {
                t = (-b + sqrt) / (2.0 * a);
            }
            Some(t)
        }
    }
}
