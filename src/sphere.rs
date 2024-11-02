use crate::types::{Hit, Point3, Ray};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn hit(&self, ray: &Ray) -> Option<Hit> {
        let oc = self.center.to_vec() - ray.origin.to_vec();
        let a = ray.direction.dot(&ray.direction);
        let h = ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrt = discriminant.sqrt();
            let mut t = (h - sqrt) / a;
            if t < 0.0 {
                t = (h + sqrt) / a;
            }
            let point = ray.at(t);
            Some(Hit {
                t,
                point,
                normal: (point.to_vec() - self.center.to_vec()) / self.radius,
            })
        }
    }
}
