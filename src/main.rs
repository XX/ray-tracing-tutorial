use std::io;

use material::Dielectric;

use crate::material::{Lambertian, Metal};
use crate::object::Hittable;
use crate::render::{Fading, Renderer};
use crate::sphere::Sphere;
use crate::types::{Color, Point3};
use crate::utils::Logger;

mod camera;
mod material;
mod object;
mod render;
mod sphere;
mod types;
mod utils;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    let mut log = Logger::new(io::stderr());

    let image_width = 800;
    let image_height = (image_width as f64 / ASPECT_RATIO) as usize;
    let mut renderer = Renderer::new(image_width, image_height)
        .with_samples_per_pixel(100)
        .with_max_depth(50);

    let ground = Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Lambertian::new(Color::new(0.8, 0.8, 0.0)),
    );
    let central = Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Lambertian::new(Color::new(0.1, 0.2, 0.5)),
    );
    let left = Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Dielectric::new(1.0 / 1.33),
    );
    let right = Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Metal::new(Color::new(0.8, 0.6, 0.2), 1.0),
    );

    let world: [Box<dyn Hittable>; 4] = [
        Box::new(ground),
        Box::new(central),
        Box::new(left),
        Box::new(right),
    ];

    log.msg("Render frame ").flush();

    let timer = renderer.render(world.as_slice(), Fading::<2>::Const(0.8));
    log.elapsed(&timer).ln();

    log.msg("Output image ").flush();

    let timer = renderer.output();
    log.elapsed(&timer).ln();
}
