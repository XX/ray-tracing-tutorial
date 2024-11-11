use std::ops::Range;

use crate::material::Material;
use crate::object::{Hit, Hittable};
use crate::types::{Point3, Ray};

pub struct Sphere<M> {
    center: Point3,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, material: M) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit> {
        let oc = self.center - ray.origin;
        let a = ray.direction.norm_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.norm_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;

        if discriminant >= 0.0 {
            let sqrt = discriminant.sqrt();

            // Find the nearest root that lies in the acceptable range.
            for root in [(h - sqrt) / a, (h + sqrt) / a] {
                if t_range.contains(&root) {
                    let point = ray.at(root);
                    return Some(Hit::new(
                        root,
                        point,
                        ray,
                        (point - self.center) / self.radius,
                        &self.material,
                    ));
                }
            }
        }
        None
    }
}
