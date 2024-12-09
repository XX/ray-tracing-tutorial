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

#[derive(Clone, Debug)]
pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let attenuation = Color::WHITE;
        let refraction_ratio = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract {
            reflect(&unit_direction, &hit.normal)
        } else {
            refract(&unit_direction, &hit.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit.point, direction);

        Some((scattered, attenuation))
    }
}

pub fn reflect(vector: &Vector3, normal: &Vector3) -> Vector3 {
    vector - 2.0 * vector.dot(normal) * normal
}

pub fn refract(vector: &Vector3, normal: &Vector3, etai_over_etat: f64) -> Vector3 {
    let cos_theta = (-vector).dot(normal).min(1.0);
    let r_out_perp = etai_over_etat * (vector + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * normal;

    r_out_perp + r_out_parallel
}
