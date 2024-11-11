use std::ops::{Deref, Range};

use crate::material::Material;
use crate::types::{Point3, Ray, Vector3};

#[derive(Copy, Clone, Debug)]
pub struct Hit<'a> {
    pub t: f64,
    pub point: Point3,
    pub normal: Vector3,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> Hit<'a> {
    /// NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn new(
        t: f64,
        point: Point3,
        ray: &Ray,
        outward_normal: Vector3,
        material: &'a dyn Material,
    ) -> Self {
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
            material,
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
        let mut nearest_hit = None;
        let mut closest_so_far = t_range.end;

        for hittable in self {
            if let Some(hit) = hittable.hit(ray, t_range.start..closest_so_far) {
                closest_so_far = hit.t;
                nearest_hit = Some(hit);
            }
        }

        nearest_hit
    }
}
