use std::fmt;

use crate::object::Hit;
use crate::types::{near_zero, random_unit_vector_on_sphere, Color, Ray, Vector3};

pub trait Material: fmt::Debug {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)>;
}

#[derive(Clone, Debug)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit.normal + random_unit_vector_on_sphere();

        // Catch degenerate scatter direction
        if near_zero(&scatter_direction) {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.point, scatter_direction);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}

#[derive(Clone, Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let reflected = reflect(&ray.direction, &hit.normal);
        let reflected = reflected.normalize() + self.fuzz * random_unit_vector_on_sphere();

        if reflected.dot(&hit.normal) > 0.0 {
            let scattered = Ray::new(hit.point, reflected);
            let attenuation = self.albedo;

            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

pub fn reflect(vector: &Vector3, normal: &Vector3) -> Vector3 {
    vector - 2.0 * vector.dot(normal) * normal
}
