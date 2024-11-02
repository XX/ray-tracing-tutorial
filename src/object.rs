use std::ops::{Deref, Range};

use crate::types::{Point3, Ray, Vector3};

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

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit>;
}

impl<T> Hittable for T
where
    T: Deref<Target = dyn Hittable>,
{
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit> {
        self.deref().hit(ray, t_range)
    }
}

impl<T> Hittable for [T]
where
    T: Hittable,
{
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit> {
        self.iter()
            .filter_map(|hittable| hittable.hit(ray, t_range.clone()))
            .min_by(|hit_a, hit_b| hit_a.t.total_cmp(&hit_b.t))
    }
}
