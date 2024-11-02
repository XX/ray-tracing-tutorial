use std::ops::Range;

use crate::types::{Point3, Ray, Vector3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit>;
}

#[derive(Copy, Clone, Debug)]
pub struct Hit {
    pub t: f64,
    pub point: Point3,
    pub normal: Vector3,
    pub front_face: bool,
}

impl Hit {
    /// NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn new(t: f64, point: Point3, ray: &Ray, outward_normal: Vector3) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            t,
            point,
            normal,
            front_face,
        }
    }
}
